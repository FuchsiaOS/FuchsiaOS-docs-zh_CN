# zx_object_wait_one

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Wait for signals on an object.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_object_wait_one(zx_handle_t handle,
                               zx_signals_t signals,
                               zx_time_t deadline,
                               zx_signals_t* observed);
```

## DESCRIPTION

`zx_object_wait_one()` is a blocking syscall that causes the caller to wait
until either the *deadline* passes or the object to which *handle* refers
asserts at least one [signal][signals] specified by the bitmask *signals*. If
the object is already asserting at least one of the specified *signals*, the
wait ends immediately with **ZX_OK**.

Upon return, if non-NULL, *observed* is a bitmap of *all* of the signals
asserted on the object. If one of the specified *signals* was asserted,
*observed* will be the set of signals asserted at the moment the specified
signal was first asserted. Otherwise, if *deadline* passes or *handle* is
closed, *observed* will contain the state of the object's signals at the time
the `zx_object_wait_one` syscall completed.

The *deadline* parameter specifies a deadline with respect to
**ZX_CLOCK_MONOTONIC** and will be automatically adjusted according to the job's
[timer slack] policy.

  * **ZX_TIME_INFINITE** is a special value meaning wait forever.
  * 0 (or any value before the current time in **ZX_CLOCK_MONOTONIC**) will
    query the current value of the signal(s).

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must have **ZX_RIGHT_WAIT**.

## RETURN VALUE

`zx_object_wait_one()` returns **ZX_OK** if any of *signals* were active when
the call was made, or observed on the object before *deadline* passes.

In the event of **ZX_ERR_TIMED_OUT**, *observed* may reflect state changes
that occurred after the deadline passed, but before the syscall returned.

In the event of **ZX_ERR_CANCELED**, *handle* has been closed,
and *observed* will have the **ZX_SIGNAL_HANDLE_CLOSED** bit set.

For any other return value, *observed* is undefined.

## ERRORS

**ZX_ERR_INVALID_ARGS**  *observed* is an invalid pointer.

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_WAIT** and may
not be waited upon.

**ZX_ERR_CANCELED**  *handle* was invalidated (e.g., closed) during the wait.

**ZX_ERR_TIMED_OUT**  The specified deadline passed before any of the specified
*signals* are observed on *handle*.

**ZX_ERR_NOT_SUPPORTED**  *handle* is a handle that cannot be waited on
(for example, a Port handle).

## NOTES

See [signals] for more information about signals and their terminology.

## SEE ALSO

 - [signals]
 - [timer slack]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[signals]: /docs/concepts/kernel/signals.md
[timer slack]: /docs/concepts/kernel/timer_slack.md
[`zx_object_wait_async()`]: object_wait_async.md
[`zx_object_wait_many()`]: object_wait_many.md
