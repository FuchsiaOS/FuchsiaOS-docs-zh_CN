# zx_bti_release_quarantine

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Releases all quarantined PMTs.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_bti_release_quarantine(zx_handle_t handle);
```

## DESCRIPTION

`zx_bti_release_quarantine()` releases all quarantined PMTs for the given BTI.
This will release the PMTs' underlying references to VMOs and physical page
pins.  The underlying physical pages may be eligible to be reallocated
afterwards.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_BTI** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_bti_release_quarantine()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a BTI handle.

**ZX_ERR_ACCESS_DENIED** *handle* does not have the **ZX_RIGHT_WRITE** right.

## SEE ALSO

 - [`zx_bti_pin()`]
 - [`zx_pmt_unpin()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_bti_pin()`]: bti_pin.md
[`zx_pmt_unpin()`]: pmt_unpin.md
