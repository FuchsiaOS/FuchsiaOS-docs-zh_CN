<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_handle_close

## Summary

Close a handle.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_handle_close(zx_handle_t handle);
```

## Description

`zx_handle_close()` closes a *handle*, causing the underlying object to be
reclaimed by the kernel if no other handles to it exist.

If the *handle* was used in a pending [`zx_object_wait_one()`] or a
[`zx_object_wait_many()`] call, the wait will be aborted.

It is not an error to close the special "never a valid handle" **ZX_HANDLE_INVALID**,
similar to `free(NULL)` being a valid call.

Closing the last handle to a peered object using `zx_handle_close()` can affect
the state of the object's peer (if any).  See also
[peered-objects][peered-objects].

## Rights

None.

## Return value

`zx_handle_close()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* isn't a valid handle.

## See also

<!-- Reference links -->
[peered-objects]: /docs/reference/kernel_objects/objects.md#peered-objects-and-the-peer-closed-state

 - [`zx_handle_close_many()`]
 - [`zx_handle_duplicate()`]
 - [`zx_handle_replace()`]

[`zx_handle_close_many()`]: handle_close_many.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
