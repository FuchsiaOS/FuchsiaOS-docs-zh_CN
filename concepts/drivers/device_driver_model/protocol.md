<!---

# Protocols in drivers

## What is a protocol?

A protocol is a strict interface definition.

The ethernet driver published an interface that conforms to `ZX_PROTOCOL_ETHERNET_IMPL`.
This means that it must provide a set of functions defined in a data structure
(in this case, `ethernet_impl_protocol_ops_t`).

These functions are common to all devices implementing the protocol &mdash; for example,
all ethernet devices must provide a function that queries the MAC address of the
interface.

Other protocols will of course have different requirements for the functions they
must provide.
For example a block device will publish an interface that conforms to the
"block implementation protocol" (`ZX_PROTOCOL_BLOCK_IMPL`) and
provide functions defined by `block_protocol_ops_t`.
This protocol includes a function that returns the size of the device in blocks,
for example.

In many cases a Protocol is used to allow drivers to be simpler by taking advantage
of a common implementation of an Interface. For example, the "block" driver implements
the common block interface, and binds to devices implementing the Block Core Protocol,
and the "ethernet" driver does the same thing for the Ethernet Interface and Ethermac
Protocol. Some protocols, such as the two cited here, make use of shared memory, and
non-rpc signaling for more efficient, lower latency, and higher throughput than could
be achieved otherwise.

Classes represent a promise that a device implements an Interface or Protocol.
Devices exist in the Device Filesystem under a topological path, like
`/sys/pci/00:02:00/e1000`. If they are a specific class, they also appear
as an alias under `/dev/class/CLASSNAME/...`. The `e1000` driver implements
the Ethermac interface, so it also shows up at `/dev/class/ethermac/000`. The names
within class directories are unique but not meaningful, and are assigned on demand.

Note: Currently names in class directories are 3 digit decimal numbers, but they
are likely to change form in the future. Clients should not assume there is any
specific meaning to a class alias name.

Example protocols:

*   the PCI root protocol (`ZX_PROTOCOL_PCIROOT`),
*   the PCI device protocol (`ZX_PROTOCOL_PCI`), and
*   the ethernet implementation protocol (`ZX_PROTOCOL_ETHERNET_IMPL`).

The names in brackets are the C language constants corresponding to the protocols, for reference.

--->

# 驱动协议

## 什么是协议？

协议是一个严格的接口定义。

网络驱动遵照`ZX_PROTOCOL_ETHERNET_IMPL`发布一个接口。这意味着它必须提供定义在数据结构中的一组功能。（在这种使用场景中，数据结构为`ethernet_impl_protocol_ops_t`）。

这些功能对于所有实现协议的设备都是通用的—例如所有网络设备必须提供一个接口查询 MAC 地址的功能。

当然其他协议有不同需求对它们必须提供的功能。

例如一个块设备将发布遵照“块实现协议”（`ZX_PROTOCOL_BLOCK_IMPL`）的接口和提供`block_protocol_ops_t`定义的功能。例如这个协议包括一个功能为返回块中的设备大小。

在许多情况下，协议被用来通过利用接口的共同实现使驱动程序来变得更简单。例如“块”驱动实现共有块接口，然后绑定实现块核心协议的设备，同样“以太网”驱动对于以太网接口和 Ethermac 协议完成同样的操作。例如在此处引用的两个协议，这些协议比起其他方式实现，通过使用共享内存和非 rpc 信号来获得更高效率，更低延迟和更高吞吐量。

类代表了一个设备实现接口或协议的约定。设备存在在设备文件系统中的拓扑路径，例如`/sys/pci/00:02:00/e1000`。如果它们是一个特定类，它们同样作为别名出现在`/dev/class/CLASSNAME/...`下。其中`e1000` 驱动实现 Ethermac 接口，所以它同样出现在`/dev/class/ethermac/000`。包含类目录的名字是唯一但没有意义的，并且按需求分配。

注意：当前在类目录内的名字是3个十进制的数字，但是它们可能会在以后改变格式。客户端不应该假设一个类别名包含任何特定意义。

示例协议：

*  PCI 根协议 (`ZX_PROTOCOL_PCIROOT`)，
*  PCI 设备协议 (`ZX_PROTOCOL_PCI`)，
* 以太网实现协议（`ZX_PROTOCOL_ETHERNET_IMPL`）。

括号内的名字是对应协议的C语言常量，以供参考。

<!---

## Platform dependent vs platform independent

Above, we mentioned that `ZX_PROTOCOL_ETHERNET_IMPL` was "close to" the functions used
by the client, but one step removed.
That's because there's one more protocol, `ZX_PROTOCOL_ETHERNET`, that sits between
the client and the driver.
This additional protocol is in place to handle functionality common to all ethernet
drivers (in order to avoid code duplication).
Such functionality includes buffer management, status reporting, and administrative
functions.

This is effectively a "platform dependent" vs "platform independent" decoupling;
common code exists in the platform independent part (once), and driver-specific code
is implemented in the platform dependent part.

