<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_msi_allocate

## Summary

Allocate Message-Signaled Interrupts (MSIs).

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_msi_allocate(zx_handle_t handle,
                            uint32_t count,
                            zx_handle_t* out_allocation);
```

## Description

`zx_msi_allocate()` allocates a contiguous block of *count* MSIs and returns an
MSI allocation object in *out_allocation* for use with [`zx_msi_create()`].
This serves to allow a PCI device to request pre-allocation of MSI resources
for use in creating Interrupt objects corresponding to the device's interrupts.
The MSI allocation object is freed when *handle* is closed and no outstanding
handles to Interrupt objects created by [`zx_msi_create()`] exist.

*count* must be 1, 2, 4, 8, 16, or 32.

*handle* must be a handle to the Root Resource, as such this syscall is
intended for use only by the platform bus and Zircon coretests.

## Rights

*handle* must have resource kind **ZX_RSRC_KIND_ROOT**.

## Return value

`zx_msi_allocate()` returns **ZX_OK** on success. In the event of failure, a
negative error value is returned.

## Errors

**ZX_ERR_INVALID_ARGS** *count* is not a valid value.

**ZX_ERR_ACCESS_DENIED** *handle* is not a Root Resource handle.

**ZX_ERR_NOT_SUPPORTED** MSIs are not supported by the platform.

## See also

 - [`zx_msi_create()`]

[`zx_msi_create()`]: msi_create.md
