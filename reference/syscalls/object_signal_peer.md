# zx_object_signal_peer

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Signal an object's peer.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_object_signal_peer(zx_handle_t handle,
                                  uint32_t clear_mask,
                                  uint32_t set_mask);
```

## DESCRIPTION

`zx_object_signal_peer()` asserts and deasserts the userspace-accessible
signal bits on the object's peer. A object peer is the opposite endpoint of a
*channel*, *socket*, *fifo*, or *eventpair*.

Most of the 32 signals are reserved for system use and are assigned to
per-object functions, like **ZX_CHANNEL_READABLE** or **ZX_TASK_TERMINATED**. There
are 8 signal bits available for userspace processes to use as they see fit:
**ZX_USER_SIGNAL_0** through **ZX_USER_SIGNAL_7**.

*Eventpair* objects also allow control over the **ZX_EVENTPAIR_SIGNALED** bit.

The *clear_mask* is first used to clear any bits indicated, and then the
*set_mask* is used to set any bits indicated.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must have **ZX_RIGHT_SIGNAL_PEER**.

## RETURN VALUE

`zx_object_signal_peer()` returns **ZX_OK** on success. In the event of
failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_ACCESS_DENIED**  *handle* lacks the right **ZX_RIGHT_SIGNAL_PEER**.

**ZX_ERR_INVALID_ARGS**  *clear_mask* or *set_mask* contain bits that are not allowed.

**ZX_ERR_NOT_SUPPORTED**  Used on an object lacking a peer.

**ZX_ERR_PEER_CLOSED**  Called on an object with a closed peer.

## SEE ALSO

 - [`zx_eventpair_create()`]
 - [`zx_object_signal()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_event_create()`]: event_create.md
[`zx_eventpair_create()`]: eventpair_create.md
[`zx_object_signal()`]: object_signal.md
