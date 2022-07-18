<!---

# DMA

Direct Memory Access (**DMA**) is a feature that allows hardware to access
memory without CPU intervention.
At the highest level, the hardware is given the source and destination of the
memory region to transfer (along with its size) and told to copy the data.
Some hardware peripherals even support the ability to do multiple
"scatter / gather" style operations, where several copy operations
can be performed, one after the other, without additional CPU intervention.

--->

# DMA 

直接内存访问（ **DMA**）是一种允许硬件不经过 CPU 介入访问内存的特性。
在最高层面上，硬件被告知要传输的存储区域的源和目的地（及其大小），并被告知要复制数据。
某些硬件周边件甚至具有多“分散/收集”类型操作的能力，这样可以执行多个复制操作，一个接一个，而不需要额外的 CPU 干预。

<!---

## DMA considerations

In order to fully appreciate the issues involved, it's important to
keep the following in mind:

*   each process operates in a virtual address space,
*   an MMU can map a contiguous virtual address range onto multiple,
    discontiguous physical address ranges (and vice-versa),
*   each process has a limited window into physical address space,
*   some peripherals support their own virtual addresses
    with an Input / Output Memory Management Unit (**IOMMU**).

Let's discuss each point in turn.

--->

## DMA 注意事项

为了充分理解所涉及的问题，重要的是要记住以下几点：

* 每一个进程都在虚拟地址空间中运行，
*  MMU 可以映射连续虚拟地址范围到多个非连续物理地址范围内（反之亦然），
* 每一个进程在物理地址空间中都有限制窗口，
* 某些周边件用输入/输出内存管理单元（**IOMMU**）支持他们自身的虚拟地址。

让我们依次来讨论以上提到的每点事项。

<!---


### Virtual, physical, and device-physical addresses

The addresses that the process has access to are virtual; that is, they are
an illusion created by the CPU's Memory Management Unit (**MMU**).
A virtual address is mapped by the MMU into a physical address.
The mapping granularity is based on a parameter called "page size," which
is at least 4k bytes, though larger sizes are available on modern processors.

--->

### 虚拟，物理和实体设备地址

进程访问的地址为虚拟地址；这就是说，它们是由 CPU 的内存管理单元（**MMU**）创造的假象。虚拟地址由 MMU 映射到物理地址中。映射的粒度是基于一个叫做“页表大小”的参数决定，这个参数最少为 4K 字节，而现代处理器上可以支持更大的大小。

![Figure: Relationship between virtual and physical addresses](images/dma-000-cropped.png)

<!---

In the diagram above, we show a specific process (process 12) with a number of
virtual addresses (in blue).
The MMU is responsible for mapping the blue virtual addresses into CPU physical
bus addresses (red).
Each process has its own mapping; so even though process 12 has a virtual address
`300`, some other process may also have a virtual address `300`.
That other process's virtual address `300` (if it exists) would be mapped
to a different physical address than the one in process 12.

> Note that we've used small decimal numbers as "addresses" to keep the discussion simple.
> In reality, each square shown above represents a page of memory (4k or more),
> and is identified by a 32 or 64 bit value (depending on the platform).

--->

在上图中，我们展示了一个特定进程（进程12）带有一组虚拟地址（蓝色框中）。
MMU 负责映射蓝色的虚拟地址到 CPU 物理总线地址上（红色框中）。
每一个进程有它自己的映射规则；所以即使进程12有虚拟地址`300`，一些其他的进程可能也有虚拟地址`300`。
那些其他进程中的虚拟地址`300`（如果它存在的话）将被映射到和进程12中不一样的物理地址中。

> 注意：为了保持讨论的简单，我们使用很小的十进制数作为”地址”。
> 事实上，每一个上述展示的方块代表了一页内存（ 4K 或者更多），
> 并由32位或64位值来标识（取决于平台决定）。

<!---

The key points shown in the diagram are:

1.  virtual addresses can be allocated in groups (three are shown, `300`-`303`, `420`-`421`,
    and `770`-`771`),
