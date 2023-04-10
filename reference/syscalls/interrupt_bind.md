<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_interrupt_bind

## Summary

Bind an interrupt object to a port.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_interrupt_bind(zx_handle_t handle,
                              zx_handle_t port_handle,
                              uint64_t key,
                              uint32_t options);
```

## Description

`zx_interrupt_bind()` binds or unbinds an interrupt object to a port.

An interrupt object may only be bound to a single port and may only be bound once.
The interrupt can only bind to a port created with **ZX_PORT_BIND_TO_INTERRUPT**
option.

When a bound interrupt object is triggered, a **ZX_PKT_TYPE_INTERRUPT** packet will
be delivered to the port it is bound to, with the timestamp (relative to **ZX_CLOCK_MONOTONIC**)
of when the interrupt was triggered in the `zx_packet_interrupt_t`.  The *key* used
when binding the interrupt will be present in the `key` field of the `zx_port_packet_t`.

If the interrupt being bound is in a triggered state, then a
**ZX_PKT_TYPE_INTERRUPT** packet will be sent to the port the interrupt is being bound to,
with the timestamp of when the interrupt was triggered. This packet will need
to be processed normally.

To bind to a port pass **ZX_INTERRUPT_BIND** in *options*.

To unbind a previously bound port pass **ZX_INTERRUPT_UNBIND** in *options*. For unbind the
*port_handle* is required but the *key* is ignored. Unbinding the port removes previously
queued packets to the port.

Before another packet may be delivered, the bound interrupt must be re-armed using the
[`zx_interrupt_ack()`] syscall.  This is (in almost all cases) best done after the interrupt
packet has been fully processed.  Especially in the case of multiple threads reading
packets from a port, if the processing thread re-arms the interrupt and it has triggered,
a packet will immediately be delivered to a waiting thread.

Interrupt packets are delivered via a dedicated queue on ports and are higher priority
than non-interrupt packets.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_INTERRUPT** and have **ZX_RIGHT_READ**.

*port_handle* must be of type **ZX_OBJ_TYPE_PORT** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_interrupt_bind()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE** *handle* or *port_handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *handle* is not an interrupt object or *port_handle* is not a port object.

**ZX_ERR_CANCELED**  [`zx_interrupt_destroy()`] was called on *handle*.

**ZX_ERR_BAD_STATE**  A thread is waiting on the interrupt using [`zx_interrupt_wait()`]

**ZX_ERR_ACCESS_DENIED** the *handle* handle lacks **ZX_RIGHT_READ** or the *port_handle* handle
lacks **ZX_RIGHT_WRITE**

**ZX_ERR_ALREADY_BOUND** this interrupt object is already bound.

**ZX_ERR_INVALID_ARGS** *options* is not **ZX_INTERRUPT_BIND** or **ZX_INTERRUPT_UNBIND**.

**ZX_ERR_NOT_FOUND** the *port* does not match the bound port.

## See also

 - [`zx_handle_close()`]
 - [`zx_interrupt_ack()`]
 - [`zx_interrupt_create()`]
 - [`zx_interrupt_destroy()`]
 - [`zx_interrupt_trigger()`]
 - [`zx_interrupt_wait()`]
 - [`zx_port_wait()`]

[`zx_handle_close()`]: handle_close.md
[`zx_interrupt_ack()`]: interrupt_ack.md
[`zx_interrupt_create()`]: interrupt_create.md
[`zx_interrupt_destroy()`]: interrupt_destroy.md
[`zx_interrupt_trigger()`]: interrupt_trigger.md
[`zx_interrupt_wait()`]: interrupt_wait.md
[`zx_port_wait()`]: port_wait.md
