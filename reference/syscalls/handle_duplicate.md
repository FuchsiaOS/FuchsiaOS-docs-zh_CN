# zx_handle_duplicate

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Duplicate a handle.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_handle_duplicate(zx_handle_t handle,
                                zx_rights_t rights,
                                zx_handle_t* out);
```

## DESCRIPTION

`zx_handle_duplicate()` creates a duplicate of *handle*, referring
to the same underlying object, with new access rights *rights*.

To duplicate the handle with the same rights use **ZX_RIGHT_SAME_RIGHTS**. If different
rights are desired they must be strictly lesser than of the source handle. It is possible
to specify no rights by using **ZX_RIGHT_NONE**. To remove **ZX_RIGHT_DUPLICATE** right when
transferring through a channel, use [`zx_channel_write_etc()`].

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must have **ZX_RIGHT_DUPLICATE**.

## RETURN VALUE

`zx_handle_duplicate()` returns **ZX_OK** and the duplicate handle via *out* on success.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* isn't a valid handle.

**ZX_ERR_INVALID_ARGS**  The *rights* requested are not a subset of *handle* rights or
*out* is an invalid pointer.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_DUPLICATE** and may not be duplicated.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## SEE ALSO

 - [rights]
 - [`zx_channel_write_etc()`]
 - [`zx_handle_close()`]
 - [`zx_handle_close_many()`]
 - [`zx_handle_replace()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[rights]: concepts/kernel/rights.md
[`zx_channel_write_etc()`]: channel_write_etc.md
[`zx_handle_close()`]: handle_close.md
[`zx_handle_close_many()`]: handle_close_many.md
[`zx_handle_replace()`]: handle_replace.md
