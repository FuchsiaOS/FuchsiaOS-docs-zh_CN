# zx_vmo_create_physical

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Create a VM object referring to a specific contiguous range of physical memory.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_create_physical(zx_handle_t resource,
                                   zx_paddr_t paddr,
                                   size_t size,
                                   zx_handle_t* out);
```

## DESCRIPTION

`zx_vmo_create_physical()` creates a new virtual memory object (VMO), which represents the
*size* bytes of physical memory beginning at physical address *paddr*.

The content size of the VMO will be initialized to the given (unrounded) size.
Use [`zx_object_get_property()`] with **ZX_PROP_VMO_CONTENT_SIZE** to read the
content size of the VMO. Use [`zx_object_set_property()`] with
**ZX_PROP_VMO_CONTENT_SIZE** to set the content size of the VMO without
actually resizing the VMO.

One handle is returned on success, representing an object with the requested
size.

The following rights will be set on the handle by default:

**ZX_RIGHT_DUPLICATE** - The handle may be duplicated.

**ZX_RIGHT_TRANSFER** - The handle may be transferred to another process.

**ZX_RIGHT_READ** - May be read from or mapped with read permissions.

**ZX_RIGHT_WRITE** - May be written to or mapped with write permissions.

**ZX_RIGHT_EXECUTE** - May be mapped with execute permissions.

**ZX_RIGHT_MAP** - May be mapped.

**ZX_RIGHT_GET_PROPERTY** - May get its properties using
[`zx_object_get_property()`].

**ZX_RIGHT_SET_PROPERTY** - May set its properties using
[`zx_object_set_property()`].

The **ZX_VMO_ZERO_CHILDREN** signal is active on a newly created VMO. It becomes
inactive whenever a child of the VMO is created and becomes active again when
all children have been destroyed and no mappings of those children into address
spaces exist.

## NOTES

The VMOs created by this syscall are not usable with [`zx_vmo_read()`] and
[`zx_vmo_write()`].

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*resource* must have resource kind **ZX_RSRC_KIND_MMIO**.

## RETURN VALUE

`zx_vmo_create_physical()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## ERRORS

**ZER_ERR_WRONG_TYPE** *resource* is not a handle to a Resource object.

**ZER_ERR_ACCESS_DENIED** *resource* does not grant access to the requested
range of memory.

**ZX_ERR_INVALID_ARGS**  *out* is an invalid pointer or NULL, or *paddr* or
*size* are not page-aligned.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## SEE ALSO

 - [`zx_vmar_map()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_object_get_property()`]: object_get_property.md
[`zx_object_set_property()`]: object_set_property.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmo_read()`]: vmo_read.md
[`zx_vmo_write()`]: vmo_write.md
