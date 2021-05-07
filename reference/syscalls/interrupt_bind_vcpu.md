# zx_interrupt_bind_vcpu

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Bind an interrupt object to a VCPU.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_interrupt_bind_vcpu(zx_handle_t handle,
                                   zx_handle_t vcpu,
                                   uint32_t options);
```

## DESCRIPTION

`zx_interrupt_bind_vcpu()` binds an interrupt object to a VCPU. When the
interrupt object is triggered, the interrupt is redirected to the VCPU, in order
to be processed by a guest with no host intervention.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_INTERRUPT** and have **ZX_RIGHT_READ**.

*vcpu* must be of type **ZX_OBJ_TYPE_VCPU** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_interrupt_bind_vcpu()` returns **ZX_OK** on success. On failure, an error value
is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE** *handle* or *vcpu* are not valid handles.

**ZX_ERR_WRONG_TYPE** *handle* is not an interrupt object or *vcpu* is not a
VCPU.

**ZX_ERR_CANCELED** [`zx_interrupt_destroy()`] was called on *handle*.

**ZX_ERR_BAD_STATE**  a thread is waiting on the interrupt using
[`zx_interrupt_wait()`].

**ZX_ERR_ACCESS_DENIED** *handle* lacks **ZX_RIGHT_READ** or *vcpu* lacks
**ZX_RIGHT_WRITE**.

**ZX_ERR_ALREADY_BOUND** *handle* is already bound to another vcpu or to a
port.

**ZX_ERR_INVALID_ARGS** *vcpu* is bound to a different guest than previously
bound VCPUs, or *options* is non-zero.

## SEE ALSO

 - [`zx_guest_create()`]
 - [`zx_interrupt_create()`]
 - [`zx_vcpu_create()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_guest_create()`]: guest_create.md
[`zx_interrupt_create()`]: interrupt_create.md
[`zx_interrupt_destroy()`]: interrupt_destroy.md
[`zx_interrupt_wait()`]: interrupt_wait.md
[`zx_vcpu_create()`]: vcpu_create.md
