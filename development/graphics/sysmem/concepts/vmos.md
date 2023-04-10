# Sysmem VMO management

This document describes the hierarchy of [VMOs][vmo] that [Sysmem][sysmem]
uses to manage memory. This document assumes a familiarity with [virtual
memory][virtualmemory] and [the accounting of non-sysmem memory][memoryusage]
on Fuchsia.

## Heaps

All VMOs are allocated from a heap. The current heaps are:

Name                                    |Fixed Pool |Description
----------------------------------------|-----------|-------------------------------------------------------------------------
Sysmem-core                             |No         |Generic main memory
SysmemAmlogicProtectedPool              |Yes        |Memory inaccessible from CPU
SysmemContiguousPool                    |Yes        |Physically-contiguous main memory
tee_secure                              |Yes        |Special-purpose protected memory for Amlogic decrypted encoded video
Sysmem-external-heap (possibly multiple)|No         |Currently used for [goldfish][FEMU]
Sysmem-contig-core                      |No         |Contiguous main memory; only used on systems without SysmemContiguousPool

Not all heaps suballocate from a fixed pool; e.g. "Core" and "Contig Core"
can allocate from main memory. Some heaps correspond directly with
[fuchsia.sysmem.HeapType][HeapType] values, but the heap chosen may also
depend on [BufferMemorySettings][BufferMemorySettings] members like `is_physically_contiguous`.

### Sysmem-core
Sysmem-core allocates from main memory. It's the default option if no
constraints are added to the memory.

### SysmemContiguousPool
The CPU and some other devices on the system only need virtually-contiguous
memory. They can pick arbitrary pages with any physical address, and rely on
MMU hardware to assign new contiguous virtual addresses. This makes it easy
to allocate memory, since any physical page can be used.

However, some hardware doesn't have MMUs or scatter-gather capability. That
hardware needs physically-contiguous address space - that's where every page
in memory is in the same order in RAM. As the system runs, main memory
becomes more and more [fragmented][fragmentation] and it becomes impossible
to find long runs of free memory, because other allocated pages happen to be
randomly scattered around memory.

To avoid this becoming a problem, Sysmem has a separate contiguous pool. It
allocates a large pool of memory shortly after boot before memory is
fragmented, then hands out smaller sections to applications. In theory this
memory could still become fragmented, but in practice it works because only
larger chunks of memory are allocated from the pool, and all memory in a
chunk is released back into the pool at the same time.

### SysmemAmlogicProtectedPool
On systems with Amlogic SoCs, [DRM (protected memory)][DRM] video needs to be
allocated in special regions with access control to ensure that applications
can't read the decrypted video. These regions must be inaccessible by the
CPU, and only accessible by the GPU and other hardware in special modes where
the hardware is wired to ensure it can't leak the memory.

Memory can't be marked as protected or unprotected arbitrarily. The hardware
can only mark a small number (< 32) of regions as protected. To support this,
sysmem can allocate a protected pool soon after boot (similar to the
contiguous pool) and tell the firmware to protect all the memory in that
region. Then it can suballocate from this pool of protected memory.

### tee_secure
tee_secure is for another type of protected memory that stores a different
type of data. The firmware allocates this region, and the
[ZBI][glossary.zircon boot image] must tell
zircon not to allocate from the memory and never to touch it. Another driver
can retrieve information on the memory and where it's located from the
firmware, and then tells sysmem. Sysmem can suballocate from this heap as
needed.

### Sysmem-external-heap
External heaps don't necessarily use real memory. For example, the goldfish
heap is an external heap that represents video memory outside the [FEMU][FEMU]
virtual machine. Clients can pass VMO handles around, but aren't supposed to
write directly to the memory; instead the goldfish driver looks up host
resources using the VMO koid.

## VMO hierarchy

Sysmem uses a hierarchy of VMOs to keep track of the memory usage for
clients. There are three things that can keep a VMO alive:

1. A handle to the VMO.

2. A mapping of a VMO into a process's address space.

3. A [PMT][pmt] representing that it's mapped onto a device.

For security reasons, sysmem can't reclaim memory for a VMO to use with
another client until all of these types of references go away. For normal
VMOs, the kernel handles this by only destroying a VMO once all references
have gone away. However, sysmem suballocates VMOs from larger physical address
ranges so it needs to have insight into whether a VMO is destroyed so it can
decide which memory ranges to reuse.

