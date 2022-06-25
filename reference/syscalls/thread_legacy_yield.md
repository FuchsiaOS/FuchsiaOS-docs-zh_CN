# zx_thread_legacy_yield

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Yield the CPU of the current thread back to the scheduler.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_thread_legacy_yield(uint32_t options);
```

## DESCRIPTION

`zx_thread_legacy_yield()` causes the calling thread to yield the CPU back to the scheduler.

 Yield may result in other threads with similar importance running ahead of the current thread,
 however, the exact behavior is unspecified.

 `options` must be zero.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_thread_legacy_yield(uint32_t options)` will always return `ZX_OK` for zero `options`.
Any other `options` value, will result in `ZX_ERR_INVALID_ARGS`.

## SEE ALSO

 - [`zx_nanosleep()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_nanosleep()`]: nanosleep.md
