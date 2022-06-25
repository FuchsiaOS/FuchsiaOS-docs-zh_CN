# zx_debuglog_create

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Create an object allowing access to the kernel debuglog.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_debuglog_create(zx_handle_t resource,
                               uint32_t options,
                               zx_handle_t* out);
```

## DESCRIPTION

`zx_debuglog_create()` creates an object allowing access to the kernel
debuglog using the `zx_debuglog_read()` and `zx_debuglog_write()` syscalls.

If *options* is set to `0`, the returned handle will have the
**ZX_RIGHT_WRITE** right, giving write-only access to the kernel debuglog. If
*options* is set to **ZX_LOG_FLAG_READABLE**, the returned handle will
additionally have **ZX_RIGHT_READ**, giving read/write access to the kernel
debuglog.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*resource* must have resource kind **ZX_RSRC_KIND_ROOT**.

## RETURN VALUE

`zx_debuglog_create()` returns **ZX_OK** on success, returning a handle to the
created object in *out*.

In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_INVALID_ARGS**  *options* contained a value not understood by the kernel

**ZX_ERR_WRONG_TYPE**  *resource* was not of the kind **ZX_RSRC_KIND_ROOT**.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.

## SEE ALSO

 - [`zx_debuglog_read()`]
 - [`zx_debuglog_write()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_debuglog_read()`]: debuglog_read.md
[`zx_debuglog_write()`]: debuglog_write.md
