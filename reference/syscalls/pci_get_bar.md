# zx_pci_get_bar

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

This function is obsolete and should not be used.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_pci_get_bar(zx_handle_t handle,
                           uint32_t bar_num,
                           zx_pci_bar_t* out_bar,
                           zx_handle_t* out_handle);
```

## DESCRIPTION

This function is obsolete and should not be used. Drivers should instead get the PCI Base Address
Register information from the [PCI driver
interface](/development/drivers/concepts/driver_development/bar.md);

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_PCI_DEVICE** and have **ZX_RIGHT_READ** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

TODO(fxbug.dev/32938)

## ERRORS

TODO(fxbug.dev/32938)

## SEE ALSO


TODO(fxbug.dev/32938)
