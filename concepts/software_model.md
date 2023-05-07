<!--
# Fuchsia's software model
 -->
# Fuchsia 的软件模型

<!--
Fuchsia is an operating system that does many things differently from
traditional operating systems. Before you attempt to develop for Fuchsia, it is
important to understand how the software model works in Fuchsia.
 -->
Fuchsia 是一个与传统操作系统有诸多不同的操作系统。在您尝试为 Fuchsia 开发之前，了解工作在 Fuchsia 中的软件模型是很重要的。

<!--
In Fuchsia, almost everything is a component and it is the unit of
executable software. The Component framework is what runs all components on
Fuchsia. For more information on components and the component framework, see
[Introduction to the Fuchsia component framework](/concepts/components/v2/introduction.md).
 -->
在 Fuchsia 中，几乎所有内容都是组件（component），它是可执行软件的单位。在 Fuchsia 上运行的所有组件都运行在组件框架（Component framework）上。要获取关于组件和组件框架的更多信息，请参阅 [Fuchsia 组件框架简介](/concepts/components/v2/introduction.md)。

<!--
An API dependency allows different components to define a dependency on another
component. These API dependencies are abstract which means that the implementation
of the interface is defined by another component and resolved by the component
framework. Components communicate with their dependencies through FIDL, which is
the language that Fuchsia uses to describe interprocess communication (IPC)
protocols of components that run on Fuchsia. For more information on FIDL, see
[FIDL overview](/concepts/fidl/overview.md).
 -->
API 依赖项允许不同组件定义对另一组件的依赖项。这些 API 依赖项是抽象的，这意味着接口的实现是由另一个组件定义的，并由组件框架解析。组件通过 FIDL 与其依赖项进行通信，其中 FIDL 是 Fuchsia 用来描述在其上运行的组件的进程内通信（interprocess communication，IPC）协议的语言。要获取关于 FIDL 的更多信息，请参阅 [FIDL 概述](/concepts/components/v2/introduction.md)。

<!--
## Distributing components through packages
 -->
## 通过包分发组件

<!--
In Fuchsia, components and their dependent files and images are often
distributed through packages which is the unit of distribution in Fuchsia.
 -->
在 Fuchsia 中，组件及其依赖的文件和图像通常是通过包（package）来分发的，它是 Fuchsia 的分发单位。

<!--
Note: For more information on packages, see [Fuchsia packages](/concepts/packages/package.md)
 -->
注意：要获取关于包的更多信息，请参阅 [Fuchsia 包](/concepts/packages/package.md)

<!--
Fuchsia resolves packaged dependencies at install time, creating an ABI
dependency. References to resources from an _external_ package are resolved at
runtime, creating an API dependency, but not an ABI dependency. (Runtime
resolution is similar to a web services architectural model.)
 -->
Fuchsia 在安装时解析打包的依赖项，从而创建 ABI 依赖项。来自外部（_external_）包的对资源的引用在运行时解析，创建 API 依赖项，而非 ABI 依赖项。（运行时解析类似于 Web 服务架构模型。）

<!--
Components are organized to keep critical dependencies in a package; and this
extends to [subpackages](/concepts/components/v2/subpackaging.md) which are
bound to their containing package at build time, allowing ABI dependencies to be
resolved statically.
 -->
组件被组织起来，以将关键依赖项保留在包中；这扩展出了[子包](/concepts/components/v2/subpackaging.md)（subpackage）的概念，它们在构建时绑定到其包含的包，从而允许静态解析 ABI 依赖项。

<!--
Note: Executable components (programs) implement API dependencies through
[FIDL](/concepts/fidl/overview.md)
 -->
注意：可执行组件（程序）通过 [FIDL](/concepts/fidl/overview.md) 实现 API 依赖项

<!--
A logical way to package components that launch other components is to use
subpackages to mirror the component parent-child relationship hierarchy such
that, if a component declares a child component, the child is loaded from a
declared subpackage of the parent component's package. This encapsulates the ABI
dependencies and ensures the presence of the expected ABI version of the child
component. Components model API dependencies through capability routing
(services exposed, routed, and used, by capability name, such as a FIDL
protocol). Package dependencies are less relevant to capability routing, except
that a parent component can orchestrate the creation of independently-packaged
peer components (subpackaged or not) and declare the capability connections
between them.
 -->
打包启动其他组件的组件的一种合乎逻辑的方法，是使用子包来反映组件的父子关系层次结构，这样，如果组件声明了子组件，那么该子组件会从父组件包的已声明子包中加载。这就封装了 ABI 依赖项，并确保了子组件预期 ABI 版本的存在。组件通过能力路由（capability routing）（通过能力名称公开、路由和使用的服务，例如 FIDL 协议）对 API 依赖项进行建模。包依赖项与能力路由不太相关，除了父组件可以协调独立打包的对等（peer）组件（子包与否）的创建，并声明其间的能力连接。

