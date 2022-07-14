# zx_nanosleep

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

High resolution sleep.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_nanosleep(zx_time_t deadline);
```

## DESCRIPTION

`zx_nanosleep()` suspends the calling thread execution until *deadline* passes
on **ZX_CLOCK_MONOTONIC**. A *deadline* value less than or equal to **0** immediately
yields the thread. *deadline* will be automatically adjusted according to the job's
[timer slack] policy.

To sleep for a duration, use [`zx_deadline_after()`] and the
**ZX_\<time-unit\>** helpers:

```
#include <zircon/syscalls.h> // zx_deadline_after, zx_nanosleep
#include <zircon/types.h> // ZX_MSEC et al.

// Sleep 50 milliseconds
zx_nanosleep(zx_deadline_after(ZX_MSEC(50)));
```

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

None.

## RETURN VALUE

`zx_nanosleep()` always returns **ZX_OK**.

## SEE ALSO

 - [timer slack]
 - [`zx_deadline_after()`]
 - [`zx_timer_cancel()`]
 - [`zx_timer_create()`]
 - [`zx_timer_set()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[timer slack]: /concepts/kernel/timer_slack.md
[`zx_deadline_after()`]: deadline_after.md
[`zx_timer_cancel()`]: timer_cancel.md
[`zx_timer_create()`]: timer_create.md
[`zx_timer_set()`]: timer_set.md