The kernel supports a `ZX_VMO_ZERO_CHILDREN` signal to help with these
use-cases - if all [children][vmo_create_child] of a VMO are closed then
`ZX_VMO_ZERO_CHILDREN` will be signaled on the parent VMO.

![VMO hierarchy](/docs/development/graphics/sysmem/images/vmo_hierarchy.png)

### Client leaf VMOs
These are the VMOs handed out to clients; clients name them by calling
[BufferCollection.SetName][SetName] before the VMOs are allocated. Clients
can also set `ZX_PROP_NAME` on the VMOs directly but that's not recommended
because the sysmem driver can't access that name.

Sysmem also holds on to references to these VMOs as long as a
BufferCollection continues to reference them, even if no child currently has
a VMO handle.

### Middle VMOs
Each leaf VMO has a middle VMO as a parent. There is a 1-1 mapping between
leaf and middle VMOs. The names are set by the heap, and are usually fixed
for all VMOs coming from a heap. For example, SysmemContiguousPool-child for
VMOs from the contiguous pool.

Sysmem uses these VMOs to detect if all references to a leaf VMO are cleared
out; once it receives the ZX_VMO_ZERO_CHILDREN signal it knows that it's safe
for it to delete the VMO and possibly reuse the space. Middle VMOs are never
passed outside the sysmem process, so clients can never reference them
directly.

### Heap VMOs
These represent the entire pool of memory that VMOs in a heap are allocated
from. They're often allocated soon after boot, to ensure that enough memory
is available. Heap VMOs may also represent a carved-out range of physical
addresses - for example tee_secure overlays a specific physical range
allocated by the bootloader.

Middle VMOs are allocated as [slices][vmo_create_child] from the heap vmo, so
each middle VMO represents a different range of memory in the heap

If a heap doesn't represent a physical pool of memory then it doesn't need a
heap VMO. In that case the Middle VMO is allocated without a parent VMO.

## Reporting memory

### Inspect
Sysmem supplies [Inspect][inspect] hierarchy to report its memory usage to
snapshots and other client applications. Here's a simple example hierarchy:

```
  root:
    sysmem:
      collections:
        logical-collection-0:
          allocator_id = 1
          heap = 0
          min_coded_height = 1024
          min_coded_width = 600
          name = vc-framebuffer
          pixel_format = 101
          pixel_format_modifier = 0
          size_bytes = 2490368
          vmo_count = 1
          collection-5:
            channel_koid = 20048
            debug_id = 5498
            debug_name = driver_host
          collection-6:
            channel_koid = 20050
            debug_id = 5498
            debug_name = driver_host
          collection-at-allocation-7:
            debug_id = 19829
            debug_name = virtual-console.cm
            min_buffer_count = 1
          collection-at-allocation-8:
            debug_id = 5498
            debug_name = driver_host
          collection-at-allocation-9:
            debug_id = 5498
            debug_name = driver_host
          vmo-20085:
            koid = 20085
      heaps:
        SysmemContiguousPool:
          allocations_failed = 0
          allocations_failed_fragmentation = 0
          free_at_high_water_mark = 37498880
          high_water_mark = 2490368
          id = 1
          is_ready = true
          last_allocation_failed_timestamp_ns = 0
          max_free_at_high_water = 37498880
          size = 39989248
          used_size = 2490368
          vmo-20085:
            koid = 20085
            size = 2490368
        SysmemRamMemoryAllocator:
          id = 0
```
Sysmem reports its view of memory through an inspect hierarchy in the
`/dev/diagnostics/class/sysmem/XXX.inspect` file (where XXX is the pseudo-random
3-digit identifier). Each logical-collection shown represents a set of identical
buffers allocated by a set of clients. Those logical-collections contain lists
of koids of live middle VMOs in that collection. koids are unique for the
lifetime of the system and can be used to uniquely identify sysmem VMOs in
memgraph output.

All heaps also have inspect nodes. These can include the size and koids of
all child VMOs, as well as information about how full the heap is and whether
it has failed any allocations. Some heaps only a name and id properties and
not information about the VMOs allocated from them.

