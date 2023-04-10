# Magma: Design

For an overview of Magma including background, hardware requirements, and
description of architecture, please see [Magma: Overview](/development/graphics/magma/README.md).

## Goals

### Extensible core

Magma aims to pragmatically minimize the architectural differences in the way
that client drivers communicate with their respective system drivers across
different gpu and client driver designs.  Where necessary, Magma adds
gpu-specific queries, command structures, and interfaces to accommodate specific
requirements.  These differences are clarified in driver-specific documentation.

### Feed forward

Wherever possible, the main IPC channel is used in a feed-forward manner to
prevent blocking client threads.  For notifications from the system driver back
to the client, driver-defined messages are sent asynchronously.

### Avoid gpu faults

Clients can cause the gpu to hang or fault, and when this happens it will affect
system critical gpu clients like the compositor, resulting in a noticeable
hiccup in user experience.  Consequently Magma attempts to minimize the
opportunities for clients to induce gpu faults.

## Architecture

As mentioned in the overview, the Magma architecture involves two driver
components: a client library, and a privileged system driver process.  Both are
gpu-specific; the client library must compile software from IR (intermediate
representation, for example SPIR-V) into machine code, and format command
buffers correctly for consumption by the hardware.  These are fed to the Magma
system driver, which performs that actual programming of the hardware.

![Block diagram of Magma architecture](/development/graphics/magma/block_diagram.png)

Magma defines two interfaces to gpu-specific code:

* The **magma** interface provides the foundation for the client driver,
  typically libvulkan
* The **msd** (magma system driver) interface dictates the entry points for an
  implementation of a magma service driver

libmagma is a convenience layer that implements the magma interface and forwards
calls appropriately to magma_system, which is a gpu-agnostic layer handling
driver initialization and teardown, transport of IPC commands, and some
validation.

The creation of a magma driver is thus simplified at a high level into two
steps:

1. Produce a client driver by leveraging the magma interface (described below)
2. Implement the msd interface to produce a magma system driver.

For details on the process of building these two components, see the
[porting](/development/graphics/magma/concepts/porting.md) guide.

## The Magma interface

The Magma interface is a service interface provided by the Magma system driver.
The interface is designed to be useful for implementing an accelerated graphics
api.  It consists of
[magma.h](/sdk/lib/magma_client/include/lib/magma/magma.h) plus gpu specific
headers (example:
[intel](/src/graphics/drivers/msd-intel-gen/include/magma_intel_gen_defs.h)).

