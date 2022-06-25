# Zircon Kernel Concepts

## Introduction

The kernel manages a number of different types of Objects. Those that are
accessible directly via system calls are C++ classes that implement the
Dispatcher interface. These are implemented in
[kernel/object](/zircon/kernel/object). Many are self-contained higher-level Objects.
Some wrap lower-level [lk][glossary.lk] primitives.

## [System Calls](reference/syscalls/README.md)

Userspace code interacts with kernel objects via system calls, and almost
exclusively via [Handles](concepts/kernel/handles.md).  In userspace, a Handle is represented as
32bit integer (type zx_handle_t).  When syscalls are executed, the kernel checks
that Handle parameters refer to an actual handle that exists within the calling
process's handle table.  The kernel further checks that the Handle is of the
correct type (passing a Thread Handle to a syscall requiring an event handle
will result in an error), and that the Handle has the required Rights for the
requested operation.

System calls fall into three broad categories, from an access standpoint:

1. Calls that have no limitations, of which there are only a very few, for
example [`zx_clock_get_monotonic()`](reference/syscalls/clock_get_monotonic.md)
and [`zx_nanosleep()`](reference/syscalls/nanosleep.md) may be called by any thread.
2. Calls that take a Handle as the first parameter, denoting the Object they act upon,
which are the vast majority, for example [`zx_channel_write()`](reference/syscalls/channel_write.md)
and [`zx_port_queue()`](reference/syscalls/port_queue.md).
3. Calls that create new Objects but do not take a Handle, such as
[`zx_event_create()`](reference/syscalls/event_create.md) and
[`zx_channel_create()`](reference/syscalls/channel_create.md).  Access to these (and limitations
upon them) is controlled by the Job in which the calling Process is contained.

System calls are provided by libzircon.so, which is a "virtual" shared
library that the Zircon kernel provides to userspace, better known as the
[*virtual Dynamic Shared Object* or vDSO](vdso.md).
They are C ELF ABI functions of the form `zx_noun_verb()` or
`zx_noun_verb_direct-object()`.

