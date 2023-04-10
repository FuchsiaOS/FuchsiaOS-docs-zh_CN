<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_job_create

## Summary

Create a new job.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_job_create(zx_handle_t parent_job,
                          uint32_t options,
                          zx_handle_t* out);
```

## Description

`zx_job_create()` creates a new child [job object](/reference/kernel_objects/job.md) given a
parent job.

Upon success a handle for the new job is returned.

The kernel keeps track of and restricts the "height" of a job, which is its
distance from the root job. It is illegal to create a job under a parent whose
height exceeds an internal "max height" value. (It is, however, legal to create
a process under such a job.)

Job handles may be waited on (TODO(cpu): expand this)

## Rights

*parent_job* must be of type **ZX_OBJ_TYPE_JOB** and have **ZX_RIGHT_MANAGE_JOB**.

## Return value

`zx_job_create()` returns **ZX_OK** and a handle to the new job
(via *out*) on success.  In the event of failure, a negative error value
is returned.

## Errors

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

## See also

 - [`zx_object_get_property()`]
 - [`zx_process_create()`]
 - [`zx_task_kill()`]

[`zx_object_get_property()`]: object_get_property.md
[`zx_process_create()`]: process_create.md
[`zx_task_kill()`]: task_kill.md
