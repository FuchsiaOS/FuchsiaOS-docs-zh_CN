<!--
# Protocol capabilities
 -->
# 协议能力

<!--
A [protocol capability][glossary.protocol-capability] is a capability backed
by a [channel][glossary.channel] that speaks a particular
[FIDL protocol][glossary.protocol].
 -->
[协议能力][glossary.protocol-capability]（protocol capability）是由[通道][glossary.channel]（channel）支持的能力，使用了特定的 [FIDL 协议][glossary.protocol]。

```fidl
library fuchsia.examples;

const MAX_STRING_LENGTH uint64 = 32;

@discoverable
protocol Echo {
    EchoString(struct {
        value string:MAX_STRING_LENGTH;
    }) -> (struct {
        response string:MAX_STRING_LENGTH;
    });
    SendString(struct {
        value string:MAX_STRING_LENGTH;
    });
    -> OnString(struct {
        response string:MAX_STRING_LENGTH;
    });
};
```

<!--
Note: For more details on FIDL protocol syntax, see the
[FIDL language reference][fidl-reference].
 -->
注意：要获取关于 FIDL 协议语法的更多细节，请参阅[FIDL 语言参考] [fidl-reference]。

<!--
Protocol implementations are served from provider components using the
[outgoing directory][glossary.outgoing-directory] and consumed from another
component's [namespace][glossary.namespace].
 -->
协议实现由使用[传出目录][glossary.outgoing-directory]（outgoing directory）的提供者组件提供，并从另一组件的[命名空间][glossary.namespace]使用。

<!--
## Providing protocol capabilities {#provide}
 -->
## 提供协议能力 {#provide}

<!--
To provide a protocol capability, a component must declare the capability and
[route](#route) it from `self`. The component hosts the protocol capability in
its [outgoing directory][glossary.outgoing-directory].
 -->
要提供目录能力，组件必须声明该能力，并从 `self` 对其[路由](#route)。组件在其[传出目录][glossary.outgoing-directory]中托管目录能力。
<!--
To define the capability, add a `capabilities` declaration for it:
 -->
要定义该能力，请为其添加 `capabilities` 声明：

```json5
{
    capabilities: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
        },
    ],
}
```

<!--
This defines a capability hosted by this component whose outgoing directory path
is `/svc/fuchsia.example.ExampleProtocol`. You can also customize the path:
 -->
这段声明定义了该组件托管的能力，其传出目录路径为 `/svc/fuchsia.example.ExampleProtocol`。您也可以自定义路径：

```json5
{
    capabilities: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
            path: "/my_svc/fuchsia.example.MyExampleProtocol",
        },
    ],
}
```

<!--
## Routing protocol capabilities {#route}
 -->
## 路由协议能力 {#route}

<!--
Components route protocol capabilities by [exposing](#expose) them to their
parent and [offering](#offer) them to their children.
 -->
组件通过将目录能力[公开](#expose)（expose）至其父级并[提供](#offer)（offer）至其子级来对其进行路由。

<!--
For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].
 -->
要获取关于框架路由组件能力方式的更多细节，请参阅[能力路由][capability-routing]。

<!--
### Exposing {#expose}
 -->
### 公开 {#expose}

<!--
Exposing a protocol capability gives the component's parent access to that
capability:
 -->
公开协议能力会给予父组件访问该能力的权限：

```json5
{
    expose: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
            from: "self",
        },
    ],
}
```

<!--
The `from: "self"` directive means that the protocol capability is
[provided](#provide) by this component.
 -->
`from: "self"` 表示协议能力由该组件[提供](#provide)。

<!--
### Offering {#offer}
 -->
### 提供 {#offer}

<!--
Offering a protocol capability gives a child component access to that
capability:
 -->
提供协议能力会给予子组件访问该能力的权限：

```json5
{
    offer: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
            from: "self",
            to: [ "#child-a", "#child_b" ],
        },
    ],
}
```

<!--
## Consuming protocol capabilities {#consume}
 -->
## 使用协议能力 {#consume}

<!--
To consume a protocol capability, the component must request the capability and
open the corresponding path in its [namespace][glossary.namespace].
 -->
要使用（consume）协议能力，组件必须请求该能力并在其[命名空间][glossary.namespace]中打开相应路径。

<!--
To request the capability, add a `use` declaration for it:
 -->
要请求该能力，请为其添加 `use`（使用）声明：

```json5
{
    use: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
        },
    ],
}
```

<!--
This populates the protocol in the component's namespace at the well-known path
`/svc/fuchsia.example.ExampleProtocol`. You can also customize the path:
 -->
这段声明定义了该组件托管的能力，其传出目录路径为 `/svc/fuchsia.example.ExampleProtocol`。您也可以自定义路径：

```json5
{
    use: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
            path: "/my_svc/fuchsia.example.MyExampleProtocol",
        },
    ],
}
```

<!--
For more information about the open request, see
[life of a protocol open][life-of-a-protocol-open].
 -->
要获取关于打开请求的更多信息，请参阅[协议打开的生命周期][life-of-a-protocol-open]。

<!--
Note: For a working example of routing a protocol capability between components,
see [`//examples/components/routing`][routing-example].
 -->
注意：要获取在组件间路由协议能力的工作示例，请参阅 [`//examples/components/routing`][routing-example]。

<!--
### Consuming optional protocol capabilities
 -->
### 使用可选协议能力

<!--
See [Connect Components: Consuming optional capabilities][consuming-optional-capabilities].
 -->
请参阅[连接组件：使用可选能力][consuming-optional-capabilities]。

<!--
## Framework protocols {#framework}
 -->
## 框架协议 {#framework}

<!--
A *framework protocol* is a protocol provided by the component framework.
Any component may `use` these capabilities by setting `framework` as the source
without an accompanying `offer` from its parent.
Fuchsia supports the following framework protocols:
 -->
“框架协议”（*framework protocol*）是组件框架提供的协议。任何组件都可以通过将框架（`framework`）设置为源来使用（`use`）这些能力，而无需相应的来自其父组件的 `offer`。Fuchsia 支持下列框架协议：

<!--
-   [`fuchsia.component.Realm`][fidl-realm]: Allows a component to manage and bind to
    its children. Scoped to the component's realm.
-   [`fuchsia.component.Binder`][fidl-binder]: Allows a component to start
    another component.
 -->
-   [`fuchsia.component.Realm`][fidl-realm]：允许组件管理和绑定至子组件。范围为组件的领域（realm）。
-   [`fuchsia.component.Binder`][fidl-binder]：允许组件启动另一组件。

```json5
{
    use: [
        {
            protocol: "fuchsia.component.Realm",
            from: "framework",
        },
    ],
}
```

[glossary.namespace]: /glossary/README.md#namespace
[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[glossary.channel]: /glossary/README.md#channel
[glossary.protocol]: /glossary/README.md#protocol
[glossary.protocol-capability]: /glossary/README.md#protocol-capability
[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[consuming-optional-capabilities]: /development/components/connect.md#consuming-optional-capabilities
[fidl-reference]: /reference/fidl/language/language.md
[fidl-binder]: /sdk/fidl/fuchsia.component/binder.fidl
[fidl-realm]: /sdk/fidl/fuchsia.component/realm.fidl
[life-of-a-protocol-open]: /concepts/components/v2/capabilities/life_of_a_protocol_open.md
[routing-example]: /examples/components/routing