The system calls are defined in a customized form of FIDL in [//zircon/vdso](/zircon/vdso/).
Those definitions are first processed by `fidlc`, and then by `kazoo`, which takes the IR
representation from `fidlc` and outputs various formats that are used as glue in the VDSO, kernel,
etc.

## [Handles](concepts/kernel/handles.md) and [Rights](concepts/kernel/rights.md)

Objects may have multiple Handles (in one or more Processes) that refer to them.

For almost all Objects, when the last open Handle that refers to an Object is closed,
the Object is either destroyed, or put into a final state that may not be undone.

Handles may be moved from one Process to another by writing them into a Channel
(using [`zx_channel_write()`](reference/syscalls/channel_write.md)), or by using
[`zx_process_start()`](reference/syscalls/process_start.md) to pass a Handle as the argument
of the first thread in a new Process.

The actions that may be taken on a Handle or the Object it refers to are governed
by the Rights associated with that Handle.  Two Handles that refer to the same Object
may have different Rights.

The [`zx_handle_duplicate()`](reference/syscalls/handle_duplicate.md) and
[`zx_handle_replace()`](reference/syscalls/handle_replace.md) system calls may be used to
obtain additional Handles referring to the same Object as the Handle passed in,
optionally with reduced Rights.  The [`zx_handle_close()`](reference/syscalls/handle_close.md)
system call closes a Handle, releasing the Object it refers to, if that Handle is
the last one for that Object. The [`zx_handle_close_many()`](reference/syscalls/handle_close_many.md)
system call similarly closes an array of handles.


## Kernel Object IDs

Every object in the kernel has a "kernel object id" or "koid" for short.
It is a 64 bit unsigned integer that can be used to identify the object
and is unique for the lifetime of the running system.
This means in particular that koids are never reused.

There are two special koid values:

**ZX_KOID_INVALID** Has the value zero and is used as a "null" sentinel.

**ZX_KOID_KERNEL** There is only one kernel, and it has its own koid.

Kernel generated koids only use 63 bits (which is plenty).
This leaves space for artificially allocated koids by having the most
significant bit set. The sequence in which kernel generated koids are allocated
is unspecified and subject to change.

Artificial koids exist to support things like identifying artificial objects,
like virtual threads in tracing, for consumption by tools.
How artificial koids are allocated is left to each program,
this document does not impose any rules or conventions.


## Running Code: Jobs, Processes, and Threads.

Threads represent threads of execution (CPU registers, stack, etc) within an
address space that is owned by the Process in which they exist.  Processes are
owned by Jobs, which define various resource limitations.  Jobs are owned by
parent Jobs, all the way up to the Root Job, which was created by the kernel at
boot and passed to [`userboot`, the first userspace Process to begin execution](concepts/process/userboot.md).

Without a Job Handle, it is not possible for a Thread within a Process to create another
Process or another Job.

[Program loading](concepts/process/program_loading.md) is provided by userspace facilities and
protocols above the kernel layer.

See: [`zx_process_create()`](reference/syscalls/process_create.md),
[`zx_process_start()`](reference/syscalls/process_start.md),
[`zx_thread_create()`](reference/syscalls/thread_create.md),
and [`zx_thread_start()`](reference/syscalls/thread_start.md).


## Message Passing: Sockets and Channels

Both Sockets and Channels are IPC Objects that are bi-directional and two-ended.
Creating a Socket or a Channel will return two Handles, one referring to each endpoint
of the Object.

Sockets are stream-oriented and data may be written into or read out of them in units
of one or more bytes.  Short writes (if the Socket's buffers are full) and short reads
(if more data is requested than in the buffers) are possible.

Channels are datagram-oriented and have a maximum message size given by **ZX_CHANNEL_MAX_MSG_BYTES**,
and may also have up to **ZX_CHANNEL_MAX_MSG_HANDLES** Handles attached to a message.
They do not support short reads or writes -- either a message fits or it does not.

When Handles are written into a Channel, they are removed from the sending Process.
When a message with Handles is read from a Channel, the Handles are added to the receiving
Process.  Between these two events, the Handles continue to exist (ensuring the Objects
they refer to continue to exist), unless the end of the Channel that they have been written
towards is closed -- at which point messages in flight to that endpoint are discarded and
any Handles they contained are closed.

See: [`zx_channel_create()`](reference/syscalls/channel_create.md),
[`zx_channel_read()`](reference/syscalls/channel_read.md),
[`zx_channel_write()`](reference/syscalls/channel_write.md),
[`zx_channel_call()`](reference/syscalls/channel_call.md),
[`zx_socket_create()`](reference/syscalls/socket_create.md),
[`zx_socket_read()`](reference/syscalls/socket_read.md),
and [`zx_socket_write()`](reference/syscalls/socket_write.md).

## Objects and Signals

Objects may have up to 32 signals (represented by the zx_signals_t type and the ZX_*_SIGNAL_*
defines), which represent a piece of information about their current state.  Channels and Sockets,
for example, may be READABLE or WRITABLE.  Processes or Threads may be TERMINATED.  And so on.

Threads may wait for signals to become active on one or more Objects.

See [signals](concepts/kernel/signals.md) for more information.

## Waiting: Wait One, Wait Many, and Ports

A Thread may use [`zx_object_wait_one()`](reference/syscalls/object_wait_one.md)
to wait for a signal to be active on a single handle or
[`zx_object_wait_many()`](reference/syscalls/object_wait_many.md) to wait for
signals on multiple handles.  Both calls allow for a timeout after
which they'll return even if no signals are pending.

Timeouts may deviate from the specified deadline according to timer
slack. See [timer slack](concepts/kernel/timer_slack.md) for more information.

If a Thread is going to wait on a large set of handles, it is more efficient to use
a Port, which is an Object that other Objects may be bound to such that when signals
are asserted on them, the Port receives a packet containing information about the
pending Signals.

See: [`zx_port_create()`](reference/syscalls/port_create.md),
[`zx_port_queue()`](reference/syscalls/port_queue.md),
[`zx_port_wait()`](reference/syscalls/port_wait.md),
and [`zx_port_cancel()`](reference/syscalls/port_cancel.md).


## Events, Event Pairs.

An Event is the simplest Object, having no other state than its collection of active Signals.

An Event Pair is one of a pair of Events that may signal each other.  A useful property of
Event Pairs is that when one side of a pair goes away (all Handles to it have been
closed), the PEER_CLOSED signal is asserted on the other side.

See: [`zx_event_create()`](reference/syscalls/event_create.md),
and [`zx_eventpair_create()`](reference/syscalls/eventpair_create.md).


## Shared Memory: Virtual Memory Objects (VMOs)

Virtual Memory Objects represent a set of physical pages of memory, or the *potential*
for pages (which will be created/filled lazily, on-demand).

They may be mapped into the address space of a Process with
[`zx_vmar_map()`](reference/syscalls/vmar_map.md) and unmapped with
[`zx_vmar_unmap()`](reference/syscalls/vmar_unmap.md).  Permissions of
mapped pages may be adjusted with [`zx_vmar_protect()`](reference/syscalls/vmar_protect.md).

VMOs may also be read from and written to directly with
[`zx_vmo_read()`](reference/syscalls/vmo_read.md) and [`zx_vmo_write()`](reference/syscalls/vmo_write.md).
Thus the cost of mapping them into an address space may be avoided for one-shot operations
like "create a VMO, write a dataset into it, and hand it to another Process to use."

## Address Space Management

Virtual Memory Address Regions (VMARs) provide an abstraction for managing a
process's address space.  At process creation time, a handle to the root VMAR
is given to the process creator.  That handle refers to a VMAR that spans the
entire address space.  This space can be carved up via the
[`zx_vmar_map()`](reference/syscalls/vmar_map.md) and
[`zx_vmar_allocate()`](reference/syscalls/vmar_allocate.md) interfaces.
[`zx_vmar_allocate()`](reference/syscalls/vmar_allocate.md) can be used to generate new
VMARs (called subregions or children), which can be used to group together
parts of the address space.

See: [`zx_vmar_map()`](reference/syscalls/vmar_map.md),
[`zx_vmar_allocate()`](reference/syscalls/vmar_allocate.md),
[`zx_vmar_protect()`](reference/syscalls/vmar_protect.md),
[`zx_vmar_unmap()`](reference/syscalls/vmar_unmap.md),
and [`zx_vmar_destroy()`](reference/syscalls/vmar_destroy.md),

## Futexes

Futexes are kernel primitives used with userspace atomic operations to implement
efficient synchronization primitives -- for example, Mutexes, which only need to make
a syscall in the contended case.  Usually they are only of interest to implementers of
standard libraries.  Zircon's libc and libc++ provide C11, C++, and pthread APIs for
mutexes, condition variables, etc, implemented in terms of Futexes.

See: [`zx_futex_wait()`](reference/syscalls/futex_wait.md),
[`zx_futex_wake()`](reference/syscalls/futex_wake.md),
and [`zx_futex_requeue()`](reference/syscalls/futex_requeue.md).


[glossary.lk]: glossary/README.md#lk
