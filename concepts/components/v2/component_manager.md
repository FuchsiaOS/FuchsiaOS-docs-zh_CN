<!--
# Component manager
 -->
# 组件管理器

<!--
The component manager is the heart of the component framework. It maintains the
[component topology][doc-topology], manages
[component lifecycle][doc-lifecycle], provides components with the
[capabilities][doc-capabilities] they require at runtime, and keeps them
isolated from one another.
 -->
组件管理器（component manager）是组件框架的核心。它维护[组件拓扑][doc-topology]（component topology），管理[组件生命周期][doc-lifecycle]，为组件在运行时提供需要的[能力][doc-capabilities]（capability），并使它们彼此隔离。

<!--
## Booting the system
 -->
## 引导系统

<!--
The component manager is responsible for starting most processes in the system.
It is one of the first processes created when the system boots and it is one
of the last processes destroyed when the system shuts down.
 -->
组件管理器负责启动系统中大多数的进程。它属于系统引导时创建的第一批进程，也属于系统关闭时销毁的最后一批进程。

<!--
The component manager coordinates the execution of all components, beginning
with the root component that is launched at boot. The root component then
asks the component manager to start other components such as the device
manager, filesystems, network stack, and other essential services.
 -->
组件管理器协调所有组件的执行，从引导时启动的根组件开始。之后，根组件要求组件管理器启动其他组件，例如设备管理器、文件系统、网络堆栈和其他基础服务。

<!--
## Intermediation
 -->
## 中介

<!--
The component manager intermediates all introductions between components at
runtime.
 -->
组件管理器在运行时充当所有组件间引入的中介（intermediation）。

<!--
For example, when a component connects to a [protocol][capability-protocol], the
component manager validates the request, uses
[capability routing][doc-capability-routing] to find the component that exposes
the desired service, starts it if needed, establishes a direct connection
between the client and the service, and continues to monitor the relationship so
that the client and service are held accountable for their behavior.
 -->
例如，当组件连接到一个[协议][capability-protocol]时，组件管理器验证请求，使用[能力路由][doc-capability-routing]来查找公开了所需服务的组件，并在需要时将其启动，建立客户与服务间的直接连接，之后继续监视该关系，以使客户和服务对其行为负责。

<!--
The component manager has a highly privileged role in the system. Through
intermediation, it makes many critical decisions for system security and
stability.
 -->
组件管理器在系统中的角色具有高度特权。通过充当中介，它为系统安全性和稳定性做出许多关键决策。

<!--
## Framework capabilities
 -->
## 框架能力

<!--
The component manager offers a variety of framework capabilities to components.
Components use these capabilities to interact with their environment with the
help of the component manager. For more details, see the corresponding
capabilities documentation:
 -->
组件管理器为组件提供了各种框架能力。组件使用这些能力在组件管理器的帮助下与其环境进行交互。要获取更多详细信息，请参阅相应能力的文档：

<!--
-   [Protocol capabilities][capability-protocol]
-   [Directory capabilities][capability-directory]
-   [Event capabilities][capability-event]
 -->
-   [协议能力][capability-protocol]（protocol capability）
-   [目录能力][capability-directory]（directory capability）
-   [事件能力][capability-event]（event capability）

<!--
## Framework extensions
 -->
## 框架扩展

<!--
The component manager supports a variety of framework extensions that
components can implement to integrate new functionality with their
[environment][doc-environments].
 -->
组件管理器支持组件可以实现的各种框架扩展，以将新功能与其[环境][doc-environments]集成在一起。

<!--
- [Runners][doc-runners]: Integrate programming language runtimes and
  application frameworks.
- [Resolvers][doc-resolvers]: Integrate software delivery systems.
 -->
- [运行器][doc-runners]：集成编程语言运行时和应用程序框架。
- [解析器][doc-resolvers]：集成软件交付系统。

[capability-directory]: /concepts/components/v2/capabilities/directory.md
[capability-event]: /concepts/components/v2/capabilities/event.md
[capability-protocol]: /concepts/components/v2/capabilities/protocol.md
[doc-capabilities]: /concepts/components/v2/capabilities
[doc-capability-routing]: /concepts/components/v2/topology.md#capability-routing
[doc-environments]: /concepts/components/v2/environments.md
[doc-lifecycle]: lifecycle.md
[doc-resolvers]: /concepts/components/v2/capabilities/resolvers.md
[doc-runners]: /concepts/components/v2/capabilities/runners.md
[doc-topology]: /concepts/components/v2/topology.md
