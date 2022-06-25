<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0121" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This document captures design deliberations, principles, and decisions made
regarding component events: concepts, manifest syntax and FIDL protocols.

Many of the decisions described below were made and implemented in early 2020.
Some other concepts are proposed which tackle shortcomings and complexity in
earlier decisions. Many features are proposed to be allowlisted to prevent
their usage outside of existing use cases while we work to migrate them to
better mechanisms.

The scope of this doc covers the following Component Framework APIs:

* CML: the language used to write [component manifests][manifest-docs]
* `fuchsia.sys2/events.fidl`: the current API surface where Component Events
  FIDL APIs live. The goal is that all event APIs upgrade to
  `fuchsia.component/events.fidl` and make them available in the SDK.

## Motivation

Component manager handles lifecycle of components internally and doesn't expose
that information to components. Some privileged clients (for example:
diagnostics, tests, component supervisor, debuggers) need a bit more insight
into lifecycle of components to perform their work.

Component events were introduced as a solution for exposing such information to
these privileged components. All these events are dispatched by component
manager when a lifecycle transition of a component instance happens. The
capability routing API was left open in some places that are called out in
this document for further design that allow components to expose and offer
custom events themselves.

## Design

This section outlines the current design of Component Events together with API
revisions, highlights where the current design is not intended as the long-term
design and calls out future features that could be designed and implemented.

### Component event stream capabilities

Component events are modeled as "event stream" capabilities. Each event
stream capability refers to a single component or a sub-tree of components
in the topology. At the time of writing this document, events are treated as
individual event capabilities (not event streams). Component manager
emits events on behalf of the components when a lifecycle transition
occurs.

Like any capability, event streams can be routed. When an event stream is:

- Exposed/Offered from framework: the event refers to the component that is
  exposing or offering the event.

- Used from parent/child: a component can listen for events that were routed to
  it from a child or from the parent. The parent of a component can directly
  use the lifecycle event streams from a child without the child explicitly
  exposing the event from framework.


#### Event streams from above root

Event streams can be used/offered by the root realm from "parent". You
might be wondering, what is the parent of the root realm if it's supposed to
be the root of the topology? In component manager terminology, capabilities
offered by component manager to the root realm are said to come from
"above root".

Event streams offered from "above root" to the root realm are scoped to the
whole component instance topology. These event streams can be downscoped to
refer to a subtree of the topology when they are routed. This can be seen
similar to how directory capabilities can be routed as a whole or as
subdirectories (`subdir` key in the routing declarations of directories).
The only way to listen for events scoped to a whole realm is to have a
downscoped event stream capability originating from the root available. Events
covering a whole realm tree are privileged/sensitive and break encapsulation
boundaries, therefore, we explicitly route them from above root providing
access control through static routing.

Consider the following example:

![A visual tree representation of the declarations shown below][example-img]

```json5
// root.cml
{
  offer: [
    {
      event_stream: "started",
      from: "parent",
      to: "#core",
      scope: "#core"
    },
  ]
}

// core.cml
{
  offer: [
    {
      event_stream: "started",
      from: "parent",
      to: "#test_manager",
      scope: "#test_manager"
    },
  ]
}

// test_manager.cml
{
  offer: [
    {
      event_stream: "started",
      from: "parent",
      to: [ "#archivist", "#tests" ]
      scope: "#tests",
    }
  ]
}

// tests.cml
{
  offer: [
    {
      event_stream: "started",
      from: "parent",
      to: [ "#test-12345" ]
      scope: "#test-12345",
    }
  ]
}

// test-12345.cml
{
  offer: [
    {
      event_stream: "started",
      from: "framework",
      scope: "#bar",
      to: "#foo"
    }
  ],
  use: [
    {
      event_stream: "started",
      from: "parent"
    },
    {
      event_stream: "stopped",
      from: "framework",
      scope: "#bar",
    }
  ]
}

// foo.cml
{
  use: [
    {
      event_stream: "started",
      from: "parent"
    }
  ]
}

// archivist.cml
{
  use: [
    {
      event_stream: "started",
      from: "parent",
    }
  ]
}
```

In this example:

- `foo` can get `started` events for `bar` given that `test-12345` routes it to
  it.

- `archivist` can get `started` events for all components under `tests`
  given that it was offered `started` from `test_manager` that got it from
  `core` that got it from `root` that got it from `above root` and was
  downscoped on the way.

- `test-12345`, the test root, can get started events about all components under
  it, for the same reasons that `archivist` can. However, unlike the archivist
  (which can get all events from `tests`), it can only get events about
  the test, given that `tests` downscoped the event for `test-12345`.

This example shows one of the core use cases of events at the moment. The
archivist is able to observe what happens inside each test in an isolated
manner. Also, each test can get events about all components under the test or
about a specific one.


#### Merging event streams

