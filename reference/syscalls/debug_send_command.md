<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_debug_send_command

## Summary

TODO(fxbug.dev/32938)

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_debug_send_command(zx_handle_t resource,
                                  const char* buffer,
                                  size_t buffer_size);
```

## Description

To use the `zx_debug_send_command()` function, you must specify
`kernel.enable-debugging-syscalls=true` on the kernel command line. Otherwise,
the function returns **ZX_ERR_NOT_SUPPORTED**.

TODO(fxbug.dev/32938)

## Rights

*resource* must have resource kind **ZX_RSRC_KIND_ROOT**.

## Return value

TODO(fxbug.dev/32938)

## Errors

TODO(fxbug.dev/32938)

## See also

TODO(fxbug.dev/32938)
