<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_ticks_per_second

## Summary

Read the number of high-precision timer ticks in a second.

## Declaration

```c
#include <zircon/syscalls.h>

zx_ticks_t zx_ticks_per_second(void);
```

## Description

`zx_ticks_per_second()` returns the number of high-precision timer ticks in a
second.

This can be used together with [`zx_ticks_get()`] to calculate the amount of
time elapsed between two subsequent calls to [`zx_ticks_get()`].

This value can vary from boot to boot of a given system. Once booted,
this value is guaranteed not to change.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_ticks_per_second()` returns the number of high-precision timer ticks in a
second.

## Errors

`zx_ticks_per_second()` does not report any error conditions.

## EXAMPLES

```
zx_ticks_t ticks_per_second = zx_ticks_per_second();
zx_ticks_t ticks_start = zx_ticks_get();

// do some more work

zx_ticks_t ticks_end = zx_ticks_get();
double elapsed_seconds = (ticks_end - ticks_start) / (double)ticks_per_second;

```

## See also

 - [`zx_ticks_get()`]

[`zx_ticks_get()`]: ticks_get.md
