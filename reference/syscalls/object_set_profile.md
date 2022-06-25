# zx_object_set_profile

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Apply a scheduling profile to a thread.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_object_set_profile(zx_handle_t handle,
                                  zx_handle_t profile,
                                  uint32_t options);
```

## DESCRIPTION

`zx_object_set_profile()` applies a [profile] to the object specified by *target*.

The parameter *profile* is a handle to a [profile] object created with [`zx_profile_create()`].

*options* is currently ignored, and should be set to `0` by callers.

Currently, the the only supported *target* object type is [thread]. Other object types may be
supported in the future.

[profile]: reference/kernel_objects/profile.md
[thread]: reference/kernel_objects/thread.md

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_THREAD** and have **ZX_RIGHT_MANAGE_THREAD**.

*profile* must be of type **ZX_OBJ_TYPE_PROFILE** and have **ZX_RIGHT_APPLY_PROFILE**.

## RETURN VALUE

Returns **ZX_OK** on success. In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *target* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *target* is not a thread handle.

**ZX_ERR_ACCESS_DENIED**  *target* does not have **ZX_RIGHT_MANAGE_THREAD** right.

**ZX_ERR_BAD_STATE**  When *target* is a thread that is still being created, is dying, or dead, and
cannot have a *profile* applied to it.

## SEE ALSO

 - [`zx_profile_create()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_profile_create()`]: profile_create.md
