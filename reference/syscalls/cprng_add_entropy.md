<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_cprng_add_entropy

## Summary

Add entropy to the kernel CPRNG.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_cprng_add_entropy(const void* buffer, size_t buffer_size);
```

## Description

`zx_cprng_add_entropy()` mixes the given entropy into the kernel CPRNG.
a privileged operation.  It will accept at most **ZX_CPRNG_ADD_ENTROPY_MAX_LEN**
bytes of entropy at a time.

## Rights

None.

## Return value

`zx_cprng_add_entropy()` returns **ZX_OK** on success.

## Errors

**ZX_ERR_INVALID_ARGS** *buffer_size* is too large, or *buffer* is not a valid
userspace pointer.

## BUGS

This syscall should be very privileged.

## See also

 - [`zx_cprng_draw()`]

[`zx_cprng_draw()`]: cprng_draw.md
