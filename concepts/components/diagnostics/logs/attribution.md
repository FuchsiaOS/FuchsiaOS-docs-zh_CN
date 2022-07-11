# Attributing LogSink connections

When a Fuchsia component wishes to write logs, it must obtain a connection to a
[`fuchsia.logger.LogSink`][logsink-protocol] in its environment, typically provided
by an instance of the [Archivist].

Typical Fuchsia service connections are anonymous such that the server and client have no
identifying information about each other. The client only sees the service in their namespace, e.g.
`/svc/fuchsia.logger.LogSink`, and the server sees an anonymous `Open()` request to their incoming
namespace.

At the same time, it's important to know from where logs come, as trustworthy provenance
metadata enables better monitoring, storage, querying, and presentation of logs. The system solves
this with a feature called "attributed logging" which identifies the source of an incoming `LogSink`
connection.

### appmgr: LogConnector

[Archivist] serves the [`fuchsia.sys.internal/LogConnectionListener`][listener-protocol] to receive
`LogSink` connections along with [`fuchsia.sys.internal/SourceIdentity`][source-identity] metadata.

#### Initializing `LogConnectionListener` for a realm

[Archivist] connects to [`fuchsia.sys.internal/LogConnector`][connector-protocol], which is provided
by [appmgr]. Archivist then calls `TakeLogConnectionListener()` to retrieve the server end of a
channel implementing [`fuchsia.sys.internal/LogConnectionListener`][listener-protocol] for the
realm where the connection was made. This behavior can be disabled by running Archivist with the
`--disable-log-connector` flag.

In production, [Archivist] runs "above" appmgr and connects via the `sys` realm, taking the
`LogConnectionListener` for appmgr's root realm and capturing all `LogSink` connections.

appmgr does not provide an attributed `LogSink` to realms where the caller has explicitly provided
its own entry for [`fuchsia.logger.LogSink`][logsink-protocol], allowing test environments to
intercept and read their own logs.

#### Making a LogSink connection

When appmgr launches a component, it instantiates a [ServiceProviderDirImpl][service-provider-dir],
populating it with services entries for the component's namespace. Each directory is created by
taking the services of the parent/enclosing environment and filtering them down to entries listed
in the component's `.cmx` file under `sandbox.services`.

If a component lists `fuchsia.logger.LogSink` in its manifest, its environment does not provide an
implementation, and appmgr has a `LogConnectionListener` initialized for the realm, an
["attributed `LogSink`"][log-connector] is provided in the component's namespace. From the
component's perspective, it behaves just as a normal `LogSink` instance. When a connection is made
to it, the sent channel is forwarded to the corresponding `LogConnectionListener` along with the
[`SourceIdentity`][source-identity].

### Component Manager: CapabilityRequested events

[Archivist's manifest] `expose`s `fuchsia.logger.LogSink` just like other service capabilities, but
it also `use`s an event from the framework, binding it to a service in its namespace:

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

This causes Component Manager to redirect incoming requests from the default `fuchsia.io` namespace
protocol to the [`fuchsia.sys2.EventStream`][event-stream] protocol. Archivist receives [Event]s on
this protocol similarly to `LogConnectionListener`, retrieving attribution metadata from the
[ComponentDescriptor] sent by Component Manager along with the `LogSink` channel's handle. The
moniker included in the descriptor is constructed during [capability routing].

Configuring a `capability_requested` event for `LogSink` does not affect capability
routing itself, only the delivery of the channel to the Archivist as an [Event] instead of as an
Open(). This means that the CML for passing the attributed `LogSink` remains the same for the rest
of the component topology.

For more information, see [Life of a protocol open] and the [events documentation][cm-events].

[appmgr]: /src/sys/appmgr/README.md
[Archivist]: /src/diagnostics/archivist/README.md
[Archivist's manifest]: /src/diagnostics/archivist/meta/archivist.cml
[CapabilityRequested]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#CapabilityRequestedPayload
[capability routing]: /concepts/components/v2/capabilities/life_of_a_protocol_open.md#the-open-triggers-capability-routing
[cm-events]: /concepts/components/v2/capabilities/event.md
[ComponentDescriptor]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#ComponentDescriptor
[connector-protocol]: /sdk/fidl/fuchsia.sys.internal/log_connector.fidl
[Event]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Event
[event-stream]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#EventStream
[listener-protocol]: /sdk/fidl/fuchsia.sys.internal/log_connector.fidl
[log-connector]: /src/sys/appmgr/log_connector_impl.h
[logsink-protocol]: /sdk/fidl/fuchsia.logger/logger.fidl
[Life of a protocol open]: /concepts/components/v2/capabilities/life_of_a_protocol_open.md
[service-provider-dir]: /src/sys/appmgr/log_connector_impl.h
[source-identity]: /sdk/fidl/fuchsia.sys.internal/source_identity.fidl
