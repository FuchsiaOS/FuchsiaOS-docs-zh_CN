# Memory usage

This file contains information about memory management and diagnosis in Zircon,
and talks about ways to examine process and system memory usage.

A process can use memory 3 ways:

 1. Mapped memory in the form of heaps, thread stacks, executable code + data.
    This memory is represented by [VMARs](/docs/reference/kernel_objects/vm_address_region.md)
    which in turn hold a reference to [VMOs](/docs/reference/kernel_objects/vm_object.md).
    The programmer usually interfaces with this memory via memory addresses.
 2. Stand-alone VMOs. These are sets of memory pages that are not mapped via a
    VMAR. The programmer interfaces with this memory via handles; usually issuing
    [vmo_read](/docs/reference/syscalls/vmo_read.md) and [vmo_write](/docs/reference/syscalls/vmo_write.md).
 3. Kernel memory in the form of handles to kernel objects.

Fuchsia follows an over-commit model: processes can allocate more memory than
can be satisfied at a given moment and as memory pages are written they are
physically allocated (wired) on the fly by kernel.

## Userspace memory

Which processes are using all of the memory?

### Dump total process memory usage

Use the `ps` tool:

```
$ ps
TASK           PSS PRIVATE  SHARED NAME
j:1028       32.9M   32.8M         root
  p:1043   1386.3k   1384k     28k bin/devmgr
  j:1082     30.0M   30.0M         zircon-drivers
    p:1209  774.3k    772k     28k /boot/bin/acpisvc
    p:1565  250.3k    248k     28k driver_host:root
    p:1619  654.3k    652k     28k driver_host:misc
    p:1688  258.3k    256k     28k driver_host:platform
    p:1867 3878.3k   3876k     28k driver_host:pci#1:1234:1111
    p:1916   24.4M   24.4M     28k driver_host:pci#3:8086:2922
  j:1103   1475.7k   1464k         zircon-services
    p:1104  298.3k    296k     28k crashlogger
    p:1290  242.3k    240k     28k netsvc
    p:2115  362.3k    360k     28k sh:console
    p:2334  266.3k    264k     28k sh:vc
    p:2441  306.3k    304k     28k /boot/bin/ps
TASK           PSS PRIVATE  SHARED NAME
```

**PSS** (proportional shared state) is a number of bytes that estimates how much
in-process mapped physical memory the process consumes. Its value is `PRIVATE +
(SHARED / sharing-ratio)`, where `sharing-ratio` is based on the number of
processes that share each of the pages in this process.

The intent is that, e.g., if four processes share a single page, 1/4 of the
bytes of that page is included in each of the four process's `PSS`. If two
processes share a different page, then each gets 1/2 of that page's bytes.

**PRIVATE** is the number of bytes that are mapped only by this process. I.e.,
no other process maps this memory. Note that this does not account for private
VMOs that are not mapped.

**SHARED** is the number of bytes that are mapped by this process and at least
one other process. Note that this does not account for shared VMOs that are not
mapped. It also does not indicate how many processes share the memory: it could
be 2, it could be 50.

### Visualize memory usage

If you have a Fuchsia build, you can use treemap to visualize memory usage by
the system.

 1. On your host machine, run the following command from the root of your
    Fuchsia checkout:

    ```./scripts/fx shell memgraph -vt | ./scripts/memory/treemap.py > mem.html```

 2. Open `mem.html` in a browser.

The `memgraph` tool generates a JSON description of system task and memory
information, which is then parsed by the `treemap.py` script. `-vt` says
to include VMOs and threads in the output.

### Dump a process's detailed memory maps

If you want to see why a specific process uses so much memory, you can run the
`vmaps` tool on its koid (koid is the ID that shows up when running ps) to see
what it has mapped into memory.

```
$ vmaps help
Usage: vmaps <process-koid>

Dumps a process's memory maps to stdout.

First column:
  "/A" -- Process address space
  "/R" -- Root VMAR
  "R"  -- VMAR (R for Region)
  "M"  -- Mapping

  Indentation indicates parent/child relationship.
```

Column tags:

-   `:sz`: The virtual size of the entry, in bytes. Not all pages are
    necessarily backed by physical memory.
-   `:res`: The amount of memory "resident" in the entry, in bytes; i.e., the
    amount of physical memory that backs the entry. This memory may be private
    (only accessible by this process) or shared by multiple processes.
-   `:vmo`: The `koid` of the VMO mapped into this region.

