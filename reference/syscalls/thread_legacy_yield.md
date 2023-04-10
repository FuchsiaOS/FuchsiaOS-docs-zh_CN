<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_thread_legacy_yield

## Summary

Yield the CPU of the current thread back to the scheduler.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_thread_legacy_yield(uint32_t options);
```

## Description

`zx_thread_legacy_yield()` causes the calling thread to yield the CPU back to the scheduler.

 Yield may result in other threads with similar importance running ahead of the current thread,
 however, the exact behavior is unspecified.

 `options` must be zero.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_thread_legacy_yield(uint32_t options)` will always return `ZX_OK` for zero `options`.
Any other `options` value, will result in `ZX_ERR_INVALID_ARGS`.

## See also

 - [`zx_nanosleep()`]

[`zx_nanosleep()`]: nanosleep.md
