# Zircon Kernel objects

[TOC]

Zircon is an object-based kernel. User mode code almost exclusively interacts
with OS resources via object [handles]. A handle can be thought of as an active
session with a specific OS subsystem scoped to a particular resource.

Zircon actively manages the following resources:

+ processor time
+ memory and address spaces
+ device-io memory
+ interrupts
+ signaling and waiting
+ inter-process communication

## Kernel objects for applications

### IPC

+ [Channel](/reference/kernel_objects/channel.md)
+ [Socket](/reference/kernel_objects/socket.md)
+ [FIFO](/reference/kernel_objects/fifo.md)

### Tasks

+ [Process](/reference/kernel_objects/process.md)
+ [Thread](/reference/kernel_objects/thread.md)
+ [Job](/reference/kernel_objects/job.md)
+ [Task](/reference/kernel_objects/task.md)

### Scheduling

+ [Profile](/reference/kernel_objects/profile.md)

### Signaling

+ [Event](/reference/kernel_objects/event.md)
+ [Event Pair](/reference/kernel_objects/eventpair.md)
+ [Futex](/reference/kernel_objects/futex.md)

### Memory and address space

+ [Virtual Memory Object](/reference/kernel_objects/vm_object.md)
+ [Virtual Memory Address Region](/reference/kernel_objects/vm_address_region.md)
+ [bus_transaction_initiator](/reference/kernel_objects/bus_transaction_initiator.md)
+ [Pager](/reference/kernel_objects/pager.md)

### Waiting

+ [Port](/reference/kernel_objects/port.md)
+ [Timer](/reference/kernel_objects/timer.md)

## Kernel objects for drivers

+ [Interrupts](/reference/kernel_objects/interrupts.md)
+ [Message Signaled Interrupts](/reference/kernel_objects/msi.md)
+ [Resource](/reference/kernel_objects/resource.md)
+ [Debuglog](/reference/kernel_objects/debuglog.md)

## Kernel object lifetime

Kernel objects are [reference-counted]. Most kernel objects are
created during a 'create' syscall and are held alive by the first handle,
given as the output of the create syscall. The caller gets the numeric id of
the handle and the handle itself is placed in the handle table of the process.

A handle is held alive as long it exists in the handle table. Handles are
removed from the handle table by:

+ Closing them via [`zx_handle_close`] which decrements the reference
count of the corresponding kernel object. Usually, when the last handle is
closed the kernel object reference count will reach 0 which causes the kernel
object to be destroyed.

+ When a channel endpoint holding an unread message containing a handle is
destroyed, all of the pending messages will also be destroyed, closing any
handles contained in the messages while doing so.

+ When the process that owns the handle table is destroyed. The kernel
effectively iterates over the entire handle table closing each handle in turn.

The reference count increases when new handles (referring to the same object)
are created via [`zx_handle_duplicate`], but also when a direct pointer
reference (by some kernel code) is acquired; therefore a kernel object lifetime
might be longer than the lifetime of the code that created it. A separate count
of active handles referencing an object is also maintained, allowing the kernel
to trigger specific behaviors when the handle count of an object reaches zero,
even if the kernel is keeping the object alive behind the scenes because of a
direct pointer reference.

There are three important cases in which kernel objects are kept alive
when there are no outstanding handles to them:

+ The object is referenced by a handle in a message which has not been consumed.
This can happen via the [channel APIs][channel-api]. While such handle is in
the channel the kernel keeps the object alive, and with a non-zero active
handle count.

+ The object is the parent of another object which is alive. This is the
case of [VMOs] attached to live [VMARs], of processes with live [threads] and
[jobs] with live processes or child jobs.

+ Threads are kept alive by the scheduler. A thread that is alive will continue
to live until it voluntarily exits by calling [`zx_thread_exit`] or the process
is terminated via [`zx_task_kill`].

The outcome of the last case is that a single thread can keep its process
and the entire lineage of jobs up to the root job alive.

## Peered object and the peer-closed state

Currently, the kernel defines the following object types as "peered" objects.

 Name                                                     | Peer-Closed Signal Name
----------------------------------------------------------+-------------------------
[Channel](/reference/kernel_objects/channel.md)      | `ZX_CHANNEL_PEER_CLOSED`
[Socket](/reference/kernel_objects/socket.md)        | `ZX_SOCKET_PEER_CLOSED`
[FIFO](/reference/kernel_objects/fifo.md)            | `ZX_FIFO_PEER_CLOSED`
[Event Pair](/reference/kernel_objects/eventpair.md) | `ZX_EVENTPAIR_PEER_CLOSED`

All peered objects are created in pairs, which are internally linked to each
other in a peer relationship.  When the active handle count of a peered object
reaches 0, if that object still has a link to its peer, the peer object will be
placed in the `PEER_CLOSED` state, causing the link to be destroyed, the
specific `ZX_*_PEER_CLOSED` signal to become asserted on the peer, and for
syscalls involving the object's peer (for example, `zx_channel_write`) to return
the error `ZX_ERR_PEER_CLOSED`.

When the final handle to an object is closed via a call to [`zx_handle_close`],
or [`zx_handle_close_many`], it is guaranteed that the object's peer (if any)
will be placed into the `PEER_CLOSED` state, asserting its associated signal in
the process, before the `zx_handle_close` syscall returns from the kernel.

Note that objects are placed into `PEER_CLOSED` when their peer's _active handle
count_ has hit zero, even of the peer object continues to live because of a
direct pointer reference held by the kernel.

## Kernel Object security

Kernel objects do not have an intrinsic notion of security and do not do
authorization checks; security rights are held by each handle. A single process
can have two different handles to the same object with different rights.

## See Also

[Handles][handles]

[handles]: /concepts/kernel/handles.md
[reference-counted]: https://en.wikipedia.org/wiki/Reference_counting
[`zx_handle_close`]: /reference/syscalls/handle_close.md
[`zx_handle_close_many`]: /reference/syscalls/handle_close_many.md
[`zx_handle_duplicate`]: /reference/syscalls/handle_duplicate.md
[`zx_thread_exit`]:/reference/syscalls/thread_exit.md
[`zx_task_kill`]: /reference/syscalls/task_kill.md
[VMOs]: /reference/kernel_objects/vm_object.md
[VMARs]: /reference/kernel_objects/vm_address_region.md
[threads]: /reference/kernel_objects/thread.md
[jobs]: /reference/kernel_objects/job.md
[channel-api]: /reference/kernel_objects/channel.md
