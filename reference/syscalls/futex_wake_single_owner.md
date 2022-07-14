# zx_futex_wake_single_owner

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Wake one thread waiting on a futex, and set the ownership of the futex to that thread.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_futex_wake_single_owner(const zx_futex_t* value_ptr);
```

## DESCRIPTION

Wake one thread waiting on a futex.
If a thread is woken, ownership of the futex is transferred to that thread. If no
thread is woken (because none are waiting), ownership of the futex is set to none.

See [`zx_futex_wake()`] for a full description.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

None.

## RETURN VALUE

`zx_futex_wake_single_owner()` returns **ZX_OK** on success.

## ERRORS

**ZX_ERR_INVALID_ARGS**  *value_ptr* is not aligned.

## SEE ALSO

 - [futex objects]
 - [`zx_futex_requeue()`]
 - [`zx_futex_wait()`]
 - [`zx_futex_wake()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[futex objects]: /reference/kernel_objects/futex.md
[`zx_futex_requeue()`]: futex_requeue.md
[`zx_futex_wait()`]: futex_wait.md
[`zx_futex_wake()`]: futex_wake.md
