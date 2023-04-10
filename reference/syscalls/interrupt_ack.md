<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_interrupt_ack

## Summary

Acknowledge an interrupt and re-arm it.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_interrupt_ack(zx_handle_t handle);
```

## Description

`zx_interrupt_ack()` acknowledges an interrupt object, causing it to be eligible
to trigger again (and delivering a packet to the port it is bound to).

If the interrupt object is a physical interrupt, if it is a level interrupt and
still asserted, or is an edge interrupt that has been asserted since it last
triggered, the interrupt will trigger immediately, delivering a packet to the
port it is bound to.

Virtual interrupts behave as edge interrupts.

This syscall only operates on interrupts bound to a port.  Interrupts
being waited upon with [`zx_interrupt_wait()`] do not need to be re-armed with this
call -- it happens automatically when [`zx_interrupt_wait()`] is called.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_INTERRUPT** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_interrupt_ack()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_WRONG_TYPE** *handle* is not an interrupt object.

**ZX_ERR_BAD_STATE** *handle* is not bound to a port.

**ZX_ERR_CANCELED**  [`zx_interrupt_destroy()`] was called on *handle*.

**ZX_ERR_ACCESS_DENIED** *handle* lacks **ZX_RIGHT_WRITE**.

## See also

 - [`zx_handle_close()`]
 - [`zx_interrupt_bind()`]
 - [`zx_interrupt_create()`]
 - [`zx_interrupt_destroy()`]
 - [`zx_interrupt_trigger()`]
 - [`zx_interrupt_wait()`]
 - [`zx_port_wait()`]

[`zx_handle_close()`]: handle_close.md
[`zx_interrupt_bind()`]: interrupt_bind.md
[`zx_interrupt_create()`]: interrupt_create.md
[`zx_interrupt_destroy()`]: interrupt_destroy.md
[`zx_interrupt_trigger()`]: interrupt_trigger.md
[`zx_interrupt_wait()`]: interrupt_wait.md
[`zx_port_wait()`]: port_wait.md
