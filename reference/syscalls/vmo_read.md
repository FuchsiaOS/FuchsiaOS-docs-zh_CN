<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vmo_read

## Summary

Read bytes from the VMO.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_read(zx_handle_t handle,
                        void* buffer,
                        uint64_t offset,
                        size_t buffer_size);
```

## Description

`zx_vmo_read()` attempts to read exactly *buffer_size* bytes from a [virtual memory
object](/docs/reference/kernel_objects/vm_object.md) (VMO) at *offset*.

*buffer* pointer to a user buffer to read bytes into.

*buffer_size* number of bytes to attempt to read. *buffer* buffer should be large
enough for at least this many bytes.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_READ**.

## Return value

`zx_vmo_read()` returns **ZX_OK** on success, and exactly *buffer_size* bytes will
have been written to *buffer*.
In the event of failure, a negative error value is returned, and the number of
bytes written to *buffer* is undefined.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a VMO handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the **ZX_RIGHT_READ** right.

**ZX_ERR_INVALID_ARGS**  *buffer* is an invalid pointer or NULL.

**ZX_ERR_NOT_FOUND** *buffer* address does not map to address in address space.

**ZX_ERR_OUT_OF_RANGE**  *offset* + *buffer_size* is greater than the size of
                         the VMO.

**ZX_ERR_BAD_STATE**  VMO has been marked uncached and is not directly readable, or the VMO is
backed by a pager and the pager or the VMO is in a bad state preventing requested pages from being
populated.

**ZX_ERR_IO** The VMO is backed by a pager and the pager encountered an I/O error while reading in
the requested pages.

**ZX_ERR_IO_DATA_INTEGRITY** The VMO is backed by a pager and the contents that were read in by the
pager are corrupted.

## See also

 - [`zx_vmo_create()`]
 - [`zx_vmo_create_child()`]
 - [`zx_vmo_get_size()`]
 - [`zx_vmo_op_range()`]
 - [`zx_vmo_set_cache_policy()`]
 - [`zx_vmo_set_size()`]
 - [`zx_vmo_write()`]

[`zx_vmo_create()`]: vmo_create.md
[`zx_vmo_create_child()`]: vmo_create_child.md
[`zx_vmo_get_size()`]: vmo_get_size.md
[`zx_vmo_op_range()`]: vmo_op_range.md
[`zx_vmo_set_cache_policy()`]: vmo_set_cache_policy.md
[`zx_vmo_set_size()`]: vmo_set_size.md
[`zx_vmo_write()`]: vmo_write.md
