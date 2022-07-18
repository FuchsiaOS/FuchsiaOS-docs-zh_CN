<!-- 
# Attributing LogSink connections
 -->
# 添加 LogSink 连接属性

<!-- 
When a Fuchsia component wishes to write logs, it must obtain a connection to a
[`fuchsia.logger.LogSink`][logsink-protocol] in its environment, typically provided
by an instance of the [Archivist].

Typical Fuchsia service connections are anonymous such that the server and client have no
identifying information about each other. The client only sees the service in their namespace, e.g.
`/svc/fuchsia.logger.LogSink`, and the server sees an anonymous `Open()` request to their incoming
namespace.
 -->
当 Fuchsia 组件希望撰写日志时，它必须获得对其环境中[`fuchsia.logger.LogSink`][logsink-protocol]的连接，通常由[归档器][Archivist]实例提供。

典型的 Fuchsia 服务连接是匿名的，因此服务器和客户端没有对于彼此的识别信息。客户端只是在其命名空间看到服务，例如 `/svc/fuchsia.logger.LogSink`，而服务器看到的是对其传入命名空间的匿名 `Open()` 请求。

<!-- 
At the same time, it's important to know from where logs come, as trustworthy provenance
metadata enables better monitoring, storage, querying, and presentation of logs. The system solves
this with a feature called "attributed logging" which identifies the source of an incoming `LogSink`
connection.
 -->
与此同时，了解日志的来源也很重要，因为可信来源的元数据带来更好的监控、存储、查询和日志呈现。系统通过名为“附带属性记录（attributed logging）”的特性来解决这个问题，这一特性能够识别 `LogSink`（“日志槽”）传入连接的来源。

<!-- 
### appmgr: LogConnector
 -->
### appmgr：日志连接器（LogConnector）

<!-- 
[Archivist] serves the [`fuchsia.sys.internal/LogConnectionListener`][listener-protocol] to receive
`LogSink` connections along with [`fuchsia.sys.internal/SourceIdentity`][source-identity] metadata.
 -->
[归档器][Archivist]服务 [`fuchsia.sys.internal/LogConnectionListener`][listener-protocol] 以接收 `LogSink` 连接和 [`fuchsia.sys.internal/SourceIdentity`][source-identity] 元数据。

<!-- 
#### Initializing `LogConnectionListener` for a realm
 -->
#### 为界初始化 `LogConnectionListener`

<!-- 
[Archivist] connects to [`fuchsia.sys.internal/LogConnector`][connector-protocol], which is provided
by [appmgr]. Archivist then calls `TakeLogConnectionListener()` to retrieve the server end of a
channel implementing [`fuchsia.sys.internal/LogConnectionListener`][listener-protocol] for the
realm where the connection was made. This behavior can be disabled by running Archivist with the
`--disable-log-connector` flag.

In production, [Archivist] runs "above" appmgr and connects via the `sys` realm, taking the
`LogConnectionListener` for appmgr's root realm and capturing all `LogSink` connections.
 -->
[归档器][Archivist]连接至由 [appmgr] 提供的 [`fuchsia.sys.internal/LogConnector`][connector-protocol]。归档器接下来调用 `TakeLogConnectionListener()` 检索通道的服务器端，该通道为建立连接的界（realm）实现了 [`fuchsia.sys.internal/LogConnectionListener`][listener-protocol]。该行为可通过带 `--disable-log-connector` 标记运行归档器禁用。

在生产中，[归档器]在 appmgr “之上”运行，并且通过 `sys` 界连接，针对 appmgr 的根界采用 `LogConnectionListener`（日志连接侦听器），并捕捉所有 `LogSink` 连接。

<!-- 
appmgr does not provide an attributed `LogSink` to realms where the caller has explicitly provided
its own entry for [`fuchsia.logger.LogSink`][logsink-protocol], allowing test environments to
intercept and read their own logs.
 -->
如果界的调用者已经显式地为 [`fuchsia.logger.LogSink`][logsink-protocol] 提供了记录（entry），允许测试环境拦截和读取其自身的日志，那么 appmgr 不会向该界提供事先添加的 `LogSink` 属性。

<!-- 
#### Making a LogSink connection
 -->
#### 建立日志槽连接

<!-- 
When appmgr launches a component, it instantiates a [ServiceProviderDirImpl][service-provider-dir],
populating it with services entries for the component's namespace. Each directory is created by
taking the services of the parent/enclosing environment and filtering them down to entries listed
in the component's `.cmx` file under `sandbox.services`.
 -->
