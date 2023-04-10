<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_timer_cancel

## Summary

Cancel a timer.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_timer_cancel(zx_handle_t handle);
```

## Description

`zx_timer_cancel()` cancels a pending timer that was started with
[`zx_timer_set()`].

Upon success the pending timer is canceled and the **ZX_TIMER_SIGNALED**
signal is de-asserted. If a new pending timer is immediately needed
rather than calling `zx_timer_cancel()` first, call [`zx_timer_set()`]
with the new deadline.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_TIMER** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_timer_cancel()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_ACCESS_DENIED**  *handle* lacks the right **ZX_RIGHT_WRITE**.

## NOTE

Calling this function before [`zx_timer_set()`] has no effect.

## See also

 - [`zx_timer_create()`]
 - [`zx_timer_set()`]

[`zx_timer_create()`]: timer_create.md
[`zx_timer_set()`]: timer_set.md