2.  virtually contiguous (e.g., `300`-`303`) is not necessarily physically contiguous.
3.  some virtual addresses are not mapped (for example, there is no virtual address
    `304`)
4.  not all physical addresses are available to each process (for example, process
    `12` doesn't have access to physical address `120`).

Depending on the hardware available on the platform, a device's address space
may or may not follow a similar translation.
Without an IOMMU, the addresses that the peripheral uses are the same as
the physical addresses used by the CPU:

--->

图中展示的关键点是：

1. 虚拟地址可以按组分配（下面展示三组， `300`-`303`, `420`-`421`和`770`-`771`），
2. 虚拟地址连续（例如 `300`-`303`）是不需要物理地址连续，
3. 某些虚拟地址是没有被映射的（例如，这里没有虚拟地址`304`）
4. 不是所有的物理地址对于每一个进程都是可用的（例如，进程`12`不能访问物理地址`120`）。

根据平台上的硬件可用性，设备地址空间可以或不用遵照相同的转译方式。
没有 IOMMU，周边件使用的地址就是和 CPU 使用的物理地址相同。

![Figure: A device that doesn't use an IOMMU](images/dma-002-cropped.png)

<!---

In the diagram above, portions of the device's address space (for example, a
frame buffer, or control registers), appear directly in the CPU's physical
address range.
That is to say, the device occupies physical addresses `122` through `125`
inclusive.

In order for the process to access the device's memory, it would need to create
an MMU mapping from some virtual addresses to the physical addresses `122` through
`125`.
We'll see how to do that, below.

But with an IOMMU, the addresses seen by a peripheral may be different than
the CPU's physical addresses:

--->

在上图中，设备地址空间（例如，一个帧缓存或控制寄存器）的部分直接出现在 CPU 的物理地址范围内。
这也就是说，设备占用从`122` 到`125`（包含）的物理地址。

为了让进程能够访问设备内存，需要创建一个从`122`到`125`的某些虚拟地址到物理地址的 MMU 映射。
我们可以从下图中看到怎样实现。

但是在 IOMMU 中，周边件看到的地址可能和 CPU 的物理地址不同：

![Figure: A device that uses an IOMMU](images/dma-001-cropped.png)

<!---

Here, the device has its own "device-physical" addresses that it knows about,
that is, addresses `0` through `3` inclusive.
It's up to the IOMMU to map the device-physical addresses `0` through `3`
into CPU physical addresses `109`, `110`, `101`, and `119`, respectively.

In this scenario, in order for the process to use the device's memory, it needs
to arrange two mappings:

*   one set from the virtual address space (e.g., `300` through `303`) to the
    CPU physical address space (`109`, `110`, `101`, and `119`, respectively),
    through the MMU, and
*   one set from the CPU physical address space (addresses `109`, `110`, `101`,
    and `119`) to the device-physical addresses (`0` through `3`) through the IOMMU.

While this may seem complicated, Zircon provides an abstraction that removes
the complexity.

Also, as we'll see below, the reason for having an IOMMU, and the benefits provided,
are similar to those obtained by having an MMU.

--->

现在，设备拥有它自己知道的“实体设备”地址，也就是说，地址从`0`到`3`（包含`3`）。
由 IOMMU 将实体物理地址`0` 到`3`分别映射到 CPU 物理地址  `109`, `110`, `101` 和`119`上。

在这种规则下，为了进程能使用设备内存，就需要安排两个映射：

* 一组通过 MMU 完成从虚拟地址空间（例如`300`到`30`）到 CPU 物理地址空间（分别到`109`, `110`, `101`, 和 `119`上）的映射，
* 一组通过 IOMMU 从 CPU 物理地址空间（`109`, `110`, `101`, 和 `119`）到实体设备地址（`0`到`3`）的映射。

虽然这看起来很复杂，但 Zircon 提供了一个抽象的概念来消除复杂性。

同样，正如我们在上图中看到的，拥有 IOMMU 的原因和它提供的好处和使用 MMU 是一样的。

<!---

### Contiguity of memory

When you allocate a large chunk of memory (e.g. with **calloc()**),
your process will, of course, see a large, contiguous virtual address range.
The MMU creates the illusion of contiguous memory at the virtual addressing
level, even though the MMU may choose to back that memory area with physically
discontiguous memory at the physical address level.

Furthermore, as processes allocate and deallocate memory, the mapping of
physical memory to virtual address space tends to become more
complex, encouraging more "swiss cheese" holes to appear (that is,
more discontiguities in the mapping).

Therefore, it's important to keep in mind that contiguous virtual addresses
are not necessarily contiguous physical addresses, and indeed that contiguous
physical memory becomes more precious over time.

--->

### 内存连续性

当你分配了一大块内存（例如使用**calloc()**）时，你当然能在进程中看到一块大的，连续的虚拟地址范围。 
MMU 创建了一个连续内存在虚拟地址上的错觉，甚至 MMU 可以在物理地址层级上不连续内存中选择内存区域。

更进一步讲，因为进程分配和解除分配内存，所以物理内存到虚拟地址空间的映射则趋向于越来越复杂，这样会有更多的碎片（类”瑞士奶酪“的孔洞）出现（这也就是更多在映射中的非连续性）。

因此，重要的是要记住，连续的虚拟地址不一定是连续的物理地址，事实上，随着时间的推移，连续的物理内存会变得越来越珍贵。

<!---

### Access controls

Another benefit of the MMU is that processes are limited in their view of
physical memory (for security and reliability reasons).
The impact on drivers, though, is that a process has to specifically request
a mapping from virtual address space to physical address space, and
have the requisite privilege in order to do so.

--->

### 访问控制

MMU 的另一个优势在于限制物理内存的视角（出于安全和可靠性的原因）。
尽管对驱动有一些影响，可能进程需要特别地请求虚拟地址空间到物理地址空间的映射，并且还要有必要权限去实现。

<!---

### IOMMU

Contiguous physical memory is generally preferred.
It's more efficient to do one transfer (with one source address and one
destination address) than it is to set up and manage multiple individual
transfers (which may require CPU intervention between each transfer in
order to set up the next one).

