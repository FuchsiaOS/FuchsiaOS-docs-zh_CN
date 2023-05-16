<!--
# Attributing LogSink connections
 -->
# 归因 LogSink 连接

<!--
When a Fuchsia component wishes to write logs, it must obtain a connection to a
[`fuchsia.logger.LogSink`][logsink-protocol] in its environment, typically provided
by an instance of the [Archivist].
 -->
Fuchsia 组件希望编写日志时，必须在其环境中获得与 [`fuchsia.logger.LogSink`][logsink-protocol] 的连接，通常由[归档器][Archivist]（Archivist）实例提供。

<!--
Typical Fuchsia service connections are anonymous such that the server and client have no
identifying information about each other. The client only sees the service in their namespace, e.g.
`/svc/fuchsia.logger.LogSink`, and the server sees an anonymous `Open()` request to their incoming
namespace.
 -->
典型的 Fuchsia 服务连接是匿名的，因此服务器和客户端没有关于彼此的识别信息。客户端只能在其命名空间中看到服务，例如 `/svc/fuchsia.logger.LogSink`，而服务器看到一个匿名的 `Open()` 请求进入其传入命名空间。

<!--
At the same time, it's important to know from where logs come, as trustworthy provenance
metadata enables better monitoring, storage, querying, and presentation of logs. The system solves
this with a feature called "attributed logging" which identifies the source of an incoming `LogSink`
connection.
 -->
同时，了解日志来源也很重要，因为可靠来源的元数据可以更好地监控、存储、查询和展现日志。系统通过名为“归因日志记录”（attributed logging）的功能解决了这一问题，该功能可以识别传入的 `LogSink` 连接的来源。

<!--
### Component Manager: CapabilityRequested events
 -->
### 组件管理器：能力请求事件

<!--
[Archivist's manifest] `expose`s `fuchsia.logger.LogSink` just like other service capabilities, but
it also `use`s an event from the framework, binding it to a service in its namespace:
 -->
[归档器的清单][Archivist's manifest]同其他服务能力类似，公开（`expose`）`fuchsia.logger.LogSink`，但也使用（`use`）来自框架的事件，将其绑定至其命名空间中的服务：

```json5
{
    event: "capability_requested",
    from: "framework",
    filter: { name: "fuchsia.logger.LogSink" },
},
{
    event_stream: "EventStream",
    subscriptions: [
        {
            event: "capability_requested",
            mode: "async",
        }
    ],
},
```

<!--
This causes Component Manager to redirect incoming requests from the default `fuchsia.io` namespace
protocol to the [`fuchsia.sys2.EventStream`][event-stream] protocol. Archivist receives [Event]s on
this protocol similarly to `LogConnectionListener`, retrieving attribution metadata from the
[ComponentDescriptor] sent by Component Manager along with the `LogSink` channel's handle. The
moniker included in the descriptor is constructed during [capability routing].
 -->
这会导致组件管理器将传入请求从默认的 `fuchsia.io` 命名空间协议重定向到 [`fuchsia.sys2.EventStream`][event-stream] 协议。归档器在此协议上接收[事件][Event]，类似于 `LogConnectionListener`（日志连接监听器），从 [ComponentDescriptor]（组件描述符）中检索组件管理器发送的归因元数据以及 `LogSink` 通道的句柄。描述符中包含的代称（moniker）是在[能力路由][capability routing]期间构造的。

<!--
Configuring a `capability_requested` event for `LogSink` does not affect capability
routing itself, only the delivery of the channel to the Archivist as an [Event] instead of as an
Open(). This means that the CML for passing the attributed `LogSink` remains the same for the rest
of the component topology.
 -->
为 `LogSink` 配置 `capability_requested`（能力请求）事件不会影响能力路由本身，只会将通道作为[事件][Event]而非 `Open()` 传递给归档器。这意味着用于传递归因 `LogSink` 的 CML 对于组件拓扑的其余部分保持不变。

<!--
For more information, see [Life of a protocol open] and the [events documentation][cm-events].
 -->
要获取更多信息，请参阅[协议打开的生命周期][Life of a protocol open]和[事件文档][cm-events]。

[Archivist]: /src/diagnostics/archivist/README.md
[Archivist's manifest]: /src/diagnostics/archivist/meta/archivist.cml
[CapabilityRequested]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#CapabilityRequestedPayload
[capability routing]: /concepts/components/v2/capabilities/life_of_a_protocol_open.md#the-open-triggers-capability-routing
[cm-events]: /concepts/components/v2/capabilities/event.md
[ComponentDescriptor]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#ComponentDescriptor
[Event]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Event
[event-stream]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#EventStream
[logsink-protocol]: /sdk/fidl/fuchsia.logger/logger.fidl
[Life of a protocol open]: /concepts/components/v2/capabilities/life_of_a_protocol_open.md