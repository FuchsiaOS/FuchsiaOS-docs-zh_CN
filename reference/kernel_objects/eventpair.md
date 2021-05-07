# Event Pair

## NAME

eventpair - Mutually signalable pair of events for concurrent programming

## SYNOPSIS

Event Pairs are linked pairs of user-signalable objects. The 8 signal
bits reserved for userspace (**ZX_USER_SIGNAL_0** through
**ZX_USER_SIGNAL_7**) may be set or cleared on the local or opposing
endpoint of an Event Pair.

## DESCRIPTION

TODO

## SYSCALLS

 - [`zx_eventpair_create()`] - create a connected pair of events
 - [`zx_object_signal()`] - set or clear the user signals on an object
 - [`zx_object_signal_peer()`] - set or clear the user signals in the opposite end

[`zx_eventpair_create()`]: /docs/reference/syscalls/eventpair_create.md
[`zx_object_signal()`]: /docs/reference/syscalls/object_signal.md
[`zx_object_signal_peer()`]: /docs/reference/syscalls/object_signal_peer.md