当 appmgr 启动一个组件时，它会实例化一个 [ServiceProviderDirImpl][service-provider-dir]，用组件命名空间进行填充。每个目录在创建前，需要先获取父/封闭环境的服务，并将它们过滤到在 `sandbox.services` 下组件的 `.cmx` 文件中列出的条目中。

<!-- 
If a component lists `fuchsia.logger.LogSink` in its manifest, its environment does not provide an
implementation, and appmgr has a `LogConnectionListener` initialized for the realm, an
["attributed `LogSink`"][log-connector] is provided in the component's namespace. From the
component's perspective, it behaves just as a normal `LogSink` instance. When a connection is made
to it, the sent channel is forwarded to the corresponding `LogConnectionListener` along with the
[`SourceIdentity`][source-identity].
 -->
如果组件在其清单中列出了 `fuchsia.logger.LogSink`，那么其环境不提供实现，appmgr 为该界初始化一个 `LogConnectionListener`，在该组件的命名空间中提供["事先添加的 `LogSink` 属性"][log-connector]从组件的角度来看，它的行为如同普通的 `LogSink` 实例。当与它建立连接时，发送的通道和 [`SourceIdentity`][source-identity]（源身份）一同转发至相应的 `LogConnectionListener`。

<!-- 
### Component Manager: CapabilityRequested events
 -->
### 组件管理器：CapabilityRequested（功能需求）事件

<!-- 
[Archivist's manifest] `expose`s `fuchsia.logger.LogSink` just like other service capabilities, but
it also `use`s an event from the framework, binding it to a service in its namespace:
 -->
[归档器的清单][Archivist's manifest]同其他服务功能一样公开（`expose`） `fuchsia.logger.LogSink`，但它也使用（`use`）来自框架的事件，将其绑定至其命名空间的服务：


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
这导致组件管理器（Component Manager）将传入请求从默认的 `fuchsia.io` 命名空间协议重定向至 [`fuchsia.sys2.EventStream`][event-stream] 协议。归档器类似于 `LogConnectionListener` 接收该协议上的[事件][Event]，从由组件管理器发送的 [ComponentDescriptor] 检索属性元数据和 `LogSink` 通道的句柄。描述符（descriptor）中包含的昵称（moniker）是在[功能路由][capability routing]（capability routing）时建构的。

<!-- 
Configuring a `capability_requested` event for `LogSink` does not affect capability
routing itself, only the delivery of the channel to the Archivist as an [Event] instead of as an
Open(). This means that the CML for passing the attributed `LogSink` remains the same for the rest
of the component topology.

For more information, see [Life of a protocol open] and the [events documentation][cm-events].
 -->
为 `LogSink` 配置 `capability_requested`（功能需求）事件并不影响功能路由本身，只是将通道作为[事件][Event]代替 Open() 传递给归档器。这意味着用于传递事先添加的 `LogSink` 属性的 CML 将对于组件拓扑的剩余部分保持不变。

要获取更多信息，请参阅 [Life of a protocol open][Life of a protocol open] 和 [cm-事件][cm-events]。

[appmgr]: /src/sys/appmgr/README.md
[Archivist]: /src/diagnostics/archivist/README.md
[Archivist's manifest]: /src/diagnostics/archivist/meta/archivist.cml
[CapabilityRequested]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#CapabilityRequestedPayload
[capability routing]: /docs/concepts/components/v2/capabilities/life_of_a_protocol_open.md#the-open-triggers-capability-routing
[cm-events]: /docs/concepts/components/v2/capabilities/event.md
[ComponentDescriptor]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#ComponentDescriptor
[connector-protocol]: /sdk/fidl/fuchsia.sys.internal/log_connector.fidl
[Event]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Event
[event-stream]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#EventStream
[listener-protocol]: /sdk/fidl/fuchsia.sys.internal/log_connector.fidl
[log-connector]: /src/sys/appmgr/log_connector_impl.h
[logsink-protocol]: /sdk/fidl/fuchsia.logger/logger.fidl
[Life of a protocol open]: /docs/concepts/components/v2/capabilities/life_of_a_protocol_open.md
[service-provider-dir]: /src/sys/appmgr/log_connector_impl.h
[source-identity]: /sdk/fidl/fuchsia.sys.internal/source_identity.fidl
