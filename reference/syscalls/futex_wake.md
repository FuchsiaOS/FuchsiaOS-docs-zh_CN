<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_futex_wake

## Summary

Wake some number of threads waiting on a futex, and set the ownership of the futex to nothing.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_futex_wake(const zx_futex_t* value_ptr, uint32_t wake_count);
```

## Description

Waking a futex causes *wake_count* threads waiting on the *value_ptr*
futex to be woken up.

Waking up zero threads is not an error condition.  Passing in an unallocated
address for *value_ptr* is not an error condition.

## OWNERSHIP

A successful call to `zx_futex_wake()` results in the owner of the futex being
set to nothing, regardless of the wake count.  In order to transfer ownership of
a futex, use the [`zx_futex_wake_single_owner()`] variant instead.
[`zx_futex_wake_single_owner()`] will attempt to wake exactly one thread from the
futex wait queue.  If there is at least one thread to wake, the owner of the
futex will be set to the thread that was woken.  Otherwise, the futex will have
no owner.

See *Ownership and Priority Inheritance* in [futex](/reference/kernel_objects/futex.md) for
details.

## Rights

None.

## Return value

`zx_futex_wake()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_INVALID_ARGS**  *value_ptr* is not aligned.

## See also

 - [futex objects]
 - [`zx_futex_requeue()`]
 - [`zx_futex_wait()`]
 - [`zx_futex_wake_single_owner()`]

[futex objects]: /reference/kernel_objects/futex.md
[`zx_futex_requeue()`]: futex_requeue.md
[`zx_futex_wait()`]: futex_wait.md
[`zx_futex_wake_single_owner()`]: futex_wake_single_owner.md
