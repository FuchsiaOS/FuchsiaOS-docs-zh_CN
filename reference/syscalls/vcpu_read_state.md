# zx_vcpu_read_state

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Read the state of a VCPU.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vcpu_read_state(zx_handle_t handle,
                               uint32_t kind,
                               void* buffer,
                               size_t buffer_size);
```

## DESCRIPTION

`zx_vcpu_read_state()` reads the state of *handle* as specified by *kind* into
*buffer*. It is only valid to read the state of *handle* when execution has been
paused.

*kind* must be **ZX_VCPU_STATE**.

`zx_vcpu_read_state()` must be called on the same thread *handle* was created
on.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_VCPU** and have **ZX_RIGHT_READ**.

## RETURN VALUE

`zx_vcpu_read_state()` returns **ZX_OK** on success. On failure, an error value
is returned.

## ERRORS

**ZX_ERR_ACCESS_DENIED** *handle* does not have the **ZX_RIGHT_READ** right.

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_BAD_STATE** *handle* is in a bad state, and state can not be read.

**ZX_ERR_INVALID_ARGS** *kind* does not name a known VCPU state, *buffer* is an
invalid pointer, or *buffer_size* does not match the expected size of *kind*.

**ZX_ERR_WRONG_TYPE** *handle* is not a handle to a VCPU.

## SEE ALSO

 - [`zx_guest_create()`]
 - [`zx_guest_set_trap()`]
 - [`zx_vcpu_create()`]
 - [`zx_vcpu_enter()`]
 - [`zx_vcpu_interrupt()`]
 - [`zx_vcpu_kick()`]
 - [`zx_vcpu_write_state()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_guest_create()`]: guest_create.md
[`zx_guest_set_trap()`]: guest_set_trap.md
[`zx_vcpu_create()`]: vcpu_create.md
[`zx_vcpu_enter()`]: vcpu_enter.md
[`zx_vcpu_interrupt()`]: vcpu_interrupt.md
[`zx_vcpu_kick()`]: vcpu_kick.md
[`zx_vcpu_write_state()`]: vcpu_write_state.md
