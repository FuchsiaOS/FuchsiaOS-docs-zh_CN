<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_pci_get_nth_device

## Summary

This function is obsolete and should not be used.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_pci_get_nth_device(zx_handle_t handle,
                                  uint32_t index,
                                  zx_pcie_device_info_t* out_info,
                                  zx_handle_t* out_handle);
```

## Description

This function is obsolete and should not be used. Drivers should instead use the PCI protocol
Typically, you obtain this in your **bind()** function through **device_get_protocol()**.

## Rights

*handle* must have resource kind **ZX_RSRC_KIND_ROOT**.

## Return value

TODO(fxbug.dev/32938)

## Errors

TODO(fxbug.dev/32938)

## See also

TODO(fxbug.dev/32938)
