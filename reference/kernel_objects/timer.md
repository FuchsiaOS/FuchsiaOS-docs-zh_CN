# Timer

## NAME

timer - An object that may be signaled at some point in the future

## SYNOPSIS

A timer is used to wait until a specified point in time has occurred
or the timer has been canceled.

## DESCRIPTION

Like other waitable objects, timers can be waited on via
[`zx_object_wait_one()`], [`zx_object_wait_many()`], or
[`zx_object_wait_async()`].

A given timer can be used over and over.

Once **ZX_TIMER_SIGNALED** is asserted, it will remain asserted until
the timer is canceled ([`zx_timer_cancel()`]) or reset ([`zx_timer_set()`]).

The typical lifecycle is:

1. `zx_timer_create()`
2. `zx_timer_set()`
3. wait for the timer to be signaled
4. optionally reset and reuse the timer (i.e. goto #2)
5. `zx_handle_close()`

## SYSCALLS

 - [`zx_timer_create()`] - create a timer
 - [`zx_timer_set()`] - set a timer
 - [`zx_timer_cancel()`] - cancel a timer

## SEE ALSO

+ [timer slack](/docs/concepts/kernel/timer_slack.md)

[`zx_object_wait_one()`]: /docs/reference/syscalls/object_wait_one.md
[`zx_object_wait_many()`]: /docs/reference/syscalls/object_wait_many.md
[`zx_object_wait_async()`]: /docs/reference/syscalls/object_wait_async.md
[`zx_timer_create()`]: /docs/reference/syscalls/timer_create.md
[`zx_timer_set()`]: /docs/reference/syscalls/timer_set.md
[`zx_timer_cancel()`]: /docs/reference/syscalls/timer_cancel.md
