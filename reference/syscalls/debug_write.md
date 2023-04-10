<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_debug_write

## Summary

Write a message to the debug serial port.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_debug_write(const char* buffer, size_t buffer_size);
```

## Description

`zx_debug_write()` attempts to write data of *buffer_size* bytes to the debug serial port.

To use the `zx_debug_write()` function, you must specify
`kernel.enable-serial-syscalls=true` or
`kernel.enable-serial-syscalls=output-only` on the kernel command line.
Otherwise, the function returns **ZX_ERR_NOT_SUPPORTED**.

`zx_debug_write` is intended for diagnostic use.  Data may be dropped or
truncated, but the data from two different `zx_debug_write` calls will not be
interleaved or reordered.

## Rights

None.

## Return value

Returns **ZX_OK** on success.

## Errors

**ZX_ERR_NOT_SUPPORTED**  `kernel.enable-serial-syscalls` is not set to `true`
or `output-only` on the kernel command line.

**ZX_ERR_INVALID_ARGS** *buffer* is NULL.

## See also

 - [kernel command line]
 - [`zx_debug_read()`]
 - [`zx_debuglog_read()`]
 - [`zx_debuglog_write()`]

[kernel command line]: /docs/reference/kernel/kernel_cmdline.md
[`zx_debug_read()`]: debug_read.md
[`zx_debuglog_read()`]: debuglog_read.md
[`zx_debuglog_write()`]: debuglog_write.md
