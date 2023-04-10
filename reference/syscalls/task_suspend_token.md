<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_task_suspend_token

This function replaces [task_suspend](task_suspend.md). When all callers are
updated, [`zx_task_suspend()`] will be deleted and this function will be renamed
[`zx_task_suspend()`].

## Summary

Suspend the given task. Currently only thread or process handles may be suspended.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_task_suspend_token(zx_handle_t handle, zx_handle_t* token);
```

## Description

`zx_task_suspend_token()` causes the requested task to suspend execution. Task
suspension is not synchronous and the task might not be suspended before the
call returns. The task will be suspended soon after `zx_task_suspend_token()` is
invoked, unless it is currently blocked in the kernel, in which case it will
suspend after being unblocked.

Invoking [`zx_task_kill()`] on a task that is suspended will successfully kill
the task.

## RESUMING

The allow the task to resume, close the suspend token handle. The task will
remain suspended as long as there are any open suspend tokens. Like suspending,
resuming is asynchronous so the thread may not be in a running state when the
[`zx_handle_close()`] call returns, even if no other suspend tokens
are open.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_THREAD** or **ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_WRITE**.

## Return value

[`zx_task_suspend()`] returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE** *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *handle* is not a thread handle.

**ZX_ERR_INVALID_ARGS**  *token*  was an invalid pointer.

**ZX_ERR_BAD_STATE**  The task is not in a state where suspending is possible.

## LIMITATIONS

Currently only thread handles are supported.

[`zx_handle_close()`]: handle_close.md
[`zx_task_kill()`]: task_kill.md
[`zx_task_suspend()`]: task_suspend.md
