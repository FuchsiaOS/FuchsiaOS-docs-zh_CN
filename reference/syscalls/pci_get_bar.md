<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_pci_get_bar

## Summary

This function is obsolete and should not be used.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_pci_get_bar(zx_handle_t handle,
                           uint32_t bar_num,
                           zx_pci_bar_t* out_bar,
                           zx_handle_t* out_handle);
```

## Description

This function is obsolete and should not be used. Drivers should instead get the PCI Base Address
Register information from the [PCI driver
interface](/development/drivers/concepts/driver_development/bar.md);

## Rights

*handle* must be of type **ZX_OBJ_TYPE_PCI_DEVICE** and have **ZX_RIGHT_READ** and have **ZX_RIGHT_WRITE**.

## Return value

TODO(fxbug.dev/32938)

## Errors

TODO(fxbug.dev/32938)

## See also

TODO(fxbug.dev/32938)
