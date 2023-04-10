<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_eventpair_create

## Summary

Create an event pair.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_eventpair_create(uint32_t options,
                                zx_handle_t* out0,
                                zx_handle_t* out1);
```

## Description

`zx_eventpair_create()` creates an event pair, which is a pair of objects that
are mutually signalable.

The signals **ZX_EVENTPAIR_SIGNALED** and **ZX_USER_SIGNAL_n** (where *n* is 0 through 7)
may be set or cleared using [`zx_object_signal()`], which modifies the signals on the
object itself, or [`zx_object_signal_peer()`], which modifies the signals on its
counterpart.

When all the handles to one of the objects have been closed, the
**ZX_EVENTPAIR_PEER_CLOSED** signal will be asserted on the opposing object.

The newly-created handles will have the **ZX_RIGHT_TRANSFER**,
**ZX_RIGHT_DUPLICATE**, **ZX_RIGHT_READ**, **ZX_RIGHT_WRITE**, **ZX_RIGHT_SIGNAL**,
and **ZX_RIGHT_SIGNAL_PEER** rights.

Currently, no options are supported, so *options* must be set to 0.

## Rights

Caller job policy must allow **ZX_POL_NEW_EVENTPAIR**.

## Return value

`zx_eventpair_create()` returns **ZX_OK** on success. On failure, a (negative)
error code is returned.

## Errors

**ZX_ERR_INVALID_ARGS**  *out0* or *out1* is an invalid pointer or NULL.

**ZX_ERR_NOT_SUPPORTED**  *options* has an unsupported flag set (i.e., is not 0).

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## See also

 - [`zx_event_create()`]
 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_handle_replace()`]
 - [`zx_object_signal()`]
 - [`zx_object_signal_peer()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]

[`zx_event_create()`]: event_create.md
[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_object_signal()`]: object_signal.md
[`zx_object_signal_peer()`]: object_signal_peer.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
