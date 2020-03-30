<!--# Sandboxing

This document describes how sandboxing works in Fuchsia.-->

# 沙盒

本文档描述了 Fuchsia 中沙盒的工作原理。

<!--## An empty process has nothing

On Fuchsia, a newly created process has nothing. A newly created process cannot
access any kernel objects, cannot allocate memory, and cannot even execute code.
Of course, such a process isn't very useful, which is why we typically create
processes with some initial resources and capabilities.-->

## 空进程没有任何内容

在 Fuchsia 中，新创建的进程没有任何内容。新创建的进程不能访问任何内核对象，不能分配内存，甚至不能执行代码。当然，这样的进程不是很有用，这就是为什么我们通常创建具有一些初始资源和能力的进程。

<!--Most commonly, a process starts executing some code with an initial stack, some
command line arguments, environment variables, a set of initial handles. One of
the most important initial handles is the `PA_VMAR_ROOT`, which the process can
use to map additional memory into its address space.-->

最常见的情况是，进程开始时执行一些带有初始堆栈、一些命令行参数、环境变量和一组初始句柄 `handles` 的代码。最重要的初始句柄之一是 `PA_VMAR_ROOT`，进程可以使用它将额外的内存映射到其地址空间。

<!--## Namespaces are the gateway to the world

Some of the initial handles given to a process are directories that the process
mounts into its _namespace_. These handles let the process discover and
communicate with other processes running on the system, including file systems
and other servers. See [Namespaces](namespaces.md) for more details.-->

## 命名空间是通向世界的大门

给进程的一些初始句柄是进程挂载到其命名空间 `_namespace_` 的目录。这些句柄允许进程发现并与系统上运行的其他进程通信，包括文件系统和其他服务。有关详细信息，请参阅[命名空间](Namespaces.md)。

<!--The namespace given to a process strongly influences how much of the system the
process can influence. Therefore, configuring the sandbox in which a process
runs amounts to configuring the process's namespace.-->

给进程的命名空间很大程度上决定了该进程可以支配多少系统资源。因此，运行配置进程的沙盒相当于该配置进程的命名空间。

<!--## Package namespace

A [component](../glossary.md#Component) run from a package is given access to
`/pkg`, which is a read-only view of the package containing the component. To
access these resources at runtime, a process can use the `/pkg` namespace. For
example, the `root_presenter` can access `cursor32.png` using the absolute path
`/pkg/data/cursor32.png`.-->

## 包命名空间

从包运行的[组件](../glossary.md#component)可以访问 `/pkg`，这是包含该组件的包的只读视图。要在运行时访问这些资源，进程可以使用 `/pkg` 命名空间。例如，`root_presenter` 可以使用绝对路径 `/pkg/data/cursor32.png` 访问 `cursor32.png`。

<!--## Services

Processes that are [components](../glossary.md#Component) receive an `/svc`
directory in their namespace. The services available through `/svc` are a
subset of the services provided by the component's
[environment](../glossary.md#Environment). This subset is determined by the
[`sandbox.services`](package_metadata.md#sandbox) whitelist in the
component's [manifest file](package_metadata.md#Component-manifest).-->

## 服务

属于[组件](../glossary.md#Component)的进程在其命名空间中接收 `/svc` 目录。通过 `/svc` 提供的服务是组件[环境](../glossary.md#Environment)提供的服务的子集。该子集由组件的[清单文件](package_metadata.md#Component-manifest)中的 [`sandbox.services`](package_metadata.md#sandbox) 白名单确定。

<!--A typical component will interact with a number of services from `/svc` in
order to play some useful role in the system. For example, the service
`fuchsia.sys.Launcher` is required if a component wishes to launch other
components.-->

典型的组件将与来自 `/svc` 的许多服务交互，以便在系统中发挥一些有用的作用。例如，如果组件希望启动其他组件，则需要 `fuchsia.sys.Launcher` 服务。

<!--Processes that are not components may or may not have `/svc`. These processes
receive whatever `/svc` their creator decided to provide to them.-->

不是组件的进程可能有也可能没有 `/svc`。这些进程接收其创建者决定提供给它们的任何 `/svc`。

<!--*NOTE:* In the past, there existed no mechanism for service sandboxing and a
component received all services in its environment. Pre-existing components
have been grandfathered to receive all services with the
`deprecated-all-services` feature, which will eventually be removed. Please do
not use `deprecated-all-services` for new components.-->

*注意:* 在过去，不存在服务沙箱机制，组件接收其环境中的所有服务。已将先前存在的组件升级为可接收所有具有 `deprecated-all-services` 功能的服务，这些服务最终将被删除。请不要在新组件使用 `deprecated-all-services`。

<!--## Configuring additional namespaces

If a process requires access to additional resources (e.g., device drivers),
the package can request access to additional names by including the `sandbox`
property in its  [Component Manifest](package_metadata.md#Component-Manifest)
for the package. For example, the following `meta/sandbox` file requests
direct access to the input driver:-->

## 配置其他命名空间

如果进程需要访问其他资源（例如，设备驱动程序），则包可以通过在其[组件清单](package_metadata.md#Component-Manifest)中包含 `sandbox` 属性来请求访问其他名称。例如，以下 `meta/sandbox` 文件请求直接访问输入驱动程序：

```
{
    "dev": [ "class/input" ]
}
```

<!--In the current implementation, the [AppMgr](../glossary.md#AppMgr) grants all such
requests, but that is likely to change as the system evolves.-->

在当前实现中，[AppMgr](../glossary.md#AppMgr) 会允许所有此类请求，但随着系统的发展，这种情况可能会发生变化。

<!--## Building a package

To build a package, use the `package()` macro in `gn` defined in
[`//build/package.gni`](https://fuchsia.googlesource.com/build/+/master/package.gni).
See the documentation for the `package()` macro for details about including resources.-->

## 构建包

若要生成包，请使用在 [`//build/package.gni`](https://fuchsia.googlesource.com/build/+/master/package.gni) 中定义的 `gn` 中的 `package()` 宏。有关导入资源的详细信息，请参见 `package()` 宏的文档。

<!--For examples, see [https://fuchsia.googlesource.com/garnet/+/master/packages/prod/fortune]
and [https://fuchsia.googlesource.com/garnet/+/master/bin/fortune/BUILD.gn].-->

例如, 您可以查看 [https://fuchsia.googlesource.com/garnet/+/master/packages/prod/fortune] 和 [https://fuchsia.googlesource.com/garnet/+/master/bin/fortune/BUILD.gn] 。
