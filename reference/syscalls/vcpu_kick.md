# zx_vcpu_kick

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Kick a VCPU, causing it to stop execution.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vcpu_kick(zx_handle_t handle);
```

## DESCRIPTION

`zx_vcpu_kick()` forces the current or next execution of `zx_vcpu_enter()` on
*handle* to return immediately with **ZX_ERR_CANCELED**.

`zx_vcpu_kick()` may be called multiple times on *handle*, but will only affect
the current or next execution of `zx_vcpu_enter()`.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_VCPU** and have **ZX_RIGHT_EXECUTE**.

## RETURN VALUE

`zx_vcpu_kick()` returns **ZX_OK** on success. On failure, an error value is
returned.

## ERRORS

**ZX_ERR_ACCESS_DENIED** *handle* does not have the **ZX_RIGHT_EXECUTE** right.

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_WRONG_TYPE** *handle* is not a handle to a VCPU.

## SEE ALSO

 - [`zx_guest_create()`]
 - [`zx_guest_set_trap()`]
 - [`zx_vcpu_create()`]
 - [`zx_vcpu_enter()`]
 - [`zx_vcpu_interrupt()`]
 - [`zx_vcpu_read_state()`]
 - [`zx_vcpu_write_state()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_guest_create()`]: guest_create.md
[`zx_guest_set_trap()`]: guest_set_trap.md
[`zx_vcpu_create()`]: vcpu_create.md
[`zx_vcpu_enter()`]: vcpu_enter.md
[`zx_vcpu_interrupt()`]: vcpu_interrupt.md
[`zx_vcpu_read_state()`]: vcpu_read_state.md
[`zx_vcpu_write_state()`]: vcpu_write_state.md
