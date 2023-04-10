# Sanitizers

## Motivation

Sanitizers are tools for detecting certain classes of bugs in code. The
operating principle varies, though sanitizers commonly (but not always) rely on
some form of compile-time instrumentation to change code in ways that expose
bugs at runtime.
Fuchsia uses a variety of sanitizers to discover and diagnose dangerous bugs
that are otherwise difficult to find.

Sanitizers are enabled by build-time flags. Sanitizer builds are continuously
exercised on Fuchsia's Continuous Integration (CI) and Commit Queue (CQ), and
serve Fuchsia C/C++ and Rust developers.

Developers typically benefit from sanitizers with no special action required,
and only need to pay attention to sanitizers when they detect a bug.
However certain limitations apply. Continue reading to learn what sanitizers are
supported and how to use them.

## Supported sanitizers

Fuchsia currently supports the following sanitizers:

*    [AddressSanitizer][llvm-asan]{:.external} (ASan) detects instances of
     out-of-bounds access, use after free / return / scope, and double free.
*    [LeakSanitizer][llvm-lsan]{:.external} (LSan) detects memory leaks.
     LeakSanitizer works like a conservative garbage collector in checking
     for leaks. Any allocations that are not reachable from reference roots
     (stacks of threads, registers of threads, global variables and
     thread-local variables) are treated as leaked.
*    [ThreadSanitizer][llvm-tsan]{:.external} (TSan) detects data races (host-only).
*    [UndefinedBehaviorSanitizer][llvm-ubsan]{:.external} (UBSan) detects
     specific issues of relying on undefined program behavior.

The following sanitizer behavior is available in the Zircon kernel:

*    Physical Memory Manager (PMM) checker (`pmm_checker`) detects
     use-after-free bugs and stray DMAs.
*    [Kernel AddressSanitizer][kasan] (KASan) extends AddressSanitizer to kernel
     code in collaboration with the PMM.
*    [Lockdep][lockdep] is a runtime lock validator that detects lock hazards
     such as deadlocks.

The following C/C++ compilation options are added by default to detect or
prevent bugs at runtime:

*   `-ftrivial-auto-var-init=pattern` (see [RFC][ftrivial-rfc]) initializes
    automatic variables to a non-zero pattern to expose bugs related to reads
    from uninitialized memory.
*   [ShadowCallStack][shadowcallstack] and [SafeStack][safestack] harden the
    generated code against stack overflows.

Lastly, Fuchsia uses [libFuzzer][llvm-libfuzzer]{:.external} and
[syzkaller][syzkaller]{: .external} to perform coverage-directed
[fuzz testing][fuzz-testing]. Fuzzers are similar to sanitizers in that they
attempt to expose bugs in the code at runtime, and they are usually used in
conjunction. Fuzzers are different from sanitizers in that fuzzers attempt to
force the execution of production code into paths that may expose a bug.

## Supported configurations

Sanitizers are currently supported in local builds and in CI/CQ under the
following configurations:

*   `bringup.x64`
*   `bringup.arm64`
*   `core.x64`
*   `core.arm64`
*   `zbi_tests.x64`
*   `zbi_tests.arm64`

In addition, sanitizers apply to host tools.

