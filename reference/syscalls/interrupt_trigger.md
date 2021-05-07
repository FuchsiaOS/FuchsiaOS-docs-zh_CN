# zx_interrupt_trigger

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Triggers a virtual interrupt object.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_interrupt_trigger(zx_handle_t handle,
                                 uint32_t options,
                                 zx_time_t timestamp);
```

## DESCRIPTION

`zx_interrupt_trigger()` is used to trigger a virtual interrupt interrupt object,
causing an interrupt message packet to arrive on the bound port, if it is bound
to a port, or [`zx_interrupt_wait()`] to return if it is waiting on this interrupt.

*options* must be zero.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_INTERRUPT** and have **ZX_RIGHT_SIGNAL**.

## RETURN VALUE

`zx_interrupt_trigger()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_WRONG_TYPE** *handle* is not an interrupt object.

**ZX_ERR_BAD_STATE** *handle* is not a virtual interrupt.

**ZX_ERR_CANCELED**  [`zx_interrupt_destroy()`] was called on *handle*.

**ZX_ERR_ACCESS_DENIED** *handle* lacks **ZX_RIGHT_SIGNAL**.

**ZX_ERR_INVALID_ARGS** *options* is non-zero.

## SEE ALSO

 - [`zx_handle_close()`]
 - [`zx_interrupt_ack()`]
 - [`zx_interrupt_bind()`]
 - [`zx_interrupt_create()`]
 - [`zx_interrupt_destroy()`]
 - [`zx_interrupt_wait()`]
 - [`zx_port_wait()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_handle_close()`]: handle_close.md
[`zx_interrupt_ack()`]: interrupt_ack.md
[`zx_interrupt_bind()`]: interrupt_bind.md
[`zx_interrupt_create()`]: interrupt_create.md
[`zx_interrupt_destroy()`]: interrupt_destroy.md
[`zx_interrupt_wait()`]: interrupt_wait.md
[`zx_port_wait()`]: port_wait.md
