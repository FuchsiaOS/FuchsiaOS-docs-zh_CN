<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_socket_write

## Summary

Write data to a socket.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_socket_write(zx_handle_t handle,
                            uint32_t options,
                            const void* buffer,
                            size_t buffer_size,
                            size_t* actual);
```

## Description

`zx_socket_write()` attempts to write *buffer_size* bytes to the socket
specified by *handle*. The pointer to *bytes* may be NULL if *buffer_size* is
zero.

If a NULL *actual* is passed in, it will be ignored.

A **ZX_SOCKET_STREAM** socket write can be short if the socket does not have
enough space for all of *buffer*. If a non-zero amount of data was written to
the socket, the amount written is returned via *actual* and the call succeeds.
Otherwise, if the socket was already full, the call returns
**ZX_ERR_SHOULD_WAIT** and the client should wait (e.g., with
[`zx_object_wait_one()`] or [`zx_object_wait_async()`]).

A **ZX_SOCKET_DATAGRAM** socket write is never short. If the socket has
insufficient space for *buffer*, it writes nothing and returns
**ZX_ERR_SHOULD_WAIT**. If the write succeeds, *buffer_size* is returned via
*actual*. Attempting to write a packet larger than the datagram socket's
capacity will fail with **ZX_ERR_OUT_OF_RANGE**.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_SOCKET** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_socket_write()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_BAD_STATE**  writing has been disabled for this socket endpoint via
[`zx_socket_set_disposition()`].

**ZX_ERR_WRONG_TYPE**  *handle* is not a socket handle.

**ZX_ERR_INVALID_ARGS**  *buffer* is an invalid pointer.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_WRITE**.

**ZX_ERR_SHOULD_WAIT**  The buffer underlying the socket is full.

**ZX_ERR_OUT_OF_RANGE**  The socket was created with **ZX_SOCKET_DATAGRAM** and
*buffer* is larger than the remaining space in the socket.

**ZX_ERR_PEER_CLOSED**  The other side of the socket is closed.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## See also

 - [`zx_socket_create()`]
 - [`zx_socket_read()`]
 - [`zx_socket_set_disposition()`]

[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_one()`]: object_wait_one.md
[`zx_socket_create()`]: socket_create.md
[`zx_socket_read()`]: socket_read.md
[`zx_socket_set_disposition()`]: socket_set_disposition.md
