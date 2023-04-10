<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_system_get_physmem

## Summary

Get amount of physical memory on the system.

## Declaration

```c
#include <zircon/syscalls.h>

uint64_t zx_system_get_physmem(void);
```

## Description

`zx_system_get_physmem()` returns the total size of physical memory on
the machine, in bytes.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_system_get_physmem()` returns a number in bytes.

## Errors

`zx_system_get_physmem()` cannot fail.

## Notes

Currently the total size of physical memory cannot change during a run of
the system, only at boot time.  This might change in the future.

## See also

 - [`zx_system_get_num_cpus()`]

[`zx_system_get_num_cpus()`]: system_get_num_cpus.md
