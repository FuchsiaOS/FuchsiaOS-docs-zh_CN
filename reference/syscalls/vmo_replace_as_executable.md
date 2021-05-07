# zx_vmo_replace_as_executable

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Add execute rights to a VMO.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_replace_as_executable(zx_handle_t handle,
                                         zx_handle_t vmex,
                                         zx_handle_t* out);
```

## DESCRIPTION

`zx_vmo_replace_as_executable()` creates a replacement for *handle*, referring
to the same underlying VM object, adding the right **ZX_RIGHT_EXECUTE**.

*handle* is always invalidated.

*vmex* may currently be **ZX_HANDLE_INVALID** to ease migration of new code,
this is TODO(fxbug.dev/24770) and will be removed.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_VMO**.

*vmex* must have resource kind **ZX_RSRC_KIND_VMEX**.

## RETURN VALUE

`zx_vmo_replace_as_executable()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* isn't a valid VM object handle, or
*vmex* isn't a valid **ZX_RSRC_KIND_VMEX** resource handle.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## SEE ALSO

 - [`zx_resource_create()`]
 - [`zx_vmar_map()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_resource_create()`]: resource_create.md
[`zx_vmar_map()`]: vmar_map.md
