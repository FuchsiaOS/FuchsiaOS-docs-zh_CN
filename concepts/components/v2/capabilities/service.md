<!--
# Service capabilities
 -->
# 服务能力

<!--
Caution: service capabilities are experimental and in development. Their
behavior and APIs could change at any time.
 -->
注意：服务能力是实验性的，正在开发中。其行为和 API 可以随时改变。

<!--
A [service capability][glossary.service-capability] is a capability that
enables discovery of one or more individually named
[FIDL service][glossary.service] instances. Service capabilities are backed by
a [glossary.channel] that speaks the [`Directory`][directory.fidl] protocol,
where each entry in the directory exposes a named [service instance](#instances).
 -->
[服务能力][glossary.service-capability]（service capability）是一种能够发现一个或多个单独的具名 [FIDL 服务][glossary.service]实例的能力。服务能力由使用 [`Directory`][directory.fidl] 协议的[通道][glossary.channel]支持，其中目录内的每个条目都公开一个具名[服务实例](#instances)。

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

service EchoService {
    regular_echo client_end:Echo;
    reversed_echo client_end:Echo;
};
```

<!--
Note: For more details on FIDL service syntax, see the
[FIDL language reference][fidl-reference].
 -->
注意：要获取关于 FIDL 服务语法的更多细节，请参阅 [FIDL 语言参考][fidl-reference]。

<!--
Service implementations are served from provider components using the
[outgoing directory][glossary.outgoing-directory] and consumed from another
component's [namespace][glossary.namespace].
 -->
服务实现通过[传出目录][glossary.outgoing-directory]从提供者组件提供，并从另一组件的[命名空间][glossary.namespace]中使用。

<!--
## Service instances {#instances}
 -->
## 服务实例 {#instances}

<!--
Multiple named instances of a service can be hosted by a single component.
These are present in the [namespace][glossary.namespace] of the consuming
component as subdirectories of the service.
The component framework generates an arbitrary, unique identifier for each
service instance name.
 -->
服务的多个具名实例可由单个组件托管。这些示例作为服务的子目录存在于消费者组件的[命名空间][glossary.namespace]中。组件框架为每个服务实例名称生成任意的唯一标识符。

<!--
For example, if the framework generates `57dfe118a2a8` as the instance name of
the `fuchsia.examples.EchoService` service, a consuming component could connect
to the protocols in that instance using the following namespace paths:
 -->
例如，如果框架生成 `57dfe118a2a8` 作为 `fuchsia.examples.EchoService` 服务的实例名称，则消费者组件可以使用以下命名空间路径连接到该实例中的协议：

- `/svc/fuchsia.examples.EchoService/57dfe118a2a8/regular_echo`
- `/svc/fuchsia.examples.EchoService/57dfe118a2a8/reversed_echo`

<!--
## Providing service capabilities {#provide}
 -->
## 提供服务能力 {#provide}

<!--
To provide a service capability, a component must declare the capability and
[route](#route) it from `self`. The component hosts the service capability in
its [outgoing directory][glossary.outgoing-directory].
 -->
要提供服务能力，组件必须声明该能力，并从 `self` 对其[路由](#route)。组件在其[传出目录][glossary.outgoing-directory]（outgoing directory）中托管服务能力。

<!--
To define the capability, add a `capabilities` declaration for it:
 -->
要定义该能力，请为其添加 `capabilities`（能力）声明：

```json5
{
    capabilities: [
        {
            service: "fuchsia.example.ExampleService",
        },
    ],
}
```

<!--
This defines a capability hosted by this component whose outgoing directory path
is `/svc/fuchsia.example.ExampleService`. You can also customize the path:
 -->
这段声明定义了该组件托管的能力，其传出目录路径为 `/svc/fuchsia.example.ExampleService`。您也可以自定义路径：

```json5
{
    capabilities: [
        {
            service: "fuchsia.example.ExampleService",
            path: "/my_svc/fuchsia.example.MyExampleService",
        },
    ],
}
```

<!--
## Routing service capabilities {#route}
 -->
## 路由服务能力 {#route}

<!--
Components route service capabilities by [exposing](#expose) them to their
parent and [offering](#offer) them to their children.
 -->
组件通过将服务能力[公开](#expose)（expose）至其父级并[提供](#offer)（offer）至其子级对其进行路由。

<!--
For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].
 -->
要获取关于框架路由组件能力方式的更多细节，请参见[能力路由][capability-routing]。

<!--
### Exposing {#expose}
 -->
### 公开 {#expose}

<!--
Exposing a service capability gives the component's parent access to that
capability:
 -->
公开（expose）服务能力会给予父组件访问该能力的权限：

```json5
{
    expose: [
        {
            service: "fuchsia.example.ExampleService",
            from: "self",
        },
    ],
}
```

<!--
The `from: "self"` directive means that the service capability is
[provided](#provide) by this component.
 -->
`from: "self"` 表示服务能力由该组件[提供](#provide)。

<!--
#### Dynamic collections
 -->
#### 动态集合

<!--
A service capability can be exposed from a [dynamic collection][collection]:
 -->
服务能力可以从[动态集合][collection]（dynamic collection）公开：

```json5
{
    collections: [
        {
            name: "coll",
            durability: "transient",
        },
    ],
    expose: [
        {
            service: "fuchsia.example.ExampleService",
            from: "#coll",
        },
    ],
}
```

<!--
Note: When routing services exposed from components in the collection, the
component framework renames each [service instance](#instances) with an
arbitrary, unique identifier to allow multiple components in the collection to
expose the same service.
 -->
注意：当对公开自集合中组件的服务进行路由时，组件框架使用任意的唯一标识符重新命名每个[服务实例](#instances)，以允许集合中的多个组件公开相同服务。

<!--
### Offering {#offer}
 -->
### 提供 {#offer}

<!--
Offering a service capability gives a child component access to that
capability:
 -->
提供（offer）服务能力会给予子组件访问该能力的权限：

```json5
{
    offer: [
        {
            service: "fuchsia.example.ExampleService",
            from: "self",
            to: [ "#child-a", "#child_b" ],
        },
    ],
}
```

<!--
## Consuming service capabilities {#consume}
 -->
## 使用服务能力 {#consume}

<!--
To consume a service capability, the component must request the capability and
open the corresponding path in its [namespace][glossary.namespace].
 -->
要使用（consume）服务能力，组件必须请求该能力并在其[命名空间][glossary.namespace]中打开相应路径。

<!--
To request the capability, add a `use` declaration for it:
 -->
要请求该能力，请为其添加 `use`（使用）声明：

```json5
{
    use: [
        {
            service: "fuchsia.example.ExampleService",
        },
    ],
}
```

<!--
This populates the service in the component's namespace at the well-known path
`/svc/fuchsia.example.ExampleService`. You can also customize the path:
 -->
这将会填充组件的命名空间中的服务，其所在路径易知，为 `/svc/fuchsia.example.ExampleService`。

```json5
{
    use: [
        {
            service: "fuchsia.example.ExampleService",
            path: "/my_svc/fuchsia.example.MyExampleService",
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
Note: For a working example of routing a service capability between components,
see [`//examples/components/services`][routing-example].
 -->
注意：要获取在组件间路由服务能力的工作示例，请参阅 [`//examples/components/services`][routing-example]。

[glossary.channel]: /glossary/README.md#channel
[glossary.namespace]: /glossary/README.md#namespace
[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[glossary.protocol]: /glossary/README.md#protocol
[glossary.service]: /glossary/README.md#service
[glossary.service-capability]: /glossary/README.md#service-capability
[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[collection]: /concepts/components/v2/realms.md#collections
[fidl-reference]: /reference/fidl/language/language.md
[life-of-a-protocol-open]: /concepts/components/v2/capabilities/life_of_a_protocol_open.md
[directory.fidl]: https://fuchsia.dev/reference/fidl/fuchsia.io#Directory
[realm.fidl]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Realm
[routing-example]: /examples/components/services
