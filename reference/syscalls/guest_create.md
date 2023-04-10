<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_guest_create

## Summary

Create a guest.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_guest_create(zx_handle_t resource,
                            uint32_t options,
                            zx_handle_t* guest_handle,
                            zx_handle_t* vmar_handle);
```

## Description

`zx_guest_create()` creates a guest, which is a virtual machine that can be run
within the hypervisor, with *vmar_handle* used to represent the physical address
space of the guest.

To create a guest, a *resource* of **ZX_RSRC_KIND_SYSTEM** with
**ZX_RSRC_SYSTEM_HYPERVISOR_BASE** must be supplied.

In order to begin execution within the guest, a VMO should be mapped into
*vmar_handle* using [`zx_vmar_map()`], and a VCPU must be created using
[`zx_vcpu_create()`], and then run using [`zx_vcpu_enter()`].

Additionally, a VMO should be mapped into *vmar_handle* to provide a guest with
physical memory.

The following rights will be set on the handle *guest_handle* by default:

**ZX_RIGHT_TRANSFER** &mdash; *guest_handle* may be transferred over a channel.

**ZX_RIGHT_DUPLICATE** &mdash; *guest_handle* may be duplicated.

**ZX_RIGHT_WRITE** &mdash; A trap to be may be set using [`zx_guest_set_trap()`].

**ZX_RIGHT_MANAGE_THREAD** &mdash; A VCPU may be created using [`zx_vcpu_create()`].

See [`zx_vmo_create()`] for the set of rights applied to *vmar_handle*.

## Rights

*resource* must have resource kind **ZX_RSRC_KIND_SYSTEM** with base
**ZX_RSRC_SYSTEM_HYPERVISOR_BASE**.

## Return value

`zx_guest_create()` returns **ZX_OK** on success. On failure, an error value is
returned.

## Errors

**ZX_ERR_NOT_SUPPORTED** The hypervisor is not supported by the device.

**ZX_ERR_ACCESS_DENIED** *resource* is not of kind **ZX_RSRC_KIND_SYSTEM** with
base **ZX_RSRC_SYSTEM_HYPERVISOR_BASE**.

**ZX_ERR_INVALID_ARGS** *guest_handle* or *vmar_handle* is an invalid pointer,
or *options* is nonzero.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_WRONG_TYPE** *resource* is not a handle to a resource.

## See also

 - [`zx_guest_set_trap()`]
 - [`zx_vcpu_create()`]
 - [`zx_vcpu_enter()`]
 - [`zx_vcpu_interrupt()`]
 - [`zx_vcpu_kick()`]
 - [`zx_vcpu_read_state()`]
 - [`zx_vcpu_write_state()`]
 - [`zx_vmar_map()`]
 - [`zx_vmo_create()`]

[`zx_guest_set_trap()`]: guest_set_trap.md
[`zx_vcpu_create()`]: vcpu_create.md
[`zx_vcpu_enter()`]: vcpu_enter.md
[`zx_vcpu_interrupt()`]: vcpu_interrupt.md
[`zx_vcpu_kick()`]: vcpu_kick.md
[`zx_vcpu_read_state()`]: vcpu_read_state.md
[`zx_vcpu_write_state()`]: vcpu_write_state.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmo_create()`]: vmo_create.md
