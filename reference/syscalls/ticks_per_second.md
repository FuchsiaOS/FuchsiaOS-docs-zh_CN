# zx_ticks_per_second

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Read the number of high-precision timer ticks in a second.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_ticks_t zx_ticks_per_second(void);
```

## DESCRIPTION

`zx_ticks_per_second()` returns the number of high-precision timer ticks in a
second.

This can be used together with [`zx_ticks_get()`] to calculate the amount of
time elapsed between two subsequent calls to [`zx_ticks_get()`].

This value can vary from boot to boot of a given system. Once booted,
this value is guaranteed not to change.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_ticks_per_second()` returns the number of high-precision timer ticks in a
second.

## ERRORS

`zx_ticks_per_second()` does not report any error conditions.

## EXAMPLES

```
zx_ticks_t ticks_per_second = zx_ticks_per_second();
zx_ticks_t ticks_start = zx_ticks_get();

// do some more work

zx_ticks_t ticks_end = zx_ticks_get();
double elapsed_seconds = (ticks_end - ticks_start) / (double)ticks_per_second;

```

## SEE ALSO

 - [`zx_ticks_get()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_ticks_get()`]: ticks_get.md
