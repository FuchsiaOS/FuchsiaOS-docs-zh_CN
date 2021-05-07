# zx_thread_exit

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Terminate the current running thread.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

[[noreturn]] void zx_thread_exit(void);
```

## DESCRIPTION

`zx_thread_exit()` causes the currently running thread to cease
running and exit.

The signal **ZX_THREAD_TERMINATED** will be asserted on the thread
object upon exit and may be observed via [`zx_object_wait_one()`]
or [`zx_object_wait_many()`] on a handle to the thread.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_thread_exit()` does not return.

## SEE ALSO

 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]
 - [`zx_thread_create()`]
 - [`zx_thread_start()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
[`zx_thread_create()`]: thread_create.md
[`zx_thread_start()`]: thread_start.md
