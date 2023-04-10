<!--
    (C) Copyright 2020 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

# Driver stack performance

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

The purpose of this document is to provide an overview into good and bad practices in regards to
performance for authoring new drivers or interacting with existing ones in Fuchsia.

This document covers the following topics:

 - [API Definition](#api-definition)
 - [Implementation](#implementation)

## API definition {#api-definition}

When authoring the API for a new driver stack, some key performance insights should be taken into
consideration. Driver stack APIs fall into the application or driver category.

Application APIs are used to give regular non-driver components access to hardware resources, while
driver APIs are used to allow drivers to communicate among themselves. For example,
[`fuchsia.hardware.network`][netdevice-fidl] is an application API to interact with network drivers,
and [`fuchsia.hardware.network.driver`][netdevice-banjo] defines the lower level driver API for network
devices.

Device driver APIs are typically `fuchsia.hardware.*` banjo and `fuchsia.hardware.*` FIDL APIs.

#### Avoid synchronous operations

Units of work that are part of the fast path should avoid the expectation of synchronous completion,
especially (but not exclusively) if the operation requires crossing process boundaries. In the case
of FIDL APIs, unnecessary synchronicity may arise from a design where a new unit of work can't be
started until the last one is completed, meaning the caller has to wait idly until it is safe to
request the next unit of work.

Synchronous operations are acceptable from the performance standpoint in slow path operations such
as setup, teardown, and configuration.

#### Encourage batching

When an API definition explicitly defines a batch of work, as opposed to always transmitting single
units, users of the API are encouraged to plumb the batching through their applications or drivers.

An important feature of batching is reducing the number of times the API needs to be exercised for a
set amount of work. For an API that crosses process boundaries, that translates directly into
reduced syscall and scheduling overhead, which helps performance.

Furthermore, if the API definition itself is providing units of work in batches, device drivers can
more easily coalesce a batch of work items into a single unit through hardware acceleration. Many
device drivers use DMA, for example, to enqueue work with the specific hardware they drive. If a
batch of work items is received at once, it may be possible to reduce the number of transactions
with the hardware. Without a well-determined batch boundary, device drivers are forced to either
interact with hardware more often than necessary or resort to heuristics (such as a polling
interval) to reduce the hardware communication burden.

#### Avoid data copies

Avoiding data copies is especially important in high-bandwidth or low-latency applications such as
networking, audio, or video. Large payloads should cross API boundaries in a VMO whenever possible.
A common strategy is to negotiate a limited number of VMOs on setup and exchange references to
regions in those VMOs during operation.

Note: When defining a `banjo` API, it's technically feasible to share virtual memory pointers over
the API boundary. API authors should always be aware that doing so restricts users of the API to
**always** be in the same process, which is undesirable from a system architecture standpoint.

#### Clarify flow control

If the API batches work and does not set strict synchronous operation expectations as suggested
above, controlling the flow of information between the API's server and client is important both for
correctness and performance.

In regular operation, the API's flow control definition must allow for all parts to be able to
perform work without being blocked waiting for their counterpart by maintaining some invariant about
the total amount of work that the system is capable of performing (either fixed by definition or
pre-negotiated on setup).

For example, `fuchsia.hardware.network` enforces flow control by defining a finite amount of "units
of work", i.e. network packets, during set up. At any point in time, both parties involved can be
aware of how many packets can still be pushed across the API boundary using only locally-maintained
state.

#### Account for ordering

In some applications, all units of work must be performed in a set order for correctness. In other
applications, however, some ordering constraints may be weakened or lifted altogether.

When the stream of work items doesn't have to be executed in an exact order, the API can reflect
that to allow drivers and applications to easily identify work that can be parallelized.

For example, networking packets usually need to maintain ordering to not break application
protocols, but that is generally only true for each application stream instance (called *flows*). A
common strategy in network adapters is to define a set number of packet queues and assign each
application *flow* deterministically to one of the queues. The networking stack can, then, safely
parallelize operating the queues without having to look into the packet's contents. Observing such
common hardware facilities and enabling the best use of them in the API can translate into
meaningful performance improvements.


## Implementation {#implementation}

This section lists performance patterns and anti-patterns to observe when implementing code that
either serves or consumes a device driver API. The implementation can be a driver itself, or an
application that interacts with services that are provided by device drivers.

The following should always be observed when writing code that is on the fast path. Note that for
device driver implementations, part of the fast path is often in interrupt threads.

 - **Avoid allocating memory or creating zircon objects**. Resources used on the fast path should
 always be pre-allocated. Be especially cautious when using libraries such as FIDL bindings (notably
 hlcpp) or `fit::function` that may cause implicit allocations.
 - **Be mindful of syscalls**. Syscalls can be costly to operate, so great care must be taken to not
 call into the kernel recklessly in performance-sensitive operations. Completely eliminating
 syscalls may not be possible, but reducing the number of calls per unit of work (with batching, for
 example) can greatly reduce system load.
 - **Return shared resources quickly**. If the API defines a finite pool of shared resources, such
 as shared memory regions, those should be reused or returned to the "available" pool as quickly as
 possible, preferably in batches.
 - **Map VMOs read-only rather than read-write whenever possible**. Mapping VMOs read-only
 reduces the number of cache operations that need to be performed for DMA. For example, when using
 DMA to write to a shared VMO that may have dirty cache lines, caches would have to be flushed and
 invalidated before and after DMA. On the other hand, with a read-only mapping the cache lines are
 known to always be clean, meaning they only need to be flushed and invalidated after DMA is
 complete.


[netdevice-fidl]: /sdk/fidl/fuchsia.hardware.network/device.fidl
[netdevice-banjo]: /sdk/fidl/fuchsia.hardware.network.driver/network-device.fidl
