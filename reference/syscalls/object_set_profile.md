# zx_object_set_profile

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Apply a scheduling profile to a thread.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_object_set_profile(zx_handle_t handle,
                                  zx_handle_t profile,
                                  uint32_t options);
```

## DESCRIPTION

`zx_object_set_profile()` applies an already created [profile] to the thread
specified in *handle*.

The parameter *profile* is a handle to a [profile] object created with
[`zx_profile_create()`].

*options* is currently ignored, and should be set to `0` by callers.

[profile]: /docs/reference/kernel_objects/profile.md

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_THREAD** and have **ZX_RIGHT_MANAGE_THREAD**.

*profile* must be of type **ZX_OBJ_TYPE_PROFILE** and have **ZX_RIGHT_APPLY_PROFILE**.

## RETURN VALUE

Returns **ZX_OK** on success. In the event of failure, a negative error value is
returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a thread handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_MANAGE_THREAD**
right.

**ZX_ERR_BAD_STATE**  The thread is still being created, is dying, or dead,
and cannot have a profile applied to it.

## SEE ALSO

 - [`zx_profile_create()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_profile_create()`]: profile_create.md
