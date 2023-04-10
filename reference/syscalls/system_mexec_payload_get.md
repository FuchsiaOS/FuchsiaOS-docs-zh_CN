<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_system_mexec_payload_get

## Summary

Return a ZBI containing ZBI entries necessary to boot this system.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_system_mexec_payload_get(zx_handle_t resource,
                                        void* buffer,
                                        size_t buffer_size);
```

## Description

`zx_system_mexec_payload_get()` accepts a resource handle and a
pointer/length corresponding to an output buffer. The head of the buffer is
overwritten with non-bootable ZBI containing a sequence of entries that should
be appended to a ZBI before passing that image to [`zx_system_mexec()`]; the
tail of the buffer is left untouched.

*resource* must be of type **ZX_RSRC_KIND_SYSTEM** with base
**ZX_RSRC_SYSTEM_MEXEC_BASE**.

*buffer* and *buffer_size* must point to a buffer that is no longer than 16KiB.

To use the `zx_system_mexec_payload_get()` function, you must specify
`kernel.enable-debugging-syscalls=true` on the kernel command line. Otherwise,
the function returns **ZX_ERR_NOT_SUPPORTED**.

## Rights

*resource* must have resource kind **ZX_RSRC_KIND_SYSTEM** with base
**ZX_RSRC_SYSTEM_MEXEC_BASE**.

## Return value

`zx_system_mexec_payload_get()` returns **ZX_OK** on success.

**ZX_ERR_NOT_SUPPORTED**  `kernel.enable-debugging-syscalls` is not set to `true`
on the kernel command line.

**ZX_ERR_BUFFER_TOO_SMALL**  If the provided buffer is too small for the ZBI.
In this case, the caller is expected to make the syscall again with a larger
buffer.

## See also

 - [`zx_system_mexec()`]

[`zx_system_mexec()`]: system_mexec.md
