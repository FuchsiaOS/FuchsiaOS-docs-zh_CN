<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vmar_unmap

## Summary

Unmap virtual memory pages.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmar_unmap(zx_handle_t handle, zx_vaddr_t addr, size_t len);
```

## Description

`zx_vmar_unmap()` unmaps all VMO mappings and destroys (as if [`zx_vmar_destroy()`]
were called) all sub-regions within the absolute range including *addr* and ending
before exclusively at `addr + len`.  Any sub-region that is in the range must
be fully in the range (i.e. partial overlaps are an error).  If a mapping is
only partially in the range, the mapping is split and the requested portion is
unmapped.

*len* must be page-aligned.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_vmar_unmap()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a VMAR handle.

**ZX_ERR_INVALID_ARGS**  *addr* is not page-aligned, *len* is 0 or not page-aligned,
or the requested range partially overlaps a sub-region, or the requested range overlapped
a sub-region and *handle* did not have **ZX_RIGHT_OP_CHILDREN**.

**ZX_ERR_BAD_STATE**  *handle* refers to a destroyed handle.

**ZX_ERR_NOT_FOUND**  Could not find the requested mapping.

## Notes

## See also

 - [`zx_vmar_allocate()`]
 - [`zx_vmar_destroy()`]
 - [`zx_vmar_map()`]
 - [`zx_vmar_protect()`]

[`zx_vmar_allocate()`]: vmar_allocate.md
[`zx_vmar_destroy()`]: vmar_destroy.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmar_protect()`]: vmar_protect.md
