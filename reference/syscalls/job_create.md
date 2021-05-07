# zx_job_create

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Create a new job.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_job_create(zx_handle_t parent_job,
                          uint32_t options,
                          zx_handle_t* out);
```

## DESCRIPTION

`zx_job_create()` creates a new child [job object](/docs/reference/kernel_objects/job.md) given a
parent job.

Upon success a handle for the new job is returned.

The kernel keeps track of and restricts the "height" of a job, which is its
distance from the root job. It is illegal to create a job under a parent whose
height exceeds an internal "max height" value. (It is, however, legal to create
a process under such a job.)

Job handles may be waited on (TODO(cpu): expand this)

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*parent_job* must be of type **ZX_OBJ_TYPE_JOB** and have **ZX_RIGHT_MANAGE_JOB**.

## RETURN VALUE

`zx_job_create()` returns **ZX_OK** and a handle to the new job
(via *out*) on success.  In the event of failure, a negative error value
is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *parent_job* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *parent_job* is not a job handle.

**ZX_ERR_INVALID_ARGS**  *options* is nonzero, or *out* is an invalid pointer.

**ZX_ERR_ACCESS_DENIED**  *parent_job* does not have the **ZX_RIGHT_WRITE** or
**ZX_RIGHT_MANAGE_JOB** right.

**ZX_ERR_OUT_OF_RANGE**  The height of *parent_job* is too large to create a child job.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_BAD_STATE**  The parent job object is in the dead state.

## SEE ALSO

 - [`zx_object_get_property()`]
 - [`zx_process_create()`]
 - [`zx_task_kill()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_object_get_property()`]: object_get_property.md
[`zx_process_create()`]: process_create.md
[`zx_task_kill()`]: task_kill.md
