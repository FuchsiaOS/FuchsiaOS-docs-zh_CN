<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_process_start

## Summary

Start execution on a process.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_process_start(zx_handle_t handle,
                             zx_handle_t thread,
                             zx_vaddr_t entry,
                             zx_vaddr_t stack,
                             zx_handle_t arg1,
                             uintptr_t arg2);
```

## Description

`zx_process_start()` is similar to [`zx_thread_start()`], but is used for the
purpose of starting the first thread in a process.

`zx_process_start()` causes a thread to begin execution at the program
counter specified by *entry* and with the stack pointer set to *stack*.
The arguments *arg1* and *arg2* are arranged to be in the architecture
specific registers used for the first two arguments of a function call
before the thread is started.  All other registers are zero upon start.

The first argument (*arg1*) is a handle, which will be transferred from
the process of the caller to the process being started, and an
appropriate handle value will be placed in arg1 for the newly started
thread. If `zx_process_start()` returns an error, *arg1* is closed rather
than transferred to the process being started.

Alternatively, *arg1* can be **ZX_HANDLE_INVALID** instead of a handle.
In this case the process starts with **ZX_HANDLE_INVALID** (i.e. zero)
in its first argument register instead of a handle.  This means there
are *no* handles in the process and *can never* be any handles to any
objects shared outside the process.  `zx_process_start()` is the only
way to transfer a handle into a process that doesn't involve the process
making some system call using a handle it already has (*arg1* is usually
the "bootstrap" handle).  A process with no handles can make the few
system calls that don't require a handle, such as [`zx_process_exit()`],
if it's been provided with a vDSO mapping.  It can create new kernel
objects with system calls that don't require a handle, such as
[`zx_vmo_create()`], but there is no way to make use of those objects
without more handles and no way to transfer them outside the process.
Its only means of communication is via the memory mapped into its
address space by others.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_WRITE**.

*thread* must be of type **ZX_OBJ_TYPE_THREAD** and have **ZX_RIGHT_WRITE**.

*arg1* must have **ZX_RIGHT_TRANSFER**.

## Return value

`zx_process_start()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE**  *process* or *thread* or *arg1* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *process* is not a process handle or *thread* is
not a thread handle.

**ZX_ERR_ACCESS_DENIED**  The handle *thread* lacks **ZX_RIGHT_WRITE** or *thread*
does not belong to *process*, or the handle *process* lacks **ZX_RIGHT_WRITE** or
*arg1* lacks **ZX_RIGHT_TRANSFER**.

**ZX_ERR_BAD_STATE**  *process* is already running or has exited.

**ZX_ERR_INVALID_ARGS** *entry* is not a userspace address, is not a
canonical address, or is not `0`.

## See also

 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]
 - [`zx_process_create()`]
 - [`zx_thread_create()`]
 - [`zx_thread_exit()`]
 - [`zx_thread_start()`]

[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
[`zx_process_create()`]: process_create.md
[`zx_process_exit()`]: process_exit.md
[`zx_thread_create()`]: thread_create.md
[`zx_thread_exit()`]: thread_exit.md
[`zx_thread_start()`]: thread_start.md
[`zx_vmo_create()`]: vmo_create.md
