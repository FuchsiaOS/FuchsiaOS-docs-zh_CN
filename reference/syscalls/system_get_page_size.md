<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_system_get_page_size

## Summary

Get the page size for the system.

## Declaration

```c
#include <zircon/syscalls.h>

uint32_t zx_system_get_page_size(void);
```

## Description

`zx_system_get_page_size()` returns the base memory page size of the system in
bytes. This number cannot change during a run of the system, only at boot time,
and is guaranteed to be an exact power of 2.

The page size represents the allocation and alignment granularity of VMOs in
`zx_vmo_create()` and the smallest unit that can be mapped via `zx_vmar_map()`.

For every architecture there are well defined minimum and maximum values,
`ZX_MIN_PAGE_SIZE` and `ZX_MAX_PAGE_SIZE`, that this will return.

| Architecture | `ZX_MIN_PAGE_SIZE` | `ZX_MAX_PAGE_SIZE` |
| ------------ | ------------------ | ------------------ |
| ARM          | 4KiB               | 64KiB              |
| X86-64       | 4KiB               | 2MiB               |

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_system_get_page_size()` returns the page size in bytes.

## Errors

`zx_system_get_page_size()` cannot fail.

## Notes

## See also

 - [`zx_vmar_map()`]
 - [`zx_vmo_create()`]

[`zx_vmar_map()`]: vmar_map.md
[`zx_vmo_create()`]: vmo_create.md
