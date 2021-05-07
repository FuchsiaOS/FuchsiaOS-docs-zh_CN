# zx_interrupt_destroy

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Destroys an interrupt object.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_interrupt_destroy(zx_handle_t handle);
```

## DESCRIPTION

`zx_interrupt_destroy()` "destroys" an interrupt object, putting it in a state
where any [`zx_interrupt_wait()`] operations on it will return **ZX_ERR_CANCELED**,
and it is unbound from any ports it was bound to.

This provides a clean shut down mechanism.  Closing the last handle to the
interrupt object results in similar cancellation but could result in use-after-close
of the handle.

If the interrupt object is bound to a port when cancellation happens, if it
has not yet triggered, or it has triggered but the packet has not yet been
received by a caller of [`zx_port_wait()`], success is returned and any packets
in flight are removed.  Otherwise, **ZX_ERR_NOT_FOUND** is returned, indicating
that the packet has been read but the interrupt has not been re-armed by calling
[`zx_interrupt_ack()`].

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_interrupt_destroy()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_WRONG_TYPE** *handle* is not an interrupt object.

**ZX_ERR_NOT_FOUND**  *handle* was bound (and now no longer is) but was not
being waited for.

**ZX_ERR_ACCESS_DENIED** *handle* lacks **ZX_RIGHT_WRITE**.

## SEE ALSO

 - [`zx_handle_close()`]
 - [`zx_interrupt_ack()`]
 - [`zx_interrupt_bind()`]
 - [`zx_interrupt_create()`]
 - [`zx_interrupt_trigger()`]
 - [`zx_interrupt_wait()`]
 - [`zx_port_wait()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_handle_close()`]: handle_close.md
[`zx_interrupt_ack()`]: interrupt_ack.md
[`zx_interrupt_bind()`]: interrupt_bind.md
[`zx_interrupt_create()`]: interrupt_create.md
[`zx_interrupt_trigger()`]: interrupt_trigger.md
[`zx_interrupt_wait()`]: interrupt_wait.md
[`zx_port_wait()`]: port_wait.md
