<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_system_mexec

## Summary

Soft reboot the system with a new kernel and bootimage.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_system_mexec(zx_handle_t resource,
                            zx_handle_t kernel_vmo,
                            zx_handle_t bootimage_vmo);
```

## Description

`zx_system_mexec()` accepts two vmo handles: *kernel_vmo* should contain a
kernel image and *bootimage_vmo* should contain an initrd whose address shall
be passed to the new kernel as a kernel argument.

To supplant the running kernel, a *resource* of **ZX_RSRC_KIND_SYSTEM** with base
**ZX_RSRC_SYSTEM_MEXEC_BASE** must be supplied.

Upon success, `zx_system_mexec()` shall supplant the currently running kernel
image with the kernel image contained within *kernel_vmo*, load the ramdisk
contained within *bootimage_vmo* to a location in physical memory and branch
directly into the new kernel while providing the address of the loaded initrd
to the new kernel.

To use the `zx_system_mexec()` function, you must specify
`kernel.enable-debugging-syscalls=true` on the kernel command line. Otherwise,
the function returns **ZX_ERR_NOT_SUPPORTED**.

## Rights

*resource* must have resource kind **ZX_RSRC_KIND_SYSTEM** with base
**ZX_RSRC_SYSTEM_MEXEC_BASE**.

*kernel_vmo* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_READ**.

*bootimage_vmo* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_READ**.

## Return value

`zx_system_mexec()` shall not return upon success.

**ZX_ERR_NOT_SUPPORTED**  `kernel.enable-debugging-syscalls` is not set to `true`
on the kernel command line.

## See also

 - [`zx_system_mexec_payload_get()`]

[`zx_system_mexec_payload_get()`]: system_mexec_payload_get.md
