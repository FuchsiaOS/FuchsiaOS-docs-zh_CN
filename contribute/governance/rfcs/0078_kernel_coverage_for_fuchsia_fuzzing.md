{% set rfcid = "RFC-0078" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!--
*** This should begin with an H2 element (for example, ## Summary).
-->

## Summary

This change will introduce new syscalls that enable collection and transfer of kernel code coverage data. The syscalls are only implemented on the **existing** _sancov_ build. No other build variants will be affected (with new syscalls returning `ZX_ERR_NOT_SUPPORTED`). The initial client for kernel coverage is the system call fuzzing engine, Syzkaller. A [proof-of-concept] for this proposal has been implemented along with unit tests (see descendant changes) to evaluate its efficacy.

## Background

Syzkaller is a coverage-guided kernel fuzzer. It generates sequences of syscalls to test the operating system, and relies on coverage information to mutate them and determine which sequences were useful. Syzkaller is already used in Fuchsia, but the current integration does not collect code coverage data.

On Fuchsia, Syzkaller runs in HostFuzzer mode, in which the fuzzing engine (`syz-fuzzer`) lives outside the fuzzed VM and communicates with a fuzzing agent (`syz-executor`) that executes a sequence of system calls and communicates the coverage back to the engine.

Kernel code coverage can be obtained using Clang's SanitizerCoverage ([sancov]) instrumentation. This instrumentation works by adding a call to `__sanitizer_cov_trace_pc_guard` on every basic block, and we then implement that function to keep track of the visited program counters (PCs).  [Linux has supported it since 2016][sancov-linux-support], and its implementation works by keeping a per-thread list of covered PCs.

Zircon supports a _sancov_ build variant that also applies to the Zircon kernel, exporting a live VMO in the sancov format (a sparse table of PCs that were hit). However, Syzkaller runs multiple programs at the same time and looks at per-syscall coverage, so we need finer-grained information.

## Requirements

A successful implementation must export kernel code coverage data in a way that is useful for fuzzing engines. Currently, the primary consumer will be Syzkaller; as such, many requirements are imposed by Syzkaller's architecture and assumptions. The requirements are as follows:

- **Thread-level granularity**: Syzkaller's `syz-executor` uses a thread pool to execute system calls, collecting coverage for each individual system call— i.e. the code executed by the kernel in the context of each system call.

  The format it consumes is a set of program counters that were hit. A pseudocode of each worker thread is as follows (each job is a system call):

```
Thread:
    Enable Coverage
    While True:
        Job <- wait for job event
        Start Tracking Coverage
        Execute Job
        Collect Coverage
        Signal job done
```

  The strategy implemented by Syzkaller implies the need to collect kernel coverage information from only those kernel threads engaged in servicing syscalls initiated by `syz-executor` worker threads.

- **Fast**: The time required to collect and transfer coverage data to the fuzzing engine should be minimized because `syz-executor` queries coverage information after running each syscall. A faster collect-and-transfer flow allows Syzkaller to test more programs in the same amount of time.

- **Low-noise**: Syzkaller performs most effectively when there is no coverage signal outside of code that is deterministically executed by the syscall under test. Fully instrumenting the kernel thread that services a `syz-executor` thread almost does this, but noise is introduced by scheduler code and servicing interrupts. A successful implementation should minimize noise as much as possible.

- **Test-only**: The features to collect and export coverage should not be enabled in regular builds. They should only be part of builds that use the _sancov_ variant. The coverage collection interface is not guaranteed to be stable; it is intended to be used only for fuzzing engines such as Syskaller. There must be no impact to memory usage or runtime performance for non-_sancov_ builds.

### Out of Scope

It would be nice to have the following requirements, but they are out of scope for this RFC. Think of them as «future work».

- Implement more sophisticated granularity and/or control flow tracking such as process-level tracking or "baton-passing" to track coverage over communicating processes.

- Provide a mechanism for excluding coverage collection on certain parts of the kernel. This is somewhat at odds with the low-noise requirement, but it is reasonable to expect that kernel-wide coverage will be sufficiently low noise to guide Syskaller in an initial implementation.

- Collect coverage data from kernel threads other than the thread executing the system call under test.

## Design

The existing _sancov_ variant will be extended to support new syscalls that enable collection and transfer of kernel code coverage data.

## Implementation

### Syscalls (Enabled on sancov Variant Only)

The following new syscalls are introduced. These syscalls are supported on _sancov_ variant builds.

- `coverage_control(uint32_t action)`: Request that the kernel either start collecting coverage data with a fresh buffer (`action=KCOV_CTRL_START`), or stop collecting coverage data (`action=KCOV_CTRL_STOP`).

- `coverage_collect(uintptr_t* buf, size_t count, size_t* actual, size_t* avail)`: Request that the kernel copy current coverage data into the userspace buffer referred to by `buf`. The only optional parameter is `avail`. The operation does not consume the data; data will remain available until the kernel buffer is reset by `coverage_control(KCOV_CTRL_START)`. The number of bytes copied to `buf` is stored in `actual` and the total number of bytes currently available is stored in `avail` (if it is set). A return value of `ZX_ERR_NO_SPACE` indicates that the kernel's coverage buffer size is insufficient and coverage data may have been lost; the return value is intended to cue the client that it may need to collect more frequently to ensure that no data are missed.

Note that these system calls only control the coverage collection of a single thread, and have no effect on any global coverage collection. This way, Syzkaller and other fuzzers can make sure that they collect coverage only for the parts that they care about.

### Kernel Memory Requirements (sancov Variant Only)

- One 300KiB buffer per user thread with coverage enabled

- One pointer to the above-mentioned buffer in ThreadDispatcher

- One counter of number of entries in the above-mentioned buffer in ThreadDispatcher

Upon enabling coverage for a thread, the kernel allocates a buffer large enough to collect coverage until collection is disabled or reset. The size of this buffer can change in the future; it will initially be 300KiB, which is approximately the size of the sancov PC table. This memory must to be always committed; this is achieved by creating a kernel-only VmMapping into the kernel's root VMAR (similar to how kstacks are created) and storing the VMAR handle in the ThreadDispatcher.

ThreadDispatcher stores a pointer to this buffer and a count of how many entries it has. If a thread exceeds the coverage limit, no new coverage is registered. `coverage_control(KCOV_CTRL_START)` resets the counter back to zero (and starts collecting a fresh buffer of coverage data); there is no need to spend time clearing the buffer. To avoid adding memory overhead in non-_sancov_ builds, these ThreadDispatcher fields could be `#ifdef`'d out.

### Sancov Data Collection

`__sanitizer_cov_trace_pc_guard` checks the current running thread to see if there's a buffer enabled and, if so, append the PC that was hit to the list. The system calls operate over the current thread, so there are no races with other threads. It is possible for a kernel thread to be interrupted while handling a system call; this will produce noise, but not a race. Once coverage is enabled for a thread, the buffer remains allocated until the thread is destroyed.

`__sanitizer_cov_trace_pc_guard` cannot take faults or exceptions while it is running, as the handlers will probably call into `__sanitizer_cov_trace_pc_guard` again and it might end up in a loop. To avoid faults, the memory used to store PCs must always be committed.

### Risk of Reentrancy

There is some risk of reentrancy between `__sanitizer__*` functions. Care is taken to control sources of reentrancy to ensure that it will not cause issues. In particular:

- `__sanitizer__*` implementations do not directly or indirectly invoke `__sanitizer__*` functions

- `__sanitizer_*` functions do not acquire any locks

As such, the only source of reentrancy is interrupts that will not cause infinite recursion or deadlock. The noise in coverage data caused by interrupts is discussed below.

### Noise in Coverage Data

In the context of this design, the following sources of noise in coverage data are expected:

- Code in `coverage_control(KCOV_CTRL_START)` executed after the buffer is reset, but before return

- Code in `coverage_control(KCOV_CTRL_STOP)` executed before collection is stopped

- Code in `coverage_collect` executed before data is finished being copied into the client buffer

- Interrupts executed while servicing the target user thread's syscalls

Most of these sources of noise are nearly deterministic (i.e., the same code paths will appear in every batch of collected coverage). Some sources can be eliminated using a sancov mechanism for denylisting particular code, but these source of noise are sufficiently small and predictable that the initial implementation will not take on the complexity of managing a denylist.

## Performance

No changes to performance on non-_sancov_ variant builds are introduced. The only deployments affected by the change are _sancov_ variant builds that exercise new syscalls. The expectation, in this case, is that performance remains the same but suddenly degrades when a thread enables kernel coverage because every syscall made by that thread will trigger a buffer write on every basic block executed in the kernel. This performance degradation is acceptable because it is only introduced when running a kernel fuzzer engine for the purpose of collecting this data.

## Security considerations

Kernel code addresses are generally considered to be very sensitive information that would be extremely valuable to an attacker. The risk of leaking this information on a production device is mitigated by enabling syscalls that surface this information in _sancov_ (test-only, non-production) variant builds **only**.

## Privacy considerations

This proposal does not involve collecting or handling user data.

## Testing

Testing is implemented in two phases:

1. **Unit tests** ingest syscall addresses extracted from the zircon image and check for the existence (and non-existence) of various syscalls under several conditions (e.g., single syscall, multiple syscalls, multiple communicating threads, thread crash, etc.)
1. **Integration tests** run on a VM because kernel symbol information is not available to userspace programs. The VM exports coverage data to the host environment where the sancov code coverage tool is used to verify that PCs belong to the expected kernel functions.

Alongside the initial implementation, the plan is to land unit tests and tests that ensure new syscalls return `ZX_ERR_NOT_SUPPROTED` in non-sancov build variants.

## Documentation

New syscalls will be integrated into Zircon documentation in the usual way, alongside caveats and build instructions explaining the relationship between the syscalls and the _sancov_ build variant.

## Drawbacks, alternatives, and unknowns

The following alternatives were considered but rejected as a part of this design and implementation:

- **Data Format**: One alternative is to keep the current Sancov format, but export it per-thread via a syscall.  Although this would work, it is inefficient because it requires copying the entire 400KiB PC table to userspace each time, whereas the actual list of PCs hit during a single system call is generally far less (for example a `zx_channel_read` of a 1KiB buffer with 2 handles collects 163 PCs and `zx_channel_write` collects 127 pcs, vs ~51k total pcs).

- **Instrumentation Method**: Two compiler-instrumented coverage alternatives are Intel Processor Trace or QEMU instrumentation. These alternatives may be viable, but they take significant effort to set up, and are not as flexible as Clang's SanitizerCoverage instrumentation.

- **API Design**: Instead of having a separate cover_collect method for copying coverage information to userspace, our original design consisted of sharing a VMO between the kernel and userspace.  However, we decided against it as doing so was not recommended by the Zircon team: vmos are not intended to be shared between kernel and userspace, but the benefit would be that we wouldn't need to copy the coverage from kernel to userspace.

- **Testing Approach**: A more expensive (but potentially more thorough) testing approach was considered: Tests are run on the host and spin up a VM. Tests execute a sequence of system calls, then exfiltrate the coverage out of the VM, and the sancov code coverage tool is used to verify that PCs belong to the expected kernel functions.

## Prior art and references

[proof-of-concept]: https://fuchsia-review.googlesource.com/c/fuchsia/+/486997/
[sancov]: https://clang.llvm.org/docs/SanitizerCoverage.html
[sancov-linux-support]: https://github.com/torvalds/linux/commit/5c9a8750a6409c63a0f01d51a9024861022f6593
