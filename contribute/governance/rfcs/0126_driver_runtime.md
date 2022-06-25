<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0126" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC establishes the design by which drivers colocated within a process will
communicate with each other. Drivers will communicate via an in-process runtime,
modeled after the zircon kernel. The runtime will provide primitives similar to
the zircon channel and port, and a new FIDL transport will be built on top of
this runtime, allowing for better performance than that which can be achieved
via zircon channels and ports. This new runtime and FIDL transport will
supersede the existing banjo runtime used by drivers today.

Communication between drivers which are not co-located in the same process will
continue to use zircon channel based FIDL. Making the underlying transport
transparent to the driver is considered out of scope of this proposal and will
be visited in a future proposal.

The RFC also establishes a set of rules for a threading model drivers should
abide by in order to efficiently share threads with one another. Serving both
messages that originate from drivers in process as well as drivers out of
process on the same threads will be possible.

## Motivation

The existing driver runtime was something that solved the requirements of the
problems at the time it was created. FIDL didn't exist yet, drivers were written
almost ubiquitously in C, and the zircon kernel had yet to be well optimized. As
a result, the interfaces drivers used to communicate with each other started out
as a function table with an associated type erased context pointer. At the point
at which C++ drivers were becoming more commonly written, FIDL had been created,
but it was not considered up to the task of inter-driver communication due to
perceived high overhead of the binding implementations. In an attempt to improve
ergonomics of C++ drivers, wrappers were being manually generated around the C
protocol definitions. To reduce the toil of needing to maintain both the C
protocol definitions and C++ wrappers, a new IDL, with a syntax largely
mirroring that of FIDL at the time, was created which could auto-generated both
the C and C++ code from a common source of truth. This later became known as
banjo. While banjo did have some initial investment, it quickly entered
maintenance mode, and has seen little improvement over the years compared to
FIDL, which has flourished over a similar time period. Banjo is no longer
meeting the current requirements, and so a re-imagining of the driver runtime is
necessary.

This design is motivated by a number of distinct problems, and wishes to address
them in one go.

### Stable ABI

The most important problem the driver framework is working on is enabling a
stable driver SDK. This is a platform level goal, which will help Fuchsia
achieve wide hardware support, and ensure we meet our aspirations of updating
the various parts of the OS without losing functionality. Banjo, the current
solution for inter-driver communication, was built without API and ABI evolution
in mind. Instead, it optimizes for simplicity and low overhead. As a result
there is currently no good mechanism for making modifications to banjo libraries
without breaking all clients and servers that depend on it. This will make it
very challenging to evolve our platform while meeting our goals.

### Thread Safety

Today, when authoring a driver, it is very challenging to implement a banjo
protocol without needing to spawn a dedicated thread in your driver to handle
requests. This is because there are no rules by which drivers utilizing those
protocols must follow when invoking methods on the protocol. When handling an
incoming call, you need to ensure your logic:

*   Deals with synchronization issues as the client may call from several
    threads, possibly in parallel.
*   Handles reentrancy.

