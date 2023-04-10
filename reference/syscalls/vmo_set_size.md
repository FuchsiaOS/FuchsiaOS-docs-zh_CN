<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_vmo_set_size

## SUMMARY

Resize a VMO object.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_set_size(zx_handle_t handle, uint64_t size);
```

## Description

`zx_vmo_set_size()` sets the new size of a [virtual memory
object](/reference/kernel_objects/vm_object.md) (VMO).

The size will be rounded up to the next page size boundary.
Subsequent calls to [`zx_vmo_get_size()`] will return the rounded up size.

The content size of the VMO will be set to the given (unrounded) size.
Use [`zx_object_get_property()`] with **ZX_PROP_VMO_CONTENT_SIZE** to read the
content size of the VMO. Use [`zx_object_set_property()`] with
**ZX_PROP_VMO_CONTENT_SIZE** to set the content size of the VMO without
actually resizing the VMO.

The data in the VMO between the given size and the end of the VMO (i.e., the next page boundary)
will be overwritten with zeros.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_WRITE** and **ZX_RIGHT_RESIZE**.

## Return value

`zx_vmo_set_size()` returns **ZX_OK** on success. In the event
of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a VMO handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the **ZX_RIGHT_WRITE** or **ZX_RIGHT_RESIZE** right.

**ZX_ERR_UNAVAILABLE** The VMO was not created with **ZX_VMO_RESIZABLE**
or **ZX_VMO_CHILD_RESIZABLE**.

**ZX_ERR_OUT_OF_RANGE**  Requested size is too large.

**ZX_ERR_NO_MEMORY**  Failure due to lack of system memory.

**ZX_ERR_BAD_STATE**  Requested size would discard pinned pages.

## See also

 - [`zx_vmo_create()`]
 - [`zx_vmo_create_child()`]
 - [`zx_vmo_get_size()`]
 - [`zx_vmo_op_range()`]
 - [`zx_vmo_read()`]
 - [`zx_vmo_write()`]

[`zx_object_get_property()`]: object_get_property.md
[`zx_object_set_property()`]: object_set_property.md
[`zx_vmo_create()`]: vmo_create.md
[`zx_vmo_create_child()`]: vmo_create_child.md
[`zx_vmo_get_size()`]: vmo_get_size.md
[`zx_vmo_op_range()`]: vmo_op_range.md
[`zx_vmo_read()`]: vmo_read.md
[`zx_vmo_write()`]: vmo_write.md
