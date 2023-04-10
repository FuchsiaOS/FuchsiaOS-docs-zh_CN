<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vcpu_create

## Summary

Create a VCPU.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vcpu_create(zx_handle_t guest,
                           uint32_t options,
                           zx_vaddr_t entry,
                           zx_handle_t* out);
```

## Description

`zx_vcpu_create()` creates a VCPU within a guest, which allows for execution
within the virtual machine. One or more VCPUs may be created per guest, where
the number of VCPUs does not need to match the number of physical CPUs on the
machine.

*entry* is the instruction pointer used to indicate where in guest physical
memory execution of the VCPU should start.

*out* is bound to the thread that created it, and all syscalls that operate on
it must be called from the same thread, with the exception of
[`zx_vcpu_interrupt()`].

Only one VCPU may exist on a thread at a time. A thread can create another VCPU
after it has closed the existing one.

N.B. VCPU is an abbreviation of virtual CPU.

The following rights will be set on the handle *out* by default:

**ZX_RIGHT_DUPLICATE** &mdash; *out* may be duplicated.

**ZX_RIGHT_TRANSFER** &mdash; *out* may be transferred over a channel.

**ZX_RIGHT_EXECUTE** &mdash; *out* may have its execution resumed (or begun)

**ZX_RIGHT_SIGNAL** &mdash; *out* may be interrupted

**ZX_RIGHT_READ** &mdash; *out* may have its state read

**ZX_RIGHT_WRITE** &mdash; may have its state written

## Rights

*guest* must be of type **ZX_OBJ_TYPE_GUEST** and have **ZX_RIGHT_MANAGE_THREAD**.

## Return value

`zx_vcpu_create()` returns **ZX_OK** on success. On failure, an error value is
returned.

## Errors

**ZX_ERR_ACCESS_DENIED** *guest* does not have the **ZX_RIGHT_MANAGE_THREAD**
right.

**ZX_ERR_BAD_HANDLE** *guest* is an invalid handle.

**ZX_ERR_BAD_STATE** The thread currently has a VCPU. Only one VCPU can be
active on a thread at a time.

**ZX_ERR_INVALID_ARGS** *args* contains an invalid argument, or *out* is an
invalid pointer, or *options* is nonzero.

**ZX_ERR_NO_MEMORY** Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_WRONG_TYPE** *guest* is not a handle to a guest.

## See also

 - [`zx_guest_create()`]
 - [`zx_guest_set_trap()`]
 - [`zx_vcpu_enter()`]
 - [`zx_vcpu_interrupt()`]
 - [`zx_vcpu_kick()`]
 - [`zx_vcpu_read_state()`]
 - [`zx_vcpu_write_state()`]

[`zx_guest_create()`]: guest_create.md
[`zx_guest_set_trap()`]: guest_set_trap.md
[`zx_vcpu_enter()`]: vcpu_enter.md
[`zx_vcpu_interrupt()`]: vcpu_interrupt.md
[`zx_vcpu_kick()`]: vcpu_kick.md
[`zx_vcpu_read_state()`]: vcpu_read_state.md
[`zx_vcpu_write_state()`]: vcpu_write_state.md
