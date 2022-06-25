# zx_bti_pin

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Pin pages and grant devices access to them.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_bti_pin(zx_handle_t handle,
                       uint32_t options,
                       zx_handle_t vmo,
                       uint64_t offset,
                       uint64_t size,
                       zx_paddr_t* addrs,
                       size_t num_addrs,
                       zx_handle_t* pmt);
```

## DESCRIPTION

`zx_bti_pin()` pins pages of a VMO (i.e. prevents them from being decommitted
with [`zx_vmo_op_range()`]) and grants the hardware
transaction ID represented by the BTI the ability to access these pages,
with the permissions specified in *options*.

*offset* must be aligned to page boundaries.

*options* is a bitfield that may contain one or more of **ZX_BTI_PERM_READ**,
**ZX_BTI_PERM_WRITE**, **ZX_BTI_PERM_EXECUTE**, **ZX_BTI_COMPRESS**, and
**ZX_BTI_CONTIGUOUS**.  In order for the call to succeed, *vmo* must have the
READ/WRITE rights corresponding to the permissions flags set in *options*.
(Note: **ZX_BTI_PERM_EXECUTE** requires **ZX_RIGHT_READ**, not **ZX_RIGHT_EXECUTE**.)
**ZX_BTI_CONTIGUOUS** is only allowed if *vmo* was allocated via
[`zx_vmo_create_contiguous()`] or [`zx_vmo_create_physical()`].
**ZX_BTI_COMPRESS** and **ZX_BTI_CONTIGUOUS** are mutually exclusive.

If the range in *vmo* specified by *offset* and *size* contains non-committed
pages, a successful invocation of this function will result in those pages
having been committed.  On failure, it is undefined whether they have been
committed.

*addrs* will be populated with the device-physical addresses of the requested
VMO pages.  These addresses may be given to devices that issue memory
transactions with the hardware transaction ID associated with the BTI.  The
number of addresses returned depends on whether the **ZX_BTI_COMPRESS** or
**ZX_BTI_CONTIGUOUS** options were given. The number of addresses will be one
of three possibilities:

1. If neither is set, one per page (`size / PAGE_SIZE`).
2. If **ZX_BTI_COMPRESS** is set, `size / minimum-contiguity`, rounded up
   (each address representing a run of *minimum-contiguity* run of bytes,
   with the last one being potentially short if *size* is not a multiple of
   *minimum-contiguity*).  It is guaranteed that all returned addresses will be
   *minimum-contiguity*-aligned.  Note that *minimum-contiguity* is discoverable
   via [`zx_object_get_info()`].
3. If **ZX_BTI_CONTIGUOUS** is set, the single address of the start of the memory.

*num_addrs* is the number of entries in the *addrs* array.  It is an error for
*num_addrs* to not match the value calculated above.

The PMT retains a reference to the associated VMO, so the underlying VMO won't be
destroyed until the PMT is unpinned.

Resizable VMOs can be pinned. If a call to [`zx_vmo_set_size()`] would discard
pinned pages, that call will fail.

## OPTIONS

- **ZX_BTI_PERM_READ**, **ZX_BTI_PERM_WRITE**, and **ZX_BTI_PERM_EXECUTE** define
  the access types that the hardware bus transaction initiator will be allowed
  to use.
- **ZX_BTI_COMPRESS** causes the returned address list to contain one entry per
  block of *minimum-contiguity* bytes, rather than one per *PAGE_SIZE*.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_BTI** and have **ZX_RIGHT_MAP**.

*vmo* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_MAP**.

If *options* & **ZX_BTI_PERM_READ**, *vmo* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_READ**.

If *options* & **ZX_BTI_PERM_WRITE**, *vmo* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_WRITE**.

If *options* & **ZX_BTI_PERM_EXECUTE**, *vmo* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_READ**.

## RETURN VALUE

On success, `zx_bti_pin()` returns **ZX_OK**.  The device-physical addresses of the
requested VMO pages will be written in *addrs*.  A handle to the created Pinned
Memory Token is returned via *pmt*.  When the PMT is no longer needed,
[`zx_pmt_unpin()`] should be invoked.

In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* or *vmo* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a BTI handle or *vmo* is not a VMO
handle.

**ZX_ERR_ACCESS_DENIED** *handle* or *vmo* does not have the **ZX_RIGHT_MAP**,
or *options* contained a permissions flag corresponding to a right that *vmo*
does not have.

**ZX_ERR_INVALID_ARGS** *options* is 0 or contains an undefined flag, either
*addrs* or *pmt* is not a valid pointer, *num_addrs* is not the same as the
number of entries that would be returned, or *offset* or *size* is not
page-aligned.

**ZX_ERR_OUT_OF_RANGE** `offset + size` is out of the bounds of *vmo*.

**ZX_ERR_UNAVAILABLE** (Temporary) At least one page in the requested range
could not be pinned at this time.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory. There is no good way for
userspace to handle this (unlikely) error. In a future build this error will no
longer occur.

## SEE ALSO

 - [`zx_bti_create()`]
 - [`zx_object_get_info()`]
 - [`zx_pmt_unpin()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_bti_create()`]: bti_create.md
[`zx_object_get_info()`]: object_get_info.md
[`zx_pmt_unpin()`]: pmt_unpin.md
[`zx_vmo_create_contiguous()`]: vmo_create_contiguous.md
[`zx_vmo_create_physical()`]: vmo_create_physical.md
[`zx_vmo_op_range()`]: vmo_op_range.md
[`zx_vmo_set_size()`]: vmo_set_size.md
