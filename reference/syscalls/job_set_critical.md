<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_job_set_critical

## Summary

Set a process as critical to a job.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_job_set_critical(zx_handle_t job,
                                uint32_t options,
                                zx_handle_t process);
```

## Description

Sets *process* as critical to *job*. When *process* terminates, *job* will be
terminated as if [`zx_task_kill()`] was called on it. The return code used will
be **ZX_TASK_RETCODE_CRITICAL_PROCESS_KILL**.

The *job* specified must be the parent of *process*, or an ancestor.

If *options* is **ZX_JOB_CRITICAL_PROCESS_RETCODE_NONZERO**, then *job* will
only be terminated if *process* has a non-zero return code.

## Rights

*job* must have **ZX_RIGHT_DESTROY**.

*process* must have **ZX_RIGHT_WAIT**.

## Return value

`zx_job_set_critical()` returns **ZX_OK** on success. In the event of failure, a
negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE**  *job* or *process* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *job* or *process* is not a job handle.

**ZX_ERR_INVALID_ARGS**  *options* is not 0 or
**ZX_JOB_CRITICAL_PROCESS_RETCODE_NONZERO**, or *job* is not the parent of
*process*, or an ancestor.

**ZX_ERR_ALREADY_BOUND**  *process* has already been set as critical to a job.

**ZX_ERR_ACCESS_DENIED**  *job* does not have **ZX_RIGHT_DESTROY** or *process*
does not have **ZX_RIGHT_WAIT**.

## See also

 - [`zx_job_create()`]
 - [`zx_process_create()`]
 - [`zx_task_kill()`]

[`zx_job_create()`]: job_create.md
[`zx_process_create()`]: process_create.md
[`zx_task_kill()`]: task_kill.md
