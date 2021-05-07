# zx_guest_create

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Create a guest.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_guest_create(zx_handle_t resource,
                            uint32_t options,
                            zx_handle_t* guest_handle,
                            zx_handle_t* vmar_handle);
```

## DESCRIPTION

`zx_guest_create()` creates a guest, which is a virtual machine that can be run
within the hypervisor, with *vmar_handle* used to represent the physical address
space of the guest.

To create a guest, a *resource* of **ZX_RSRC_KIND_HYPERVISOR** must be supplied.

In order to begin execution within the guest, a VMO should be mapped into
*vmar_handle* using [`zx_vmar_map()`], and a VCPU must be created using
[`zx_vcpu_create()`], and then run using [`zx_vcpu_resume()`].

Additionally, a VMO should be mapped into *vmar_handle* to provide a guest with
physical memory.

The following rights will be set on the handle *guest_handle* by default:

**ZX_RIGHT_TRANSFER** &mdash; *guest_handle* may be transferred over a channel.

**ZX_RIGHT_DUPLICATE** &mdash; *guest_handle* may be duplicated.

**ZX_RIGHT_WRITE** &mdash; A trap to be may be set using [`zx_guest_set_trap()`].

**ZX_RIGHT_MANAGE_PROCESS** &mdash; A VCPU may be created using [`zx_vcpu_create()`].

See [`zx_vmo_create()`] for the set of rights applied to *vmar_handle*.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*resource* must have resource kind **ZX_RSRC_KIND_HYPERVISOR**.

## RETURN VALUE

`zx_guest_create()` returns **ZX_OK** on success. On failure, an error value is
returned.

## ERRORS

**ZX_ERR_ACCESS_DENIED** *resource* is not of **ZX_RSRC_KIND_HYPERVISOR**.

**ZX_ERR_INVALID_ARGS** *guest_handle* or *vmar_handle* is an invalid pointer,
or *options* is nonzero.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_WRONG_TYPE** *resource* is not a handle to a resource.

## SEE ALSO

 - [`zx_guest_set_trap()`]
 - [`zx_vcpu_create()`]
 - [`zx_vcpu_interrupt()`]
 - [`zx_vcpu_read_state()`]
 - [`zx_vcpu_resume()`]
 - [`zx_vcpu_write_state()`]
 - [`zx_vmar_map()`]
 - [`zx_vmo_create()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_guest_set_trap()`]: guest_set_trap.md
[`zx_vcpu_create()`]: vcpu_create.md
[`zx_vcpu_interrupt()`]: vcpu_interrupt.md
[`zx_vcpu_read_state()`]: vcpu_read_state.md
[`zx_vcpu_resume()`]: vcpu_resume.md
[`zx_vcpu_write_state()`]: vcpu_write_state.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmo_create()`]: vmo_create.md
