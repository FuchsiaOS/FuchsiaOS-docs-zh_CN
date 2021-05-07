# zx_pci_cfg_pio_rw

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32938)

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_pci_cfg_pio_rw(zx_handle_t handle,
                              uint8_t bus,
                              uint8_t dev,
                              uint8_t func,
                              uint8_t offset,
                              uint32_t* val,
                              size_t width,
                              uint32_t write);
```

## DESCRIPTION

TODO(fxbug.dev/32938)

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must have resource kind **ZX_RSRC_KIND_ROOT**.

## RETURN VALUE

TODO(fxbug.dev/32938)

## ERRORS

TODO(fxbug.dev/32938)

## SEE ALSO


TODO(fxbug.dev/32938)