The `allocator_id` of a logical-collection matches the `id` of the heap used
to allocate its memory.

The inspect data is limited because sysmem doesn't have a view into other
processes in the system. For example, it doesn't know which other processes
are holding onto references to its VMOs, only that at least one process is.
It also doesn't know the exact names of client processes that created VMOs.
Sysmem clients are supposed to call Allocator.SetDebugClientInfo with their
process name and koid, but that's not enforced and there's no guarantee that
the name the client sets is correct.

However, there are some pieces of information that can only be determined
from the inspect data. For example, a client process can hold onto a channel
to a BufferCollection without holding on to any handles to the VMOs. Only
sysmem knows the mapping between BufferCollection channels and VMOs inside
its process. The `channel_koid` property provides information on the server
koid of the channel.

### ZX_INFO_PROCESS_VMOs
This syscall is used by the `memgraph` and `mem` tools. It can determine what
processes have references to VMOs, which is important for attributing memory
to processes in a secure way.

The VMO hierarchy that sysmem uses can cause problems for these tools. For
example, `mem` ignores VMOs that don't have any [committed][memoryusage]
memory to avoid cluttering the output. That causes mem to ignore leaf VMOs
because it's the root VMO in the tree that actually allocated the memory. Mem
has some hacks to propagate memory information down the tree for VMOs that
are children of `SysmemContiguousPool` and `SysmemAmlogicProtectedPool` - it
looks at the "size" of the leaf VMO and assumes that all that memory is
allocated. This works only for fixed-size pools that are allocated with no
overlap, which is why it's restricted to a hard-coded set of pools.

External heap VMOs are also complicated since they don't actually take up
memory inside the [guest virtual machine][guest]. As such, mem is doing the
right thing in not reporting them (their committed memory size is 0), but
that means it's hard to attribute memory on the host system to processes
inside the guest.

`memgraph -v` does less processing of the memory information, but then the user
needs to do their own processing to determine memory usage. It can also be
difficult to determine what VMOs come from sysmem, since they don't
necessarily have consistent names.

### A unified approach
Any utility that wants a complete and accurate view of sysmem VMOs must
synthesize inspect and `ZX_INFO_PROCESS_VMOS` information. Sysmem's inspect
data should be the source of truth for what sysmem VMOs exist, and the kernel
is the source of truth for which processes hold references to VMOs. This would
require iterating through logical-buffer-collection entries and listing their
koids, then looking through `ZX_INFO_PROCESS_VMOS` to find their sizes and
what processes reference their children.

A utility can snapshot `ZX_INFO_HANDLE_TABLE` for every process. Then it can
look up the koid in `channel_koid` using that table to determine which
process is retaining that BufferCollection.

There are some circumstances where memory can't be accounted correctly. The
main problem is that handles held in channel messages aren't reported
anywhere, which makes it impossible to account for those references. A client
could shove a VMO handle into a channel and never read from the channel, and
even the kernel wouldn't know who to attribute the memory to. The debug
client info is usable as a fallback in those cases.

## Potential future changes

- Make a middle VMO for each client, so sysmem can determine itself which
clients still have references to VMOs.

- Have component framework pass an unforgeable identifier to sysmem instead
of having the client pass a forgeable debug name.

[glossary.zircon boot image]: /docs/glossary/README.md#zircon-boot-image
[vmo]: /docs/reference/kernel_objects/vm_object.md
[pmt]: /docs/reference/kernel_objects/pinned_memory_token.md
[vmo_create_child]: /docs/reference/syscalls/vmo_create_child.md
[sysmem]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem
[HeapType]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#HeapType
[BufferMemorySettings]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#BufferMemorySettings
[SetName]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem#BufferCollection.SetName
[inspect]: /docs/development/diagnostics/inspect
[FEMU]: /docs/development/build/emulator.md
[DRM]: https://en.wikipedia.org/wiki/Digital_rights_management
[memoryusage]: /docs/development/kernel/memory/memory.md
[guest]: https://en.wikipedia.org/wiki/Virtualization
[virtualmemory]: https://en.wikipedia.org/wiki/Virtual_memory
[fragmentation]: https://en.wikipedia.org/wiki/Fragmentation_(computing)
