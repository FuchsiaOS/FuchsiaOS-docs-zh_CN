# zx_stream_seek

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Modify the seek offset.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_stream_seek(zx_handle_t handle,
                           zx_stream_seek_origin_t whence,
                           int64_t offset,
                           zx_off_t* out_seek);
```

## DESCRIPTION

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

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_STREAM** and have **ZX_RIGHT_WRITE** or have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_stream_seek()` returns **ZX_OK** on success, and writes the resulting seek
offset, relative to the start of the stream, into *out_offset* (if non-NULL).

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a stream handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the **ZX_RIGHT_READ** or
**ZX_RIGHT_WRITE** right.

**ZX_ERR_INVALID_ARGS**  *whence* is an invalid `zx_stream_seek_origin_t` or
the resulting seek would be negative or exceed the maximum representable
`zx_off_t`.

## SEE ALSO

 - [`zx_stream_create()`]
 - [`zx_stream_readv()`]
 - [`zx_stream_readv_at()`]
 - [`zx_stream_writev()`]
 - [`zx_stream_writev_at()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_stream_create()`]: stream_create.md
[`zx_stream_readv()`]: stream_readv.md
[`zx_stream_readv_at()`]: stream_readv_at.md
[`zx_stream_writev()`]: stream_writev.md
[`zx_stream_writev_at()`]: stream_writev_at.md
