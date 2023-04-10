<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_bti_release_quarantine

## Summary

Releases all quarantined PMTs.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_bti_release_quarantine(zx_handle_t handle);
```

## Description

`zx_bti_release_quarantine()` releases all quarantined PMTs for the given BTI.
This will release the PMTs' underlying references to VMOs and physical page
pins.  The underlying physical pages may be eligible to be reallocated
afterwards.

## Rights

*handle* must be of type **ZX_OBJ_TYPE_BTI** and have **ZX_RIGHT_WRITE**.

## Return value

`zx_bti_release_quarantine()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a BTI handle.

**ZX_ERR_ACCESS_DENIED** *handle* does not have the **ZX_RIGHT_WRITE** right.

## See also

 - [`zx_bti_pin()`]
 - [`zx_pmt_unpin()`]

[`zx_bti_pin()`]: bti_pin.md
[`zx_pmt_unpin()`]: pmt_unpin.md
