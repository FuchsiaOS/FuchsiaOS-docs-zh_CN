# Pragmatic

Fuchsia is a production-grade operating system designed
to power consumer devices and products used for business-critical applications.
As such, Fuchsia is not a playground for experimental operating system concepts.
Instead, practical use cases arising from partner and product needs drive the
platform’s roadmap.

By prioritizing security, updatability, and performance,
our goal is to create an operating system
that meets the needs and expectations of developers, manufacturers, and
consumers.

Fuchsia may delay in pursuing new or experimental features
to ensure that Fuchsia is as good—or better—than the alternatives.

## A kernel that is practical, not minimal

**[Zircon](/docs/concepts/kernel/README.md)
is a pragmatic, message-passing kernel—not a microkernel**

Although Fuchsia applies many of the concepts popularized by microkernels,
Fuchsia does not strive for minimality.
For example, Fuchsia has over 170 syscalls,
which is vastly more than that of a typical microkernel.
Instead of minimality,
the system architecture is guided by practical concerns
about security, privacy, and performance.

## Fuchsia provides pathways for porting existing software

**[A POSIX-lite API](https://fuchsia.dev/reference/fidl/fuchsia.posix)
 eases porting costs;
 [the component framework](/docs/concepts/components/v2/capabilities/runners.md)
 encourages developers to bring their existing application runtimes**

There is a large ecosystem of software for existing platforms;
developers shouldn't need to rewrite everything from scratch.
Fuchsia supports a subset of POSIX to ease porting costs.
Furthermore, Fuchsia’s software model encourages developers
to bring their own runtimes in the form of component runners.
This allows developers to use their desired application frameworks
and re-use much of their application code.

## A flexible scheduler optimizes the system

**[Fair scheduling](/docs/concepts/kernel/fair_scheduler.md)
gives the system more flexibility to schedule work**

Increasing the choices available to the system scheduler gives the scheduler
the flexibility to optimize for power, throughput, or latency,
as appropriate for the situation.
At any given time, there are more threads in the system
that are ready to do useful work than there would be
if threads commonly blocked one another.

## On the roadmap

This section covers features on
[Fuchsia's roadmap](/docs/contribute/roadmap/index.md).

### Performance is a priority

**[Asynchronous communication](/docs/concepts/fidl/overview.md#messaging_models)
reduces latency**

Fuchsia makes heavy use of asynchronous communication,
which reduces latency by letting the sender proceed
without waiting for the receiver.
This is important for delivering software that can come and go
on a device as needed,
to account for network latency.

Fuchsia does not yet achieve its performance goals,
but this is an area under active development.
For example, performance related storage enhancements are on the
[project roadmap](/docs/contribute/roadmap/index.md).