On Fuchsia, Magma includes a
[magma_sysmem.h](/sdk/lib/magma_client/include/lib/magma/magma_sysmem.h) header
that clients may use to interact with
[sysmem](/development/graphics/sysmem/concepts/sysmem.md).  The Magma
headers and a static library that implement them are available in the Fuchsia
SDK at [@fuchsia_pkg//pkg/magma_client][libmagma].

### Physical devices
During the Fuchsia boot sequence, a Magma system driver is instantiated for each
physical device capable of accelerated graphics.  The instantiation creates a
device binding in the class gpu; for example, in a single gpu system the device
is bound to a node under /dev/class/gpu.  With appropriate application privilege,
client drivers may scan for the presence of one or more gpu class devices, and open
them.  Synchronous queries may be performed on the device file descriptor to
return various parameters, some of which may be useful for helping the
application decide which physical devices, under which configuration, to work
with.

### Connections
When the application declares its intent to work with a particular physical
device, a connection is established to the Magma system driver. In Vulkan, for
example, the intention to work with a physical device is expressed by calling
vkCreateDevice().  This connection forms the basis for all further communication
between the client driver and system driver.  A connection allows the client
driver to allocate buffers and map them into the gpu address space.  The
connection defines a memory isolation boundary; Magma guarantees that buffers
mapped into one connection’s address space are by default not accessible to
another connection.  Buffer sharing between connections is possible with
explicit export/import.

### Contexts
To execute work on the gpu, a context is needed.  Contexts are scheduled for
execution on the gpu by the Magma system driver.  Contexts should contain all
gpu state required to allow for multiple contexts to be switched onto the
hardware.  Command buffers are used to set the state of the gpu, so command
buffers are submitted to a particular context.  Magma supports multiple contexts
per connection; this is to allow for more than one context to share a single
address space.

When a client connection is closed, to avoid gpu fault the address space must
remain alive while gpu is executing work using that address space; therefore,
context takes a shared reference on the address space.

### Buffers and Mappings
Currently Magma requires a unified memory architecture, as is the case with most
mobile hardware, where cpu and gpu access the same physical memory.   Magma
buffers are just zircon virtual memory objects
([VMOs](/reference/kernel_objects/vm_object.md)). Client drivers allocate
buffers and register those buffers with the system driver.

Gpu address space management may be performed by the client or by the system
driver. The client driver design may dictate the model.

If the system driver manages buffer mapping lifetime, the system driver ensures
mappings, and their underlying buffers, are alive while command buffers
referencing them are outstanding on the gpu.  Since mapping is slow (because it
requires ensuring that buffer pages are committed, and modifying page tables to
reference the correct bus address for each page), buffers mappings must either
persist for the lifetime of the buffer, or a gpu mapping cache could be used to
limit the amount of memory used by cached mappings.

The disadvantage of system driver managed buffer mappings is when building
command lists, the client needs to know the gpu address of mapped buffers; so
command buffers must be patched by the Magma service driver prior to execution.
For this reason, it is preferred to have the client driver explicitly manage gpu
mapping lifetime.  The disadvantage with the explicit approach is that a client
may unmap or release a buffer while a mapping is in flight on the gpu; if this
occurs, the page table entries will be invalidated while in use by the gpu,
likely causing a gpu fault.

A mitigation for this disadvantage is possible if each command buffer is
accompanied by a list of dependent buffer mappings; then, the command buffer can
share ownership of the gpu mappings; and if an unmap or release is received
while a resource is inflight, this is treated as a fatal error to the client
without disrupting the gpu.

### Command submission

Commands consist of vendor-specific data that modify the state of the gpu and
trigger code execution on the gpu compute cores.  The system driver is
responsible for queueing and scheduling submitted commands onto the gpu for
execution.  Various scheduling algorithms are possible: FIFO (default),
priority, time slicing.  Pre-emption of inflight command buffers, if supported
by hardware, can be used to implement context prioritization.

### Semaphores

Magma provides semaphores as a general signalling mechanism that can be used to
implement Vulkan fences and semaphores.  Magma semaphores are built on zircon
[events](/reference/kernel_objects/event.md).

### Summary of object ownership
* Client: owns connections; shared ownership of buffers, mappings, contexts
* Connection: shared ownership of address space
* Context: shared ownership of address space
* Address space: shared ownership of mappings
* Mappings: shared ownership of buffers
* Command buffer: shared ownership of context; may have shared ownership of
  mappings

## Thread Model

The thread model used for each installed GPU device and driver is as follows:

The msd is typically loaded by the [platform bus
driver](/development/drivers/concepts/device_driver_model/platform-bus.md)
and a msd main devhost thread is created.  The msd main thread in turn creates
a device thread to talk to the GPU and a driver-dependent number of interrupt
threads to service GPU interrupts.

A client driver library that implements the Vulkan api is referred to as a
**vcd** (Vulkan Client Driver).  When a Vulkan application starts and makes a
new VkDevice, the vcd makes a request to the msd to establish a connection for
the device over which all Vulkan commands will be communicated.  The msd main
thread responds to this call by creating a new connection thread to service all
client commands. The connection thread in turn creates two zircon communication
channels: the primary channel and the notification channel.

Vulkan state configuration, resource creation and drawing command buffers are
sent from the vcd to the msd over the primary channel.  The notification channel
is used to convey asynchronous status messages back to the vcd.  A good example
of a notification the vcd may be interested in is the completion of a command
buffer.  The exact messages sent over the device and notification channels along
with how those messages are handled varies by GPU driver.

![Vulkan driver thread model](/development/graphics/magma/vulkan_driver_thread_model.png)

Note that the process boundary enclosing the msd is the Fuchsia devhost process
boundary for the msd.  This devhost process may include threads from other
drivers as well but only msd-specific threads are shown here.

## Error Handling

When an error occurs in the magma service driver, the corresponding connection
is killed.  When the client driver attempts to access the closed connection it
typically will pass up a "device lost" Vulkan api error.

## Power Management

Power management involves adjustment of the gpu execution frequency based on a
measurement of gpu load, and potential power or thermal limitations.  Power
management will be detailed in further documentation.

## Testing Strategy

See [test_strategy](test_strategy.md).

[libmagma]: /src/graphics/lib/magma/src/libmagma