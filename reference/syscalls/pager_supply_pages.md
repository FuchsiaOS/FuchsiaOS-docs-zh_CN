<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_pager_supply_pages

## Summary

Supply pages into a pager owned vmo.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_pager_supply_pages(zx_handle_t pager,
                                  zx_handle_t pager_vmo,
                                  uint64_t offset,
                                  uint64_t length,
                                  zx_handle_t aux_vmo,
                                  uint64_t aux_offset);
```

## Description

Moves the pages of *aux_vmo* in the range [*aux_offset*, *aux_offset* + *length*) to *pager_vmo* in
the range [*offset*, *offset* + *length*). Any pages in *pager_vmo* in the specified range will not
be replaced; instead the corresponding pages from *aux_vmo* will be freed. *aux_vmo* must have been
created by [`zx_vmo_create()`], must have no children, and must have no pinned pages in the
specified range. Any uncommitted pages in *aux_vmo* will cause zero pages, or equivalent, to be
inserted into *pager_vmo*. After this operation, the specified region of *aux_vmo* will be fully
decommitted.

## Rights

*pager* must be of type **ZX_OBJ_TYPE_PAGER**.

*pager_vmo* must be of type **ZX_OBJ_TYPE_VMO**.

*aux_vmo* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_READ** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_pager_supply_pages()` returns ZX_OK on success, or one of the following error codes on failure.
On failure the specified range of *aux_vmo* may be either untouched or fully decommitted. If
*aux_vmo* is decommitted, then an unspecified number of pages in *pager_vmo* will have been
populated.

## Errors

**ZX_ERR_BAD_HANDLE** *pager*, *pager_vmo*, or *aux_vmo* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *pager* is not a pager handle, *pager_vmo* is not a vmo handle, or
*aux_vmo* is not a vmo handle.

**ZX_ERR_INVALID_ARGS**  *pager_vmo* is not a vmo created from *pager*, or *offset*, *size*,
or *aux_offset* is not page aligned.

**ZX_ERR_ACCESS_DENIED** *aux_vmo* is does not have **ZX_RIGHT_WRITE** or **ZX_RIGHT_READ**.

**ZX_ERR_BAD_STATE** *aux_vmo* is not in a state where it can supply the required pages.

**ZX_ERR_NOT_SUPPORTED** *aux_vmo* is a physical vmo or a contiguous vmo.

**ZX_ERR_OUT_OF_RANGE** The specified range in *pager_vmo* or *aux_vmo* is invalid.

**ZX_ERR_NO_MEMORY** Failure due to lack of memory.

## See also

 - [`zx_pager_create_vmo()`]
 - [`zx_pager_detach_vmo()`]
 - [`zx_pager_op_range()`]

[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_pager_detach_vmo()`]: pager_detach_vmo.md
[`zx_pager_op_range()`]: pager_op_range.md
[`zx_vmo_create()`]: vmo_create.md
