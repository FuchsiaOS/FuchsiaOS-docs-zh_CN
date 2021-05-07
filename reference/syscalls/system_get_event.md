# zx_system_get_event

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Retrieve a handle to a system event.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_system_get_event(zx_handle_t root_job,
                                uint32_t kind,
                                zx_handle_t* event);
```

## DESCRIPTION

*root_job* must be a handle to the root job of the system.

*kind* must be one of the following:

- **ZX_SYSTEM_EVENT_OUT_OF_MEMORY** - An *event* will be returned that will
assert ZX_EVENT_SIGNALED when the system is in an out-of-memory situation.
A process that is waiting on this event must quickly perform any important
shutdown work. It is unspecified how much memory is available at the time this
event is signaled, and unspecified how long the waiting process has to act
before the kernel starts terminating processes or starting a full system reboot.

- **ZX_SYSTEM_EVENT_IMMINENT_OUT_OF_MEMORY** - An *event* will be returned that
will assert ZX_EVENT_SIGNALED when the system is nearing an out-of-memory
situation. This event is signaled a little earlier than the
ZX_SYSTEM_EVENT_OUT_OF_MEMORY event. The intent of this event is to allow the
waiter to gather diagnostics related to the impending
ZX_SYSTEM_EVENT_OUT_OF_MEMORY event, since it might be too late to do so
reliably when ZX_SYSTEM_EVENT_OUT_OF_MEMORY is signaled.

- **ZX_SYSTEM_EVENT_MEMORY_PRESSURE_CRITICAL** - An *event* will be returned
that will assert ZX_EVENT_SIGNALED when available memory on the system is
critically low. The memory pressure level in this case is less severe than in
the case of ZX_SYSTEM_EVENT_OUT_OF_MEMORY. The exact amount of memory available
at the time the event is signaled is unspecified.

- **ZX_SYSTEM_EVENT_MEMORY_PRESSURE_WARNING** - An *event* will be returned
that will assert ZX_EVENT_SIGNALED when available memory on the system is
approaching the critically low range. The memory pressure level in this case is
less severe than in the case of ZX_SYSTEM_EVENT_MEMORY_PRESSURE_CRITICAL. The
exact amount of memory available at the time the event is signaled is
unspecified.

- **ZX_SYSTEM_EVENT_MEMORY_PRESSURE_NORMAL** - An *event* will be returned that
will assert ZX_EVENT_SIGNALED when available memory on the system is
healthy. The exact amount of memory available at the time the event is signaled
is unspecified.

The kernel will assert ZX_EVENT_SIGNALED on these five events in the following
order of increasing severity: ZX_SYSTEM_EVENT_MEMORY_PRESSURE_NORMAL,
ZX_SYSTEM_EVENT_MEMORY_PRESSURE_WARNING,
ZX_SYSTEM_EVENT_MEMORY_PRESSURE_CRITICAL, ZX_SYSTEM_EVENT_IMMINENT_OUT_OF_MEMORY,
and ZX_SYSTEM_EVENT_OUT_OF_MEMORY. Exactly one of these events will assert
ZX_EVENT_SIGNALED at a given time.

Both ZX_SYSTEM_EVENT_OUT_OF_MEMORY and ZX_SYSTEM_EVENT_MEMORY_PRESSURE_\*
retrieve events corresponding to system memory pressure levels, but there is a
key difference in the way these events are intended to be used. A process
waiting on the any of the ZX_SYSTEM_EVENT_MEMORY_PRESSURE_\* events must
undertake actions that free up memory and attempt to relieve the memory
pressure on the system. On the other hand, a process waiting on the
ZX_SYSTEM_EVENT_OUT_OF_MEMORY event must perform necessary actions in
preparation for a clean shutdown - at this point it is too late to attempt
recovery to a healthy memory pressure level.

The ZX_SYSTEM_EVENT_IMMINENT_OUT_OF_MEMORY event can be seen as a companion event
for ZX_SYSTEM_EVENT_OUT_OF_MEMORY, which does not trigger memory reclamation
itself, but instead is used to capture memory diagnostics what will help debug
the closely following ZX_SYSTEM_EVENT_OUT_OF_MEMORY event (if there is one).

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

None.

## RETURN VALUE

`zx_system_get_event()` returns ZX_OK on success, and *event* will be a valid
handle, or an error code from below on failure.

## ERRORS

**ZX_ERR_ACCESS_DENIED** The calling process' policy was invalid, the handle
*root_job* did not have ZX_RIGHT_MANAGE_PROCESS rights for *kind*
ZX_SYSTEM_EVENT_OUT_OF_MEMORY, *root_job* was not the
root job of the system.

**ZX_ERR_INVALID_ARGS** *kind* was not one of the supported values specified
above.
