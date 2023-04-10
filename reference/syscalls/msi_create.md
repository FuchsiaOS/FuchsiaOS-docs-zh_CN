<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_msi_create

## Summary

Create an Interrupt object from a Messaged-Signaled Interrupt (MSI) allocation.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_msi_create(zx_handle_t handle,
                          uint32_t options,
                          uint32_t msi_id,
                          zx_handle_t vmo,
                          size_t vmo_offset,
                          zx_handle_t* out_interrupt);
```

## Description

`zx_msi_create()` creates an Interrupt object corresponding to a given MSI
within an MSI allocation *handle* created by [`zx_msi_allocate()`]. This
object can be used with the various interrupt syscalls in the same manner one
would use an Interrupt object returned by [`zx_interrupt_create()`]. Only one
Interrupt object can be created per *msi_id* at a time, but the same *vmo* and
*vmo_offset* is usable for multiple MSIs created from the same MSI allocation
object. This allows for a holder of the *root resource* to both allocate vectors
and MSIs from the system, as well as create Interrupt objects corresponding to
those vectors for use in device drivers.

*vmo* should be a handle referring to a physical vmo (created through
*[`zx_vmo_create_physical()`]) with the cache policy
*ZX_CACHE_POLICY_UNCACHED_DEVICE. For test purposes a contiguous
*[`zx_vmo_create_contiguous()`] vmo can be used.

*msi_id* corresponds to the vector in a given MSI allocation to create an Interrupt
object for. *msi_id* must be be between 0 and the allocation size minus 1, inclusive.

*vmo_offset* corresponds to the offset within *vmo* where the MSI capability
structure begins. Details of this structure can be found in the PCI Local Bus
Specification v3.0, section 6.8.

*options* should be 0.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_MSI**.

*vmo* must be of type **ZX_OBJ_TYPE_VMO**.

*vmo* must have **ZX_RIGHT_MAP**.

## Return value

`zx_msi_create()` returns **ZX_OK** on success. In the event of failure, a
negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE** the *handle* is invalid.

**ZX_ERR_WRONG_TYPE** *handle* is not the appropriate type.

**ZX_ERR_INVALID_ARGS** *msi_id* is not a valid MSI id within the allocation
referred to by *handle*, *vmo* is not the size of a ZX_PAGE_SIZE, *vmo* is
not physical or contiguous, *vmo* does not have cache policy set to
ZX_CACHE_POLICY_UNCACHED_DEVICE, *vmo* does not appear to contain a supported
MSI/MSI-X capability, *vmo_offset* is out of bounds of *vmo*, *vmo_offset* is
invalid for the type of capability structure found, *options* is not 0, or
*out_interrupt* is NULL.

**ZX_ERR_ALREADY_BOUND** An Interrupt object corresponding to *msi_id* already
*exists.

## See also

 - [`zx_interrupt_wait()`]
 - [`zx_msi_allocate()`]
 - [`zx_vmo_create_contiguous()`]
 - [`zx_vmo_create_physical()`]
 - [`zx_vmo_set_cache_policy()`]

[`zx_interrupt_create()`]: interrupt_create.md
[`zx_interrupt_wait()`]: interrupt_wait.md
[`zx_msi_allocate()`]: msi_allocate.md
[`zx_vmo_create_contiguous()`]: vmo_create_contiguous.md
[`zx_vmo_create_physical()`]: vmo_create_physical.md
[`zx_vmo_set_cache_policy()`]: vmo_set_cache_policy.md
