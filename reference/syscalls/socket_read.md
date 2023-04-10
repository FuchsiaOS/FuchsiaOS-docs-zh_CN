<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_socket_read

## Summary

Read data from a socket.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_socket_read(zx_handle_t handle,
                           uint32_t options,
                           void* buffer,
                           size_t buffer_size,
                           size_t* actual);
```

## Description

`zx_socket_read()` attempts to read *buffer_size* bytes into *buffer*. If
successful, the number of bytes actually read are return via
*actual*.

If a NULL *actual* is passed in, it will be ignored.

If the socket was created with **ZX_SOCKET_DATAGRAM**, this syscall reads
only the first available datagram in the socket (if one is present).
If *buffer* is too small for the datagram, then the read will be
truncated, and any remaining bytes in the datagram will be discarded.

Supported *options* are:

* **ZX_SOCKET_PEEK** to leave the message in the socket.

To determine how many bytes are available to read, use the **rx_buf_available**
field of the resulting `zx_info_socket_t`, which you can obtain using the
**ZX_INFO_SOCKET** topic for [`zx_object_get_info()`].

## Rights

*handle* must be of type **ZX_OBJ_TYPE_SOCKET** and have **ZX_RIGHT_READ**.

## Return value

`zx_socket_read()` returns **ZX_OK** on success, and writes into
*actual* (if non-NULL) the exact number of bytes read.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_BAD_STATE**  writing to *handle*'s peer has been disabled via
[`zx_socket_set_disposition()`] and no pending data remains on *handle*.

**ZX_ERR_WRONG_TYPE**  *handle* is not a socket handle.

**ZX_ERR_INVALID_ARGS** If any of *buffer* or *actual* are non-NULL
but invalid pointers, or if *buffer* is NULL, or if *options* is zero.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_READ**.

**ZX_ERR_SHOULD_WAIT**  The socket contained no data to read.

**ZX_ERR_PEER_CLOSED**  The other side of the socket is closed and no data is
readable.

## See also

 - [`zx_socket_create()`]
 - [`zx_socket_set_disposition()`]
 - [`zx_socket_write()`]

[`zx_object_get_info()`]: object_get_info.md
[`zx_socket_create()`]: socket_create.md
[`zx_socket_set_disposition()`]: socket_set_disposition.md
[`zx_socket_write()`]: socket_write.md