This architecture is repeated in multiple places.
With block devices, for example, the hardware driver binds to the bus (e.g., PCI)
and provides a `ZX_PROTOCOL_BLOCK_IMPL` protocol.
The platform independent driver binds to `ZX_PROTOCOL_BLOCK_IMPL`, and publishes the
client-facing protocol, `ZX_PROTOCOL_BLOCK`.

You'll also see this with the display controllers, I<sup>2</sup>C bus, and serial drivers.

--->

## 平台依赖 vs 平台独立 

上述章节中，我们提及过`ZX_PROTOCOL_ETHERNET_IMPL`是“接近”客户端使用的函数，但又差一步。

这是因为还有一个`ZX_PROTOCOL_ETHERNET`的协议，它位于客户端和驱动程序之间。

这个附加协议的存在是为了处理所有以太网驱动的共同函数（为了避免代码重复）。

这样的函数中包含缓存管理，状态上报和管理功能。

这实际上是一种“平台依赖“和”平台独立“的解耦；通用代码存在在平台独立部分（曾经），驱动独有代码实现在平台依赖部分。

这种结构在很多地方是重复的。例如在块设备中，硬件驱动绑定总线（例如， PCI 总线）并提供`ZX_PROTOCOL_BLOCK_IMPL`协议。

平台独立驱动绑定`ZX_PROTOCOL_BLOCK_IMPL`，并且发布面向客户端的协议 `ZX_PROTOCOL_BLOCK`。

你同样可以在显示控制器，  I<sup>2</sup>C 总线和串口驱动中看到它。

<!--- More content? -->

<!---

## Process / protocol mapping

In order to keep the discussions above simple, we didn't talk about process separation
as it relates to the drivers.
To understand the issues, let's see how other operating systems deal with them,
and compare that to the Fuchsia approach.

In a monolithic kernel, such as Linux, many drivers are implemented within the kernel.
This means that they share the same address space, and effectively live in the same
"process."

The major problem with this approach is fault isolation / exploitation.
A bad driver can take out the entire kernel, because it lives in the same address
space and thus has privileged access to all kernel memory and resources.
A compromised driver can present a security threat for the same reason.

--->

## 进程/协议映射

为了保持上述讨论的简单，我们没有谈及与驱动有关的进程分离。

为了了解这个问题，让我们一起来了解其他操作系统是怎样处理它们的，然后和 Fuchsia 中的处理方式进行对比。

在例如Linux的宏内核中，很多驱动实现都是在内核中。这就意味着它们共享相同的地址空间和高效地存在在同一“进程”中。

这种方式的主要问题在于故障隔离/暴露。

一个异常驱动可能使整个内核失效，因为它存在在同一个地址空间内，所以拥有所有内核内存和资源的特权访问。

出于同样的原因，一个受损的驱动就会带来安全威胁。

<!---

The other extreme, that is, putting each and every driver service into its own
process, is used by some microkernel operating systems.
Its major drawback is that if one driver relies on the services of another driver,
the kernel must effect at least a context switch operation (if not a data transfer
as well) between the two driver processes.
While microkernel operating systems are usually designed to be fast at these
kinds of operations, performing them at high frequency is undesirable.

The approach taken by Fuchsia is based on the concept of a driver host.
A driver host is a process that contains a protocol stack &mdash; that is, one or
more protocols that work together.
The driver host loads drivers from ELF shared libraries (called Dynamic Shared Objects,
or **DSO**s).
In the [simple drivers](/docs/development/drivers/developer_guide/simple.md) section,
we'll see the meta information that's contained in the DSO to facilitate the discovery process.

The protocol stack effectively allows the creation of a complete "driver" for
a device, consisting of platform dependent and platform independent components,
in a self-contained process container.

For the advanced reader, take a look at the `dm dump` command available from
the Fuchsia command line.
It displays a tree of devices, and shows you the process ID, DSO name, and
other useful information.

Here's a highly-edited version showing just the PCI ethernet driver parts:

--->

而另一个极端，则是把每一个驱动服务放到它自己进程中，这已经被一些微内核操作系统所采用。

它主要的缺点在于如果一个驱动依赖另一个驱动的服务，那么内核必然在至少是两个驱动进程中需要切换上下文操作时受到影响（如果不是同样的数据转换）。

尽管微内核操作系统通常被设计为处理这些操作尽可能快，但是能以很高的频率切换是不太现实的。

Fuchsia 使用的方式则是基于驱动主机的概念。

一个驱动主机是一个包含协议栈的进程—那就意味着，一个或者更多协议可以共同工作。

驱动主机从 ELF 共享库中加载驱动（叫做动态共享对象，或者 **DSO**s）。

在 [simple drivers](/docs/development/drivers/developer_guide/simple.md) 章节中，我们将看到包含在 DSO 中操作发现进程的元信息。

协议栈允许高效地为一个设备创建一个完成的“驱动”，其中包含平台依赖和平台独立的组件在一个包含自身进程的容器中。

