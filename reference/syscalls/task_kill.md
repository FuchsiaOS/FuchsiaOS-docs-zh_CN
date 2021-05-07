# zx_task_kill

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Kill the provided task (job or process).

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_task_kill(zx_handle_t handle);
```

## DESCRIPTION

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
**ZX_POL_ACTION_KILL** or when the system is running low on memory [OOM](/docs/development/kernel/memory/oom.md).

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must have **ZX_RIGHT_DESTROY**.

## RETURN VALUE

On success, `zx_task_kill()` returns **ZX_OK**. If a process uses
this syscall to kill itself, this syscall does not return.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a task handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the **ZX_RIGHT_DESTROY**
right.

## SEE ALSO

 - [`zx_job_create()`]
 - [`zx_process_create()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_job_create()`]: job_create.md
[`zx_object_get_info()`]: object_get_info.md
[`zx_process_create()`]: process_create.md
