# Zircon Fair Scheduler

## Introduction

As part of the overall scheduler development effort, Zircon is moving to a new
fair scheduler as the primary scheduler for the system. This document discusses
the properties of the scheduler and how to enable it for testing prior to
roll-out.

## Enabling the Fair Scheduler

The fair scheduler is disabled by default. The new scheduler is enabled at
compile time by setting the GN build argument `enable_fair_scheduler` to true.

You can set this variable in your GN invocation like this:

```
gn gen build-zircon --args='enable_fair_scheduler=true'
```

## Detailed Scheduler Tracing

The new scheduler includes detailed tracing instrumentation to analyze the
behavior of the scheduler and its interaction with and impact on the competing
workloads in the system. Detailed tracing is enabled at compile time by setting
the GN build argument `detailed_scheduler_tracing` to true.

You can set this variable in your GN invocation like this:

```
gn gen build-zircon --args='enable_fair_scheduler=true detailed_scheduler_tracing=true'
```

Use the `kernel:sched` trace category to include the detailed scheduler
information in your trace session. It's a good idea to also include the
`kernel:irq` category because interrupts can cause scheduler activity that might
otherwise appear unconnected to other events.

```
fx traceutil record -categories kernel:sched,kernel:irq,<other categories> -stream -duration 4s -buffer-size 64
```

### Summary of Scheduler Events

The detailed scheduler events are primarily duration and flow events. The
events appear in Chromium Trace Viewer in the timelines labeled `cpu-0`
through `cpu-N`, where `N` is the number of CPUs in the system. These timelines
represent per-CPU activity in the kernel, which includes interrupts and
thread scheduling.

The fair scheduler emits duration events including the following:

* **sched_block**: The active thread blocks on a Zircon object, futex,
  kernel-internal lock.
* **sched_unblock**: The active thread unblocks another thread due to
  interacting with a Zircon object, futex, or kernel-internal lock.
* **sched_unblock_list**: A variation of **sched_block** when the action of the
  active thread may wake one or more threads at once (e.g.
  `wait_queue_wake_all`).
* **sched_yield**: The active thread called `zx_thread_yield`.
* **sched_preempt**: An interrupt requests re-evaluation of the run queue. This
  is due to either the time slice timer expiring, another CPU requesting a
  reschedule after modifying the CPU's run queue, or a hardware interrupt
  handler waking up a thread on the CPU.
* **sched_reschedule**: A kernel operation changed the run queue of the current
  CPU and a different thread _might_ need to run.

The fair scheduler emits flow events including the following:

* **sched_latency**: A flow that connects the point in time right after a thread
  enters the run queue to the point in time right before the (potentially
  different) target CPU context switches to the thread. This flow event is
  useful for visualizing cross-CPU scheduling activity and observing the
  runnable-to-running scheduler latency of a thread at any point in time.

A NOTE ABOUT FLOW EVENTS: Sometimes displaying flow events is not enabled by
default in Chromium Trace Viewer. Use the `View Options` menu in the upper right
corner of the page and make sure the `Flow events` checkbox is checked.

You can also disable flow events display if there are too many and the rendering
becomes too slow. Try zooming into a smaller region before enabling flow event
display for better performance in very large traces.

## Fair Scheduling Overview

Fair scheduling is a discipline that divides CPU bandwidth between competing
threads, such that each receives a weighted proportion of the CPU over time.
In this discipline, each thread is assigned a weight, which is somewhat similar
to a priority in other scheduling disciplines. Threads receive CPU time in
proportion to their weight, relative to the weights of other competing threads.
This proportional bandwidth distribution has useful properties that make fair
scheduling a good choice as the primary scheduling discipline in a general
purpose operating system.

Briefly, these properties are:

* **Intuitive bandwidth allocation mechanism**: A thread with twice the weight
  of another thread will receive approximately twice the CPU time, relative to
  the other thread over time. Whereas, a thread with the same weight as another
  will receive approximately the same CPU time, relative to the other thread
  over time.
* **Starvation free for all threads**: Proportional bandwidth division ensures
  that all competing threads receive CPU time in a timely manner, regardless of
  how low the thread weight is relative to other threads. Notably, this property
  prevents unbounded priority inversion.
* **Fair response to system overload**: When the system is overloaded, all
  threads share proportionally in the slowdown. Solving overload conditions is
  often simpler than managing complex priority interactions required in other
  scheduling disciplines.
* **Stability under evolving demands**: Adapts well to a wide range of workloads
  with minimal intervention compared to other scheduling disciplines.

A NOTE ABOUT DEADLINES: While fair scheduling is appropriate for the vast
majority of workloads, there are some tasks that require very specific timing
and/or do not adapt well to overload conditions. For example, these workloads
include low-latency audio / graphics, high-frequency sensors, and high-rate /
low-latency networking. These specialized tasks are better served with a
deadline scheduler, which is planned for later in the Zircon scheduler
development cycle.

## Fair Scheduling in Zircon

The Zircon fair scheduler is based primarily on the Weighted Fair Queuing (WFQ)
discipline, with insights from other similar queuing and scheduling disciplines.
Adopting aspects of the Worst-Case Fair Weighted Fair Queuing (WF2Q) discipline,
a modification of WFQ, is planned to improve control over tuning of latency
versus throughput.

The following subsections outline the algorithm as implemented in Zircon. From
here on, "fair scheduler" and "Zircon fair scheduler" are used interchangeably.

### Ordering Thread Execution

