# Futex

## NAME

futex - A primitive for creating userspace synchronization tools.

## SYNOPSIS

A **futex** is a Fast Userspace muTEX. It is a low level
synchronization primitive that is a building block for higher level
APIs such as `pthread_mutex_t` and `pthread_cond_t`.

Futexes are designed to not enter the kernel or allocate kernel
resources in the uncontested case.

## DESCRIPTION

The zircon futex implementation currently supports three operations distributed
over 6 syscalls:

```C
    zx_status_t zx_futex_wait(const zx_futex_t* value_ptr,
                              zx_futex_t current_value,
                              zx_handle_t new_futex_owner,
                              zx_time_t deadline);
    zx_status_t zx_futex_wake(const zx_futex_t* value_ptr, uint32_t wake_count);
    zx_status_t zx_futex_wake_single_owner(const zx_futex_t* value_ptr);
    zx_status_t zx_futex_requeue(const zx_futex_t* value_ptr,
                                 uint32_t wake_count,
                                 zx_futex_t current_value,
                                 const zx_futex_t* requeue_ptr,
                                 uint32_t requeue_count,
                                 zx_handle_t new_requeue_owner);
    zx_status_t zx_futex_requeue_single_owner(const zx_futex_t* value_ptr,
                                              zx_futex_t current_value,
                                              const zx_futex_t* requeue_ptr,
                                              uint32_t requeue_count,
                                              zx_handle_t new_requeue_owner);
    zx_status_t zx_futex_get_owner(const zx_futex_t* value_ptr, uint64_t* koid);
```

All of these share a `value_ptr` parameter, which is the virtual
address of an aligned userspace integer. This virtual address is the
information used in kernel to track what futex given threads are
waiting on. The kernel does not currently modify the value of
`*value_ptr` (but see below for future operations that might do
so). It is up to userspace code to correctly atomically modify this
value across threads in order to build mutexes and so on.

Note that with [address tagging][address_tagging], userspace pointers
won't always have a 1-to-1 mapping of futex instances in the kernel. Addresses
which have been stripped of architecture-specific tagging information are used
for futex IDs. For example, on ARM where [Top-Byte-Ignore (TBI)][tbi] is
enabled, a futex pointer with the value `0x0A000000FF123450` has the same
futex ID as a futex pointer with the value `0x0B000000FF123450`, because while
their tags (`0x0A` and `0x0B`) are different, their address bits are the same.

See the [`zx_futex_wait()`], [`zx_futex_wake()`], [`zx_futex_requeue()`], and
[`zx_futex_get_owner()`] man pages for more details.

## RIGHTS

Futex objects do not have any rights associated with them.

There are only 2 primitive operations that userspace code can perform on a
futex: waiting and waking (requeue is a combination of the two).  Because
futexes are strictly a process local concept, revoking access to either of these
operations would make the futex functionally worthless.

Additionally, from the kernel's perspective, futexes are ephemeral objects whose
state only exists while the futex has waiters.  Without a more durable state
present in the kernel, it is more or less impossible to have a persisted concept
of rights for a futex.

### Differences from Linux futexes

Note that all of the zircon futex operations key off of the virtual
address of an userspace pointer. This differs from the Linux
implementation, which distinguishes private futex operations (which
correspond to our in-process-only ones) from ones shared across
address spaces.

As noted above, all of our futex operations leave the value of the
futex unmodified from the kernel. Other potential operations, such as
Linux's `FUTEX_WAKE_OP`, requires atomic manipulation of the value
from the kernel, which our current implementation does not require.

### Ownership and Priority Inheritance

#### Overview

Some runtimes may need to implement synchronization primitives based on futexes
which exhibit priority inheritance behavior.  In order to support these users,
zircon futexes have a concept of 'ownership' which can be used to implement such
primitives.  Use of this feature is optional.

At any point in time, a futex may be either unowned, or owned by a single
thread.  When a thread owns one or more futexes, its effective priority becomes
the maximum of its base priority, and the priorities of all of the current
waiters of all of the futexes currently owned by it.  As soon a thread no longer
owns a futex, the pressure of the priorities of the futex's waiters disappears
from the relationship above.  Once the thread no longer owns any futexes, its
priority will relax back to its base priority.

Signaling of the owner of a futex is the responsibility of the userspace code,
as is applying the ownership concept properly when constructing a specific type
of synchronization object that needs priority inheritance behavior.

Zircon futexes have at most a single owner.  Multiple ownership of futexes for
the purpose of priority inheritance is not supported.  The owner of a futex may
never simultaneously be a waiter for the same futex.

#### Assigning Ownership

