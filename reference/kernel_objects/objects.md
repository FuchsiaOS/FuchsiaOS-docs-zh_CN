# Zircon Kernel objects

[TOC]

Zircon is an object-based kernel. User mode code almost exclusively interacts
with OS resources via object handles. A handle can be thought of as an active
session with a specific OS subsystem scoped to a particular resource.

Zircon actively manages the following resources:

+ processor time
+ memory and address spaces
+ device-io memory
+ interrupts
+ signaling and waiting

## Kernel objects for applications

### IPC
+ [Channel](/docs/reference/kernel_objects/channel.md)
+ [Socket](/docs/reference/kernel_objects/socket.md)
+ [FIFO](/docs/reference/kernel_objects/fifo.md)

### Tasks
+ [Process](/docs/reference/kernel_objects/process.md)
+ [Thread](/docs/reference/kernel_objects/thread.md)
+ [Job](/docs/reference/kernel_objects/job.md)
+ [Task](/docs/reference/kernel_objects/task.md)

### Scheduling
+ [Profile](/docs/reference/kernel_objects/profile.md)

### Signaling
+ [Event](/docs/reference/kernel_objects/event.md)
+ [Event Pair](/docs/reference/kernel_objects/eventpair.md)
+ [Futex](/docs/reference/kernel_objects/futex.md)

### Memory and address space
+ [Virtual Memory Object](/docs/reference/kernel_objects/vm_object.md)
+ [Virtual Memory Address Region](/docs/reference/kernel_objects/vm_address_region.md)
+ [bus_transaction_initiator](/docs/reference/kernel_objects/bus_transaction_initiator.md)
+ [Pager](/docs/reference/kernel_objects/pager.md)

### Waiting
+ [Port](/docs/reference/kernel_objects/port.md)
+ [Timer](/docs/reference/kernel_objects/timer.md)

## Kernel objects for drivers

+ [Interrupts](/docs/reference/kernel_objects/interrupts.md)
  + [Message Signaled Interrupts](/docs/reference/kernel_objects/msi.md)
+ [Resource](/docs/reference/kernel_objects/resource.md)
+ [Debuglog](/docs/reference/kernel_objects/debuglog.md)

## Kernel Object and LK
Some kernel objects wrap one or more LK-level constructs. For example the
Thread object wraps one `thread_t`. However the Channel does not wrap
any LK-level objects.

## Kernel object lifetime
Kernel objects are ref-counted. Most kernel objects are born during a syscall
and are held alive at refcount = 1 by the handle, which binds the handle value
given as the output of the syscall. The handle object is held alive as long it
is attached to a handle table. Handles are detached from the handle table
closing them (for example via `sys_close()`), which decrements the refcount of
the kernel object. Usually, when the last handle is closed the kernel object
refcount will reach 0 which causes the destructor to be run.

The refcount increases both when new handles (referring to the object) are
created and when a direct pointer reference (by some kernel code) is acquired;
therefore a kernel object lifetime might be longer than the lifetime of the
process that created it.

## Dispatchers
A kernel object is implemented as a C++ class that derives from `Dispatcher`
and that overrides the methods it implements. Thus, for example, the code
of the Thread object is found in `ThreadDispatcher`. There is plenty of
code that only cares about kernel objects in the generic sense, in that case
the name you'll see is `fbl::RefPtr<Dispatcher>`.

## Kernel Object security
In principle, kernel objects do not have an intrinsic notion of security and
do not do authorization checks; security rights are held by each handle. A
single process can have two different handles to the same object with
different rights.

## See Also
[Handles](/docs/concepts/kernel/handles.md)
