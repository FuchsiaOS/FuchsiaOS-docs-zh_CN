# zx_timer_set

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Start a timer.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_timer_set(zx_handle_t handle,
                         zx_time_t deadline,
                         zx_duration_t slack);
```

## DESCRIPTION

`zx_timer_set()` starts a one-shot timer that will fire when
*deadline* passes. If a previous call to `zx_timer_set()` was
pending, the previous timer is canceled and
**ZX_TIMER_SIGNALED** is de-asserted as needed.

The *deadline* parameter specifies a deadline with respect to
**ZX_CLOCK_MONOTONIC**. To wait for a relative interval,
use [`zx_deadline_after()`] returned value in *deadline*.

To fire the timer immediately pass a *deadline* less than or equal to **0**.

When the timer fires it asserts **ZX_TIMER_SIGNALED**. To de-assert this
signal call [`zx_timer_cancel()`] or `zx_timer_set()` again.

The *slack* parameter specifies a range from *deadline* - *slack* to
*deadline* + *slack* during which the timer is allowed to fire. The system
uses this parameter as a hint to coalesce nearby timers.

The precise coalescing behavior is controlled by the *options* parameter
specified when the timer was created. **ZX_TIMER_SLACK_EARLY** allows only
firing in the *deadline* - *slack* interval and **ZX_TIMER_SLACK_LATE**
allows only firing in the *deadline* + *slack* interval. The default
option value of 0 is **ZX_TIMER_SLACK_CENTER** and allows both early and
late firing with an effective interval of *deadline* - *slack* to
*deadline* + *slack*

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_TIMER** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_timer_set()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.


## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_ACCESS_DENIED**  *handle* lacks the right **ZX_RIGHT_WRITE**.

**ZX_ERR_OUT_OF_RANGE**  *slack* is negative.

## SEE ALSO

 - [`zx_deadline_after()`]
 - [`zx_timer_cancel()`]
 - [`zx_timer_create()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_deadline_after()`]: deadline_after.md
[`zx_timer_cancel()`]: timer_cancel.md
[`zx_timer_create()`]: timer_create.md
