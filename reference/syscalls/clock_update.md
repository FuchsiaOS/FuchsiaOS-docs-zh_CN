# zx_clock_update

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Make adjustments to a clock object.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_clock_update(zx_handle_t handle,
                            uint64_t options,
                            const void* args);
```

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

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
`zx_clock_update_args_v2_t` structure that they wish to adjust, then passes the
structure to the update call, setting the bits in `options` which indicate both
the explicit version of the structure (version 2), and which of these fields are
valid and should be set. Defined `options` bits are

+ **ZX_CLOCK_UPDATE_OPTION_SYNTHETIC_VALUE_VALID**
+ **ZX_CLOCK_UPDATE_OPTION_REFERENCE_VALUE_VALID**
+ **ZX_CLOCK_UPDATE_OPTION_RATE_ADJUST_VALID**
+ **ZX_CLOCK_UPDATE_OPTION_ERROR_BOUND_VALID**

The version of the structure is passed using the `ZX_CLOCK_ARGS_VERSION(...)`
macro, specifically `ZX_CLOCK_ARGS_VERSION(2)` for the version 2 structure.

For example

```c
#include <zircon/syscalls.h>
#include <zircon/syscalls/clock.h>

void MaintainMyClock(zx_handle_t the_clock) {
  zx_clock_update_args_v2_t args;
  zx_status_t status;

  // Set the clock's value to 1500. Note that this also starts the clock.
  args.synthetic_value = 1500;
  status = zx_clock_update(the_clock,
                           ZX_CLOCK_ARGS_VERSION(2) | ZX_CLOCK_UPDATE_OPTION_SYNTHETIC_VALUE_VALID,
                           &args);
  if (status != ZX_OK) {
    // Panic!
    return;
  }

  // Make the clock run 23 PPM slower than nominal relative to clock monotonic.
  args.rate_adjust = -23;
  status = zx_clock_update(the_clock,
                           ZX_CLOCK_ARGS_VERSION(2) | ZX_CLOCK_UPDATE_OPTION_RATE_ADJUST_VALID,
                           &args);
  if (status != ZX_OK) {
    // Halt and catch fire
    return;
  }

  // Set the clock to 100,000, make it run 50 PPM faster than nominal, and specify an error bound of
  // +/- 400mSec, all at the same time.
  const uint64_t options = ZX_CLOCK_ARGS_VERSION(2) |
                           ZX_CLOCK_UPDATE_OPTION_SYNTHETIC_VALUE_VALID |
                           ZX_CLOCK_UPDATE_OPTION_RATE_ADJUST_VALID |
                           ZX_CLOCK_UPDATE_OPTION_ERROR_BOUND_VALID;
  args.synthetic_value = 100000;
  args.rate_adjust = 50;
  args.error_bound = ZX_MSEC(400);
  status = zx_clock_update(the_clock, options, &args);
  if (status != ZX_OK) {
    // Burn down, fall over, and then sink into the swamp.
    return;
  }
}
```

### Explicitly provided reference times. {#explicit-reference-time}

With the addition of the V2 update structure, it is now possible (with some
limitations) to explicitly control the reference time used for a clock update
operation. Note that, upon success, the actual new reference <-> synthetic
transformation specified by the user's update arguments will replace the old
transformation during the call to `zx_clock_update`. Supplying an explicit
reference time does _not_ affect when the actual transformation is updated, it
will always take effect during the call to `zx_clock_update`.

Diagrams provided in
[RFC-0077](contribute/governance/rfcs/0077_zx_clock_update_accuracy.md)
may help to understand the effects of the operations described below.

#### Synthetic value updates with an explicitly provided reference time.

When users update the synthetic value (`S`) of a clock with an explicitly
provided reference time (`R`), they are specifying a point (`[R, S]`) through
which the new transformation will pass. In other words, the new transformation
explicitly specifies that "when it is time R on the reference timeline, it will
be time S on the synthetic timeline".

```c
#include <zircon/syscalls.h>
#include <zircon/syscalls/clock.h>

