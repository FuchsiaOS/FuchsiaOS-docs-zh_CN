<!--
# Introduction to Fuchsia components
 -->
# Fuchsia 组件简介

<!--
## Overview
 -->
## 概述

<!--
This document offers a brief conceptual overview of Components and the
Component Framework.
 -->
本文档对组件和组件框架进行了简要的概念性叙述。

<!--
In Fuchsia, _[component][glossary.component]_ is the term for the common
abstraction that defines how all software[^1] (regardless of source,
programming language, or runtime) is described, sandboxed, and executed on a
Fuchsia system.
 -->
在 Fuchsia 中，“[组件][glossary.component]”是一个通用的抽象术语，它定义了一切软件[^1]（无论是什么源、编程语言或运行时）在 Fuchsia 系统中是如何被描述、沙盒化以及执行的。

<!--
[^1]: With the exception of early-boot software necessary to run components.
 -->
[^1]: 运行组件所需的早期引导软件除外。

<!--
### What is sandboxing?
 -->
### 什么是沙盒？

<!--
Sandboxing is a security mechanism to isolate programs from each other at
runtime. In Fuchsia, all software is sandboxed. When a program is initially
created, it does not have the ability to do anything -- not even to allocate
memory. The program relies on its creator to provide the capabilities needed
for it to execute. This isolation property allows Fuchsia to employ the
_principle of least privilege_: programs are provided only the minimal set
of capabilities needed to execute.
 -->
沙盒（sandboxing）是一种安全机制，可以在运行时将程序彼此隔离。在 Fuchsia 中，所有软件都是沙盒化的。程序在最初创建时，没有能力做任何事情——甚至无法分配内存。该程序依靠其创建者提供执行所需的能力（capability）。该隔离属性允许 Fuchsia 利用“最小特权原理”（_principle of least privilege_）：仅为程序提供执行所需的最小能力集。

<!--
## Component Framework
 -->
## 组件框架

<!--
The Component Framework (CF) consists of the core concepts, tools, APIs,
runtime, and libraries necessary to describe and run components and to
coordinate communication and access to resources between components.
 -->
组件框架（Component Framework，CF）由核心概念、工具、API、运行时和所需的库组成，用来描述和运行组件，以及协调组件之间的通信和资源访问。

<!--
The Component Framework includes:
 -->
组件框架包括：

<!--
-   CF concepts, including _component_, _component manifest_,
    _runner_, _realm_, _environment_, _capabilities_, and _resolver_.
-   The [`component_manager`][doc-component-manager] process, which coordinates
    the communication and sharing of resources between components.
