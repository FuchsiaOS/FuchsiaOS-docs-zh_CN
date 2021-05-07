# zx_cprng_add_entropy

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Add entropy to the kernel CPRNG.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_cprng_add_entropy(const void* buffer, size_t buffer_size);
```

## DESCRIPTION

`zx_cprng_add_entropy()` mixes the given entropy into the kernel CPRNG.
a privileged operation.  It will accept at most **ZX_CPRNG_ADD_ENTROPY_MAX_LEN**
bytes of entropy at a time.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_cprng_add_entropy()` returns **ZX_OK** on success.

## ERRORS

**ZX_ERR_INVALID_ARGS** *buffer_size* is too large, or *buffer* is not a valid
userspace pointer.

## BUGS

This syscall should be very privileged.

## SEE ALSO

 - [`zx_cprng_draw()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_cprng_draw()`]: cprng_draw.md
