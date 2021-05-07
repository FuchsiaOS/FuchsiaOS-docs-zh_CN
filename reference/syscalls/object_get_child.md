# zx_object_get_child

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Given a kernel object with children objects, obtain a handle to the child specified by the provided kernel object id.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_object_get_child(zx_handle_t handle,
                                uint64_t koid,
                                zx_rights_t rights,
                                zx_handle_t* out);
```

## DESCRIPTION

`zx_object_get_child()` attempts to find a child of the object referred to
by *handle* which has the kernel object id specified by *koid*.  If such an
object exists, and the requested *rights* are not greater than those provided
by the *handle* to the parent, a new handle to the specified child object is
returned.

*rights* may be **ZX_RIGHT_SAME_RIGHTS** which will result in rights equivalent
to the those on the *handle*.

If the object is a *Process*, the *Threads* it contains may be obtained by
this call.

If the object is a *Job*, its (immediate) child *Jobs* and the *Processes*
it contains may be obtained by this call.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must have **ZX_RIGHT_ENUMERATE**.

## RETURN VALUE

On success, **ZX_OK** is returned and a handle to the desired child object is returned via *out*.


## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a *Process*, *Job*, or *Resource*.

**ZX_ERR_ACCESS_DENIED**   *handle* lacks the right **ZX_RIGHT_ENUMERATE** or *rights* specifies
rights that are not present on *handle*.

**ZX_ERR_NOT_FOUND**  *handle* does not have a child with the kernel object id *koid*.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_INVALID_ARGS**  *out* is an invalid pointer.


## SEE ALSO

 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_handle_replace()`]
 - [`zx_object_get_info()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_object_get_info()`]: object_get_info.md
