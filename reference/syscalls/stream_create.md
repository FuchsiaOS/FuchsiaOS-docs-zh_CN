# zx_stream_create

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Create a stream from a VMO.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_stream_create(uint32_t options,
                             zx_handle_t vmo,
                             zx_off_t seek,
                             zx_handle_t* out_stream);
```

## DESCRIPTION

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

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_stream_create()` returns **ZX_OK** on success. In the event of
failure, one of the following values is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *vmo* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *vmo* is not a VMO handle.

**ZX_ERR_ACCESS_DENIED**  *vmo* does not have the rights required for the given
options.

**ZX_ERR_INVALID_ARGS**  *out_stream* is an invalid pointer or NULL, *options*
has an unsupported bit set to 1.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.

## SEE ALSO

 - [`zx_stream_readv()`]
 - [`zx_stream_readv_at()`]
 - [`zx_stream_seek()`]
 - [`zx_stream_writev()`]
 - [`zx_stream_writev_at()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_stream_readv()`]: stream_readv.md
[`zx_stream_readv_at()`]: stream_readv_at.md
[`zx_stream_seek()`]: stream_seek.md
[`zx_stream_writev()`]: stream_writev.md
[`zx_stream_writev_at()`]: stream_writev_at.md
