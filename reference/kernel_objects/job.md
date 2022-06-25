# Job

## NAME

job - Control a group of processes

## SYNOPSIS

A job is a group of [processes](process.md) and possibly other (child)
jobs. Jobs are used to track privileges to perform kernel operations (i.e., make
various syscalls, with various options), and track and limit basic resource
(e.g., memory, CPU) consumption. Every process belongs to a single job. All the
jobs on a Fuchsia system form a tree, with every job, except the root job,
belonging to a single (parent) job.

## DESCRIPTION

A job is an object consisting of the following:

+ a reference to a parent job
+ a set of child jobs (each of which has this job as its parent)
+ a set of member processes
+ a set of policies [⚠ not implemented]

Jobs allow "applications" that are composed of more than one process to be
controlled as a single entity.

For more information, see [Jobs](/docs/concepts/process/jobs.md).


## SYSCALLS

 - [`zx_job_create()`] - create a new child job.
 - [`zx_job_set_critical()`] - set a process as critical to a job.
 - [`zx_job_set_policy()`] - set policy for new processes in the job.
 - [`zx_process_create()`] - create a new process within a job.
 - [`zx_task_create_exception_channel()`] - listen for task exceptions
 - [`zx_task_kill()`] - cause a task to stop running.

[`zx_job_create()`]: /docs/reference/syscalls/job_create.md
[`zx_job_set_critical()`]: /docs/reference/syscalls/job_set_critical.md
[`zx_job_set_policy()`]: /docs/reference/syscalls/job_set_policy.md
[`zx_process_create()`]: /docs/reference/syscalls/process_create.md
[`zx_task_create_exception_channel()`]: /docs/reference/syscalls/task_create_exception_channel.md
[`zx_task_kill()`]: /docs/reference/syscalls/task_kill.md
