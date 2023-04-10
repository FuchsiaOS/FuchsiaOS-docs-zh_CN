{% set rfcid = "RFC-0091" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }} - {{ rfc.title }}
<!-- *** DO NOT EDIT ABOVE THIS LINE -->

## Summary

This RFC proposes adding a new event type -
`ZX_SYSTEM_EVENT_IMMINENT_OUT_OF_MEMORY`, to the `zx_system_get_event()`
syscall.  The new kernel event will be signaled prior to
`ZX_SYSTEM_EVENT_OUT_OF_MEMORY`, to facilitate capturing memory diagnostics
close to the point of OOM.

## Motivation

`ZX_SYSTEM_EVENT_OUT_OF_MEMORY` is signaled when the system has almost run out
of free physical memory to be able to fulfill memory allocation requests. The
`driver_manager` process waits on this OOM event, and when it is signaled asks
`fshost` to shut down filesystems, so that the device can be rebooted as cleanly
as possible. There exists no good mechanism to capture meaningful memory state
at this point though, which is critical in order to root cause the OOM.

Simply waiting on the OOM event, and trying to write out a memory report at the
point of the OOM is not feasible, because that would race with filesystems
shutting down. So instead of relying on `ZX_SYSTEM_EVENT_OUT_OF_MEMORY`, this
RFC proposes exposing a new event `ZX_SYSTEM_EVENT_IMMINENT_OUT_OF_MEMORY` which
will be signaled as close to the OOM as possible, while trying to provide some
headroom before the OOM so that a memory report can be reliably written out. For
example, if `ZX_SYSTEM_EVENT_OUT_OF_MEMORY` is signaled when the system free
memory level drops to 50MB, `ZX_SYSTEM_EVENT_IMMINENT_OUT_OF_MEMORY` can be
signaled at 60MB of free memory.

There is also a desire to perform orderly shutdown on OOM in the long term,
which would involve a higher level userspace component (not `driver_manager`)
waiting on the `ZX_SYSTEM_EVENT_OUT_OF_MEMORY` event and shutting down certain
key components as well as filesystems. That is a larger scope project, at which
point we will have to revisit the OOM event signaling and reporting code paths.
As such the proposal in this RFC can be seen as an intermediate step, requiring
minimal changes to the existing design. The `memory_monitor` process already
waits on the Normal, Warning, and Critical events and generates memory reports
when required, so it can be easily extended to wait on an Imminent-OOM event.
This event might be removed in the future if report generation is handled by
orderly shutdown on OOM, or we might decide to still keep it around if the
report provides additional value.

## Design

The kernel memory reclamation system signals memory pressure events based on
three memory watermarks - Warning, Critical, and OOM. This logic can easily be
extended to include a new Imminent-OOM watermark between Critical and OOM. The
delta between the Imminent-OOM and OOM watermarks can be configured by a kernel
commandline option.

To maintain consistency with the existing memory watermarks, which can also be
configured via the kernel commandline, this delta will be an absolute amount of
memory in MB. That also means that a single value will likely not be feasible
for all platforms, and will require tuning, which is a property shared by
existing memory watermarks as well. Also, memory report generation itself will
require a certain amount of memory, which should also be taken into account when
picking a value for this delta.

As with other kernel events supported by `zx_system_get_event()`, a handle to
the root job will be required. Unlike `ZX_SYSTEM_EVENT_OUT_OF_MEMORY`,
`ZX_SYSTEM_EVENT_IMMINENT_OUT_OF_MEMORY` will not require the
`ZX_RIGHT_MANAGE_PROCESS` right. This makes the new event similar to
`ZX_SYSTEM_EVENT_MEMORY_PRESSURE_NORMAL`,
`ZX_SYSTEM_EVENT_MEMORY_PRESSURE_WARNING` and
`ZX_SYSTEM_EVENT_MEMORY_PRESSURE_CRITICAL`, which also do not require the
`ZX_RIGHT_MANAGE_PROCESS` right. In other words, Normal, Warning, Critical, and
Imminent-OOM events can use the `RootJobForInspect` protocol, while the OOM
event requires the `RootJob` protocol.

Implementation in the kernel will require minimal changes to the existing memory
reclamation logic.  We are simply adding another watermark to the list of
watermarks passed to `pmm_init_reclamation()`.  The only aspect where the
Imminent-OOM level differs from the other memory pressure levels, is that it
will not trigger any memory reclamation in the kernel (eviction of pager backed
memory and discardable memory). This level is intended purely as a diagnostic
level; triggering memory reclamation at this point would alter the memory state
the event is meant to capture in the first place. As a result, this new event
will not have a corresponding signal in the `fuchsia.memorypressure` protocol,
which is used to drive memory reclamation in userspace.

