# zx_handle_close_many

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Close a number of handles.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_handle_close_many(const zx_handle_t* handles,
                                 size_t num_handles);
```

## DESCRIPTION

`zx_handle_close_many()` closes a number of handles, causing each
underlying object to be reclaimed by the kernel if no other handles to
it exist.

If a handle was used in a pending [`zx_object_wait_one()`] or a
[`zx_object_wait_many()`] call, the wait will be aborted.

This operation closes all handles presented to it, even if one or more
of the handles is duplicate or invalid.

It is not an error to close the special "never a valid handle" **ZX_HANDLE_INVALID**,
similar to `free(NULL)` being a valid call.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

None.

## RETURN VALUE

`zx_handle_close_many()` returns **ZX_OK** on success.

## ERRORS

**ZX_ERR_BAD_HANDLE**  One of the *handles* isn't a valid handle, or the same handle is
present multiple times.

## SEE ALSO

 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_handle_replace()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