-   [FIDL APIs](#fidl-apis) implemented by `component_manager`, or implemented
    by other components and used by `component_manager`, for the purposes of
    coordination.
-   Developer tools to build, execute, and test components.
-   Language-specific libraries for components to use to
    interact with the system. ([example](/sdk/lib/sys))
-   Testing tools and libraries to write unit and integration tests that
    exercise one or many components.
    ([example][doc-realm-builder])
 -->
-   CF 概念，包括“组件”（_component_）、“组件清单”（_component manifest_）、“运行器”（_runner_）、“领域”（_realm_）、“环境”（_environment_）、“能力”（_capabilities_）和“解析器”（_resolver_）。
-   [`component_manager`][doc-component-manager]（组件管理器）进程，它协调组件之间的通信和资源共享。
-   [FIDL API](#fidl-apis)，由`component_manager` 实现，或由其他组件实现并被`component_manager`使用，以协调为目的。
-   开发人员工具，用于构建、执行和测试组件。
-   语言特定库，用于组件与系统进行交互。（[示例](/sdk/lib/sys)）
-   测试工具与库，用于编写针对一个或多个组件的单元和集成测试。（[示例][doc-realm-builder]）

<!--
## Capabilities
 -->
## 能力

<!--
Since Fuchsia is a capability-based operating system, components interact with
each other through the use of _[capabilities][glossary.capability]_.
A capability combines access to a resource and a set of rights, providing both a
mechanism for access control and a means by which to interact with the resource.
 -->
由于 Fuchsia 是基于的操作系统，因此组件之间通过使用“[能力][glossary.capability]”彼此交互。能力结合了对资源的访问和一套权利，既提供了访问控制机制，又提供了与资源交互的手段。

<!--
To support the complex composition of software present in today's products, the
Component Framework provides distinct [capability types][doc-capabilities] built
upon Zircon [kernel objects][glossary.kernel-object].
A common representation of a capability is a [channel][glossary.channel] that
speaks a particular [FIDL][glossary.fidl] protocol.
 -->
为了支持当今产品中存在的软件复杂组成，组件框架提供了构建在 Zircon [内核对象][glossary.kernel-objects]之上的不同[能力类型][doc-capabilities]。能力的一种常见表现形式是[通道][glossary.channel]（channel），其使用一种特定的 [FIDL][glossary.fidl] 协议。

<!--
The Component Framework assembles the _[namespace][glossary.namespace]_ for a
component using [component declarations][glossary.component-declaration] that
describe the capabilities the component requires to function.
Components can discover the available capabilities in their namespace using the
[`fuchsia.io.Directory`][fidl-directory] protocol.
 -->
组件框架使用[组件声明][glossary.component-declaration]（component declaration）为组件组装“[命名空间][glossary.namespace]”（_namespace_），描述了组件运转需要功能。组件可以使用 [`fuchsia.io.Directory`][fidl-directory] 协议发现在其命名空间中可用的能力。

<!--
At runtime, every component receives its namespace as well as a handle to the
server end of a `Directory` channel. This `Directory` channel is called the
the _[outgoing directory][glossary.outgoing-directory]_. Through the
outgoing directory, the component's executable makes discoverable any
capabilities that it provides.
 -->
在运行时，每个组件都会收到其命名空间以及 `Directory` 通道的服务器端句柄。该 `Directory` 频道称为“[传出目录][glossary.outgoing-directory]”（_outgoing directory_）。通过传出目录，组件的可执行文件可发现其提供的任何功能。

<!--
The Component Framework brokers discovery from a providing component's
outgoing directory to a consuming component's namespace through a process called
_[capability routing][glossary.capability-routing]_.
While most capabilities are routed to component instances, _runner_ and
_resolver_ capabilities are routed to _[environments][glossary.environment]_.
Environments configure the behavior of the framework for the realms to which
they are assigned.
 -->
组件框架通过“[能力路由][glossary.capability-routing]”（_capability routing_）进程，来协调使用者组件的命名空间发现提供者组件的传出目录所提供的能力。虽然大多数能力是路由到组件实例的，但“运行器”和“解析器”能力被是路由到“[环境][glossary.environment]”的。环境为其分配到的领域配置框架的行为。

<!--
Note: In the Fuchsia process layer, "having a capability" means the process
holds a handle to the kernel object capability in its handle table. In the
Component Framework, we often use "having a capability" to mean that the
capability is discoverable through the component's namespace at runtime.
 -->
注意：在 Fuchsia 进程层中，“具有一项能力”是指该进程在其句柄表中具有内核对象能力的句柄。在组件框架中，我们经常使用“具有一项能力”来表示在运行时可以通过组件的命名空间发现该能力。

<!--
Further reading:
 -->
深入阅读：

<!--
* [Zircon kernel objects][doc-kernel-objects]
* [Component Framework capabilities][doc-capabilities]
* [Environments][doc-environments]
 -->
* [Zircon 内核对象][doc-kernel-objects]
* [组件框架能力][doc-capabilities]
* [环境][doc-environments]

<!--
## Components
 -->
## 组件

<!--
_Components_ are the foundational building blocks of software running in Fuchsia.
Each component is a composable, sandboxed module that interacts with other
components through capabilities.
 -->
**组件**是在 Fuchsia 中运行的软件的基础构建块。每个组件都是一个可组合的沙盒化模块，可通过能力与其他组件进行交互。

<!--
At its core, a component consists of the following:
 -->
核心上，一个组件包括以下内容：

<!--
* A [Component URL][glossary.component-url], which uniquely identifies that
  component.
* A [Component manifest][glossary.component-manifest], which describes how to
  launch the component, as well as any capability routes.
 -->
* 一个[组件网址][glossary.component-url]（Component URL），它独特地标识了该组件。
* 一份[组件清单][glossary.component-manifest]，其中描述了如何启动该组件以及任何功能路由。

<!--
The Component Framework relies on _component resolvers_ to retrieve components
from their origin. Resolvers take a component URL as an input and produce a
component manifest and (optionally) an access mechanism to the bytes of a
software package as output.
 -->
组件框架依赖于组件解析器（_component resolvers_）从其来源中检索组件。解析器将组件网址作为输入，产生组件清单以及（可选）软件包比特的访问机制作为输出。

<!--
Components that include an executable program may specify any runtime
(such as a raw process or a virtual machine) provided to the Component Framework
through a _[component runner][glossary.runner]_. Runners consume parts of the
manifest and the package, and provide the component's binary with a way to
execute.
 -->
包括可执行程序的组件可以通过“[组件运行器][glossary.runner]”（_component runner_）指定提供给组件框架的任何运行时（例如原始进程或虚拟机）。运行器使用清单和包的一部分，并为组件的二进制文件提供一种执行方式。

<!--
Note: Components without an executable program may still route capabilities and
host children, but no code will be executed for the component.
 -->
注意：没有可执行程序的组件仍可能会路由能力和托管子组件，但该组件不会执行任何代码。

<!--
Resolvers and runners are themselves capabilities and interact directly with the
framework to extend its functionality. Components can implement these
capabilities to add support for new component origins and runtimes.
 -->
解析器和运行器自身就是能力，并直接与框架互动以扩展其功能性。组件可以实现这些能力，为新组件来源和运行时提供支持。

<!--
Note: To bootstrap the system, `component_manager` includes a built-in
resolver, the `boot-resolver`, which resolves `fuchsia-boot://` URLs to
manifests on the boot image, as well as a built-in runner, the ELF runner,
which executes ELF binaries stored in signed Fuchsia packages.
 -->
注意：为了引导系统，`component_manager` 包括一个内置的解析器——`boot-resolver`，和一个内置运行器——ELF 运行器。前者可将启动镜像上的 `fuchsia-boot://` 网址解析为清单，后者可以执行存储在已签名 Fuchsia 包中的 ELF 二进制文件。

<!--
Further reading:
 -->
深入阅读：

<!--
* [Component manager][doc-component-manager]
* [Resolver capability][doc-resolvers]
* [Runner capability][doc-runners]
* [Fuchsia packages][doc-packages]
 -->
* [组件管理器][doc-component-manager]
* [解析器能力][doc-resolvers]
* [运行器能力][doc-runners]
* [Fuchsia 包][doc-packages]

<!--
### Composition
 -->
### 组合

<!--
A component together with its children are referred to as a
_[realm][glossary.realm]_.
The collective [parent][glossary.parent-component-instance] and
[child][glossary.child-component-instance] relationships of all individual
components are referred to as the
_[component instance tree][glossary.component-instance-tree]_.
A _[moniker][glossary.moniker]_ is a topological path that identifies a specific
component instance within a component instance tree. You will often see
monikers represented as POSIX-like path strings.
 -->
组件及其子组件合称为“[领域][glossary.realm]”（_realm_）。所有单独组件的集体[父][glossary.parent-component-instance][子][glossary.child-component-instance]关系称为[组件实例树][glossary.component-instance-tree]（_component instance tree_）。[代称][glossary.moniker]（_moniker_）是一种拓扑路径，可在组件实例树中识别特定的组件实例。您会经常看到代称表示为类 POSIX 路径字符串。

<!--
_[Component topology][glossary.component-topology]_ is the term for the
component instance tree and the collective capability routes over that tree.
 -->
_[组件拓扑][glossary.component-topology]（_Component topology_）是组件实例树和该树上的集体能力路由的术语。

<!--
Further reading:
 -->
深入阅读：

<!--
* [Component topology][doc-topology]
* [Realms][doc-realms]
 -->
* [组件拓扑][doc-topology]
* [领域][doc-realms]

<!--
### Lifecycle
 -->
### 生命周期

<!--
Components move through the following lifecycle states:
 -->
组件在以下生命周期状态间转移：

<!--
* Discovered
* Started
* Stopped
* Destroyed
 -->
* 发现（discovered）
* 启动（started）
* 停止（stopped）
* 销毁（destroyed）

<!--
Components are discovered either a) by virtue of being statically declared as a
child of another component in a component manifest, or b) by being added to a
[component collection][glossary.component-collection] at runtime. Similarly,
components are destroyed implicitly by being removed from the list of static
children in a component manifest, or explicitly by being removed from a
component collection at runtime.
 -->
