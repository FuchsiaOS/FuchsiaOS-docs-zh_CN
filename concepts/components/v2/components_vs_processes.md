<!--
# Components vs. processes
 -->
# 组件与进程的比较

<!--
This document explains how the concept of components differs from processes and
related concepts.
 -->
本文档说明了组件的概念与进程及有关概念的不同。

<!--
The Zircon kernel defines [processes] and other [task objects] that are common
in modern operating systems. The abstraction of [component instances] sometimes
correlates with Zircon task abstractions, but not always.
 -->
Zircon 内核定义了现代操作系统常见的[进程][processes]和其他[任务对象][task objects]。[组件实例][component instances]的抽象有时与 Zircon 任务抽象相关，但并非总是如此。

<!--
## Examples
 -->
## 示例

<!--
The relationship between components and Zircon tasks differs, often as defined
by [component runners], which implement strategies for launching component
instances.
 -->
组件和 Zircon 任务之间的关系通常是由[组件运行器][component runners]定义的，该运行器对启动组件实例的策略进行实现。

<!--
-   [ELF Runner] launches components by creating a new [job] that contains a
    process that's created from a given executable file in ELF format.
-   Dart Runner launches a new Dart isolate in a Dart Virtual Machine. A Dart
    VM is implemented as a process that can host one or more Dart isolate.
    Dart isolates execute on [threads], but don't necessarily have an
    assigned thread (this is a VM implementation detail).
-   Web runner can launch one or more web pages as components, and host them
    the same web engine container or in separate containers per its isolation
    policy. Web pages are typically isolated by being hosted in separate
    processes.
 -->
-   [ELF 运行器][ELF Runner]通过创建新的[作业][job]（job）来启动组件，该作业包含一个创建自给定 ELF 格式可执行文件的进程。
-   Dart 运行器在 Dart 虚拟机中启动了新的 Dart 隔离（isolate）。DART 虚拟机被实现为可以托管一个或多个 Dart 隔离的进程。DART 隔离在[线程][threads]上执行，但不一定具有分配的线程（这是虚拟机实现细节）。
-   Web 运行器可以将一个或多个网页作为组件启动，并根据其隔离策略将它们托管在同一或各自的 Web 引擎容器中。网页通常是通过托管在单独的进程中进行隔离的。

[processes]: /reference/kernel_objects/process.md
[task objects]: /reference/kernel_objects/objects.md#tasks
[component instances]: /concepts/components/v2/topology.md#component-instances
[component runners]: /concepts/components/v2/capabilities/runners.md
[ELF Runner]: /concepts/components/v2/elf_runner.md
[job]: /reference/kernel_objects/job.md
[threads]: /reference/kernel_objects/thread.md