Ownership of a futex is assigned via each 'wait' or 'requeue' operation.  In the
case of a requeue operation, the target futex is the requeue futex, not the
wake_futex.  Users pass a handle to a thread indicating who the current owner of
the futex should be, or **ZX_HANDLE_INVALID** if there should be no owner.

+ Passing a valid handle to a thread to indicate the futex owner is the
  responsibility of the userspace code.  Passing an invalid handle, or a handle
  to a non-thread object will result in the wait/requeue operation failing.
+ Threads that have not been started yet may not own a futex.  Any attempt to
  assign ownership of a futex to a thread that has not yet been started will
  result in the wait/requeue operation failing.
+ Threads that have exited may not be the owner of a futex.  If a thread exits
  while it owns a futex, the futex will reset to being owned by no one.  If a
  user attempts assign ownership of a futex to a thread that has exited, the
  wait/requeue operation will behave as if ZX_HANDLE_INVALID had been passed as
  the new futex owner.
+ If the wait/requeue operation succeeds, the owner of the target futex will
  _always_ be set to either the thread specified, or nothing if
  **ZX_HANDLE_INVALID** is passed.
+ In particular, if the wait/requeue operation fails because of a mismatch
  between the expected futex value and the actual futex value, the owner of the
  futex will remain unchanged and the status code for the operation will be
  ZX_ERR_BAD_STATE. This error code will be returned regardless of the value
  passed for handle indicating ownership, even if the value passed would have
  resulted in a status of ZX_ERR_BAD_HANDLE being returned.

#### Transferring Ownership

Ownership of a futex may be transferred by the kernel on behalf of the user
during a wake operation or a requeue operation.  In the case of a requeue
operation, the target of the transfer is the wake_futex, not the requeue_futex.
Ownership transfer only takes place when using the
[`zx_futex_wake_single_owner()`] or [`zx_futex_requeue_single_owner()`]
variants of the wake/requeue operations.  The `single_owner` variants of
these operations will release exactly one waiter, and
assign ownership of the futex to the released thread.

+ If there are _no_ waiters during the wake operation, then there is already no
  owner.  This will remain unchanged.
+ If a requeue operation fails because of a mismatch between the expected futex
  value and the actual futex value, the owner of the futex will remain
  unchanged.
+ A successful call to either of the non-single_owner variants of the
  wake/requeue operation will cause the target futex's owner to be set to
  nothing.

### Papers about futexes

- [Fuss, Futexes and Furwocks: Fast Userlevel Locking in Linux](https://www.kernel.org/doc/ols/2002/ols2002-pages-479-495.pdf), Hubertus Franke and Rusty Russell

    This is the original white paper describing the Linux futex. It
    documents the history and design of the original implementation,
    prior (failed) attempts at creating a fast userspace
    synchronization primitive, and performance measurements.

- [Futexes Are Tricky](https://www.akkadia.org/drepper/futex.pdf), Ulrich Drepper

    This paper describes some gotchas and implementation details of
    futexes in Linux. It discusses the kernel implementation, and goes
    into more detail about correct and efficient userspace
    implementations of mutexes, condition variables, and so on.

- [Mutexes and Condition Variables using Futexes](http://locklessinc.com/articles/mutex_cv_futex/)

    Further commentary on "Futexes are tricky", outlining a simple
    implementation that avoids the need for `FUTEX_CMP_REQUEUE`

- [Locking in WebKit](https://webkit.org/blog/6161/locking-in-webkit/), Filip Pizlo

    An in-depth tour of the locking primitives in WebKit, complete with
    benchmarks and analysis. Contains a detailed explanation of the "parking
    lot" concept, which allows very compact representation of userspace
    mutexes.

## SYSCALLS

 - [`zx_futex_wait()`]
 - [`zx_futex_wake()`]
 - [`zx_futex_requeue()`]
 - [`zx_futex_get_owner()`]

[`zx_futex_get_owner()`]: /docs/reference/syscalls/futex_get_owner.md
[`zx_futex_requeue()`]: /docs/reference/syscalls/futex_requeue.md
[`zx_futex_requeue_single_owner()`]: /docs/reference/syscalls/futex_requeue_single_owner.md
[`zx_futex_wait()`]: /docs/reference/syscalls/futex_wait.md
[`zx_futex_wake()`]: /docs/reference/syscalls/futex_wake.md
[`zx_futex_wake_single_owner()`]: /docs/reference/syscalls/futex_wake_single_owner.md
[address_tagging]: /docs/contribute/governance/rfcs/0143_userspace_top_byte_ignore.md
[tbi]: https://developer.arm.com/documentation/den0024/a/ch12s05s01
