<!--
# Life of a protocol open
 -->
# 协议打开的生命周期

<!--
This document describes the steps that occur when a component attempts to
connect to a protocol in its namespace.
 -->
本文档描述了组件尝试在其命名空间中连接到协议时发生的步骤。

<!--
These steps apply to the [Components v2][glossary.components-v2] model as run
under component manager.
 -->
这些步骤适用于组件管理器下运行的[组件 v2][glossary.components-v2] 模型。

<!--
At a high level these steps are:
 -->
这些步骤概括为：

<!--
-   Component manager will [construct a component's namespace][ns-construction]
    based on the `use` declarations in its manifest.
-   Once running, a component will attempt to [open a protocol][protocol-open]
    in its namespace.
-   This `Open` request is received by component manager, which performs the
    [capability routing][cap-routing] necessary to find the component providing
    the protocol.
-   Component manager [binds to the component providing the protocol][binding]
    and connects the `Open` request to it
 -->
-   组件管理器将基于组件清单中的 `use`（使用）声明[构造组件的命名空间][ns-construction]。
-   组件一旦运行，就会尝试在其命名空间中[打开协议][protocol-open]。
-   此 `Open`（打开）请求由组件管理器接收，并由其执行查找提供协议的组件所必需的[能力路由][cap-routing]。
-   组件管理器[绑定至提供协议的组件][binding]并将 `Open` 请求连接到它。
<!--
## Constructing a component's namespace
 -->
## 构造组件的名称空间

<!--
A [_namespace_][namespaces] is a set of directories that are offered to a
component when it is started. Each directory is associated with a file system
path through which the component may access files and protocols offered by other
components.
 -->
“[命名空间]”[namespaces]（_namespace_）是在组件启动时对其提供的一组目录。每个目录与文件系统路径关联，组件可以通过该路径访问其他组件提供的文件和协议。

<!--
These directories take the form of [handles][handle] to [channels][channel],
over which the component can use
[the `fuchsia.io.Directory` FIDL protocol][fuchsia.io].
 -->
这些目录采用[通道][channel][句柄][handle]（handle to channels）的形式，组件可以通过它们使用 [`fuchsia.io.Directory` FIDL 协议][fuchsia.io]。

<!--
For example, all components will receive a handle to the contents of the package
from which they were created at `/pkg`. This means that a component can see what
binaries are available in their package by reading the contents of `/pkg/bin`.
 -->
例如，所有组件都将收到一个指向一些内容的句柄，而组件是由这些内容创建在 `/pkg` 的。这意味着组件可以通过读取 `/pkg/bin` 的内容来查看其包中可用的二进制文件。

<!--
The `use` declarations in [the component's manifest][component-manifests]
determine how the namespace is populated. When a protocol capability is used...
 -->
[组件清单][component-manifests]中的 `use` 声明确定了命名空间的填充方式。当使用协议能力时……

```json5
use: [
    {
        protocol: "fuchsia.example.Foo",
    },
]
```

<!--
...component manager will add an entry to the component's namespace for the
parent directory of the protocol. In this example, the namespace path of the
protocol is `/svc/fuchsia.example.Foo` (the default path assignment), which
means that component manager will add a handle for `/svc` to the namespace.
 -->
……组件管理器将为该协议父目录的组件命名空间添加一个条目。此示例中，该协议的命名空间路径是 `/svc/fuchsia.example.foo`（默认路径分配），这意味着组件管理器将向命名空间添加 `/svc` 句柄。

<!--
The `/svc` directory is provided by component manager itself, and component
manager will respond to requests for protocols to this directory for the
lifetime of the component.
 -->
`/svc` 目录由组件管理器自身提供，组件管理器将在组件的生命周期内响应对该目录的协议请求。

<!--
The exact semantics of what appears in the namespace varies based on capability
type. For example if a directory capability is used instead of the protocol
capability...
 -->
命名空间中出现的内容的确切语义因能力类型而异。例如，如果使用了目录能力而非协议能力……

```json5
use: [
    {
        directory: "example-data",
        rights: [ "r*" ],
        path: "/example/data",
    },
]
```

<!--
...a handle for the directory itself appears in the namespace instead of a
handle for the parent directory. In this example, this means that a handle for
`/example/data` will appear in the namespace, whereas if this path was used for
a protocol capability `/example` would appear in the namespace.
 -->
……那么目录本身的句柄会出现在命名空间中，而非父目录的句柄。此示例中，这意味着 `/example/data` 句柄将出现在命名空间中，而如果该路径用于协议能力，那么 `/example` 将出现在命名空间中。

<!--
## A component opens a protocol
 -->
## 组件打开协议

<!--
When a component wants to open a protocol it creates a new channel pair, and
sends one end of this pair via an `Open` request over a channel in its
namespace. For example, if the component wanted to open a connection to
`/svc/fuchsia.example.Foo`, one end of the new channel pair would be sent over
the `/svc` handle in its namespace. The component may then call the
`fuchsia.example.Foo` protocol over the channel.
 -->
组件在想要打开协议时，会创建一个新的通道对，并利用其命名空间中的通道，通过 `Open` 请求发送该通道对的一端。例如，如果组件想要打开到 `/svc/fuchsia.example.Foo` 的连接，新通道对的一端将通过其命名空间中的 `/svc` 句柄发送。之后该组件可以通过通道调用`fuchsia.example.Foo` 协议。

<!--
Since the directory containing the protocol (`/svc`) is provided by component
manager, it is component manager that will receive the server end of the new
channel via the `Open` request sent by the component. Component manager then
must identify the component providing the protocol over this channel.
 -->
由于包含协议的目录（`/svc`）是由组件管理器提供的，因此将由组件管理器通过组件发送的 `Open` 请求接收新通道的服务器端。之后组件管理器必须识别通过该通道提供协议的组件。

<!--
## The `Open` triggers capability routing
 -->
## `Open` 触发能力路由

<!--
To determine the component that provides the protocol over the channel,
component manager must walk the tree of components, following `offer` and
`expose` declarations to find the capability's source. This process is referred
to as _capability routing_.
 -->
要确定通过通道提供协议的组件，组件管理器必须遍历组件树，根据 `offer`（提供）和 `expose`（公开）声明查找能力的来源。该过程称为“能力路由”（_capability routing_）。

<!--
Starting at the parent of the component that triggered the capability routing,
component manager will inspect each component's manifest, looking for an `offer`
declaration whose destination matches the child. The offer will specify a source
of either `parent`, `self`, or the name of a child. If the offer came from the
component's realm it will continue to walk up the tree, and if the offer came
from one of the component's children it will walk down the tree to that child.
 -->
从触发能力路由的父组件开始，组件管理器将检查每个组件的清单，寻找目标与该子组件匹配的 `offer` 声明。`offer` 将指定 `parent`、`self` 或子组件名称的源。如果 `offer` 来自组件的领域，那么将继续沿着树向上遍历；如果 `offer` 来自子组件，那么将沿着树向下遍历至该子组件。

<!--
Once the routing begins walking down the tree it will look for `expose`
declarations, which will specify a source of either `self` or the name of a
child. If the capability came from a child then component manager will continue
to walk down the tree.
 -->
路由一旦开始沿着树向下遍历，就将寻找 `expose` 声明，该声明会指定 `self` 或子组件名称的源。如果能力来自子组件，那么组件管理器将继续沿着树向下遍历。

<!--
Once an `offer` or `expose` declaration with a source of `self` is found, then
component manager can hand off the channel to that component.
 -->
一旦找到源为 `self` 的 `offer` 或 `expose` 声明，组件管理器就可以将通道移交给该组件。

<!--
If at any step of the way the chain is invalid, component manager will log an
error and close the channel it received from the `Open` call. This can be caused
by various situations, such as:
 -->
如果该链条中的任何一步无效，组件管理器将记录错误并关闭其从 `Open` 调用接收的通道。这可能由多种情况引起，例如：

<!--
-   A component `C` offered a capability from `parent`, but its parent `R` did
    not offer the capability to `C`.
-   A component `C` offered a capability from its child `D`, but child `D` did
    not expose the capability to `C`.
 -->
-   组件 `C` 提供了来自 `parent`（父组件）的能力，而其父组件 `R` 却没有向 `C` 提供该能力。
-   组件 `C` 提供了来自其子组件 `D` 的能力，而该子组件 `D` 却没有向 `C` 公开该能力。
- 
<!--
For example, consider the following tree of components and their manifests
(`program` blocks and runner setup omitted for brevity):
 -->
例如，考虑以下组件树及其清单（为简洁起见，省略了 `program` 块和运行器设置）：

```
    C
   / \
  B   D
 /
A

A.cml:
{
    // ...
    capabilities: [
        {
            protocol: "fuchsia.example.Foo",
        },
    ],
    expose: [
        {
            protocol: "fuchsia.example.Foo",
            from: "self",
        },
    ],
}

B.cml:
{
    // ...
    expose: [
        {
            protocol: "fuchsia.example.Foo",
            from: "#A",
        },
    ],
    children: [
        {
            name: "A",
            url: "fuchsia-pkg://fuchsia.com/a#meta/a.cm",
        },
    ]
}

C.cml:
{
    // ...
    offer: [
        {
            protocol: "fuchsia.example.Foo",
            from: "#B",
            to: [ "#D" ],
        },
    ]
    children: [
        {
            name: "B",
            url: "fuchsia-pkg://fuchsia.com/b#meta/b.cm",
        },
        {
            name: "D",
            url: "fuchsia-pkg://fuchsia.com/d#meta/d.cm",
        },
    ]
}

D.cml:
{
    // ...
    use: [
        {
            protocol: "fuchsia.example.Foo",
        },
    ],
}
```

<!--
When `D` calls `Open` on `/svc/fuchsia.example.Foo` in its namespace, component
manager will walk the tree to find the component that should provide this
protocol. It will start at `D`'s parent, `C`, and from there:
 -->
当 `D` 在其命名空间中的 `/svc/fuchsia.example.Foo` 调用 `Open` 时，组件管理器将遍历树来查找应当提供该协议的组件。它将从 `D` 的父组件 `C` 开始，并在此：

<!--
-   Look for the `offer` declaration for `fuchsia.example.Foo` to `D`, and see
    that it comes from child `B`.
-   Look for the `expose` declaration for `fuchsia.example.Foo` from `B`, and
    see that it comes from `A`.
-   Look for the `expose` declaration for `fuchsia.example.Foo` from `A`, and
    see that it comes from `self`. This means that `A` is the component
    providing the capability that `D` is attempting to use.
 -->
-   寻找 `offer` 至 `D` 的 `fuchsia.example.Foo` 声明，发现其来自子组件 `B`。
-   寻找 `expose` 自 `B` 的 `fuchsia.example.Foo` 声明，发现其来自 `A`。
-   寻找 `expose` 自 `A` 的 `fuchsia.example.Foo` 声明，发现其来自 `self`。这意味着 `A` 是提供 `D` 尝试使用的能力的组件。

<!--
Now that the provider component has been found, component manager can attempt to
hand off the channel it received via the `Open` request.
 -->
找到提供者组件之后，组件管理器可以尝试移交其通过 `Open` 请求接收到的通道。

<!--
## Binding to a component and sending a protocol channel
 -->
## 绑定至组件并发送协议通道

<!--
With the provider found the client component is now bound to the provider. This
will cause the component to start running if it is currently stopped.
 -->
找到提供者组件后，客户端组件绑定到了提供者组件。这将导致组件从停止状态开始运行。

<!--
Every component upon being started receives a server handle to an
[outgoing directory][glossary.outgoing-directory] in its handle table.
When a component is bound, component manager forwards the server end of the
protocol channel to the providing component's outgoing directory, under the
source path in the providing component's `offer` or `expose` declaration.
 -->
每个组件在启动时都会在其句柄表中接收到[传出目录][glossary.outgoing-directory]（outgoing directory）的服务器句柄。绑定组件时，组件管理器将协议通道的服务器端转发至提供者组件的传出目录，位于提供者组件的 `offer` 或 `expose` 声明中的源路径下。

<!--
In the above example component manager will send an `Open` request over the
outgoing directory handle for component `A` to the `/svc/fuchsia.example.Foo`
path, providing the channel handle that it received from component `D` when it
called `Open` to component manager.
 -->
在上述示例中，当组件 `D` 向组件管理器调用 `Open` 时，组件管理器将通过组件 `A` 的传出目录句柄向 `/svc/fuchsia.example.Foo` 路径发送 `Open` 请求，同时提供它从组件 `D` 收到的通道句柄。

<!--
It is then up to component `A` to receive this request and start responding to
messages over the channel it was given.
 -->
之后，由组件 `A` 接收此请求并开始通过给定的通道响应消息。

<!--
Since component manager directly forwards the server end of the protocol channel
to the provider component's outgoing directory, it is not involved in message
proxying and is entirely out of the picture after capability routing is
completed. Once a connection to another component is established, they talk
directly to each other with no arbiter in the middle.
 -->
由于组件管理器直接将协议通道的服务器端转发到提供者组件的传出目录，因此它不参与消息代理，并且在能力路由完成后彻底淡出视野。一旦建立了与另一个组件的连接，它们就会直接相互对话，中间没有仲裁者。

<!--
## Caveats
 -->
## 注意事项

<!--
### Runtime unpredictability
 -->
### 运行时不可预测性

<!--
Due to the runtime nature of capability routing and the behavior of the
components providing capabilities, there is no way to know if a given component
can successfully access a capability in its namespace before it attempts to do
so. Even if a valid offer/expose chain exists for the capability, package
updates could break this chain at runtime, and it's entirely possible a
component that claims to provide a capability in its manifest will fail to do so
when run.
 -->
由于能力路由的运行时特性和提供能力的组件行为，在尝试之前，是无法预知给定组件能否成功访问其命名空间中的能力的。即使该能力具有有效的提供/公开链，包更新也可能在运行时破坏该链，且声称在其清单中提供能力的组件在运行时完全有可能无法提供。
<!--
### Offered vs ambient capabilities
 -->
### 提供的能力与环境能力的比较

<!--
Some capabilities are provided by the component framework itself, and can be
directly used by (or will be implicitly provided to) components without their
parent offering these capabilities. Currently these are:
 -->
有些能力是由组件框架自身提供的，可以直接用于（或将会隐式提供至）组件，而无需其父组件提供这些能力。这些能力目前有：

<!--
-   `/pkg`: a handle to the package from which the component was created.
-   [`/svc/fuchsia.component.Realm`][realm.fidl]: a protocol which components can use
    to manage their own realm.
 -->
-   `/pkg`：创建组件的包的句柄。
-   [`/svc/fuchsia.component.Realm`][realm.fidl]：可以由组件用来管理自己领域的协议。

[binding]: #binding-to-a-component-and-sending-a-protocol-channel
[cap-routing]: #the-open-triggers-capability-routing
[channel]: /reference/kernel_objects/channel.md
[component-manifests]: /concepts/components/v2/component_manifests.md
[fuchsia.io]: https://fuchsia.dev/reference/fidl/fuchsia.io
[glossary.components-v2]: /glossary/README.md#components-v2
[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[handle]: /concepts/kernel/handles.md
[namespaces]: /concepts/process/namespaces.md
[ns-construction]: #constructing-a-components-namespace
[protocol-open]: #a-component-opens-a-protocol
[realm.fidl]: /sdk/fidl/fuchsia.component/realm.fidl
