# zx_debug_write

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Write a message to the debug serial port.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_debug_write(const char* buffer, size_t buffer_size);
```

## DESCRIPTION

`zx_debug_write()` attempts to write data of *buffer_size* bytes to the debug serial port.

To use the `zx_debug_write()` function, you must specify
`kernel.enable-serial-syscalls=true` or
`kernel.enable-serial-syscalls=output-only` on the kernel command line.
Otherwise, the function returns **ZX_ERR_NOT_SUPPORTED**.

`zx_debug_write` is intended for diagnostic use.  Data may be dropped or
truncated, but the data from two different `zx_debug_write` calls will not be
interleaved or reordered.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

None.

## RETURN VALUE

Returns **ZX_OK** on success.

## ERRORS

**ZX_ERR_NOT_SUPPORTED**  `kernel.enable-serial-syscalls` is not set to `true`
or `output-only` on the kernel command line.

**ZX_ERR_INVALID_ARGS** *buffer* is NULL.

## SEE ALSO

 - [kernel command line]
 - [`zx_debug_read()`]
 - [`zx_debuglog_read()`]
 - [`zx_debuglog_write()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[kernel command line]: /reference/kernel/kernel_cmdline.md
[`zx_debug_read()`]: debug_read.md
[`zx_debuglog_read()`]: debuglog_read.md
[`zx_debuglog_write()`]: debuglog_write.md
