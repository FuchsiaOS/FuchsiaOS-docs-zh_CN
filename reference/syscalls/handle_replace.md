<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_handle_replace

## Summary

Replace a handle.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_handle_replace(zx_handle_t handle,
                              zx_rights_t rights,
                              zx_handle_t* out);
```

## Description

`zx_handle_replace()` creates a replacement for *handle*, referring to
the same underlying object, with new access rights *rights*.

*handle* is always invalidated.

If *rights* is **ZX_RIGHT_SAME_RIGHTS**, the replacement handle will
have the same rights as the original handle. Otherwise, *rights* must be
a subset of original handle's rights.

## Rights

None.

## Return value

`zx_handle_replace()` returns **ZX_OK** and the replacement handle (via *out*)
on success.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* isn't a valid handle.

**ZX_ERR_INVALID_ARGS**  The *rights* requested are not a subset of
*handle*'s rights or *out* is an invalid pointer.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## See also

 - [`zx_handle_close()`]
 - [`zx_handle_close_many()`]
 - [`zx_handle_duplicate()`]

[`zx_handle_close()`]: handle_close.md
[`zx_handle_close_many()`]: handle_close_many.md
[`zx_handle_duplicate()`]: handle_duplicate.md