One of the primary jobs of the scheduler is to decide which order to execute
competing threads on the CPU. The fair scheduler makes these decisions
separately on each CPU. Essentially, each CPU runs a separate instance of the
scheduler and manages its own run queue.

In this approach, a thread may compete only on one CPU at a time. A thread can
be in one of three states: _ready_, _running_ or _blocked_ (other states are not
relevant to this discussion.) For each CPU, at most one thread is in the
_running_ state at any time: this thread executes on the CPU, all other
competing threads await execution in the _ready_ state, while blocked threads
are not in competition. The threads in the _ready_ state are enqueued in the
CPU's run queue; the order of threads in the run queue determines which thread
runs next.

The fair scheduler, unlike **O(1)** scheduling disciplines such as priority
round-robin (RR), uses an ordering criteria to compare and order threads in the
run queue. This is implemented using a balanced binary tree, and means that
scheduling decisions generally cost **O(log n)** to perform. While this is more
expensive than an **O(1)** scheduler, the result is a near-optimal worst case
delay bound (queuing time) for all competing threads.

### Ordering Criteria

Two concepts are used to order threads in the run queue: _virtual timeline_ and
per-thread _normalized rate_. The _virtual timeline_ tracks when each thread in
the run queue would finish a _normalized time slice_ if it ran to completion.
A _normalized time slice_ is proportional to the thread's _normalized rate_,
which in turn is inversely proportional to the thread's weight. Threads are
ordered in the run queue by ascending _finish time_ in the _virtual timeline_.

The inverse proportional relationship to weight causes higher weighed threads
to be inserted closer to the front of the run queue than lower weighted threads
with similar arrival times. However, this is bounded over time: the longer a
thread waits in the run queue, the less likely a newly arriving thread, however
highly weighted, will be inserted before it. This property is key to the
fairness of the scheduler.

The following sections define the scheduler in more precise terms.

### Per-Thread Scheduling State

For each thread **P[i]** we define the following state:

* Weight **w[i]**: Real number representing the relative weight of the thread.
* Start Time **s[i]**: The start time of the thread in the CPU's virtual
  timeline.
* Finish Time **f[i]**: The finish time of the thread in the CPU's virtual
  timeline.
* Time Slice **t[i]**: The size of the time slice for the current period.

### Per-CPU Scheduling State

For each CPU **C[j]** we define the following state:

* Number of Threads **n[j]**: The number of runnable threads competing on the
  CPU.
* Scheduling Period **p[j]**: The period in which all competing threads on the
  CPU execute approximately once.
* Total Weight **W[j]**: The sum of the weights of the threads competing on the
  CPU.

When a thread enters competition for a CPU, its weight is added to the CPU's
total weight. Likewise, when a thread blocks or is migrated to another CPU the
thread's weight is subtracted from the CPU's total weight. The total includes
the weights of the _ready_ threads and the current _running_ thread.

### Tunable State

We define the following tunable state, which may either be global or per-CPU:

* Minimum Granularity **M**: The smallest time slice allocated to any thread.
* Target Latency **L**: The target scheduling period for the CPU unless there
  are too many threads to give each thread as least one minimum granularity time
  slice.

### Definitions

We define the following relationships for the key scheduler variables:

#### Scheduling Period

The scheduling period controls the size of time slices. When there are few
threads competing, the scheduling period defaults to the _target latency_. This
results in larger time slices and fewer preemptions, improving throughput and
potentially power consumption. When the number of threads is too large the
scheduling period stretches such that each task receives at least the _minimum
granularity_ time slice.

Let **N** be the maximum number of competing threads before period stretching.

**N** = floor(**L** / **M**)

**p[j]** = **n[j]** > **N** --> **M** * **n[j]**, **L**

#### Virtual Timeline

When a thread enters the run queue, either by newly joining the competition for
the CPU or completing a time slice, the thread's _virtual_ start and finish time
are computed. As the current fair scheduler is based on WFQ, the finish time is
used to select the position for the thread in the run queue relative to other
threads. Later, when WF2Q is implemented, both the start and finish time are
considered.

Some WFQ implementations use the thread's actual time slice to calculate the
_normalized time slice_ for the timeline. However, the actual time slice depends
on the total weight of the CPU (see below), a value that changes as threads enter
competition. The Zircon fair scheduler instead uses the scheduling period as an
idealized uniform time slice for the _virtual timeline_, because its value
changes less dramatically. Using a uniform value for all threads avoids skewing
the _virtual timeline_ unfairly in favor threads that join early.

Let **T** be the system time of CPU **C[j]** when thread **P[i]** enters the run
queue.

**s[i]** = **T**

**f[i]** = **s[i]** + **p[j]** / **w[i]**

### Time Slice

When a thread is selected to run, its time slice is calculated based on its
relative rate and the scheduling period.

Let **g** be the integer number of _minimum granularity_ units **M** in the
current _scheduling period_ **p[j]** of CPU **C[j]**.

Let **R** be the relative rate of thread **P[i]**.

**g** = floor(**p[j]** / **M**)

**R** = **w[i]** / **W[j]**

**t[i]** = ceil(**g** * **R**) * **M**

This definition ensures that **t[i]** is an integer multiple of the _minimum
granularity_ **M**, while remaining approximately proportional to the relative
rate of the thread.

### Yield

Yielding immediately expires the thread's time slice and returns it to the run
queue. This behavior is similar to yielding in **O(1)** scheduling: the yielding
thread is guaranteed to queue behind threads of the same or greater weight.
However, the yielding thread may or may not skip ahead of lower weight threads,
depending on how long other threads have been waiting to run.
