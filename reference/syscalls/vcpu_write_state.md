<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vcpu_write_state

## Summary

Write the state of a VCPU.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vcpu_write_state(zx_handle_t handle,
                                uint32_t kind,
                                const void* buffer,
                                size_t buffer_size);
```

## Description

`zx_vcpu_write_state()` writes the state of *handle* as specified by *kind* from
*buffer*. It is only valid to write the state of *handle* when execution has
been paused.

*kind* may be **ZX_VCPU_STATE** or **ZX_VCPU_IO**.

`zx_vcpu_write_state()` must be called on the same thread *handle* was created
on.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_VCPU** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_vcpu_write_state()` returns **ZX_OK** on success. On failure, an error value
is returned.

## Errors

**ZX_ERR_ACCESS_DENIED** *handle* does not have the **ZX_RIGHT_WRITE** right.

**ZX_ERR_BAD_HANDLE** *handle* is an invalid handle.

**ZX_ERR_BAD_STATE** *handle* is in a bad state, and state can not be written.

**ZX_ERR_INVALID_ARGS** *kind* does not name a known VCPU state, *buffer* is an
invalid pointer, or *buffer_size* does not match the expected size of *kind*.

**ZX_ERR_WRONG_TYPE** *handle* is not a handle to a VCPU.

## See also

 - [`zx_guest_create()`]
 - [`zx_guest_set_trap()`]
 - [`zx_vcpu_create()`]
 - [`zx_vcpu_enter()`]
 - [`zx_vcpu_interrupt()`]
 - [`zx_vcpu_kick()`]
 - [`zx_vcpu_read_state()`]

[`zx_guest_create()`]: guest_create.md
[`zx_guest_set_trap()`]: guest_set_trap.md
[`zx_vcpu_create()`]: vcpu_create.md
[`zx_vcpu_enter()`]: vcpu_enter.md
[`zx_vcpu_interrupt()`]: vcpu_interrupt.md
[`zx_vcpu_kick()`]: vcpu_kick.md
[`zx_vcpu_read_state()`]: vcpu_read_state.md
