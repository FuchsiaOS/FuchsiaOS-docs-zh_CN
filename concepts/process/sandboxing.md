<!-- # Sandboxing

This document describes how sandboxing works for a process in Fuchsia. -->

# 沙盒

本文档描述了沙盒是如何为一个 Fuchsia 进程工作的。

<!-- ## A new process has nothing

In Fuchsia, a newly created process is empty. It cannot access any kernel
objects, allocate memory, or execute code. Because of this, processes are
usually created with some initial resources and capabilities. -->

## 新进程啥也没有

在 Fuchsia 中，一个新创建的进程是空的。它不能访问任何内核对象，不能分配内存也不能运行代码。正因如此，进程创建时通常只有一些初始资源和功能。

<!-- Most commonly, a process starts executing some code with an initial stack, some
command line arguments, some environment variables, and a set of initial
handles.
[Zircon program loading and dynamic linking](/docs/concepts/booting/program_loading.md)
describes the resources provided to programs when starting. -->

通常，一个进程会带着一些命令行参数、一些环境变量和一系列初始操作，在一个初始的堆栈上开始执行一些代码。[Zircon 程序的加载与动态链接](/docs/concepts/booting/program_loading.md) 描述了程序开始执行时所提供的资源。

<!-- ## Namespaces are the gateway to the world

Some of the initial handles given to a process are directories that the process
mounts into its _namespace_. These handles let the process discover and
communicate with other processes running on the system, including file systems
and other servers. See [Namespaces](/docs/concepts/process/namespaces.md) for
more details. -->

## 命名空间是通向世界的大门

进程的一些初始操作会指导它挂载到它的 _命名空间_ 上。这些操作使该进程能够发现和与系统上其他进程进行交流，包括文件系统和其他服务器。详见 [命名空间](/docs/concepts/process/namespaces.md)。

<!-- The namespace given to a process strongly influences how much of the system the
process can influence. Therefore, configuring the sandbox in which a process
runs amounts to configuring the process's namespace. -->

给予进程的命名空间在很大程度上决定了这个进程能在多大范围里影响到操作系统。因此，配置某个进程运行的沙盒相当于配置该进程的命名空间。

<!-- ## Package namespace

A [component](/docs/glossary.md#component) run from a package is given access to
`/pkg`, which is a read-only view of the package containing the component. To
access these resources at runtime, a process can use the `/pkg` namespace. For
example, the `root_presenter` can access `cursor32.png` using the absolute path
`/pkg/data/cursor32.png`. -->

## 包命名空间

一个 [组件](/docs/glossary.md#component) 从一个能访问 `/pkg` 的包开始运行，`/pkg` 是一个包含该组件的包的只读视图。一个进程能通过 `/pkg` 命名空间在运行时访问这些资源。例如，`root_presenter` 能使用绝对路径 `/pkg/data/cursor32.png` 来访问 `cursor32.png`。
.

<!-- ## Services

Processes that are [components](/docs/glossary.md#component) receive an `/svc`
directory in their namespace. The contents of `/svc` are populated differently
depending on whether the component is [v1](/docs/glossary.md#components-v1) or
[v2](/docs/glossary.md#components-v2). -->

## 服务

组件进程在它们的命名空间接受一个 `/svc` 目录。`/svc` 的内容根据该组件是 [v1](/docs/glossary.md#components-v1) 还是 [v2](/docs/glossary.md#components-v2) 而不同。

<!-- A typical component will interact with a number of services from `/svc` in order
to play some useful role in the system. For example, the service
`fuchsia.logger.LogSink` is required if a component wishes to log. -->

一个典型的组件会和 `/svc` 中一定数量的服务进行交互，以便在系统中发挥一些有用的作用。例如，当一个组件希望记录日志时，服务 `fuchsia.logger.LogSink` 将会被用到。

<!-- Processes that are not components may or may not have `/svc`. These processes
receive whatever `/svc` their creator provided to them. -->

不是组件的进程可能有也可能没有 `/svc`。这些进程会接受它的创建者所提供的任何  `/svc`。

### Components v1 {#services-components-v1}

<!-- In Components v1, the services available through `/svc` are a subset of the
services provided by the component's
[environment](/docs/glossary.md#environment). This subset is determined by the
[`sandbox.services`](/docs/concepts/components/v1/component_manifests.md#sandbox)
allowlist in the component's
[manifest file](/docs/concepts/components/v1/component_manifests.md). -->

### v1 组件 {#services-components-v1}

在 v1 组件中，通过 `/svc` 获得的服务是该组件的 [环境](/docs/glossary.md#environment) 所提供的服务的一个子集。这个子集由组件 [清单文件（manifest file）](/docs/concepts/components/v1/component_manifests.md) 中的 [`沙盒服务（sandbox.services）`](/docs/concepts/components/v1/component_manifests.md#sandbox) 许可表来决定。

<!-- ### Components v2 {#services-components-v2}

In Components v2, the services available through `/svc` are determined by a
component manifest's
[`use`](/docs/concepts/components/v2/component_manifests.md#use) declarations,
typically [`protocol`](/docs/concepts/components/v2/capabilities/protocol.md)
declarations. -->

### v2 组件 {#services-components-v2}

在 v2 组件中，能通过 `/svc` 获得的服务由组件清单（manifest）中的 [`use`](/docs/concepts/components/v2/component_manifests.md#use) 声明，典型例子是 [`协议`](/docs/concepts/components/v2/capabilities/protocol.md) 的声明。

<!-- ## Configuring additional namespaces

If a component requires access to additional resources (for example, device
drivers), the package can request access to additional names by including the
`sandbox` property in its
[Component Manifest](/docs/concepts/components/v1/component_manifests.md) for
the package. For example, to request direct access to the input drive, include
the following `dev` array in your `sandbox`:

```
{
    "dev": [ "class/input" ]
}
``` -->

## 配置额外的命名空间

如果一个组件需要访问额外的资源（如，设备驱动），它的包就能够通过在包的 [组件清单（Component Manifest）](/docs/concepts/components/v1/component_manifests.md) 内的 `沙盒（sandbox）` 属性中添加额外的名字，以此进行访问请求。例如，在 `沙盒（sandbox）` 中添加如下的 `dev` 数组来请求直接访问输入设备：

```
{
    "dev": [ "class/input" ]
}
```
