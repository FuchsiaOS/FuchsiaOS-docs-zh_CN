# Processes

This document introduces the concept of Processes in Zircon.

## Overview

A Zircon process is an instance of a program, consisting of a set of
instructions that are executed by one or more threads, along with a collection
of resources the program may use to carry out its objectives and interact with
the system.

The kernel manages processes using capabilities called Process Objects. Thread
Objects are associated with a particular Process Object, which provides the
memory and handles to other kernel objects necessary for I/O and computation by
the associated threads.

Every process starts with a single Virtual Memory Address Region (VMAR), the
process root VMAR, that spans the entire user address space
(see [`zx_process_create()`](reference/syscalls/process_create.md)).
The root VMAR may be used directly or subdivided into child VMARs.

VMARs are used to map Virtual Memory Objects (VMOs), which provide the code,
data, anonymous, and shared memory pages needed by the program into the address
space of the process.

A process stops execution when:

*   The last thread in the process is terminated or exits.
*   The process calls [`zx_process_exit()`](reference/syscalls/process_exit.md)
    to terminate itself.
*   The parent job terminates the process.
*   The parent job is destroyed.

## Processes and jobs

Processes are owned by [jobs](concepts/process/jobs.md), which support
grouping one or more processes and sub-jobs into a single entity that manages
resource limits and permissions, and also provides lifetime control for
the group.

Creating a process requires a handle to a job, which the newly created process
becomes a child of. Only processes that have a handle to a job can create a new
process or job, effectively restricting which processes may manually create
other processes.

Many Fuchsia processes do not have a job handle and must use a mechanism
provided by the system, such as the Component Framework, to start
another process.

For more information, see [jobs](concepts/process/jobs.md).
