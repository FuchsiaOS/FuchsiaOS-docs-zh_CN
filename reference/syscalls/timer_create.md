<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_timer_create

## Summary

Create a timer.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_timer_create(uint32_t options,
                            zx_clock_t clock_id,
                            zx_handle_t* out);
```

## Description

`zx_timer_create()` creates a timer, an object that can signal
when a specified point in time has been reached. The only valid
*clock_id* is **ZX_CLOCK_MONOTONIC**.

The *options* value specifies the coalescing behavior, which
controls whether the system can fire the time earlier or later
depending on other pending timers.

The possible values are:

+ **ZX_TIMER_SLACK_CENTER**
+ **ZX_TIMER_SLACK_EARLY**
+ **ZX_TIMER_SLACK_LATE**

Passing 0 in options is equivalent to **ZX_TIMER_SLACK_CENTER**.

See [timer slack](/concepts/kernel/timer_slack.md) for more information.

The returned handle has the **ZX_RIGHT_DUPLICATE**, **ZX_RIGHT_TRANSFER**,
**ZX_RIGHT_WRITE**, **ZX_RIGHT_SIGNAL**, **ZX_RIGHT_WAIT**, and
**ZX_RIGHT_INSPECT** rights.

## Rights

Caller job policy must allow **ZX_POL_NEW_TIMER**.

## Return value

`zx_timer_create()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## Errors

**ZX_ERR_INVALID_ARGS**  *out* is an invalid pointer or NULL or
*options* is not one of the **ZX_TIMER_SLACK** values or *clock_id* is
any value other than **ZX_CLOCK_MONOTONIC**.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## See also

 - [`zx_deadline_after()`]
 - [`zx_handle_close()`]
 - [`zx_timer_cancel()`]
 - [`zx_timer_set()`]

[`zx_deadline_after()`]: deadline_after.md
[`zx_handle_close()`]: handle_close.md
[`zx_timer_cancel()`]: timer_cancel.md
[`zx_timer_set()`]: timer_set.md
