<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0198" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Magma is the GPU driver design for Fuchsia. It can also be used to support other hardware
compute accelerators.  This RFC is an API design doc for the Magma FIDL protocols.

Magma has two driver components: a hardware-specific library loaded into each application's
address space ("client driver", sometimes known as "Installable client driver" or "ICD");
and the Magma system driver that interfaces with the hardware. Communication between the two
"drivers" is defined by the Magma protocols.

In this doc, "application" is used to denote the software component that uses the API exposed
by the client driver.

## Motivation

The Magma API provides a solution to the problem of supporting hardware accelerated graphics and
compute APIs, primarily Vulkan, on Fuchsia systems.  Its API concepts are not specific to Vulkan,
reducing friction supporting other client APIs such as VA-API for video, OpenVX and OpenCL.

Magma is designed to be used by implementations of the [Vulkan](https://www.vulkan.org/) API.
Vulkan is a C-style ABI available in the Fuchsia SDK.  These implementations ("ICDs") are typically
developed by hardware vendors, because accelerator hardware is large and complex, and building an
implementation requires detailed knowledge of hardware programming.

Since applications may use Vulkan directly, ICDs are libraries provided by the system, but cannot
access the hardware.  Using Magma, an ICD may interface with accelerator hardware to fulfill the
requirements of the Vulkan API, which primarily consists of executing client-defined programs.

A Magma system driver ("MSD") implements the server side of this protocol by scheduling and
executing client requests on the hardware. Thus, Magma is intended to connect the ICD and MSD,
playing a similar role to the [DRM](https://www.kernel.org/doc/html/v4.18/gpu/index.html) (Direct Rendering Manager) on Linux.

Magma's goal is to pragmatically minimize the degree of vendor specific aspects in the API. This
makes it easier to reason about API security, and facilitate security auditing of drivers.

Instead of having each GPU vendor design their own ICD-MSD interface (as is often the case on
Linux), Magma provides a core set of API features and methods that are commonly required, and where
necessary, Magma allows for vendor-specific queries and command structures. Vendor specifics are
not included in the Magma definitions, but are clarified in driver-specific documentation.

Support for display hardware is a non-goal.

The Magma API is currently in use by three drivers in production.  The build and release processes
for these client drivers are complicated because the API is not available in the Fuchsia SDK.

For all current cases, a [client library](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/sdk/lib/magma_client/include/lib/magma/magma.h)
is in use that facilitates integration with C code bases (C being the preferred language in Mesa,
which provides many open source graphics drivers) and provides a convenient abstraction layer for
virtualization.  Review and possible SDK inclusion of the client library is a future exercise.

## Stakeholders

_Facilitator:_

N/A

_Reviewers:_

jbauman@, rlb@: hardware graphics

_Consulted:_

API Council

_Socialization:_

N/A

## Design

Magma has three protocols, defined at:

https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/sdk/fidl/fuchsia.gpu.magma/magma.fidl

Some names have a numeric suffix (eg, BufferRangeOp2); the number is a detail of how the protocols
have evolved, and can be ignored.

1. Device

During the Fuchsia boot sequence, a Magma system driver is instantiated for each physical device
capable of accelerated graphics and/or compute.  With appropriate application [privilege](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/sdk/lib/vulkan/client.shard.cml),
clients may scan for the presence of one or more "gpu" class devices, and open them.  Synchronous
queries may be performed via the device protocol to obtain parameters which are useful for helping
the application decide which physical devices, under which configuration, to work with.  The query
parameters are also used for deciding which types of ICDs are compatible with each GPU device.
The exact behavior of driver-specific and device-specific protocols will depend on those query
parameters.

When the application declares its intent to work with a particular physical device, a connection is
established to the Magma system driver. In Vulkan, for example, the intention to work with a
physical device is expressed by calling vkCreateDevice().  This connection forms the basis for all
further communication between the client driver and system driver.

The connection comprises two channels. The primary channel is for the Primary protocol, mostly in
the client-server direction, and the other channel is used for notifications from server to client.

2. Primary

The primary protocol consists of a small number of core features implemented with a feed forward
design.  Errors trigger an epitaph message followed by closing of the connection channels.

a) Importing shared resources, buffers and events (ImportObject, ReleaseObject)

Buffers are shared memory objects backed by Zircon VMOs. Zircon events may by used for
synchronization where the API expects a semaphore.

Objects are client allocated and referenced by their KOID (kernel object ID). Client allocation is
a feed forward design choice.  This assumes all buffers come from system memory, which is generally
true for mobile class hardware.

Future support for desktop class GPUs with dedicated memory may require API changes.

Future support for external Vulkan timeline semaphores may require importing a new type of kernel
object.

b) Mapping buffers (MapBufferGpu, UnmapBufferGpu, BufferRangeOp)

