<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_port_cancel

## Summary

Cancels async port notifications on an object.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_port_cancel(zx_handle_t handle,
                           zx_handle_t source,
                           uint64_t key);
```

## Description

`zx_port_cancel()` is a non-blocking syscall that cancels
all pending [`zx_object_wait_async()`] operations made with *source* and *key*.

When this call succeeds no new packets from the object pointed by
*source* with *key* will be delivered to *handle*, and pending queued
packets that match *source* and *key* are removed from the port.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_PORT** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_port_cancel()` returns **ZX_OK** if cancellation succeeded and
either queued packets were removed or pending [`zx_object_wait_async()`] were
canceled.

## Errors

**ZX_ERR_BAD_HANDLE**  *source* or *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a port handle.

**ZX_ERR_ACCESS_DENIED**  *source* or *handle* does not have **ZX_RIGHT_WRITE**.

**ZX_ERR_NOT_SUPPORTED**  *source* is a handle that cannot be waited on.

**ZX_ERR_NOT_FOUND** if either no pending packets or pending
[`zx_object_wait_async()`] calls with *source* and *key* were found.

## See also

 - [`zx_port_wait()`]

[`zx_object_wait_async()`]: object_wait_async.md
[`zx_port_wait()`]: port_wait.md
