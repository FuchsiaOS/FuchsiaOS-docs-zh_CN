# zx_channel_call_etc

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Send a message to a channel and await a reply.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_channel_call_etc(zx_handle_t handle,
                                uint32_t options,
                                zx_time_t deadline,
                                zx_channel_call_etc_args_t* args,
                                uint32_t* actual_bytes,
                                uint32_t* actual_handles);
```

## DESCRIPTION

`zx_channel_call_etc()` writes a request to a channel and blocks until it
receives a response. It is an extension of [`zx_channel_call()`] that
incorporates the functionality of [`zx_channel_write_etc()`] and
[`zx_channel_read_etc()`] for write and read phases, instead of the more basic
[`zx_channel_write()`] and  [`zx_channel_read()`]. See [`zx_channel_call()`] for
a full description of channel calls.

The effect of a call to `zx_channel_call_etc()` is similar to performing a
sequence of calls to [`zx_channel_write_etc()`], [`zx_object_wait_one()`] and
[`zx_channel_read_etc()`] in that order. However, a key difference is that
`zx_channel_call_etc()` will wait for an incoming message matches the outgoing
message's transaction id. The arguments that would be supplied to
`zx_channel_read_etc()` and `zx_channel_write_etc()` are instead specified with
`zx_channel_call_etc_args_t`:

```
typedef struct {
    const void* wr_bytes;
    zx_handle_disposition_t* wr_handles;
    void *rd_bytes;
    zx_handle_info_t* rd_handles;
    uint32_t wr_num_bytes;
    uint32_t wr_num_handles;
    uint32_t rd_num_bytes;
    uint32_t rd_num_handles;
} zx_channel_call_etc_args_t;
```

### ZX_CHANNEL_WRITE_USE_IOVEC option

When the **ZX_CHANNEL_WRITE_USE_IOVEC** option is specified, `wr_bytes` is
interpreted as an array of `zx_channel_iovec_t`, specifying slices of bytes to
sequentially copy to the message in order. `num_wr_bytes` specifies the number
of `zx_channel_iovec_t` array elements in `wr_bytes`.

```c
typedef struct zx_channel_iovec {
  const void* buffer;      // User-space bytes.
  uint32_t capacity;       // Number of bytes.
  uint32_t reserved;       // Reserved.
} zx_channel_iovec_t;
```

There can be at most **ZX_CHANNEL_MAX_MSG_IOVEC** or `8192`
`zx_channel_iovec_t` elements of the `wr_bytes` array with the sum of
`capacity` across all `zx_channel_iovec_t` not exceeding
**ZX_CHANNEL_MAX_MSG_BYTES** or `65536` bytes. `buffer` need not be aligned and
it may only be `NULL` if `capacity` is zero. `reserved` must be set to zero.

Either all `zx_channel_iovec_t` are copied and the message is sent, or none
are copied and the message is not sent. Usage for sending handles is unchanged.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_CHANNEL** and have **ZX_RIGHT_READ** and have **ZX_RIGHT_WRITE**.

All wr_handles of *args* must have **ZX_RIGHT_TRANSFER**.

## RETURN VALUE

`zx_channel_call_etc()` returns **ZX_OK** on success and the number of bytes and
count of handles in the reply message are returned via *actual_bytes* and
*actual_handles*, respectively.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle, any element in
*handles* is not a valid handle, or there are duplicates among the handles
in the *handles* array.

**ZX_ERR_WRONG_TYPE**  *handle* is not a channel handle, or any source
handle in *wr_handles* did not match the object type type.

**ZX_ERR_INVALID_ARGS**  any of the provided pointers are invalid or null,
or *wr_num_bytes* is less than four, or *options* is nonzero, or any source
handle in *wr_handles\[i\]->handle* did not have the rights specified in
*wr_handle\[i\]->rights*.
If the **ZX_CHANNEL_WRITE_USE_IOVEC** option is specified,
**ZX_ERR_INVALID_ARGS** will be produced if the *buffer* field contains an
invalid pointer or if the reserved field is non-zero.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_WRITE** or
any element in *handles* does not have **ZX_RIGHT_TRANSFER**.

**ZX_ERR_PEER_CLOSED**  The other side of the channel was closed or became
closed while waiting for the reply.

**ZX_ERR_CANCELED**  *handle* was closed while waiting for a reply. TODO(fxbug.dev/34013):
Transferring a channel with pending calls currently leads to undefined behavior. With
the current implementation, transferring such a channel does not interrupt the
pending calls, as it does not close the underlying channel endpoint. Programs should
be aware of this behavior, but they **must not** rely on it.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_OUT_OF_RANGE**  *wr_num_bytes* or *wr_num_handles* are larger than the
largest allowable size for channel messages.
If the **ZX_CHANNEL_WRITE_USE_IOVEC** option is specified,
**ZX_ERR_OUT_OF_RANGE** will be produced if *num_bytes* is larger than
**ZX_CHANNEL_MAX_MSG_IOVEC** or the sum of the iovec capacities exceeds
**ZX_CHANNEL_MAX_MSG_BYTES**.

**ZX_ERR_BUFFER_TOO_SMALL**  *rd_num_bytes* or *rd_num_handles* are too small
to contain the reply message.

**ZX_ERR_NOT_SUPPORTED**  one of the handles in *handles* was *handle*
(the handle to the channel being written to).

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_channel_call()`]: channel_call.md
[`zx_channel_create()`]: channel_create.md
[`zx_channel_read()`]: channel_read.md
[`zx_channel_read_etc()`]: channel_read_etc.md
[`zx_channel_write()`]: channel_write.md
[`zx_channel_write_etc()`]: channel_write_etc.md
[`zx_object_wait_one()`]: object_wait_one.md
