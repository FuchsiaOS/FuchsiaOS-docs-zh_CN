# zx_task_suspend

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Suspend the given task. Currently only thread or process handles may be suspended.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_task_suspend(zx_handle_t handle, zx_handle_t* token);
```

## DESCRIPTION

`zx_task_suspend()` causes the requested task to suspend
execution. Task suspension is not synchronous and the task might not
be suspended before the call returns. The task will be suspended soon
after `zx_task_suspend()` is invoked, unless it is currently blocked in
the kernel, in which case it will suspend after being unblocked.

Tasks can be suspended and/or resumed before they are started. If a task is
started while suspended, it will enter suspension before executing any code.
Similarly, starting a new thread on a suspended process will suspend the thread
before it executes any code.

Invoking [`zx_task_kill()`] on a task that is suspended will successfully kill
the task.

A task cannot suspend itself or any of its parent tasks because it would never
receive the suspend token and would be unable to resume execution.

## RESUMING

To allow the task to resume, close the suspend token handle. The task will
remain suspended as long as there are any open suspend tokens. Like suspending,
resuming is asynchronous so the thread may not be in a running state when the
[`zx_handle_close()`] call returns, even if no other suspend tokens
are open.

## SIGNALS AND EXCEPTIONS

There are two relevant signals that a thread can assert:

- **ZX_THREAD_RUNNING**
- **ZX_THREAD_SUSPENDED**

Neither of these will be asserted until the thread is started via
[`zx_process_start()`] or [`zx_thread_start()`]. When
a thread starts, it will assert **ZX_THREAD_RUNNING** whether it is suspended
or not, but if it is suspended will then switch to **ZX_THREAD_SUSPENDED**
before executing any code.

The **ZX_EXCP_PROCESS_STARTING** and **ZX_EXCP_THREAD_STARTING** debug
exceptions will also be sent on start whether the task is suspended or not.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_THREAD** or **ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_task_suspend()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE** *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *handle* is not a thread or process handle.

**ZX_ERR_INVALID_ARGS**  *token*  was an invalid pointer.

**ZX_ERR_BAD_STATE**  The task is already dying or dead and cannot be suspended.

**ZX_ERR_NO_MEMORY**  Failed to allocate memory.

**ZX_ERR_NOT_SUPPORTED**  The calling thread is attempting to suspend itself or
                          one of its parent tasks.

## LIMITATIONS

Currently only thread and process handles are supported.

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_handle_close()`]: handle_close.md
[`zx_process_start()`]: process_start.md
[`zx_task_kill()`]: task_kill.md
[`zx_thread_start()`]: thread_start.md
