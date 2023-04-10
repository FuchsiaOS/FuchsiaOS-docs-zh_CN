<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_pmt_unpin

## Summary

Unpin pages and revoke device access to them.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_pmt_unpin(zx_handle_t handle);
```

## Description

`zx_pmt_unpin()` unpins pages that were previously pinned by [`zx_bti_pin()`],
and revokes the access that was granted by the pin call.

Always consumes *handle*. It is invalid to use *handle* afterwards, including
to call [`zx_handle_close()`] on it.

## Rights

TODO(fxbug.dev/32253)

## Return value

On success, `zx_pmt_unpin()` returns **ZX_OK**.
In the event of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a PMT handle.

## See also

 - [`zx_bti_create()`]
 - [`zx_bti_pin()`]
 - [`zx_bti_release_quarantine()`]

[`zx_bti_create()`]: bti_create.md
[`zx_bti_pin()`]: bti_pin.md
[`zx_bti_release_quarantine()`]: bti_release_quarantine.md
[`zx_handle_close()`]: handle_close.md
