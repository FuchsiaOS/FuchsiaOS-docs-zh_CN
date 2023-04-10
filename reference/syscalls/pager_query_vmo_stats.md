<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_pager_query_vmo_stats

## NAME

Query pager related statistics on a pager owned VMO.

## SYNOPSIS

```c
#include <zircon/syscalls.h>

zx_status_t zx_pager_query_vmo_stats(zx_handle_t pager,
                                     zx_handle_t pager_vmo,
                                     uint32_t options,
                                     void* buffer,
                                     size_t buffer_size);
```

## Description

Queries *pager_vmo* for any pager related statistics, e.g. whether *pager_vmo* has been modified.
The *pager_vmo* must have previously been created from the *pager* by [`zx_pager_create_vmo()`].

*options* can be **ZX_PAGER_RESET_VMO_STATS** if the caller also wishes to reset the queried stats.
An *options* value of 0 does not reset any state, and performs a pure query.

*buffer* should be a pointer to a `zx_pager_vmo_stats_t` struct that will hold the result of the
query, and *buffer_size* should be large enough to accommodate the struct.

```c
typedef struct zx_pager_vmo_stats {
  // Will be set to ZX_PAGER_VMO_STATS_MODIFIED if the VMO was modified, or 0 otherwise.
  // Note that this can be set to 0 if a previous zx_pager_query_vmo_stats() call specified the
  // ZX_PAGER_RESET_VMO_STATS option, which resets the modified state.
  uint32_t modified;
} zx_pager_vmo_stats_t;
```

Note that this call can have an effect on future `zx_pager_query_vmo_stats()` calls by consuming
queryable state if the **ZX_PAGER_RESET_VMO_STATS** option is specified. For example, if a
`zx_vmo_write()` is followed by two consecutive `zx_pager_query_vmo_stats()` with the
**ZX_PAGER_RESET_VMO_STATS** option, only the first of those will see `modified` set to
**ZX_PAGER_VMO_STATS_MODIFIED**. Since no further modifications took place after the first
`zx_pager_query_vmo_stats()`, the second `zx_pager_query_vmo_stats()` will return `modified` as 0.

## Rights

*pager* must be of type **ZX_OBJ_TYPE_PAGER**.

*pager_vmo* must be of type **ZX_OBJ_TYPE_VMO**.

## Return value

`zx_pager_query_vmo_stats()` returns **ZX_OK** on success. In the event of failure, a negative error
value is returned.

## Errors

**ZX_ERR_BAD_HANDLE** *pager* or *pager_vmo* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *pager* is not a pager handle, or *pager_vmo* is not a vmo handle.

**ZX_ERR_INVALID_ARGS**  *pager_vmo* is not a vmo created from *pager*, or *options* is neither 0 or
**ZX_PAGER_RESET_VMO_STATS**.

## See also

 - [`zx_pager_create_vmo()`]
 - [`zx_pager_detach_vmo()`]
 - [`zx_pager_op_range()`]
 - [`zx_pager_query_dirty_ranges()`]
 - [`zx_pager_supply_pages()`]

[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_pager_detach_vmo()`]: pager_detach_vmo.md
[`zx_pager_op_range()`]: pager_op_range.md
[`zx_pager_query_dirty_ranges()`]: pager_query_dirty_ranges.md
[`zx_pager_supply_pages()`]: pager_supply_pages.md
