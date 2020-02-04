<!-- # Glossary -->
# 术语表

<!-- ## Overview -->
## 概述

<!--
The purpose of this document is to provide short definitions for a collection of technical terms
used in the Fuchsia source tree.
-->
这篇文档旨在对 Fuchsia 源代码树中的术语集提供简明定义。

<!-- #### Adding new definitions -->
#### 添加新定义

<!--
- A definition should be limited to two or three sentences and deliver a high-level description of a
term.
- When another non-trivial technical term needs to be employed as part of the description,
consider adding a definition for that term and linking to it from the original definition.
- A definition should be complemented by a list of links to more detailed documentation and related
topics.
-->
- 每条定义的长度应限制在两到三句话之内。定义应对数据进行高层次描述。
- 当另一个特别术语被用作描述的一部分时，请考虑对新术语添加描述并于原描述创建指向此处的链接。
- 每条定义应有一系列的补充链接，提供更详细的文档和相关话题。

<!-- ## Terms -->
## 术语

#### **Agent**
<!-- FT -->

<!--
A component whose life cycle is not tied to any story, is a singleton in user scope, and provides
services to other components. An agent can be invoked by other components or by the system in
response to triggers like push notifications. An agent can provide services to components, send and
receive messages, and make proposals to give suggestions to the user.
-->
一个生命周期与任何 story 无关、在用户空间中单独存在、为其他组件提供服务的组件。Agent 可被其他组件或系统作为对触发器回应调用，如通知推送。Agent 为组件提供服务，发送和接收信息，并产生对用户的建议。

#### **应用管理器** **AppMgr**
<!-- FA -->

