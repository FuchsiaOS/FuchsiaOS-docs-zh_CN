# zx_debuglog_write

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Write a message to the kernel debuglog.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_debuglog_write(zx_handle_t handle,
                              uint32_t options,
                              const void* buffer,
                              size_t buffer_size);
```

## DESCRIPTION

`zx_debuglog_write()` attempts to write *buffer* of size *buffer_size* bytes
to the kernel debuglog. The log entry is written at severity `ZX_LOG_INFO`.

*buffer* does not strictly require any particular format, but most userspace
tooling assumes the contents will be a valid UTF-8 string without any
NUL-termination. *buffer* will be truncated to some size less than
ZX_LOG_RECORD_MAX so that it may fit in the `data` field of `zx_log_record_t`.

<!-- TODO(fxbug.dev/72345): Consider documenting ZX_LOG_LOCAL -->

*options* must be set to `0`.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_LOG** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_debuglog_read()` returns **ZX_OK** on success. In the event of failure,
a negative error value is returned.

## ERRORS

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_WRITE**.

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_INVALID_ARGS**  An invalid value to *options* was given, or *buffer*
was an invalid pointer.

**ZX_ERR_WRONG_TYPE**  *handle* is not a debuglog handle.

## SEE ALSO

 - [`fuchsia.boot.WriteOnlyLog`](https://fuchsia.dev/reference/fidl/fuchsia.boot#WriteOnlyLog)

 - [`zx_debug_write()`]
 - [`zx_debuglog_create()`]
 - [`zx_debuglog_read()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_debug_write()`]: debug_write.md
[`zx_debuglog_create()`]: debuglog_create.md
[`zx_debuglog_read()`]: debuglog_read.md
