<!-- A component is a program that runs on Fuchsia in its own sandbox.
Each component is a composable module that interacts with other components
through their capabilities. All software on Fuchsia is defined as a component
except for the kernel image, bootstrapping processes, and the Component Manager. -->
组件是 Fuchsia 上运行在其自有沙盒中的程序。每个组件都是一个可组合的模块，通过其能力（capability）与其它组件交互。Fuchsia 中所有软件都被定义为组件，除了内核镜像、引导进程以及组件管理器（Component Manager）。

<!-- Fuchsia's component framework is responsible for running nearly all user space
software in the system. The Component Manager is a system process that coordinates
the execution and lifecycle of all component instances, maintains the component
topology, provides components with their capabilities, and keeps them isolated
from one another. -->
Fuchsia 的组件框架致力于让系统可以运行几乎所有的用户空间软件。组件管理器是一个系统进程，负责协调各个组件实例的执行和生命周期、维护组件拓扑、给组件提供相应功能，并使它们彼此隔离。

<!-- Components obtain privileges to access various parts of the wider system through
capabilities. Each component can declare new capabilities that they offer to the
system and capabilities provided by other components (or the framework) that
they require to function. Component Manager resolves and validates all capability
requests between components to ensure they match capabilities that the component
has been properly granted. -->
组件通过能力来获得访问更广泛系统的各个部分的权限。每个组件都可以声明它们提供给系统的新能力，以及它们运行所需的其他组件（或框架）提供的能力。组件管理器解析并验证组件间的所有能力请求，以确保它们与组件已被正确授予的能力相匹配。

<aside class="key-point">
<!-- <b>Legacy components</b> -->
<b>旧版组件</b>

<!-- <p>This section focuses on modern components whose manifest declarations are
written in component manifest language (CML). The legacy framework based on
<code>appmgr</code> and declared using CMX manifests is not covered here. -->
<p>本节重点介绍使用组件清单语言 (CML) 编写清单声明的现代组件。此处未介绍基于 <code>appmgr</code> 并使用 CMX 清单声明的旧框架。

<!-- <p>For more details on the legacy component framework, see
<a href="/concepts/components/v1">legacy components</a>. -->
<p>要获取关于会话框架的更多细节，请参阅
<a href="/concepts/components/v1">旧版组件</a>。
</aside>
