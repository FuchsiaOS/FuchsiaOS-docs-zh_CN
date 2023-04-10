<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vmar_destroy

## Summary

Destroy a virtual memory address region.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmar_destroy(zx_handle_t handle);
```

## Description

`zx_vmar_destroy()` unmaps all mappings within the given region, and destroys
all sub-regions of the region.  Note that this operation is logically recursive.

This operation does not close *handle*.  Any outstanding handles to this
VMAR will remain valid handles, but all VMAR operations on them will fail.

The root VMAR, as obtained by `zx_process_create()`, cannot be destroyed.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_vmar_destroy()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_ACCESS_DENIED** *handle* does not have **ZX_RIGHT_OP_CHILDREN**.

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a VMAR handle.

**ZX_ERR_BAD_STATE**  This region is already destroyed.

**ZX_ERR_NOT_SUPPORTED** *handle* is a root VMAR.

## Notes

## See also

 - [`zx_vmar_allocate()`]
 - [`zx_vmar_map()`]
 - [`zx_vmar_protect()`]
 - [`zx_vmar_unmap()`]

[`zx_vmar_allocate()`]: vmar_allocate.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmar_protect()`]: vmar_protect.md
[`zx_vmar_unmap()`]: vmar_unmap.md
