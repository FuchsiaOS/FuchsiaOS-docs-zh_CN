<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_channel_read

## Summary

Read a message from a channel.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_channel_read(zx_handle_t handle,
                            uint32_t options,
                            void* bytes,
                            zx_handle_t* handles,
                            uint32_t num_bytes,
                            uint32_t num_handles,
                            uint32_t* actual_bytes,
                            uint32_t* actual_handles);
```

## Description

`zx_channel_read()` attempts to read the first message from the channel
specified by *handle* into the provided *bytes* and/or *handles* buffers.

The parameters *num_bytes* and *num_handles* are used to specify the size of the
respective read buffers. *num_bytes* is a count of bytes, and
*num_handles* is a count of elements of type `zx_handle_t`.

The length of *bytes*, in bytes, is stored in the location pointed to by
*actual_bytes*.  The number of handles is stored in the location pointed to by
*actual_handles*.  Either *actual_bytes* or *actual_handles* may be NULL, in
which case they will be ignored.

Channel messages may contain both byte data and handle payloads and may
only be read in their entirety.  Partial reads are not possible.

The *bytes* buffer is written before the *handles* buffer. In the event of
overlap between these two buffers, the contents written to *handles*
will overwrite the portion of *bytes* it overlaps.

When communicating to an untrusted party over a channel, it is recommended that
the [`zx_channel_read_etc()`] form is used and each handle type
and rights are validated against the expected values.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_CHANNEL** and have **ZX_RIGHT_READ**.

## Return value

Returns **ZX_OK** on success. If non-NULL, the locations pointed to by
*actual_bytes* and *actual_handles* contain the exact number of bytes and count
of handles read.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a channel handle.

**ZX_ERR_INVALID_ARGS**  If any of *bytes*, *handles*, *actual_bytes*, or
*actual_handles* are non-NULL and an invalid pointer.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_READ**.

**ZX_ERR_SHOULD_WAIT**  The channel contained no messages to read and the other side of the
channel is open.

**ZX_ERR_PEER_CLOSED**  The channel contained no messages to read and the other side of the
channel is closed.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_BUFFER_TOO_SMALL**  The provided *bytes* or *handles* buffers
are too small (in which case, the minimum sizes necessary to receive
the message will be written to *actual_bytes* and *actual_handles*,
provided they are non-NULL). If *options* has **ZX_CHANNEL_READ_MAY_DISCARD**
set, then the message is discarded.

## See also

 - [`zx_channel_call()`]
 - [`zx_channel_create()`]
 - [`zx_channel_read_etc()`]
 - [`zx_channel_write()`]
 - [`zx_channel_write_etc()`]
 - [`zx_handle_close()`]
 - [`zx_handle_replace()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]

[`zx_channel_call()`]: channel_call.md
[`zx_channel_create()`]: channel_create.md
[`zx_channel_read_etc()`]: channel_read_etc.md
[`zx_channel_write()`]: channel_write.md
[`zx_channel_write_etc()`]: channel_write_etc.md
[`zx_handle_close()`]: handle_close.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
