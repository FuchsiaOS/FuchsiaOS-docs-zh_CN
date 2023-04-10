<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0178" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes changing `zx_task_create_exception_channel` to allow up to 32
debug exception channels to be created on one job.

## Motivation

Debuggers on Fuchsia depend on creating debug exception channels on jobs to
monitor process starts, so that they can capture the process of interest and
attach to it eagerly.  The debug exception channel is usually created on the
root job of the system, so that debuggers can monitor all processes.

When a process is starting up, the kernel will walk the job hierarchy looking
for a debug job exception channel to notify.  This walk starts with the new
process's containing job and goes up the hierarchy until a debug job exception
channel is found or the root job is reached.

However, there are two drawbacks in today's implementation.

- For a given job, there can at most one debug exception channel, which means at
  most one debugger can monitor any given subtree of a job hierarchy.
- Only the first found debug exception channel will be notified, i.e., the
  debugger on a child job prevents the debugger on the parent job from observing
  process starts.

## Stakeholders

_Facilitator:_ cpu@google.com

_Reviewers:_ brettw@google.com, maniscalco@google.com

_Consulted:_ johngro@google.com

_Socialization:_ The idea was discussed in Fuchsia's Kernel Evolution Working
Group.

## Design

We propose that Zircon allow multiple debug job exception channels on one job.

There are [5 types of exception channels][exception_channel_types] in Zircon:
debug job, job, debug process, process, and thread.  Each of them can be created
at most once on the corresponding object.  This restriction is set to avoid
tricky situations when e.g. multiple exception handlers set different values for
`ZX_PROP_EXCEPTION_STATE`.

However, "debug job" is distinctive here because it's a notification-only
channel: the only exception type it can receive is `ZX_EXCP_PROCESS_STARTING`
where the `ZX_PROP_EXCEPTION_STATE` is ignored.  Thus it's possible to allow
multiple debug exception channels on one job without worrying about
inconsistencies.

With multiple debug job exception channels created on one job, a
`ZX_EXCP_PROCESS_STARTING` event will be sent to all channels sequentially,
allowing multiple listeners to inspect the process.  The restriction that only
one debugger can attach to a given process doesn't change, as the "debug
process" exception channel is still solely owned and all debuggers except the
first one will get a `ZX_ERR_ALREADY_BOUND`.

In addition, we also propose modifying the event propagation for
`ZX_EXCP_PROCESS_STARTING` events so that one event will propagate all the way
to the root even if there's an exception channel created on a child job.  Debug
job exception channels at lower levels in the job hierarchy will be notified
before those at higher levels.  At any given level, the earlier created channels
will be notified before the later created channels.  The process can continue
starting only when all exception channels are notified and the exception objects
are closed.

Note, it's possible for any exception message receiver to hold up process
startup indefinitely by simply not closing the handle to the exception object.
However, they cannot outright stop process startup.

## Implementation

There'll be no API or ABI change to the syscalls.  The existing behavior of
`zx_task_create_exception_channel` will be changed to allow up to `N` channels
to be created instead of returning `ZX_ERR_ALREADY_BOUND` after the first one.

## Performance

The performance could decrease because more listeners could possibly block
process starts.  However, the debug exception channel is not expected to be used
in a production environment or be held by a long-running program.  It's intended
only for debuggers or similar debugging tools so the impact should be minimal.

As a general rule, the debuggers should close the exception handle immediately
after necessary setup is performed.

## Security considerations

To avoid DoS attacks against the kernel, we should limit the maximum number of
the debug job exception channels created on one job.  The limit should be large
enough to allow any reasonable number of debuggers to run, for example, 32.

## Testing

New test cases will be added to `//zircon/system/utest/debugger` to cover this
feature.

## Documentation

The documents describing [Exception Handling][exception_handling] and
[`zx_task_create_exception_channel`][task_create_exception_channel] will be
updated to reflect the change.

## Drawbacks, alternatives, and unknowns

### Alternative: User-space delegate of process starts on the root job

This method avoids changes to the kernel.  Instead, a user-space delegate will
hold the root job debug channel and provide a FIDL interface for debuggers to
subscribe to process starts.  The delegate could be the component manager
itself, who serves the `fuchsia.kernel.RootJob` today, or a standalone program.

The protocol will look like

```fidl
@discoverable
protocol RootJob {
    /// Hanging get pattern
    GetProcessStartingEvent() -> (resource struct {
        process zx.handle:PROCESS;
        continue zx.handle:EVENTPAIR;  // dropping this eventpair will propagate
                                       // the event to the next listener.
    }) error zx.status;
};
```
The problems with this method are

- It only applies to the root job.  The debugger needs two logics for root job
  and non-root job.
- An exception channel created on a child job can stop events from propagating
  to the root.
- It's confusing to introduce a FIDL API with functions similarly provided by a
  syscall.

### Alternative: debug_agent supports multiple clients

We can also make debug_agent support multiple clients to solve today's problem,
i.e., the debug_agent becomes a singleton.

The problems are

- More work will be involved.
- debug_agent will become the designated debugger for Fuchsia.  Other debuggers
  have to talk with the debug_agent, which will expose a large and unstable
  interface.

[exception_channel_types]: ../../../concepts/kernel/exceptions.md#exception_channel_types
[exception_handling]: ../../../concepts/kernel/exceptions.md
[task_create_exception_channel]: ../../../reference/syscalls/task_create_exception_channel.md
