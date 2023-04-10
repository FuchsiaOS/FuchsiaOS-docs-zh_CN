<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_channel_write

## Summary

Write a message to a channel.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_channel_write(zx_handle_t handle,
                             uint32_t options,
                             const void* bytes,
                             uint32_t num_bytes,
                             const zx_handle_t* handles,
                             uint32_t num_handles);
```

## Description

`zx_channel_write()` attempts to write a message of *num_bytes*
bytes and *num_handles* handles to the channel specified by
*handle*.  The pointers *handles* and *bytes* may be NULL if their
respective sizes are zero.

On success, all *num_handles* of the handles in the *handles* array
are attached to the message and will become available to the reader
of that message from the opposite end of the channel.

All handles are discarded and no longer available to the caller, on
success or failure. Use [`zx_channel_write_etc()`] if handles need
to be preserved by the sender.

It is invalid to include *handle* (the handle of the channel being written
to) in the *handles* array (the handles being sent in the message).

The maximum number of handles that may be sent in a message is
**ZX_CHANNEL_MAX_MSG_HANDLES**, which is 64.

The maximum number of bytes that may be sent in a message is
**ZX_CHANNEL_MAX_MSG_BYTES**, which is 65536.

Messages are drained by [`zx_channel_read()`] or [`zx_channel_read_etc()`].
Failure to drain the messages in a timely fashion can cause excessive kernel
memory to be used, which might generate an exception. See
[ipc limits](/docs/concepts/kernel/ipc_limits.md) for details.

### ZX_CHANNEL_WRITE_USE_IOVEC option

When the **ZX_CHANNEL_WRITE_USE_IOVEC** option is specified, `bytes` is
interpreted as an array of `zx_channel_iovec_t`, specifying slices of bytes to
sequentially copy to the message in order. `num_bytes` specifies the number of
`zx_channel_iovec_t` array elements in `bytes`.

```c
typedef struct zx_channel_iovec {
  const void* buffer;      // User-space bytes.
  uint32_t capacity;       // Number of bytes.
  uint32_t reserved;       // Reserved.
} zx_channel_iovec_t;
```

There can be at most **ZX_CHANNEL_MAX_MSG_IOVEC** or `8192`
`zx_channel_iovec_t` elements of the `bytes` array with the sum of `capacity`
across all `zx_channel_iovec_t` not exceeding **ZX_CHANNEL_MAX_MSG_BYTES** or
`65536` bytes. `buffer` need not be aligned and it may only be `NULL` if
`capacity` is zero. `reserved` must be set to zero.

Either all `zx_channel_iovec_t` are copied and the message is sent, or none
are copied and the message is not sent. Usage for sending handles is unchanged.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_CHANNEL** and have **ZX_RIGHT_WRITE**.

Every entry of *handles* must have **ZX_RIGHT_TRANSFER**.

## Return value

`zx_channel_write()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_BAD_HANDLE** `handle` is not a valid handle, any element in
`handles` is not a valid handle, or there are repeated handles among the
handles in the `handles` array.

**ZX_ERR_WRONG_TYPE** `handle` is not a channel handle.

**ZX_ERR_INVALID_ARGS** `bytes` is an invalid pointer, `handles`
is an invalid pointer, or `options` contains an invalid option bit.
If the **ZX_CHANNEL_WRITE_USE_IOVEC** option is specified,
**ZX_ERR_INVALID_ARGS** will be produced if the `buffer` field contains an
invalid pointer or if the reserved field is non-zero.

**ZX_ERR_NOT_SUPPORTED** `handle` was found in the `handles` array.
A handle to the channel performing the write cannot be included in the
`handles` array. In other words a channel handle cannot be written to its own channel.
Fix the error by making sure that `handle` is not in the `handles` array.

**ZX_ERR_ACCESS_DENIED** `handle` does not have **ZX_RIGHT_WRITE** or
any element in `handles` does not have **ZX_RIGHT_TRANSFER**.

**ZX_ERR_PEER_CLOSED**  The other side of the channel is closed.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_OUT_OF_RANGE**  `num_bytes` or `num_handles` are larger than
**ZX_CHANNEL_MAX_MSG_BYTES** or **ZX_CHANNEL_MAX_MSG_HANDLES** respectively.
If the **ZX_CHANNEL_WRITE_USE_IOVEC** option is specified,
**ZX_ERR_OUT_OF_RANGE** will be produced if `num_bytes` is larger than
**ZX_CHANNEL_MAX_MSG_IOVEC** or the sum of the iovec capacities exceeds
**ZX_CHANNEL_MAX_MSG_BYTES**.

## Notes

*num_handles* is a count of the number of elements in the *handles*
array, not its size in bytes.

The byte size limitation on messages is not yet finalized.

## See also

 - [`zx_channel_call()`]
 - [`zx_channel_create()`]
 - [`zx_channel_read()`]
 - [`zx_channel_read_etc()`]
 - [`zx_channel_write_etc()`]
 - [`zx_handle_close()`]
 - [`zx_handle_replace()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]

[`zx_channel_call()`]: channel_call.md
[`zx_channel_create()`]: channel_create.md
[`zx_channel_read()`]: channel_read.md
[`zx_channel_read_etc()`]: channel_read_etc.md
[`zx_channel_write_etc()`]: channel_write_etc.md
[`zx_handle_close()`]: handle_close.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
