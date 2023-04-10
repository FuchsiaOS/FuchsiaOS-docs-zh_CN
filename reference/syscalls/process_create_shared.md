<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_process_create_shared

## Summary

Create a new process that can share part of its address space with other processes.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_process_create_shared(zx_handle_t shared_proc,
                                     uint32_t options,
                                     const char* name,
                                     size_t name_size,
                                     zx_handle_t* proc_handle,
                                     zx_handle_t* restricted_vmar_handle);
```

## Description

`zx_process_create_shared()` creates a new process that shares part of its address space
with `shared_proc`. The created process will be added to the same job as `shared_proc`.

`shared_proc` must have been created with `ZX_PROCESS_SHARED`, or via
`zx_process_create_shared()`.

The address space of the created process is split in two: the *shared* portion which is
shared with `shared_proc`, and the *restricted* portion which is private to the created
process. Each thread in the process begins executing with the shared portion active.

For more detail, see [`zx_process_create()`].

## Rights

*shared_proc* must be of type **ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_MANAGE_PROCESS**.

See [`zx_process_create()`].

## Return value

On success, `zx_process_create_shared()` returns **ZX_OK**, a handle to the new process
(via *proc_handle*), and a handle to the root of its restricted address space (via
*restricted_vmar_handle*).  In the event of failure, a negative error value is returned.

For more detail, see [`zx_process_create()`].

## Errors

**ZX_ERR_INVALID_ARGS**  *shared_proc* is a valid handle, but pointed to a process that
was not created via `zx_process_create_shared()`, or `zx_process_create()` with
`ZX_PROCESS_SHARED`.

**ZX_ERR_BAD_HANDLE**  *shared_proc* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *shared_proc* is not a process handle.

**ZX_ERR_BAD_STATE** *shared_proc* is a valid handle but the shared state is no longer valid
(likely due to all processes sharing the state terminating).

**ZX_ERR_ACCESS_DENIED**  *shared_proc* does not have the **ZX_RIGHT_MANAGE_PROCESS** right
(only when not **ZX_HANDLE_INVALID**).

For other errors, see [`zx_process_create()`].

## See also

 - [`zx_process_create()`]

[`zx_process_create()`]: process_create.md
