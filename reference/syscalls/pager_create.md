<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_pager_create

## Summary

Create a new pager object.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_pager_create(uint32_t options, zx_handle_t* out);
```

## Description

`zx_pager_create()` creates a new pager object.

When a pager object is destroyed, any accesses to its vmos that would have required communicating
with the pager will fail as if [`zx_pager_detach_vmo()`] had been called. Furthermore, the kernel
will make an effort to ensure that the faults happen as quickly as possible (e.g. by evicting
present pages), but the precise behavior is implementation dependent.

## Rights

Caller job policy must allow **ZX_POL_NEW_PAGER**.

## Return value

`zx_pager_create()` returns ZX_OK on success, or one of the following error codes on failure.

## Errors

**ZX_ERR_INVALID_ARGS** *out* is an invalid pointer or NULL or *options* is
any value other than 0.

**ZX_ERR_NO_MEMORY** Failure due to lack of memory.

## See also

 - [`zx_pager_create_vmo()`]
 - [`zx_pager_detach_vmo()`]
 - [`zx_pager_supply_pages()`]

[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_pager_detach_vmo()`]: pager_detach_vmo.md
[`zx_pager_supply_pages()`]: pager_supply_pages.md
