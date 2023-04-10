<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_task_kill

## Summary

Kill the provided task (job, process, or thread).

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_task_kill(zx_handle_t handle);
```

## Description

This asynchronously kills the given process or job and its children recursively,
until the entire task tree rooted at *handle* is dead.
Killing a thread is not supported.

It is possible to wait for the task to be dead via the **ZX_TASK_TERMINATED**
signal. When the procedure completes, as observed by the signal, the task and
all its children are considered to be in the dead state and most operations
will no longer succeed.

If *handle* is a job and the syscall is successful, the job can no longer be
used to create new processes.

When a process or job is killed via this syscall, the `return_code` is
**ZX_TASK_RETCODE_SYSCALL_KILL** as reported by [`zx_object_get_info()`] via
the **ZX_INFO_PROCESS** or **ZX_INFO_JOB** topic.

Processes and Jobs can also be killed by other agents such as the Job policy with
**ZX_POL_ACTION_KILL** or when the system is running low on memory [OOM](/development/kernel/memory/oom.md).

## Rights

*handle* must have **ZX_RIGHT_DESTROY**.

## Return value

On success, `zx_task_kill()` returns **ZX_OK**. If a process uses
this syscall to kill itself, this syscall does not return.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a task handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the **ZX_RIGHT_DESTROY**
right.

## See also

 - [`zx_job_create()`]
 - [`zx_process_create()`]

[`zx_job_create()`]: job_create.md
[`zx_object_get_info()`]: object_get_info.md
[`zx_process_create()`]: process_create.md
