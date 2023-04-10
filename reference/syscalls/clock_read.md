<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_clock_read

## Summary

Perform a basic read of the clock.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_clock_read(zx_handle_t handle, zx_time_t* now);
```

## Rights

*handle* must be of type **ZX_OBJ_TYPE_CLOCK** and have **ZX_RIGHT_READ**.

## Description

Perform a basic read of the clock object and return its current time in the
*now* out parameter.

## Return value

On success, returns **ZX_OK** along with the clock's current time in the *now* output parameter.

## Errors

 - **ZX_ERR_BAD_HANDLE** : *handle* is either an invalid handle, or a handle to
   an object type that is not **ZX_OBJ_TYPE_CLOCK**.
 - **ZX_ERR_ACCESS_DENIED** : *handle* lacks the **ZX_RIGHT_READ** right.

## See also

 - [clocks]
 - [`zx_clock_create()`]
 - [`zx_clock_get_details()`]
 - [`zx_clock_update()`]

[clocks]: /docs/reference/kernel_objects/clock.md
[`zx_clock_create()`]: clock_create.md
[`zx_clock_get_details()`]: clock_get_details.md
[`zx_clock_update()`]: clock_update.md
