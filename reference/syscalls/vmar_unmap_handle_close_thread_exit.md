# zx_vmar_unmap_handle_close_thread_exit

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Unmap memory, close handle, exit.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmar_unmap_handle_close_thread_exit(zx_handle_t vmar_handle,
                                                   zx_vaddr_t addr,
                                                   size_t size,
                                                   zx_handle_t close_handle);
```

## DESCRIPTION

`zx_vmar_unmap_handle_close_thread_exit()` does a sequence of three operations:

1. `zx_vmar_unmap(vmar_handle, addr, size)`
2. `zx_handle_close(close_handle)`
3. `zx_thread_exit()`

The expectation is that the first operation unmaps a region including the
calling thread's own stack.  (It's not required, but it's permitted.)  This
is valid for this call, though it would be invalid for [`zx_vmar_unmap()`] or
any other call.

If the [`zx_vmar_unmap()`] operation is successful, then this call never returns.
If *close_handle* is an invalid handle so that the [`zx_handle_close()`] operation
fails, then the thread takes a trap (as if by `__builtin_trap();`).

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_vmar_unmap_handle_close_thread_exit()` does not return on success.

## ERRORS

Same as [`zx_vmar_unmap()`].

## NOTES

The intended use for this is for a dying thread to unmap its own stack,
close its own thread handle, and exit.  The thread handle cannot be closed
beforehand because closing the last handle to a thread kills that thread.
The stack cannot be unmapped beforehand because the thread must have some
stack space on which to make its final system calls.

This call is used for detached threads, while
[`zx_futex_wake_handle_close_thread_exit()`]
is used for joinable threads.

## SEE ALSO

 - [`zx_futex_wake_handle_close_thread_exit()`]
 - [`zx_handle_close()`]
 - [`zx_thread_exit()`]
 - [`zx_vmar_unmap()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_futex_wake_handle_close_thread_exit()`]: futex_wake_handle_close_thread_exit.md
[`zx_handle_close()`]: handle_close.md
[`zx_thread_exit()`]: thread_exit.md
[`zx_vmar_unmap()`]: vmar_unmap.md
