# zx_object_signal

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Signal an object.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_object_signal(zx_handle_t handle,
                             uint32_t clear_mask,
                             uint32_t set_mask);
```

## DESCRIPTION

`zx_object_signal()` asserts and deasserts the userspace-accessible signal
bits on an object.

Most of the 32 signals are reserved for system use and are assigned to
per-object functions, like **ZX_CHANNEL_READABLE** or **ZX_TASK_TERMINATED**. There
are 8 signal bits available for userspace processes to use as they see fit:
**ZX_USER_SIGNAL_0** through **ZX_USER_SIGNAL_7**.

*Event* objects also allow control over the **ZX_EVENT_SIGNALED** bit.

*Eventpair* objects also allow control over the **ZX_EVENTPAIR_SIGNALED** bit.

The *clear_mask* is first used to clear any bits indicated, and then the
*set_mask* is used to set any bits indicated.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must have **ZX_RIGHT_SIGNAL**.

## RETURN VALUE

`zx_object_signal()` returns **ZX_OK** on success. In the event of failure, a
negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_ACCESS_DENIED**  *handle* lacks the right **ZX_RIGHT_SIGNAL**.

**ZX_ERR_INVALID_ARGS**  *clear_mask* or *set_mask* contain bits that are not allowed.

## SEE ALSO

 - [`zx_event_create()`]
 - [`zx_eventpair_create()`]
 - [`zx_object_signal_peer()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_event_create()`]: event_create.md
[`zx_eventpair_create()`]: eventpair_create.md
[`zx_object_signal_peer()`]: object_signal_peer.md
