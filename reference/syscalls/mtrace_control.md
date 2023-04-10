<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_mtrace_control

## Summary

TODO(fxbug.dev/32938)

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_mtrace_control(zx_handle_t handle,
                              uint32_t kind,
                              uint32_t action,
                              uint32_t options,
                              void* ptr,
                              size_t ptr_size);
```

## Description

To use the `zx_mtrace_control()` function, you must specify
`kernel.enable-debugging-syscalls=true` on the kernel command line. Otherwise,
the function returns **ZX_ERR_NOT_SUPPORTED**.

TODO(fxbug.dev/32938)

## Rights

*handle* must have resource kind **ZX_RSRC_KIND_ROOT**.

## Return value

TODO(fxbug.dev/32938)

## Errors

TODO(fxbug.dev/32938)

## See also

TODO(fxbug.dev/32938)
