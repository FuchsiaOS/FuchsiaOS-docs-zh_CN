# Updatable
# 可更新

<!-- 
Fuchsia distributes software in packages,
which are hermetically sealed bundles of components, related files, and dependencies.
Fuchsia packages are designed to be updated independently or even delivered ephemerally,
which means they can come and go from the device as needed and the software is always up to date,
like a web page.
 -->
Fuchsia 以包（package）的形式分发软件，它们是组件、相关文件和依赖的封闭套装（bundle）。Fuchsia 包为独立升级甚至是短期递交而设计，这就意味着它们能够按需进出设备，并且软件始终是最新的，如同网页一样。

<!-- 
Fuchsia aims to provide drivers with a binary-stable interface.
In the future,
drivers compiled for one version of Fuchsia will continue to work
in future versions of Fuchsia without needing to be modified or even recompiled.
This approach means that Fuchsia devices will be able
to update to newer versions of Fuchsia seamlessly while keeping their existing drivers.
 -->
Fuchsia 旨在通过二进制稳定的接口提供驱动。在未来，为 Fuchsia 的某一版本编译的驱动将在 Fuchsia 的后续版本中继续运作，而无需修改，哪怕是重新编译。这一方案意味着 Fuchsia 设备将能够在保留它们已有驱动的情况下无缝升级至新版本 Fuchsia。

<!-- 
## Almost all software on Fuchsia is a component
 -->
## Fuchsia 上的几乎所有软件都是组件

<!-- 
**[The component framework](/docs/concepts/components/v2/introduction.md)
makes it easier to update the system as new software is created**

The kernel has a minimal set of responsibilities,
nearly everything else is in a user space component.
Components are identified by URLs and
can be resolved, downloaded, and executed on demand like the web.
They are governed by the same mechanisms and they all work together.
Hermetic packaging of components leads to more portable software.
 -->
**[组件框架](/concepts/components/v2/introduction.md)（component framework）使得当新软件创建时更新系统更加容易**

内核拥有最小化的职责，几乎其他所有事项都归于用户空间组件中。组件像网络一样通过 URL 辨识，能够进行解析、下载、按需执行。它们通过相同机制管理，并协同工作。组件的封闭包装使得软件更加便携。

<!-- 
## Software is interchangeable and reusable
 -->
## 软件是可交换和重复使用的

<!-- 
**[Fuchsia Interface Definition Language (FIDL)](/docs/concepts/fidl/overview.md)
enables loose coupling between components**

Components exchange capabilities as defined by FIDL protocols.
Software is composed at runtime through protocols
rather than through static composition of libraries.
Fuchsia has no system libraries.
Even the C standard library [(libc)](/docs/concepts/system/libc.md)
is expressed as a dependency,
delivered only when software needs it.
Components can be swapped with another implementation
as long they express the same FIDL protocol.
 -->
**[Fuchsia 接口定义语言（Fuchsia Interface Definition Language，FIDL）](/concepts/fidl/overview.md)启用了组件间的松散耦合（loose coupling）**

组件按 FIDL 协议的定义交换功能。软件在运行时通过协议组合，而不是通过库的静态组合。Fuchsia 没有系统库，即使是 C 标准库（[libc](/concepts/system/libc.md)）也是以依赖方式表示的，仅在软件需要的时候传递。组件可以与另一实现进行互换，只要它们表达了相同的 FIDL 协议。

<!-- 
## Push updates and security patches to all products on demand
 -->
## 按需向所有产品推送更新和安全性补丁

<!-- 
**[Fuchsia packages](/docs/concepts/packages/package.md)
are the units of software distribution**

All software is delivered in packages that
can be updated independently and delivered on demand, like the web.
This enables a vulnerability patch to be pushed to all Fuchsia products at once
without the need for individual product coordination.
 -->
**[Fuchsia 包](/concepts/packages/package.md) 是软件分发的单位**

一切软件都是以包的形式交付的，可以独立更新、按需传递，就像网络一样。这使得脆弱性布丁能够立刻推送至所有 Fuchsia 产品，无需单独的产品协调。

<!-- 
## On the roadmap
 -->
## 路线图所列

<!-- 
This section covers features on
[Fuchsia's roadmap](/docs/contribute/roadmap/index.md).
 -->
这一部分涵盖了 [Fuchsia 路线图](/contribute/roadmap/index.md)中的特性。

<!-- 
### Update the system without modifying the driver
 -->
### 更新系统而不修改驱动

<!-- 
**[Drivers](/docs/concepts/drivers/getting_started.md)
and system services are designed as user space components that
can be updated independently of the core OS**

We are designing the system so that Fuchsia products can receive system updates
without having to modify or recompile drivers.
Drivers, system services, and end-user applications would be updated
independently through the same mechanism, reducing the maintenance burden.
Device owners could receive Fuchsia updates without having to update
their drivers.
 -->
**[驱动](/concepts/drivers/getting_started.md)和系统服务设计为用户空间组件，能够独立于核心操作系统进行更新**

我们正在设计系统，以使得 Fuchsia 产品能够不必修改或重新编译驱动而接收系统更新。驱动、系统服务和终端用户应用将会通过相同机制独立更新，从而减少维护负担。设备所有者可以接收 Fuchsia 更新而不必更新驱动。
