<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vcpu_kick

## Summary

Kick a VCPU, causing it to stop execution.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vcpu_kick(zx_handle_t handle);
```

## Description

`zx_vcpu_kick()` forces the current or next execution of `zx_vcpu_enter()` on
*handle* to return immediately with **ZX_ERR_CANCELED**.

`zx_vcpu_kick()` may be called multiple times on *handle*, but will only affect
the current or next execution of `zx_vcpu_enter()`.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_VCPU** and have **ZX_RIGHT_EXECUTE**.

## Return value

`zx_vcpu_kick()` returns **ZX_OK** on success. On failure, an error value is
returned.

## Errors

**ZX_ERR_ACCESS_DENIED** *handle* does not have the **ZX_RIGHT_EXECUTE** right.

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_WRONG_TYPE** *handle* is not a handle to a VCPU.

## See also

 - [`zx_guest_create()`]
 - [`zx_guest_set_trap()`]
 - [`zx_vcpu_create()`]
 - [`zx_vcpu_enter()`]
 - [`zx_vcpu_interrupt()`]
 - [`zx_vcpu_read_state()`]
 - [`zx_vcpu_write_state()`]

[`zx_guest_create()`]: guest_create.md
[`zx_guest_set_trap()`]: guest_set_trap.md
[`zx_vcpu_create()`]: vcpu_create.md
[`zx_vcpu_enter()`]: vcpu_enter.md
[`zx_vcpu_interrupt()`]: vcpu_interrupt.md
[`zx_vcpu_read_state()`]: vcpu_read_state.md
[`zx_vcpu_write_state()`]: vcpu_write_state.md
