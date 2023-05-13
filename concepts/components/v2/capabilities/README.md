<!--
# Capabilities
 -->
# 能力

<!--
Components interact with one another through [capabilities][glossary.capability].
A capability combines access to a resource and a set of rights, providing a
access control and a means for interacting with the resource. Fuchsia
capabilities typically access underlying [kernel objects][glossary.kernel-object]
through [handles][glossary.handle] provided in the component's
[namespace][glossary.namespace].
 -->
组件间通过[能力][glossary.capability]（capability）交互。能力将对资源的访问及一系列权限相结合，提供了访问控制和与资源交互的方式。Fuchsia 能力通常通过组件[命名空间][glossary.namespace]中提供的[句柄][glossary.handle]访问底层[内核对象][glossary.kernel-object]。

<!--
A component can interact with the system and other components only through the
discoverable capabilities from its namespace and the few
[numbered handles][src-processargs] it receives.
 -->
组件只能通过其命名空间的可发现功能以及它收到的少数[已编号句柄][src-processargs]（numbered handles）来与系统和其他组件进行交互。

<!--
## Capability routing {#routing}
 -->
## 能力路由 {#routing}

<!--
Components declare new capabilities that they offer to the system and
capabilities provided by other components (or the framework) that they require
in their [component manifest][doc-component-manifest]. Component framework uses
these declarations to populate the namespace.
 -->
组件在[组件清单][doc-component-manifest]（component manifest）中声明其提供给系统的新能力以及所需的由其他组件（或框架）提供的能力。组件框架（component framework）使用这些声明来填充命名空间。

<!--
For capabilities to be available at runtime, there must also be a valid
[capability route][glossary.capability-routing] from the consuming component to
a provider. Since capabilities are most often routed through parent components
to their children, parent components play an important role in defining the
sandboxes for their child components.
 -->
对于在运行时可用的功能，还必须存在从使用者组件到提供者组件的有效[能力路由][glossary.capability-routing]。由于能力最常通过父组件路由至其子组件，因此父组件在为其子组件定义沙箱方面发挥重要作用。

<!--
Some capability types are routed to [environments][glossary.environment] rather
than individual component instances. Environments configure the behavior of
the framework for the realms where they are assigned. Capabilities routed to
environments are accessed and used by the framework. Component instances do not
have runtime access to the capabilities in their environment.
 -->
某些功能类型被路由到[环境][glossary.environment]（environment），而不是单个组件实例（component instance）。环境为分配的领域（realm）配置框架的行为。路由到环境的能力由框架访问和使用。组件实例对其环境中的能力没有运行时访问权限。

<!--
### Routing terminology {#routing-terminology}
 -->
### 路由术语 {#routing-terminology}

<!--
Routing terminology divides into the following categories:
 -->
路由术语分为以下类别：

<!--
1.  Declarations of how capabilities are routed between the component, its
    parent, and its children:
    -   `offer`: Declares that the capability listed is made available to a
        [child component][doc-children] instance or a
        [child collection][doc-collections].
    -   `expose`: Declares that the capabilities listed are made available to
        the parent component or to the framework. It is valid to `expose` from
        `self` or from a child component.
1.  Declarations of capabilities consumed or provided by the component:
    -   `use`: For executable components, declares capabilities that this
        component requires in its [namespace][glossary.namespace] at runtime.
        Capabilities are routed from the `parent` unless otherwise specified,
        and each capability must have a valid route from its source.
    -   `capabilities`: Declares capabilities that this component provides.
        Capabilities that are offered or exposed from `self` must appear here.
        These capabilities often map to a node in the
        [outgoing directory][glossary.outgoing-directory].
 -->
1.  能力在组件、其父组件及其子组件之间路由方式的声明：
    -   `offer`（提供）：声明列出的能力可用于[子组件][doc-children]实例（child component instance）或[子集合][doc-collections]（child collection）。
    -   `expose`（公开）：声明列出的能力可用于父组件或框架。从 `self` 或子组件“公开”（`expose`）是有效的。
1.  组件使用或提供能力的声明：
    -   `use`（使用）：对于可执行组件，声明该组件在运行时其[命名空间][glossary.namespace]中所需的能力。除非另有说明，否则能力路由自“父组件”，且每项能力必须具有来自源的有效路由。
    -   `capabilities`（能力）：声明该组件提供的能力。从 `self` 提供或公开的能力必须在此出现。这些能力通常映射到[传出目录][glossary.outgoing-directory]（outgoing directory）中的节点。

<!--
## Capability types {#capability-types}
 -->
## 能力类型 {#capability-types}

<!--
The following capabilities can be routed:
 -->
以下能力可以路由：

<!--
| type                                 | description                   | routed to                         |
| ------------------------------------ | ----------------------------- | --------------------------------- |
| [`protocol`][capability-protocol]    | A filesystem node that is     | components                        |
:                                      : used to open a channel backed :                                   :
:                                      : by a FIDL protocol.           :                                   :
| [`service`][capability-service]      | A filesystem directory that   | components                        |
:                                      : is used to open a channel to  :                                   :
:                                      : one of several service        :                                   :
:                                      : instances.                    :                                   :
| [`directory`][capability-directory]  | A filesystem directory.       | components                        |
:                                      :                               :                                   :
| [`storage`][capability-storage]      | A writable filesystem         | components                        |
:                                      : directory that is isolated to :                                   :
:                                      : the component using it.       :                                   :
| [`resolver`][capability-resolver]    | A capability that, when       | [environments][doc-environments]  |
:                                      : registered in an environment, :                                   :
:                                      : causes a component with a     :                                   :
:                                      : particular URL scheme to be   :                                   :
:                                      : resolved with that resolver.  :                                   :
| [`runner`][capability-runner]        | A capability that, when       | [environments][doc-environments]  |
:                                      : registered in an environment, :                                   :
:                                      : allows the framework to use   :                                   :
:                                      : that runner when starting     :                                   :
:                                      : components.                   :                                   :
 -->
