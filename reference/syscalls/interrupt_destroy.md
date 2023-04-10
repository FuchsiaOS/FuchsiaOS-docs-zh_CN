<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_interrupt_destroy

## Summary

Destroys an interrupt object.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_interrupt_destroy(zx_handle_t handle);
```

## Description

`zx_interrupt_destroy()` "destroys" an interrupt object, putting it in a state
where any [`zx_interrupt_wait()`] operations on it will return **ZX_ERR_CANCELED**,
and it is unbound from any ports it was bound to.

This provides a clean shut down mechanism.  Closing the last handle to the
interrupt object results in similar cancellation but could result in use-after-close
of the handle.

If the interrupt object is bound to a port when cancellation happens, if it
has not yet triggered, or it has triggered but the packet has not yet been
received by a caller of [`zx_port_wait()`], success is returned and any packets
in flight are removed.  Otherwise, **ZX_ERR_NOT_FOUND** is returned, indicating
that the packet has been read but the interrupt has not been re-armed by calling
[`zx_interrupt_ack()`].

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_interrupt_destroy()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_WRONG_TYPE** *handle* is not an interrupt object.

**ZX_ERR_NOT_FOUND**  *handle* was bound (and now no longer is) but was not
being waited for.

**ZX_ERR_ACCESS_DENIED** *handle* lacks **ZX_RIGHT_WRITE**.

## See also

 - [`zx_handle_close()`]
 - [`zx_interrupt_ack()`]
 - [`zx_interrupt_bind()`]
 - [`zx_interrupt_create()`]
 - [`zx_interrupt_trigger()`]
 - [`zx_interrupt_wait()`]
 - [`zx_port_wait()`]

[`zx_handle_close()`]: handle_close.md
[`zx_interrupt_ack()`]: interrupt_ack.md
[`zx_interrupt_bind()`]: interrupt_bind.md
[`zx_interrupt_create()`]: interrupt_create.md
[`zx_interrupt_trigger()`]: interrupt_trigger.md
[`zx_interrupt_wait()`]: interrupt_wait.md
[`zx_port_wait()`]: port_wait.md
