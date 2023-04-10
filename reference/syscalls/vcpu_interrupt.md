<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vcpu_interrupt

## Summary

Raise an interrupt on a VCPU.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vcpu_interrupt(zx_handle_t handle, uint32_t vector);
```

## Description

`zx_vcpu_interrupt()` raises an interrupt of *vector* on *handle*, and may be
called from any thread.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_VCPU** and have **ZX_RIGHT_SIGNAL**.

## Return value

`zx_vcpu_interrupt()` returns **ZX_OK** on success. On failure, an error value is
returned.

## Errors

**ZX_ERR_ACCESS_DENIED** *handle* does not have the **ZX_RIGHT_SIGNAL** right.

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_OUT_OF_RANGE** *vector* is outside of the range interrupts supported by
the current architecture.

**ZX_ERR_WRONG_TYPE** *handle* is not a handle to a VCPU.

## See also

 - [`zx_guest_create()`]
 - [`zx_guest_set_trap()`]
 - [`zx_vcpu_create()`]
 - [`zx_vcpu_enter()`]
 - [`zx_vcpu_kick()`]
 - [`zx_vcpu_read_state()`]
 - [`zx_vcpu_write_state()`]

[`zx_guest_create()`]: guest_create.md
[`zx_guest_set_trap()`]: guest_set_trap.md
[`zx_vcpu_create()`]: vcpu_create.md
[`zx_vcpu_enter()`]: vcpu_enter.md
[`zx_vcpu_kick()`]: vcpu_kick.md
[`zx_vcpu_read_state()`]: vcpu_read_state.md
[`zx_vcpu_write_state()`]: vcpu_write_state.md