The obvious way to handle the first issue is by acquiring a lock. However,
simply acquiring a lock can easily lead to a deadlock as your driver may call
back out into another driver, and in that same stack frame, it may be called
back into an attempt to re-acquire that same lock. All lock implementations used
within drivers today are not reentrant safe, and we do not wish to start using
recursive locks for reasons explored in more depth in the
[alternatives section](#recursive).

As a consequence, the only way to correctly handle this today is by pushing work
onto a queue and servicing that queue later. Because the current driver runtime
provides no mechanisms to schedule work to occur later, the driver must
instantiate its own thread to service this queue.

This is problematic because it destroys much of the expected performance benefit
of colocating drivers in the same process. Additionally, there is a large tax in
terms of driver implementation complexity. Many drivers on the system do not
choose to push work onto queues, and as such are likely one new client away from
being used in a way that will inevitably end up deadlocking the system. We have
a storied history of such bugs lurking in our drivers, and only surfacing in a
flakey manner due to reentrancy only occurring on error conditions that rarely
occur.

### Boilerplate Serialization

An additional problem motivating the new runtime is the lack of congruence with
communication with non-drivers. It is typical for a driver to implement a FIDL
service, exposed to non-driver components, and handle requests by forwarding the
request, often with a limited amount of translation, to another driver. This
process is fraught with boilerplate code as while the definitions for the types
(structs, enums, etc) and even protocols may be shared between zircon channel
transport FIDL and banjo transport FIDL, the generated types are completely
inconsistent. As a result, the driver author must implement logic to serialize
from one set of generated types to another.

This particular issue has often been cited as one of the largest ergonomic pain
points of writing a driver today. The problem used to be worse because it only
recently became possible to share types between different FIDL transports, and
banjo itself only became a FIDL transport recently as well. Previously, the
definitions for the types could easily go out of sync with respect to each
other, causing subtle bugs. Those bugs can still occur if the manual
serialization logic is not correctly updated when the definitions change.

## Stakeholders

*Facilitator:* abarth@google.com

*Reviewers:* abarth@google.com (FEC member), abdulla@google.com (FDF),
yifeit@google.com (FIDL), eieio@google.com (scheduling/Zircon)

*Consulted:* Members of drivers team, networking team, storage team, kernel
team, FIDL team, and performance team.

*Socialization:* A draft of this RFC was sent to the FEC discuss mailing list
for comment. Earlier forms of the concepts found in this design were circulated
amongst driver framework, FIDL, and Zircon teams.

## Design

In more concrete terms, the design will be realized by a set of new primitives,
and a new FIDL transport which takes advantage of these primitives. Before
getting into more details about the design, we first explore some of the
additional requirements which we used to arrive at this design.

### Requirements

#### Performance

One of the high level goals of the new driver runtime is to achieve a higher
degree of performance that is achievable by utilizing the driver runtime. The
performance requirements for drivers vary widely, but broadly speaking we aim to
optimize for the following metrics:

*   High total throughput
*   High I/O operations per second (IOPS)
*   Low latency
*   Low CPU utilization

For instance, we would like to maximize throughput of an NVMe SSD, or ensure
minimal latency occurs between an input event occurring and software being
informed of the event. While to some degree, these tasks require much more of
the system working in conjunction than just the driver framework to meet the
desired performance levels, we want to ensure that the driver runtime does not
become a bottleneck.

Much of the design focuses on optimizations we can employ to improve the
performance of inter-driver communication beyond what is achievable via zircon
channel based FIDL. If we are not able to outperform zircon channel based FIDL
on all of the aforementioned metrics, it will likely not be a suitable
replacement for the current banjo runtime. The need to outperform zircon channel
based FIDL is discussed in a [later section](#zircon-channel-perf).

#### Driver Author Ergonomics {#ergo-req}

Another high level goal for the new driver runtime is to ensure that drivers
remain simple to author. Maximizing performance without regard to how driver
authors are impacted by design decisions would not lead to a useful result.
Providing options where authors can choose between ergonomics or zero cost
overhead will be essential toward realizing our goals. Additionally, being able
to evolve a driver written from a simple implementation to a more performance
one without necessitating a rewrite is ideal.

#### Security and Resilience {#security}

Drivers which reside in the same process share a security boundary. This is
because, despite all attempts to isolate drivers, we can not actually deny their
ability to access memory regions belonging to other drivers in the shared
process. This is intentional as the shared address space is one of the key
benefits of colocation that we can exploit to improve performance beyond that
which channels may offer. As a result, we need to assume that any reasonably
well equipped malicious entity which gains control of one driver in the driver
host will be able to access all capabilities of all drivers in that same
process. As a result, we need not consider implementing security checks within
the driver runtime layer with the aim of providing additional security benefits.

However, this does not mean we shouldn't still employ many of the same
mechanisms typically used for providing security guarantees, as they may improve
resilience of individual drivers. For instance, rather than blindly assuming a
pointer points to data within a buffer, we can still validate that is the case.
This will help limit damage inflicted by a bug, aiding in root causing the
issue. Said another way, it is still fruitful to protect against mistakes. The
rest of the design should carry this in mind.

#### Zero Copy Option

Channel based fidl, even under most optimal conditions, results in at least 2
copies. One copy from userspace into the kernel (via `zx_channel_write`), and
then another from the kernel into userspace (via `zx_channel_read`). More often
than not, additional copies occur due to a linearization step in the client, as
well as choosing to handle the request asynchronously in the server, or moving
data into idiomatic language built-in types, resulting in 4 copies total.
Because drivers utilizing the new driver runtime will exist in the same process,
and therefore address space, it is possible to make the optimal case require 0
copies, and the ergonomic case to result in only 1 copy.

Most data passed between drivers, similar to channel based FIDL, should be
control data structures, with the data-plane data being stored in VMOs. This
allows us to mitigate the most expensive copies. However, control data
structures can add up in size, reaching several kilobytes in certain
circumstances (eg network packet batch requests). In addition, in several driver
stacks, data is mostly untouched, and passed through several layers. We can
elide all of these copies. In fact, in the current banjo based runtime, we go
out of our way to achieve zero copy, however the means by which we achieve it is
largely problematic, and bug prone. It is one of the many things preventing us
from embracing rust driver development.

It is noteworthy to mention that the copies in FIDL are usually not considered a
performance bottleneck as syscall overhead dominates. That said, in the driver
runtime we are avoiding syscalls and therefore the copies are expected to become
a considerable portion of time spent sending messages between drivers.

From a performance perspective, these copies likely will not result in
meaningful bottlenecks in the system today. They may have a measurable impact on
CPU utilization, however, and for that reason, providing the ability to reach
zero copy is considered useful.

### Push vs Pull

Zircon channels work on a pull based mechanism. Userspace registers for signals
on a channel to know when it has data to be read, and once notified, reads from
the channel into a buffer it provides. FIDL bindings often invert this mechanism
to be push based instead. A callback is registered, and the callback is
triggered when something is ready for it.

We have a great deal of flexibility to either move towards a completely push
based mechanism, or continue emulating the zircon channel mechanism. For the
average user, push is a great deal more ergonomic, and possibly more performant
as the pull model requires re-entering the runtime after being signaled. In
other systems which opt to be more performant, such as [Windows IOCP][iocp], and
[Linux io\_uring][iouring], buffers are often pre-registered, to achieve greater
performance, by removing that extra entry into the kernel, as well as a copy.
For an in-process runtime, entering the runtime is cheap, and it's not actually
necessary to pre-register a buffer to achieve zero-copy, so introducing
differences in our API from that of the zircon channel is not necessarily a win.
Additionally, rust as a language relies heavily on the pull based mechanism, and
would likely have an impedance mismatch with it's model if we chose to shift
away from pull at the transport level.

### Primitives

As alluded to earlier, the new design will require new primitives to enable a
new FIDL transport to be built. The primitives themselves are largely modeled
after primitives provided by the zircon kernel.

The API/ABI of the primitives will be C based, similar to the existing libdriver
API. We will provide per language wrappers for more idiomatic usage. While it
may be possible to define the API via FIDL, similar to how the zircon syscall
interface is defined in FIDL, it is likely not worth the effort as the overall
set of APIs should remain quite minimal.

#### Arena {#arena}

The first primitive is an arena. In order to reduce the number of copies, we
need a way to ensure that the lifetime of data associated with the request is at
least as long as the request is outstanding. The data provided to the transport
will necessarily be backed by a buffer. In channel based FIDL bindings, inbound
message data often starts on the stack, and may be moved into a heap allocation
on-demand if the request needs to be replied to asynchronously. It is possible
for FIDL bindings to read directly into heap memory, but at the time of writing,
that doesn't happen. If, instead, we back the data by a runtime provided arena,
we can tie the lifetime of the request to an arena. We can also enable
additional allocations to be made against the arena and guarantee that they will
live as long as the request. When handling a request, it is normal to either
forward it to another driver, or make a request to a downstream driver; in these
cases the same arena can be repurposed and passed alongside the new request. It
may be feasible via such a scheme to have an operation navigate many drivers
without ever needing to make a global allocation. In driver stacks such as the
block stack, it is typical to navigate 6 or more drivers in the same driver
host, mostly forwarding the same request over and over to lower level drivers,
making minimal modifications to the data.

The runtime will provide APIs for creating arena, decrementing references to
them, performing allocations, as well as checking whether a provided memory
region is backed by the arena. This last API is unusual, but necessary for
robustness in the FIDL transport implementation. An API freeing individual
allocations is possible to add later, but the initial implementation will skip
it. Freeing allocations will only occur when the arena itself is destroyed,
which occurs when all references to the arena are removed.

The runtime is free to optimize the arena as it deems necessary. For instance,
the base implementation may choose to simply forward all allocation requests to
the global malloc, and wait until the arena is destroyed before issuing free for
every allocation which was incurred. Alternatively, it may be implemented as a
single allocation bump arena, with a max size which adapts to maximum per
request allocations required. Over time we can possibly expand on the arena
interface to allow the clients to provide hints about what strategy the
underlying arena should utilize as the same arena strategy may not be optimal in
all driver stacks.

#### Channel

The second primitive is a channel primitive. It will have an interface almost
identical to one zircon channels have. The biggest differences include:

*   The write API will take in an arena object, incrementing its reference
    count.
*   The write API will require all buffers passed into it (data + handle table)
    be backed by the arena. As a corollary, the runtime will take ownership of
    these buffers (as well as all others backed by the arena).
*   The read API will provide an arena which the caller takes ownership of that
    reference.
*   The read API will provide the buffers for data and the handle table, rather
    than expecting the caller to provide buffers which are copied into.

These differences are necessary to allow the FIDL bindings built on top to
achieve zero copy.

Similarities between zircon channels include:

*   Channels are created in pairs.
*   Channels may not be duplicated, and as such are bi-directional single
    producer single consumer queues.

#### Dispatcher

The last primitive is something akin to the zircon port object. Rather than
provide a mechanism for blocking the current thread, instead, it will provide a
mechanism to indicate via callback which registered channel is currently
readable or closed. A mechanism for registering driver runtime channels will
also be available.

Drivers will have the ability to create multiple dispatchers, similar to their
ability to create multiple ports. Dispatchers are the main agent by which the
[threading model mentioned in a later section](#threading-model) is implemented.
When creating a dispatcher, it will be required to specify which of the
threading modes that dispatcher should operate under.

Additionally there will be an API for receiving an `async_dispatcher_t*` from
the driver runtime dispatcher object, enabling waiting for zircon objects and
posting tasks. The callbacks registered against this dispatcher will conform to
the same threading guarantees that runtime channels will be subject to.

#### Example

While we avoid specifying the exact interface for the primitives, it may be
useful while evaluating the design to see strawman example of what it could look
like:

https://fuchsia-review.googlesource.com/c/fuchsia/+/549562

### Threading Model {#threading-model}

Other than sharing the same address space, the second major benefit of colocated
drivers is the fact that it is possible to make better scheduling decisions than
afforded by the kernel. Rather than provide each driver with its own thread, we
want to exploit the ability for drivers to share the same thread(s). When
multiple independent pieces of code share the same thread, they must abide by
some sort of contract to ensure correctness.

A threading model sets rules and expectations which driver authors can leverage
to ensure correctness. If done correctly, the threading model can drastically
simplify the development experience. Driver developers who do not wish to deal
with concurrency, synchronization, and reentrancy should have mechanisms that
allow for them to ignore these concerns, while still being able to write
functional code. At the other extreme, the threading model must offer enough
freedom for drivers to hit the performance goals expressed earlier in this
document.

To that end, we propose a threading model which provides the following two high
level modes for drivers to operate under:

*   Concurrent and synchronized.
*   Concurrent and unsynchronized.

It may be useful to start by defining the terms used:

*   **Concurrent**: Multiple operations may be interleaved with respect to one
    another.
*   **Synchronized**: At any given moment in time, no two hardware threads will
    be inside the driver at the same time. All calls will be ordered with
    respect to each other.
*   **Unsynchronized**: At any given moment in time, multiple hardware threads
    may be inside the driver at the same time. There are no guarantees of
    ordering in which incoming calls will occur.

Synchronized does not mean that work will be pinned to a single zircon thread.
The runtime is free to migrate the driver to any thread it manages, as long as
it provides guarantees of synchronization. As such, usage of thread local
storage (TLS) is not safe while under the synchronized mode. In its stead, the
driver runtime may offer an alternative API. It may be helpful to view the
synchronized mode as a "virtual" thread or fiber.

The unsynchronized mode allows drivers the greatest degree of flexibility. At
the cost of extra work, the driver may utilize more granular locking schemes,
enabling higher degrees of performance. Drivers opting into this mode have a
higher risk of deadlocking the process, and as a result we may place
restrictions in the future on drivers choosing to opt into this mode.

These modes will be defined at runtime and associated with the driver runtime
dispatcher objects described earlier. Because it is possible to have multiple
dispatchers, it is also possible to mix and match these modes in a single driver
to manage concurrency requirements. For instance, it may be possible to create
multiple serial, synchronized dispatchers, to deal with two pieces of hardware
which need not be synchronized with respect to each other. In practice, it is
expected that drivers will use a single dispatcher, and opt to utilize language
specific constructs to manage additional concurrency and synchronization
requirements. This is because language specific constructs, such as
`fpromise::promise` in C++ or `std::future` in rust allow for even better
scheduling decisions to be made than the driver runtime can accomplish with just
multiple dispatchers as dependencies are more granularly tracked. Drivers
written in C, because it lacks any sort of canonical concurrency management
primitives or libraries, is the most likely language to utilize the driver
runtime primitives to manage concurrency.

#### Implications for Inter-driver Communication

Because the primitives described above do not provide any mechanisms by which a
driver may call into another driver, all calls must be mediated by the runtime.
When a driver runtime channel is written to, rather than automatically queueing
up the work, we can instead, in the same stack frame, choose to call into
another driver. The runtime can make the determination of whether to call into
the driver which owns the other end of the channel by checking what threading
mode the dispatcher the other end of the channel is registered to is set to. If
it is unsynchronized, it will always call into the other driver in the same
stack frame. If the dispatcher is synchronized and concurrent, the runtime can
try and acquire the dispatcher's lock, and if successful, call into the driver
in the same stack frame. If it is unsuccessful, it can queue up the work to be
handled later.

This may seem like a trivial optimization, but preliminary benchmarking shows
that avoiding the need to return to the async loop can have large improvements
on both latency as well as CPU utilization.

#### Reentrancy

In all modes described, reentrancy will not be something drivers must deal with.
The fact that the runtime mediates all interactions between drivers also affords
us the ability to determine whether in the current call stack, the driver we
wish to enter has already been entered. If it has, rather than call directly
into the driver, the runtime will instead queue up the work into a queue, which
will be serviced by another zircon thread or the same thread once it has
returned back to the original runtime loop.

#### Blocking

Note: Blocking here refers to the act of making a syscall which yields the
current thread's time slice, e.g. `zx_channel_call`, `zx_object_wait_one`,
`zx_port_wait`, etc. Other actions that result in "blocking" such as accessing a
page which is not wired, or attempting to acquire a futex, are not considered
blocking here.

In all modes discussed previously, there has been an implicit requirement that
drivers participating in shared threads do not block. There may be valid reasons
to want to block threads. In order to allow for these use cases, when creating a
dispatcher, in addition to setting the threading mode the dispatcher will
utilize, the driver may specify whether it will need to be able to block when
called into. This will allow the runtime to determine the minimum number of
threads it must create to avoid the risk of deadlocks. For instance if a driver
creates N dispatchers, all which may block, then the runtime will need to
allocate a minimum N+1 threads to service the various dispatchers.

The ability to specify block in dispatcher which is also unsynchronized will not
be supported initially, but given a valid use case, we can re-evaluate this
decision. The reason being that the simple N+1 thread strategy described above
won't work as an unknown number of threads may have entered that driver and they
may all block.

#### Work Priority / Thread Profiles

Some work has a higher priority than others. While being able to assign priority
and inherit priority at a work level, this not something which is easily done.
Rather than try to innovate in this space beyond what the zircon kernel
currently provides, we will offer priority as a dispatcher level concept. A
profile can be specified when creating a dispatcher. While the runtime will not
necessarily guarantee that all callbacks will occur on a zircon thread
configured to run with that profile, it will at least spawn one zircon thread
per zircon thread profile provided.

There is a lot of room to explore here in terms of how the runtime can optimally
handle thread profiles, and likely some level of collaboration with the zircon
scheduler team and driver authors will be necessary to arrive at a solution that
meets the needs of the use cases thread profiles are useful for.

### FIDL Bindings

Rather than completely re-imagine FIDL, the goal of bindings which target the
driver runtime transport will be able to make the minimal set of user facing
changes necessary to realize the advantages that the driver runtime provides.
The expectation is that each language binding which currently targets zircon
channels which is also supported for writing drivers would be forked to provide
a variant which also targets the driver runtime transport. For the initial
implementation the LLCPP bindings will be utilized as C++ is the primary
language used for authoring drivers, and will likely remain the most well
supported language for a while to come. The rest of the section will assume the
LLCPP bindings when making specific remarks about FIDL bindings.

Non-protocol types are expected to be used in their entirety with no
modifications. This is important as being able to share generated types between
multiple transports is an important property driver authors desire.

Classes and structs generated for protocols, also known as the messaging layer,
will necessarily be re-generated. It is possible that we could template them to
conditionally take advantage of the driver runtime APIs, but that would require
that we generate support for both the channel and driver runtime transports for
every protocol. Another option which has been suggested is creating a minimal
class abstracting the channel read and write APIs for the underlying transport.
This unfortunately would not work for our design as the arena would be an
awkward concept in channel based FIDL, and the buffer ownership relationships
for the read and write APIs are incongruent between the two transports. Some
level of collaboration with the team(s) maintaining the various bindings is
expected to determine the appropriate level of code-reuse and divergence and
bindings.

#### Per Request arena

As mentioned in [an earlier section](#arena), the driver runtime will provide an
arena object, which will be passed alongside messages sent through channels. The
FIDL bindings are expected to take advantage of this arena to achieve zero copy.

Users may opt to not utilize the arena for creating structures sent to other
drivers. This is fine, the bindings can ensure types are copied into buffers
backed by a runtime provided arena allocator. The improved ergonomics of not
specifying an arena will necessarily result in a copy. The only design choice we
may opt for here is to make it very obvious that the copy is occurring, so that
the copy doesn't occur out of ignorance.

Users may prefer to allocate all of their objects on the stack, hoping that they
are used completely within the same stackframe as the call itself, something
which may be possible in certain scenarios depending on threading constraints.
In fact, this is something that drivers do today with banjo. While this is
something we could possibly support by lazily moving types into being backed by
the arena when we deem it necessary to move the type into the heap (via
something similar to a `ToAsync()` call in the receiver), we will avoid
supporting this. Instead, the same logic which causes the message to be copied
into an arena backed buffer as described above should be utilized. We can
potentially revisit the idea later if a strong use case comes up.

#### Message Validation

Currently, when drivers communicate with each other, we take no actions to
validate that the messages meet the required contract specified by the interface
they are using by performing actions such as enum validation nor zircon handle
validation. In the new runtime, we can likely start performing some of this
validation, but we will need to determine which validation we perform on a per
validation feature basis. For instance, enum validation is likely cheap and
unlikely to result in any measurable performance loss. On the other hand, zircon
channel validation will likely be quite costly if we need to make a round trip
through the kernel, so we will consider how to remove it. Additionally,
stripping channels of rights is something we will choose to avoid as it again
requires a round trip through the kernel. The process by which we will make
these determinations will require benchmarking final decisions about which
validations we choose to perform is left to the implementation.

The reason we can make these choices is because drivers within the same driver
host share the same security boundary. These validation steps will only serve to
improve our resilience.

#### Request Forwarding

It is a common operation for drivers to receive a request, make some minimal
modifications, and forward it to a lower level driver. Being able to do this
cheaply and in an ergonomic manner is a goal. Additionally, being able to
forward a message to a channel where the driver owns both ends of the channel is
likely to be a useful activity, as the driver will need to possibly manage
concurrency and park data somewhere. While it could possibly push it on to a
queue, utilizing a driver runtime channel as a queue is likely to be an easy way
to achieve the same effect with less code.

### Transport Level Cancelation

When a driver must cancel some outstanding work, due to a new condition or
requirement, there is no uniform way to achieve this, in neither banjo nor
zircon channel based FIDL today. In channel based FIDL, many bindings will let
you ignore the replies which will eventually come. Depending on use case, that
may be adequate, as it allows deallocating state associated with the request
which may be the sole objective. However, sometimes it is necessary to propagate
the cancelation so that the downstream driver is aware of the cancelation. When
this happens, the approach is to either build support at the protocol layer, or
to simply close the client end of the channel pair. The latter solution only
works if *all* outstanding transactions need to be canceled, and no
synchronization with the server is required, as you cannot get acknowledgement.

The driver runtime channel can provide the same level of support that zircon
channel transport provides. Building support into the transport layer for
transaction level cancelation propagation is a tempting idea, however, due to
the split of duties between FIDL and the transport, quite challenging. The
transport is unaware of transaction IDs as they are a FIDL concept built on top
of the channel primitive. It's also perhaps not worthwhile to deviate from how
zircon channels work as it could possibly lead to confusions in developers that
need to deal with both transports. It would also make the possibility of a layer
which abstracts over multiple transports more challenging to implement.

## Implementation

The implementation of this design will largely occur in three phases:

1.  Implement driver host runtime APIs
1.  Implement FIDL bindings built on top of the driver host runtime APIs
1.  Migrate clients from banjo

### Driver Host Runtime APIs

The runtime APIs will be implemented in the new driver host, used for drivers
running as components. Rather than grant all drivers the ability to utilize the
APIs, ability to utilize them will be gated behind a new component manifest
field, named `driver_runtime`, specified alongside the `colocate` field. This
property will allow the driver runner to know whether it should provide the APIs
to the driver. All drivers in the same driver\_host must have the same value for
this property.

In addition to the primitives described earlier in the design, support will need
to be added to establish a mechanism by which new driver transport channels can
be transferred between drivers on bind. Additionally, an idiom for describing
bind rules and node properties to enable drivers to bind to devices implementing
driver transport FIDL services will need to be implemented.

Support will be added in isolated devmgr such that we can spawn drivers that run
in the new runner. This environment will be utilized for implementing the
primitives, writing tests, as well as running performance benchmarks.

The initial implementation of the runtime APIs will focus on an MVP to solidify
the final API such that we can start work on the FIDL bindings. Once ready, work
on the FIDL bindings can proceed in parallel while the implementation of the
runtime APIs can be optimized.

### FIDL Bindings

The FIDL bindings will be written inside the existing `fidlgen_llcpp` codebase.
Outputting the driver runtime headers will be gated behind flags. Implementation
strategy will follow a similar strategy as above, focusing on starting with a
minimal set of changes to get the bindings to work, utilizing the new driver
runtime. Once it is working, we will establish microbenchmarks to understand the
performance. We can then iterate on the bindings, making optimizations, such as
removing validation steps we deem unnecessary.

Wire type definitions will eventually be added as part of a shared header, but
until work is done to split those out of the existing FIDL headers into
dedicated headers, we will simply re-emit the types. This will cause issues for
any drivers which attempt to use both zircon channel and driver runtime
transports, but we will hopefully have finished work on splitting out wire
definitions into a shared header by that point.

### Migration {#migration}

Migration in particular will be quite challenging. This is despite the fact that
almost all drivers reside within the fuchsia.git repository. Rather than attempt
to move the entire world from banjo to the new runtime in one go, it will be far
simpler to migrate a single driver host at a time. The strategy will entail the
following steps:

1.  Banjo protocols being used by a driver being ported will need to be cloned
    and modified to create a version targeting the driver transport.
1.  Drivers being ported will implement support for both the banjo and the
    driver transport FIDL service.
1.  A new component manifest, bind program, and build target will be created for
    the driver indicating it targets the new runtime.
1.  For each board the driver lives in, once all drivers in the driver host(s)
    it inhabits have been ported, the board gni file which includes the driver
    will be updated to utilize the new version of the driver instead of the old
    one targeting banjo.
1.  The driver build variant being utilized for banjo can be deleted, as can any
    code in the driver using or serving banjo protocols.
1.  Once a banjo protocol is no longer being utilized by any drivers, it may be
    deleted.

For drivers which are included in several driver hosts, where not every driver
has been ported, it may be necessary to include both versions in the board at
the same time. Bind rules and the component runner field will ensure the correct
version of the driver is loaded in the appropriate driver hosts. Drivers will
also be able to easily detect whether they are bound to a driver exposing a
banjo service or driver transport FIDL service.

Many teams will be involved in the migration as a single team cannot migrate all
300+ fuchsia drivers by themselves. A detailed migration document that allows
most migrations to occur with minimal assistance will be necessary.
Additionally, prior to expected teams to perform migrations, we will need to
ensure adequate advanced notice is provided so they can plan for it.

We will also need to determine criteria which helps us organize the list of
drivers we port. For instance, we may wish to optimize for the minimal number of
duplicated drivers in a product build, while also parallelizing as many drivers
which get ported as much as possible. A process will necessarily be created and
actively managed to ensure we don't make mistakes causing regressions along the
way, as well as making sure the migration completes in a timely manner.

#### Evaluating Co-location

It will also be important for teams performing migrations to re-assess whether
or not their drivers need to live in process with other drivers or not. As
another artifact, the driver framework team will come up with a document helping
driver authors make this evaluation. It may be necessary for teams to perform
some benchmarking to help guide this decision making process, and building out
support to help make this sort of benchmarking easy to perform may also prove
useful.

## Performance

Many of the design points specified earlier in the RFC will necessarily need to
be benchmarked and measured before we commit to them in earnest. While we have
already done preliminary benchmarking to help us determine the direction of the
driver runtime we wish to head in, we will need to continue to be rigorous to
ensure all optimizations outlined are useful.

Microbenchmarks will be established to ensure that both at the transport layer
as well as at the FIDL binding level that performance is better than the zircon
channel equivalents.

Additionally, we will need to construct more e2e oriented benchmarks to ensure
that a more holistic level we are no compromising on any of the core metrics we
are optimizing for:

*   High total throughput
*   High I/O operations per second (IOPS)
*   Low latency
*   Low CPU utilization

We will likely utilize the most important use cases to determine drivers which
we port to the new runtime earlier, so that we may start benchmarking results.
Some example driver stacks may include:

*   NVMe SSD
*   Ethernet NIC
*   Display touch input
*   USB Audio

Ensuring we get a good variety of driver use cases will help us build confidence
that we're not overly optimizing for any single use case.

These benchmarks will be performed at the driver interface layer rather than
from a higher level interface involving all layers which typically comprise the
technology stack for the relevant device. This is because it is challenging to
use end to end benchmarks to get a good understanding of driver performance
today as drivers are often not the bottleneck when it comes to performance. We
are working as a platform on addressing the known bottlenecks, but we need to
ensure that drivers don't become the new bottleneck.

### Preliminary Benchmark Results {#zircon-channel-perf}

We have performed extensive benchmarks to help us determine the direction to
take with this design. We took existing stacks (block and network) and tested
the following scenarios:

*   Inserted channel write and read calls into all banjo calls (not passing any
    data, no thread hops).
*   Deferred all banjo calls to be run as an asynchronous task on a shared
    dispatcher loop (no thread hops inserted).
*   Deferred all banjo calls to be run as an asynchronous task on per driver
    async dispatcher loops.

Workloads run against these modified driver stacks varied queue lengths,
operations sizes, total workload size, as well as read vs write.

The benchmarks were run on NUCs utilizing the x64-reduced-perf-variation board.
Predictably, all benchmarks reduced overall throughput, increased tail
latencies, and increased CPU utilization.

*   Differences were especially significant with low queue lengths of size 1
    and 2.
*   Per driver threads had worse results overall.
*   Inserting channel reads and writes into banjo calls didn't significantly
    affect throughput, but had the most significant impact on tail latencies.
*   CPU utilization had relative increases of 50%-150% depending on parameters
    on all experiments trialed. Absolute CPU utilization was always non-trivial
    (10%-150%) so the the relative increases resulted in significant increases
    to absolute CPU utilization as well.

Full results can be found [here][perf-results] and [here][perf-results-2].

## Ergonomics

As mentioned in [an earlier section under requirements](#ergo-req), ergonomics
is very important to the overall design. Introducing additional concepts beyond
the ones already needed to write code for fuchsia is something we would like to
avoid. Driver authors already need to know and understand FIDL, as channel based
FIDL is how they interact with non-driver components. As such, by reducing the
amount of differences for inter-driver communication from that of interaction
with non-driver components would lead to a reduction in new concepts. Banjo is a
completely different technology, and while replacing it with something that is
overall more complex may seem like we are reducing ergonomics, the fact that it
is so close to channel based FIDL will hopefully lead to an increased overall
ergonomics. The fact that we will be able to share types, a long standing
developer pain point also lends confidence to this expectation.

Additionally, the introduction of a threading model will hopefully greatly
simplify the ease of writing correct drivers, again improving overall
ergonomics.

One of the places where ergonomics may be the most troubling is with support for
C drivers. Because the fuchsia platform no longer has any C drivers in tree, we
won't have a great understanding on how this proposal impacts C drivers. In
fact, the initial implementation won't even support C drivers, as FIDL currently
lacks proper C bindings even for the zircon channel transport. A lot of the
design choices used for this design were based on systems which are primarily
used for C, so chances are, as long as the C FIDL bindings that do get written
are ergonomic to use, then C drivers will not suffer. The fact that we won't
have sufficient feedback on this front for quite some time does mean it is a
risk.

It may be useful that during the migration process we conduct user studies to
understand whether our anticipated outcome is reflected in reality.

## Backwards Compatibility

We are implementing these changes before exporting any banjo interfaces via the
fuchsia SDK. As such, we do not have any limitations imposed us to ensure
backwards compatibility. However, due to the fact that we will need to perform a
piecewise migration, we will likely need to ensure some level of backwards
compatibility where both banjo and driver transport FIDL are supported in the
same driver at the same time for a short period. This is gone over in more
detail in the [migration section](#migration).

## Security considerations

The design proposed in this RFC should not alter the security architecture of
the system. Drivers which currently exist in the same process will continue to
do so, so the security boundaries should not shift. However, as an expected
outcome of a migration to the new driver runtime, it is expected that clients
will [re-evaluate](#documentation) on a per interface basis whether it makes
sense for their drivers to be colocated in the same process as their parents or
not. We will author documentation to help provide guidance for developers making
this evaluation.

## Privacy considerations

There is no expected privacy impact from this design.

## Testing

Different parts of the design will be tested via different mechanisms. The
driver runtime will be tested via unittests based loosely off of similar tests
for the equivalent zircon primitives. Additionally, integration tests will be
written utilizing the isolated devmgr and CFv2 test framework.

The driver transport FIDL bindings will be tested via GIDL as well as via
integration tests ensuring correctness. The integration tests will necessarily
need to be written on a per binding basis, and mimicking the integration tests
which already exist for the zircon channel transport variant of that binding
will be the most likely path taken. These integration tests will likely also
require utilizing the isolated devmgr and CFv2 test framework.

Drivers being migrated to the new driver runtime will likely need to have test
plans conceived on a per driver basis. Isolated devmgr based tests should
support the new transport without any special efforts required. For unittesting,
it is likely that a testing library will need to be written to provide
implementations of the driver runtime APIs without needing to run the test
within a driver host. A similar approach can be used to the one currently used
for emulating the libdriver API in unittests. Code sharing between the test
library and the driver host implementation of the APIs will be considered to
reduce drift in functionality.

## Documentation {#documentation}

New guides will be necessary to describe how to write a driver which consumes a
driver transport FIDL, as well as how to create a driver which serves it. They
will need to be written on a per binding basis, but since we are initially
targeting only llcpp, only one set needs to be written. Ideally we can adapt the
existing llcpp guides to reduce the overall amount of effort required to produce
the guides.

The reference sections for drivers will also need to be updated to include
relevant information about the runtime APIs. Information on the threading model
and how to write thread-safe code will also be necessary.

A best practice document helping driver authors determine whether to use the
driver transport or zircon channel transport version of FIDL will be necessary.

Lastly, as mentioned in the [migration section](#migration), documentation on
how to perform migrations from banjo to driver transport FIDL will be necessary.

## Drawbacks, alternatives, and unknowns

### Linearization

In order to achieve zero copy, we can avoid the linearization step which FIDL
performs today. Critically, this will violate the existing FIDL contract in
terms of structure layout by doing so. By avoiding linearization, this also
means we don't perform the transformation from decoded form into encoded form.

One of the reasons FIDL does things the way it does is that it allows decoding
to be much simpler. In particular, ensuring that all bytes are accounted for is
trivial to perform in one pass, as well as bounds checks that all the data
referred to lives within the message buffer. Due to the security concerns
discussed in the [Security and Resilience section](#security), it is not
considered necessary for us to worry about the former concern. For the latter
concern, we simply need to ensure that all data buffers point to memory
allocated by the arena. The driver runtime will provide an API which allows us
to perform this check.

One of the touted benefits of linearization is the ability to memcpy the buffer
around. Because the message is already backed by an arena, moving ownership of
the message is as easy as transferring ownership of the arena. Not being able to
copy the message is not necessarily a downside with any obvious ramifications.

Another benefit of linearization is that it simplifies decode. It's actually
possible that the extra complexity necessary to decode a message outweighs the
wins by not performing a copy of the data. This will need to be carefully
measured to ensure that skipping linearization is a clear win.

Lastly, linearization can improve cache performance by getting better spacial
locality. If the arena is implemented well (as a bump allocator), spacial
locality should still be good, and temporal locality improvements from calling
into other drivers within the same stackframe should make any losses here
unlikely to cause issues.

While this RFC is not suggesting that skipping linearization will the plan of
record, implementing the feature set required to do so so that we can evaluate
whether it would be a worthwhile feature to implement is planned as follow up
work.

#### Leaked Handles

One of the problems that we will encounter by skipping linearization is that we
may leak handles in scenarios where the sender of a FIDL message is using a
field in a union or table unknown to the receiver (by potentially using a newer
version of the FIDL library). In both zircon channel transport as well as the
proposed driver runtime transport, the receiver can safely ignore the new field
and otherwise understand the rest of the message contents. However, in the
zircon channel variant, handles are stored separately from the data, meaning
that if the handles were in an unknown field, the receiver remains aware of the
existence of the handle, and can close the unused handles. If we skip the
encoding step entirely we would lose our ability to know about all handles,
regardless of understanding the full message structure.

A solution for this is to partially encode and decode, without linearizing. More
specifically, the handles would be ripped out and replaced with offsets into a
linear handle table during encode, and on receive, the handles will be moved
back into the object during decode. Unused table entries can be kept track of
and closed during decode, similar to the zircon channel based approach. This
halfway approach should net us most of the performance benefits of skipping the
linearization step, while avoiding the potential pitfalls.

#### Wire Format Migrations

Another issue is that during FIDL wire format migrations, two peer drivers might
be using objects of incompatible layouts. For example, we're currently
[halving the size of a FIDL envelope][envelope]. This problem is solved in the
IPC case today by adding a transformer in-between which converts between the two
encoded wire formats. While we might write an additional transformer for the
non-linearized, decoded format, it is an additional burden for maintenance.

#### Non-LLCPP Bindings

As of the time of writing, only a single FIDL binding implementation, LLCPP, can
make use of the optimizations afforded by skipping the linearization step, as
well as handle possibly decode from a non-linearized state as it is the only
bindings that natively understand the wire format. For other bindings, on the
receive side, an extra step of linearizing the buffer can be performed prior to
proceeding with otherwise normal binding specific decode. Additionally, on the
send side, linearization is still acceptable to perform, the bindings just need
to take care to avoid turning pointers into offsets.

### Transparent Transport

The design as presented in this document leave the choice of whether they are
utilizing the zircon channel transport of the driver transport version of FIDL
for a particular protocol completely within the hands of the driver author. An
initial version of this design had opted to hide the underlying transport from
the driver in an effort to both improve overall driver ergonomics, reducing the
number of concepts to learn, as well as give the platform more control over
whether a driver is co-located or not. After much debate, it was decided to punt
on this idea as it greatly complicates the design, and introduces many new
problems that need to be solved. For instance, the need to linearize messages
would become a dynamic decision rather than one known at compile time.

If we decide to attempt to move forward with an attempt to make the underlying
transport transparent to the driver, a follow up RFC will need to be written.

### Operations

If we introduce a concept at the transport layer to track logical operations as
they navigate through various drivers in a stack, we will potentially be able to
offer better diagnostics, and perhaps make better scheduling decisions. As
described in this design, similar to zircon channel based FIDL, transacations
are a protocol layer concept currently. An earlier version of this proposal
considered making allocators single ownership allowing us to use them as a
stand-in for tracking operations between drivers, but utilizing allocators that
way was likely flawed and the current design now proposes a refcounted allocator
design instead. Revisiting a first class operation concept in the transport is
an imrpovement we can consider for the future.

### Single Copy Transport

If we decide that it is not important to achieve zero copy, we have many options
available to explore for alternative designs. We could choose to remove the
arena, and instead require pre-register completion buffers or rings similar to
[Windows IOCP][iocp], [io\_uring][iouring], or even NVMe designs. When a request
is completed, we would write the result into the pre-register buffer/ring,
likely as part of the linearization step. This sort of design lends itself to
built-in backpressure quite well. Another option would be to keep the arena, but
just always perform linearization. Weighing the pros and cons of these
approaches can be expanded upon if reviewers of this RFC would find it useful.

### Transport Backpressure

A well known issue with zircon channels is the fact that they do not have any
provisions for backpressure. At the time of writing, there exists a global
channel message limit, which if hit, results in the process owning the channel
end on the receiving end being killed.

Because we are designing a new channel primitive, we could choose to do
something better here. Given the large scope of this proposal, it was deemed
that we should punt on this. However, it is likely an area worth exploring, as
this problem is an area of active investigation by the FIDL team. Trialing a new
approach in the driver runtime may prove to be a good way to understand if the
approach taken is worth also implementing in the zircon syscall interface, where
the stakes are bit higher.

### Bi-directional Channels and Events

It has been suggested that the bi-direction nature of zircon channels is a
mistake. We could possibly choose to not make the driver runtime channels
bi-directional, instead requiring two channel pairs (4 ends) in order to get two
way communication, similar to the golang channel built-in. As a corollary, we
would not support fidl events over the driver runtime transport. Rather than
drift in terms of supported FIDL features the driver runtime transport supports,
it is likely to better match the feature set as much as possible. In addition to
reducing user confusion, this will make it easier to achieve an opaque transport
in the future if we decide to move in that direction.

### Recursive Locks {#recursive}

As an alternative to not supporting automatically queueing work that would
result in re-entrant access into a driver, we could possibly just allow it if
drivers opt into it. In order to correctly handle this, drivers would
necessarily need to either do one of two things:

1.  Queue up work themselves onto a queue and schedule the queue to be serviced
    later.
1.  Use recursive locks.

The first option has seemingly no benefits over the design presented in the RFC.
The second option requires utilizing a lock we have initially chosen to avoid
implementing support for in our platform. The reason being that recursive locks
make it challenging, if not impossible to ensure correctness in terms of
ordering in which locks are acquired. When locks are acquired in multiple
different orders, then a potential deadlock will lurk in the code. Rather than
risk this issue, it is simpler to just ensure that we never reentrantly enter a
driver.

### Rust Driver Support

We don't plan to support rust drivers as part of this proposal, but we do
anticipate that we would likely enable it in the near future. There is a large
desire from driver authors within fuchsia to write their drivers in rust, and
banjo has made it very challenging for the driver framework team to fully
embrace rust support. Ensuring that we can easily enable drivers written in rust
is a design consideration, although not necessarily a motivating factor.

It is expected that rust FIDL bindings for the driver transport as described
previously in this RFC will be achievable. The specific design and
implementation of such bindings is left open at this time, and will likely be
the subject of a future RFC.

### Work Priority

At some point in the future, it would be great to explore how to inherit
priority at a work or message level rather than at a dispatcher level. While
there are tenuous plans to explore this idea within the zircon kernel team, the
driver runtime has a lot of flexibility to experiment here and innovate beyond
what the kernel offers. This will likely be an important area of focus once
initial implementation of the runtime is done, and low hanging fruit
optimizations are made.

### Power Managed Channels

In other driver frameworks automatically stopping servicing of work when lower
level drivers are in suspended state is a useful operation. The design presented
in this RFC does not make any affordance to allow for this, however for the sake
of improved driver author ergonomics, it is an area of exploration. There are
many open questions at the moment with respect to how involved the driver
framework should be when it comes to power management.

## Prior art and references

Neither the concept of an in process threading model is a new idea, nor are
ideas about how to handle concurrency efficiently and ergonomically. The
approach taken here takes much inspiration from the following sources (+ more):

*   [Windows Driver Framework Queues](https://www.osr.com/nt-insider/2011-issue2/basics-wdf-queues/)
*   [Window Driver Framework Sync Scopes](https://www.osr.com/nt-insider/2014-issue3/understanding-sync-scope-wdf-drivers/)
*   [Common Object Model](https://en.wikipedia.org/wiki/Component_Object_Model)
*   [Apple Grand Central Dispatch (GCD)](https://developer.apple.com/library/archive/documentation/General/Conceptual/ConcurrencyProgrammingGuide/Introduction/Introduction.html)
*   [Rust Streams](https://blog.yoshuawuyts.com/streams-concurrency)
*   [Recursive Locks](https://blog.stephencleary.com/2013/04/recursive-re-entrant-locks.html)

[iocp]: https://docs.microsoft.com/en-us/windows/win32/fileio/i-o-completion-ports
[iouring]: https://lwn.net/Articles/776703/
[perf-results]: https://docs.google.com/spreadsheets/d/1orIJAnLLciTb7XSYAP1p_Zv94es6DII8UHerqOPEg88/edit#gid=425822193
[perf-results-2]: https://docs.google.com/spreadsheets/d/1dQSb1P7uL5MeTAfNDudEcpmKmT-pUQw0-sXK0tShd04/edit#gid=425822193
[envelope]: 0114_fidl_envelope_inlining.md
