# zx_vmo_create_contiguous

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32938)

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_create_contiguous(zx_handle_t bti,
                                     size_t size,
                                     uint32_t alignment_log2,
                                     zx_handle_t* out);
```

## DESCRIPTION

TODO(fxbug.dev/32938)

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*bti* must be of type **ZX_OBJ_TYPE_BTI** and have **ZX_RIGHT_MAP**.

## RETURN VALUE

TODO(fxbug.dev/32938)

## ERRORS

TODO(fxbug.dev/32938)

## SEE ALSO


TODO(fxbug.dev/32938)
