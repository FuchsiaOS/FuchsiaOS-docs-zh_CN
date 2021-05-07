# zx_channel_create

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Create a channel.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_channel_create(uint32_t options,
                              zx_handle_t* out0,
                              zx_handle_t* out1);
```

## DESCRIPTION

`zx_channel_create()` creates a channel, a bi-directional
datagram-style message transport capable of sending raw data bytes
as well as handles from one side to the other.

Two handles are returned on success, providing access to both sides
of the channel.  Messages written to one handle may be read from
the opposite.

The handles will have **ZX_RIGHT_TRANSFER** (allowing them to be sent
to another process via [`zx_channel_write()`]), **ZX_RIGHT_WRITE** (allowing
messages to be written to them), and **ZX_RIGHT_READ** (allowing messages
to be read from them).


## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_channel_create()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_INVALID_ARGS**  *out0* or *out1* is an invalid pointer or NULL or
*options* is any value other than 0.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## SEE ALSO

 - [`zx_channel_call()`]
 - [`zx_channel_read()`]
 - [`zx_channel_write()`]
 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_handle_replace()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_channel_call()`]: channel_call.md
[`zx_channel_read()`]: channel_read.md
[`zx_channel_write()`]: channel_write.md
[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
