<!--
# Component lifecycle
 -->
# 组件生命周期

<!--
This document describes how Component manager interacts with individual component
instances to manage their lifecycle.
 -->
本文档描述了组件管理器如何与单个组件实例进行交互以管理其生命周期。

<!--
## Lifecycle states {#states}
 -->
## 生命周期 {#states}

<!--
Component instances progress through the following major lifecycle states:
 -->
组件实例经历以下主要生命周期状态：

<!--
![Component lifecycle states](images/component-lifecycle.png){: width="662"}
 -->
![组件生命周期](images/component-lifecycle.png){：width =“ 662”}

<!--
Component instances may retain isolated persistent state on a storage medium
while they are not running, which can be used to help them maintain continuity
across restarts.
 -->
组件实例可能在未运行时在存储介质上保留隔离持久状态，这可用于帮助它们在重新启动后保持连续性。

<!--
### Created {#creating}
 -->
### 创建 {#creating}

<!--
A component instance may be created in the following ways:
 -->
组件实例可以通过以下方式创建（create）：

<!--
-   Configured as the root component of Component manager.
-   Statically discovered as the [child][doc-manifests-children] of another
    component.
-   Dynamically created at runtime in a [collection][doc-collections].
 -->
-   配置为组件管理器的根组件。
-   静态发现为另一组件的[子组件][doc-manifests-children]。
-   运行时在[集合][doc-manifests-children]（collection）中动态创建。

<!--
Every component instance has a component URL that describes how to resolve the
component, and a moniker that uniquely identifies the instance by its path from
the root. For more details, see [component identifiers][doc-identifiers].
 -->
每个组件实例都有一个组件网址，其描述了解析组件的方法，以及利用相对于根的路径来标识实例的代称。要获取更多详细信息，请参阅[组件标识符][doc-identifiers]。