To become accessible to the hardware, buffers must be mapped into the device address space. Each
client connection is given a private GPU address space (see Security and Privacy), so Magma
expects device address space management to be performed by the client.

For some drivers, MapBufferGpu may imply page table population in the device MMU, while for some
drivers, explicit control over page table population is supported via buffer range operations.

Commit of buffer pages is implied by page table population.  Any client page decommit operations
should be coordinated with the system driver via appropriate depopulate operations.

c) Creating execution contexts (CreateContext, DestroyContext)

To execute work on the hardware, a context is needed. Contexts contain all hardware state required
to allow for multiple contexts to be switched onto the hardware, and are scheduled for execution by
the Magma system driver. Magma supports multiple contexts per connection; this allows for more than
one context to share a single address space.

d) Executing commands (ExecuteCommand, ExecuteImmediateCommands)

Commands consist of vendor-specific data that modify the state of the hardware and trigger code
execution on the accelerator's compute cores.  Program code is provided by the application to the
client driver, typically in a machine-independent format such as SPIR-V.  The client driver must
translate the code into machine level instructions understood by the accelerator hardware.

Magma provides two interfaces for command execution:

ExecuteCommand assumes that the program code and associated resources are contained in shared
buffers.  Synchronization is provided with a list of wait semaphores that must be signaled before
the work is scheduled, and signal semaphores that are signaled when the command completes.

ExecuteImmediateCommands sends commands inline.  Multiple commands may be included in a single channel
message if the driver defines a way of splitting them.  The number of command bytes per message is
currently bounded at 2k, an arbitrary limit that has worked well to date, but doesn't allow for
efficient use of the full channel message size.  Semaphores are provided which can be used either to
wait or signal, depending on associated command data.

Future consideration for command submission include: using fuchsia.mem.Data with temporary VMOs, or
a model such as Linux "io_uring": https://en.wikipedia.org/wiki/Io_uring

e) Flow control

To prevent a client pushing too many messages into the primary channel, and to prevent excessive use
of memory by buffer objects pending import in the primary channel, the device protocol publishes
maximum values, and the primary protocol dictates that clients keep counts, and delay sending
messages to keep those counts less than the published maximums.  The server provides event messages
to inform clients when counts may be decremented.

f) Performance counters

The Primary protocol exposes a number of APIs specifically for interaction with hardware counters
that are critical for applications to achieve a deep understanding of behavior and the performance
characteristics of programs running on accelerator hardware.

3. Notifications

The notification channel is used for messages which are sent only in the reverse (server-client)
direction, typically in response to command completion.  The content of the notification messages
tend to vary from vendor to vendor, so an empty protocol has been defined.

A notification channel separate from the primary channel can make it easier for the client to
process notification events, for example the client may use a dedicated thread.  On the primary
channel, client calls can come from multiple threads, where none of the loops are polling in an
async dispatcher, and we don't want to proxy those calls into the primary channel for performance
reasons.  This makes reading asynchronous events difficult (except for the flow control events
which only need to be read during sending of messages).

## Unknowns

There are currently four supported Magma drivers (three in production), and two others have been
prototyped. The bringup of each additional Magma driver gives additional insight as to how well the
Magma protocols are suited to replacement of the various existing Linux-based client driver-kernel
driver interfaces.  Adding support for a desktop class GPU would give confidence that the core
APIs are compatible with a diverse range of hardware.

