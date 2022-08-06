<!-- ## Component lifecycle -->
## 组件生命周期

<!-- 
Component instances are created and destroyed when they are added and removed
in the component topology. This can happen in one of two ways:
 -->
在组件拓扑中添加和移除组件实例时，会创建和销毁组件实例。这可以通过以下两种方式之一发生：

<!-- 
* **Statically**: The instance is declared in the component manifest as a child
  of another component in the tree. Static components are only created and
  destroyed when an update changes the component topology.
 -->
* **静态地**：该实例在组件清单中被声明为树中另一个组件的子组件。静态组件仅当一个更新更改组件拓扑时才会创建和销毁。
<!-- 
* **Dynamically**: The instance is added or removed in a component `collection`
  at runtime using the `fuchsia.component.Realm` protocol. Dynamic components are
  destroyed on system shutdown.
 -->
* **动态地**：实例在运行时使用 `fuchsia.component.Realm` 协议在组件 `collection` 中添加或移除。动态组件在系统关闭时销毁。

<!-- 
Once a component is destroyed, the framework removes its persistent state
(such as local storage).
 -->
一旦组件被销毁，框架就会移除它的持久状态（比如本地存储）。

<!-- 
The framework starts a component instance when another component attempts to
open a channel to it — known as **binding**. Binding happens **implicitly** when
connecting to a capability exposed by the component. Binding to a component that
is already started connects to the currently running instance.
 -->
框架会启动一个组件实例当另一个组件尝试打开一个通道到该组件实例时——这称为**绑定**。当连接到一个由组件公开的能力时，绑定会**隐式**发生。绑定到已经启动的组件会连接到当前运行的实例。

<!-- 
<aside class="key-point">
Components are initially <strong>stopped</strong> when they are created. A
component must be successfully <strong>resolved</strong> by a component resolver
before it can <strong>start</strong>.
</aside>
 -->
<aside class="key-point">组件在创建时，最初是<strong>停止的</strong>。组件必须先由组件解析器成功<strong>解析</strong>，然后才能<strong>启动</strong>。</aside>

<!-- 
Components may stop themselves by exiting the program (as defined by the
component's `runner`), or the framework may stop the component as part of
system shutdown. Before being destroyed, the framework moves components to a
**shutdown** state to indicate that it cannot be started again.
 -->
组件可能会通过退出程序（由组件的 `runner` 定义）自行停止，或者框架可能会在系统关闭时停止组件。在被销毁之前，框架将组件移动到**关闭**状态以指示它不能再次启动。

<!-- 
![Diagram showing how components have two distinct states: instance and
execution. Together, these states describe the "component lifecycle."]
(/get-started/images/components/component-lifecycle.png){: width="662"}
 -->
![该图显示了组件如何具有两种不同的状态：实例和执行。这些状态描述了“组件生命周期”。](/get-started/images/components/component-lifecycle.png){: width="662"}

<!-- 
Note: For more details on component states and execution, see
[component lifecycle](/concepts/components/v2/lifecycle.md).
 -->
注意：要获取关于组件状态和执行的更多详细信息，请参阅[组件生命周期](/concepts/components/v2/lifecycle.md)。
