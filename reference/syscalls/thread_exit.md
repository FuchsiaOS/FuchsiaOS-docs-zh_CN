<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_thread_exit

## Summary

Terminate the current running thread.

## Declaration

```c
#include <zircon/syscalls.h>

[[noreturn]] void zx_thread_exit(void);
```

## Description

`zx_thread_exit()` causes the currently running thread to cease
running and exit.

The signal **ZX_THREAD_TERMINATED** will be asserted on the thread
object upon exit and may be observed via [`zx_object_wait_one()`]
or [`zx_object_wait_many()`] on a handle to the thread.

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_thread_exit()` does not return.

## See also

 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]
 - [`zx_thread_create()`]
 - [`zx_thread_start()`]

[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
[`zx_thread_create()`]: thread_create.md
[`zx_thread_start()`]: thread_start.md
