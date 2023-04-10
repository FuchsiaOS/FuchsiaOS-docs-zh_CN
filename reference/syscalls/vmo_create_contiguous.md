<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vmo_create_contiguous

## SUMMARY

TODO(fxbug.dev/32938)

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_create_contiguous(zx_handle_t bti,
                                     size_t size,
                                     uint32_t alignment_log2,
                                     zx_handle_t* out);
```

## Description

TODO(fxbug.dev/32938)

## Rights

*bti* must be of type **ZX_OBJ_TYPE_BTI** and have **ZX_RIGHT_MAP**.

## Return value

TODO(fxbug.dev/32938)

## Errors

TODO(fxbug.dev/32938)

## See also

TODO(fxbug.dev/32938)
