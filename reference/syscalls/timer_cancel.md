# zx_timer_cancel

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Cancel a timer.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_timer_cancel(zx_handle_t handle);
```

## DESCRIPTION

`zx_timer_cancel()` cancels a pending timer that was started with
[`zx_timer_set()`].

Upon success the pending timer is canceled and the **ZX_TIMER_SIGNALED**
signal is de-asserted. If a new pending timer is immediately needed
rather than calling `zx_timer_cancel()` first, call [`zx_timer_set()`]
with the new deadline.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_TIMER** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_timer_cancel()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_ACCESS_DENIED**  *handle* lacks the right **ZX_RIGHT_WRITE**.

## NOTE

Calling this function before [`zx_timer_set()`] has no effect.

## SEE ALSO

 - [`zx_timer_create()`]
 - [`zx_timer_set()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_timer_create()`]: timer_create.md
[`zx_timer_set()`]: timer_set.md
