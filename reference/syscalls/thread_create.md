# zx_thread_create

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Create a thread.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_thread_create(zx_handle_t process,
                             const char* name,
                             size_t name_size,
                             uint32_t options,
                             zx_handle_t* out);
```

## DESCRIPTION

`zx_thread_create()` creates a thread within the specified process.

Upon success a handle for the new thread is returned.  The thread
will not start executing until [`zx_thread_start()`] is called.

*name* is silently truncated to a maximum of `ZX_MAX_NAME_LEN-1` characters.

Thread handles may be waited on and will assert the signal
**ZX_THREAD_TERMINATED** when the thread stops executing (due to
[`zx_thread_exit()`] being called).

*process* is the controlling [process object](/docs/reference/kernel_objects/process.md) for the
new thread, which will become a child of that process.

For thread lifecycle details see [thread object](/docs/reference/kernel_objects/thread.md).

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*process* must be of type **ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_MANAGE_THREAD**.

## RETURN VALUE

On success, `zx_thread_create()` returns **ZX_OK** and a handle (via *out*)
to the new thread.  In the event of failure, a negative error value is
returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *process* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *process* is not a process handle.

**ZX_ERR_ACCESS_DENIED**  *process* does not have the **ZX_RIGHT_MANAGE_THREAD** right.

**ZX_ERR_INVALID_ARGS**  *name* or *out* was an invalid pointer, or *options* was
non-zero.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## SEE ALSO

 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]
 - [`zx_thread_exit()`]
 - [`zx_thread_start()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
[`zx_thread_exit()`]: thread_exit.md
[`zx_thread_start()`]: thread_start.md
