# zx_vmo_create_child

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Create a child of a VM Object.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_create_child(zx_handle_t handle,
                                uint32_t options,
                                uint64_t offset,
                                uint64_t size,
                                zx_handle_t* out);
```

## DESCRIPTION

`zx_vmo_create_child()` creates a new [virtual memory
object](/reference/kernel_objects/vm_object.md) (VMO) a child of an existing vmo. The behavior
of the semantics depends on the type of the child.

One handle is returned on success, representing an object with the requested
size.

*options* must contain exactly one of the following flags to specify the
child type:

- **ZX_VMO_CHILD_SNAPSHOT** -  Create a child that behaves as if an eager copy is performed. When a
write occurs, both the parent and child perform a lazy copy. Lazy copying allows both the child and
the parent to diverge from each other. Any reads from ranges outside of the parent VMO's size
contain zeros, and writes allocate new zero filled pages.
This flag is not supported on:
 - VMOs with pinned regions.
 - VMOs created with or descended from [`zx_vmo_create_physical()`] or
   [`zx_vmo_create_contiguous()`]
 - VMOs backed by a user pager.
For information on VMO syscall interactions with children, see [NOTES](#notes).

ZX_VMO_CHILD_SNAPSHOT creates an immutable VMO when the ZX_VMO_CHILD_NO_WRITE
option is enabled and the ZX_VMO_CHILD_RESIZABLE option is disabled.

- **ZX_VMO_CHILD_SNAPSHOT_AT_LEAST_ON_WRITE** -  Create a child that behaves with at least copy on
write semantics. Any write operation on the child brings in a copy of the page from the parent,
after which its contents may diverge from the parent. Until a page is written to, and copied, reads
are permitted, although not guaranteed, to return changing values if the parent performs writes.
This flag may not be used for VMOs created with [`zx_vmo_create_physical()`],
[`zx_vmo_create_contiguous()`] or descendants of such VMOs.
For information on VMO syscall interactions with children, see [NOTES](#notes).

- **ZX_VMO_CHILD_SLICE** - Create a slice that has direct read/write access into
a section of the parent. All operations on the slice vmo behave as if they were
done on the parent. A slice differs from a duplicate handle to the parent by allowing
access to only a subrange of the parent vmo, and allowing for the
**ZX_VMO_ZERO_CHILDREN** signal to be used. This flag may be used with vmos created with
[`zx_vmo_create_physical()`] or [`zx_vmo_create_contiguous()`] and their descendants.

In addition, *options* can contain zero or more of the following flags to
further specify the child's behavior:

- **ZX_VMO_CHILD_RESIZABLE** - Create a resizable child VMO.

- **ZX_VMO_CHILD_NO_WRITE** - Create a child that cannot be written to. This is incompatible with
                              **ZX_VMO_CHILD_RESIZABLE**.

*offset* must be page aligned.

*offset* + *size* may not exceed the range of a 64bit unsigned value.

Both offset and size may start or extend beyond the original VMO's size.

The size of the VMO will be rounded up to the next page size boundary.

The content size of the VMO will be initialized to the given (unrounded) size.
Use [`zx_object_get_property()`] with **ZX_PROP_VMO_CONTENT_SIZE** to read the
content size of the VMO. Use [`zx_object_set_property()`] with
**ZX_PROP_VMO_CONTENT_SIZE** to set the content size of the VMO without
actually resizing the VMO.

By default the rights of the child handle will be the same as the
original with a few exceptions. See [`zx_vmo_create()`] for a
discussion of the details of each right.

In all cases if **ZX_VMO_NO_WRITE** is set then **ZX_RIGHT_WRITE** will be removed.

If *options* is **ZX_VMO_CHILD_SNAPSHOT** or **ZX_VMO_CHILD_SNAPSHOT_AT_LEAST_ON_WRITE** and
**ZX_VMO_CHILD_NO_WRITE** is not set then **ZX_RIGHT_WRITE** will be added and **ZX_RIGHT_EXECUTE**
will be removed.

## NOTES

Creating a child VMO causes the existing (source) VMO **ZX_VMO_ZERO_CHILDREN** signal
to become inactive. Only when the last child is destroyed and no mappings
of those child into address spaces exist, will **ZX_VMO_ZERO_CHILDREN** become
active again.

Non-slice child vmos will interact with the VMO syscalls in the following ways:

- The COMMIT mode of [`zx_vmo_op_range()`] on a child will commit pages into the child that
  have the same content as its parent's corresponding pages. If those pages are supplied by a
  pager, this operation will also commit those pages in the parent. Otherwise, if those pages
  are not comitted in the parent, zero-filled pages will be comitted directly into
  child, without affecting the parent.
- The DECOMMIT mode of [`zx_vmo_op_range()`] is not supported.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_DUPLICATE** and have **ZX_RIGHT_READ**.

## RETURN VALUE

`zx_vmo_create_child()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_TYPE**  Input handle is not a VMO.

**ZX_ERR_ACCESS_DENIED**  Input handle does not have sufficient rights.

**ZX_ERR_INVALID_ARGS**  *out* is an invalid pointer or NULL
or the offset is not page aligned, or an incompatible combination of *options* was given.

**ZX_ERR_OUT_OF_RANGE**  *offset* + *size* is too large.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_BAD_STATE**  A COW child could not be created because the vmo has some
pinned pages.

**ZX_ERR_NOT_SUPPORTED**  Input handle is a discardable VMO.

## SEE ALSO

 - [`zx_vmar_map()`]
 - [`zx_vmo_create()`]
 - [`zx_vmo_get_size()`]
 - [`zx_vmo_op_range()`]
 - [`zx_vmo_read()`]
 - [`zx_vmo_set_size()`]
 - [`zx_vmo_write()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_object_get_property()`]: object_get_property.md
[`zx_object_set_property()`]: object_set_property.md
[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmo_create()`]: vmo_create.md
[`zx_vmo_create_contiguous()`]: vmo_create_contiguous.md
[`zx_vmo_create_physical()`]: vmo_create_physical.md
[`zx_vmo_get_size()`]: vmo_get_size.md
[`zx_vmo_op_range()`]: vmo_op_range.md
[`zx_vmo_read()`]: vmo_read.md
[`zx_vmo_set_size()`]: vmo_set_size.md
[`zx_vmo_write()`]: vmo_write.md
