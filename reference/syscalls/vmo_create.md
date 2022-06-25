# zx_vmo_create

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Create a VM object.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_create(uint64_t size, uint32_t options, zx_handle_t* out);
```

## DESCRIPTION

`zx_vmo_create()` creates a new, zero-filled, [virtual memory
object](/docs/reference/kernel_objects/vm_object.md) (VMO), which represents a container of zero to
*size* bytes of memory managed by the operating system.

The size of the VMO will be rounded up to the next system page size boundary,
as reported by [`zx_system_get_page_size()`]. Use [`zx_vmo_get_size()`] to
return the current size of the VMO.

The content size of the VMO will be initialized to the given (unrounded) size.
Use [`zx_object_get_property()`] with **ZX_PROP_VMO_CONTENT_SIZE** to read the
content size of the VMO. Use [`zx_object_set_property()`] with
**ZX_PROP_VMO_CONTENT_SIZE** to set the content size of the VMO without
actually resizing the VMO.

One handle is returned on success, representing an object with the requested
size.

The following rights will be set on the handle by default:

  - **ZX_RIGHT_DUPLICATE** - The handle may be duplicated.

  - **ZX_RIGHT_TRANSFER** - The handle may be transferred to another process.

  - **ZX_RIGHT_READ** - May be read from or mapped with read permissions.

  - **ZX_RIGHT_WRITE** - May be written to or mapped with write permissions.

  - **ZX_RIGHT_MAP** - May be mapped.

  - **ZX_RIGHT_GET_PROPERTY** - May get its properties using [`zx_object_get_property()`].

  - **ZX_RIGHT_SET_PROPERTY** - May set its properties using [`zx_object_set_property()`].

The *options* field can be 0 or a combination of:

  - **ZX_VMO_RESIZABLE** to create a VMO that can change size. Children of a non-resizable VMO can
    be resized.

  - **ZX_VMO_DISCARDABLE** to create a VMO that the kernel can discard pages from under memory
    pressure. Use [`zx_vmo_op_range()`] with **ZX_VMO_OP_LOCK** to lock discardable VMOs when in
    use, and unlock them when done with **ZX_VMO_OP_UNLOCK** making them eligible for reclamation by
    the kernel. A newly created discardable VMO is initially unlocked.

The **ZX_VMO_ZERO_CHILDREN** signal is active on a newly created VMO. It becomes
inactive whenever a child of the VMO is created and becomes active again when
all children have been destroyed and no mappings of those children into address
spaces exist.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Caller job policy must allow **ZX_POL_NEW_VMO**.

## RETURN VALUE

`zx_vmo_create()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_INVALID_ARGS**  *out* is an invalid pointer or NULL or *options* is
any value other than 0.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_OUT_OF_RANGE**  Requested size is too large.

## SEE ALSO

 - [`zx_system_get_page_size()`]
 - [`zx_vmar_map()`]
 - [`zx_vmo_create_child()`]
 - [`zx_vmo_get_size()`]
 - [`zx_vmo_op_range()`]
 - [`zx_vmo_read()`]
 - [`zx_vmo_replace_as_executable()`]
 - [`zx_vmo_set_size()`]
 - [`zx_vmo_write()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_object_get_property()`]: object_get_property.md
[`zx_object_set_property()`]: object_set_property.md
[`zx_system_get_page_size()`]: system_get_page_size.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmo_create_child()`]: vmo_create_child.md
[`zx_vmo_get_size()`]: vmo_get_size.md
[`zx_vmo_op_range()`]: vmo_op_range.md
[`zx_vmo_read()`]: vmo_read.md
[`zx_vmo_replace_as_executable()`]: vmo_replace_as_executable.md
[`zx_vmo_set_size()`]: vmo_set_size.md
[`zx_vmo_write()`]: vmo_write.md