<!--
Once created, a component instance can then be [resolved](#resolving) or
[destroyed](#destroying).
 -->
创建之后，组件实例就可以[被解析](#resolving)或[被销毁](#destroying)。

<!--
### Resolved {#resolving}
 -->
### 解析 {#resolving}

<!--
Resolving a component instance fetches the component declaration for the
specified component URL. Component manager resolves component URLs by finding a
[component resolver][doc-resolvers] that supports a matching URL scheme in the
environment. Developers can resolve components manually using the
[`ffx component resolve`][ref-ffx-resolve] command.
 -->
解析（resolve）组件实例会获取指定组件网址的组件声明。组件管理器通过在环境中找到支持匹配网址方案的[组件解析器][doc-resolvers]来解析组件网址。开发人员可以使用 [`ffx component resolve`][ref-ffx-resolve] 命令手动解析组件。

<!--
Components must successfully resolve before they can be [started](#starting).
 -->
组件必须在[启动](#starting)之前成功解析。

<!--
### Started {#starting}
 -->
### 启动 {#starting}

<!--
Starting a component instance loads and runs the component's program and
provides it access to the capabilities that it requires.
 -->
启动（start）组件实例会加载并运行组件的程序，并为其提供所需能力的访问权限。

<!--
The most common reason for starting a component instance is when another
component [binds](#binding) to one of its exposed capabilities. Developers can
also start components manually using the [`ffx component start`][ref-ffx-start]
command.
 -->
启动组件实例的最常见原因是另一个组件[绑定](#binding)至其公开的能力之一。开发人员还可以使用 [`ffx component start`][ref-ffx-start] 命令手动启动组件。

<!--
Once started, a component instance continues to run until it is
[stopped](#stopping).
 -->
启动之后，组件实例将继续运行，直到[停止](#stopping)。

<!--
### Stopped {#stopping}
 -->
### 停止 {#stopping}

<!--
Stopping a component instance terminates the component's program but preserves
its [persistent state][doc-storage]. Components enter this state when their
program exits, as defined by the component's [runner][doc-runners].
 -->
停止（stop）组件实例会终止组件的程序，但保留其[持久状态][doc-storage]（persistent state）。组件的程序退出时，该组件会进入此状态，具体由组件的[运行器][doc-runners]定义。

<!--
The Component Framework may stop a component instance for the following reasons:
 -->
组件框架可能由于以下原因而停止组件实例：

<!--
-   The component is about to be destroyed.
-   The system is shutting down.
 -->
-   该组件即将销毁。
-   系统正在关闭。

<!--
A component can implement a lifecycle handler ([example][handler-example]) to
receive a notification of events such as impending termination.
Note that components may not receive these events in circumstances such as
resource exhaustion, crashes, or power failure.
 -->
组件可以实现生命周期处理程序（[示例][handler-example]），以接收例如即将终止等事件的通知。请注意，在诸如资源耗尽、崩溃或电力故障的情况下，组件可能无法接收到这些事件。

<!--
Once stopped, a component instance may be [restarted](#starting) or
[shutdown](#shutdown).
 -->
停止之后，组件实例可以[重新启动](#starting)或[关闭](#shutdown)。

<!--
### Shutdown {#shutdown}
 -->
### 关闭 {#shutdown}

<!--
Component manager sets the final execution state of a component instance to
shutdown to indicate that it cannot be restarted and to signal that the instance
can be safely [destroyed](#destroying).
 -->
组件管理器将组件实例的最终执行状态设置为关闭（shutdown），以表明不能将其重新启动，并表示该实例可以安全[销毁](#destroying)。

<!--
### Destroyed {#destroying}
 -->
### 销毁 {#destroying}

<!--
A component instance may be destroyed in the following ways:
 -->
组件实例可以通过以下方式销毁（destroy）：

<!--
-   Dynamically removed from a [collection][doc-collections] at runtime. This is
    also true if the component is a descendant of another component being removed.
 -->
-   运行时从[集合][doc-collections]（collection）中动态删除。对于组件是要删除的另一个组件的后代的情况同理。

<!--
Once destroyed, Component manager completely removes the instance from the
component topology, including all persistent state. New instances of the same
component will each have their own identity and state distinct from all prior
instances.
 -->
销毁之后，组件管理器将完全从组件拓扑（包括所有持久状态）中删除实例。同一组件的新实例将各自拥有自己与所有先前实例不同的身份和状态。

<!--
## Lifecycle actions {#actions}
 -->
## 生命周期动作 {#actions}

<!--
This section describes common actions used by the Component Framework to
transition the lifecycle state of component instances.
 -->
本节介绍了组件框架使用的常见动作，以将组件实例过渡到不同的生命周期状态。

<!--
### Bind {#binding}
 -->
### 绑定 {#binding}

<!--
A component instance `A` _binds_ to another component instance `B` when `A`
connects to some capability that is provided by `B`. This causes component `B`
to [start](#starting) if it is not already running.
 -->
当组件实例 `A`连接到另一组件实例 `B` 提供的某些能力时，`A`“绑定”到 `B`。如果组件 `B` 尚未运行，这将导致它[启动](#starting)。

<!--
Concretely, there are two ways that `A` can bind to `B`:
 -->
具体地，`A` 可以通过两种方法绑定到 `B`：

<!--
-   `A` connects to a capability in its namespace which is
    [exposed][doc-manifests-expose] or [offered][doc-manifests-offer] by `B`.
    This is the most common way.
-   `A` binds to the [`fuchsia.component.Binder`][binder.fidl]
    [framework protocol][doc-framework-protocol] which is exposed or offered
    by `B`. Unlike a traditional capability, this protocol
    is implemented by the component framework.
 -->
-   `A` 连接到其命名空间中的功能，该功能由 `B` [公开][doc-manifests-expose]或[提供][doc-manifests-offer]。这是最常见的方式。
-   `A` 绑定到由 `B` 公开或提供的 [`fuchsia.component.Binder`][binder.fidl] [框架协议][doc-framework-protocol]。 与传统能力不同，该协议由组件框架实现。

<!--
Note: For more details on running components during development, see
[Run components][doc-run].
 -->
注意：要获取开发过程中运行组件的更多细节，请参阅[运行组件][doc-run]。

[binder.fidl]: https://fuchsia.dev/reference/fidl/fuchsia.component#Binder
[doc-framework-protocol]: capabilities/protocol.md#framework
[doc-collections]: realms.md#collections
[doc-identifiers]: identifiers.md
[doc-manifests-children]: https://fuchsia.dev/reference/cml#children
[doc-manifests-expose]: https://fuchsia.dev/reference/cml#expose
[doc-manifests-offer]: https://fuchsia.dev/reference/cml#offer
[doc-manifests]: component_manifests.md
[doc-resolvers]: capabilities/resolvers.md
[doc-runners]: capabilities/runners.md
[doc-storage]: capabilities/storage.md
[doc-run]: /development/components/run.md
[handler-example]: /examples/components/lifecycle
[ref-ffx-resolve]: https://fuchsia.dev/reference/tools/sdk/ffx#resolve
[ref-ffx-start]: https://fuchsia.dev/reference/tools/sdk/ffx#start
