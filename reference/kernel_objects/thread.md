# Thread

## NAME

thread - runnable / computation entity

## SYNOPSIS

TODO

## DESCRIPTION

The thread object is the construct that represents a time-shared CPU execution
context. Thread objects live associated to a particular
[Process Object](process.md), which provides the memory and the handles to other
objects necessary for I/O and computation.

### Lifetime
Threads are created by calling [`zx_thread_create()`], but only start executing
when either [`zx_thread_start()`] or [`zx_process_start()`] are called. Both syscalls
take as an argument the entrypoint of the initial routine to execute.

The thread passed to [`zx_process_start()`] should be the first thread to start execution
on a process.

A thread terminates execution:
+ by calling [`zx_thread_exit()`]
+ by calling [`zx_vmar_unmap_handle_close_thread_exit()`]
+ by calling [`zx_futex_wake_handle_close_thread_exit()`]
+ when the parent process terminates
+ by calling [`zx_task_kill()`] with the thread's handle
+ after generating an exception for which there is no handler or the handler
decides to terminate the thread.

Returning from the entrypoint routine does not terminate execution. The last
action of the entrypoint should be to call [`zx_thread_exit()`] or one of the
above mentioned `_exit()` variants.

Closing the last handle to a thread does not terminate execution. In order to
forcefully kill a thread for which there is no available handle, use
[`zx_object_get_child()`] to obtain a handle to the thread. This method is strongly
discouraged. Killing a thread that is executing might leave the process in a
corrupt state.

Fuchsia native threads are always *detached*. That is, there is no *join()* operation
needed to do a clean termination. However, some runtimes above the kernel, such as
C11 or POSIX might require threads to be joined.

### Signals
Threads provide the following signals:
+ `ZX_THREAD_TERMINATED`
+ `ZX_THREAD_SUSPENDED`
+ `ZX_THREAD_RUNNING`

When a thread is started `ZX_THREAD_RUNNING` is asserted. When it is suspended
`ZX_THREAD_RUNNING` is deasserted, and `ZX_THREAD_SUSPENDED` is asserted. When
the thread is resumed `ZX_THREAD_SUSPENDED` is deasserted and
`ZX_THREAD_RUNNING` is asserted. When a thread terminates both
`ZX_THREAD_RUNNING` and `ZX_THREAD_SUSPENDED` are deasserted and
`ZX_THREAD_TERMINATED` is asserted.

Note that signals are OR'd into the state maintained by the
[`zx_object_wait_*()`](/docs/reference/syscalls/object_wait_one.md) family of functions thus
you may see any combination of requested signals when they return.

## SYSCALLS

 - [`zx_thread_create()`] - create a new thread within a process
 - [`zx_thread_exit()`] - exit the current thread
 - [`zx_thread_read_state()`] - read register state from a thread
 - [`zx_thread_start()`] - cause a new thread to start executing
 - [`zx_thread_write_state()`] - modify register state of a thread

<br>

 - [`zx_task_create_exception_channel()`] - listen for task exceptions
 - [`zx_task_kill()`] - cause a task to stop running

[`zx_futex_wake_handle_close_thread_exit()`]: /docs/reference/syscalls/futex_wake_handle_close_thread_exit.md
[`zx_object_get_child()`]: /docs/reference/syscalls/object_get_child.md
[`zx_process_start()`]: /docs/reference/syscalls/process_start.md
[`zx_task_create_exception_channel()`]: /docs/reference/syscalls/task_create_exception_channel.md
[`zx_task_kill()`]: /docs/reference/syscalls/task_kill.md
[`zx_thread_create()`]: /docs/reference/syscalls/thread_create.md
[`zx_thread_exit()`]: /docs/reference/syscalls/thread_exit.md
[`zx_thread_read_state()`]: /docs/reference/syscalls/thread_read_state.md
[`zx_thread_write_state()`]: /docs/reference/syscalls/thread_write_state.md
[`zx_thread_start()`]: /docs/reference/syscalls/thread_start.md
[`zx_vmar_unmap_handle_close_thread_exit()`]: /docs/reference/syscalls/vmar_unmap_handle_close_thread_exit.md
