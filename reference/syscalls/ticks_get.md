<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_ticks_get

## Summary

Read the number of high-precision timer ticks since boot.

## Declaration

```c
#include <zircon/syscalls.h>

zx_ticks_t zx_ticks_get(void);
```

## Description

`zx_ticks_get()` returns the number of high-precision timer ticks since boot.

These ticks may be processor cycles, high speed timer, profiling timer, etc.
They are not guaranteed to continue advancing when the system is asleep.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_ticks_get()` returns the number of high-precision timer ticks since boot.

## Errors

`zx_ticks_get()` does not report any error conditions.

## Notes

The returned value may be highly variable. Factors that can affect it include:

- Changes in processor frequency
- Migration between processors
- Reset of the processor cycle counter
- Reordering of instructions (if required, use a memory barrier)

## See also

[ticks_per_second](ticks_per_second.md)
