# zx_vcpu_resume

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Resume execution of a VCPU.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>
#include <zircon/syscalls/port.h>

zx_status_t zx_vcpu_resume(zx_handle_t handle, zx_port_packet_t* packet);
```

## DESCRIPTION

`zx_vcpu_resume()` begins or resumes execution of *handle*, and blocks until it
has paused execution. On pause of execution, *packet* is populated with reason
for the pause. After handling the reason, execution may be resumed by calling
`zx_vcpu_resume()` again.

`zx_vcpu_resume()` must be called on the same thread *handle* was created on.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_VCPU** and have **ZX_RIGHT_EXECUTE**.

## RETURN VALUE

`zx_vcpu_resume()` returns **ZX_OK** on success. On failure, an error value is
returned.

## ERRORS

**ZX_ERR_ACCESS_DENIED** *handle* does not have the **ZX_RIGHT_EXECUTE** right.

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_BAD_STATE** *handle* is in a bad state, and can not be executed.

**ZX_ERR_CANCELED** *handle* execution was canceled while waiting on an event.

**ZX_ERR_INTERNAL** There was an error executing *handle*.

**ZX_ERR_INVALID_ARGS** *packet* is an invalid pointer.

**ZX_ERR_NOT_SUPPORTED** An unsupported operation was encountered while
executing *handle*.

**ZX_ERR_WRONG_TYPE** *handle* is not a handle to a VCPU.

## SEE ALSO

 - [`zx_guest_create()`]
 - [`zx_guest_set_trap()`]
 - [`zx_vcpu_create()`]
 - [`zx_vcpu_interrupt()`]
 - [`zx_vcpu_read_state()`]
 - [`zx_vcpu_write_state()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_guest_create()`]: guest_create.md
[`zx_guest_set_trap()`]: guest_set_trap.md
[`zx_vcpu_create()`]: vcpu_create.md
[`zx_vcpu_interrupt()`]: vcpu_interrupt.md
[`zx_vcpu_read_state()`]: vcpu_read_state.md
[`zx_vcpu_write_state()`]: vcpu_write_state.md
