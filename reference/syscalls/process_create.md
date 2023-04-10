<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_process_create

## Summary

Create a new process.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_process_create(zx_handle_t job,
                              const char* name,
                              size_t name_size,
                              uint32_t options,
                              zx_handle_t* proc_handle,
                              zx_handle_t* vmar_handle);
```

## Description

`zx_process_create()` creates a new process.

Upon success, handles for the new process and the root of its address space
are returned.  The thread will not start executing until [`zx_process_start()`] is
called.

*name* is silently truncated to a maximum of `ZX_MAX_NAME_LEN-1` characters.

When the last handle to a process is closed, the process is destroyed.

Process handles may be waited on and will assert the signal
**ZX_PROCESS_TERMINATED** when the process exits.

*job* is the controlling [job object](/docs/reference/kernel_objects/job.md) for the new
process, which will become a child of that job.

*options* can contain zero or the following flag:

 - **ZX_PROCESS_SHARED** to create a process that can share its address space with another
   process. Processes created with this flag can be passed to `zx_process_create_shared()`.

## Rights

*job* must be of type **ZX_OBJ_TYPE_JOB** and have **ZX_RIGHT_MANAGE_PROCESS**.

Caller job policy must allow **ZX_POL_NEW_PROCESS**.

## Return value

On success, `zx_process_create()` returns **ZX_OK**, a handle to the new process
(via *proc_handle*), and a handle to the root of its address space (via
*vmar_handle*).  In the event of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE**  *job* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *job* is not a job handle.

**ZX_ERR_ACCESS_DENIED**  *job* does not have the **ZX_RIGHT_WRITE** right
(only when not **ZX_HANDLE_INVALID**).

**ZX_ERR_INVALID_ARGS**  *name*, *proc_handle*, or *vmar_handle*  was an invalid pointer,
or *options* contained invalid options.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_BAD_STATE**  The job object is in the dead state.

## See also

 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_job_create()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]
 - [`zx_process_create_shared()`]
 - [`zx_process_start()`]
 - [`zx_task_kill()`]
 - [`zx_thread_create()`]
 - [`zx_thread_exit()`]
 - [`zx_thread_start()`]

[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_job_create()`]: job_create.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
[`zx_process_create_shared()`]: process_create_shared.md
[`zx_process_start()`]: process_start.md
[`zx_task_kill()`]: task_kill.md
[`zx_thread_create()`]: thread_create.md
[`zx_thread_exit()`]: thread_exit.md
[`zx_thread_start()`]: thread_start.md
