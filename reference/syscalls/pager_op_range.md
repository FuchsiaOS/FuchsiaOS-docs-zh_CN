# zx_pager_op_range

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Perform an operation on a range of a pager owned vmo.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_pager_op_range(zx_handle_t pager,
                              uint32_t op,
                              zx_handle_t pager_vmo,
                              uint64_t offset,
                              uint64_t length,
                              uint64_t data);
```

## DESCRIPTION

Performs a pager operation, specified by *op* on *pager_vmo* in the range [*offset*, *offset* +
*length*). The *pager_vmo* must have previously been created from the *pager* by
[`zx_pager_create_vmo()`]. *offset* and *length* must be page aligned. *data* is an optional
parameter, if the specified *op* supports one.

Operations that can be performed, i.e. values *op* can take:

**ZX_PAGER_OP_FAIL** - The userspace pager failed to fulfill page requests for *pager_vmo* in the
range [*offset*, *offset* + *length*). *data* contains the error encountered (a `zx_status_t`
error code) - permitted values are **ZX_ERR_IO**, **ZX_ERR_IO_DATA_INTEGRITY** and
**ZX_ERR_BAD_STATE**.  This will signal threads that might be waiting on page requests in that
range, unblocking them. If the blocked thread was requesting pages through a [`zx_vmo_read()`] or a
[`zx_vmo_op_range()`] with **ZX_VMO_OP_COMMIT**, the call will fail and the error status (*data*)
will be returned. If the blocked thread was requesting pages through a VMAR mapping, the thread
will take a fatal page fault exception.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*pager* must be of type **ZX_OBJ_TYPE_PAGER**.

*pager_vmo* must be of type **ZX_OBJ_TYPE_VMO**.

## RETURN VALUE

`zx_pager_op_range()` returns ZX_OK on success, or one of the following error codes on failure.

## ERRORS

**ZX_ERR_BAD_HANDLE** *pager* or *pager_vmo* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *pager* is not a pager handle, or *pager_vmo* is not a vmo handle.

**ZX_ERR_INVALID_ARGS**  *pager_vmo* is not a vmo created from *pager*, or *offset* or *length* is
not page aligned, or *op* is **ZX_PAGER_OP_FAIL** and *data* is not one of **ZX_ERR_IO**,
**ZX_ERR_IO_DATA_INTEGRITY** or **ZX_ERR_BAD_STATE**.

**ZX_ERR_OUT_OF_RANGE** The specified range in *pager_vmo* is invalid.

**ZX_ERR_NOT_SUPPORTED**  *op* is not supported on the specified range in *pager_vmo*.

## SEE ALSO

 - [`zx_pager_create_vmo()`]
 - [`zx_pager_detach_vmo()`]
 - [`zx_pager_supply_pages()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_pager_detach_vmo()`]: pager_detach_vmo.md
[`zx_pager_supply_pages()`]: pager_supply_pages.md
[`zx_vmo_op_range()`]: vmo_op_range.md
[`zx_vmo_read()`]: vmo_read.md
