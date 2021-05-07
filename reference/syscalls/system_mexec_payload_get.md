# zx_system_mexec_payload_get

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Return a ZBI containing ZBI entries necessary to boot this system.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_system_mexec_payload_get(zx_handle_t resource,
                                        void* buffer,
                                        size_t buffer_size);
```

## DESCRIPTION

`zx_system_mexec_payload_get()` accepts a resource handle and a
pointer/length corresponding to an output buffer. The head of the buffer is
overwritten with an incomplete ZBI containing a sequence of entries that should
be appended to a ZBI before passing that image to [`zx_system_mexec()`]; the
tail of the buffer is left untouched.

*resource* must be of type **ZX_RSRC_KIND_ROOT**.

*buffer* and *buffer_size* must point to a buffer that is no longer than 16KiB.

To use the `zx_system_mexec_payload_get()` function, you must specify
`kernel.enable-debugging-syscalls=true` on the kernel command line. Otherwise,
the function returns **ZX_ERR_NOT_SUPPORTED**.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*resource* must have resource kind **ZX_RSRC_KIND_ROOT**.

## RETURN VALUE

`zx_system_mexec_payload_get()` returns **ZX_OK** on success.

**ZX_ERR_NOT_SUPPORTED**  `kernel.enable-debugging-syscalls` is not set to `true`
on the kernel command line.

**ZX_ERR_BUFFER_TOO_SMALL**  If the provided buffer is too small for the ZBI.

## SEE ALSO

 - [`zx_system_mexec()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_system_mexec()`]: system_mexec.md
