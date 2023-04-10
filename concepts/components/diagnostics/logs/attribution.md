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