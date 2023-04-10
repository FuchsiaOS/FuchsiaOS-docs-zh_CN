# Task

## NAME

Task - "Runnable" subclass of kernel objects (threads, processes, and jobs)

## SYNOPSIS

[Threads](thread.md), [processes](process.md), and [jobs](job.md) objects
are all tasks. They share the ability to be suspended, resumed, and
killed.

## DESCRIPTION

TODO

## SYSCALLS

 - [`zx_task_create_exception_channel()`] - listen for task exceptions
 - [`zx_task_kill()`] - cause a task to stop running

[`zx_task_create_exception_channel()`]: /docs/reference/syscalls/task_create_exception_channel.md
[`zx_task_kill()`]: /docs/reference/syscalls/task_kill.md
