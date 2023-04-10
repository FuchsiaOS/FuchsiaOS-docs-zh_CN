<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vcpu_enter

## Summary

Enter a VCPU, causing it to resume execution.

## Declaration

```c
#include <zircon/syscalls.h>
#include <zircon/syscalls/port.h>

zx_status_t zx_vcpu_enter(zx_handle_t handle, zx_port_packet_t* packet);
```

## Description

`zx_vcpu_enter()` begins or resumes execution of *handle*, and blocks until it
has paused execution. On pause of execution, *packet* is populated with reason
for the pause. After handling the reason, execution may be resumed by calling
`zx_vcpu_enter()` again.

If `zx_vcpu_enter()` returns **ZX_ERR_CANCELED** due to `zx_vcpu_kick()` being
called on *handle*, execution may be resumed by calling `zx_vcpu_enter()` again.

`zx_vcpu_enter()` must be called on the same thread *handle* was created on.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_VCPU** and have **ZX_RIGHT_EXECUTE**.

## Return value

`zx_vcpu_enter()` returns **ZX_OK** on success. On failure, an error value is
returned.

## Errors

**ZX_ERR_ACCESS_DENIED** *handle* does not have the **ZX_RIGHT_EXECUTE** right.

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_BAD_STATE** *handle* is in a bad state, and can not be executed.

**ZX_ERR_CANCELED** execution of *handle* was canceled, due to `zx_vcpu_kick()`
being called on *handle*.

**ZX_ERR_INTERNAL** There was an error executing *handle*.

**ZX_ERR_INVALID_ARGS** *packet* is an invalid pointer.

**ZX_ERR_NOT_SUPPORTED** An unsupported operation was encountered while
executing *handle*.

**ZX_ERR_WRONG_TYPE** *handle* is not a handle to a VCPU.

## See also

 - [`zx_guest_create()`]
 - [`zx_guest_set_trap()`]
 - [`zx_vcpu_create()`]
 - [`zx_vcpu_interrupt()`]
 - [`zx_vcpu_kick()`]
 - [`zx_vcpu_read_state()`]
 - [`zx_vcpu_write_state()`]

[`zx_guest_create()`]: guest_create.md
[`zx_guest_set_trap()`]: guest_set_trap.md
[`zx_vcpu_create()`]: vcpu_create.md
[`zx_vcpu_interrupt()`]: vcpu_interrupt.md
[`zx_vcpu_kick()`]: vcpu_kick.md
[`zx_vcpu_read_state()`]: vcpu_read_state.md
[`zx_vcpu_write_state()`]: vcpu_write_state.md
