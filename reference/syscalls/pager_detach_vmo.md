# zx_pager_detach_vmo

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Detaches a vmo from a pager.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_pager_detach_vmo(zx_handle_t pager, zx_handle_t vmo);
```

## DESCRIPTION

Detaching *vmo* from *pager* causes the kernel to stop queuing page requests for the vmo. Subsequent
accesses that would have generated page requests will instead fail.

No new **ZX_PAGER_VMO_READ** requests will be generated after detaching, but some requests may
still be in flight. The pager service is free to ignore these requests, as the kernel will resume and
fault the threads that generated these requests. The final request the pager service will
receive is a **ZX_PAGER_VMO_COMPLETE** request.

The kernel is free to evict clean pages from deregistered vmos.

TODO(stevensd): Update once writeback is supported.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

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

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_pager_create_vmo()`]: pager_create_vmo.md