The IOMMU, if available, alleviates this problem by doing the same thing for
the peripherals that the CPU's MMU does for the process &mdash; it gives the peripheral
the illusion that it's dealing with a contiguous address space by
mapping multiple discontiguous chunks into a virtually contiguous space.
By limiting the mapping region, the IOMMU also provides security (in the same way as
the MMU does), by preventing the peripheral from accessing memory that's not "in scope"
for the current operation.

--->

### IOMMU

连续物理内存通常是首选项。
比起建立和管理多个独立转换（为了建立下一次转换，在每次转移时都可能需要 CPU 介入），完成一次转换（使用一个源地址和一个目的地地址）是更有效率的。

如果 IOMMU 可用，通过对外围设备执行与 CPU 的 MMU 对进程执行的相同操作，可以缓解这个问题 — 使用映射多个非连续块到一个虚拟连续空间，这让外围设备产生一种它们可以处理一个连续地址空间的假象。
通过限制映射区域， IOMMU 同样可以提供安全性（和 MMU 使用同样的方式），防止外设访问不在当前操作“范围内“的内存。

<!---

### Tying it all together

So, it may appear that you need to worry about virtual, physical, and device-physical
address spaces when you are writing your driver.
But that's not the case.

--->

### 综上所述

所以，当你在写自己的驱动时，就会出现你需要关注虚拟，物理和实体设备地址空间的情况。
但事实并非如此。

<!---

## DMA and your driver

Zircon provides a set of functions that allow you to cleanly deal with all of the
above.
The following work together:

*   a Bus Transaction Initiator ([BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md)), and
*   a Virtual Memory Object ([VMO](/docs/reference/kernel_objects/vm_object.md)).

The [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md)
kernel object provides an abstraction of the model, and an API to deal with
physical (or device-physical) addresses associated with
[VMO](/docs/reference/kernel_objects/vm_object.md)s.

In your driver's initialization, call
**pci_get_bti()**
to obtain a [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md) handle:

--->

## DMA 和你的驱动

Zircon 提供一组功能，让你可以简单的处理以上提到的所有内容。
以下是要做的事：

