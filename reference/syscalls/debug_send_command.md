# zx_debug_send_command

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32938)

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_debug_send_command(zx_handle_t resource,
                                  const char* buffer,
                                  size_t buffer_size);
```

## DESCRIPTION

To use the `zx_debug_send_command()` function, you must specify
`kernel.enable-debugging-syscalls=true` on the kernel command line. Otherwise,
the function returns **ZX_ERR_NOT_SUPPORTED**.

TODO(fxbug.dev/32938)

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*resource* must have resource kind **ZX_RSRC_KIND_ROOT**.

## RETURN VALUE

TODO(fxbug.dev/32938)

## ERRORS

TODO(fxbug.dev/32938)

## SEE ALSO


TODO(fxbug.dev/32938)