```
$ vmaps 2470
/A ________01000000-00007ffffffff000    128.0T:sz                    'proc:2470'
/R ________01000000-00007ffffffff000    128.0T:sz                    'root'
...
# This 'R' region is a dynamic library. The r-x section is .text, the r--
# section is .rodata, and the rw- section is .data + .bss.
R  00000187bc867000-00000187bc881000      104k:sz                    'useralloc'
 M 00000187bc867000-00000187bc87d000 r-x   88k:sz   0B:res  2535:vmo 'libfdio.so'
 M 00000187bc87e000-00000187bc87f000 r--    4k:sz   4k:res  2537:vmo 'libfdio.so'
 M 00000187bc87f000-00000187bc881000 rw-    8k:sz   8k:res  2537:vmo 'libfdio.so'
...
# This 2MB anonymous mapping is probably part of the heap.
M  0000246812b91000-0000246812d91000 rw-    2M:sz  76k:res  2542:vmo 'mmap-anonymous'
...
# This region looks like a stack: a big chunk of virtual space (:sz) with a
# slightly-smaller mapping inside (accounting for a 4k guard page), and only a
# small amount actually committed (:res).
R  0000358923d92000-0000358923dd3000      260k:sz                    'useralloc'
 M 0000358923d93000-0000358923dd3000 rw-  256k:sz  16k:res  2538:vmo ''
...
# The stack for the initial thread, which is allocated differently.
M  0000400cbba84000-0000400cbbac4000 rw-  256k:sz   4k:res  2513:vmo 'initial-stack'
...
# The vDSO, which only has .text and .rodata.
R  000047e1ab874000-000047e1ab87b000       28k:sz                    'useralloc'
 M 000047e1ab874000-000047e1ab87a000 r--   24k:sz  24k:res  1031:vmo 'vdso/full'
 M 000047e1ab87a000-000047e1ab87b000 r-x    4k:sz   4k:res  1031:vmo 'vdso/full'
...
# The main binary for this process.
R  000059f5c7068000-000059f5c708d000      148k:sz                    'useralloc'
 M 000059f5c7068000-000059f5c7088000 r-x  128k:sz   0B:res  2476:vmo '/boot/bin/sh'
 M 000059f5c7089000-000059f5c708b000 r--    8k:sz   8k:res  2517:vmo '/boot/bin/sh'
 M 000059f5c708b000-000059f5c708d000 rw-    8k:sz   8k:res  2517:vmo '/boot/bin/sh'
...
```

> You can also display memory mappings using the `aspace` command in
> [zxdb](/docs/development/debugger/README.md).

### Dump all VMOs associated with a process

```
vmos <pid>
```

This will also show unmapped VMOs, which neither `ps` nor `vmaps` currently
account for.

It also shows whether a given VMO is a child, along with its parent's koid.

```
$ vmos 1118
rights  koid parent #chld #map #shr    size   alloc name
rwxmdt  1170      -     0    1    1      4k      4k stack: msg of 0x5a
r-xmdt  1031      -     2   28   14     28k     28k vdso/full
     -  1298      -     0    1    1      2M     68k jemalloc-heap
     -  1381      -     0    3    1    516k      8k self-dump-thread:0x12afe79c8b38
     -  1233   1232     1    1    1   33.6k      4k libbacktrace.so
     -  1237   1233     0    1    1      4k      4k data:libbacktrace.so
...
     -  1153   1146     1    1    1  883.2k     12k ld.so.1
     -  1158   1153     0    1    1     16k     12k data:ld.so.1
     -  1159      -     0    1    1     12k     12k bss:ld.so.1
rights  koid parent #chld #map #shr    size   alloc name
```

Columns:

-   `rights`: If the process points to the VMO via a handle, this column shows
    the rights that the handle has, zero or more of:
    -   `r`: `ZX_RIGHT_READ`
    -   `w`: `ZX_RIGHT_WRITE`
    -   `x`: `ZX_RIGHT_EXECUTE`
    -   `m`: `ZX_RIGHT_MAP`
    -   `d`: `ZX_RIGHT_DUPLICATE`
    -   `t`: `ZX_RIGHT_TRANSFER`
    -   **NOTE**: Non-handle entries will have a single '-' in this column.
-   `koid`: The koid of the VMO, if it has one. Zero otherwise. A VMO without a
    koid was created by the kernel, and has never had a userspace handle.
-   `parent`: The koid of the VMO's parent, if it's a child.
-   `#chld`: The number of active children of the VMO.
-   `#map`: The number of times the VMO is currently mapped into VMARs.
-   `#shr`: The number of processes that map (share) the VMO.
-   `size`: The VMO's current size, in bytes.
-   `alloc`: The amount of physical memory allocated to the VMO, in bytes.
    -   **NOTE**: If this column contains the value `phys`, it means that the
        VMO points to a raw physical address range like a memory-mapped device.
        `phys` VMOs do not consume RAM.
-   `name`: The name of the VMO, or `-` if its name is empty.