## Implementation

The `zx_system_get_event()` syscall can be extended in a single CL; waiters on
existing kernel events will not be affected in any way with this change. Waiters
on the new event can later adopt it in a separate CL.

## Performance

The new event will not affect the performance of waiters of the existing
event. The new event is also expected to have similar performance
characteristics as the existing ones, both for retrieving the event handle and
for waiting on it. Kernel performance for signaling the event will also be
similar to the existing ones - when the free memory level changes, the PMM
system finds the closest memory watermark in order to infer the memory pressure
level; the proposal is to simply add a new watermark without changing the rest
of the existing logic.

## Ergonomics

None.

## Backwards Compatibility

None.

## Security considerations

None.

## Privacy considerations

None.

## Testing

Zircon core-tests will be written that retrieve and wait on
`ZX_SYSTEM_EVENT_IMMINENT_OUT_OF_MEMORY`. Unit tests to verify client side
behavior will also be written when the event is adopted.

## Documentation

The syscall documentation for `zx_system_get_event()` will need to be updated.
An entry will also be added to the kernel concepts documentation explaining the
motivation behind the various memory pressure levels, the response that is
expected at each, and how they tie in with the rest of the system.

## Drawbacks, alternatives, and unknowns

### Drawback: Memory reports might not be precise

Since the event is signaled prior to OOM, the memory state captured will not
coincide exactly with the OOM. However, as long as the delta from the OOM
watermark is small enough (a few tens of MB), that should still prove useful in
diagnosing the OOM.

We might see cases where a report is generated without an OOM following it,
i.e. if the device is somehow able to recover after the Imminent-OOM event. Even
though there will not be an OOM event associated with such a report, it is
useful memory data to have nevertheless, as it still points to increased memory
usage that is worth investigating.

It is also possible for the waiter of the event to not be able to generate
reports in the case of rapid memory allocation spikes. Such cases will still
have to be diagnosed by reproducing the issue locally while gathering memory
metrics in parallel. Reports should still be able to capture memory diagnostics
for a majority of OOMs, since very rapid allocation spikes are rare.

### Alternative 1: Using existing events instead of a new one

`ZX_SYSTEM_EVENT_OUT_OF_MEMORY` cannot be used to reliably write out a memory
report. This is because the OOM event is used to drive filesystem shutdown, so
we might not be able to write out the report in time.

This interaction could be changed by moving the OOM signal handling out from
`driver_manager` to another userspace component that can drive a more orderly
shutdown - first writing out a report, and then shutting down filesystems. This
approach would need some synchronization to make sure the report is successfully
written out before filesystems are shut down. It might also require us to relax
the OOM watermark and the reboot delay interval after sending out the OOM
signal, since we would now be performing more work when the OOM event is
signaled. This change to how the OOM signal is handled ties in with the larger
effort of orderly shutdown on OOM (briefly described in the Motivation section),
which is a larger scope effort that might be undertaken in the future.

The other existing event that is signaled prior to OOM is
`ZX_SYSTEM_EVENT_MEMORY_PRESSURE_CRITICAL`, which we do use to write memory
reports. However, the Critical memory watermark is far enough from the OOM
watermark that these reports can miss important memory state changes closer to
the OOM.

### Alternative 2: Richer API for more diagnostic events

Instead of exposing a specialized event close to OOM, we could have an alternate
API where the user can specify multiple custom diagnostic memory levels to be
notified at.  This allows for a more flexible API which is easier to extend in
the future.  However, no concrete use cases exist at this time indicating that
such an API would be useful.

The other memory pressure levels where it might be useful to capture diagnostic
information are Warning and Critical. For these levels, we can directly use the
kernel events signaling the level change. We already do this for the Critical
level. The OOM level is special in this respect, because it is not feasible to
capture diagnostics exactly at the point of the OOM. Capturing state at the
point of the OOM is what we would want in the ideal case. Since that is not
possible, we need another signal close enough to the OOM to be able to capture
relevant state, but not so close that we run into a similar problem and are not
able to write a report.

## Prior art and references

The [`zx_system_get_event`](/docs/reference/syscalls/system_get_event.md)
docs describe how the kernel memory events work. The
[`fuchsia.memorypressure`](https://fuchsia.dev/reference/fidl/fuchsia.memorypressure)
docs describe the userspace memory pressure signals that are built on top of the
kernel events.

