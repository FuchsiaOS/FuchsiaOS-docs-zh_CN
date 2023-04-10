<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vmo_write

## Summary

Write bytes to the VMO.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_write(zx_handle_t handle,
                         const void* buffer,
                         uint64_t offset,
                         size_t buffer_size);
```

## Description

`zx_vmo_write()` attempts to write exactly *buffer_size* bytes to a [virtual memory
object](/docs/reference/kernel_objects/vm_object.md) (VMO) at *offset*.

*buffer* pointer to a user buffer to write bytes from.

*buffer_size* number of bytes to attempt to write.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_vmo_write()` returns **ZX_OK** on success, and exactly *buffer_size* bytes will
have been written from *buffer*.
In the event of failure, a negative error value is returned, and the number of
bytes written from *buffer* is undefined.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a VMO handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the **ZX_RIGHT_WRITE** right.

**ZX_ERR_INVALID_ARGS**  *buffer* is an invalid pointer or NULL.

**ZX_ERR_NOT_FOUND** *buffer* address does not map to address in address space.

**ZX_ERR_NO_MEMORY**  Failure to allocate system memory to complete write.

**ZX_ERR_OUT_OF_RANGE**  *offset* + *buffer_size* is greater than the size of
                         the VMO.

**ZX_ERR_BAD_STATE**  VMO has been marked uncached and is not directly writable.

## See also

 - [`zx_vmo_create()`]
 - [`zx_vmo_create_child()`]
 - [`zx_vmo_get_size()`]
 - [`zx_vmo_op_range()`]
 - [`zx_vmo_read()`]
 - [`zx_vmo_set_cache_policy()`]
 - [`zx_vmo_set_size()`]

[`zx_vmo_create()`]: vmo_create.md
[`zx_vmo_create_child()`]: vmo_create_child.md
[`zx_vmo_get_size()`]: vmo_get_size.md
[`zx_vmo_op_range()`]: vmo_op_range.md
[`zx_vmo_read()`]: vmo_read.md
[`zx_vmo_set_cache_policy()`]: vmo_set_cache_policy.md
[`zx_vmo_set_size()`]: vmo_set_size.md
