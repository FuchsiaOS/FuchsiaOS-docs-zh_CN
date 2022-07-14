# Channel

## NAME

channel - Bidirectional interprocess communication

## SYNOPSIS

A channel is a bidirectional transport of messages consisting of some
amount of byte data and some number of handles.

## DESCRIPTION

Channels have two endpoints. Each endpoint, logically, maintains an ordered
queue of messages to be read. Writing to an endpoint enqueues a message in the
other endpoint's queue. When the last handle to an endpoint is closed the unread
messages in that endpoint's queue are destroyed. Because destroying a message
closes any handles contained by the message, closing a channel endpoint may have
a recursive effect (e.g. channel contains a message, which contains a channel,
which contains a message, and so on).

Closing the last handle to a channel has no impact on the lifetime of messages
previously written to that channel. This gives channels "fire and forget"
semantics.

A message consists of some amount of data and some number of handles. A call to
[`zx_channel_write()`] enqueues one message, and a call to [`zx_channel_read()`]
dequeues one message (if any are queued). A thread can block until messages are
pending via [`zx_object_wait_one()`] or other waiting mechanisms.

Alternatively, a call to [`zx_channel_call()`] enqueues a message in one
direction of the channel, waits for a corresponding response, and
dequeues the response message. In call mode, corresponding responses
are identified via the first 4 bytes of the message, called the
transaction ID. The kernel supplies distinct transaction IDs (always with the
high bit set) for messages written with [`zx_channel_call()`].

The process of sending a message via a channel has two steps. The first is to
atomically write the data into the channel and move ownership of all handles in
the message into this channel. This operation always consumes the handles: at
the end of the call, all handles either are all in the channel or are all
discarded. The second operation, channel read, is similar: on success
all the handles in the next message are atomically moved into the
receiving process' handle table. On failure, the channel retains
ownership unless the **ZX_CHANNEL_READ_MAY_DISCARD** option
is specified, then they are dropped.

Unlike many other kernel object types, channels are not duplicatable. Thus, there
is only ever one handle associated with a channel endpoint, and the process holding
that handle is considered the owner. Only the owner can read or write messages or send
the channel endpoint to another process.

When ownership of a channel endpoint moves from one process to another,
messages will not be reordered or truncated, even if a write is in progress.
Messages before the transfer event belong to the previous owner and messages
after the transfer belong to the new owner.
The same applies if a read is in progress when the endpoint is transferred.

The above sequential guarantee is not provided for other kernel objects, even if
the last remaining handle is stripped of the **ZX_RIGHT_DUPLICATE** right.

## SYSCALLS

 - [`zx_channel_call()`] - synchronously send a message and receive a reply
 - [`zx_channel_create()`] - create a new channel
 - [`zx_channel_read()`] - receive a message from a channel
 - [`zx_channel_write()`] - write a message to a channel

<br>

 - [`zx_object_wait_one()`] - wait for signals on one object

## SEE ALSO

+ [Zircon concepts](/concepts/kernel/concepts.md)
+ [Handles](/concepts/kernel/handles.md)

[`zx_channel_call()`]: /reference/syscalls/channel_call.md
[`zx_channel_create()`]: /reference/syscalls/channel_create.md
[`zx_channel_read()`]: /reference/syscalls/channel_read.md
[`zx_channel_write()`]: /reference/syscalls/channel_write.md
[`zx_object_wait_one()`]: /reference/syscalls/object_wait_one.md
