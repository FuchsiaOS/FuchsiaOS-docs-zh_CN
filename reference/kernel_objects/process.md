# Process

## NAME

process - Process abstraction

## SYNOPSIS

A zircon process is an instance of a program in the traditional
sense: a set of instructions that will be executed by one or more
threads, along with a collection of resources.

## DESCRIPTION

The process object is a container of the following resources:

+ [Handles](concepts/kernel/handles.md)
+ [Virtual Memory Address Regions](vm_address_region.md)
+ [Threads](thread.md)

In general, it is associated with code, which it is executing until it is
forcefully terminated or the program exits.

Processes are owned by [jobs](job.md) and allow an application that is
composed by more than one process to be treated as a single entity, from the
perspective of resource and permission limits, as well as lifetime control.

For more information, see [Processes Overview](concepts/process/overview.md).

### Lifetime
A process is created via [`zx_process_create()`] and its execution begins with
[`zx_process_start()`].

The process stops execution when:

+ the last thread is terminated or exits
+ the process calls [`zx_process_exit()`]
+ the parent job terminates the process
+ the parent job is destroyed

The call to [`zx_process_start()`] cannot be issued twice. New threads cannot
be added to a process that was started and then its last thread has exited.

## SYSCALLS

 - [`zx_process_create()`] - create a new process within a job
 - [`zx_process_read_memory()`] - read from a process's address space
 - [`zx_process_start()`] - cause a new process to start executing
 - [`zx_process_write_memory()`] - write to a process's address space
 - [`zx_process_exit()`] - exit the current process

<br>

 - [`zx_job_create()`] - create a new job within a parent job

<br>

 - [`zx_task_create_exception_channel()`] - listen for task exceptions

<br>

 - [`zx_vmar_map()`] - Map memory into an address space range
 - [`zx_vmar_protect()`] - Change permissions on an address space range
 - [`zx_vmar_unmap()`] - Unmap memory from an address space range

[`zx_job_create()`]: reference/syscalls/job_create.md
[`zx_process_create()`]: reference/syscalls/process_create.md
[`zx_process_exit()`]: reference/syscalls/process_exit.md
[`zx_process_read_memory()`]: reference/syscalls/process_read_memory.md
[`zx_process_start()`]: reference/syscalls/process_start.md
[`zx_process_write_memory()`]: reference/syscalls/process_write_memory.md
[`zx_task_create_exception_channel()`]: reference/syscalls/task_create_exception_channel.md
[`zx_vmar_map()`]: reference/syscalls/vmar_map.md
[`zx_vmar_protect()`]: reference/syscalls/vmar_protect.md
[`zx_vmar_unmap()`]: reference/syscalls/vmar_unmap.md