对于更资深的读者，可以查看 Fuchsia 命令行中的`dm dump`指令。

它可以展示设备树，并向你展示了进程 ID ， DSO 名字和其他有用信息。

下述为一个显示 PCI 以太网驱动部分的高度编辑版本：

```
1. [root]
2.    [sys]
3.       <sys> pid=1416 /boot/driver/bus-acpi.so
4.          [acpi] pid=1416 /boot/driver/bus-acpi.so
5.          [pci] pid=1416 /boot/driver/bus-acpi.so
            ...
6.             [00:02:00] pid=1416 /boot/driver/bus-pci.so
7.                <00:02:00> pid=2052 /boot/driver/bus-pci.proxy.so
8.                   [e1000] pid=2052 /boot/driver/e1000.so
9.                      [ethernet] pid=2052 /boot/driver/ethernet.so
```

<!---

From the above, you can see that process ID `1416` (lines 3 through 6)
is the Advanced Configuration and Power Interface (**ACPI**) driver, implemented
by the DSO `bus-acpi.so`.

During primary enumeration, the ACPI DSO detected a PCI bus.
This caused the publication of a parent with `ZX_PROTOCOL_PCI_ROOT` (line 5,
causing the appearance of the `[pci]` entry),
which then caused the driver host to load the `bus-pci.so` DSO and bind to it.
That DSO is the "base PCI driver" to which we've been referring throughout the
discussions above.

During its binding, the base PCI driver enumerated the PCI bus, and found an ethernet
card (line 6 detects bus 0, device 2, function 0, shown as `[00:02:00]`).
(Of course, many other devices were found as well, but we've removed them from
the above listing for simplicity).

The detection of this device then caused the base PCI driver to publish a new parent
with `ZX_PROTOCOL_PCI` and the device's VID and DID.
Additionally, a new driver host (process ID `2052`) was created and loaded with the
`bus-pci.proxy.so` DSO (line 7).
This proxy serves as the interface from the new driver host (pid `2052`) to the base PCI
driver (pid `1416`).

--->

从上表中，你可以看到进程ID `1416`（第3行到第6行）是一个由 DSO `bus-acpi.so`实现的高级配置和电源接口（**ACPI**）驱动。

在初级查验期间， ACPI DSO 发现一个 PCI 总线。这导致了有 `ZX_PROTOCOL_PCI_ROOT`的父节点的发布（第5行，因为 `[pci]` 入口的出现），这接下来引起驱动主机加载`bus-pci.so`   DSO 并绑定它。那个 DSO 就是我们上述讨论中参考的“基础 PCI 驱动”。

在它的绑定过程中，基础 PCI 驱动遍历了 PCI 总线，找到一个以太网卡（第6行中`[00:02:00]`表示总线0，设备2，功能0）。

（当然，许多其他设备也同样被找到，但是我们为了简便已经把它们从上述列表中移除）。

发现这个设备后触发基础 PCI 驱动发布一个新的`ZX_PROTOCOL_PCI` 父节点和设备 VID 与 DID 。

此外一个新的驱动主机（进程 ID 为`2052`）被创建然后使用`bus-pci.proxy.so`  DSO 加载（第7行）。

对于基础 PCI 驱动来说（进程`1416`），这个代理充当新驱动主机（进程`2052`）的接口。

<!---

> This is where the decision was made to "sever" the device driver into its own
> process &mdash; the new driver host and the base PCI driver now live in two
> different processes.

The new driver host `2052` then finds a matching child (the `e1000.so`
DSO on line 8; it's considered a match because it has `ZX_PROTOCOL_PCI` and the correct
VID and DID).
That DSO publishes a `ZX_PROTOCOL_ETHERNET_IMPL`, which binds to a matching
child (the `ethernet.so` DSO on line 9; it's considered a match because it has a
`ZX_PROTOCOL_ETHERNET_IMPL` protocol).

What's not shown by this chain is that the final DSO (`ethernet.so`) publishes
a `ZX_PROTOCOL_ETHERNET` &mdash; that's the piece that clients can use, so of
course there's no further "device" binding involved.

--->
> 这就是决定将设备驱动“分割”到自己进程中的地方—新的驱动主机和基础 PCI 驱动现在存在在两个不同的进程中。

新驱动主机`2052` 接下来找到匹配的子节点（第8行的 DSO  `e1000.so`；因为它拥有`ZX_PROTOCOL_PCI`和正确的 VID 和 DID ，所以被认为是其匹配项）。

 DSO 发布 `ZX_PROTOCOL_ETHERNET_IMPL`，它绑定在匹配的子节点上（第9行的 DSO  `ethernet.so` ；因为它有一个 `ZX_PROTOCOL_ETHERNET_IMPL` 所以被认为是匹配项）。

这条链路没有展示出来最终 DSO （`ethernet.so`）发布`ZX_PROTOCOL_ETHERNET`—这是一个客户端可以使用的部分，所以这里当然就没有更多“设备”来绑定。

