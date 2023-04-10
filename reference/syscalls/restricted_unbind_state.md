<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_restricted_unbind_state

## Summary

Unbind a restricted state VMO from the current thread.

## Declaration

```c
#include <zircon/syscalls-next.h>

zx_status_t zx_restricted_unbind_state(uint32_t options);
```

## Description

Unbind any restricted state VMO that may be bound to the calling thread.

See also [`zx_restricted_bind_state`].

It is not an error to call unbind on a thread that has no bound VMO.

*options* must be zero.

## Rights

None.

## Errors

**ZX_ERR_INVALID_ARGS**  *options* is any value other than 0.

## See also

- [`zx_restricted_enter()`]

- [`zx_restricted_bind_state()`]

[`zx_restricted_enter()`]: restricted_enter.md
[`zx_restricted_bind_state()`]: restricted_bind_state.md
