<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_futex_requeue

## Summary

Wake some number of threads waiting on a futex, and move more waiters to another wait queue.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_futex_requeue(const zx_futex_t* value_ptr,
                             uint32_t wake_count,
                             zx_futex_t current_value,
                             const zx_futex_t* requeue_ptr,
                             uint32_t requeue_count,
                             zx_handle_t new_requeue_owner);
```

## Description

Requeuing is a generalization of waking. First, the kernel verifies
that the value in *current_value* matches the value of the futex at
*value_ptr*, and if not reports **ZX_ERR_BAD_STATE**. After waking *wake_count*
threads, *requeue_count* threads are moved from the original futex's
wait queue to the wait queue corresponding to *requeue_ptr*, another
futex.

This requeueing behavior may be used to avoid thundering herds on wake.

## OWNERSHIP

A requeue operation targets two futexes, the _wake futex_ and the _requeue
futex_.  The ownership implications for each are discussed separately.
Generally, if the call fails for any reason, no changes to ownership for either
futex are made.

See *Ownership and Priority Inheritance* in [futex](/docs/reference/kernel_objects/futex.md) for
details.

### Effects on the _wake futex_ target

A successful call to `zx_futex_requeue()` results in the owner of the futex being
set to nothing, regardless of the wake count.  In order to transfer ownership of
a futex, use the [`zx_futex_requeue_single_owner()`] variant instead.
[`zx_futex_requeue_single_owner()`] will attempt to wake exactly one thread from the
futex wait queue.  If there is at least one thread to wake, the owner of the futex will be
set to the thread that was woken.  Otherwise, the futex
will have no owner.

### Effects on the _requeue futex_ target

A successful call to `zx_futex_requeue()` or [`zx_futex_requeue_single_owner()`]
results in the owner of the futex being set to the thread referenced by the
*new_requeue_owner* handle, or to nothing if *new_requeue_owner* is
**ZX_HANDLE_INVALID**.

## Rights

None.

## Return value

`zx_futex_requeue()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_INVALID_ARGS**  One of the following is true:

+ Either *value_ptr* or *requeue_ptr* is not a valid userspace pointer
+ Either *value_ptr* or *requeue_ptr* is not aligned to a `sizeof(zx_futex_t)` boundary.
+ *value_ptr* is the same futex as *requeue_ptr*
+ *new_requeue_owner* is currently a member of the waiters for either *value_ptr* or *requeue_ptr*
+ *new_requeue_owner* has not been started yet.

**ZX_ERR_BAD_HANDLE**  *new_requeue_owner* is not **ZX_HANDLE_INVALID**, and not a valid handle AND
*current_value* matches the value at *value_ptr*

**ZX_ERR_WRONG_TYPE**  *new_requeue_owner* is a valid handle, but is not a handle to a thread.

**ZX_ERR_BAD_STATE**  *current_value* does not match the value at *value_ptr*.

## See also

 - [futex objects]
 - [`zx_futex_requeue_single_owner()`]
 - [`zx_futex_wait()`]
 - [`zx_futex_wake()`]

[futex objects]: /docs/reference/kernel_objects/futex.md
[`zx_futex_requeue_single_owner()`]: futex_requeue_single_owner.md
[`zx_futex_wait()`]: futex_wait.md
[`zx_futex_wake()`]: futex_wake.md