* 总线交易启动器（[BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md)），和
* 虚拟内存对象（[VMO](/docs/reference/kernel_objects/vm_object.md)）。

[BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md) 内核对象提供一个抽象的模型和 API 来处理 [VMO](/docs/reference/kernel_objects/vm_object.md) 相关的物理（或实体物理）地址。

在你的驱动初始化时，调用
**pci_get_bti()**
来获取 [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md) 句柄。

```c
zx_status_t pci_get_bti(const pci_protocol_t* pci,
                        uint32_t index,
                        zx_handle_t* bti_handle);
```

<!---

The **pci_get_bti()**
function takes a `pci` protocol pointer (just like all the other **pci_...()** functions
discussed above) and an `index` (reserved for future use, use `0`).
It returns a [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md)
handle through the `bti_handle` pointer argument.

Next, you need a [VMO](/docs/reference/kernel_objects/vm_object.md).
Simplistically, you can think of the [VMO](/docs/reference/kernel_objects/vm_object.md)
as a pointer to a chunk of memory,
but it's more than that &mdash; it's a kernel object that represents a set
of virtual pages (that may or may not have physical pages committed to them),
which can be mapped into the virtual address space of the driver process.
(It's even more than that, but that's a discussion for a different chapter.)

Ultimately, these pages serve as the source or destination of the DMA transfer.

There are two functions,
[**zx_vmo_create()**](/docs/reference/syscalls/vmo_create.md)
and
[**zx_vmo_create_contiguous()**](/docs/reference/syscalls/vmo_create_contiguous.md)
that allocate memory and bind it to a [VMO](/docs/reference/kernel_objects/vm_object.md):

--->

**pci_get_bti()**
函数获取`pci`协议指针（就像上述讨论的所有其他**pci_...()**函数一样）和`index` （保留给未来使用，现在使用`0`）。
它通过`bti_handle`指针参数返回了一个 [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md) 句柄。

接下来，你需要一个 [VMO](/docs/reference/kernel_objects/vm_object.md) 。
简而言之，你可以认为 [VMO](/docs/reference/kernel_objects/vm_object.md) 作为一个内存块的指针，但是比指针更多的是 — 它是一个代表了一组虚拟页的内核对象（可能有，也可能没有确定的物理页），它可以被映射到驱动进程的虚拟地址空间。（它甚至还有其他更多意义，但我们将在不同的章节中再讨论。）

最后，这些页作为 DMA 传输的源或目的地。

这里有两个函数，[**zx_vmo_create()**](/docs/reference/syscalls/vmo_create.md)
和
[**zx_vmo_create_contiguous()**](/docs/reference/syscalls/vmo_create_contiguous.md)
可以分配内存并绑定到 [VMO](/docs/reference/kernel_objects/vm_object.md)上：

```c
zx_status_t zx_vmo_create(uint64_t size,
                          uint32_t options,
                          zx_handle_t* out);

zx_status_t zx_vmo_create_contiguous(zx_handle_t bti,
                                     size_t size,
                                     uint32_t alignment_log2,
                                     zx_handle_t* out);
```

<!---

As you can see, they both take a `size` parameter indicating the number of bytes required,
and they both return a [VMO](/docs/reference/kernel_objects/vm_object.md) (through `out`).
They both allocate virtually contiguous pages, for a given size.

> Note that this differs from the standard C library memory allocation functions,
> (e.g., **malloc()**), which allocate virtually contiguous memory, but without
> regard to page boundaries. Two small **malloc()** calls in a row might allocate
> two memory regions from the *same* page, for instance, whereas
> the [VMO](/docs/reference/kernel_objects/vm_object.md)
> creation functions will always allocate memory starting with a *new* page.

--->

正如你所了解到，它们同样都是获取`size`参数代表所需的字节数，并且它们同样都返回了一个 [VMO](/docs/reference/kernel_objects/vm_object.md)  （通过`out`句柄）。
它们都对给定的大小分配虚拟的连续页。

