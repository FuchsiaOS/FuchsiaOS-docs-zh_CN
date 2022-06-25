# zx_pager_detach_vmo

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Detaches a vmo from a pager.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_pager_detach_vmo(zx_handle_t pager, zx_handle_t vmo);
```

## DESCRIPTION

Detaching *vmo* from *pager* causes the kernel to stop queuing page requests for the vmo. Subsequent
accesses that would have generated page requests will instead fail.

No new **ZX_PAGER_VMO_READ** and **ZX_PAGER_VMO_DIRTY** requests will be generated after detaching,
but some requests may still be in flight. The pager service is free to ignore these requests, as the
kernel will resume and fault the threads that generated these requests. The final request the pager
service will receive is a **ZX_PAGER_VMO_COMPLETE** request.

The kernel is free to evict clean pages from detached vmos, but will retain any dirty pages. Upon
receiving the **ZX_PAGER_VMO_COMPLETE** request, the pager service is expected to query these ranges
with [`zx_pager_query_dirty_ranges()`] and write them back with [`zx_pager_op_range()`]
**ZX_PAGER_OP_WRITEBACK_BEGIN** and **ZX_PAGER_OP_WRITEBACK_END**. Once they have been written back,
these pages will become clean again, so the kernel is free to evict them.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*pager* must be of type **ZX_OBJ_TYPE_PAGER**.

*vmo* must be of type **ZX_OBJ_TYPE_VMO**.

## RETURN VALUE

`zx_pager_detach_vmo()` returns ZX_OK on success, or one of the following error codes on failure.

## ERRORS

**ZX_ERR_BAD_HANDLE** *pager* or *vmo* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *pager* is not a pager handle or *vmo* is not a vmo handle.

**ZX_ERR_INVALID_ARGS**  *vmo* is not a vmo created from *pager*.

## SEE ALSO

 - [`zx_pager_create_vmo()`]
 - [`zx_pager_op_range()`]
 - [`zx_pager_query_dirty_ranges()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_pager_op_range()`]: pager_op_range.md
[`zx_pager_query_dirty_ranges()`]: pager_query_dirty_ranges.md
