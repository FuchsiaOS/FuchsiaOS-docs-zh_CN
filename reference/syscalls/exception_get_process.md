# zx_exception_get_process

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Create a handle for the exception's process.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_exception_get_process(zx_handle_t handle, zx_handle_t* out);
```

## DESCRIPTION

The exception process is only available for job and process exception channels,
thread exceptions cannot access their parent process handles.

*handle* is the exception handle.

*out* will be filled with a new handle to the exception process. This handle
will have the same rights as the task given to
[`zx_task_create_exception_channel()`].

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_EXCEPTION**.

## RETURN VALUE

`zx_exception_get_process()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_ACCESS_DENIED** *handle* was received via a thread exception channel.

**ZX_ERR_BAD_HANDLE** *handle* is not a valid handle.

**ZX_ERR_NO_MEMORY**  Failed to allocate memory for a new handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not an exception.

## SEE ALSO

 - [exceptions]
 - [`zx_exception_get_thread()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[exceptions]: /concepts/kernel/exceptions.md
[`zx_exception_get_thread()`]: exception_get_thread.md
[`zx_task_create_exception_channel()`]: task_create_exception_channel.md
