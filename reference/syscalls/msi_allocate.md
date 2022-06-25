# zx_msi_allocate

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Allocate Message-Signaled Interrupts (MSIs).

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_msi_allocate(zx_handle_t handle,
                            uint32_t count,
                            zx_handle_t* out_allocation);
```

## DESCRIPTION

`zx_msi_allocate()` allocates a contiguous block of *count* MSIs and returns an
MSI allocation object in *out_allocation* for use with [`zx_msi_create()`].
This serves to allow a PCI device to request pre-allocation of MSI resources
for use in creating Interrupt objects corresponding to the device's interrupts.
The MSI allocation object is freed when *handle* is closed and no outstanding
handles to Interrupt objects created by [`zx_msi_create()`] exist.

*count* must be 1, 2, 4, 8, 16, or 32.

*handle* must be a handle to the Root Resource, as such this syscall is
intended for use only by the platform bus and Zircon coretests.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must have resource kind **ZX_RSRC_KIND_ROOT**.

## RETURN VALUE

`zx_msi_allocate()` returns **ZX_OK** on success. In the event of failure, a
negative error value is returned.

## ERRORS

**ZX_ERR_INVALID_ARGS** *count* is not a valid value.

**ZX_ERR_ACCESS_DENIED** *handle* is not a Root Resource handle.

**ZX_ERR_NOT_SUPPORTED** MSIs are not supported by the platform.

## SEE ALSO

 - [`zx_msi_create()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_msi_create()`]: msi_create.md