<!--
The Application Manager (AppMgr) is responsible for launching components and managing the namespaces
in which those components run. It is the first process started in the `fuchsia` job by the [DevMgr]
(#DevMgr).
-->
应用管理器（AppMgr）负责启动组件和管理这些组件运行的命名空间。它是由 [DevMgr]（#DevMgr）在 `fuchsia` job 中启动的第一个进程。

#### **Armadillo**
<!-- FN  -->

<!--
An implementation of a session shell.
- [Source](https://fuchsia.googlesource.com/topaz/+/master/shell/armadillo/)
-->
一个会话 shell 的实现。
- [源代码](https://fuchsia.googlesource.com/topaz/+/master/shell/armadillo/)

#### **Base shell**
<!-- OT -->

<!--
The platform-guaranteed set of software functionality which provides a basic user-facing interface
for boot, first-use, authentication, escape from and selection of session shells, and device recovery.
-->
平台保证的一套软件功能。提供引导时的用户界面、初次使用、认证、会话 shell 的退出和选择和设备恢复服务。

<!-- #### **Component** -->
#### **组件** **Component**
<!-- OT -->

<!--
A component is a unit of execution and accounting. It consists of a manifest file and associated
code, which comes from a Fuchsia package. A component runs in a sandbox, accesses objects via its
[namespace](#Namespace) and publishes objects via its export directory. [Modules](#Module) and
[Agents](#Agent) are examples of components. Components are most commonly distributed inside
[Fuchsia Packages](#fuchsia-package).
-->
组件是一个执行和管理单元。由来自一个 Fuchsia 包的一个清单文件和相关的代码组成。组件运行在沙箱中，通过其命名空间 [namespace](#Namespace) 访问对象，并通过其导出目录放出对象。例如，[Module](#Module) 和 [Agent](#Agent) 都是组件。组件通常在 [Fuchsia 包](#fuchsia-package) 中被分发。

<!-- #### **Component manifest** -->
#### **组件清单** **Component Manifest**
<!-- OT -->

<!--
A component manifest (.cmx) is a JSON file with the file extension `.cmx`, typically located in the
package’s `meta/` directory with information that declares how to run the component and what
capabilities it receives upon launch. In particular, the component manifest describes how the
component is sandboxed. See [Component manifest](the-book/package_metadata.md#Component-manifest)
for a detailed description.

These files end in `.cmx`, so they are also known as "cmx files".
-->
组件清单（.cmx）是文件扩展名为 `.cmx` 的 JSON 文件。通常位于所属包的 `meta/` 目录。包含关于该组件如何运行和启动时获得哪些能力的信息，特别是该组件被沙箱隔离的过程。详细描述请参见 [组件清单](the-book/package_metadata.md#Component-manifest)。

#### **Channel**
<!-- FT -->

A Channel is the fundamental IPC primitive provided by Zircon.  It is a bidirectional, datagram-like
transport that can transfer small messages including [Handles](#Handle).
- [Channel Overview](https://fuchsia.googlesource.com/zircon/+/master/docs/objects/channel.md)

#### **DevHost**
<!-- FA -->

<!--
A Device Host (DevHost) is a process containing one or more device drivers.  They are created by the
Device Manager, as needed, to provide isolation between drivers for stability and security.
-->
设备主机（DevHost）是包含一个或更多设备驱动的进程。由设备管理器按需创建，从而通过提供驱动间的隔离带来稳定性和安全性。

#### **DevMgr**
<!-- FA -->

<!--
The Device Manager (DevMgr) is responsible for enumerating, loading, and managing the life cycle of
device drivers, as well as low level system tasks (providing filesystem servers for the boot
filesystem, launching [AppMgr](#AppMgr), and so on).
-->
设备管理器（DevMgr）负责枚举、加载和管理设备驱动和低级系统任务（为引导文件系统提供文件系统服务器、启动 [AppMgr](#AppMgr) 等等）的生命周期。

#### **DDK**
<!-- FA -->

<!--
The Driver Development Kit is the documentation, APIs, and ABIs necessary to build Zircon Device
Drivers.  Device drivers are implemented as ELF shared libraries loaded by Zircon's Device Manager.
- [DDK Overview](https://fuchsia.googlesource.com/zircon/+/master/docs/ddk/overview.md)
- [DDK includes](https://fuchsia.googlesource.com/zircon/+/master/system/ulib/ddk/include/ddk/)
-->
驱动开发套件是编译 Zircon 设备驱动所需的文档、API 和 ABI。设备驱动以 ELF 共享库的形式实现，由 Zircon 的设备管理器加载。

<!-- #### **Driver** -->
<!-- OT -->

<!--
A driver is a dynamic shared library which [DevMgr](#DevMgr) can load into a [DevHost](#DevHost)
and that enables, and controls one or more devices.
- [Reference](https://fuchsia.googlesource.com/zircon/+/master/docs/ddk/driver-development.md)
- [Driver Sources](https://fuchsia.googlesource.com/zircon/+/master/system/dev)
-->
驱动是可由 [DevMgr](#DevMgr) 载入到 [DevHost](#DevHost) 的动态共享库，使一个或更多设备可用并将其控制。
- [参考](https://fuchsia.googlesource.com/zircon/+/master/docs/ddk/driver-development.md)
- [驱动源代码](https://fuchsia.googlesource.com/zircon/+/master/system/dev)

<!-- #### **Environment** -->
#### 环境
<!-- OT -->

<!--
A container for a set of components, which provides a way to manage their lifecycle and provision
services for them. All components in an environment receive access to (a subset of) the
environment's services.
-->
一系列组件的容器，提供管理这些组件生命周期的方式并为它们提供服务。所有环境中的组件都能访问所属环境的服务（的子集）。

#### **Escher**
<!-- FF -->

<!--
Graphics library for compositing user interface content. Its design is inspired by modern real-time
and physically based rendering techniques though we anticipate most of the content it renders to
have non-realistic or stylized qualities suitable for user interfaces.
-->
绘制用户界面内容的图形库。其设计灵感来源于现代的实时物理渲染方法，尽管它渲染的大多数内容预计会有适合用户界面的非现实或风格化特征。

#### **FAR**

The Fuchsia Archive Format is a container for files to be used by Zircon and Fuchsia.
- [FAR Spec](the-book/archive_format.md)

#### **fdio**

fdio is the Zircon IO Library.  It provides the implementation of posix-style open(), close(),
read(), write(), select(), poll(), etc, against the RemoteIO RPC protocol.  These APIs are return-
not-supported stubs in libc, and linking against libfdio overrides these stubs with functional
implementations.
- [Source](https://fuchsia.googlesource.com/zircon/+/master/system/ulib/fdio/)

#### **FIDL**

The Fuchsia Interface Definition Language (FIDL) is a language for defining protocols for use over
[Channels](#channel). FIDL is programming language agnostic and has bindings for many popular
languages, including C, C++, Dart, Go, and Rust. This approach lets system components written in a
variety of languages interact seamlessly.

#### **Flutter**

[Flutter](https://flutter.io/) is a functional-reactive user interface framework optimized for
Fuchsia and is used by many system components. Flutter also runs on a variety of other platform,
including Android and iOS. Fuchsia itself does not require you to use any particular language or
user interface framework.

#### **Fuchsia Package**

A Fuchsia Package is a unit of software distribution. It is a collection of files, such as:
manifests, metadata, zero or more executables (e.g. [Components](#component)), and assets.

#### **FVM**

Fuchsia Volume Manager is a partition manager providing dynamically allocated groups of blocks known
as slices into a virtual block address space. The FVM partitions provide a block interface enabling
filesystems to interact with it in a manner largely consistent with a regular block device. -
[Filesystems](the-book/filesystems.md)

#### **Garnet**

Garnet is one of the four layers of the Fuchsia codebase.
- [The Fuchsia layer cake](development/source_code/layers.md)
- [Source](https://fuchsia.googlesource.com/garnet/+/master)

#### **GN**

GN is a meta-build system which generates build files so that Fuchsia can be built with [Ninja]
(#ninja). GN is fast and comes with solid tools to manage and explore dependencies. GN files, named
`BUILD.gn`, are located all over the repository.
- [Language and operation](https://gn.googlesource.com/gn/+/master/docs/language.md)
- [Reference](https://gn.googlesource.com/gn/+/master/docs/reference.md)
- [Fuchsia build overview](development/build/overview.md)

#### **Handle**

A Handle is how a userspace process refers to a [kernel object](#Kernel-Object). They can be passed
to other processes over [Channel](#Channel)s.
- [Reference](https://fuchsia.googlesource.com/zircon/+/HEAD/docs/handles.md)

#### **Hub**

The hub is a portal for introspection.  It enables tools to access detailed structural information
about realms and component instances at runtime, such as their names, job and process ids, and
published services.
- [Hub](the-book/hub.md)

#### **Jiri**

Jiri is a tool for multi-repo development. It is used to checkout the Fuchsia codebase. It supports
various subcommands which makes it easy for developers to manage their local checkouts.
- [Reference](https://fuchsia.googlesource.com/jiri/+/master/README.md)
- [Sub commands](https://fuchsia.googlesource.com/jiri/+/master/README.md#main-commands-are)
- [Behaviour](https://fuchsia.googlesource.com/jiri/+/master/behaviour.md)
- [Tips and tricks](https://fuchsia.googlesource.com/jiri/+/master/howdoi.md)

#### **Job**

A Job is a [kernel object](#Kernel-Object) that groups a set of related processes, their child
processes and their jobs (if any). Every process in the system belongs to a job and all jobs form
a single rooted tree.
- [Job Overview](https://fuchsia.googlesource.com/zircon/+/master/docs/objects/job.md)

#### **Kernel Object**

A kernel object is a kernel data structure which is used to regulate access to system resources
such as memory, i/o, processor time and access to other processes. Userspace can only reference
kernel objects via [Handles](#Handle).
- [Reference](https://fuchsia.googlesource.com/zircon/+/HEAD/docs/objects.md)

#### **Ledger**

[Ledger](https://fuchsia.googlesource.com/peridot/+/master/docs/ledger/README.md) is a distributed
storage system for Fuchsia. Applications use Ledger either directly or through state synchronization
primitives exposed by the Modular framework that are based on Ledger under-the-hood.

#### **LK**

Little Kernel (LK) is the embedded kernel that formed the core of the Zircon Kernel. LK is more
microcontroller-centric and lacks support for MMUs, userspace, system calls -- features that Zircon
added.
- [LK on Github](https://github.com/littlekernel/lk)

#### **Maxwell**

Services to expose ambient and task-related context, suggestions and infrastructure for leveraging
machine intelligence.

#### **Module**

A component with a `module` metadata file which primarily describes the Module's data compatibility
and semantic role.

Modules show UI and participate in Stories at runtime.
- [module metadata format](https://fuchsia.googlesource.com/peridot/+/HEAD/docs/modular/manifests/module.md)

#### **Mozart**

The view subsystem. Includes views, input, compositor, and GPU service.

#### **Musl**

Fuchsia's standard C library (libc) is based on Musl Libc.
- [Source](https://fuchsia.googlesource.com/zircon/+/master/third_party/ulib/musl/)
- [Musl Homepage](https://www.musl-libc.org/)

#### **Namespace**

A namespace is the composite hierarchy of files, directories, sockets, [service](#Service)s, and
other named objects which are offered to components by their [environment](#Environment).
- [Fuchsia Namespace Spec](the-book/namespaces.md)

#### **Netstack**

TODO(tkilbourn): add definition.

#### **Ninja**

Ninja is the build system executing Fuchsia builds. It is a small build system with a strong
emphasis on speed. Unlike other systems, Ninja files are not supposed to be manually written but
should be generated by other systems, such as [GN](#gn) in Fuchsia.
- [Manual](https://ninja-build.org/manual.html)
- [Ninja rules in GN](https://gn.googlesource.com/gn/+/master/docs/reference.md#ninja_rules)
- [Fuchsia build overview](development/build/overview.md)

#### **Package**

Package is an overloaded term. Package may refer to a [Fuchsia Package](#fuchsia-package) or a [GN
build package](#GN).

#### **Paver**

A tool in Zircon that installs partition images to internal storage of a device.
- [Guide for installing Fuchsia with paver](/development/workflows/paving.md).

#### **Peridot**

Peridot is one of the four layers of the Fuchsia codebase.
- [The Fuchsia layer cake](development/source_code/layers.md)
- [Source](https://fuchsia.googlesource.com/peridot/+/master)

#### **Realm**

Synonym for [environment](#environment).

#### **RemoteIO**

RemoteIO is the Zircon RPC protocol used between fdio (open/close/read/write/ioctl) and filesystems,
device drivers, etc.  As part of [FIDL](#FIDL) v2, it will become a set of FIDL Interfaces (File,
Directory, Device, ...) allowing easier interoperability, and more flexible asynchronous IO for
clients or servers.

#### **Service**

A service is an implementation of a [FIDL](#FIDL) interface. Components can offer their creator a
set of services, which the creator can either use directly or offer to other components.

Services can also be obtained by interface name from a [Namespace](#namespace), which lets the
component that created the namespace pick the implementation of the interface. Long-running
services, such as [Mozart](#mozart), are typically obtained through a [Namespace](#namespace),
which lets many clients connect to a common implementation.

#### **Story**

A user-facing logical container encapsulating human activity, satisfied by one or more related
modules. Stories allow users to organize activities in ways they find natural, without developers
having to imagine all those ways ahead of time.

#### **Story Shell**

The system responsible for the visual presentation of a story. Includes the presenter component,
plus structure and state information read from each story.

#### **Topaz**

Topaz is one of the four layers of the Fuchsia codebase.
- [The Fuchsia layer cake](development/source_code/layers.md)
- [Source](https://fuchsia.googlesource.com/topaz/+/master)

#### **Session Shell**

The replaceable set of software functionality that works in conjunction with devices to create an environment in which people can interact with mods, agents and suggestions.

#### **VDSO**

The VDSO is a Virtual Shared Library -- it is provided by the [Zircon](#Zircon) kernel and does not
appear in the filesystem or a package.  It provides the Zircon System Call API/ABI to userspace
processes in the form of an ELF library that's "always there." In the Fuchsia SDK and [Zircon DDK]
(#DDK) it exists as `libzircon.so` for the purpose of having something to pass to the linker
representing the VDSO.

#### **VMAR**

A Virtual Memory Address Range is a Zircon [kernel object](#Kernel-Object) that controls where and
how VMOs may be mapped into the address space of a process.
- [VMAR Overview](https://fuchsia.googlesource.com/zircon/+/master/docs/objects/vm_address_region.md)

#### **VMO**

A Virtual Memory Object is a Zircon [kernel object](#Kernel-Object) that represents a collection of
pages (or the potential for pages) which may be read, written, mapped into the address space of a
process, or shared with another process by passing a [Handle](#Handle) over a [Channel](#Channel).
- [VMO Overview](https://fuchsia.googlesource.com/zircon/+/master/docs/objects/vm_object.md)

#### **Zedboot** ####

Zedboot is a recovery image that is used to install and boot a full Fuchsia system. Zedboot is
actually an instance of the Zircon kernel with a minimal set of drivers and services running used to
bootstrap a complete Fuchsia system on a target device. Upon startup, Zedboot listens on the network
for instructions from a bootserver which may instruct Zedboot to [install](#paver) a new OS. Upon
completing the installation Zedboot will reboot into the newly installed system.

#### **Zircon**

Zircon is the [microkernel](https://en.wikipedia.org/wiki/Microkernel) and lowest level userspace
components (driver runtime environment, core drivers, libc, etc) at the core of Fuchsia.  In a
traditional monolithic kernel, many of the userspace components of Zircon would be part of the
kernel itself. Zircon is also one of the four layers of the Fuchsia codebase.
- [Zircon Documentation](https://fuchsia.googlesource.com/zircon/+/master/README.md)
- [Zircon Concepts](https://fuchsia.googlesource.com/zircon/+/master/docs/concepts.md)
- [The Fuchsia layer cake](development/source_code/layers.md)
- [Source](https://fuchsia.googlesource.com/zircon/+/master)

#### **ZX**

ZX is an abbreviation of "Zircon" used in Zircon C APIs/ABIs (`zx_channel_create()`, `zx_handle_t`,
`ZX_EVENT_SIGNALED`, etc) and libraries (libzx in particular).

#### **ZXDB**

The native low-level system debugger.
- [Reference](https://fuchsia.googlesource.com/garnet/+/master/docs/debugger.md)
