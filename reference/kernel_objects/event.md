# Event

## NAME

event - Signalable event for concurrent programming

## SYNOPSIS

Events are user-signalable objects. The 8 signal bits reserved for
userspace (**ZX_USER_SIGNAL_0** through **ZX_USER_SIGNAL_7**) may be set,
cleared, and waited upon.

## DESCRIPTION

TODO

## SYSCALLS

 - [`zx_event_create()`] - create an event
 - [`zx_object_signal()`] - set or clear the user signals on an object

## SEE ALSO

 - [eventpair](eventpair.md) - linked pairs of signalable objects

[`zx_event_create()`]: /docs/reference/syscalls/event_create.md
[`zx_object_signal()`]: /docs/reference/syscalls/object_signal.md
