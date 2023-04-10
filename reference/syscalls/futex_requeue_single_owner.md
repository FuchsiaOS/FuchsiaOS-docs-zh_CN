<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_futex_requeue_single_owner

## Summary

Wake one thread waiting on a futex, and requeue more waiters to another futex wait queue.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_futex_requeue_single_owner(const zx_futex_t* value_ptr,
                                          zx_futex_t current_value,
                                          const zx_futex_t* requeue_ptr,
                                          uint32_t requeue_count,
                                          zx_handle_t new_requeue_owner);
```

## Description

Wake one thread waiting on *value_ptr* and assign ownership of *value_ptr* to
the thread that was woken. If there are no threads waiting on *value_ptr* then
the ownership of *value_ptr* is set to none.

Then move up to *requeue_count* threads that are still waiting on *value_ptr* from
the *value_ptr* futex to the *requeue_ptr* futex.

`zx_futex_requeue_single_owner` is similar to `zx_futex_requeue` with a
*wake_count* of 1, except that `zx_futex_requeue_single_owner` changes the
ownership of *value_ptr* to the woken thread.
See [`zx_futex_requeue()`] for a full description.

## Rights

None.

## Return value

`zx_futex_requeue_single_owner()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_INVALID_ARGS**  One of the following is true:

+ Either *value_ptr* or *requeue_ptr* is not a valid userspace pointer
+ Either *value_ptr* or *requeue_ptr* is not aligned to a `sizeof(zx_futex_t)` boundary.
+ *value_ptr* is the same futex as *requeue_ptr*
+ *new_requeue_owner* is currently a member of the waiters for either *value_ptr* or *requeue_ptr*

**ZX_ERR_BAD_HANDLE**  *new_requeue_owner* is not **ZX_HANDLE_INVALID**, and not a valid handle.

**ZX_ERR_WRONG_TYPE**  *new_requeue_owner* is a valid handle, but is not a handle to a thread.

**ZX_ERR_BAD_STATE**  *current_value* does not match the value at *value_ptr*.

## See also

 - [futex objects]
 - [`zx_futex_requeue()`]
 - [`zx_futex_wait()`]
 - [`zx_futex_wake()`]

[futex objects]: /reference/kernel_objects/futex.md
[`zx_futex_requeue()`]: futex_requeue.md
[`zx_futex_wait()`]: futex_wait.md
[`zx_futex_wake()`]: futex_wake.md
