<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_process_read_memory

## Summary

Read from the given process's address space.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_process_read_memory(zx_handle_t handle,
                                   zx_vaddr_t vaddr,
                                   void* buffer,
                                   size_t buffer_size,
                                   size_t* actual);
```

## Description

`zx_process_read_memory()` attempts to read memory of the specified process.

This function will eventually be replaced with something vmo-centric.

*vaddr* the address of the block of memory to read.

*buffer* pointer to a user buffer to read bytes into.

*buffer_size* number of bytes to attempt to read. *buffer* buffer must be large
enough for at least this many bytes. *buffer_size* must be greater than zero
and less than or equal to 64MB.

*actual* the actual number of bytes read is stored here. Less bytes than
requested may be returned if *vaddr*+*buffer_size* extends beyond the memory
mapped in the process.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_READ** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_process_read_memory()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned, and the number of
bytes written to *buffer* is undefined.

## Errors

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the **ZX_RIGHT_READ** right
or
**ZX_WRITE_RIGHT** is needed for historical reasons.

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_BAD_STATE**  the process's memory is not accessible (e.g.,
the process is being terminated),
or the requested memory is not cacheable.

**ZX_ERR_INVALID_ARGS** *buffer* is an invalid pointer or NULL,
or *buffer_size* is zero or greater than 64MB.

**ZX_ERR_NO_MEMORY** the process does not have any memory at the
requested address.

**ZX_ERR_WRONG_TYPE**  *handle* is not a process handle.

## See also

 - [`zx_process_write_memory()`]

[`zx_process_write_memory()`]: process_write_memory.md
