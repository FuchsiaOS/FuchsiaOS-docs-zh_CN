# zx_thread_start

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Start execution on a thread.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_thread_start(zx_handle_t handle,
                            zx_vaddr_t thread_entry,
                            zx_vaddr_t stack,
                            uintptr_t arg1,
                            uintptr_t arg2);
```

## DESCRIPTION

`zx_thread_start()` causes a thread to begin execution at the program counter
specified by *thread_entry* and with the stack pointer set to *stack*. The
arguments *arg1* and *arg2* are arranged to be in the architecture specific
registers used for the first two arguments of a function call before the thread
is started.  All other registers are zero upon start.

When the last handle to a thread is closed, the thread is destroyed.

Thread handles may be waited on and will assert the signal
**ZX_THREAD_TERMINATED** when the thread stops executing (due to
[`zx_thread_exit()`] being called).

*thread_entry* shall point to a function that must call [`zx_thread_exit()`] or
[`zx_futex_wake_handle_close_thread_exit()`] or
[`zx_vmar_unmap_handle_close_thread_exit()`] before reaching the last instruction.
Below is an example:

```
void thread_entry(uintptr_t arg1, uintptr_t arg2) __attribute__((noreturn)) {
	// do work here.

	zx_thread_exit();
}
```

Failing to call one of the exit functions before reaching the end of
the function will cause an architecture / toolchain specific exception.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_THREAD** and have **ZX_RIGHT_MANAGE_THREAD**.

## RETURN VALUE

`zx_thread_start()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *thread* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *thread* is not a thread handle.

**ZX_ERR_ACCESS_DENIED**  The handle *thread* lacks **ZX_RIGHT_WRITE**.

**ZX_ERR_BAD_STATE**  *thread* is not ready to run or the process *thread*
is part of is no longer alive.

**ZX_ERR_INVALID_ARGS** *thread_entry* is not a userspace address, is not a
[canonical address], or is not `0`.

## SEE ALSO

 - [`zx_futex_wake_handle_close_thread_exit()`]
 - [`zx_handle_close()`]
 - [`zx_handle_duplicate()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]
 - [`zx_thread_create()`]
 - [`zx_thread_exit()`]
 - [`zx_vmar_unmap_handle_close_thread_exit()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_futex_wake_handle_close_thread_exit()`]: futex_wake_handle_close_thread_exit.md
[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
[`zx_thread_create()`]: thread_create.md
[`zx_thread_exit()`]: thread_exit.md
[`zx_vmar_unmap_handle_close_thread_exit()`]: vmar_unmap_handle_close_thread_exit.md
