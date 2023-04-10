<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_pci_add_subtract_io_range

## Summary

This function is obsolete and should not be used.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_pci_add_subtract_io_range(zx_handle_t handle,
                                         uint32_t mmio,
                                         uint64_t base,
                                         uint64_t len,
                                         uint32_t add);
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
