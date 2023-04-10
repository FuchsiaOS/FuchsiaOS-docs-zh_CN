{% set rfcid = "RFC-0009" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

Waiting for signals to be asserted on an object is usually level-triggered and
a check is done at the start of `zx_object_wait_async` in case the signal is
already active, in which case a packet is immediately sent to the port.
This RFC concerns adding an option to `zx_object_wait_async`, `ZX_WAIT_ASYNC_EDGE`,
which does not perform that initial check and thus will only produce a packet
when the signal transitions from inactive to active after the call.

It may be that `zx_object_wait_async` is called with `ZX_WAIT_ASYNC_EDGE` with
the signals on the object already active. In this case, a packet will be queued
on the port only after the signal on the object becomes inactive and then
subsequently is asserted. In fact, this is how the `ZX_WAIT_ASYNC_EDGE` is commonly
used.

## Motivation

The `epoll` polling mechanism in Linux can function in two modes - level-triggered
and edge-triggered. Fuchsia's waiting features, particularly `zx_object_wait_async`
and `zx_port_wait` already make level-triggered polling possible. However, edge-triggered
polling requires the ability to wait on a signal on an object that is already
active is expected (through I/O) to become inactive and subsequently
active again, queuing a packet on the port on this subsequent signal transition.
This is the intent of `ZX_WAIT_ASYNC_EDGE`.

## Design

Implementation of the `ZX_WAIT_ASYNC_EDGE` flag of `zx_object_wait_async` is
fortunately quite simple.

At present, if one of the signal set is already active, the observer's `OnMatch`
method is called directly without any further action. Otherwise, if none of the
signal set is active, the set is added to the interest list of the `DispatchObject`
via the supplied `SignalObserver`.

The proposal is that, if the `ZX_WAIT_ASYNC_EDGE` flag is specified, the initial check
is omitted and the signal set added to the interest list of the `DispatchObject`
regardless of the initial signal state. In this mode of operation, one of the
signals must transition from inactive to active for a packet to be queued on
the supplied port (possibly requiring a signal to become inactive in the process).

### Use of ZX_WAIT_ASYNC_EDGE in epoll with EPOLLET edge triggering

The main use of this change is to enable edge-triggered polling with the EPOLLET flag
in epoll. Waiting in Zircon differs from polling in `epoll` in that file descriptors are
added to an `epoll` file descriptor using `epoll_ctl/EPOLL_CTL_ADD` and are continually
monitored until removed with `epoll_ctl/EPOLL_CTL_DEL`. Zircon waiting, especially
with `zx_object_wait_async`, is always one-shot and the file object must be "re-armed"
by calling `zx_object_wait_async` again after a signal has become active.

Because `epoll` use must operate by repeatedly calling `epoll_wait` (without necessarily
calling `epoll_ctl`), this re-arming call to `zx_object_wait_async` must occur somewhere
in `epoll_wait`.

For the default level-triggered polling, in `epoll_wait` once `zx_port_wait` returns with
a signalled file object, we cannot call `zx_object_wait_async` before returning, because
the signal on that object is actve and will generate a duplicate packet on the port. Therefore,
a list of active level-triggered file descriptors is maintained and `zx_object_wait_async`
is called on file descriptors in this list on entering `epoll_wait` prior to waiting in
`zx_port_wait`.

For edge-triggered polling, after `epoll_wait` returns, non-blocking I/O should be
performed until `EWOULDBLOCK` is returned. At that point the signal on the file object will
be inactive. At this point `epoll_wait` should be called. However, if the signal on the file
object becomes active between the I/O operation returning `EWOULDBLOCK` and `epoll_wait` being
called, that event will be lost unless `zx_object_wait_async` has already been called.
It follows that, in edge-triggered mode, the call to `zx_object_wait_async` to re-arm
the file object must be called *before* `epoll_wait` returns. This is where `ZX_WAIT_ASYNC_EDGE`
is necessary. The call to `zx_object_wait_async` can be called with this flag between
`zx_port_wait` returning and `epoll_wait` returning, because although the signal is active
at this point, the `ZX_WAIT_ASYNC_EDGE` skips the check that the signals are active
(which they are at this point), so no packet is immediately queued on the port.
This means that when the I/O occurs until `EWOULDBLOCK`, the file object is already being
monitored by `zx_object_wait_async` and there is no gap in coverage.

## Implementation

The addition of the `ZX_WAIT_ASYNC_EDGE` option to `zx_object_wait_async` has
already been implemented in [fxr/438521](https://fuchsia-review.googlesource.com/c/fuchsia/+/438521)
and its use in epoll has been implemented in
[fxr/438656](https://fuchsia-review.googlesource.com/c/fuchsia/+/438656).

## Performance

The performance impact will be negligible as only an extra added method parameter
and a check for that parameter is added to the existing code.

## Security considerations

N/A

## Privacy considerations

N/A

## Testing

Additional unit tests have been added.

## Documentation

Documentation has been added to `zx_object_wait_async` in the implementing CL.
[zx_object_wait_async](/docs/reference/syscalls/object_wait_async.md)

## Drawbacks, alternatives, and unknowns

This appears to be the simplest way of implementing this feature and
is analogous to how edge-triggered polling is implemented in other
operating systems.

Care should be taken to not miss I/O events when using this flag.
A signal may become active after performing I/O and before making
a call to `zx_object_wait_async`, in which case the transition from
unsignalled to signalled may be missed. In practice, the `ZX_WAIT_ASYNC_EDGE`
flag is used immediately `zx_port_wait` has returned indicating that
the signal is active on the object. In this way, after non-blocking
I/O is performed on the object until the signal is inactive (usually
until `ZX_ERR_SHOULD_WAIT` is returned) `zx_port_wait` can be called to
wait until additional I/O is ready.

Because of the pattern in edge-triggered `epoll_wait` with `EPOLLET` of (1)
`epoll_wait` (2) non-blocking I/O until fd is not-ready (3) `epoll_wait` again,
an alternative to `ZX_WAIT_ASYNC_EDGE` would be to perform a check in every
I/O operation to see if the file descriptor has been added to an epoll file
descriptor via `epoll_ctl` with `EPOLLET` (and has ceased to be ready) and
re-arms a wait. This would require considerable modification to `zxio` and
`fdio` for a somewhat rare use case.

## Prior art and references

The purpose of this change is to emulate the EPOLLET flag in Linux:

https://man7.org/linux/man-pages/man7/epoll.7.html
