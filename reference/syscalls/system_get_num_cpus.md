<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_system_get_num_cpus

## Summary

Get number of logical processors on the system.

## Declaration

```c
#include <zircon/syscalls.h>

uint32_t zx_system_get_num_cpus(void);
```

## Description

`zx_system_get_num_cpus()` returns the number of CPUs (logical processors)
that exist on the system currently running.  This number cannot change
during a run of the system, only at boot time.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_system_get_num_cpus()` returns the number of CPUs.

## Errors

`zx_system_get_num_cpus()` cannot fail.

## Notes

## See also

 - [`zx_system_get_physmem()`]

[`zx_system_get_physmem()`]: system_get_physmem.md
