# zx_fifo_create

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Create a fifo.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_fifo_create(size_t elem_count,
                           size_t elem_size,
                           uint32_t options,
                           zx_handle_t* out0,
                           zx_handle_t* out1);
```

## DESCRIPTION

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

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_fifo_create()` returns **ZX_OK** on success. In the event of
failure, one of the following values is returned.

## ERRORS

**ZX_ERR_INVALID_ARGS**  *out0* or *out1* is an invalid pointer or NULL or
*options* is any value other than 0.

**ZX_ERR_OUT_OF_RANGE**  *elem_count* or *elem_size* is zero, or
*elem_count* * *elem_size* is greater than 4096.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.


## SEE ALSO

 - [`zx_fifo_read()`]
 - [`zx_fifo_write()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_fifo_read()`]: fifo_read.md
[`zx_fifo_write()`]: fifo_write.md
