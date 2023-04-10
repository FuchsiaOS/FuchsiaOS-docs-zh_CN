<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_fifo_read

## Summary

Read data from a fifo.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_fifo_read(zx_handle_t handle,
                         size_t elem_size,
                         void* data,
                         size_t count,
                         size_t* actual_count);
```

## Description

`zx_fifo_read()` attempts to read up to *count* elements from the fifo
*handle* into *data*.

Fewer elements may be read than requested if there are insufficient
elements in the fifo to fulfill the entire request. The number of
elements actually read is returned via *actual_count*.

The element size specified by *elem_size* must match the element size
that was passed into [`zx_fifo_create()`].

*data* must have a size of at least `count * elem_size` bytes.

*actual_count* is allowed to be NULL. This is useful when reading
a single element: if *count* is 1 and `zx_fifo_read()` returns **ZX_OK**,
*actual_count* is guaranteed to be 1 and thus can be safely ignored.

It is not legal to read zero elements.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_FIFO** and have **ZX_RIGHT_READ**.

## Return value

`zx_fifo_read()` returns **ZX_OK** on success, and returns
the number of elements read (at least one) via *actual_count*.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a fifo handle.

**ZX_ERR_INVALID_ARGS**  *data* is an invalid pointer or *actual_count*
is an invalid pointer.

**ZX_ERR_OUT_OF_RANGE**  *count* is zero or *elem_size* is not equal
to the element size of the fifo.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_READ**.

**ZX_ERR_PEER_CLOSED**  The other side of the fifo is closed.

**ZX_ERR_SHOULD_WAIT**  The fifo is empty.

## See also

 - [`zx_fifo_create()`]
 - [`zx_fifo_write()`]

[`zx_fifo_create()`]: fifo_create.md
[`zx_fifo_write()`]: fifo_write.md