To relate this back to `ps`: each VMO contributes, for its mapped portions
(since not all or any of a VMO's pages may be mapped):

```
PRIVATE =  #shr == 1 ? alloc : 0
SHARED  =  #shr  > 1 ? alloc : 0
PSS     =  PRIVATE + (SHARED / #shr)
```

> You can also display VMO information using the `handle` command in
> [zxdb](/docs/development/debugger/kernel_objects.md).

### Dump "hidden" (unmapped and kernel) VMOs

Note: This is a kernel command, and will print to the kernel console.

```
k zx vmos hidden
```

Similar to `vmos <pid>`, but dumps all VMOs in the system that are not mapped
into any process:

-   VMOs that userspace has handles to but does not map
-   VMOs that are mapped only into kernel space
-   Kernel-only, unmapped VMOs that have no handles

A `koid` value of zero means that only the kernel has a reference to that VMO.

A `#map` value of zero means that the VMO is not mapped into any address space.

**See also**: `k zx vmos all`, which dumps all VMOs in the system. **NOTE**:
It's very common for this output to be truncated because of kernel console
buffer limitations, so it's often better to combine the `k zx vmos hidden`
output with a `vmaps` dump of each user process.

### Limitations

Neither `ps` nor `vmaps` currently account for:

-   VMOs or VMO subranges that are not mapped. E.g., you could create a VMO,
    write 1G of data into it, and it won't show up here.

None of the process-dumping tools account for:

-   Multiply-mapped pages. If you create multiple mappings using the same range
    of a VMO, any committed pages of the VMO will be counted as many times as
    those pages are mapped. This could be inside the same process, or could be
    between processes if those processes share a VMO.

    Note that "multiply-mapped pages" includes copy-on-write.
-   Underlying kernel memory overhead for resources allocated by a process.
    E.g., a process could have a million handles open, and those handles consume
    kernel memory.

    You can look at process handle consumption with the `k zx ps` command; run
    `k zx ps help` for a description of its columns.
-   Copy-on-write (COW) cloned VMOs. The clean (non-dirty, non-copied) pages of
    a clone will not count towards "shared" for a process that maps the clone,
    and those same pages may mistakenly count towards "private" of a process
    that maps the parent (cloned) VMO.

    TODO(dbort): Fix this; the tools were written before COW clones existed.

## Kernel memory

### Dump system memory arenas and kernel heap usage

Running `kstats -m` will continuously dump information about physical memory
usage and availability.

```
$ kstats -m
--- 2017-06-07T05:51:08.021Z ---
mem total      free      VMOs     kheap     kfree     wired       mmu       ipc     other
    2048M   1686.4M    317.8M      5.1M      0.9M     17.8M     20.0M      0.1M      0.0M

--- 2017-06-07T05:51:09.021Z ---
...
```

Fields:

-   The `-t` option show the timestamp `2017-06-07T05:51:08.021Z`, when the stats
    were collected, as an ISO 8601 string.
-   `total`: The total amount of physical memory available to the system.
-   `free`: The amount of unallocated memory.
-   `VMOs`: The amount of memory committed to VMOs, both kernel and user. A
    superset of all userspace memory. Does not include certain VMOs that fall
    under `wired`.
-   `kheap`: The amount of kernel heap memory marked as allocated.
-   `kfree`: The amount of kernel heap memory marked as free.
-   `wired`: The amount of memory reserved by and mapped into the kernel for
    reasons not covered by other fields in this struct. Typically for readonly
    data like the ram disk and kernel image, and for early-boot dynamic memory.
-   `mmu`: The amount of memory used for architecture-specific MMU metadata like
    page tables.
-   `ipc`: The amount of memory used for interprocess communication.
-   `other`: Everything else as `other`.

### Dump the kernel address space

Note: This is a kernel command, and will print to the kernel console.

```
k zx asd kernel
```

Dumps the kernel's VMAR/mapping/VMO hierarchy, similar to the `vmaps` tool for
user processes.

```
$ k zx asd kernel
as 0xffffffff80252b20 [0xffffff8000000000 0xffffffffffffffff] sz 0x8000000000 fl 0x1 ref 71 'kernel'
  vmar 0xffffffff802529a0 [0xffffff8000000000 0xffffffffffffffff] sz 0x8000000000 ref 1 'root'
    map 0xffffff80015f89a0 [0xffffff8000000000 0xffffff8fffffffff] sz 0x1000000000 mmufl 0x18 vmo 0xffffff80015f8890/k0 off 0 p ages 0 ref 1 ''
      vmo 0xffffff80015f8890/k0 size 0 pages 0 ref 1 parent k0
    map 0xffffff80015f8b30 [0xffffff9000000000 0xffffff9000000fff] sz 0x1000 mmufl 0x18 vmo 0xffffff80015f8a40/k0 off 0 pages 0 ref 1 ''
      object 0xffffff80015f8a40 base 0x7ffe2000 size 0x1000 ref 1
    map 0xffffff80015f8cc0 [0xffffff9000001000 0xffffff9000001fff] sz 0x1000 mmufl 0x1a vmo 0xffffff80015f8bd0/k0 off 0 pages 0 ref 1 ''
      object 0xffffff80015f8bd0 base 0xfed00000 size 0x1000 ref 1
...
```
