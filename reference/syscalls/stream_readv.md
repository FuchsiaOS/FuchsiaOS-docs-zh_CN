<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_stream_readv

## Summary

Read data from a stream at the current seek offset.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_stream_readv(zx_handle_t handle,
                            uint32_t options,
                            zx_iovec_t* vectors,
                            size_t num_vectors,
                            size_t* actual);
```

## Description

`zx_stream_readv()` attempts to read bytes from the stream, starting at the
current seek offset, into the buffers specified by *vectors* and *num_vectors*.
If successful, the number of bytes actually read are return via *actual*.

If the current seek offset is beyond the end of the stream, `zx_stream_readv()`
will succeed in reading zero bytes.

If a NULL *actual* is passed in, it will be ignored.

Advances the seek offset of the stream by the actual number of bytes read.
If the read fails, the seek offset could either remain the same or have
been changed to an unspecified value.

If the contents of *vectors* change during this operation, if any of the buffers
overlap, or if any of the buffers overlap *vectors*, the behavior is unspecified.

*options* is reserved for future use and must be 0.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_STREAM** and have **ZX_RIGHT_READ**.

## Return value

`zx_stream_readv()` returns **ZX_OK** on success, and writes into
*actual* (if non-NULL) the exact number of bytes read.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a stream handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the **ZX_RIGHT_READ** right.

**ZX_ERR_INVALID_ARGS**  *vectors* is an invalid `zx_iovec_t` or *options* is
nonzero.

**ZX_ERR_NOT_FOUND**  the *vectors* address, or an address specified within
*vectors* does not map to address in address space.

**ZX_ERR_BAD_STATE**  the underlying data source cannot be read.

## See also

 - [`zx_stream_create()`]
 - [`zx_stream_readv_at()`]
 - [`zx_stream_seek()`]
 - [`zx_stream_writev()`]
 - [`zx_stream_writev_at()`]

[`zx_stream_create()`]: stream_create.md
[`zx_stream_readv_at()`]: stream_readv_at.md
[`zx_stream_seek()`]: stream_seek.md
[`zx_stream_writev()`]: stream_writev.md
[`zx_stream_writev_at()`]: stream_writev_at.md
