<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vmo_replace_as_executable

## SUMMARY

Add execute rights to a VMO.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_replace_as_executable(zx_handle_t handle,
                                         zx_handle_t vmex,
                                         zx_handle_t* out);
```

## Description

`zx_vmo_replace_as_executable()` creates a replacement for *handle*, referring
to the same underlying [virtual memory object](/reference/kernel_objects/vm_object.md) (VMO),
adding the right **ZX_RIGHT_EXECUTE**.

*handle* is always invalidated.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_VMO**.

*vmex* must have resource kind **ZX_RSRC_KIND_SYSTEM** with base
**ZX_RSRC_SYSTEM_VMEX_BASE**.

## Return value

`zx_vmo_replace_as_executable()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* isn't a valid VM object handle, or
*vmex* isn't a valid **ZX_RSRC_KIND_SYSTEM** resource handle with base
**ZX_RSRC_SYSTEM_VMEX_BASE**.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## See also

 - [`zx_resource_create()`]
 - [`zx_vmar_map()`]

[`zx_resource_create()`]: resource_create.md
[`zx_vmar_map()`]: vmar_map.md
