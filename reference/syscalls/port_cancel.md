# zx_port_cancel

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Cancels async port notifications on an object.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_port_cancel(zx_handle_t handle,
                           zx_handle_t source,
                           uint64_t key);
```

## DESCRIPTION

`zx_port_cancel()` is a non-blocking syscall that cancels
pending [`zx_object_wait_async()`] calls done with *source* and *key*.

When this call succeeds no new packets from the object pointed by
*source* with *key* will be delivered to *handle*, and pending queued
packets that match *source* and *key* are removed from the port.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_PORT** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_port_cancel()` returns **ZX_OK** if cancellation succeeded and
either queued packets were removed or pending [`zx_object_wait_async()`] were
canceled.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *source* or *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a port handle.

**ZX_ERR_ACCESS_DENIED**  *source* or *handle* does not have **ZX_RIGHT_WRITE**.

**ZX_ERR_NOT_SUPPORTED**  *source* is a handle that cannot be waited on.

**ZX_ERR_NOT_FOUND** if either no pending packets or pending
[`zx_object_wait_async()`] calls with *source* and *key* were found.

## SEE ALSO

 - [`zx_port_wait()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_object_wait_async()`]: object_wait_async.md
[`zx_port_wait()`]: port_wait.md