Event streams of the same type can be merged as a single stream.
For example, in the example above, `#test-12345` could offer the `stopped` from
`foo` and `bar` to some other component as a single capability. That component
would then get the `stopped` event for both `foo` and `bar`.

```json5
// core.cml
offer: [
  {
    event_stream: "stopped",
    from: [ "#netstack", "#supervisor" ],
    to: "#someone",
  }
]
```


The ability to expose/offer an event from self is not allowed for now. However,
there's room for growth here to allow events to expose/offer custom events they
dispatch themselves.

#### Event modes

At the time of writing this document, events can be ingested in an asynchronous
or a synchronous way. The intent is to have *only* async events and deprecate
sync events entirely.

Consuming an event synchronously allows the subscriber to block component
manager while it handles the event and then resume. This was introduced for
testing initially and also with debuggers in mind.

Since then, we have learned that tests using sync events can be written using
async events in general. Consequently, there's been
[work](https://fuchsia-review.googlesource.com/q/485617+OR+485637+OR+488997+OR+489800)
towards eliminating sync events entirely but a few uses remain in component
manager internal tests. It's believed that sync events will be useful
in a debugger (such as step or zxdb), but at that point we'll invent a solution
that accurately fits the needs of a debugger.

The proposal is to allowlist tests that use sync events and work towards
eliminating sync events entirely.

#### Event Types

At the time of writing this document, we have two classes of events:

- Lifecycle events. These events reflect changes in the
  [lifecycle][component-lifecycle-doc] of a component instance and are emitted
  by component manager which manages such information.
- Deprecated events. These events do not reflect changes in the lifecycle of
  a component instance and we are working to remove them entirely in favor of
  more appropriate solutions.

We have the following lifecycle event types:

- `Discovered`: This is the first stage in the lifecycle of components.
  Dispatched for dynamic children when they're created, for static children
  when their parent is resolved, and for the root when the component manager
  starts.
- `Resolved`: An instance's declaration was resolved successfully for the first
  time.
- `Started`: This instance has started, according to component manager.
  However, if this is an executable component, the runner has further work to
  do to launch the component. The component is starting to run, but
  may not have started executing code.
- `Stopped`: An instance was stopped successfully. This event must occur before
  Destroyed.
- `Destroyed`: Destruction of an instance has begun. The instance has been
  stopped by this point. The instance still exists in the parent's realm but
  will soon be removed.
- `Purged`: An instance was destroyed successfully. The instance is stopped
  and no longer exists in the parent's realm.

And the following deprecated event types:

- `Running`: This event is **synthesized** by component manager for all
  components that are already running at the moment of subscription. This event
  is derived from `started`, but for components that started (and didn't stop)
  before the listener subscribed for events. Eventually, the desire is to have
  a Component Framework Query API to understand what is running. Therefore, the
  plan is to allowlist this event to its only client, the Archivist, and at a
  future time use the new API and remove `running`.

- `Capability Routed`: This event was introduced for usage in tests. It has
  [recently been removed](https://fuchsia-review.googlesource.com/q/topic:capability-routed-remove)
  entirely from tests using it and we are in the process of completely removing
  it.


- `Capability Requested`: This event was introduced as a temporary solution to
  provide component attribution to `fuchsia.logger/LogSink` connections. Since
  then it's also been used to provide attribution to
  `fuchsia.debugdata/Publisher` connections. This was never meant to be a long
  term solution. Since the events system was for privileged components only,
  building this feature with Component Events was a low-commitment approach.
  It was understood that, if the use cases grew, the time will come to invent a
  more standardized solution. The current plan is to work to remove this
  event entirely and in the meantime allowlist it to its only clients:
  Archivist, debug data and test manager (for debug data).


- `Directory Ready`: This event was introduced as a solution to provide the
  `out/diagnostics` directory exposed by components to the Archivist for
  inspect data aggregation. The Diagnostics team has plans to design VMO backed
  logs. Given that logs need to be available before the component starts
  serving the diagnostics directory, this approach becomes obsolete. A new
  solution for providing the inspect and logs VMO to the Archivist will be
  designed that guarantees that logs are available even before the component
  starts its async loop. The proposal for now is to allowlist this event to the
  only user it has (the Archivist) and work towards removing it entirely.

#### Routing CML syntax

##### Use {#use}

```json5
{
    use: [
        {
            event_stream: [
                "running",
                "started",
                "stopped",
            ],
            from: "parent",
            mode: "async",
            path: "/my_stream"
        },
    ]
}
```

The use declaration contains:

- `event_stream`: a single event name or a list of event names.
- `from`: the source of the capability. Allowed values: parent or a child
  reference. Using an event from framework or self is not allowed.
- `path`: The path where the event stream will be served. This is optional. When
  given the incoming namespace of the component will contain a service file with
  the given name for a `fuchsia.component.EventStream` that component manager
  serves (see [Use](#use)). When not provided, the component can use
  `EventSource` to start consuming events at a specific point.
- `scope`: when an event is used from framework, the scope is required to
  specify the child (or array of children) which the event will be about. When
  the event is used from parent, the `scope` can be used to refer downscope the
  event to a certain child scope, otherwise the event will carry the scope
  coming from the parent.
- `mode`: defaults to `async`. As mentioned earlier, the only event mode will
  be async. Therefore only allowlisted tests will be permitted to use mode
  "sync" until modes are completely eliminated. This field will eventually go
  away entirely.
- `filter`: this is currently used only for `DiagnosticsReady` and for
  `CapabilityRequested`. As mentioned earlier, these events are being removed
  entirely so no details are provided about filters as they won't be relevant
  to a non-diagnostics developer.

It's also possible to use an event coming from different sources. In the
following example, a component will get a single event stream with `started` and
`stopped` events scoped as it was offered by its parent, as well as a `started`
when its child `#child` starts.

```json5
use: [
  {
    event_stream: [
      {
        name: [ "started", "stopped ],
        from: "parent",
      },
      {
        name: "started",
        from: "framework",
        scope: "#child",
      }
    ]
  }
]
```

##### Offer

```json5
{
    offer: [
        {
            event_stream: "started",
            from: [ "#child_a", "parent", ],
            to: "#archivist",
            as: "started_foo"
        },
    ]
}
```

The offer declaration contains:

- `event_stream`: a single event name or a list of event names.
- `scope`: when an event is offered from framework, the scope allows one to
  define the child (or an array of children) of which the event is about. If
  no scope is specified, the scope is self, meaning the event is about the
  component itself. When an event is offered from parent, the scope allows to
  downscope the event to a child scope.
- `from`: one or multiple sources of the capability. When multiple sources are
  given the streams are considered to be merged. Allowed values: framework,
  parent or a child reference. Offering from self is not allowed.
- `to`: the child reference to which the capability is offered.
- `as`: the target name of the capability (rename). This can only be provided
  when a single event stream name is given.

##### Expose

```json5
{
    expose: [
        {
            event_stream: "started",
            from: "framework",
            scope: [ "#child", "#other_child" ],
            as: "foo_started"
        },
    ]
}
```

The expose declaration contains:

- `event_stream`: a single event name or a list of event names.
- `scope`: when an event is exposed from `framework`, the scope is required and
  allows one to define the child (or array of children) which the event is
  about.
- `from`: one more more sources of the capability. When multiple sources are
  given the streams are considered to be merged. Allowed values: parent or
  a child reference. Exposing an event from self is not allowed.
- `as`: the target name of the capability (rename). This can only be provided
  when a single event stream name is given.

### The `EventSource` protocol

`EventSource` is the protocol that allows component instances to subscribe to
event streams. It's a builtin capability any component can use from `framework`
to consume events that are explicitly routed to them.

### Static event streams {#static-event-streams}

Static event streams are a way of acquiring events through a protocol in the
incoming `/events` directory. Unlike runtime subscription through
`EventSource.Subscribe`, events can buffer and trigger a
[startup](http://fxrev.dev/535167) of the component when they arrive.

Components declare they `use` an event stream at some path and component manager
will create an entry in their incoming namespace at the given path which is a
`fuchsia.component.EventStream` served by component manager (see
[Use section](#use)).

Events are buffered internally until a `GetNext` call is performed. The size of
this buffer will be decided during implementation (it could be a large buffer or
the maximum size of the next batch or something else) and will be clearly
documented in the FIDL API and events documentation. If the number of buffered
events exceeds the defined amount (due to a consumer being too slow) component
manager will drop events and close the connection. The component can then
reconnect when it's ready to receive events. This is a workaround for lack of
flow control in Zircon. We want to prevent channel overflows which wouldn't be
possible if we were to invert who serves the `EventStream` protocol (the
component instead of component manager) and we don't want to use excessive
amount of memory in component manager. A well-behaved client which consumes
events at a steady rate won't need to worry about missing events. Clients
consuming events too slowly will be notified of dropped events. This will
happen through an event in the stream from which they are consuming. A couple
alternatives to perform this notification could be:

- An event of the same type with an error payload indicating how many events of
  that type were dropped.
- A special field under `fuchsia.component.EventErrorPayload`: `dropped`. This
  field would contain an `EventType` and a number indicating the number of
  events of that type that were dropped.
- A FIDL event in the protocol, that the client receives when events are
  dropped specifying how many were dropped and of what type. This alternative
  can introduce another DoS vector though, as the channel could fill with
  events.

### FIDL APIs

At the time of writing this document, the protocols for consuming events are
defined in `fuchsia.sys2/events.fidl`. This proposal introduces a few changes
with the goal of upgrading the protocols and structures to `fuchsia.component`
and make them available in the SDK:

#### Component manager serves the `EventStream` instead of the component

This implies changing `EventSource#Subscribe` to take a request:

```fidl
[Discoverable]
protocol EventSource {
    Subscribe(vector<EventSubscription> events, request<EventStream> stream)
        -> () error fuchsia.component.Error;
};
```

#### `EventStream.GetNext` provides a batch of events

Since component manager now serves the protocol and batches events, as discussed
in a previous section, the new stream protocol returns a batch of events on
`GetNext` rather than a single event.

```fidl
protocol EventStream {
    GetNext() -> vec<Event>;
};
```

#### `TakeStaticEventStream` is dropped

The method `EventSource.TakeStaticEventStream` is removed in favor of placing
the event stream directly in the incoming namespace as discussed in the previous
section.

#### Stricter `Event` and `EventResult`

At the time of writing this document, all these data types are tables. However,
their data is very well defined and we don't expect to add more to it. To
improve ergonomics by removing handling of optional fields, we'll make them
`struct`s and non flexible unions:

```fidl
struct Event {
    EventHeader header;
    EventResult event_result;
};


union EventResult {
    1: EventPayload payload;
    2: EventError error;
};
```

## Implementation

Most of this design is already implemented. A few bits remain that need to be
implemented before we expose this API in the SDK. In particular:

- Allowlisting of events: `directory ready`, `running` and
  `capability requested`.
- Placing a static event stream in the incoming namespace and removing the
  `EventStream.TakeStaticEventStream` method.
- CML updates since today we route `events` and we want to route `event streams`
  which can be merged in routing.
- Offer events about the whole topology from "above root" to the root and
  implement the `scope` keyword in event routing declarations to allow to
  have events from a subtree in the topology.
- Expose events. At the time of writing this document, we only support `offer`
  and `use`.
- At origin, events now refer to a single component except for the events that
  are offered to the root realm from "above root". When routed, events can be
  aggregated so they could refer to more than one component.

## Performance

The event system builds on top of the existing internal hook system that
component manager implements, therefore the performance implications of
dispatching the events to other components interested in them are negligible.
In fact, there are improvements in performance since now if a component is
interested in only events from a single component and not a whole subtree, now
it's possible to dispatch only events for that component instead of the whole
subtree reducing the number of syscalls involved.

## Ergonomics

This RFC introduces improvements in ergonomics of events:

- If a component is interested in a single component event, now that can be
  expressed in CML without requiring code to filter them.
- All components now have the `EventSource` protocol available as a builtin
  framework capability and they don't require routing it explicitly.
- There's a single capability that is routed `event stream` instead of routing
  `events` and consuming `event streams` as today.
- `Event streams` can be merged when routing, reducing the amount of routing
  statements developers need to write in CML.

## Backwards compatibility

This change does not break compatibility. There'll be in-tree trivial
refactors at the component manager level (and root, core, bootstrap realms)
but not in client components. Additionally, there'll be refactors in tests that
use events today (all in-tree), since events will be about a single component
now.

## Security and privacy considerations

This proposal maintains security properties discussed in a security and privacy
review in 2020. Previously the `EventSource` protocol was explicitly routed
since events were about a whole subtree. Now this protocol is a builtin
capability and events are about specific components. Events about a whole
subtree are explicitly routed and static verification can be performed to
ensure non-privileged components aren't getting these events.

## Testing

All event features and semantics must have integration testing in component
manager.

## Documentation

Updates to documentation must be made in the [Event Capabilities][doc-events]
and [Component Manifests][manifest-docs] pages.

## Drawbacks, alternatives, and unknowns

### Offer an `EventSink` capability from a component to framework

Under this idea, a component would expose a protocol `RealmEventSink` to
framework. The framework would then open this capability and push events into
it as they occur. This proposal lacks a way of defining from what scope the
component will get the events and there's no clear path to restrict such a scope
or be able to statically verify that a component is not getting events from a
part of the topology from which it should have no visibility. Under the current
design, it's statically possible to express getting events about a specific set
of components (for regular use cases) or a whole subtree in the topology (for
diagnostics use cases in production and tests).

### Source of self for expose and offer routing declarations

`expose/offer` event from `self` is left as an open area for future design work
of custom events that a component might expose and dispatch itself.

## Prior art and references

Documentation about the current state of events can be found in
the [Event capabilities][doc-events] and in the
[Events FIDL definitions][sys2-events-fidl].

[component-lifecycle-doc]: concepts/components/v2/lifecycle.md
[doc-events]: concepts/components/v2/capabilities/event.md
[example-img]: resources/0121_component_events/routing_example.png
[manifest-docs]: concepts/components/v2/component_manifests.md
[sys2-events-fidl]: /sdk/fidl/fuchsia.sys2/events.fidl
