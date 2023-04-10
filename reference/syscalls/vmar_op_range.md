<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vmar_op_range

## Summary

Perform an operation on VMOs mapped into this VMAR.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmar_op_range(zx_handle_t handle,
                             uint32_t op,
                             zx_vaddr_t address,
                             size_t size,
                             void* buffer,
                             size_t buffer_size);
```

## Description

`zx_vmar_op_range()` performs operation *op* on VMOs mapped in the range *address* to
*address*+*size*.

*address* and *size* must fall entirely within this VMAR, and must meet the alignment requirements
specified for the corresponding VMO *op* (if there is one) by [`zx_vmo_op_range()`].

*buffer* and *buffer_size* are currently unused, and must be empty.

The supported operations are:

**ZX_VMAR_OP_COMMIT** - Requires that *handle*, and the VMO handles used to create any affected
mappings, have the **ZX_RIGHT_WRITE** right and that the address range must not have any child VMARs.
The operation's semantics are otherwise as described by
[`zx_vmo_op_range()`](/reference/syscalls/vmo_op_range.md) **ZX_VMO_OP_COMMIT**.

**ZX_VMO_OP_DECOMMIT** - Deprecated. Use **ZX_VMAR_OP_DECOMMIT** instead.

**ZX_VMAR_OP_DECOMMIT** - Requires that *handle*, and the VMO handles used to create any affected
mappings, have the **ZX_RIGHT_WRITE** right and that the address range must not have any child VMARs.
The operation's semantics are otherwise as described by
[`zx_vmo_op_range()`](/reference/syscalls/vmo_op_range.md) **ZX_VMO_OP_DECOMMIT**.

**ZX_VMAR_OP_MAP_RANGE** - Populates entries in the CPU page tables (or architectural equivalent)
for committed pages in the given range. Entries for uncommitted pages in the range are not
populated. Skips entries that already exist for any page in the range.

**ZX_VMAR_OP_DONT_NEED** - Hints that pages in the specified range are not needed anymore and should
be considered for memory reclamation. Intended to be used on mappings for VMOs created with
[`zx_pager_create_vmo()`](/reference/syscalls/pager_create_vmo.md); trivially succeeds for
mappings for other VMO types.

Please refer to [`zx_vmo_op_range()`](/reference/syscalls/vmo_op_range.md)
**ZX_VMO_OP_DONT_NEED** for more details.

**ZX_VMAR_OP_ALWAYS_NEED** - Hints that pages in the specified range are important and should be
protected from memory reclamation. Intended to be used on mappings for VMOs created with
[`zx_pager_create_vmo()`](/reference/syscalls/pager_create_vmo.md); trivially succeeds for
mappings for other VMO types.

Please refer to [`zx_vmo_op_range()`](/reference/syscalls/vmo_op_range.md)
**ZX_VMO_OP_ALWAYS_NEED** for more details.

## Rights

If *op* is **ZX_VMAR_OP_DECOMMIT**, *handle* must have **ZX_RIGHT_WRITE**.

If *op* is **ZX_VMAR_OP_COMMIT**, *handle* must have **ZX_RIGHT_WRITE**.

## Return value

`zx_vmar_op_range()` returns **ZX_OK** on success. In the event of failure, a negative error value
is returned.

## Errors

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the proper rights for the requested change, or the
original VMO handle used to created one of the affected mappings did not have the rights for the
requested change, or the VMAR itself did not allow the requested change.

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_BAD_STATE**  *handle* is not a live VMAR, or the range specified by *address* and *size*
spans unmapped regions.

**ZX_ERR_INVALID_ARGS**  Returned for any of the following reasons:

 - *buffer* is non-null.
 - *buffer_size* is non-zero.
 - *size* is zero.
 - *address* was not page-aligned.
 - *op* is not a valid operation.
 - *op* was one of **ZX_VMAR_OP_COMMIT** or **ZX_VMAR_OP_DECOMMIT** and the requested range
   had subregions.
 - the requested range overlapped a sub-region and *handle* did not have
   **ZX_RIGHT_OP_CHILDREN**.

**ZX_ERR_NOT_SUPPORTED**  *op* was not **ZX_VMO_OP_DECOMMIT**, or one or more mapped VMOs do not
support the requested *op*.

**ZX_ERR_OUT_OF_RANGE**  The range specified by *address* and *size* is not wholly within the VM
address range specified by *handle*.

**ZX_ERR_WRONG_TYPE**  *handle* is not a VMAR handle.

## See also

 - [`zx_vmar_map()`]
 - [`zx_vmar_unmap()`]
 - [`zx_vmo_op_range()`]

[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmar_unmap()`]: vmar_unmap.md
[`zx_vmo_op_range()`]: vmo_op_range.md
