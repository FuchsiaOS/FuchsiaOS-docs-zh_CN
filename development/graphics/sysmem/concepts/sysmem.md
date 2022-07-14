# Sysmem overview

[Sysmem][sysmem] is a [FIDL][fidl] service that allocates shared memory for
use by multiple applications and hardware blocks. This document gives an
overview of its major features and what it provides to the system.

## Motivation

Modern systems have specialized hardware blocks that can perform operations
on data. Some examples of these blocks:

* Rendering computer graphics with a GPU.
* Encoding or decoding video with a hardware codec.
* Capturing high-quality photographs with a camera and DSP.
* Compositing images to a display using the display controller’s overlay
  engine.
* Evaluating neural networks with a TPU.

Often, we want to create pipelines passing data between those separate
hardware blocks. Some examples:

* Decoding video with a hardware codec and compositing it to the display
 together with user interface content rendered on the GPU.
* Applying a neural network to a live camera feed.

To do this efficiently and avoid copying, drivers and applications must agree
on the format data is in and where the data is located in memory. This
agreement allows the output of one unit to be used directly by the next.
Hardware units have strict constraints on these properties and also
preferences about which data layout will give the best performance.

Sysmem is a global service that can be fed constraints by applications and
allocate buffers such that all the constraints are met. If multiple formats
are supported, it can choose between them based on the expected performance.

Sysmem buffers are often used to represent images and sysmem has special
support for negotiating image formats. However the sysmem interface can also
be used for audio or any other type of data.

Sysmem does not manage the flow of data once buffers have been allocated.
Applications in a pipeline are responsible for coordinating between each
other to ensure synchronization.

## Allocation process (simplified)

1. [Participants](#participants) connect to the
    [fuchsia.sysmem.Allocator][Allocator] service
1. One participant (called the initiator) creates an initial
    [buffer collection token](#token).
1. That participant [duplicates] the token and sends the duplicate to other
    participants.
1. Those participants can also duplicate and send tokens, recursively, until
    all participants have received a token.
1. Each participant [binds][bind] its token to get a buffer collection.
1. Each participant [sets constraints][setconstraints] on the buffer collection
1. Sysmem chooses a format that can satisfy all the constraints. This can
    only happen after every participant has bound its token and set constraints
    on the collection.
1. Sysmem allocates a number of [buffers](#Buffers) using that format.
1. Participants retrieve the buffers from sysmem, as well as information
   about the allocated format.

The information returned by sysmem is the set of constraints the image
formats in that buffer must satisfy.

At this point participants can use the buffers with the allocated format,
subject to pipeline-specific constraints on the flow of data between
participants. Since multiple image sizes may be usable from a buffer,
participants must work together to choose an image size before they can
determine the precise image format. This allows a pipeline to switch image
sizes on the fly without needing to reallocate buffers.

If a new participant needs to be added to an existing collection which has
already undergone constraint negotiation and allocation, a new token can be
[attached][AttachToken] to the buffer collection. The new participant's
constraints must be satisfied using the already-allocated buffer collection,
or the logical allocation (from the point of view of the new participant) fails.
To increase the chances that the new participant can be added successfully, one
of the participants present during initial allocation can mark a token
[dispensable][SetDispensable]. This allows the token to be used to specify
stand-in constraints so that the buffer collection will be able to later
accommodate a new participant with the same constraints.

## Buffer destruction

All references to a buffer must be removed before sysmem will destroy it and
allow the memory to be reused. These include:

* [Handles][handles] to the VMO.
* [CPU mappings][map] of the VMO.
* [Child VMOs][vmo_create_child] of the VMO.
* [Pins][pmt] of the VMO for use by hardware.
* [Channels][channel] to a Buffer Collection containing that VMOs.

## Glossary

### Buffers

A buffer represents a single image or other piece of memory an application
will work with. Sysmem currently uses one [VMO][vmo] per buffer. Clients can
[map][map] the memory onto the CPU or [pin][pmt] it to use it with a hardware
block.

### Participants

A participant is any application or driver that wants to access a buffer. All
participants must connect to sysmem to negotiate memory formats.

### Image formats

An [image format][ImageFormat] is the entire set of properties needed for a
client to interpret the memory as a set of pixels. For example, it includes
the pixel format, size in width and height, row pitch in bytes between rows,
and color space.

### Buffer settings

[Buffer settings][SingleBufferSettings] are a complete description of the
properties of a buffer. These include the caching information, and other
properties that participants may need to access the memory. For images the
buffer settings will also have an image format.

Buffer settings do not include the specific memory address, so multiple
different buffers may have the same buffer settings.

### Heaps

A [heap][HeapType] represents one specific type of memory on a system. A
system may have multiple heaps with different performance characteristics
from each other. Some heaps may only be usable from a subset of hardware
devices on a system.

Some heaps may not be accessible from the CPU. For those heaps, the VMO
representing a buffer cannot be used directly but is instead used as a key.
An application wishing to use the buffer must send its VMO handle to the heap
driver and the heap driver can return information about what memory to use.

Example heaps:

* Main system memory.
* VRAM on a discrete GPU.
* A carved-out area of system memory that's only usable by some pieces of
   hardware.

### Constraints

[Constraints][constraints] specify the set of
[BufferSettings][SingleBufferSettings] that a participant is able to use.
Participants often specify multiple potential buffer settings in a single set
of constraints, which gives sysmem the flexibility to pick any of them and
reduces the risk that there are no settings that can satisfy the constraints
from all participants.

### Negotiation

Negotiation is the process where sysmem looks at all the constraints from
participants and chooses buffer settings that work for all of them. If
multiple settings could work, sysmem can use information about how clients
will use the buffer and the architecture of the system to choose the optimal
settings.

### Buffer collection
A [buffer collection][BufferCollection] is a set of multiple buffers with all
the same buffer settings. Sysmem allocates an entire buffer collection at
once. Multiple participants may have FIDL channels open to the same buffer
collection.

### Buffer collection token {#token}
A [token][BufferCollectionToken] is used in the initial stages of the
negotiation process before memory is allocated. Tokens can be duplicated and
passed between processes before finally being [bound][bind] to a buffer
collection.

[vmo]: /reference/kernel_objects/vm_object.md
[sysmem]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem
[HeapType]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#HeapType
[ImageFormat]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#ImageFormat_2
[SingleBufferSettings]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#SingleBufferSettings
[duplicates]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#BufferCollectionToken.Duplicate
[Allocator]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#Allocator
[BufferCollectionToken]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#BufferCollectionToken
[BufferCollection]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#BufferCollection
[channel]: /reference/kernel_objects/channel.md
[pmt]: /reference/kernel_objects/pinned_memory_token.md
[vmo_create_child]: /reference/syscalls/vmo_create_child.md
[handles]: /concepts/kernel/handles.md
[bind]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#Allocator.BindSharedCollection
[setconstraints]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#BufferCollection.SetConstraints
[fidl]: /development/languages/fidl/README.md
[map]: /reference/syscalls/vmar_map.md
[constraints]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#BufferCollectionConstraints
[AttachToken]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#BufferCollection.AttachToken
[SetDispensable]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#BufferCollectionToken.SetDispensable
