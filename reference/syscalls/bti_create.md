# zx_bti_create

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Create a new bus transaction initiator.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_bti_create(zx_handle_t iommu,
                          uint32_t options,
                          uint64_t bti_id,
                          zx_handle_t* out);
```

## DESCRIPTION

`zx_bti_create()` creates a new [bus transaction initiator](/docs/reference/kernel_objects/bus_transaction_initiator.md)
given a handle to an IOMMU and a hardware transaction identifier for a device
downstream of that IOMMU.

*options* must be 0 (reserved for future definition of creation flags).

Upon success a handle for the new BTI is returned.  This handle will have rights
**ZX_RIGHT_READ**, **ZX_RIGHT_WRITE**, **ZX_RIGHT_MAP**, **ZX_RIGHT_INSPECT**,
**ZX_RIGHT_DUPLICATE**, and **ZX_RIGHT_TRANSFER**.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*iommu* must be of type **ZX_OBJ_TYPE_IOMMU** and have **ZX_RIGHT_NONE**.

## RETURN VALUE

`zx_bti_create()` returns **ZX_OK** and a handle to the new BTI
(via *out*) on success.  In the event of failure, a negative error value
is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *iommu* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *iommu* is not an iommu handle.

**ZX_ERR_ACCESS_DENIED**  *iommu* handle does not have sufficient privileges.

**ZX_ERR_INVALID_ARGS**  *bti_id* is invalid on the given IOMMU,
*out* is an invalid pointer, or *options* is non-zero.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## SEE ALSO

 - [`zx_bti_pin()`]
 - [`zx_bti_release_quarantine()`]
 - [`zx_pmt_unpin()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_bti_pin()`]: bti_pin.md
[`zx_bti_release_quarantine()`]: bti_release_quarantine.md
[`zx_pmt_unpin()`]: pmt_unpin.md
