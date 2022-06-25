# zx_vmar_map

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Add a memory mapping.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmar_map(zx_handle_t handle,
                        zx_vm_option_t options,
                        size_t vmar_offset,
                        zx_handle_t vmo,
                        uint64_t vmo_offset,
                        size_t len,
                        zx_vaddr_t* mapped_addr);
```

## DESCRIPTION

Maps the given VMO into the given virtual memory address region.  The mapping
retains a reference to the underlying virtual memory object, which means
closing the VMO handle does not remove the mapping added by this function.

*options* is a bit vector of the following:

- **ZX_VM_SPECIFIC**  Use the *vmar_offset* to place the mapping, invalid if
  *handle* does not have the **ZX_VM_CAN_MAP_SPECIFIC** permission.
  *vmar_offset* is an offset relative to the base address of the given VMAR.
  It is an error to specify a range that overlaps with another VMAR or mapping.
- **ZX_VM_SPECIFIC_OVERWRITE**  Same as **ZX_VM_SPECIFIC**, but can
  overlap another mapping.  It is still an error to partially-overlap another VMAR.
  If the range meets these requirements, it will atomically (with respect to all
  other map/unmap/protect operations) replace existing mappings in the range
  specified by *vmar_offset* and *len*. If that range partially overlaps any
  mappings, then the portions of those mappings outside the range will remain mapped.
- **ZX_VM_OFFSET_IS_UPPER_LIMIT**  Interpret the *vmar_offset* as an upper limit
  to constrain the selection of the offset by the kernel, invalid if *handle*
  does not have the **ZX_VM_CAN_MAP_SPECIFIC** permission. The resulting mapping
  will have an offset + *len* that is <= *vmar_offset*. This option cannot be
  specified if **ZX_VM_SPECIFIC** or **ZX_VM_SPECIFIC_OVERWRITE** is used.
- **ZX_VM_PERM_READ**  Map *vmo* as readable.  It is an error if *handle*
  does not have **ZX_VM_CAN_MAP_READ** permissions, the *handle* does
  not have the **ZX_RIGHT_READ** right, or the *vmo* handle does not have the
  **ZX_RIGHT_READ** right.
- **ZX_VM_PERM_WRITE**  Map *vmo* as writable.  It is an error if *handle*
  does not have **ZX_VM_CAN_MAP_WRITE** permissions, the *handle* does
  not have the **ZX_RIGHT_WRITE** right, the *vmo* handle does not have the
  **ZX_RIGHT_WRITE** right, or *options* does not specify **ZX_VM_PERM_READ**.
- **ZX_VM_PERM_EXECUTE**  Map *vmo* as executable.  It is an error if *handle*
  does not have **ZX_VM_CAN_MAP_EXECUTE** permissions, the *handle* handle does
  not have the **ZX_RIGHT_EXECUTE** right, the *vmo* handle does not have the
  **ZX_RIGHT_EXECUTE** right, or *options* does not specify **ZX_VM_PERM_READ**.
- **ZX_VM_MAP_RANGE**  Immediately page into the new mapping all backed
  regions of the VMO.  This cannot be specified if
  **ZX_VM_SPECIFIC_OVERWRITE** is used.
- **ZX_VM_ALLOW_FAULTS** Required if it would be possible for the created
  mapping to generate faults. In particular, it is required if *vmo* is resizable,
  if *vmo* is non-resizable but the mapping extends past the end of *vmo*, if
  *vmo* is discardable, or if *vmo* was created from [`zx_pager_create_vmo()`].
- **ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED** Map *vmo* as readable if the system does
  not support mapping execute-only pages. If the system can map execute-only
  this flag is ignored.

*vmar_offset* must be 0 if *options* does not have **ZX_VM_SPECIFIC**,
**ZX_VM_SPECIFIC_OVERWRITE** or **ZX_VM_OFFSET_IS_UPPER_LIMIT** set.
**ZX_VM_OFFSET_IS_UPPER_LIMIT** serves to constrain the selection range, and
otherwise behaves similarly to the case where neither **ZX_VM_SPECIFIC** nor
**ZX_VM_SPECIFIC_OVERWRITE** are set, with the mapping being assigned an offset
at random by the kernel (with an allocator determined by policy set on the
target VMAR).

*len* must be non-zero and page-aligned.

In addition one of the following power-of-two alignment flags can added:

- **ZX_VM_ALIGN_1KB** aligns *child_addr* to a power-of-2 at least 1K bytes.
- **ZX_VM_ALIGN_2KB** aligns *child_addr* to a power-of-2 at least 2K bytes.
- **ZX_VM_ALIGN_4KB** aligns *child_addr* to a power-of-2 at least 4K bytes.
- **ZX_VM_ALIGN_8KB** aligns *child_addr* to a power-of-2 at least 8K bytes.
and continues up to
- **ZX_VM_ALIGN_4GB** aligns *child_addr* to a power-of-2 at least 4G bytes.

Using **ZX_VM_ALIGN** flags with **ZX_VM_SPECIFIC** will fail if the vmar
base address + *vmo_offset* are not aligned to the requested value.


## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_VMAR**.

*vmo* must be of type **ZX_OBJ_TYPE_VMO**.

## RETURN VALUE

`zx_vmar_map()` returns **ZX_OK** and the absolute base address of the
mapping (via *mapped_addr*) on success.  The base address will be page-aligned
and non-zero.  In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* or *vmo* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* or *vmo* is not a VMAR or VMO handle, respectively.

**ZX_ERR_BAD_STATE**  *handle* refers to a destroyed VMAR.

**ZX_ERR_INVALID_ARGS** for any of the following:
 - *mapped_addr* or *options* is not valid.
 - *vmar_offset* is non-zero when none of **ZX_VM_SPECIFIC**, **ZX_VM_SPECIFIC_OVERWRITE** or
   **ZX_VM_OFFSET_IS_UPPER_LIMIT** is specified.
 - **ZX_VM_SPECIFIC_OVERWRITE** and **ZX_VM_MAP_RANGE** are both specified.
 - **ZX_VM_OFFSET_IS_UPPER_LIMIT** is specified together with either **ZX_VM_SPECIFIC**
   or **ZX_VM_SPECIFIC_OVERWRITE**.
 - *vmar_offset* and *len* describe an unsatisfiable allocation due to exceeding the region bounds.
 - *vmar_offset* or *vmo_offset* is not page-aligned.
 - *len* is 0 or not page-aligned.

**ZX_ERR_ALREADY_EXISTS**  **ZX_VM_SPECIFIC** has been specified without
**ZX_VM_SPECIFIC_OVERWRITE**, and the requested range overlaps with another mapping.

**ZX_ERR_NO_RESOURCES** If a spot could not be found in the VMAR to create the mapping.

**ZX_ERR_ACCESS_DENIED**  Insufficient privileges to make the requested mapping.

**ZX_ERR_NOT_SUPPORTED** If the vmo is resizable, discardable, or backed by a pager but
**ZX_VM_ALLOW_FAULTS** is not set.

**ZX_ERR_BUFFER_TOO_SMALL** The VMO is not resizable and the mapping extends past the end
of the VMO but **ZX_VM_ALLOW_FAULTS** is not set.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_OUT_OF_RANGE** `vmo_offset + ROUNDUP(len, PAGE_SIZE)` overflows.

## NOTES

The VMO that backs a memory mapping can be resized to a smaller size. This can cause the
thread is reading or writing to the VMAR region to fault. To avoid this hazard, services
that receive VMOs from clients should use **ZX_VM_REQUIRE_NON_RESIZABLE** when mapping
the VMO.


## SEE ALSO

 - [`zx_vmar_allocate()`]
 - [`zx_vmar_destroy()`]
 - [`zx_vmar_protect()`]
 - [`zx_vmar_unmap()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_vmar_allocate()`]: vmar_allocate.md
[`zx_vmar_destroy()`]: vmar_destroy.md
[`zx_vmar_protect()`]: vmar_protect.md
[`zx_vmar_unmap()`]: vmar_unmap.md
