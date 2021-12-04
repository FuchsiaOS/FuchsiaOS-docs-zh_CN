<!-- 
# Secure
 -->
# 安全

<!-- 
Security and privacy are woven deeply into the architecture of Fuchsia.
The basic building blocks of Fuchsia, the kernel primitives,
are exposed to applications as object-capabilities.
This means that applications running on Fuchsia have no ambient authority:
applications can interact only with the objects
to which they have been granted access explicitly.
 -->
安全性和隐私性根植于 Fuchsia 的架构之中。Fuchsia 的基本构件——内核原语（kernel primitive）是以对象功能（object-capability）的形式暴露给应用的。这意味着，在 Fuchsia 上运行的应用不具备环境权限（ambient authority）：应用仅能与它们所显式授予访问权限的对象进行交互。

<!--
Software is delivered in hermetic packages and everything is sandboxed.
All software that runs on the system, including applications and system
components, receives the least privilege it needs to perform its job and
gains access only to the information it needs to know.
Because capabilities routing and software isolation are enforced by the
operating system, developers don’t have to build an additional
system for security.
 -->
软件以封闭包（hermetic package）的方式递交，一切都在沙盒之中。系统上运行的包括应用和系统组件在内的所有软件，会获取其完成工作所需的最低权限，并仅获得其所需要了解的信息。由于功能路由（capability routing）和软件隔离（software isolation）是由操作系统强制进行的，因此开发者并不需要为安全性而构建额外的系统。

<!-- 
## Fuchsia builds on a kernel designed to securely isolate software
 -->
## Fuchsia 构建在为了安全隔离软件而设计的内核之上

<!-- 
**[Zircon](/docs/concepts/kernel/README.md)
is a capability-based, object-oriented kernel**

The Zircon system fully isolates processes by default,
and must explicitly grant capabilities and resources.
Fuchsia passes capabilities and resources by handles rather than name,
which leads to a system that only grants software access to what it needs.
 -->
**[Zircon](/concepts/kernel/README.md) 是基于功能、面向对象的内核**

Zircon 系统默认完全隔离进程，并且必须显式地授予功能和资源。Fuchsia 通过句柄而非名称传递功能和资源，这就使得系统仅仅授予软件其所需的权限。

<!-- 
## Components are the fundamental unit of software execution
 -->
## 组件是软件执行的基本单元

<!-- 
**[Components](/docs/concepts/components/v2/introduction.md)
are isolated containers for Fuchsia software**

Nearly all user space software is a component,
from system services to end-user applications.
The component framework encourages the composition of loosely coupled software.
Capabilities used and exposed must be explicitly declared.
 -->
**[组件](/concepts/components/v2/introduction.md)（component）是针对 Fuchsia 软件的孤立容器**

从系统服务到终端用户应用，几乎所有用户空间软件都是组件。组件框架鼓励松散耦合软件（loosely coupled software）的组合。使用和暴露的功能必须显式声明。

<!-- 
## Software is delivered in self-contained packages
 -->
## 软件通过自包含包的形式递交

<!-- 
**[Packages](/docs/concepts/packages/package.md)
have everything they need to run every time**

Components are distributed through hermetic, or self-contained, packages
that include all needed files.
Fuchsia packages are a collection of components, files, and metadata.
Isolated namespaces mean a component only has visibility to its own package.
 -->
**[包](/concepts/packages/package.md)每次运行时都具备其需要的一切**

包通过封闭包或自包含包（self-contained package）的方式分发，它们包含了所需的全部文件。Fuchsia 包是组件、文件和元数据的集合。孤立的命名空间就是一个仅对其自己的包具有可视性的组件。

<!-- 
## Fuchsia has no global file system or ambient authority
 -->
## Fuchsia 不具备全局文件系统或环境权限

<!-- 
**[Namespaces](/docs/concepts/process/namespaces.md)
prevent programs from escaping their containers**

Fuchsia aims to have no ambient authority,
which means every operation is scoped to an object capability.
Similarly, Fuchsia has no global file system.
Instead, each program is given its own local namespace in which to operate.
 -->
**[命名空间](/concepts/process/namespaces.md)阻止程序逃脱容器**

Fuchsia 旨在取消环境权限，也就是说，每一步操作都细究至对象功能上。类似地，Fuchsia 没有全局文件系统。取而代之的是，每个程序都被赋予了它们自己的本地命名空间，以在其中进行操作。
