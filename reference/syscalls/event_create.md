<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_event_create

## Summary

Create an event.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_event_create(uint32_t options, zx_handle_t* out);
```

## Description

`zx_event_create()` creates an event, which is an object that is signalable. That
is, its **ZX_USER_SIGNAL_n** (where *n* is 0 through 7) signals can be
manipulated using [`zx_object_signal()`].

The newly-created handle will have the [basic
rights](/concepts/kernel/rights.md#zx_rights_basic) plus **ZX_RIGHT_SIGNAL**.

## Rights

Caller job policy must allow **ZX_POL_NEW_EVENT**.

## Return value

`zx_event_create()` returns **ZX_OK** and a valid event handle (via *out*) on success.
On failure, an error value is returned.

## Errors

**ZX_ERR_INVALID_ARGS**  *out* is an invalid pointer, or *options* is nonzero.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## See also

 - [`zx_eventpair_create()`]
 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_handle_replace()`]
 - [`zx_object_signal()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]

[`zx_eventpair_create()`]: eventpair_create.md
[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_object_signal()`]: object_signal.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
