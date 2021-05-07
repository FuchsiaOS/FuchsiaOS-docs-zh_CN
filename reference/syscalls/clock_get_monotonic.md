# zx_clock_get_monotonic

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Acquire the current monotonic time.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_time_t zx_clock_get_monotonic(void);
```

## DESCRIPTION

`zx_clock_get_monotonic()` returns the current time in the system
monotonic clock. This is the number of nanoseconds since the system was
powered on. It does not always reset on reboot and does not adjust during
sleep, and thus should not be used as a reliable source of uptime.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

[`zx_clock_get()`] returns the current monotonic time.

## ERRORS

`zx_clock_get_monotonic()` cannot fail.

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_clock_get()`]: clock_get.md
