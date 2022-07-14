# zx_futex_wait

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Wait on a futex.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_futex_wait(const zx_futex_t* value_ptr,
                          zx_futex_t current_value,
                          zx_handle_t new_futex_owner,
                          zx_time_t deadline);
```

## DESCRIPTION

`zx_futex_wait()` atomically verifies that *value_ptr* still contains the value
*current_value* and sleeps until the futex is made available by a call to
`zx_futex_wake`. Optionally, the thread can also be woken up after the
*deadline* (with respect to **ZX_CLOCK_MONOTONIC**) passes. *deadline* may be
automatically adjusted according to the job's [timer slack] policy.

## SPURIOUS WAKEUPS

A component that uses futexes should be prepared to handle spurious
wakeups.  A spurious wakeup is a situation where `zx_futex_wait()`
returns successfully even though the component did not wake the waiter
by calling [`zx_futex_wake()`].

Zircon's implementation of futexes currently does not generate
spurious wakeups itself.  However, commonly-used algorithms that use
futexes can sometimes generate spurious wakeups.  For example, the
usual implementation of `mutex_unlock` can potentially produce a
[`zx_futex_wake()`] call on a memory location after the location has been
freed and reused for unrelated purposes.

## OWNERSHIP

A successful call to `zx_futex_wait()` results in the owner of the futex being
set to the thread referenced by the *new_futex_owner* handle, or to nothing if
*new_futex_owner* is **ZX_HANDLE_INVALID**.

See *Ownership and Priority Inheritance* in [futex](/reference/kernel_objects/futex.md) for
details.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

None.

## RETURN VALUE

`zx_futex_wait()` returns **ZX_OK** on success.

## ERRORS

**ZX_ERR_INVALID_ARGS**  One of the following is true:

+ *value_ptr* is not a valid userspace pointer
+ *value_ptr* is not aligned to a `sizeof(zx_futex_t)` boundary.
+ *new_futex_owner* is currently a member of the waiters for *value_ptr*.
+ *new_futex_owner* has not been started yet.

**ZX_ERR_BAD_HANDLE**  *new_futex_owner* is not **ZX_HANDLE_INVALID**, and not a valid handle AND
*current_value* matches the value at *value_ptr*

**ZX_ERR_WRONG_TYPE**  *new_futex_owner* is a valid handle, but is not a handle to a thread.

**ZX_ERR_BAD_STATE**  *current_value* does not match the value at *value_ptr*.

**ZX_ERR_TIMED_OUT**  The thread was not woken before *deadline* passed.

## SEE ALSO

 - [futex objects]
 - [timer slack]
 - [`zx_futex_requeue()`]
 - [`zx_futex_wake()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[futex objects]: /reference/kernel_objects/futex.md
[timer slack]: /concepts/kernel/timer_slack.md
[`zx_futex_requeue()`]: futex_requeue.md
[`zx_futex_wake()`]: futex_wake.md
