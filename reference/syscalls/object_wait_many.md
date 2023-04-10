<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_object_wait_many

## Summary

Wait for signals on multiple objects.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_object_wait_many(zx_wait_item_t* items,
                                size_t num_items,
                                zx_time_t deadline);
```

## Description

`zx_object_wait_many()` is a blocking syscall that causes the caller to wait
until either the *deadline* passes or at least one object referred to in
*items* has a specified [signal][signals] asserted. If an object is already
asserting at least one of the specified signals, the wait ends immediately with
**ZX_OK**.

```
typedef struct {
    zx_handle_t handle;
    zx_signals_t waitfor;
    zx_signals_t pending;
} zx_wait_item_t;
```

The caller must provide *count* `zx_wait_item_t`s in the *items* array,
containing the handle and signals bitmask to wait for for each item.
Each item should contain a valid *handle* referring to an object to
wait for, and a bitmask *waitfor* indicating which signals should wake
the calling thread.

The *deadline* parameter specifies a deadline with respect to
**ZX_CLOCK_MONOTONIC** and will be automatically adjusted according to the job's
[timer slack] policy.  **ZX_TIME_INFINITE** is a special value meaning wait
forever.

Upon return, the *pending* field of *items* is filled with bitmaps indicating
which signals are pending for each item.

The maximum number of items that may be waited upon is **ZX_WAIT_MANY_MAX_ITEMS**,
which is 64.  To wait on more objects at once use [Ports](/docs/reference/kernel_objects/port.md).

## Rights

Every entry of *items* must have a *handle* field with **ZX_RIGHT_WAIT**.

## Return value

`zx_object_wait_many()` returns **ZX_OK** if any of *waitfor* signals were
active when the call was made, or observed on their respective object before
*deadline* passed.

In the event of **ZX_ERR_TIMED_OUT**, *items* may reflect state changes
that occurred after the deadline passed, but before the syscall returned.

In the event of **ZX_ERR_CANCELED**, one or more of the items being waited
upon have had their handles closed, and the *pending* field for those items
will have the **ZX_SIGNAL_HANDLE_CLOSED** bit set.

For any other return value, the *pending* fields of *items* are undefined.

## Errors

**ZX_ERR_INVALID_ARGS**  *items* isn't a valid pointer.

**ZX_ERR_OUT_OF_RANGE**  *count* is greater than **ZX_WAIT_MANY_MAX_ITEMS**.

**ZX_ERR_BAD_HANDLE**  one of *items* contains an invalid handle.

**ZX_ERR_ACCESS_DENIED**  One or more of the provided *handles* does not
have **ZX_RIGHT_WAIT** and may not be waited upon.

**ZX_ERR_CANCELED**  One or more of the provided *handles* was invalidated
(e.g., closed) during the wait.

**ZX_ERR_TIMED_OUT**  The specified deadline passed before any of the specified
signals are observed on any of the specified handles. Note that calls with a
*count* of 0 will still wait until *deadline* has passed before returning.

**ZX_ERR_NOT_SUPPORTED**  One of the *items* contains a handle that cannot
be waited one (for example, a Port handle).

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## BUGS

*pending* more properly should be called *observed*.

## Notes

See [signals] for more information about signals and their terminology.

## See also

 - [signals]
 - [timer slack]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_one()`]

[signals]: /docs/concepts/kernel/signals.md
[timer slack]: /docs/concepts/kernel/timer_slack.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_one()`]: object_wait_one.md
