# zx_resource_create

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Create a resource object.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_resource_create(zx_handle_t parent_rsrc,
                               uint32_t options,
                               uint64_t base,
                               size_t size,
                               const char* name,
                               size_t name_size,
                               zx_handle_t* resource_out);
```

## DESCRIPTION

`zx_resource_create()` creates a resource object for use with other DDK
syscalls. Resources are typically handed out to bus drivers and rarely need to
be interacted with directly by drivers using driver protocols. Resource objects
grant access to an address space range starting at *base* up to but not
including *base* + *size*. Two special values for *kind* exist:
**ZX_RSRC_KIND_ROOT** and **ZX_RSRC_KIND_HYPERVISOR**. These resources have no
range associated with them and are used as a privilege check.

*parent_rsrc* must be a handle to a resource of *kind* **ZX_RSRC_KIND_ROOT**, or
a resource that matches the requested *kind* and contains [*base*, *base*+size*]
in its range.

*options* must specify which kind of resource to create and may contain optional
flags. Valid kinds of resources are **ZX_RSRC_KIND_MMIO**, **ZX_RSRC_KIND_IRQ**,
**ZX_RSRC_KIND_IOPORT** (x86 only), **ZX_RSRC_KIND_ROOT**,
**ZX_RSRC_KIND_HYPERVISOR**, **ZX_RSRC_KIND_VMEX**, and **ZX_RSRC_KIND_SMC**
(ARM only).
**ZX_RSRC_KIND_ROOT**, **ZX_RSRC_KIND_HYPERVISOR**, and **ZX_RSRC_KIND_VMEX**
must be paired with zero values for *base* and *size*, as they do not use
an address space range.
At this time the only optional flag is **ZX_RSRC_FLAG_EXCLUSIVE**. If
**ZX_RSRC_FLAG_EXCLUSIVE** is provided then the syscall will attempt to
exclusively reserve the requested address space region, preventing other
resources creation from overlapping with it as long as it exists.

*name* and *name_size* are optional and truncated to **ZX_MAX_NAME_LENGTH** - 1.
This name is provided for debugging / tool use only and is not used by the
kernel.

On success, a valid resource handle is returned in *resource_out*.

## RETURN VALUE

`zx_resource_create()` returns **ZX_OK** on success. In the event of failure, a
negative error value is returned.

The returned handle will have **ZX_RIGHT_TRANSFER** (allowing it to be sent to
another process via [`zx_channel_write()`]), **ZX_RIGHT_DUPLICATE** (allowing
the handle to be duplicated), **ZX_RIGHT_INSPECT** (to allow inspection of the
object with [`zx_object_get_info()`] and **ZX_RIGHT_WRITE** which is checked by
`zx_resource_create()` itself.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*parent_rsrc* must be of type **ZX_OBJ_TYPE_RESOURCE** and have **ZX_RIGHT_WRITE**.

## ERRORS

**ZX_ERR_BAD_HANDLE** the *parent_rsrc* handle is invalid.

**ZX_ERR_WRONG_TYPE** the *parent_rsrc* handle is not a resource handle.

**ZX_ERR_ACCESS_DENIED** The *parent_rsrc* handle is not a resource of either
*kind* or **ZX_RSRC_KIND_ROOT**.

**ZX_ERR_INVALID_ARGS** *options* contains an invalid kind or flag combination,
*name* is an invalid pointer, or the *kind* specified is one of
**ZX_RSRC_KIND_ROOT** or **ZX_RSRC_KIND_HYPERVISOR** but *base* and *size* are
not 0.

**ZX_ERR_NO_MEMORY** Failure due to lack of memory. There is no good way for
userspace to handle this (unlikely) error. In a future build this error will no
longer occur.

## SEE ALSO

 - [`zx_handle_close()`]
 - [`zx_interrupt_create()`]
 - [`zx_ioports_request()`]
 - [`zx_vmo_create_physical()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_channel_write()`]: channel_write.md
[`zx_handle_close()`]: handle_close.md
[`zx_interrupt_create()`]: interrupt_create.md
[`zx_ioports_request()`]: ioports_request.md
[`zx_object_get_info()`]: object_get_info.md
[`zx_vmo_create_physical()`]: vmo_create_physical.md
