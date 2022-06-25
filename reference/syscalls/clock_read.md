# zx_clock_read

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Perform a basic read of the clock.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_clock_read(zx_handle_t handle, zx_time_t* now);
```

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_CLOCK** and have **ZX_RIGHT_READ**.

## DESCRIPTION

Perform a basic read of the clock object and return its current time in the
*now* out parameter.

## RETURN VALUE

On success, returns **ZX_OK** along with the clock's current time in the *now* output parameter.

## ERRORS

 - **ZX_ERR_BAD_HANDLE** : *handle* is either an invalid handle, or a handle to
   an object type that is not **ZX_OBJ_TYPE_CLOCK**.
 - **ZX_ERR_ACCESS_DENIED** : *handle* lacks the **ZX_RIGHT_READ** right.

## SEE ALSO

 - [clocks]
 - [`zx_clock_create()`]
 - [`zx_clock_get_details()`]
 - [`zx_clock_update()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[clocks]: reference/kernel_objects/clock.md
[`zx_clock_create()`]: clock_create.md
[`zx_clock_get_details()`]: clock_get_details.md
[`zx_clock_update()`]: clock_update.md
