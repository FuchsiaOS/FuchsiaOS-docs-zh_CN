# UTC Architecture

## Introduction

Time synchronization for Fuchsia must be flexible: different products built on
Fuchsia must be able to use different sources of time and these sources must be
able to be reconfigured or replaced as these products evolve.

This page defines the basic architecture we use to provide this flexibility: the
components that are involved, the roles and responsibilities of these
components, and the interactions between them.

## Architecture

The following diagram illustrates the components involved in time
synchronization and the basic relationships between them.

![This figure shows the Fuchsia UTC architecture.](images/utc_architecture.png)

### Kernel

The kernel defines the concept of a
[clock object](reference/kernel_objects/clock.md) that may be used to
track the progress of time. Each clock object is a one dimensional affine
transformation of the clock monotonic reference timeline, which may be adjusted
by a userspace component (the "clock maintainer"), and observed by many other
userspace components (the “clients”).

### Component Manager

Component Manager is responsible for creating the UTC clock and distributing it
to other userspace components.

Component Manager creates a kernel clock object for UTC time, setting the
"backstop time" to the time of the last CL included in the build from by reading
an output of the build process. Note that Component Manager does not start this
UTC clock.

Component Manager passes a read-only handle to the UTC clock to the component
instances it starts and exposes the
[`fuchsia.time.Maintenance`](https://fuchsia.dev/reference/fidl/fuchsia.time#Maintenance)
service to distribute a read/write handle to the UTC clock. In a production
system Timekeeper should be the only component with access to this service.

### Time Clients

Time clients are the users of UTC time on a Fuchsia device. Fuchsia’s
implementation of libc uses the clock handle supplied by component manager to
read UTC time, and therefore any component instance may act as a time client by
using the standard time API provided by its runtime. See
[language support](../language_support.md) for further details.

Components that need deeper insight into the state of time synchronization may
acquire the UTC clock handle and use it to call
[`zx_clock_get_details`](reference/syscalls/clock_get_details.md) or wait
on the `ZX_CLOCK_STARTED` signal.

### Timekeeper

Timekeeper is responsible for starting and maintaining the
UTC clock in line with the sources and policy defined by the product.

Timekeeper connects to the
[`fuchsia.time.Maintenance`](https://fuchsia.dev/reference/fidl/fuchsia.time#Maintenance)
service to acquire a writable handle to the UTC clock on launch. It launches and
connects to the time source component(s) configured by the product and connects
to
[`fuchsia.time.external.PushSource`](https://fuchsia.dev/reference/fidl/fuchsia.time.external#PushSource)
or
[`fuchsia.time.external.PullSource`](https://fuchsia.dev/reference/fidl/fuchsia.time.external#PullSource)
to receive time synchronization information from each of these sources. *Note:
As of Q4 2020 time sources are hardcoded in Timekeeper and cannot yet be
configured by product. PullSource is not yet supported.*

For a particular product, each time source is configured to play
one of four different roles:

1. **Primary**: A primary source is used to maintain the UTC clock whenever it is
   both available and consistent with any gating source (i.e. the time reported
   by the primary source and gating source are within some upper bound).
2. **Fallback**: A fallback source is used to maintain the UTC clock when the
   primary source is unavailable or inconsistent, provided the fallback source
   is both available and consistent with any gating source.
3. **Gating**: A gating source is used to provide a validity check on another
   (usually more accurate but less trusted) time source. The gating source is
   used to determine whether a primary or fallback source should be used. When
   neither a primary nor fallback source are available (or when they are
   available but inconsistent) a gating source may be used to maintain the UTC
   clock.
4. **Monitor**: A monitor source is never used to maintain the UTC clock.
   Metrics are recorded that allow the performance of the monitor source to be
   assessed, making monitors a way to safely test or "dark launch" new or
   modified algorithms.

Although this is a flexible system that could support many sources, we do not
expect most products will require more than two different time sources, with
Primary-only, Primary+Fallback, Primary+Gating, and Primary+Monitor being the
common configurations. *Note: As of Q4 2020 only Primary and Monitor sources
are supported*.

Timekeeper has sole discretion in applying the information from time sources to
the UTC clock. It may discard updates that have a significant uncertainty or
that appear to be outliers. When a significant clock correction needs to be
applied Timekeeper must balance three conflicting desires:

1. Step changes in time should be avoided if possible.
2. The frequency adjustment used to apply a correction by slewing should not be
   excessive.
3. The time taken to apply a correction by slewing should not be excessive.

Timekeeper is the central authority for performance of the hardware oscillator
on a device. It tracks the observed oscillator error and applies UTC corrections
to accommodate for this error. Timekeeper uses stash to store oscillator error
across power cycles. *Note: As of Q4 2020 the full frequency correction
algorithm is not yet implemented*.

On devices that have a real time clock (RTC), Timekeeper is responsible for
reading the RTC during initialization and periodically updating the RTC to
reflect the UTC clock.

### Time Sources

Each time source component is responsible for supplying information that
could be used to synchronize UTC based on one or more remote sources that the
time source communicates with. Each time source component exposes either the
[`fuchsia.time.external.PushSource`](https://fuchsia.dev/reference/fidl/fuchsia.time.external#PushSource)
or
[`fuchsia.time.external.PullSource`](https://fuchsia.dev/reference/fidl/fuchsia.time.external#PullSource)
service (or both).

Many time sources communicate with servers over a network using some protocol
capable of time synchronization, such as NTP, Roughtime, or HTTPS (see
[HTTPSDate time source](/src/sys/time/httpsdate_time_source)). Some time sources
could instead communicate with local hardware such as a GPS receiver or VLF
receiver.

A time source component encapsulates knowledge of the
protocol used to interact with the remote source and is responsible for
following the rules of this protocol. This means that where multiple remote
sources use the same protocol, a single time source component instance is
usually responsible for communicating with all of them and implementing any
cross-remote-source requirements in the protocol.

Each time source should be independent of the others, providing flexibility in
how time sources are installed and avoiding complex failure modes caused by the
interaction between multiple time sources. This means a time source should never
use system UTC directly in its implementation (because the system UTC may have
incorporated inputs from other time sources). Instead, all time sources use the
system monotonic clock for their reference time.

## Interfaces

### Time Source to Timekeeper

Time sources supply time updates to Timekeeper.

Each time update is expressed as a correspondence pair between the monotonic
clock at the time the update was most valid and the UTC determined by the
time source. Sending updates as a correspondence pair rather than an absolute
time means the FIDL latency between the time source and Timekeeper does not
directly impact accuracy, and clearly communicates the monotonic time at which
the update was most valid. In addition to the correspondence pair, the time
source also sends a standard deviation to convey the uncertainty associated
with the update.

Time sources may support two different modes of operation:

* In **push mode** the time source determines when a time update should be
  generated and autonomously sends these updates to Timekeeper.
* In **pull mode** Timekeeper determines when a time update should be generated
  and requests these updates from the time source.

Push mode is preferred in nearly all circumstances since the time source has
better knowledge of timing constraints in the protocol, remote resource
utilization, and the availability of any dependencies, letting it make more
appropriate decisions. Pull mode may be appropriate for trivial time sources or
when a time source is used very infrequently (e.g. every few hours) as a gating
source. When pull mode is used a time source may reject time update requests,
for example if a request would violate a maximum rate constraint in the
protocol. For time sources that support both modes, Timekeeper determines which
mode to use and connects to the corresponding service. A time source may choose
to generate updates less frequently or not at all when no connections are open
to these services.

In push mode Timekeeper has no knowledge of when a time update should occur so
cannot infer a failure from the absence of a successful update: in push time
mode a time source also provides its overall health state, letting Timekeeper
make better choices about when to switch time sources.

### Timekeeper to Time Source

Timekeeper supplies global information that may aid a time source in generating
time updates, including:

* The frequency tolerance of the oscillator.
* The observed oscillator frequency.

Providing these data ensures time sources do not need to duplicate Timekeeper
functionality. *As of Q4 2020 no time sources have required these data so the
API has not yet been defined*.

Timekeeper does not provide information about the global synchronization state
of the system, or about whether updates from a particular source will actually
be used to maintain the system clock. This intentional decision avoids creating
feedback loops between time sources and ensures that a time source will behave
consistently across roles.
