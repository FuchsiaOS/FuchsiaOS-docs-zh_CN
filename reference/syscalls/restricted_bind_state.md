<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_restricted_bind_state

## Summary

Create and bind a restricted state VMO to the current thread.

## Declaration

```c
#include <zircon/syscalls-next.h>

zx_status_t zx_restricted_bind_state(uint32_t options, zx_handle_t* out_vmo);
```

## Description

Create a VMO to hold a `zx_restricted_state_t`.  Bind the VMO to the current
thread so that subsequent calls to [`zx_restricted_enter()`] will use it to
restore/save the restricted mode state upon entering/leaving restricted mode.

While the returned VMO, `out_vmo`, is similar to one created by
[`zx_vmo_create()`], some operations are unsupported and may fail with an error.
For example, resizing and creating a child VMO are unsupported.  Mapping,
unmapping, and reading/writing via [`zx_vmo_read()`]/[`zx_vmo_write()`] are
supported.

Only one restricted state VMO may be bound to a thread at a time.  Attempting to
bind another one will replace the already bound VMO.

A bound VMO will be destroyed only after the last user handle is closed, the
last user mapping is removed, and one of the following occur:

  - It is replaced via `zx_restricted_bind_state()`.

  - It is explicitly removed via [`zx_restricted_unbind_state()`].

  - The thread is destroyed.

Like any other VMO, once the VMO has been mapped it will be retained by its
mapping so the caller may close the handle and access the memory directly via
the mapping.

Upon entering restricted mode `zx_restricted_state_t` at offset 0 of the VMO
will be loaded and execution will resume accordingly.  Upon leaving restricted
mode, the thread's restricted state will be saved at offset 0 of VMO.

*options* must be zero.

Note: If a handle to the newly created VMO cannot be returned because `out_vmo`
is an invalid pointer, the VMO may still be bound to the thread even when the
call returns **ZX_ERR_INVALID_ARGS**.  A caller can recover from this state by
calling [`zx_restricted_unbind_state()`] or calling
[zx_restricted_bind_state()`] again with a valid `out_vmo`.

## Rights

Caller's job policy must allow **ZX_POL_NEW_VMO**.

## Errors

**ZX_ERR_INVALID_ARGS**  *out_vmo* is an invalid pointer or NULL, or *options*
is any value other than 0.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## See also

- [`zx_restricted_enter()`]

- [`zx_restricted_unbind_state()`]

[`zx_restricted_enter()`]: restricted_enter.md
[`zx_restricted_unbind_state()`]: restricted_unbind_state.md
[`zx_vmo_create()`]: vmo_create.md
[`zx_vmo_read()`]: vmo_read.md
[`zx_vmo_write()`]: vmo_write.md
