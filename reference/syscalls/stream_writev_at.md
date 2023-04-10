<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_stream_writev_at

## Summary

Write data to a stream at the given offset.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_stream_writev_at(zx_handle_t handle,
                                uint32_t options,
                                zx_off_t offset,
                                const zx_iovec_t* vectors,
                                size_t num_vectors,
                                size_t* actual);
```

## Description

`zx_stream_writev_at()` attempts to write bytes to the stream, starting at the
given *offset*, from the buffers specified by *vectors* and *num_vectors*.
If successful, the number of bytes actually written are return via *actual*.

If the write operation would write beyond the end of the stream, the function
will attempt to increase the content size of the stream in order to receive the
given data, filling any new, unwritten content with zero bytes.

If the resize operation fails after some amount of data was written to the
stream, the function will return successfully.  If no bytes were written to
stream, the operation will return **ZX_ERR_FILE_BIG** or **ZX_ERR_NO_SPACE**,
as appropriate.

If a NULL *actual* is passed in, it will be ignored.

Does not advance the seek offset of the stream.

If the contents of *vectors* change during this operation, if any of the buffers
overlap, or if any of the buffers overlap *vectors*, the behavior is unspecified.

*options* is reserved for future use and must be 0.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_STREAM** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_stream_writev_at()` returns **ZX_OK** on success, and writes into
*actual* (if non-NULL) the exact number of bytes written.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a stream handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the ZX_RIGHT_WRITE right.

**ZX_ERR_INVALID_ARGS** *vectors* is an invalid zx_iovec_t or *options* has an
unsupported bit set to 1.

**ZX_ERR_NOT_FOUND**  the *vectors* address, or an address specified within
*vectors* does not map to address in address space.

**ZX_ERR_BAD_STATE**  the underlying data source cannot be written.

**ZX_ERR_FILE_BIG**  the stream has exceeded a predefined maximum size limit.

**ZX_ERR_NO_SPACE**  the underlying storage medium does not have sufficient space.

## See also

 - [`zx_stream_create()`]
 - [`zx_stream_readv()`]
 - [`zx_stream_readv_at()`]
 - [`zx_stream_seek()`]
 - [`zx_stream_writev()`]

[`zx_stream_create()`]: stream_create.md
[`zx_stream_readv()`]: stream_readv.md
[`zx_stream_readv_at()`]: stream_readv_at.md
[`zx_stream_seek()`]: stream_seek.md
[`zx_stream_writev()`]: stream_writev.md
