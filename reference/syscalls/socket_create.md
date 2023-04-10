<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_socket_create

## Summary

Create a socket.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_socket_create(uint32_t options,
                             zx_handle_t* out0,
                             zx_handle_t* out1);
```

## Description

`zx_socket_create()` creates a socket, a connected pair of
bidirectional stream transports, that can move only data, and that
have a maximum capacity.

Data written to one handle may be read from the opposite.

The *options* must set either the **ZX_SOCKET_STREAM** or
**ZX_SOCKET_DATAGRAM** flag.

## Rights

Caller job policy must allow **ZX_POL_NEW_SOCKET**.

## Return value

`zx_socket_create()` returns **ZX_OK** on success. In the event of
failure, one of the following values is returned.

## Errors

**ZX_ERR_INVALID_ARGS**  *out0* or *out1* is an invalid pointer or NULL or
*options* is any value other than **ZX_SOCKET_STREAM** or **ZX_SOCKET_DATAGRAM**.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## LIMITATIONS

The maximum capacity is not currently set-able.

## See also

 - [`zx_socket_read()`]
 - [`zx_socket_set_disposition()`]
 - [`zx_socket_write()`]

[`zx_socket_read()`]: socket_read.md
[`zx_socket_set_disposition()`]: socket_set_disposition.md
[`zx_socket_write()`]: socket_write.md
