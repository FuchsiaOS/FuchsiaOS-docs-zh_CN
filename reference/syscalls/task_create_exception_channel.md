# zx_task_create_exception_channel

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Create an exception channel for a given job, process, or thread.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_task_create_exception_channel(zx_handle_t handle,
                                             uint32_t options,
                                             zx_handle_t* out);
```

## DESCRIPTION

`zx_task_create_exception_channel()` creates a channel that will receive
exceptions from the thread, process, or job.

*handle* is the thread, process, or job handle to receive exceptions from.

*options* can be 0 or **ZX_EXCEPTION_CHANNEL_DEBUGGER** to register for debug
exceptions (process or job only).

*out* will be filled with the newly created channel endpoint on success. This
channel will be read-only with the following rights:

* **ZX_RIGHT_TRANSFER**
* **ZX_RIGHT_WAIT**
* **ZX_RIGHT_READ**

### Exception Messages

When an exception occurs, the channel will receive a message containing one
exception handle and one `zx_exception_info_t` data.

The thread will remain blocked in the exception until the received exception
handle is closed, at which point it will either resume or exception processing
will continue according to the chosen behavior (see **ZX_PROP_EXCEPTION_STATE**
in [`zx_object_get_property()`]).

### Unbinding

Closing the created channel handle will unregister the exception handler. If
an exception message is waiting in the channel at the time it's closed, exception
handling will continue on to the next handler in the search order.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must have **ZX_RIGHT_INSPECT** and have **ZX_RIGHT_DUPLICATE** and have **ZX_RIGHT_TRANSFER** and have **ZX_RIGHT_MANAGE_THREAD**.

If *handle* is of type **ZX_OBJ_TYPE_JOB** or **ZX_OBJ_TYPE_PROCESS**, it must have **ZX_RIGHT_ENUMERATE**.

## RETURN VALUE

`zx_task_create_exception_channel()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_ACCESS_DENIED** The caller has a job policy in place preventing the
creation of new channels.

**ZX_ERR_ALREADY_BOUND** *handle* is already bound to an exception channel.

**ZX_ERR_BAD_HANDLE** *handle* is not a valid handle.

**ZX_ERR_BAD_STATE** *handle* is dying or dead.

**ZX_ERR_INVALID_ARGS** A bad value has been passed in *options*.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_WRONG_TYPE**  *handle* is not that of a job, process, or thread.

## SEE ALSO

 - [exceptions]
 - [`zx_channel_read()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[exceptions]: /concepts/kernel/exceptions.md
[`zx_channel_read()`]: channel_read.md
[`zx_object_get_property()`]: object_get_property.md
