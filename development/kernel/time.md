# Time units

## Userspace exposed time units

*zx\_time\_t* is in nanoseconds.

Use [`zx_clock_get_monotonic()`] to get the current time as the number of nanoseconds since boot.

## Kernel-internal time units

*lk\_time\_t* is in nanoseconds.

When writing code that will run in the kernel, to read the current monotonic clock value, use:

```
#include <platform.h>

lk_time_t current_time(void);
```

[`zx_clock_get_monotonic()`]: reference/syscalls/clock_get_monotonic.md