组件通过在组件清单中被静态声明为另一个组件的子组件的方式，或通过在运行时添加到[组件集合][glossary.component-collection]（component collection）的方式被发现。类似地，组件通过从组件清单内静态子组件列表中删除的方式隐式销毁，或者通过在运行时从组件集合中删除来显式销毁。

<!--
When a component is started or stopped, `component_manager` coordinates with
the appropriate runner to execute or terminate the component's executable.
 -->
组件启动或停止时，`component_manager` 与相关运行器协调，以执行或终止组件的可执行文件。

<!--
Further reading:
 -->
深入阅读：

<!--
* [Component lifecycle][doc-lifecycle]
 -->
* [组件生命周期][doc-lifecycle]

[fidl-directory]: https://fuchsia.dev/reference/fidl/fuchsia.io#Directory
[glossary.capability]: /glossary#capability
[glossary.handle]: /glossary#handle
[glossary.channel]: /glossary#channel
[glossary.realm]: /glossary#realm
[glossary.environment]: /glossary#environment
[glossary.outgoing-directory]: /glossary#outgoing-directory
[glossary.moniker]: /glossary#moniker
[glossary.runner]: /glossary#runner
[glossary.parent-component-instance]: /glossary#parent-component-instance
[glossary.child-component-instance]: /glossary#child-component-instance
[glossary.component-collection]: /glossary#component-collection
[glossary.component-manifest]: /glossary#component-manifest
[glossary.component-url]: /glossary#component-url
[glossary.component-instance-tree]: /glossary#component-instance-tree
[glossary.component-topology]: /glossary#component-topology
[glossary.namespace]: /glossary#namespace
[glossary.component-declaration]: /glossary#component-declaration
[glossary.kernel-object]: /glossary#kernel-object
[glossary.capability-routing]: /glossary#capability-routing
[glossary.fidl]: /glossary#fidl
[doc-capabilities]: /concepts/components/v2/capabilities/README.md
[doc-kernel-objects]: /reference/kernel_objects/objects.md
[doc-storage-capability]: /concepts/components/v2/capabilities/storage.md
[doc-component-manager]: /concepts/components/v2/component_manager.md
[doc-declarations]: /concepts/components/v2/component_manifests.md#component-declaration
[doc-design-principles]: /concepts/components/v2/design_principles.md
[doc-environments]: /concepts/components/v2/environments.md
[doc-instances]: /concepts/components/v2/topology.md#component-instances
[doc-lifecycle]: /concepts/components/v2/lifecycle.md
[doc-realm-builder]: /development/testing/components/realm_builder.md
[doc-realms]: /concepts/components/v2/realms.md
[doc-runners]: /concepts/components/v2/capabilities/runners.md
[doc-resolvers]: /concepts/components/v2/capabilities/resolvers.md
[doc-topology]: /concepts/components/v2/topology.md
[doc-packages]: /concepts/packages/package.md
