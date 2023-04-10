<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_deadline_after

## Summary

Convert a time relative to now to an absolute deadline.

## Declaration

```c
#include <zircon/syscalls.h>

zx_time_t zx_deadline_after(zx_duration_t nanoseconds);
```

## Description

`zx_deadline_after()` is a utility for converting from now-relative durations
to absolute deadlines. If *nanoseconds* plus the current time is bigger than the
maximum value for `zx_time_t`, the output is clamped to **ZX_TIME_INFINITE**.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_deadline_after()` returns the absolute time (with respect to **ZX_CLOCK_MONOTONIC**)
that is *nanoseconds* nanoseconds from now.

## Errors

`zx_deadline_after()` does not report any error conditions.

## EXAMPLES

```
// Sleep 50 milliseconds
zx_time_t deadline = zx_deadline_after(ZX_MSEC(50));
zx_nanosleep(deadline);
```

## See also

[ticks_get](ticks_get.md)
