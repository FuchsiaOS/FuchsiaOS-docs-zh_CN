<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_port_create

## Summary

Create an IO port.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_port_create(uint32_t options, zx_handle_t* out);
```

## Description

`zx_port_create()` creates a port: a waitable object that can be used to read
packets queued by kernel or by user-mode.

If you need this port to be bound to an interrupt, pass **ZX_PORT_BIND_TO_INTERRUPT** to *options*,
otherwise it should be **0**.

In the case where a port is bound to an interrupt, the interrupt packets are delivered via a
dedicated queue on ports and are higher priority than other non-interrupt packets.

The returned handle will have:

  * `ZX_RIGHT_TRANSFER`: allowing them to be sent to another process through [`zx_channel_write()`].
  * `ZX_RIGHT_WRITE`: allowing packets to be *queued*.
  * `ZX_RIGHT_READ`: allowing packets to be *read*.
  * `ZX_RIGHT_DUPLICATE`: allowing them to be *duplicated*.

## Rights

Caller job policy must allow **ZX_POL_NEW_PORT**.

## Return value

`zx_port_create()` returns **ZX_OK** and a valid IO port handle via *out* on
success. In the event of failure, an error value is returned.

## Errors

**ZX_ERR_INVALID_ARGS** *options* has an invalid value, or *out* is an
invalid pointer or NULL.

**ZX_ERR_NO_MEMORY** Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future builds this error will no longer occur.

## See also

 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_handle_replace()`]
 - [`zx_object_wait_async()`]
 - [`zx_port_queue()`]
 - [`zx_port_wait()`]

[`zx_channel_write()`]: channel_write.md
[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_port_queue()`]: port_queue.md
[`zx_port_wait()`]: port_wait.md
