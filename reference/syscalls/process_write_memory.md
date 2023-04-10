<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_process_write_memory

## Summary

Write into the given process's address space.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_process_write_memory(zx_handle_t handle,
                                    zx_vaddr_t vaddr,
                                    const void* buffer,
                                    size_t buffer_size,
                                    size_t* actual);
```

## Description

`zx_process_write_memory()` attempts to write memory of the specified process.

This function will eventually be replaced with something vmo-centric.

*vaddr* the address of the block of memory to write.

*buffer* pointer to a user buffer containing the bytes to write.

*buffer_size* number of bytes to attempt to write. *buffer* buffer must be
large enough for at least this many bytes. *buffer_size* must be greater than
zero and less than or equal to 64MB.

*actual_size* the actual number of bytes written is stored here. Less bytes
than requested may be returned if *vaddr*+*buffer_size* extends beyond the
memory mapped in the process.

To use the `zx_process_write_memory()` function, you must specify
`kernel.enable-debugging-syscalls=true` on the kernel command line. Otherwise,
the function returns **ZX_ERR_NOT_SUPPORTED**.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_process_write_memory()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned, and the number of
bytes written to *buffer* is undefined.

## Errors

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the **ZX_RIGHT_WRITE** right or
the address range to write falls into a protected area like the vDSO.

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_BAD_STATE**  the process's memory is not accessible (e.g.,
the process is being terminated),
or the requested memory is not cacheable.

**ZX_ERR_INVALID_ARGS**  *buffer* is an invalid pointer or NULL,
or *buffer_size* is zero or greater than 64MB.

**ZX_ERR_NO_MEMORY**  the process does not have any memory at the
requested address.

**ZX_ERR_NOT_SUPPORTED**  `kernel.enable-debugging-syscalls` is not set to `true`
on the kernel command line.

**ZX_ERR_WRONG_TYPE**  *handle* is not a process handle.

## See also

 - [`zx_process_read_memory()`]

[`zx_process_read_memory()`]: process_read_memory.md
