# Zircon Signals

## Introduction

A signal is a single bit of information that waitable zircon kernel objects expose to
applications.  Each object can expose one or more signals, most of which are specific
to the type of object.

For example, the signal **ZX_CHANNEL_READABLE** indicates "this channel endpoint has
messages to read", and **ZX_PROCESS_TERMINATED** indicates "this process stopped running."

The signals for an object are stored in a `uint32` bitmask, and their values (which are
object-specific) are defined in the header[`zircon/types.h`](/zircon/system/public/zircon/types.h).
The typedef `zx_signals_t` is used to refer to signal bitmasks in syscalls and other APIs.

Most objects are waitable.  Ports are an example of a non-waitable object.

## State, State Changes and their Terminology

A signal is said to be **Active** when its bit is 1 and **Inactive** when its bit is 0.

A signal is said to be **Asserted** when it is made **Active** in response to an event
(even if it was already **Active**), and is said to be **Deasserted** when it is made
**Inactive** in response to an event (even if it was already **Inactive**).

For example:  When a message is written into a Channel endpoint, the **ZX_CHANNEL_READABLE**
signal of the opposing endpoint is **asserted** (which causes that signal to become **active**,
if it were not already active).  When the last message in a Channel endpoint's
queue is read from that endpoint, the **ZX_CHANNEL_READABLE** signal of that endpoint is
**deasserted** (which causes that signal to become **inactive**)

## Observing Signals

The syscalls [`zx_object_wait_one()`], [`zx_object_wait_many()`], and
[`zx_object_wait_async()`], in combination with a Port, can be used to wait for
specified signals on one or more objects.

If multiple threads are operating on an object, the results of these syscalls
may already be out of date by the time the calling thread actually acts on them.
For example, a thread waiting on a Channel for the **ZX_CHANNEL_READABLE**
signal may wake from a [`zx_object_wait_one()`] syscall only to find that there
are no pending messages, because another thread has already read it.

## Synthetic Signals

The signal `ZX_SIGNAL_HANDLE_CLOSED` is a synthetic signal only exists in the
results of [`zx_object_wait_one()`] or [`zx_object_wait_many()`] and indicates
that a handle that was being waited upon has been been closed causing the wait
operation to be aborted.

This signal can only be obtained as a result of the above two wait calls when the wait itself
returns with **ZX_ERR_CANCELED**.

## User Signals

There are eight User Signals (**ZX_USER_SIGNAL_0** through **ZX_USER_SIGNAL_7**), which may
asserted or deasserted using the [`zx_object_signal()`] and [`zx_object_signal_peer()`] syscalls,
provided the handle has the appropriate rights (**ZX_RIGHT_SIGNAL** or **ZX_RIGHT_SIGNAL_PEER**,
respectively).  These User Signals are always initially inactive, and are only modified by
the object signal syscalls.

## See Also

 - [`zx_object_signal()`]
 - [`zx_object_signal_peer()`]
 - [`zx_object_wait_async()`]
 - [`zx_object_wait_many()`]
 - [`zx_object_wait_one()`]



[`zx_object_get_info()`]: /reference/syscalls/object_get_info.md
[`zx_object_signal()`]: /reference/syscalls/object_signal.md
[`zx_object_signal_peer()`]: /reference/syscalls/object_signal.md
[`zx_object_wait_async()`]: /reference/syscalls/object_wait_async.md
[`zx_object_wait_many()`]: /reference/syscalls/object_wait_many.md
[`zx_object_wait_one()`]: /reference/syscalls/object_wait_one.md
