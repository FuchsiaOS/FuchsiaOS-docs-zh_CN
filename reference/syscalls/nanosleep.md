<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_nanosleep

## Summary

High resolution sleep.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_nanosleep(zx_time_t deadline);
```

## Description

`zx_nanosleep()` suspends the calling thread execution until *deadline* passes
on **ZX_CLOCK_MONOTONIC**. *deadline* will be automatically adjusted according to the job's
[timer slack] policy.

To sleep for a duration, use [`zx_deadline_after()`] and the
**ZX_\<time-unit\>** helpers:

```
#include <zircon/syscalls.h> // zx_deadline_after, zx_nanosleep
#include <zircon/types.h> // ZX_MSEC et al.

// Sleep 50 milliseconds
zx_nanosleep(zx_deadline_after(ZX_MSEC(50)));
```

## Rights

None.

## Return value

`zx_nanosleep()` always returns **ZX_OK**.

## See also

 - [timer slack]
 - [`zx_deadline_after()`]
 - [`zx_timer_cancel()`]
 - [`zx_timer_create()`]
 - [`zx_timer_set()`]

[timer slack]: /docs/concepts/kernel/timer_slack.md
[`zx_deadline_after()`]: deadline_after.md
[`zx_timer_cancel()`]: timer_cancel.md
[`zx_timer_create()`]: timer_create.md
[`zx_timer_set()`]: timer_set.md
