# zx_clock_update

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Make adjustments to a clock object.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_clock_update(zx_handle_t handle,
                            uint64_t options,
                            const void* args);
```

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_CLOCK** and have **ZX_RIGHT_WRITE**.

## DESCRIPTION

Three different parameters may be dynamically controlled by a clock maintainer.
They are

+ The clock's current value.
+ The clock's rate adjustment, expressed in PPM deviation from nominal.
+ The clock's current estimated error bounds.

When a clock maintainer wishes to change one or more of these parameters, they
may do so using the `zx_clock_update` syscall. Updating a clock's parameters is
an atomic operation from the perspective of all other users in the system.

The first update operation performed by a clock maintainer must include a valid
value. This update is the update that starts the clock and defines its initial
value. Before this update operation has succeeded, the **ZX_CLOCK_STARTED**
signal will be de-asserted, and afterwards it will be asserted and remain so for
the lifetime of the clock.

In order to update a clock, a user fills out the fields of a
`zx_clock_update_args_v1_t` structure that they wish to adjust, then passes the
structure to the update call, setting the bits in `options` that indicate which
of these fields are valid and should be set. Defined `options` bits are

+ **ZX_CLOCK_UPDATE_OPTION_VALUE_VALID**
+ **ZX_CLOCK_UPDATE_OPTION_RATE_ADJUST_VALID**
+ **ZX_CLOCK_UPDATE_OPTION_ERROR_BOUND_VALID**

In addition, maintainer **must** indicate that they are using the V1 version of
the struct using the ZX_CLOCK_ARGS_VERSION(...) macro.

For example

```c
#include <zircon/syscalls.h>
#include <zircon/syscalls/clock.h>

void MaintainMyClock(zx_handle_t the_clock) {
  zx_clock_update_args_v1_t args;
  zx_handle_t the_clock;
  zx_status_t status;

  // Set the clock's value to 1500. Note that this also starts the clock.
  args.value = 1500;
  status = zx_clock_update(the_clock,
                           ZX_CLOCK_ARGS_VERSION(1) | ZX_CLOCK_UPDATE_OPTION_VALUE_VALID,
                           &args);
  if (status != ZX_OK) {
    // Panic!
    return;
  }

  // Make the clock run 23 PPM slower than nominal relative to clock monotonic.
  args.rate_adjust = -23;
  status = zx_clock_update(the_clock,
                           ZX_CLOCK_ARGS_VERSION(1) | ZX_CLOCK_UPDATE_OPTION_RATE_ADJUST_VALID,
                           &args);
  if (status != ZX_OK) {
    // Halt and catch fire
    return;
  }

  // Set the clock to 100,000, make it run 50 PPM faster than nominal, and specify an error bound of
  // +/- 400mSec, all at the same time.
  const uint64_t options = ZX_CLOCK_ARGS_VERSION(1) |
                           ZX_CLOCK_UPDATE_OPTION_VALUE_VALID |
                           ZX_CLOCK_UPDATE_OPTION_RATE_ADJUST_VALID |
                           ZX_CLOCK_UPDATE_OPTION_ERROR_BOUND_VALID;
  args.value = 100000;
  args.rate_adjust = 50;
  args.error_bound = ZX_MSEC(400);
  status = zx_clock_update(the_clock, options, &args);
  if (status != ZX_OK) {
    // Burn down, fall over, and then sink into the swamp.
    return;
  }
}
```

## RETURN VALUE

On success, returns **ZX_OK**.

## ERRORS

 - **ZX_ERR_BAD_HANDLE** : *handle* is either an invalid handle, or a handle to
   an object type that is not **ZX_OBJ_TYPE_CLOCK**.
 - **ZX_ERR_ACCESS_DENIED** : *handle* lacks the **ZX_RIGHT_WRITE** right.
 - **ZX_ERR_INVALID_ARGS** : The update request made is incompatible with the
   properties of the clock. See the **DESCRIPTION** section for details of
   permissible clock update operations. Otherwise, the version/pointer of
   the arguments structure is incorrect.

## SEE ALSO

 - [clocks]
 - [`zx_clock_create()`]
 - [`zx_clock_get_details()`]
 - [`zx_clock_read()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[clocks]: /docs/reference/kernel_objects/clock.md
[`zx_clock_create()`]: clock_create.md
[`zx_clock_get_details()`]: clock_get_details.md
[`zx_clock_read()`]: clock_read.md
