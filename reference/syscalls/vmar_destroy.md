# zx_vmar_destroy

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Destroy a virtual memory address region.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmar_destroy(zx_handle_t handle);
```

## DESCRIPTION

`zx_vmar_destroy()` unmaps all mappings within the given region, and destroys
all sub-regions of the region.  Note that this operation is logically recursive.

This operation does not close *handle*.  Any outstanding handles to this
VMAR will remain valid handles, but all VMAR operations on them will fail.

The root VMAR, as obtained by `zx_process_create()`, cannot be destroyed.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_vmar_destroy()` returns **ZX_OK** on success.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a VMAR handle.

**ZX_ERR_BAD_STATE**  This region is already destroyed.

**ZX_ERR_NOT_SUPPORTED** *handle* is a root VMAR.

## NOTES

## SEE ALSO

 - [`zx_vmar_allocate()`]
 - [`zx_vmar_map()`]
 - [`zx_vmar_protect()`]
 - [`zx_vmar_unmap()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_vmar_allocate()`]: vmar_allocate.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmar_protect()`]: vmar_protect.md
[`zx_vmar_unmap()`]: vmar_unmap.md
