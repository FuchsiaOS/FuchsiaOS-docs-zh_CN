# zx_futex_wake_handle_close_thread_exit

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Write to futex, wake futex, close handle, exit.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

[[noreturn]] void zx_futex_wake_handle_close_thread_exit(
    const zx_futex_t* value_ptr,
    uint32_t wake_count,
    int32_t new_value,
    zx_handle_t close_handle);
```

## DESCRIPTION

`zx_futex_wake_handle_close_thread_exit()` does a sequence of four operations:

1. `atomic_store_explicit(value_ptr, new_value, memory_order_release);`
2. `zx_futex_wake(value_ptr, wake_count);`
3. `zx_handle_close(close_handle);`
4. `zx_thread_exit();`

The expectation is that as soon as the first operation completes,
other threads may unmap or reuse the memory containing the calling
thread's own stack.  This is valid for this call, though it would be
invalid for plain [`zx_futex_wake()`] or any other call.

If any of the operations fail, then the thread takes a trap (as if by `__builtin_trap();`).

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_futex_wake_handle_close_thread_exit()` does not return.

## ERRORS

None.

## NOTES

The intended use for this is for a dying thread to alert another thread
waiting for its completion, close its own thread handle, and exit.
The thread handle cannot be closed beforehand because closing the last
handle to a thread kills that thread.  The write to *value_ptr* can't be
done before this call because any time after the write, a joining thread might
reuse or deallocate this thread's stack, which may cause issues with calling
conventions into this function.

This call is used for joinable threads, while
[`zx_vmar_unmap_handle_close_thread_exit()`]
is used for detached threads.

## SEE ALSO

 - [futex objects]
 - [`zx_futex_wake()`]
 - [`zx_handle_close()`]
 - [`zx_thread_exit()`]
 - [`zx_vmar_unmap_handle_close_thread_exit()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[futex objects]: /docs/reference/kernel_objects/futex.md
[`zx_futex_wake()`]: futex_wake.md
[`zx_handle_close()`]: handle_close.md
[`zx_thread_exit()`]: thread_exit.md
[`zx_vmar_unmap_handle_close_thread_exit()`]: vmar_unmap_handle_close_thread_exit.md
