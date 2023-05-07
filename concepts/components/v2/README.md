<!--
# Components
 -->
# 组件

<!--
Components are the basic unit of executable software on Fuchsia.
 -->
组件是 Fuchsia 中可执行软件的基本单元。

<!--
## Architectural concepts
 -->
## 架构性概念

<!--
-   [Introduction](introduction.md): Understanding components and the component
    framework.
-   [Component manager](component_manager.md): The runtime.
-   [Lifecycle](lifecycle.md): Component instance progression from creation to
    destruction.
-   [Topology](topology.md): The relationships among component instances.
-   [Realms](realms.md): Sub-trees of the component instance topology.
-   [Identifiers](identifiers.md): Identifiers for components and
    component instances.
 -->
 -  [简介](introduction.md)：了解组件和组件框架。
 -  [组件管理器](component_manager.md)（component manager）：运行时。
 -  [生命周期](lifecycle.md)（lifecycle）：组件实例从创建到销毁的发展过程。
 -  [拓扑](topology.md)（topology）：组件实例之间的关系。
 -  [领域](realms.md)（realm）：组件实例拓扑的子树。
 -  [标识符](identifiers.md)（identifier）：组件和组件实例的标识符。

<!--
## Developing components
 -->
## 开发组件

<!--
-   [Capabilities](capabilities/README.md): Different types of capabilities and
    how to route them between components.
-   [Component manifests](component_manifests.md): How to define a component for
    the framework.
-   [ELF runner](elf_runner.md): How to launch a component from an ELF file.
    Typically useful for developing system components in C++, Rust, or Go.
 -->
 -  [能力](capabilities/README.md)（capability）：不同类型的能力，以及如何在组件之间对其路由。
 -  [组件清单](component_manifests.md)（component manifest）：如何为框架定义组件。
 -  [ELF 运行器](elf_runner.md)（ELF runner）：如何从 ELF 文件启动组件。通常可用于开发 C++、Rust 或 Go 中的系统组件。

<!--
## Extending the component framework
 -->
## 扩展组件框架

<!--
-   [Runners](capabilities/runners.md): Instantiate components; add support for
    more runtimes.
-   [Resolvers](capabilities/resolvers.md): Find components from URLs; add
    support for methods of software packaging and distribution.
 -->
 -  [运行器](capabilities/runners.md)（runner）：实例化组件；为更多运行时提供支持。
 -  [解析器](capabilities/resolvers.md)（resolver）：从网址查找组件；软件打包和分发方法提供支持。

<!--
## Diagnostics
 -->
## 诊断工具

<!--
-   [Hub](hub.md): A live view of the component topology at runtime.
 -->
 -  [Hub](hub.md)：运行时组件拓扑的实时视图。

<!--
## Internals
 -->
## 内部结构

<!--
-   [Component manifest design principles][rfc0093]
-   [Components vs. processes](components_vs_processes.md): how the concepts
    differ.
 -->
 -  [组件清单设计原则][rfc0093]
 -  [组件与进程的比较](components_vs_processes.md)：概念差异。

[rfc0093]: /contribute/governance/rfcs/0093_component_manifest_design_principles.md
