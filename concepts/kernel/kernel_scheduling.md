Zircon Scheduling
=================

Background
==========

The primary responsibility of any scheduler is to share the limited
resource of processor time between all threads that wish to use it. In a
general purpose operating system, it tries to do so in a fair way,
ensuring that all threads are allowed to make some progress.

Our scheduler is an evolution of LK’s scheduler. As such it
started as a minimal scheduler implementation and was extended to meet
our needs as the project grew.

Design
======

#### Overview

In essence there is a scheduler running on each logical CPU in the machine.
These schedulers run independently and use IPI (Inter-Processor
Interrupts) to coordinate. However each CPU is responsible for
scheduling the threads that are running on it. See [*CPU
Assignment*](#cpu-assignment-and-migration) below for how we decide
which CPU a thread is on, and how/when it migrates.

Each CPU has its own set of priority queues. One for each priority level
in the system, currently 32. Note that these are fifo queues, not the data
structure known as a priority queue. In each queue is an ordered list of
runnable threads awaiting execution. When it is time for a new thread to run,
the scheduler simply looks at the highest numbered queue that contains a thread,
pops the head off of that queue and runs that thread.See
[*Priority Management*](#priority-management) below for more details
about how it decides which thread should be in which queue. If there are no
threads in the queues to run it will instead run the idle thread, see [Realtime
and idle threads](#realtime-and-idle-threads) below for more details.

Each thread is assigned the same timeslice size (THREAD_INITIAL_TIME_SLICE)
when it is picked to start running. If it uses its whole timeslice it will be
reinserted at the end of the appropriate priority queue. However if it has
some of its timeslice remaining from a previous run it will be inserted at the
head of the priority queue so it will be able to resume as quickly as possible.
When it is picked back up again it will only run for the remainder of its
previous timeslice.

When the scheduler selects a new thread from the priority queue it sets
the CPU's preemption timer for either a full timeslice, or the remainder of the
previous timeslice. When that timer fires the scheduler will stop execution on
that thread, add it to the appropriate queue, select another thread and start
over again.

If a thread blocks waiting for a shared resource then it's taken out of
its priority queue and is placed in a wait queue for the shared resource.
When it is unblocked it will be reinserted in the appropriate priority
queue of an eligible CPU ([*CPU
Assignment*](#cpu-assignment-and-migration)) and if it had remaining timeslice
to run it will be added to the front of the queue for expedited handling.

#### Priority management {#priority-management}

There are three different factors used to determine the effective
priority of a thread, the effective priority being what is used to
determine which queue it will be in.

The first factor is the base priority, which is simply the thread’s
requested priority. There are currently 32 levels with 0 being the
lowest and 31 being the highest.

The second factor is the priority boost. This is a value bounded between
\[-MAX_PRIORITY_ADJ, MAX_PRIORITY_ADJ\] used to offset the base priority, it is
modified by the following cases:

-   When a thread is unblocked, after waiting on a shared resource or
    sleeping, it is given a one point boost.

-   When a thread yields (volunteers to give up control), or volunteers
    to reschedule, its boost is decremented by one but is capped at 0
    (won’t go negative).

-   When a thread is preempted and has used up its entire timeslice, its
    boost is decremented by one but is able to go negative.

The third factor is its inherited priority. If the thread is in control
of a shared resource and it is blocking another thread of a higher
priority then it is given a temporary boost up to that thread’s priority
to allow it to finish quickly and allow the higher priority thread to
resume.

The effective priority of the thread is either the inherited priority,
if it has one, or the base priority plus its boost. When this priority
changes, due to any of the factors changing, the scheduler will move it
to a new priority queue and reschedule the CPU. Allowing it to have
control if it is now the highest priority task, or relinquish control if
it is no longer highest.

The intent in this system is to ensure that interactive threads are
serviced quickly. These are usually the threads that interact directly
with the user and cause user-perceivable latency. These threads usually
do little work and spend most of their time blocked awaiting another
user event. So they get the priority boost from unblocking while
background threads that do most of the processing receive the priority
penalty for using their entire timeslice.

#### CPU assignment and migration {#cpu-assignment-and-migration}

Threads are able to request which CPUs on which they wish to run using a
CPU affinity mask, a 32 bit mask where 0b001 is CPU 1, 0b100 is CPU 3,
and 0b101 is either CPU 1 or CPU 3. This mask is usually respected but
if the CPUs it requests are all inactive it will be assigned to another
CPU. Also notable, if it is “pinned” to a CPU, that is its mask contains
only one CPU, and that CPU becomes inactive the thread will sit
unserviced until that CPU becomes active again. See [CPU
activation](#cpu-activation) below for details.

When selecting a CPU for a thread the scheduler will choose, in order:

1.  The CPU doing the selection, if it is **idle** and in the affinity mask.

2.  The CPU the thread last ran on, if it is **idle** and in the affinity mask.

3.  Any **idle** CPU in the affinity mask.

4.  The CPU the thread last ran on, if it is active and in the affinity mask.

5.  The CPU doing the selection, if it is the only one in the affinity mask or
    all cpus in the mask are not active.

6.  Any active CPU in the affinity mask.

If the thread is running on a CPU not in its affinity mask (due to case
5 above) the scheduler will try to rectify this every time the thread is
preempted, yields, or voluntarily reschedules. Also if the thread
changes its affinity mask the scheduler may migrate it.

Every time a thread comes back from waiting on a shared resource or
sleeping and needs to be assigned a priority queue, the scheduler will
re-evaluate its CPU choice for the thread, using the above logic, and
may move it.

#### CPU activation {#cpu-activation}

When a CPU is being deactivated, that is shutdown and removed from the
system, the scheduler will transition all running threads onto other
CPUs. The only exception is threads that are “pinned”, that is they only
have the deactivating CPU in their affinity mask, these threads are put
back into the run queue where they will sit unserviced until the CPU is
reactivated.

When a CPU is reactivated it will service the waiting pinned threads and
threads that are running on non-Affinity CPUs should be migrated back
pretty quickly by their CPUs scheduler due to the above rules. There is
no active rebalancing of threads to the newly awakened CPU, but as it
should be idle more often, it should see some migration due to the logic
laid out above in [CPU assignment and
migration](#cpu-assignment-and-migration).

#### Realtime and idle threads {#realtime-and-idle-threads}

These are special threads that are treated a little differently.

The idle thread runs when no other threads are runnable. There is one on
each CPU and it lives outside of the priority queues, but effectively in
a priority queue of -1. It is used to track idle time and can be used by
platform implementations for a low power wait mode.

Realtime threads (marked with `THREAD_FLAG_REAL_TIME`) are allowed to run without
preemption and will run until they block, yield, or manually reschedule.
