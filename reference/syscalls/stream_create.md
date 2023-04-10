<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_stream_create

## Summary

Create a stream from a VMO.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_stream_create(uint32_t options,
                             zx_handle_t vmo,
                             zx_off_t seek,
                             zx_handle_t* out_stream);
```

## Description

`zx_stream_create()` creates a stream, which reads and writes the data in an
underlying VMO.

The seek offset for the stream is initialized to *seek*.

# Options

**ZX_STREAM_MODE_READ**  The stream will be used for reading.  If the given
*vmo* lacks **ZX_RIGHT_READ**, this function will return
**ZX_ERR_ACCESS_DENIED**.  Otherwise, **ZX_RIGHT_READ** will be included as a
right on the created stream object.

**ZX_STREAM_MODE_WRITE**  The stream will be used for writing.  If the given
*vmo* lacks **ZX_RIGHT_WRITE**, this function will return
**ZX_ERR_ACCESS_DENIED**.  Otherwise, **ZX_RIGHT_WRITE** will be included as a
right on the created stream object.

**ZX_STREAM_MODE_APPEND**  The stream is created in append mode. A stream in
append mode will atomically set the seek offset of the stream to the content
size of the stream prior to writing data in `zx_stream_writev()`.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_stream_create()` returns **ZX_OK** on success. In the event of
failure, one of the following values is returned.

## Errors

**ZX_ERR_BAD_HANDLE**  *vmo* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *vmo* is not a VMO handle.

**ZX_ERR_ACCESS_DENIED**  *vmo* does not have the rights required for the given
options.

**ZX_ERR_INVALID_ARGS**  *out_stream* is an invalid pointer or NULL, *options*
has an unsupported bit set to 1.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.

## See also

 - [`zx_stream_readv()`]
 - [`zx_stream_readv_at()`]
 - [`zx_stream_seek()`]
 - [`zx_stream_writev()`]
 - [`zx_stream_writev_at()`]

[`zx_stream_readv()`]: stream_readv.md
[`zx_stream_readv_at()`]: stream_readv_at.md
[`zx_stream_seek()`]: stream_seek.md
[`zx_stream_writev()`]: stream_writev.md
[`zx_stream_writev_at()`]: stream_writev_at.md
