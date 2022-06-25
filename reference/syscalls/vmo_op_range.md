# zx_vmo_op_range

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Perform an operation on a range of a VMO.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_op_range(zx_handle_t handle,
                            uint32_t op,
                            uint64_t offset,
                            uint64_t size,
                            void* buffer,
                            size_t buffer_size);
```

## DESCRIPTION

`zx_vmo_op_range()` performs cache and memory operations against pages held by the [virtual memory
object](/docs/reference/kernel_objects/vm_object.md) (VMO).

*offset* byte offset specifying the starting location for *op* in the VMO's held memory.

*size* length, in bytes, to perform the operation on.

*op* the operation to perform:

*buffer* and *buffer_size* are currently unused.

**ZX_VMO_OP_COMMIT** - Commit *size* bytes worth of pages starting at byte *offset* for the VMO.
More information can be found in the [vm object documentation](/docs/reference/kernel_objects/vm_object.md).
Requires the **ZX_RIGHT_WRITE** right.

**ZX_VMO_OP_DECOMMIT** - Release a range of pages previously committed to the VMO from *offset*
to *offset*+*size*, which resets that range's bytes to 0. Requires the **ZX_RIGHT_WRITE** right.
This is only supported for vmos created from [`zx_vmo_create()`], which do not have non-slice
children, and for slice children of such vmos. Provided range must be page aligned.

**ZX_VMO_OP_ZERO** - Resets the range of bytes in the VMO from *offset* to *offset*+*size* to
0. This is semantically equivalent to writing 0's with
[`zx_vmo_write()`](/docs/reference/syscalls/vmo_write.md), except that it is able to be done more
efficiently and save memory by de-duping to shared zero pages. Requires the **ZX_RIGHT_WRITE** right.

**ZX_VMO_OP_LOCK** - Locks a range of pages in a discardable VMO, preventing them from being
discarded by the kernel. Guaranteed to successfully lock the VMO and return **ZX_OK** if the
arguments are valid.  *buffer* should point to a `zx_vmo_lock_state_t` struct, and *buffer_size*
should accommodate the struct. Returns information about the locked and previously discarded ranges
in *buffer*, so that clients can reinitialize discarded contents if needed.

The entire VMO should be locked at once, so *offset* should be 0 and *size* should be the current
size of the VMO.  Requires the **ZX_RIGHT_READ** or **ZX_RIGHT_WRITE** right. Note that locking
itself does not commit any pages in the VMO; it just marks the state of the VMO as “undiscardable”
by the kernel.

*buffer* should be a pointer of type `zx_info_lock_state_t`.

```c
typedef struct zx_vmo_lock_state {
  // |offset| and |size| track the locked range, and will be set to the |offset|
  // and |size| arguments passed in if the ZX_VMO_OP_LOCK is successful.
  uint64_t offset;
  uint64_t size;
  // |discarded_offset| and |discarded_size| track the discarded range prior to
  // the lock operation. This is the maximal range within the locked range that
  // contains discarded pages; not all pages within this range might have been
  // discarded. Both |discarded_offset| and |discarded_size| will be set to 0 if
  // the range was not discarded.
  uint64_t discarded_offset;
  uint64_t discarded_size;
} zx_vmo_lock_state_t;
```

**ZX_VMO_OP_TRY_LOCK** - Locks a range of pages in a discardable VMO, preventing them from being
discarded by the kernel. Will only succeed if the range has not already been discarded by the
kernel, and will fail with **ZX_ERR_UNAVAILABLE** otherwise. This operation is meant as a
lightweight alternative to **ZX_VMO_OP_LOCK** for trying to lock the VMO without having to set up
the *buffer* argument. It also affords clients the choice to not take any action following failure
to lock the VMO; clients must use **ZX_VMO_OP_LOCK** if they wish to lock the VMO again.

The entire VMO should be locked at once, so *offset* should be 0 and *size* should be the current
size of the VMO.  Requires the **ZX_RIGHT_READ** or **ZX_RIGHT_WRITE** right. Note that locking
itself does not commit any pages in the VMO; it just marks the state of the VMO as “undiscardable”
by the kernel.

**ZX_VMO_OP_UNLOCK** - Unlocks a range of pages in a discardable VMO, indicating that the kernel is
free to discard them under memory pressure. Unlocked pages that have not been discarded yet will be
counted as committed pages.

The entire VMO should be unlocked at once, so *offset* should be 0 and *size* should be the current
size of the VMO. Requires the **ZX_RIGHT_READ** or **ZX_RIGHT_WRITE** right.

**ZX_VMO_OP_CACHE_SYNC** - Synchronize instruction caches with data caches, so previous writes are
visible to instruction fetches.
Requires the **ZX_RIGHT_READ** right.

**ZX_VMO_OP_CACHE_INVALIDATE** - Performs a cache invalidation operation so that future reads see
external changes to main memory. Note, this operation is only available when
`kernel.enable-debugging-syscalls` is true. When debugging syscalls are not enabled, this operation
will fail with **ZX_ERR_NOT_SUPPORTED**
Requires the **ZX_RIGHT_WRITE** right.

**ZX_VMO_OP_CACHE_CLEAN** - Clean (write back) data caches, so previous writes are visible in main
memory.
Requires the **ZX_RIGHT_READ** right.

**ZX_VMO_OP_CACHE_CLEAN_INVALIDATE** - Performs cache clean and invalidate operations together.
Requires the **ZX_RIGHT_READ** right.

**ZX_VMO_OP_DONT_NEED** - Hints that pages in the specified range are not needed anymore and should
be considered for memory reclamation. Intended to be used with VMOs created with
[`zx_pager_create_vmo()`](/docs/reference/syscalls/pager_create_vmo.md); trivially succeeds for
other VMOs.

This only applies to pages in the given range that are already committed, i.e. no new pages will be
committed as a result of this op. If required, *offset* will be rounded down to the previous page
boundary and *offset*+*size* will be rounded up to the next page boundary.

**ZX_VMO_OP_ALWAYS_NEED** - Hints that pages in the specified range are important and should be
protected from memory reclamation. The kernel may decide to override this hint when the system is
under extreme memory pressure. This hint also does not prevent pages from being freed by means other
than memory reclamation (e.g. a decommit, VMO resize, or VMO destruction). Intended to be used with
VMOs created with [`zx_pager_create_vmo()`](/docs/reference/syscalls/pager_create_vmo.md); trivially
succeeds for other VMOs.

This may commit pages in the given range where applicable, e.g. if the VMO is directly backed by a
pager, its pages will be committed, or in the case of a clone, pages in the parent that are visible
to the clone will be committed. If required, *offset* will be rounded down to the previous page
boundary and *offset*+*size* will be rounded up to the next page boundary.


## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

If *op* is **ZX_VMO_OP_COMMIT**, *handle* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_WRITE**.

If *op* is **ZX_VMO_OP_DECOMMIT**, *handle* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_WRITE**.

If *op* is **ZX_VMO_OP_CACHE_SYNC**, *handle* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_READ**.

If *op* is **ZX_VMO_OP_CACHE_INVALIDATE**, *handle* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_WRITE**.

If *op* is **ZX_VMO_OP_CACHE_CLEAN**, *handle* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_READ**.

If *op* is **ZX_VMO_OP_CACHE_CLEAN_INVALIDATE**, *handle* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_READ**.

## RETURN VALUE

`zx_vmo_op_range()` returns **ZX_OK** on success. In the event of failure, a negative error
value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_OUT_OF_RANGE**  An invalid memory range specified by *offset* and *size*.

**ZX_ERR_NO_MEMORY**  Allocations to commit pages for **ZX_VMO_OP_COMMIT** or **ZX_VMO_OP_ZERO**
failed.

**ZX_ERR_WRONG_TYPE**  *handle* is not a VMO handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have sufficient rights to perform the operation.

**ZX_ERR_INVALID_ARGS**  *buffer* is an invalid pointer (if required by the operation), *op* is not
a valid operation, *size* is zero and *op* is a cache operation, or *op* was **ZX_VMO_OP_DECOMMIT**
and range was not page aligned.

**ZX_ERR_NOT_SUPPORTED**  *op* was **ZX_VMO_OP_LOCK**, **ZX_VMO_OP_TRY_LOCK** or
**ZX_VMO_OP_UNLOCK** and the VMO is not discardable, or *op* was **ZX_VMO_OP_DECOMMIT** and the
underlying VMO does not allow decommiting, or *op* was **ZX_VMO_OP_CACHE_INVALIDATE** and
`kernel.enable-debugging-syscalls` is false.

**ZX_ERR_UNAVAILABLE** *op* was **ZX_VMO_OP_TRY_LOCK**, the VMO was discardable and the VMO has been
discarded by the kernel.

**ZX_ERR_BAD_STATE**  *op* was **ZX_VMO_OP_COMMIT**, the VMO is backed by a pager and the pager or
the VMO is in a bad state preventing requested pages from being populated. *op* was
**ZX_VMO_OP_UNLOCK**, the VMO is discardable and the VMO was not previously locked.

**ZX_ERR_IO** *op* was **ZX_VMO_OP_COMMIT**, the VMO is backed by a pager and the pager encountered
an I/O error while committing the requested pages.

**ZX_ERR_IO_DATA_INTEGRITY** *op* was **ZX_VMO_OP_COMMIT**, the VMO is backed by a pager and the
contents that were read in by the pager for the pages being committed are corrupted.

## SEE ALSO

 - [`zx_vmo_create()`]
 - [`zx_vmo_create_child()`]
 - [`zx_vmo_get_size()`]
 - [`zx_vmo_read()`]
 - [`zx_vmo_set_size()`]
 - [`zx_vmo_write()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_vmo_create()`]: vmo_create.md
[`zx_vmo_create_child()`]: vmo_create_child.md
[`zx_vmo_get_size()`]: vmo_get_size.md
[`zx_vmo_read()`]: vmo_read.md
[`zx_vmo_set_size()`]: vmo_set_size.md
[`zx_vmo_write()`]: vmo_write.md