| 类型                                        | 描述                                                                      | 路由至                   |
| ------------------------------------------- | ------------------------------------------------------------------------- | ------------------------ |
| [`protocol`][capability-protocol]（协议）   | 用于打开由 FIDL 协议支持的通道的文件系统节点。                            | 组件                     |
| [`service`][capability-service]（服务）     | 用于打开通向多个服务实例之一的通道的文件系统目录。                        | 组件                     |
| [`directory`][capability-directory]（目录） | 文件系统目录。                                                            | 组件                     |
| [`storage`][capability-storage]（存储）     | 与使用它的组件隔离的可写文件系统目录。                                    | 组件                     |
| [`resolver`][capability-resolver]（解析器） | 一种能力，当在环境中注册时，会导致具有特定 URL 方案的组件被该解析器解析。 | [环境][doc-environments] |
| [`runner`][capability-runner]（运行器）     | 一种能力，当在环境中注册时，允许框架在启动组件时使用该运行器。            | [环境][doc-environments] |

<!--
## Examples {#examples}
 -->
## 示例 {#examples}

<!--
Consider the following example that describes capability routing through the
component instance tree:
 -->
考虑以下示例，其描述了穿过组件实例树的能力路由：

<!--
<br>![Capability routing example](/concepts/components/v2/images/capability_routing_example.png)<br>
 -->
<br>![能力路由示例](/concepts/components/v2/images/capability_routing_example.png)<br>

<!--
In this example:
 -->
本示例中：

<!--
-   The `echo` component instance provides the `fuchsia.Echo` protocol as one
    of its declared *capabilities*.
-   The `echo_tool` component instance requires the *use* of the
    `fuchsia.Echo` protocol capability.
 -->
-   `echo` 组件实例将 `fuchsia.Echo` 协议作为其声明的“能力”（*capabilities*）之一提供。
-   `echo_tool` 组件实例需要“使用”（*use*）`fuchsia.Echo` 协议能力。

<!--
Each intermediate component cooperates to explicitly route `fuchsia.Echo`
from `echo` to `echo_tool`:
 -->
每个中间组件一同合作将 `fuchsia.Echo` 从 `echo` 显式路由到 `echo_tool`：

<!--
1.  `echo` *exposes* `fuchsia.Echo` from `self` so the protocol is visible to
    its parent, `services`.
1.  `services` *exposes* `fuchsia.Echo` from its child `echo` to its parent,
    `shell`.
1.  `shell` *offers* `fuchsia.Echo` from its child `services` to another child,
    `tools`.
1.  `tools` *offers* `fuchsia.Echo` from `parent` to its child, `echo_tool`.
 -->
1.   `echo` 将来自 `self` 的 `fuchsia.Echo` 公开（*expose*），使得该协议对其父组件 `services` 可见。
1.   `services` 将来自其子组件 `echo` 的 `fuchsia.Echo` 公开（*expose*）至其父组件 `shell`。
1.   `shell` 将来自其子组件 `services` 的 `fuchsia.Echo` 提供（*offers*）至另一子组件 `tools`。
1.   `tools` 将来自 `parent` 的 `fuchsia.Echo` 提供（*offers*）至其子组件 `echo_tool`。

<!--
Component Framework grants the request from `echo_tool` to use `fuchsia.Echo`
because a valid route is found to a component providing that protocol capability.
 -->
组件框架允许来自 `echo_tool` 的请求使用 `fuchsia.Echo`，因为找到了提供该协议能力的组件的有效路由。

<!--
For more information on how components connect to capabilities at runtime, see
[Life of a protocol open][doc-protocol-open].
 -->
要获取关于组件在运行时连接到能力的方式的更多信息，请参阅[协议打开的生命周期][doc-protocol-open]。

[capability-protocol]: /concepts/components/v2/capabilities/protocol.md
[capability-service]: /concepts/components/v2/capabilities/service.md
[capability-directory]: /concepts/components/v2/capabilities/directory.md
[capability-storage]: /concepts/components/v2/capabilities/storage.md
[capability-resolver]: /concepts/components/v2/capabilities/resolvers.md
[capability-runner]: /concepts/components/v2/capabilities/runners.md
[doc-children]: /concepts/components/v2/realms.md##child-component-instances
[doc-collections]: /concepts/components/v2/realms.md#collections
[doc-component-manifest]: /concepts/components/v2/component_manifests.md
[doc-environments]: /concepts/components/v2/environments.md
[doc-outgoing-directory]: /concepts/packages/system.md#outgoing_directory
[doc-protocol-open]: /concepts/components/v2/capabilities/life_of_a_protocol_open.md
[doc-resolvers]: /concepts/components/v2/capabilities/resolvers.md
[glossary.capability]: /glossary#capability
[glossary.capability-routing]: /glossary#capability-routing
[glossary.child]: /glossary#child-component-instance
[glossary.component]: /glossary#component
[glossary.environment]: /glossary#environment
[glossary.handle]: /glossary#handle
[glossary.kernel-object]: /glossary#kernel-object
[glossary.namespace]: /glossary#namespace
[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[glossary.parent]: /glossary#parent-component-instance
[src-processargs]: /zircon/system/public/zircon/processargs.h
