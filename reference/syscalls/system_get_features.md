<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_system_get_features

## Summary

Get supported hardware capabilities.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_system_get_features(uint32_t kind, uint32_t* features);
```

## Description

`zx_system_get_features()` populates *features* with a bit mask of
hardware-specific features.  *kind* indicates the specific type of features
to retrieve, e.g. **ZX_FEATURE_KIND_CPU**.  The supported kinds and the meaning
of individual feature bits is hardware-dependent.  **ZX_FEATURE_KIND_VM** is not
hardware-dependent and returns a bitset currently the only meaningful bit
is **ZX_VM_FEATURE_CAN_MAP_XOM** which is 1 if the system can map pages with
execute only permission.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_system_get_features()`  returns **ZX_OK** on success.

## Errors

**ZX_ERR_NOT_SUPPORTED**  The requested feature kind is not available on this
platform.

## Notes
Refer to [Install Fuchsia on a device](/development/hardware/README.md)
for supported processor architectures.

Refer to [zircon/features.h](/zircon/system/public/zircon/features.h) for kinds
of features and individual feature bits.

## See also

 - [`zx_system_get_num_cpus()`]
 - [`zx_system_get_physmem()`]

[`zx_system_get_num_cpus()`]: system_get_num_cpus.md
[`zx_system_get_physmem()`]: system_get_physmem.md
