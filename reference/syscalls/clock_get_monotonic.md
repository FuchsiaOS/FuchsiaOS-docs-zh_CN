<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_clock_get_monotonic

## Summary

Acquire the current monotonic time.

## Declaration

```c
#include <zircon/syscalls.h>

zx_time_t zx_clock_get_monotonic(void);
```

## Description

`zx_clock_get_monotonic()` returns the current time in the system
monotonic clock. This is the number of nanoseconds since the system was
powered on. It does not always reset on reboot and does not adjust during
sleep, and thus should not be used as a reliable source of uptime.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_clock_get_monotonic()` returns the current monotonic time.

## Errors

`zx_clock_get_monotonic()` cannot fail.
