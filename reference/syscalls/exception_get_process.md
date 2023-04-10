<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_exception_get_process

## Summary

Create a handle for the exception's process.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_exception_get_process(zx_handle_t handle, zx_handle_t* out);
```

## Description

The exception process is only available for job and process exception channels,
thread exceptions cannot access their parent process handles.

*handle* is the exception handle.

*out* will be filled with a new handle to the exception process. This handle
will have the same rights as the task given to
[`zx_task_create_exception_channel()`].

## Rights

*handle* must be of type **ZX_OBJ_TYPE_EXCEPTION**.

## Return value

`zx_exception_get_process()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## Errors

**ZX_ERR_ACCESS_DENIED** *handle* was received via a thread exception channel.

**ZX_ERR_BAD_HANDLE** *handle* is not a valid handle.

**ZX_ERR_NO_MEMORY**  Failed to allocate memory for a new handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not an exception.

## See also

 - [exceptions]
 - [`zx_exception_get_thread()`]

[exceptions]: /concepts/kernel/exceptions.md
[`zx_exception_get_thread()`]: exception_get_thread.md
[`zx_task_create_exception_channel()`]: task_create_exception_channel.md
