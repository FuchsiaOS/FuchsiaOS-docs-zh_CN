# Event capabilities (Components v2) {#event-capabilities}

<<../../_v2_banner.md>>

Event capabilities allow components receive or offer events under the scope of a
particular realm.

Components that wish to listen for events should have routed to them:

-   [`fuchsia.sys2.EventSource`][event-source]: allows the
    component to listen for events. Component manager will wait
    for the component to handle the event. This is used for
    [hermetic tests][hermetic-tests].

At the moment, events can only originate from the framework itself and are
limited to lifecycle events. Refer to [`fuchsia.sys2.EventType`][event-type] for
the complete list of supported events and their explanations.

## Event filters {#event-filters}

Most event declarations consist of only the event name. However, some of them
may contain filters. Event filters support filtering events based on additional
parameters defined in a key-value mapping.

These filters can be routed as subsets. For example, let's say component A
offers an event `foo` with filters `x: [/a, /b, /c]`. A component B might route
this event using only a subset of filters `x: [/b, /c]` and a component C could
use this event using a single filter `x: /b`.

For example, the `capability_ready` event defines a filter for the `path`. The
`path` is one or more paths exposed to framework that the component is
interested in offering or listening to.

## Static Event Streams {#event-streams}

Event subscriptions can be set up statically in CML files via static event streams.
Static event streams are similar to dynamically created event streams but are set
up during the resolution of a component's manifest. The following is an example of the
syntax of a static event stream.

```json5
{
    event: "resolved",
    modes: [ "sync" ],
    from: "framework",
},
{
    event_stream: "StartComponentTree",
    subscriptions: [
        {
            event: "resolved",
            mode: "sync",
        }
    ],
},
```

In the syntax above, the component synchronously listens for `resolved` events of child
components. This blocks children from starting, giving components an opportunity to set
up dynamic event streams prior to starting child components so that no events are missed.

`StartComponentTree` is the conventional name to use for blocking child components and used
by client libraries via a `start_component_tree` API.

## Offering events {#offering-events}

Events may be [offered][routing-terminology] to children. For example, a
component wishing to expose `started`, `stopped` and `capability_ready` to a
child of itself could do the following:

```
{
    offer: [
        {
            event: [
                "started",
                "stopped",
            ],
            from: "parent",
            to: [ "#child" ],
        },
        {
            event: "capability_ready",
            from: "parent",
            as: "foo_bar_ready",
            filter: { path: [ "/foo", "/bar"] },
            to: [ "#child" ],
        }
    ]
}
```

Events can be offered from two sources:

-   `parent`: A component that was offered an event (`started` for example) can
    offer this same event from its parent. The scope of the offered event will
    be the same scope of the `started` event that the component was offered.

-   `framework`: A component can also offer an event that its parent didn't
    offer to it. The scope of this event will be the component's realm itself
    and all its descendants.

## Using events {#using-events}

A component that wants to receive events declares in its manifest the events it
is interested in and the `EventSource` protocol. Both the protocol and the
events should be offered to the component.

Events can come from two sources:

-   `framework`: events used from framework are scoped to the component using
    them. For example, given a topology `A -> B -> C` where `A` is the parent of
    `B` and `B` of `C`. Suppose that `B` uses `started` from `framework`. `B`
    will be able to see when `C` starts but it won't be able to see when a
    sibling of itself (another child of `A`) starts.

-   `parent`: events used from the parent have been offered by the parent and
    are scoped to the parent's scope. For example, given a topology `A -> B ->
    C` where `A` is the parent of `B` and `B` of `C`. Suppose that `A` offers
    `started` to `B` and `B` uses `started` from `parent`. `B` will be able to
    see when `C` starts but it will also be able to see when a sibling of itself
    (a child of `A`) starts.

For example, a component that was offered the events from the
[example above](#offering-events) could use some of them as follows:

```
{
    use: [
        {
            protocol: "fuchsia.sys2.EventSource",
            from: "parent",
        },
        {
            event: "started",
            from: "parent",
        },
        {
            event: ["stopped", "destroyed"],
            from: "framework"
        }
        {
            event: "foo_bar_ready",
            from: "parent",
            filter: { path: "/foo" },
        }
    ]
}
```

Above, the component was offered `started`, `stopped` and `foo_bar_ready`. In
this example, the component uses the `started` it was offered and
`foo_bar_ready` but only for `/foo` capabilities, not `/bar`. Also, the
component decided to not use the `stopped` event it was offered. Instead the
component used the event from `framework`, which means that it will only see
`stopped` and `destroyed` events for components in its own realm.

[hermetic-tests]: ../opaque_test.md
[event-source]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#EventSource
[event-source]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#EventSource
[event-type]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#EventType
[routing-terminology]: ../component_manifests.md#routing-terminology
