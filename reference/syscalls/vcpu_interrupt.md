# zx_vcpu_interrupt

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Raise an interrupt on a VCPU.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vcpu_interrupt(zx_handle_t handle, uint32_t vector);
```

## DESCRIPTION

`zx_vcpu_interrupt()` raises an interrupt of *vector* on *handle*, and may be
called from any thread.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_VCPU** and have **ZX_RIGHT_SIGNAL**.

## RETURN VALUE

`zx_vcpu_interrupt()` returns **ZX_OK** on success. On failure, an error value is
returned.

## ERRORS

**ZX_ERR_ACCESS_DENIED** *handle* does not have the **ZX_RIGHT_SIGNAL** right.

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_OUT_OF_RANGE** *vector* is outside of the range interrupts supported by
the current architecture.

**ZX_ERR_WRONG_TYPE** *handle* is not a handle to a VCPU.

## SEE ALSO

 - [`zx_guest_create()`]
 - [`zx_guest_set_trap()`]
 - [`zx_vcpu_create()`]
 - [`zx_vcpu_read_state()`]
 - [`zx_vcpu_resume()`]
 - [`zx_vcpu_write_state()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_guest_create()`]: guest_create.md
[`zx_guest_set_trap()`]: guest_set_trap.md
[`zx_vcpu_create()`]: vcpu_create.md
[`zx_vcpu_read_state()`]: vcpu_read_state.md
[`zx_vcpu_resume()`]: vcpu_resume.md
[`zx_vcpu_write_state()`]: vcpu_write_state.md