Tests for all of the above are exercised on CI/CQ on [qemu][qemu]{:.external}
and [Intel NUC][nuc]. Other platforms are not tested with sanitizers due to
resources and capacity issues, but you may test on these platforms locally
using the build workflow [below](#build).

Additional tryjobs for configurations defined under `//vendor` may be shown in
Gerrit and in CI consoles for certain signed-in users. Look for configurations
with `-asan` in their name.

The sanitizers listed above are applied to C/C++ code. In addition, LSan is
applied to Rust code for detecting [Rust memory leaks][rust-leaks]{:.external}.

## Troubleshooting sanitizer issues

### Build

#### Fuchsia platform build (in-tree)

To reproduce a sanitizer build, specify the sanitizer variants:

```posix-terminal
fx set {{ '<var label="product">product</var>' }} --variant asan-ubsan --variant host_asan-ubsan
```

Alternatively you may select to only instrument certain binaries:

```posix-terminal
fx set {{ '<var label="product">product</var>' }} --variant asan-ubsan/{{ '<var>executable_name</var>' }}
```

The selective instrumentation workflow is useful for testing locally on
hardware where a fully instrumented build does not fit on the device.

Specifically to detect use-after-free bugs in kernel code you will need to
[enable the kernel PMM checker][enable-pmm-checker].

#### Out-of-tree build

When compiling with the Fuchsia toolchain it is sufficient to pass the
`-fsanitize=` flag to indicate which sanitizers to use.
See the [compiler documentation][fsanitize]{:.external}.

When creating a Fuchsia package with instrumented components, you need to
ensure that your package contains all runtime dependencies including the
sanitizer runtime, which is distributed as part of the Clang toolchain, and
instrumented C library, which is distributed as part of the Fuchsia SDK
under sysroot.

### Test

Test as you normally would, in your local workflow or on a CQ builder which has
sanitizers enabled (the tryjob has `asan` in the name). If a sanitizer detects
an issue then messages will be printed to the logs containing one of the
following strings:

*   `ERROR: AddressSanitizer`
*   `ERROR: LeakSanitizer`
*   `SUMMARY: UndefinedBehaviorSanitizer`
*   `WARNING: ThreadSanitizer`

Following these messages you will find stack traces that identify the nature of
the problem and point to the root cause. You can find these messages in
`fx log`.

Note that the test that triggered the sanitizer may still appear as passing.
Sanitizer issues don't manifest as test failures.

Issues detected by sanitizers typically have similar root causes. You may be
able to find references for prior work by [searching Fuchsia bugs][fxb] for a
bug with some of the same keywords that you're seeing in the sanitizer output.

## Known issues

### `#[should_panic]`

Fuchsia's Rust builds [abort on `panic!`][rust-panic-abort]. This significantly
reduces binary size. An unfortunate side effect is that tests that use the
`#[should_panic]` attribute may falsely detect memory leaks. These tests emit an
expected panic and then exit without unwinding, which means they don't free
their [heap allocations][rust-heap]. To LeakSanitizer this is indistinguishable
from a real memory leak.

If this issue affects your test then you can disable it in sanitizer builds by
following [this example][fxr605381].

See: [Issue 88496: Rust tests that should_panic trigger leaksanitizer][fxb88496]

## Best practices

### Ensure that your code is exercised by tests

Sanitizers expose bugs at runtime. Unless your code runs, such as within a test
or generally on Fuchsia in such a way that's exercised by CI/CQ, sanitizers
won't be able to expose bugs in your code.

The best way to ensure sanitizer coverage for your code is to ensure test
coverage under the same configuration. Consult the guide on [test
coverage][test-coverage].

### Don't suppress sanitizers in your code

Sanitizers may be suppressed for certain build targets. Most commonly this is
used for issues that predate the introduction of sanitizer support, especially
for issues in third party code that the Fuchsia project doesn't own.

Suppressed sanitizers should be considered tech debt, as they not only hide old
bugs but keep you from discovering new bugs as they're introduced to your code.
Ideally new suppressions should not be added, and existing suppressions should
be removed and the underlying bugs fixed.

Suppressing sanitizers may be done by editing the `BUILD.gn` file that defines
your executable target as follows:

```gn
executable("please_fix_the_bugs") {
  ...
  # TODO(fxbug.dev/12345): delete the below and fix the memory bug.
  deps += [ "//build/config/sanitizers:suppress-asan-stack-use-after-return" ]
  # TODO(fxbug.dev/12345): delete the below and fix the memory bug.
  deps += [ "//build/config/sanitizers:suppress-asan-container-overflow" ]
  # TODO(fxbug.dev/12345): delete the below and fix the memory leak.
  deps += [ "//build/config/sanitizers:suppress-lsan.DO-NOT-USE-THIS" ]
}
```

The examples above demonstrate suppressing all sanitizers. However you should at
most suppress sanitizers that are causing failures. Please track suppressions
by filing a bug and referencing it in the comment as shown above.

Another common approach for disabling sanitizers works as follows:

```gn
executable("too_slow_when_built_with_asan") {
  ...
  exclude_toolchain_tags = [ "asan" ]
}
```

Both examples above suppress at the granularity of an entire executable.
For finer-grained suppressions you may detect the presence of sanitizers in
code. This is useful for instance for suppressing sanitizers in a particular test
case, but not more broadly. For instance this is used by tests that intentionally
introduce memory errors and test the sanitizer runtime itself.

For C/C++ see:

*   [Conditional Compilation with
    `__has_feature(address_sanitizer)`][asan-conditional]{:.external}
*   [Disabling Instrumentation with
    `__attribute__((no_sanitize("address")))`][asan-disabling]{:.external}

For Rust, you can follow this pattern:

```rust
#[cfg(test)]
mod tests {
    #[test]
    // TODO(fxbug.dev/12345): delete the below and fix the leak
    #[cfg_attr(feature = "variant_asan", ignore)]
    fn test_that_leaks() {
        // ...
    }
}
```

### Test for flakiness

Sanitizer errors may be flaky if the code under test's behavior is
non-deterministic. For instance a memory leak may only happen under certain race
conditions. If sanitizer errors appear flaky, consult the guide on [testing for
flakiness in CQ][testing-flakiness].

### File good bugs

When encountering a sanitizer issue, file a bug containing all the
troubleshooting information that's available.

Example: [Issue 73214: ASAN use-after-scope in blobfs][fxb73214]

The bug report contains:

*   The error provided by the sanitizer (ASan in this case).
*   Instructions on how to build & test to reproduce the error.
*   Details of the investigation that followed, with specific code pointers as
    needed.
*   References to relevant changes, such as in this case the change to fix the
    root cause for the bug.

## Roadmap

Ongoing work:

*   [Hardware-accelerated AddressSanitizer][llvm-hwasan]{:.external} (hwasan):
    significantly reduce the memory overhead of asan, making instrumentation
    viable on RAM-constrained devices and working to close testing gaps for
    hardware-dependent code.
    See also: [RFC-0143: Userspace Top-Byte-Ignore][rfc-0143].
*   [GWP-ASan][llvm-gwp-asan]{:.external}: efforts are currently underway to
    demonstrate the use of this sampling version of asan to detect bugs in the
    field.
*   [Coverage-directed kernel fuzzing via syscalls][rfc-0078].

Areas for future work:

*   [ThreadSanitizer][llvm-tsan]{:.external} (TSan): detecting data races.
*   Kernel support for detecting concurrency bugs.
*   Extending sanitizer support for Rust, such as detecting memory safety bugs
    in Rust `unsafe {}` code blocks or across [FFI][ffi]{:.external} calls, or
    detecting undefined behavior bugs.
*   [MemorySanitizer][llvm-msan]{:.external} (MSan): detecting reads from
    uninitialized memory.

See also: [sanitizers in the 2021 roadmap][sanitizers-2021-roadmap].

[asan-conditional]: https://clang.llvm.org/docs/AddressSanitizer.html#conditional-compilation-with-has-feature-address-sanitizer
[asan-disabling]: https://clang.llvm.org/docs/AddressSanitizer.html#disabling-instrumentation-with-attribute-no-sanitize-address
[enable-pmm-checker]:  /gen/boot-options.md#kernel_pmm_checker_enable_bool
[ffi]: https://doc.rust-lang.org/nomicon/ffi.html
[fsanitize]: https://clang.llvm.org/docs/UsersManual.html#controlling-code-generation
[ftrivial-rfc]: https://lists.llvm.org/pipermail/cfe-dev/2018-November/060172.html
[fuzz-testing]: /contribute/testing/fuzz_testing.md
[fxb]: https://bugs.fuchsia.dev/p/fuchsia/issues/list
[fxb73214]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=73214
[fxb88496]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=88496
[fxr605381]: https://fuchsia-review.googlesource.com/c/fuchsia/+/605381/
[kasan]: /zircon/kernel/lib/instrumentation/asan/README.md
[lockdep]: /concepts/kernel/lockdep.md
[llvm-asan]: https://clang.llvm.org/docs/AddressSanitizer.html
[llvm-gwp-asan]: https://llvm.org/docs/GwpAsan.html
[llvm-hwasan]: https://clang.llvm.org/docs/HardwareAssistedAddressSanitizerDesign.html
[llvm-libfuzzer]: https://llvm.org/docs/LibFuzzer.html
[llvm-lsan]: https://clang.llvm.org/docs/LeakSanitizer.html
[llvm-msan]: https://clang.llvm.org/docs/MemorySanitizer.html
[llvm-tsan]: https://clang.llvm.org/docs/ThreadSanitizer.html
[llvm-ubsan]: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html
[nuc]: /development/hardware/intel_nuc.md
[qemu]: https://www.qemu.org
[rfc-0078]: /contribute/governance/rfcs/0078_kernel_coverage_for_fuchsia_fuzzing.md
[rfc-0143]: /contribute/governance/rfcs/0143_userspace_top_byte_ignore.md
[rust-heap]: https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-the-heap.html#the-heap
[rust-leaks]: https://doc.rust-lang.org/nomicon/leaking.html
[rust-panic-abort]: https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic
[safestack]: /concepts/kernel/safestack.md
[sanitizers-2021-roadmap]: /contribute/roadmap/2021/invest_in_sanitizers.md
[shadowcallstack]: /concepts/kernel/shadow_call_stack.md
[syzkaller]: https://github.com/google/syzkaller
[test-coverage]: /contribute/testing/coverage.md
[testing-flakiness]: /development/testing/testing_for_flakiness_in_cq.md
