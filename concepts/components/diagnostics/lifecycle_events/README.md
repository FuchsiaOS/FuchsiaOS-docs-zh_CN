# Lifecycle events

The [Archivist][archivist] consumes lifecycle events to ingest diagnostics data. Additionally, it provides an
interface to read those lifecycle events for diagnostics purposes. This document explains what
these events are and through which interface they can be accessed for diagnostics.

## Archivist consumption of lifecycle events {#archivist-consumption}

The archivist ingests events from the component framework.
The following diagram shows a very high level overview of the three lifecycle events
(started, directory_ready and stopped) the archivist is interested in:

- {appmgr}

  ![Figure: Flow of lifecycle events under appmgr](appmgr_lifecycle_flow.png)

  The archivist consumes the following lifecycle events under appmgr through
  [`fuchsia.sys.internal.ComponentEventProvider`][component_event_provider]:

  - **Started**: Sent by appmgr when a component starts, the [runner][runner] might still need to
    launch the component. This event is synthesized for all components that exist at the moment the
    archivist starts listening for events.
  - **Stopped**: Sent by appmgr when a component stops. The runner might still need to tear down the
    component, but the component is gone from the framework perspective.
  - **Diagnostics ready**: Sent by appmgr when a component's `out/diagnostics` directory is being
    served by the component.


- {component manager}

  ![Figure: Flow of lifecycle events under component manager](component_manager_lifecycle_flow.png)

  The archivist consumes the following lifecycle events under component manager through
  [`fuchsia.sys2.EventSource`][event_source]:

  - **Started**: Sent by component manager when a component starts, the [runner][runner] might still
    need to launch the component, but the component has started from the framework perspective.
  - **Stopped**: Sent by component manager when a component stops, the runner might still need to to
    tear down the component, but the component is gone from the framework perspective.
  - **Running**: Sent by component manager for all components that are running at the moment the
    archivist starts listening for events. In other words, a synthesized started event. This event
    is provided to the reader as **Started**, but consumed from the framework as “Running”.
  - **Directory ready**: The archivist listens for directory ready of the `out/diagnostics`
    directory. When the component starts serving this directory, the component manager sends this
    event to the Archivist.


## Reading lifecycle events

Lifecycle events can be read through the ArchiveAccessor. Only the `snapshot` mode is supported.

<!-- TODO(fxbug.dev/60763): link to ArchiveAccessor documentation where each mode is explained -->

Results are returned as a `vector<FormattedContent>` with each entry's variant matching the
requested `Format`, although JSON is the only supported format.


### JSON object contents

Each JSON object in the array is one event entry. Like other data types in ArchiveAccessor,
each object consists of several fields, although the contents of metadata and payload differ
from other sources. The following is an example of a JSON object entry:

```
{
    "version": 1,
    "moniker": "core/network/netstack",
    "data_source": "LifecycleEvent",
    "metadata": {
        "timestamp": 1234567890,
        "lifecycle_event_type": "Started",
        "component_url": "fuchsia-pkg://fuchsia.com/network#meta/netstack.cm",
        “errors”: []
    },
    "payload": null,
}

```

#### Monikers

Monikers identify the component related to the triggered event.

As explained in [Archivist consumption of lifecycle events](#archivist-consumption), there are two
systems that provide the archivist with events, appmgr and component manager:

-   Components running under appmgr have a `.cmx` extension
-   Components running under component manager have a `.cm` extension

#### Timestamp

The time is recorded using the kernel's monotonic clock (nanoseconds) and conveyed without
modification as an unsigned integer. The time is when the event was created by the component
manager and appmgr, which also provide the time.

#### Lifecycle event type

These are the valid values for lifecycle event types:

- DiagnosticsReady (diagnostics data for the component
is ready).
- Started (the component has started)
- Stopped (the component has stopped)
- LogSinkConnected (a client at some point connected to the LogSink protocol).

#### Component URL

The URL with which the component related to this event was launched.

#### Errors

Optional vector of errors encountered by the platform when handling this event.
Usually, no errors are expected for lifecycle events, so in most cases this is empty.


#### Payload

The payload is always be empty for lifecycle events. Other types of data sources, like logs and
inspect, contain a payload. For more information, refer to the
[ArchiveAccessor documentation][archive_accessor].


## Related docs

- [Event capabilities][event_capabilities]
- [Inspect discovery and hosting - Archivist section][inspect_discovery_hosting]


[archivist]: reference/diagnostics/inspect/tree.md#archivist
[event_source]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#EventSource
[component_event_provider]: https://fuchsia.dev/reference/fidl/fuchsia.sys.internal#ComponentEventProvider
[event_capabilities]: concepts/components/v2/capabilities/event.md
[inspect_discovery_hosting]: reference/diagnostics/inspect/tree.md#archivist
[component_runner]: glossary#runner