API areas that could use improvement are mostly: command execution and notification handling.

Feedback from API council is appreciated especially on these topics:

a) How are the APIs similar or different from other Fuchsia APIs?
b) Are there opportunities to make the APIs more future proof?
c) Any thoughts on the usefulness of the io_uring model for Fuchsia?

The API has been reviewed in brief by a domain expert at Google; however a more thorough review
that includes other experts/other related teams at Google could be beneficial.

## Performance

The Device protocol contains synchronous methods, but these should be used only during an
application's setup phase.  The Primary protocol contains asynchronous methods, and command
completions are returned asynchronously over the notification channel.

Most data are shared in VMOs, so data copying is minimized.  Creating and mapping buffers can be
slow, but these operations should occur rarely, assuming an application is well designed (eg,
performs memory suballocation).

For most cases, performance of Magma drivers on Fuchsia has been measured to be equivalent to the
same client driver codebase running on Linux on the same hardware.

## Ergonomics, Usability

The API design is similar to existing Linux-based client driver-kernel driver interfaces that have
been studied, with the exception that allocations are performed client side where possible.

The semantics of the API are believed to be reasonably intuitive because the API has been in use for
two years, has been revised during that time, and is currently serving three drivers in production,
from different vendors.

In the future, for Vulkan timeline semaphores a new kernel object may be deemed necessary; if so,
it may be added to ObjectType, and where semaphores are specified by KOID in the execute APIs,
"timeline" objects may be referenced in place of events.

## Backwards Compatibility

FIDL tables have been used in various places where future extensibility has been anticipated.

## Security

The Magma API lets applications run unvalidated programs on an important shared system resource,
so security is an important topic.  A security review was completed 06/2021: see https://fxbug.dev/67673.

It is possible for clients to submit long-running programs which deny access to the hardware to
other clients.  The Magma system driver ensures that long running programs are aborted with an
[acceptable](https://fxbug.dev/94071) degree of system disruption, similar to the behavior of other
systems.

Graphics memory can contain sensitive information such as passwords, so it's important that
clients' memory access is isolated.  Client connections are isolated from one another through
independent hardware address spaces: that is, a client's program can only access memory buffers
that have been imported to the connection (created by the client, or received explicitly from
outside).  This approach is subject to possible hardware vulnerabilities.

The number of clients may exceed the number of hardware-provided address spaces. MSDs are expected
to either schedule access to those resources or reject clients entirely.

For performance reasons, it isn't practical to validate client commands and programs, so the hardware
is expected to be resilient.

## Privacy

Device IDs provided by querying DEVICE_ID are generally not unique.

Performance counter events can be used to determine a detailed set of statistics for other
workloads on the system, which can be used to fingerprint what other processes are doing. Because
of that, they're more restricted than most Magma APIs and are limited to the set processes that
can access /dev/class/gpu-performance-counters devices.

## Testing

A simple functional test of the API is provided here:

https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/graphics/lib/magma/tests/integration/test_magma_fidl.cc

It can be difficult to write more sophisticated tests that exercise the Magma APIs without
including hardware specifics.  Client driver code contains those hardware programming details,
so test suites that exercise the API exposed by the client drivers are critical.

Vulkan conformance tests are tested in continuous integration for two production Magma drivers.
OpenVX conformance tests are run on the Magma driver for ML.

Note that when building a Magma driver, developers are expected to build a functional Magma system
driver that implements the server end of the Magma protocols, before they can test client driver
code that uses the Magma APIs.

## Documentation

Docs are inline with the FIDL definitions.

## Drawbacks, alternatives, and unknowns

This design is in sync with Vulkan which is designed to minimize driver overhead and maximize
system graphics performance.  A drawback of the design is that for security, it relies to a
large extent on hardware, whereas alternative designs may attempt to provide validation of client
commands. Validation, however, is complex, imperfect, and likely comes at a steep performance cost.

## Prior art and references

Linux GPU Driver Developer's Guide: https://www.kernel.org/doc/html/v4.18/gpu/index.html
