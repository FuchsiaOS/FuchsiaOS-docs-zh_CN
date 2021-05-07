# zx_channel_read_etc

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Read a message from a channel.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_channel_read_etc(zx_handle_t handle,
                                uint32_t options,
                                void* bytes,
                                zx_handle_info_t* handles,
                                uint32_t num_bytes,
                                uint32_t num_handles,
                                uint32_t* actual_bytes,
                                uint32_t* actual_handles);
```

## DESCRIPTION

See [`zx_channel_read()`] for a full description.

Both forms of read behave the same except that [`zx_channel_read()`] returns an
array of raw `zx_handle_t` handle values while `zx_channel_read_etc()` returns
an array of `zx_handle_info_t` structures of the form:

```
typedef struct {
    zx_handle_t handle;     // handle value
    zx_obj_type_t type;     // type of object, see ZX_OBJ_TYPE_
    zx_rights_t rights;     // handle rights
    uint32_t unused;        // set to zero
} zx_handle_info_t;
```

When communicating to an untrusted party over a channel, it is recommended
that the `zx_channel_read_etc()` form is used and each handle type and rights
are validated against the expected values.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_CHANNEL** and have **ZX_RIGHT_READ**.

## RETURN VALUE

Both forms of read return **ZX_OK** on success, if *actual_bytes*
and *actual_handles* (if non-NULL), contain the exact number of bytes
and count of handles read.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a channel handle.

**ZX_ERR_INVALID_ARGS**  If any of *bytes*, *handles*, *actual_bytes*, or
*actual_handles* are non-NULL and an invalid pointer.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_READ**.

**ZX_ERR_SHOULD_WAIT**  The channel contained no messages to read.

**ZX_ERR_PEER_CLOSED**  The other side of the channel is closed.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_BUFFER_TOO_SMALL**  The provided *bytes* or *handles* buffers
are too small (in which case, the minimum sizes necessary to receive
the message will be written to *actual_bytes* and *actual_handles*,
provided they are non-NULL). If *options* has **ZX_CHANNEL_READ_MAY_DISCARD**
set, then the message is discarded.

## NOTES

*num_handles* and *actual_handles* are counts of the number of elements
in the *handles* array, not its size in bytes.

## SEE ALSO

 - [`zx_channel_call()`]
 - [`zx_channel_create()`]
 - [`zx_channel_read()`]
 - [`zx_channel_write()`]
 - [`zx_channel_write_etc()`]
 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_handle_replace()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_channel_call()`]: channel_call.md
[`zx_channel_create()`]: channel_create.md
[`zx_channel_read()`]: channel_read.md
[`zx_channel_write()`]: channel_write.md
[`zx_channel_write_etc()`]: channel_write_etc.md
[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