// Set the syntheic value of the clock to be "synth" at the explicitly provided
// reference time "ref". In other words, upon success, this update operation will
// cause the clock's transformation from reference to synthetic time to
// specifically pass through the point (ref, synth)
zx_status_t SetSynthAtRef(zx_handle_t the_clock, zx_time_t ref, zx_time_t synth) {
  zx_clock_update_args_v2_t args;

  uint64_t options = ZX_CLOCK_ARGS_VERSION(2) |
                     ZX_CLOCK_UPDATE_OPTION_REFERENCE_VALUE_VALID |
                     ZX_CLOCK_UPDATE_OPTION_SYNTHETIC_VALUE_VALID;

  // Note that these options are equivalent, they just use a shorthand to
  // specify that both values are valid.
  options = ZX_CLOCK_ARGS_VERSION(2) | ZX_CLOCK_UPDATE_OPTION_BOTH_VALUES_VALID;

  args.reference_value = ref;
  args.synthetic_value = synth;
  return zx_clock_update(the_clock, options, args);
}
```

#### Rate adjustment updates with an explicitly provided reference time.

Let `T(R)` be the function which transforms a reference time `R` to a synthetic
time for a clock before an update operation. When users adjust the rate of a
clock with an explicitly provided reference time (`R`), they are specifying the
slope of the new transformation `T'(R)` for the clock, such that `T'(R) = T(R)`.
In other words, at reference time `R`, the new transformation will pass through
the same synthetic time that the old transformation did, but with a different
slope.

```c
#include <zircon/syscalls.h>
#include <zircon/syscalls/clock.h>

zx_status_t SetRateAtRef(zx_handle_t the_clock, zx_time_t ref, int32_t ppm_adj) {
  zx_clock_update_args_v2_t args;

  const uint64_t options = ZX_CLOCK_ARGS_VERSION(2) |
                           ZX_CLOCK_UPDATE_OPTION_REFERENCE_VALUE_VALID |
                           ZX_CLOCK_UPDATE_OPTION_RATE_ADJUST_VALID;
  args.reference_value = ref;
  args.rate_adjust = ppm_adj;

  return zx_clock_update(the_clock, options, args);
}
```

#### Notes, rules, and limitations.

 - Explicit reference values are _not required_. It is still possible to omit
   the reference value during an update operation. The update operation will
   simply use the current reference time when the operation is processed.
 - When providing an explicit reference value for a clock update operation,
   either a synthetic value, or a rate adjustment, or both, must also be
   provided. It is not legal to attempt to update only the error bounds at an
   explicit reference value.
 - Explicitly providing a reference value to an update operation for a continuous
   clock is never allowed as it would virtually always imply a discontinuity.
 - Explicitly providing a reference value to an update operation for a monotonic
   clock _is_ allowed, but _only_ if the behavior of the clock remains monotonic
   after the update.
 - An explicitly provided reference value during an update operation which would
   cause a read performed at a reference time of "now" to violate the configured
   backstop time of the clock will cause the operation to be rejected.
 - When updating a monotonic clock, it is not possible to effect both a
   synthetic value update and a rate adjustment simultaneously.

Details provided in
[RFC-0077](contribute/governance/rfcs/0077_zx_clock_update_accuracy.md)
may help to understand the reasoning behind some of these rules and limitations.

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

 - [RFC-0077](contribute/governance/rfcs/0077_zx_clock_update_accuracy.md)

 - [clock transformations]
 - [clocks]
 - [`zx_clock_create()`]
 - [`zx_clock_get_details()`]
 - [`zx_clock_read()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[clock transformations]: concepts/kernel/clock_transformations.md
[clocks]: reference/kernel_objects/clock.md
[`zx_clock_create()`]: clock_create.md
[`zx_clock_get_details()`]: clock_get_details.md
[`zx_clock_read()`]: clock_read.md
