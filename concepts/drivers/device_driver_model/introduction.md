<!---

# Overview

At the highest level, a device driver's job is to provide a uniform interface to
a particular device, while hiding details specific to the device's implementation.

Two different ethernet drivers, for example, both allow a client to send packets
out an interface, using the exact same C language function.
Each driver is responsible for managing its own hardware in a way that makes the
client interfaces identical, even though the hardware is different.

Note that the interfaces that are provided by the driver may be "intermediate" &mdash;
that is, they might not necessarily represent the "final" device in the chain.

Consider a PCI-based ethernet device.
First, a base PCI driver is required that understands how to talk to the PCI bus itself.
This driver doesn't know anything about ethernet, but it does know
how to deal with the specific PCI chipset present on the machine.

It enumerates the devices on that bus, collects information
from the various registers on each device, and provides functions that allow
its clients (such as the PCI-based ethernet driver) to perform PCI operations
like allocating an interrupt or a DMA channel.

Thus, this base PCI driver provides services to the ethernet driver, allowing
the ethernet driver to manage its associated hardware.

At the same time, other devices (such as a video card) could also use the base PCI
driver in a similar manner to manage their hardware.

--->

# 概述

在系统顶层中，设备驱动的工作就是针对特殊设备来提供一个通用接口，来隐藏设备实现的具体细节。

例如对于两个不同的网络驱动，都允许客户端使用完全相同的C语言函数来向接口发送数据包。

每一个驱动负责通过使客户端使用相同接口的方式来管理它自己的硬件，即使其中的硬件是完全不同的。

注意，驱动提供的接口可能是“中间的” — 这就表明，它们可能并不是代表在链式中的“最终”设备。

考虑基于 PCI 的网络设备。

首先，一个基础 PCI 驱动是必需的，它用来了解怎样和 PCI 总线自身通信。该驱动并不知道任何与网络相关的东西，但是它知道怎么处理在机器上的特定 PCI 芯片。

它枚举了总线上的设备，从每个设备上的多个寄存器中收集信息，然后提供函数来允许它的客户端（例如基于 PCI 的网络驱动）来处理例如分配中断或者 DMA 通道的 PCI 操作。

因此，这个基础 PCI 驱动向网络驱动提供服务，允许网络驱动来管理它的关联硬件。

与此同时，其他设备（例如视频卡）同样也可以通过类似的方式来使用基础 PCI 驱动。

<!---

# The Fuchsia model

In order to provide maximum flexibility, drivers in the Fuchsia world are allowed
to bind to matching "parent" devices, and publish "children" of their own.
This hierarchy extends as required: one driver might publish a child, only to have
another driver consider that child their parent, with the second driver publishing
its own children, and so on.

In order to understand how this works, let's follow the PCI-based ethernet example.

The system starts by providing a special "PCI root" parent.
Effectively, it's saying "I know that there's a PCI bus on this system, when you
find it, bind it *here*."

Drivers are evaluated by the system (a directory is searched), and drivers that
match are automatically bound.

In this case, a driver that binds to a "PCI root" parent is found, and bound.

--->

# Fuchsia 模型

为了提供最大程度的灵活性，在 Fuchsia 系统中的驱动被允许绑定在匹配的“父”设备中，然后发布在它们自己的“子”设备里。

该层级结构可以根据需要进行扩展：一个驱动想要发布它的子设备，而另一个驱动则认为这个子设备是自身的父设备，第二代驱动又发布自己的子设备，以此类推。

为了了解层级结构是如何工作的，让我们以基于 PCI 的网络驱动作为示例。

系统启动后提供一个特殊” PCI 根“父设备。实际上这就相当于表明“我知道有一个 PCI 总线在系统中，当你找到它时，请*在这里*绑定。“

驱动被系统鉴定（搜索目录），然后驱动将自动完成匹配绑定。

在这个使用场景中，找到一个与“ PCI 根”父类绑定的驱动程序，并进行了绑定。

<!---

This is the base PCI driver.
It's job is to configure the PCI bus, and enumerate the peripherals on the bus.

The PCI bus has specific conventions for how peripherals are identified:
a combination of a Vendor ID (**VID**) and Device ID (**DID**) uniquely identifies
all possible PCI devices.During enumeration, these values are read from the peripheral, and new parent
nodes are published containing the detected VID and DID (and a host of other
information).

Every time a new device is published, the same process as described above (for
the initial PCI root device publication) repeats;
that is, drivers are evaluated by the system, searching for drivers that match
up with the new parents' characteristics.

Whereas with the PCI root device we were searching for a driver that matched
a certain kind of functionality (called a "protocol," we'll see this shortly), in
this case, however, we're searching for drivers that match a different
protocol, namely one that satisfies the requirements of "is a PCI device and
has a given VID and DID."

If a suitable driver is found (one that matches the required protocol, VID and
DID), it's bound to the parent.

As part of binding, we initialize the driver &mdash; this involves such operations
as setting up the card for operation, bringing up the interface(s), and
publishing a child or children of this device.
In the case of the PCI ethernet driver, it publishes the "ethernet" interface,
which conforms to yet another protocol, called the "ethernet implementation" protocol.
This protocol represents a common protocol that's close to the functions that
clients use (but is one step removed; we'll come back to this).

--->

这就是基础 PCI 驱动。

它的工作就是配置 PCI 总线，并枚举在总线上的周边件。

 PCI 总线对于周边件如何识别有一个特定的约定：一个供应商 ID（**VID**)和一个设备 ID（**DID**）的组合，可以独特地识别所有可能的 PCI 设备。当列举设备时，这些值从周边件中读出，新的发布父节点就包含识别的 VID 和 DID （和一些其他的主机信息）。

每一次当一个新的设备被发布时，将重复上述同样的过程（针对初始化 PCI 根设备的发布）；

这就是驱动被系统鉴定后，搜索匹配新父设备特征的驱动。

而对于 PCI 根设备，我们正在搜索一种符合某种功能的驱动（被叫做“协议”，我们很快就会了解“协议”相关内容），在这种使用场景中，尽管我们搜索的驱动匹配不同的协议，也就是一个满足“是一个 PCI 设备并且提供 VID 和 DID ”的需求。

如果找到合适的驱动（一个满足所需协议，包含 VID 和 DID ），它将被绑定在父设备上。

作为绑定的一部分，我们初始化驱动—这包含以下操作，设置卡的操作，启动接口和发布一个或多个该设备的子设备。

在 PCI 网络驱动的示例中，发布“ethernet”接口，用来遵从另一个叫做“ ethernet implementation ”的协议。这个协议代表了一个通用的协议适用客户端使用的功能。（但是移除了一个步骤，我们后续再讨论这个问题）。

