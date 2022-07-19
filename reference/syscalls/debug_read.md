# zx_debug_read

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Read a message from the debug serial port.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_debug_read(zx_handle_t handle,
                          char* buffer,
                          size_t buffer_size,
                          size_t* actual);
```

## DESCRIPTION

`zx_debug_read()` attempts to read data from the debug serial port.
The parameter *buffer_size* is used to specify the byte size of the read buffer.
The length of *buffer*, in bytes, is stored in the location pointed to by
*actual*.

This function will wait until at least one byte is available before it returns.
This can return up to *buffer_size* bytes.

**NOTE:** There is only one buffer of the data that is coming from the debug
serial, and calling `zx_debug_read` consumes this data. If multiple programs are calling
this at once, they will each receive pieces of the data stream.


To use the `zx_debug_read()` function, you must specify
`kernel.enable-serial-syscalls=true` on the kernel command line. Otherwise,
the function returns **ZX_ERR_NOT_SUPPORTED**.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must have resource kind **ZX_RSRC_KIND_ROOT**.

## RETURN VALUE

Returns **ZX_OK** on success. The location pointed to by *buffer* contains
*actual* bytes that were read.

## ERRORS

**ZX_ERR_NOT_SUPPORTED**  `kernel.enable-serial-syscalls` is not set to `true`
on the kernel command line.

**ZX_ERR_INVALID_ARGS**  *buffer* or *actual* are NULL.

## SEE ALSO

 - [kernel command line]
 - [`zx_debug_write()`]
 - [`zx_debuglog_read()`]
 - [`zx_debuglog_write()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[kernel command line]: /reference/kernel/kernel_cmdline.md
[`zx_debug_write()`]: debug_write.md
[`zx_debuglog_read()`]: debuglog_read.md
[`zx_debuglog_write()`]: debuglog_write.md
