# zx_cprng_draw

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Draw from the kernel's CPRNG.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

void zx_cprng_draw(void* buffer, size_t buffer_size);
```

## DESCRIPTION

`zx_cprng_draw()` draws random bytes from the kernel CPRNG.  This data should
be suitable for cryptographic applications.

Clients that require a large volume of randomness should consider using these
bytes to seed a user-space random number generator for better performance.
"Large" here would mean a large multiple of **ZX_CPRNG_DRAW_MAX_LEN**, which is
currently 256. As always, test the actual latency of your call site before
optimizing.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## NOTES

`zx_cprng_draw()` terminates the calling process if **buffer** is not a valid
userspace pointer.

There are no other error conditions.  If its arguments are valid,
`zx_cprng_draw()` will succeed.

## SEE ALSO

 - [`zx_cprng_add_entropy()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_cprng_add_entropy()`]: cprng_add_entropy.md
