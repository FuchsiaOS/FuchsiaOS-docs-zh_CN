# zx_job_set_critical

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Set a process as critical to a job.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_job_set_critical(zx_handle_t job,
                                uint32_t options,
                                zx_handle_t process);
```

## DESCRIPTION

Sets *process* as critical to *job*. When *process* terminates, *job* will be
terminated as if [`zx_task_kill()`] was called on it. The return code used will
be **ZX_TASK_RETCODE_CRITICAL_PROCESS_KILL**.

The *job* specified must be the parent of *process*, or an ancestor.

If *options* is **ZX_JOB_CRITICAL_PROCESS_RETCODE_NONZERO**, then *job* will
only be terminated if *process* has a non-zero return code.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*job* must have **ZX_RIGHT_DESTROY**.

*process* must have **ZX_RIGHT_WAIT**.

## RETURN VALUE

`zx_job_set_critical()` returns **ZX_OK** on success. In the event of failure, a
negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *job* or *process* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *job* or *process* is not a job handle.

**ZX_ERR_INVALID_ARGS**  *options* is not 0 or
**ZX_JOB_CRITICAL_PROCESS_RETCODE_NONZERO**, or *job* is not the parent of
*process*, or an ancestor.

**ZX_ERR_ALREADY_BOUND**  *process* has already been set as critical to a job.

**ZX_ERR_ACCESS_DENIED**  *job* does not have **ZX_RIGHT_DESTROY** or *process*
does not have **ZX_RIGHT_WAIT**.

## SEE ALSO

 - [`zx_job_create()`]
 - [`zx_process_create()`]
 - [`zx_task_kill()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_job_create()`]: job_create.md
[`zx_process_create()`]: process_create.md
[`zx_task_kill()`]: task_kill.md
