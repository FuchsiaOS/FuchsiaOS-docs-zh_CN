<!--
A protocol handle is a well-known object that provides an implementation of a
FIDL protocol that is discoverable using component namespaces. The component
framework facilitates protocol discovery between
[components](/glossary/README.md#component) using capabilities.
Capability routing describes which component should act as the provider for any
given client. Once the proper components are identified, the
[Component Manager](/glossary/README.md#component-manager)
initiates connections between components using handles found in each
component's namespace.
 -->
协议句柄是一个很常见的对象，它提供了一个 FIDL 协议的实现，FIDL 协议可以通过组件命名空间找到。组件框架使用能力促进了[组件](/glossary/README.md#component)之间的协议发现。能力路由描述了哪个组件应该作为任何给定客户端的提供者。一旦确定了适当的组件，[组件管理器](/glossary/README.md#component-manager)就会使用每个组件命名空间中的句柄来启动组件之间的连接。

<!--
Consider the following example for a `fuchsia.example.Foo` protocol:
 -->
请看以下 `fuchsia.example.Foo` 协议的示例：

<!--
![Diagram showing how connecting components is a combination of capability
routing and protocol serving. Components must serve the implementation of a
protocol they offer to other components.]
(/get-started/images/fidl/protocol-serving.png){: width="629"}
 -->
![图中显示了如何连接组件，组件是能力路由和协议服务的结合。组件必须为它们提供给其他组件的协议的实现来提供服务。](/get-started/images/fidl/protocol-serving.png){: width="629"}

<!--
The diagram highlights the main elements involved in performing the connection:
 -->
该图强调了执行连接所涉及的主要元素：

<!--
1.  The provider component statically **declares** the protocol in the
    `capabilities` section of the manifest. This enables the component framework
    to perform capability routing.
 -->
1.  提供者组件在清单的 `capabilities` 部分中静态地**声明**协议。这使组件框架能够执行能力路由。
<!--
2.  A client component statically **requests** the protocol in the `use` section
    of the manifest. This creates the `/svc/fuchsia.example.Foo` protocol entry
    in the client's namespace if capability routing is successful.
 -->
2.  客户端组件在清单（manifest）的 `use` 部分静态地**请求**协议。如果能力路由成功，这将在客户端的命名空间中创建 `/svc/fuchsia.example.Foo` 协议项。
<!--
3.  The provider code **publishes** the implementation at runtime. This creates
    a protocol entry at `/svc/fuchsia.example.Foo` in the provider's outgoing
    directory.
 -->
3.  提供者代码在运行时**发布**实现。这将在提供者传出目录中的 `/svc/fuchsia.example.Foo` 处创建一个协议项。
<!--
4.  The client code **connects** to the protocol handle at runtime. This opens a
    FIDL connection to the implementation running in the provider component.
 -->
4.  客户端代码在运行时**连接**到协议句柄。这将与运行在提供者组件中的实现打开一个 FIDL 连接。
