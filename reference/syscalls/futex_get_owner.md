<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_futex_get_owner

## Summary

Fetch the koid current owner of a futex, if any.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_futex_get_owner(const zx_futex_t* value_ptr, zx_koid_t* koid);
```

## Description

Fetch the koid of the current owner of the futex identified by *value_ptr*, or
**ZX_KOID_INVALID** if there is no current owner.  Knowledge of the ownership of
a futex typically serves no purpose when building synchronization primitives
from futexes.  This syscall is used primarily for testing.

See *Ownership and Priority Inheritance* in [futex](/docs/reference/kernel_objects/futex.md) for
details.

## Rights

None.

## Return value

`zx_futex_get_owner()` returns **ZX_OK** on success, and koids hold the owner of
the futex at the time of the syscall, or **ZX_KOID_INVALID** if there was no
owner.

## Errors

**ZX_ERR_INVALID_ARGS**  One of the following is true:

+ *value_ptr* is not a valid userspace pointer.
+ *value_ptr* is not aligned to a `sizeof(zx_futex_t)` boundary.
+ *koid* is not a valid userspace pointer.

## See also

[futex objects](/docs/reference/kernel_objects/futex.md)
