# Jobs

This document discsusses the concept of jobs in Fuchsia, specifically,
how to use jobs and how jobs relate to processes.

Note: This document is currently being iterated on and is subject to change.

## Overview

In Fuchsia, jobs are a means of organizing, controlling, and regulating
processes, which are the main consumers of resources within the
operating system.

In Fuchsia, a job is a [kernel object](/docs/reference/kernel_objects/objects.md)
consisting of the following:

*   A reference to a parent job
*   A set of child jobs (each of which has this job as its parent)
*   A set of member processes
*   A set of policies

[Jobs](/docs/reference/kernel_objects/job.md) are
containers of processes and other "child" jobs. Jobs are used to track
privileges in order to perform kernel operations, such as making syscalls,
tracking, and limiting basic resource (for example, memory and CPU) consumption.
In Fuchsia, jobs propagate exceptions upwards along the job tree
but apply policies and quotas in a top down manner. For more information, see
[Exception handling](/docs/concepts/kernel/exceptions.md).

Each process can only have one job. All the jobs on a Fuchsia system
form a tree, with every job belonging to a parent job, except for the root job,
which is parentless.

The root job is created by
the kernel at boot and passed to `userboot`. This is the first userspace
process to begin execution.

## Jobs and processes

A Zircon process is an instance of a program in the traditional sense, which is,
a set of instructions that are executed by one or more threads, along with a
collection of resources.
[`zx_process_create()`](/docs/reference/syscalls/process_create.md) creates a
new process within a job. The thread is not executed
until [`zx_process_start()`](/docs/reference/syscalls/process_start.md)
is called.

A job may be used to terminate, get exceptions from, or debug a child process.
To create a process you have to pass a job to `zx_process_create()`. In Fuchsia,
a process cannot allocate memory or communicate without being explicitly
granted resources necessary to do so.

### Setting job policies

A security policy, resource policy, or both may be set on an empty job. The
job's effective policies are the combination of the parent's effective policies
and the policies specified in the job policy.

The [`zx_job_set_policy()`](/docs/reference/syscalls/job_set_policy.md) system
call sets job security and resource policies to an empty job. After this call
succeeds any new child process or child job has the new effective policy
applied to it.

## Related syscalls

*   [`zx_job_create()`](/docs/reference/syscalls/job_create.md) creates a new
job within a job; creates a new child [job object](/docs/reference/kernel_objects/job.md)
given a parent job.
*   [`zx_job_set_critical()`](/docs/reference/syscalls/job_set_critical.md) sets
a process as critical to a job.
*   When a give process terminates, the corresponding job is be terminated as
if [`zx_task_kill()`](/docs/reference/syscalls/task_kill.md) was called on
it. The return code used is `ZX_TASK_RETCODE_CRITICAL_PROCESS_KILL`.
*   [`zx_process_create()`](/docs/reference/syscalls/process_create.md) creates
a new process.
*   [`zx_thread_create()`](/docs/reference/syscalls/thread_create.md) creates a
thread within the specified process. The thread does not start executing until
[`zx_thread_start()`](/docs/reference/syscalls/thread_start.md) is called.