> 注意：这和标准 C 库的内存分配函数不同，（例如**malloc()**），它分配了虚拟连续内存，但不考虑页的界限。连续两次调用小的**malloc()**可能从*相同*的页中分配两个内存区域，但是，例如 [VMO](/docs/reference/kernel_objects/vm_object.md) 创建函数却总是从一个*新*的页开始分配内存。

<!---

The
[**zx_vmo_create_contiguous()**](/docs/reference/syscalls/vmo_create_contiguous.md)
function does what
[**zx_vmo_create()**](/docs/reference/syscalls/vmo_create.md)
does, *and* ensures that the pages are suitably
organized for use with the specified [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md)
(which is why it needs the [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md) handle).
It also features an `alignment_log2` parameter that can be used to specify a minimum
alignment requirement.
As the name suggests, it must be an integer power of 2 (with the value `0` indicating
page aligned).

At this point, you have two "views" of the allocated memory:

*   one contiguous virtual address space that represents memory
    from the point of view of the driver, and
*   a set of (possibly contiguous, possibly committed) physical pages
    for use by the peripheral.

Before using these pages, you need to ensure that they are present in memory (that is,
"committed" &mdash; the physical pages are accessible to your process), and that the
peripheral has access to them (through the IOMMU if present).
You will also need the addresses of the pages (from the point of view of the device)
so that you can program the DMA controller on your device to access them.

--->

[**zx_vmo_create_contiguous()**](/docs/reference/syscalls/vmo_create_contiguous.md)
函数则和
[**zx_vmo_create()**](/docs/reference/syscalls/vmo_create.md)
函数完成相同功能，*并且*确保页对于特定的 [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md) 被适当地组织起来以供使用（这也是它为什么需要 [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md) 句柄的原因）。
它同样提供一个`alignment_log2`参数的功能，可以被用来明确最小对齐需求。
正如名字所表达的，它必须是一个2的整数次方（数值`0`表示页对齐）。

在这一点上，你将对分配内存有两种“视角”：

- 一个连续虚拟地址空间，代表驱动看到的内存视角，和
- 一组（可能连续，可能确定）物理页，代表外设使用。

在使用这些页之前，你需要确保它们在内存中存在（这就是“确定”的意思—你的进程可以访问这些物理页），并且外设也可以访问它们（如果 IOMMU 存在，则通过它访问）。
同样你也需要页的地址（从设备的角度看），这样你可以在你设备上操作 DMA 控制器去访问它们。

<!---

The
[**zx_bti_pin()**](/docs/reference/syscalls/bti_pin.md)
function is used to do all that:

--->

[**zx_bti_pin()**](/docs/reference/syscalls/bti_pin.md)
函数用来做所有的事：

```c
#include <zircon/syscalls.h>

zx_status_t zx_bti_pin(zx_handle_t bti, uint32_t options,
                       zx_handle_t vmo, uint64_t offset, uint64_t size,
                       zx_paddr_t* addrs, size_t addrs_count,
                       zx_handle_t* pmt);
```

<!---

There are 8 parameters to this function:

Parameter       | Purpose
----------------|------------------------------------
`bti`           | the [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md) for this peripheral
`options`       | options (see below)
`vmo`           | the [VMO](/docs/reference/kernel_objects/vm_object.md) for this memory region
`offset`        | offset from the start of the [VMO](/docs/reference/kernel_objects/vm_object.md)
`size`          | total number of bytes in [VMO](/docs/reference/kernel_objects/vm_object.md)
`addrs`         | list of return addresses
`addrs_count`   | number of elements in `addrs`
`pmt`           | returned [PMT](/docs/reference/kernel_objects/pinned_memory_token.md) (see below)

--->

在函数中有8个参数：

