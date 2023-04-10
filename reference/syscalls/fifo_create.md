<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_fifo_create

## Summary

Create a fifo.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_fifo_create(size_t elem_count,
                           size_t elem_size,
                           uint32_t options,
                           zx_handle_t* out0,
                           zx_handle_t* out1);
```

## Description

`zx_fifo_create()` creates a fifo, which is actually a pair of fifos
of *elem_count* entries of *elem_size* bytes.  Two endpoints are
returned.  Writing to one endpoint enqueues an element into the fifo
that the opposing endpoint reads from.

Fifos are intended to be the control plane for shared memory transports.
Their read and write operations are more efficient than *sockets* or
*channels*, but there are severe restrictions on the size of elements
and buffers.

The total size of each fifo (`elem_count * elem_size`) may not exceed 4096 bytes.

The *options* argument must be 0.

## Rights

Caller job policy must allow **ZX_POL_NEW_FIFO**.

## Return value

`zx_fifo_create()` returns **ZX_OK** on success. In the event of
failure, one of the following values is returned.

## Errors

**ZX_ERR_INVALID_ARGS**  *out0* or *out1* is an invalid pointer or NULL or
*options* is any value other than 0.

**ZX_ERR_OUT_OF_RANGE**  *elem_count* or *elem_size* is zero, or
*elem_count* * *elem_size* is greater than 4096.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## See also

 - [`zx_fifo_read()`]
 - [`zx_fifo_write()`]

[`zx_fifo_read()`]: fifo_read.md
[`zx_fifo_write()`]: fifo_write.md
