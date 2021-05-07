{% set rfcid = "RFC-0074" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->
<!--
*** This should begin with an H2 element (for example, ## Summary).
-->

## Summary

We refactor `ZX_INFO_PROCESS` to expose a process's start time and also to
clean up the flags describing its runtime.  We say a process has a valid start
time when a `ZX_INFO_PROCESS_FLAG_STARTED` flag is present and define it to be
the monotonic time at which `zx_process_start()` was called, when valid.

## Motivation

First filed at crbug.com/726484 and then fxbug.dev/30751, Chromium requested
the feature in order to give a base time for trace events, extending a
platform-agnostic interface already supporting Linux, macOS, and Windows.
Further, fxbug.dev/43108 was filed to leverage this same feature in order to
include uptime in crash reports.

## Design

In a small, straightforward extension of `ProcessDispatcher`, we would record
the time in `ProcessDispatcher::AddInitializedThread()` for the initial thread
and then pass it along in `ProcessDispatcher::GetInfo()`; the former method is
the main "starting" routine of `zx_process_start()`, while the latter is that
of `zx_get_object_info()` for a process handle with `ZX_INFO_PROCESS`.

As for the struct populated by `zx_object_get_info()`, currently we have
```
typedef struct zx_info_process {
    // The process's return code; only valid if |exited| is true.
    // If the process was killed, it will be one of the ZX_TASK_RETCODE values.
    int64_t return_code;
    // True if the process has ever left the initial creation state,
    // even if it has exited as well.
    bool started;
    // If true, the process has exited and |return_code| is valid.
    bool exited;
    // True if a debugger is attached to the process.
    bool debugger_attached;
    uint8_t padding1[5];
} zx_info_process_t;
```
We would evolve it to be
```
typedef struct zx_info_process {
    // The process's return code; only valid if the
    // |ZX_PROCESS_INFO_FLAG_EXITED| flag is set. If the process was killed, it
    // will be one of the |ZX_TASK_RETCODE| values.
    int64_t return_code;
    // The monotonic time at which `zx_process_start()` was called, only valid
    // if the |ZX_INFO_PROCESS_FLAG_STARTED| flag is set.
    zx_time_t start_time;
    // Bitwise OR of ZX_INFO_PROCESS_FLAG_* values.
    uint32_t flags;
    uint8_t padding1[4];
} zx_info_process_t;
```
for which the following flag values would be introduced:
```
// Whether the process has started. `zx_process_info_t::start_time` is only
// valid if this flag is set.
#define ZX_INFO_PROCESS_FLAG_STARTED (1u << 0)

// Whether the process has exited.
#define ZX_INFO_PROCESS_FLAG_EXITED (1u << 1)

// Whether a debugger is attached to the process.
#define ZX_INFO_PROCESS_FLAG_DEBUGGER_ATTACHED (1u << 2)
```
The boolean-to-flag refactor is not strictly necessary, but it would bring
`zx_info_process_t` in line with current syscall struct policy, save a byte
on padding, and would not increase the amount of work needed to carry out this
proposal (and also would save the next engineer from a fair amount of hassle).

## Implementation

1. Rename the old struct to `zx_info_process_v1_t` and create an alias to refer
to it by its old name (`zx_info_process_t` -> `zx_info_process_v1_t`). Rename
the topic `ZX_INFO_PROCESS` to `ZX_INFO_PROCESS_V1` and create an alias to the
old name (`ZX_INFO_PROCESS` -> `ZX_INFO_PROCESS_V1`).

2. Add the new struct (`zx_info_process_v2_t`) and topic
(`ZX_INFO_PROCESS_V2`). Change the kernel to track process start time and
recognize the new topic and struct.

3. Update all code (in-tree and out-of-tree) to use `ZX_INFO_PROCESS_V2`.

4. Change the `ZX_INFO_PROCESS` and `zx_info_process_t` alias to refer to the
v2 topic and struct.

5. Wait until the previous change has been fully rolled out.

6. Update all code (in-tree and out-of-tree) to use `ZX_INFO_PROCESS` again.

7. Remove the v1 topic and struct, as well as the v2 alias.

The Rust and Go versions of these types will be similarly updated.

## Performance

The added logic should incur a negligible runtime cost, especially as it would
be amortized over the lifetime of an individual process.

## Security considerations

If a program already has a handle to a process, it can do and glean much more
than the time at which that process started. Moreover, start time is already
exposed - without seeming concern - in a number of [other operating systems](#prior-art-and-references).

## Privacy considerations

See [Security considerations](#security-considerations) above.

## Testing

Zircon's process-related core tests will be extended to assert, for example,
that time samples taken before and after a `zx_process_start()` do indeed
sandwich the recorded start time, and that a not-yet-started process has
`ZX_INFO_PROCESS_FLAG_STARTED` unset.

## Documentation

`ZX_INFO_PROCESS` documentation will be updated accordingly.

## Drawbacks, alternatives, and unknowns

Also considered was exposing the "start" time as a more general, task-level
concept (exposed under, say, `ZX_INFO_TASK_RUNTIME`). For threads, things
extend indentically: a thread's start time is the point at which
`zx_thread_start()` is called. For a job, it would then be most natural to say
that it "starts" when its first child starts. Since jobs can be nested,
however, this would introduce walking task subtrees, which is relatively more
complicated logic than dealing with the other two task types. Ultimately the
extensions were decided against, as they would not meet a current need and
would allow us to sidestep the need for task hierarchy walking.

## Prior art and references

Linux:  [`/proc/[pid]/stat`](https://man7.org/linux/man-pages/man5/procfs.5.html) exposes `starttime`.
FreeBSD: [`/proc/[pid]/status`](https://www.freebsd.org/cgi/man.cgi?query=procfs) exposes start time in a space-separated list of statistics.
macOS: The `proc_pidinfo()` syscalls seems to expose this.
...