| Parameter     | Purpose                                                      |
| ------------- | ------------------------------------------------------------ |
| `bti`         | 这个外设的 [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md) |
| `options`     | 选项(见后续描述)                                             |
| `vmo`         | 这个内存区域的 [VMO](/docs/reference/kernel_objects/vm_object.md) |
| `offset`      | [VMO](/docs/reference/kernel_objects/vm_object.md) 的起始偏移 |
| `size         | [VMO](/docs/reference/kernel_objects/vm_object.md) 中的所有字节数 |
| `addrs`       | 返回地址的列表                                               |
| `addrs_count` | `addrs` 的元素数量                                           |
| `pmt`         | 返回 [PMT](/docs/reference/kernel_objects/pinned_memory_token.md) (见后续描述) |

<!---

The `addrs` parameter is a pointer to an array of `zx_paddr_t` that you supply.
This is where the peripheral addresses for each page are returned into.
The array is `addrs_count` elements long, and must match the count of
elements expected from
[**zx_bti_pin()**](/docs/reference/syscalls/bti_pin.md).

> The values written into `addrs` are suitable for programming the peripheral's
> DMA controller &mdash; that is, they take into account any translations that
> may be performed by an IOMMU, if present.

--->

`addrs`参数是一组你要提供的`zx_paddr_t`指针。
这就是每个页的外设地址被返回的地方。
数组为`addrs_count`长度，而且必须匹配[**zx_bti_pin()**](/docs/reference/syscalls/bti_pin.md)期望的长度。

> `addrs`中写入的数据对于规划外设的 DMA 控制器要适配— 也就是说，如果存在的话，需要考虑可能由 IOMMU 执行的任何转译。

<!---

On a technical note, the other effect of
[**zx_bti_pin()**](/docs/reference/syscalls/bti_pin.md)
is that the kernel will ensure those pages are not decommitted
(i.e., moved or reused) while pinned.

The `options` argument is actually a bitmap of options:

Option                  | Purpose
------------------------|--------------------------------
`ZX_BTI_PERM_READ`      | pages can be read by the peripheral (written by the driver)
`ZX_BTI_PERM_WRITE`     | pages can be written by the peripheral (read by the driver)
`ZX_BTI_COMPRESS`       | (see "Minimum contiguity property," below)

For example, refer to the diagrams above showing "Device #3".
If an IOMMU is present, `addrs` would contain `0`, `1`, `2`, and `3` (that is,
the device-physical addresses).
If no IOMMU is present, `addrs` would contain `109`, `110`, `101`, and `119` (that is,
the physical addresses).

--->

从技术角度来看，[**zx_bti_pin()**](/docs/reference/syscalls/bti_pin.md) 的其他影响在于驱动将确保那些页在选中时没有被回收（例如移除或重用）。

`options` 参数是一个选项的位图：

| Option              | Purpose                    |
| ------------------- | -------------------------- |
| `ZX_BTI_PERM_READ`  | 外设可读的页 (由驱动写入)  |
| `ZX_BTI_PERM_WRITE` | 外设可写的页（由驱动读取） |
| `ZX_BTI_COMPRESS`   | （参见下述“最小邻接性”）   |

例如，参考上述展示的"Device #3"图片。
如果存在 IOMMU ， `addrs`将包含 `0`, `1`, `2`, 和 `3` （也就是设备-物理地址）。
如果不存在 IOMMU，`addrs`将包含 `109`, `110`, `101`, 和 `119` （也就是物理地址）。

<!---

### Permissions

Keep in mind that the permissions are from the perspective
*of the peripheral*, and not the driver.
For example, in a block device **write** operation, the device **reads** from memory pages and
therefore the driver specifies `ZX_BTI_PERM_READ`, and vice versa in the block device read.

--->

### 权限

请记住，这些权限是从*外设*的角度出发，而不是驱动。
例如，在块设备执行**写** 操作，设备从内存页中执行**读取**，因此驱动规定`ZX_BTI_PERM_READ`，反之亦然，在块设备中读取的话。

<!---

### Minimum contiguity property

By default, each address returned through `addrs` is one page long.
Larger chunks may be requested by setting the `ZX_BTI_COMPRESS` option
in the `options` argument.
In that case, the length of each entry returned corresponds to the "minimum contiguity" property.
While you can't set this property, you can read it with
[**zx_object_get_info()**](/docs/reference/syscalls/object_get_info.md).
Effectively, the minimum contiguity property is a guarantee that
[**zx_bti_pin()**](/docs/reference/syscalls/bti_pin.md)
will always be able to return addresses that are contiguous for at least that many bytes.

For example, if the property had the value 1MB, then a call to
[**zx_bti_pin()**](/docs/reference/syscalls/bti_pin.md)
with a requested size of 2MB would return at most two physically-contiguous runs.
If the requested size was 2.5MB, it would return at most three physically-contiguous runs,
and so on.

--->

### 最小邻接性

默认情况，每一个通过`addrs`返回的地址都是一个页的长度。
更大的块通过在`options`参数中设置`ZX_BTI_COMPRESS`选项来申请。
在这种使用场景下，每一个条目返回的长度对应“最小邻接"属性。
当你没有设置这个属性时，你可以通过[**zx_object_get_info()**](/docs/reference/syscalls/object_get_info.md) 读取。
实际上，最小邻接性是[**zx_bti_pin()**](/docs/reference/syscalls/bti_pin.md) 总是可以返回至少有那么多字节的连续地址的保证。

例如如果设置为 1 MB，那么调用 [**zx_bti_pin()**](/docs/reference/syscalls/bti_pin.md) 请求 2 MB 页将返回最多两个物理连续的地址。
如果请求的大小为 2.5 MB，它将返回最多三个物理连续的地址，等等。

<!---

### Pinned Memory Token (PMT)

[`zx_bti_pin()`](/docs/reference/syscalls/bti_pin.md) returns a Pinned Memory Token
([PMT](/docs/reference/kernel_objects/pinned_memory_token.md))
upon success in the *pmt* argument.
The driver must call [`zx_pmt_unpin()`](/docs/reference/syscalls/pmt_unpin.md) when the device is
done with the memory transaction to unpin and revoke access to the memory pages by the device.

--->

### 选中内存令牌 （ PMT ）

[`zx_bti_pin()`](/docs/reference/syscalls/bti_pin.md)  运行成功，则通过*pmt*参数返回选中内存令牌 （[PMT](/docs/reference/kernel_objects/pinned_memory_token.md)）。
当设备完成内存转移，取消固定并撤销对内存的访问，驱动必须调用[`zx_pmt_unpin()`](/docs/reference/syscalls/pmt_unpin.md) 。

<!---

## Advanced topics

### Cache Coherency

On fully DMA-coherent architectures, hardware ensures the data in the CPU cache is the same
as the data in main memory without software intervention. Not all architectures are
DMA-coherent. On these systems, the driver must ensure the CPU cache is made coherent by
invoking appropriate cache operations on the memory range before performing DMA operations,
so that no stale data will be accessed.

To invoke cache operations on the memory represented by
[VMO](/docs/reference/kernel_objects/vm_object.md)s, use the
[`zx_vmo_op_range()`](/docs/reference/syscalls/vmo_op_range.md)
syscall.
Prior to a peripheral-read
(driver-write) operation, clean the cache using `ZX_VMO_OP_CACHE_CLEAN` to write out dirty
data to main memory. Prior to a peripheral-write (driver-read), mark the cache lines
as invalid using `ZX_VMO_OP_CACHE_INVALIDATE` to ensure data is fetched from main
memory on the next access.

--->

## 拓展话题

### 缓存一致性

在完全 DMA 一致性的架构中，硬件在没有软件干预下，确保 CPU 缓存中的数据和在主存中的数据相同。但不是所有的架构都是 DMA 一致性的。在这些系统上，驱动必须通过在执行 DMA 操作之前调用内存范围上的适当缓存操作来确保 CPU 缓存是一致的，这样就不会访问到旧数据了。

为了在 [VMO](/docs/reference/kernel_objects/vm_object.md) 代表的内存上调用缓存操作，使用[`zx_vmo_op_range()`](/docs/reference/syscalls/vmo_op_range.md) 系统调用接口。
在外设读取（驱动写入）操作之前，使用`ZX_VMO_OP_CACHE_CLEAN`清除缓存，来写出脏数据到主存中。在外设写入（驱动读取）操作之前，使用`ZX_VMO_OP_CACHE_INVALIDATE`标记无效化缓存线，来确保在下一次访问时从主存中取出数据。
