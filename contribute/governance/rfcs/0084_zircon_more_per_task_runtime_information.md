{% set rfcid = "RFC-0084" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

The `ZX_INFO_TASK_RUNTIME` topic provides a way to retrieve the total amount of
time a task has spent running on a CPU or queued to run. In order to diagnose
scheduling bottlenecks, we propose adding additional fine-grained runtime
information, specifically the amount of time spent waiting for a page fault and
the amount of time spent waiting on a kernel mutex.

## Motivation

Real-time tasks have deadlines. Sometimes, deadlines are missed because tasks
spend an unexpectedly long time blocked in the kernel. To debug these cases, it
is helpful to understand why the task was blocked. For example, if a task has a
10ms deadline, but spends 11ms waiting on a page fault, we can conclude that the
deadline was missed because the page fault was too slow.

Concretely, we want to improve diagnostics produced by media subsystems such as
`audio_core`. In `audio_core`, each mixer task must complete within 10ms. If a
task takes longer than 10ms, the user will hear glitchy audio. Recently, we have
seen missed deadlines due to slow page faults (where `audio_core` executable
pages needed to be paged back in) and contention on the kernel's heap mutex.
These problems cannot be diagnosed by snapshots -- instead we must record a
trace, which is a tedious process that is sometimes hampered by difficulty of
reproducing the problem locally (as the bug might trigger only with a specific
app in a specific environment). These are high-priority problems that have
severely negative impacts on media performance.

Our goal is to export enough diagnostic information from the kernel so that we
can diagnose these problems from a snapshot, without requiring a deep-dive into
an execution trace. This document proposes adding new statistics to
`zx_info_task_runtime_t` to more completely answer the question "why was this
task not runnable?".

## Background on Zircon thread deadlines

Zircon deadline profiles have three components: period, capacity, and deadline.
When a thread is assigned a deadline profile, Zircon guarantees that each
_period_, the thread is allocated up to _capacity_ CPU within _deadline_ of the
start of each _period_. Consider an example:

*   Period = 10ms
*   Capacity = 2ms
*   Deadline = 5ms

Every 10ms, the thread is allocated 2ms of CPU within the next 5ms. There are
two ways a deadline can be missed:

1.  The task is scheduled late. For example, if the next period starts at time
    T, but the task is not scheduled until time T+4ms, it's impossible for the
    task to make its deadline (it should have been scheduled no later than
    T+3ms). The kernel can detect this kind of missed deadline, but in practice,
    this should never happen unless the scheduler is buggy or oversubscribed.

2.  The task takes longer than 2ms to complete. The kernel cannot know when this
    happens because it doesn't understand task boundaries. For example, if the
    task is scheduled at T+1ms and runs for a total of 1ms, then blocks for 9ms,
    the kernel cannot know if the task missed its deadline (because it was
    blocked on something) or if the task asked for 2ms but needed just 1ms to
    complete (then slept for 9ms to wait for the next period).

Our goal is to help diagnose the second kind of missed deadline. Currently, if a
task runs for too long, it can query `ZX_INFO_TASK_RUNTIME` to learn how much
time was spent running on a CPU in user space. If that `cpu_time` is larger than
the task's expected runtime, the task knows it simply ran for too long. However,
if the `cpu_time` is a small fraction of the total task time, then the task
spent most of its time in the kernel, likely blocked and not runnable. The goal
of this RFC is to help understand where that time was spent.

## Design space

Our goal is to answer the question "why was this task not runnable?". We must
make a few decisions:

1.  How complete should our answer be? Specifically, should we enumerate all
    reasons a task was blocked, or just a few reasons that seem important?

1.  At what granularity should we answer this question? For example, should we
    report only simple events that are understandable at the user level, such as
    "blocked on zx_channel_read", or should we include lower-level events that
    are specific to Zircon's current implementation?

1.  If we report N statistics, should we require that those N statistics not
    overlap, or should we allow them to overlap, perhaps in an arbitrary way?

The simplest, most direct approach is to enumerate a few events we care about
and produce statistics for those events. Given that media subsystems have seen
problems with page faults and kernel lock contention, we might produce
statistics for "time spent in page faults" and "time spent blocked on kernel
locks".

There may be other issues lurking that will not be captured by these two
statistics. Hence, a complete solution is attractive. One idea is to enumerate
all the ways that a thread can enter the kernel. This includes N hardware
interrupts (timers and device interrupts) and K software interrupts (system
calls and faults). We then produce N+K statistics, one for each kind of
interrupt. A timer would start when the thread enters the kernel and stop when
control is returned to the user-space thread. However, user-level can already
compute "time spent in syscall X", so having the kernel compute this information
is redundant.

Another idea is to produce statistics "time spent running on the CPU while in
kernel mode" and "time spent blocked on X", where X is a set of kernel
primitives, such as "kernel lock" or "channel". This idea runs the risk of bloat
and churn as the set of kernel primitives changes over time.

Stepping back, what we really want is to fetch traces from devices in the wild.
Ideally we'd continuously record a trace into a circular buffer and upload that
buffer after hitting a
[`TRACE_ALERT`](https://cs.opensource.google/fuchsia/fuchsia/+/main:zircon/system/ulib/trace/include/lib/trace/internal/event_common.h;drc=cda406c8b5fc88d434183b1009010ba6f2c3c7b6;l=406).

## Design

Although a fully complete solution is desirable, it may take a long time to
design and build. We have an immediate need to diagnose performance regressions
in the wild. Hence, I propose that we add two targeted metrics to address our
immediate problems: the amount of time spent waiting for page faults, and the
amount of time spent waiting for kernel locks.

Regarding the design space questions above:

1.  We will not aim for completeness.

1.  The granularity is arbitrary (we'll record whatever we think we need)

1.  Statistics may overlap in arbitrary ways

### Kernel changes

```
// This struct contains a partial breakdown of time spent by this task since
// creation. The breakdown is not complete and individual fields may overlap:
// there is no expectation that these fields should sum to an equivalent
// "wall time".
typedef struct zx_info_task_runtime {
  // Existing fields
  zx_duration_t cpu_time;
  zx_duration_t queue_time;

  // New fields below here

  // The total amount of time this task and its children spent handling page faults.
  zx_duration_t page_fault_time;

  // The total amount of time this task and its children spent waiting on contended
  // kernel locks.
  zx_duration_t lock_contention_time;

} zx_info_task_runtime_t;
```

Both of these fields will be computed per-thread, then summed across processes
and jobs as is currently done for `cpu_time` and `queue_time`. Note that
per-process and per-job aggregation is not needed by the media subsystems, but
it's included here for consistency with the existing fields in
`zx_info_task_runtime_t`.

There are multiple kinds of page faults and multiple kernel locks.
`page_fault_time` represents the total time spent handling all kinds of page
faults. By covering all page faults, we avoid needing to explain which subset of
page faults are covered, which could be difficult as the kernel may add or
remove certain kinds of page faults as the implementation changes over time and
as new architectures are supported.

`lock_contention_time` covers all contended locks. However, the term "contended"
is left intentionally underspecified so the kernel may evolve its implementation
over time to balance the cost of measuring contention against the benefit of
reporting contended time. See Implementation (below) for additional discussion.

### How user space can diagnose missed deadlines

Given these new fields, user space can use code like the following to diagnose
missed deadlines:

```
for (;;) {
  zx_object_get_info(current_thread, ZX_TASK_RUNTIME_INFO, &start_info, ...)
  deadline_task()
  if (current_time() > deadline) {
    zx_object_get_info(current_thread, ZX_TASK_RUNTIME_INFO, &end_info, ...)
    // ...
    // report stats from (end_info - start_info)
    // ...
  }
}

```

## Implementation

`page_fault_time` will compute the total time taken by all page fault handlers.
In the current implementation, this includes `vmm_page_fault_handler` and
`vmm_accessed_fault_handler`.

`lock_contention_time` will compute the total time taken by
`Mutex::AcquireContendedMutex` and `BrwLock::Block`. This method already has
access to the current `Thread` and the `current_ticks()`. The implementation
will not cover spin locks. Although spin locks can be contended, we ignore spin
locks for now because measuring contention on spin locks may be prohibitively
expensive.

To minimize overhead, we will record these durations as tick counts and
translate to a `zx_duration_t` during the `zx_object_get_info` system call.
Other details of the implementation will follow the existing pattern used by
`cpu_time` and `queue_time`. A prototype implementation is available at
fxrev.dev/469818.

## Performance

We will run Zircon mutex benchmarks to verify there are no regressions. We will
run these benchmarks on raw hardware (x86 and ARM). Additionally, to verify
there are no regressions in virtualized environments, we will run these
benchmarks on QEMU (x86 and ARM).

## Backwards compatibility

The `zx_info_task_runtime_t` struct will be versioned, similar to what has been
done for other `zx_info_*` structs (for an example, see fxrev.dev/406754).

## Security considerations

`ZX_INFO_TASK_RUNTIME` is a side channel that can leak information about the
inspected task. For example, `page_fault_time` might be used to measure a task's
memory access patterns. As a mitigation for this kind of leakage, the
`ZX_INFO_TASK_RUNTIME` topic already requires `ZX_RIGHT_INSPECT`. Any user with
that right can be assumed to have access to a task's private data.

`ZX_INFO_TASK_RUNTIME` can also leak indirect information about other tasks. For
example, if a task knows its own `page_fault_time`, it may be able to infer the
memory access patterns of _other_ tasks. Similarly, if a task knows how much
time it spent waiting on contested kernel locks, it may be able to infer how
other tasks are using shared kernel resources. In the future, we may build
`zx_info_task_runtime_t` using low resolution timers. This does not necessarily
prevent timing attacks but it can limit their effectiveness.

Another defense is to limit `zx_info_task_runtime_t` access to special developer
builds. However, this would significantly limit the usefulness of this feature:
often we have trouble reproducing performance bugs in developement environments.
We require a solution that can be enabled in production builds.

To avoid this side channel entirely, we'd need to separate metric _reporting_
and metric _inspection_ into separate capabilities. For example, if we
continuously record a trace into a circular buffer and upload that buffer to a
special channel or port after hitting a
[`TRACE_ALERT`](https://cs.opensource.google/fuchsia/fuchsia/+/main:zircon/system/ulib/trace/include/lib/trace/internal/event_common.h;drc=cda406c8b5fc88d434183b1009010ba6f2c3c7b6;l=406),
then the task which triggered the `TRACE_ALERT` would not need to be given
access to the trace, which eliminates the side channel. As mentioned earlier,
such a solution will take a long time to design and build, while we have an
immediate need to address today.

## Privacy considerations

None.

## Documentation

The Zircon syscall documentation will need to be updated to include the new
`zx_info_task_runtime_t` fields.

## Prior art and references

In Linux, the most closely related prior art is `getrusage`, which reports user
and system CPU time and counts of page faults, I/O ops, and context switches.
Windows has `GetThreadTimes`, which reports user and system CPU time. Hardware
performance counters, such as `RDPMC` on x86, provide similar information and
have
[similar security concerns](https://www.kernel.org/doc/Documentation/admin-guide/perf-security.rst).

## Testing

We will manually test by logging new runtime information through `audio_core`
(and we've already done this for the my prototype implementation: see
fxrev.dev/469819).

We will update existing tests for `cpu_time` and `queue_time` to test the old
version of `zx_info_task_runtime_t`, which will be named
`zx_info_task_runtime_v1_t`. Additionally, Zircon's
[abi_type_validator.h](https://cs.opensource.google/fuchsia/fuchsia/+/main:zircon/kernel/lib/abi_type_validator/include/lib/abi_type_validator.h;bpv=1;bpt=0;drc=bce936b9b16d91dba88d5372bb10361ff2e7645a)
will be updated to validate the old and new ABIs. This will ensure that ABI
backwards compatibility is preserved.

It's not easy to add integration tests for this feature because, e.g., there's
no API to force the kernel to suffer lock contention or trigger a page fault
(other than a process-killing segmentation fault).
