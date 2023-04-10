<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vmar_protect

## Summary

Set protection of virtual memory pages.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmar_protect(zx_handle_t handle,
                            zx_vm_option_t options,
                            zx_vaddr_t addr,
                            size_t len);
```

## Description

`zx_vmar_protect()` alters the access protections for the memory mappings
in the range of *len* bytes starting from *addr*. The *options* argument should
be a bitwise-or of one or more of the following:

- **ZX_VM_PERM_READ**  Map as readable.  It is an error if *handle*
  does not have **ZX_VM_CAN_MAP_READ** permissions or *handle* does
  not have the **ZX_RIGHT_READ** right.  It is also an error if the VMO handle
  used to create the mapping did not have the **ZX_RIGHT_READ** right.
- **ZX_VM_PERM_WRITE**  Map as writable.  It is an error if *handle*
  does not have **ZX_VM_CAN_MAP_WRITE** permissions or *handle* does
  not have the **ZX_RIGHT_WRITE** right.  It is also an error if the VMO handle
  used to create the mapping did not have the **ZX_RIGHT_WRITE** right.
- **ZX_VM_PERM_EXECUTE**  Map as executable.  It is an error if *handle*
  does not have **ZX_VM_CAN_MAP_EXECUTE** permissions or *handle* does
  not have the **ZX_RIGHT_EXECUTE** right.  It is also an error if the VMO handle
  used to create the mapping did not have the **ZX_RIGHT_EXECUTE** right.
- **ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED** Map as readable if the system does
  not support mapping execute-only pages. If the system can map execute-only
  this flag is ignored.

For any mappings in sub-regions in the requested range, their access permissions must either
be reduced, or left unchanged, by the requested change.

*len* must be page-aligned.

## Rights

If *options* & **ZX_VM_PERM_READ**, *handle* must be of type **ZX_OBJ_TYPE_VMAR** and have **ZX_RIGHT_READ**.

If *options* & **ZX_VM_PERM_WRITE**, *handle* must be of type **ZX_OBJ_TYPE_VMAR** and have **ZX_RIGHT_WRITE**.

If *options* & **ZX_VM_PERM_EXECUTE**, *handle* must be of type **ZX_OBJ_TYPE_VMAR** and have **ZX_RIGHT_EXECUTE**.

## Return value

`zx_vmar_protect()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a VMAR handle.

**ZX_ERR_INVALID_ARGS**  *prot_flags* is an unsupported combination of flags
(e.g., **ZX_VM_PERM_WRITE** but not **ZX_VM_PERM_READ**), *addr* is
not page-aligned, *len* is 0, or some subrange of the requested range is
occupied by a subregion and *handle* did not have **ZX_RIGHT_OP_CHILDREN**.

**ZX_ERR_NOT_FOUND**  Some subrange of the requested range is not mapped.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the proper rights for the
requested change, the original VMO handle used to create the mapping did not
have the rights for the requested change, or the VMAR itself does not allow
the requested change, or there is a mapping in a sub-region that would have
its mapping permissions increased.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## Notes

For failures other than **ZX_ERR_NO_MEMORY**, all access permissions in the range
will have been left unchanged.

## See also

 - [`zx_vmar_allocate()`]
 - [`zx_vmar_destroy()`]
 - [`zx_vmar_map()`]
 - [`zx_vmar_unmap()`]

[`zx_vmar_allocate()`]: vmar_allocate.md
[`zx_vmar_destroy()`]: vmar_destroy.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmar_unmap()`]: vmar_unmap.md
