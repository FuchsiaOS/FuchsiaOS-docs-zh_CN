# zx_channel_call

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Send a message to a channel and await a reply.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_channel_call(zx_handle_t handle,
                            uint32_t options,
                            zx_time_t deadline,
                            const zx_channel_call_args_t* args,
                            uint32_t* actual_bytes,
                            uint32_t* actual_handles);
```

## DESCRIPTION

`zx_channel_call()` is like a combined [`zx_channel_write()`], [`zx_object_wait_one()`],
and [`zx_channel_read()`], with the addition of a feature where a transaction id at
the front of the message payload *bytes* is used to match reply messages with send
messages, enabling multiple calling threads to share a channel without any additional
userspace bookkeeping.

The write and read phases of this operation behave like [`zx_channel_write()`] and
[`zx_channel_read()`] with the difference that their parameters are provided via the
`zx_channel_call_args_t` structure.

The first four bytes of the written and read back messages are treated as a
transaction ID of type `zx_txid_t`.  The kernel generates a txid for the
written message, replacing that part of the message as read from userspace.
The kernel generated txid will be between 0x80000000 and 0xFFFFFFFF, and will
not collide with any txid from any other `zx_channel_call()` in progress against
this channel endpoint.  If the written message has a length of fewer than four
bytes, an error is reported.

When the outbound message is written, simultaneously an interest is registered
for inbound messages of the matching txid.

*deadline* may be automatically adjusted according to the job's [timer slack]
policy.

While the slack-adjusted *deadline* has not passed, if an inbound message
arrives with a matching txid, instead of being added to the tail of the general
inbound message queue, it is delivered directly to the thread waiting in
`zx_channel_call()`.

If such a reply arrives after the slack-adjusted *deadline* has passed, it will
arrive in the general inbound message queue, cause **ZX_CHANNEL_READABLE** to be
signaled, etc.

Inbound messages that are too large to fit in *rd_num_bytes* and *rd_num_handles*
are discarded and **ZX_ERR_BUFFER_TOO_SMALL** is returned in that case.

As with [`zx_channel_write()`], the handles in *handles* are always consumed by
`zx_channel_call()` and no longer exist in the calling process.

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

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_CHANNEL** and have **ZX_RIGHT_READ** and have **ZX_RIGHT_WRITE**.

All wr_handles of *args* must have **ZX_RIGHT_TRANSFER**.

## RETURN VALUE

`zx_channel_call()` returns **ZX_OK** on success and the number of bytes and
count of handles in the reply message are returned via *actual_bytes* and
*actual_handles*, respectively.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle, any element in
*handles* is not a valid handle, or there are duplicates among the handles
in the *handles* array.

**ZX_ERR_WRONG_TYPE**  *handle* is not a channel handle.

**ZX_ERR_INVALID_ARGS**  any of the provided pointers are invalid or null,
or *wr_num_bytes* is less than four, or *options* is nonzero.
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

## NOTES

The facilities provided by `zx_channel_call()` can interoperate with message dispatchers
using [`zx_channel_read()`] and [`zx_channel_write()`] directly, provided the following rules
are observed:

1. A server receiving synchronous messages via [`zx_channel_read()`] should ensure that the
txid of incoming messages is reflected back in outgoing responses via [`zx_channel_write()`]
so that clients using `zx_channel_call()` can correctly route the replies.

2. A client sending messages via [`zx_channel_write()`] that will be replied to should ensure
that it uses txids between 0 and 0x7FFFFFFF only, to avoid colliding with other threads
communicating via `zx_channel_call()`.

If a `zx_channel_call()` returns due to **ZX_ERR_TIMED_OUT**, if the server eventually replies,
at some point in the future, the reply *could* match another outbound request (provided about
2^31 `zx_channel_call()`s have happened since the original request.  This syscall is designed
around the expectation that timeouts are generally fatal and clients do not expect to continue
communications on a channel that is timing out.

## SEE ALSO

 - [timer slack]
 - [`zx_channel_create()`]
 - [`zx_channel_read()`]
 - [`zx_channel_write()`]
 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_handle_replace()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[timer slack]: /docs/concepts/kernel/timer_slack.md
[`zx_channel_create()`]: channel_create.md
[`zx_channel_read()`]: channel_read.md
[`zx_channel_write()`]: channel_write.md
[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
