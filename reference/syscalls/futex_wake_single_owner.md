<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_futex_wake_single_owner

## Summary

Wake one thread waiting on a futex, and set the ownership of the futex to that thread.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_futex_wake_single_owner(const zx_futex_t* value_ptr);
```

## Description

Wake one thread waiting on a futex.
If a thread is woken, ownership of the futex is transferred to that thread. If no
thread is woken (because none are waiting), ownership of the futex is set to none.

See [`zx_futex_wake()`] for a full description.

## Rights

None.

## Return value

`zx_futex_wake_single_owner()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_INVALID_ARGS**  *value_ptr* is not aligned.

## See also

 - [futex objects]
 - [`zx_futex_requeue()`]
 - [`zx_futex_wait()`]
 - [`zx_futex_wake()`]

[futex objects]: /reference/kernel_objects/futex.md
[`zx_futex_requeue()`]: futex_requeue.md
[`zx_futex_wait()`]: futex_wait.md
[`zx_futex_wake()`]: futex_wake.md
