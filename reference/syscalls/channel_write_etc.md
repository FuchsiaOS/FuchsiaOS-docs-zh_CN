<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_channel_write_etc

## Summary

Write a message to a channel.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_channel_write_etc(zx_handle_t handle,
                                 uint32_t options,
                                 const void* bytes,
                                 uint32_t num_bytes,
                                 zx_handle_disposition_t* handles,
                                 uint32_t num_handles);
```

## Description

Like [`zx_channel_write()`] it attempts to write a message of *num_bytes*
bytes and *num_handles* handles to the channel specified by *handle*, but in
addition it will perform operations for the handles that are being
transferred with *handles* being an array of `zx_handle_disposition_t`:

```
typedef struct zx_handle_disposition {
    zx_handle_op_t operation;
    zx_handle_t handle;
    zx_obj_type_t type;
    zx_rights_t rights;
    zx_status_t result;
} zx_handle_disposition_t;
```
In zx_handle_disposition_t, *handle* is the source handle to be operated on,
*rights* is the desired final rights (not a mask) and *result* must be set
to **ZX_OK**. All source handles must have **ZX_RIGHT_TRANSFER**, but
it can be removed in *rights* so that it is not available to the message
receiver.

*type* is used to perform validation of the object type that the caller
expects *handle* to be. It can be *ZX_OBJ_TYPE_NONE* to skip validation
checks or one of `zx_obj_type_t` defined types.

The operation applied to *handle* is one of:

*   **ZX_HANDLE_OP_MOVE** This is equivalent to first issuing [`zx_handle_replace()`] then
     [`zx_channel_write()`]. The source handle is always closed.

*   **ZX_HANDLE_OP_DUPLICATE** This is equivalent to first issuing [`zx_handle_duplicate()`]
    then [`zx_channel_write()`]. The source handle always remains open and accessible to the
    caller.

*handle* will be transferred with capability *rights* which can be **ZX_RIGHT_SAME_RIGHTS**
or a reduced set of rights, or **ZX_RIGHT_NONE**. In addition, this operation allows removing
**ZX_RIGHT_TRANSFER** in *rights* so that capability is not available for the receiver.

If any operation fails, the error code for that source handle is written to *result*, and the
first failure is made available in the return value for `zx_channel_write_etc()`. All
operations in the *handles* array are attempted, even if one or more operations fail.

All operations for each entry must succeed for the message to be written. On success, handles
are attached to the message and will become available to the reader of that message from the
opposite end of the channel.

It is invalid to include *handle* (the handle of the channel being written to) in the
*handles* array (the handles being sent in the message).

The maximum number of handles that may be sent in a message is **ZX_CHANNEL_MAX_MSG_HANDLES**,
which is 64.

The maximum number of bytes that may be sent in a message is **ZX_CHANNEL_MAX_MSG_BYTES**,
which is 65536.

Messages are drained by [`zx_channel_read()`] or [`zx_channel_read_etc()`]. Failure to drain the
messages in a timely fashion can cause excessive kernel memory to be used, which might generate an
exception. See [ipc limits](/concepts/kernel/ipc_limits.md) for details.

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

`zx_channel_write_etc()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle, any source handle in
*handles* is not a valid handle, or there are repeated handles
in the *handles* array if **ZX_HANDLE_OP_DUPLICATE** flags is not present.

**ZX_ERR_WRONG_TYPE**  *handle* is not a channel handle, or any source handle
in *handles* did not match the object type *type*.

**ZX_ERR_INVALID_ARGS**  *bytes* is an invalid pointer, *handles*
is an invalid pointer, or *options* is nonzero, or *operation* is not
one of ZX_HANDLE_OP_MOVE or ZX_HANDLE_OP_DUPLICATE, or any source
handle in *handles\[i\]->handle* did not have the rights specified in
*whandle\[i\]->rights*.
If the **ZX_CHANNEL_WRITE_USE_IOVEC** option is specified,
**ZX_ERR_INVALID_ARGS** will be produced if the *buffer* field contains an
invalid pointer or if the reserved field is non-zero.

**ZX_ERR_NOT_SUPPORTED**  *handle* is included in the *handles* array.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_WRITE** or
any source handle in *handles* does not have **ZX_RIGHT_TRANSFER**, or
any source handle in *handles* does not have **ZX_RIGHT_DUPLICATE** when
**ZX_HANDLE_OP_DUPLICATE** operation is specified.

**ZX_ERR_PEER_CLOSED**  The other side of the channel is closed.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_OUT_OF_RANGE**  *num_bytes* or *num_handles* are larger than
**ZX_CHANNEL_MAX_MSG_BYTES** or **ZX_CHANNEL_MAX_MSG_HANDLES** respectively.
If the **ZX_CHANNEL_WRITE_USE_IOVEC** option is specified,
**ZX_ERR_OUT_OF_RANGE** will be produced if *num_bytes* is larger than
**ZX_CHANNEL_MAX_MSG_IOVEC** or the sum of the iovec capacities exceeds
**ZX_CHANNEL_MAX_MSG_BYTES**.

## Notes

If the caller removes the **ZX_RIGHT_TRANSFER** to a handle attached
to a message, the reader of the message will receive a handle that cannot
be written to any other channel, but still can be using according to its
rights and can be closed if not needed.

## See also

 - [`zx_channel_call()`]
 - [`zx_channel_create()`]
 - [`zx_channel_read()`]
 - [`zx_channel_read_etc()`]
 - [`zx_channel_write()`]

[`zx_channel_call()`]: channel_call.md
[`zx_channel_create()`]: channel_create.md
[`zx_channel_read()`]: channel_read.md
[`zx_channel_read_etc()`]: channel_read_etc.md
[`zx_channel_write()`]: channel_write.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_handle_replace()`]: handle_replace.md
