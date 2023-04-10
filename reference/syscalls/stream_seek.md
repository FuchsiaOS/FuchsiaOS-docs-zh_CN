<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_stream_seek

## Summary

Modify the seek offset.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_stream_seek(zx_handle_t handle,
                           zx_stream_seek_origin_t whence,
                           int64_t offset,
                           zx_off_t* out_seek);
```

## Description

`zx_stream_seek()` sets the seek offset of the stream to *offset* relative to
*whence*.

If the resulting seek offset were to be negative or exceed the maximum
representable `zx_off_t`, `zx_stream_seek()` returns **ZX_ERR_INVALID_ARGS**.

The resulting seek offset might extend beyond the end of the stream. Setting
such a seek offset does not cause `zx_stream_seek()` to return an error, but
attempting to read or write data at that seek offset might generate an error.

## WHENCE

**ZX_STREAM_SEEK_ORIGIN_START**  set the seek offset relative to the start of
the stream.

**ZX_STREAM_SEEK_ORIGIN_CURRENT**  set the seek offset relative to the current
seek offset of the stream.

**ZX_STREAM_SEEK_ORIGIN_END**  set the seek offset relative to the end of the
stream, as defined by the content size of the stream.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_STREAM** and have **ZX_RIGHT_WRITE** or have **ZX_RIGHT_WRITE**.

## Return value

`zx_stream_seek()` returns **ZX_OK** on success, and writes the resulting seek
offset, relative to the start of the stream, into *out_offset* (if non-NULL).

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a stream handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the **ZX_RIGHT_READ** or
**ZX_RIGHT_WRITE** right.

**ZX_ERR_INVALID_ARGS**  *whence* is an invalid `zx_stream_seek_origin_t` or
the resulting seek would be negative or exceed the maximum representable
`zx_off_t`.

## See also

 - [`zx_stream_create()`]
 - [`zx_stream_readv()`]
 - [`zx_stream_readv_at()`]
 - [`zx_stream_writev()`]
 - [`zx_stream_writev_at()`]

[`zx_stream_create()`]: stream_create.md
[`zx_stream_readv()`]: stream_readv.md
[`zx_stream_readv_at()`]: stream_readv_at.md
[`zx_stream_writev()`]: stream_writev.md
[`zx_stream_writev_at()`]: stream_writev_at.md
