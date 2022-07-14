# zx_object_wait_async

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Subscribe for signals on an object.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_object_wait_async(zx_handle_t handle,
                                 zx_handle_t port,
                                 uint64_t key,
                                 zx_signals_t signals,
                                 uint32_t options);
```

## DESCRIPTION

`zx_object_wait_async()` is a non-blocking syscall that causes packets to be
enqueued on *port* when the object specified by *handle* has one or more of the
specified [signals] asserted. Use [`zx_port_wait()`] to retrieve the packets.

*handle* points to the object that is to be watched for changes and must be a waitable object.

The *options* argument can be 0 or it can be one or more of

  * ZX_WAIT_ASYNC_TIMESTAMP which causes the system to capture a timestamp when
    the wait triggered.
  * ZX_WAIT_ASYNC_EDGE causes the port to not enqueue a packet for signals active
    at the time of the `zx_object_wait_async()` call.

The *signals* argument is a bitmask indicating which [signals] on the object
specified by *handle* will cause a packet to be enqueued.

Without **ZX_WAIT_ASYNC_EDGE**, if **any** of the signals in *signals* are active
when `zx_object_wait_async()` is called or become active afterwards, a packet will
be enqueued on *port*.

With **ZX_WAIT_ASYNC_EDGE**, a packet will be enqueued on *port* only after one or more
signals in *signals* have transitioned from inactive to active. When using this option,
care should be taken that an inactive signal becomes unexpectedly active before
the call `zx_object_wait_async()` has completed. In such cases the transition can be missed
and no packet will ever be queued to the port. For example, this is often used
before performing non-blocking I/O until the signal becomes inatctive, ensuring that
a subsequent transition from inactive to active will cause a packet to be queued.

When a packet is enqueued, it will contain all of the currently-asserted signals
(not just the ones listed in the *signals* argument).  Once a packet has been enqueued
the asynchronous waiting ends. No further packets will be enqueued. Note that signals
are OR'd into the state maintained by the port thus you may see any combination of requested
signals when [`zx_port_wait()`] returns.

[`zx_port_cancel()`] will terminate the operation and if a packet was
in the queue on behalf of the operation, that packet will be removed from the queue.

If *handle* is closed, the operation will also be terminated, but packets already
in the queue are not affected.

Packets generated via this syscall will have *type* set to **ZX_PKT_TYPE_SIGNAL_ONE**
and the union is of type `zx_packet_signal_t`:

```
typedef struct zx_packet_signal {
    zx_signals_t trigger;
    zx_signals_t observed;
    uint64_t count;
    zx_time_t timestamp;       // depends on ZX_WAIT_ASYNC_TIMESTAMP
    uint64_t reserved1;
} zx_packet_signal_t;
```

*trigger* is the signals used in the call to `zx_object_wait_async()`,
*observed* is the signals actually observed, and *timestamp* is clock-monotonic
time when the object state transitioned to meet the trigger condition. If
options does not include ZX_WAIT_ASYNC_TIMESTAMP the timestamp is reported as 0.

Use the `zx_port_packet_t`'s *key* member to track what object this packet corresponds.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must have **ZX_RIGHT_WAIT**.

*port* must be of type **ZX_OBJ_TYPE_PORT** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_object_wait_async()` returns **ZX_OK** if the subscription succeeded.

## ERRORS

**ZX_ERR_INVALID_ARGS**  *options* has bits other than **ZX_WAIT_ASYNC_TIMESTAMP**
and **ZX_WAIT_ASYNC_EDGE** set.

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle or *port* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *port* is not a Port handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_WAIT** or *port*
does not have **ZX_RIGHT_WRITE**.

**ZX_ERR_NOT_SUPPORTED**  *handle* is a handle that cannot be waited on.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## NOTES

See [signals] for more information about signals and their terminology.

## SEE ALSO

 - [signals]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]
 - [`zx_port_cancel()`]
 - [`zx_port_queue()`]
 - [`zx_port_wait()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[signals]: /concepts/kernel/signals.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
[`zx_port_cancel()`]: port_cancel.md
[`zx_port_queue()`]: port_queue.md
[`zx_port_wait()`]: port_wait.md
