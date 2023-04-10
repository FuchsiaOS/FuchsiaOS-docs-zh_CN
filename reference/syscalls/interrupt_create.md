<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_interrupt_create

## Summary

Create an interrupt object.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_interrupt_create(zx_handle_t src_obj,
                                uint32_t src_num,
                                uint32_t options,
                                zx_handle_t* out_handle);
```

## Description

`zx_interrupt_create()` creates an interrupt object that represents a physical
or virtual interrupt.

If *options* is **ZX_INTERRUPT_VIRTUAL**, *src_obj* and *src_num* are ignored and
a virtual interrupt is returned.

Otherwise *src_obj* must be a suitable resource for creating platform interrupts
or a PCI object, and *src_num* is the associated interrupt number.  This restricts
the creation of interrupts to the internals of the DDK (driver development kit).
Physical interrupts are obtained by drivers through various DDK APIs.

Physical interrupts honor the options **ZX_INTERRUPT_EDGE_LOW**, **ZX_INTERRUPT_EDGE_HIGH**,
**ZX_INTERRUPT_LEVEL_LOW**, **ZX_INTERRUPT_LEVEL_HIGH**, and **ZX_INTERRUPT_REMAP_IRQ**.

The handles will have **ZX_RIGHT_INSPECT**, **ZX_RIGHT_DUPLICATE**, **ZX_RIGHT_TRANSFER**
(allowing them to be sent to another process via [`zx_channel_write()`]), **ZX_RIGHT_READ**,
**ZX_RIGHT_WRITE** (required for [`zx_interrupt_ack()`]), **ZX_RIGHT_WAIT** (required for
[`zx_interrupt_wait()`], and **ZX_RIGHT_SIGNAL** (required for [`zx_interrupt_trigger()`]).

Interrupts are said to be "triggered" when the underlying physical interrupt occurs
or when [`zx_interrupt_trigger()`] is called on a virtual interrupt.  A triggered interrupt,
when bound to a port with [`zx_interrupt_bind()`], causes a packet to be delivered to the port.

If not bound to a port, an interrupt object may be waited on with [`zx_interrupt_wait()`].

Interrupts cannot be waited on with the `zx_object_wait_` family of calls.

## Rights

*src_obj* must have resource kind **ZX_RSRC_KIND_IRQ**.

## Return value

`zx_interrupt_create()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE** the *src_obj* handle is invalid (if this is not a virtual interrupt)

**ZX_ERR_WRONG_TYPE** the *src_obj* handle is not of an appropriate type to create an interrupt.

**ZX_ERR_ACCESS_DENIED** the *src_obj* handle does not allow this operation.

**ZX_ERR_INVALID_ARGS** *options* contains invalid flags or the *out_handle*
parameter is an invalid pointer.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## See also

 - [`zx_handle_close()`]
 - [`zx_interrupt_ack()`]
 - [`zx_interrupt_bind()`]
 - [`zx_interrupt_destroy()`]
 - [`zx_interrupt_wait()`]
 - [`zx_port_wait()`]

[`zx_channel_write()`]: channel_write.md
[`zx_handle_close()`]: handle_close.md
[`zx_interrupt_ack()`]: interrupt_ack.md
[`zx_interrupt_bind()`]: interrupt_bind.md
[`zx_interrupt_destroy()`]: interrupt_destroy.md
[`zx_interrupt_trigger()`]: interrupt_trigger.md
[`zx_interrupt_wait()`]: interrupt_wait.md
[`zx_port_wait()`]: port_wait.md
