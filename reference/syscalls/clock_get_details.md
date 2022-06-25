# zx_clock_get_details

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Fetch all of the low level details of the clock's current status.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_clock_get_details(zx_handle_t handle,
                                 uint64_t options,
                                 void* details);
```

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_CLOCK** and have **ZX_RIGHT_READ**.

## DESCRIPTION

Fetches the fine grained details of the clock object. See
[clocks](reference/kernel_objects/clock.md) for the specifics of the details
reported. Currently, there is only one details structure defined for clocks,
`zx_clock_details_v1_t`. Users must specify the version of the structure using
the options parameter as well as providing at least
`sizeof(zx_clock_details_v1_t)` bytes of storage via the `details`. For
example:

```c
#include <zircon/syscalls.h>
#include <zircon/syscalls/clock.h>

void GetSomeDetails(zx_handle_t the_clock) {
  zx_clock_details_v1_t details;
  zx_status_t status;

  status = zx_clock_get_details(the_clock, ZX_CLOCK_ARGS_VERSION(1), &details);
  if (status == ZX_OK) {
    // Do great things with our details.
  }
}
```

## RETURN VALUE

On success, returns **ZX_OK** along with clock details stored in the *details*
out parameter.

## ERRORS

 - **ZX_ERR_BAD_HANDLE** : *handle* is either an invalid handle, or a handle to
   an object type that is not **ZX_OBJ_TYPE_CLOCK**.
 - **ZX_ERR_ACCESS_DENIED** : *handle* lacks the **ZX_RIGHT_READ** right.
 - **ZX_ERR_INVALID_ARGS** : The version of the details structure signaled by
   `options` is invalid, or the pointer of the structure passed via `details` is bad.

## SEE ALSO

 - [clock transformations]
 - [clocks]
 - [`zx_clock_create()`]
 - [`zx_clock_read()`]
 - [`zx_clock_update()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[clock transformations]: concepts/kernel/clock_transformations.md
[clocks]: reference/kernel_objects/clock.md
[`zx_clock_create()`]: clock_create.md
[`zx_clock_read()`]: clock_read.md
[`zx_clock_update()`]: clock_update.md
