# Event capabilities

<<../../_v2_banner.md>>

Event capabilities allow components to subscribe to specific events that occur
during the various stages of the [component lifecycle][doc-lifecycle].
This includes information about component state changes as well as capability
routing.

For a complete list of supported events and their descriptions, see the
[`fuchsia.sys2.EventType`][event-type] reference documentation.

## Providing event capabilities {#provide}

Event capabilities always originate from component manager. They are provided to
components as a *framework capability*. Individual components cannot declare
events in their manifest `capabilities`.

Each event capability covers a specific *scope*, determined by the
[realm][doc-realms] of the component where the capability was initially offered.
The scope of the capability does not change, even when the capability is
[routed](#route) to another component.

## Routing event capabilities {#route}

Components can [offer](#offer) and [expose](#expose) event capabilities that
they receive from the framework to other components.

For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].

### Exposing {#expose}

Exposing an event capability gives the component's parent access to that
capability:

```json5
{
    expose: [
        {
            event: "started",
            from: "framework",
        },
    ]
}
```

The `from: "framework"` directive means that the protocol capability is
[provided](#provide) by component manager.

### Offering {#offer}

Offering an event capability gives a child component access to that
capability:

```json5
{
    offer: [
        {
            event: "started",
            from: "parent",
            to: [ "#child-a", "#child_b" ],
        },
        {
            event: "stopped",
            from: "framework",
            to: "#child-c",
        }
    ]
}
```

Events can be offered from two sources:

-   `framework`: The event capability provided by component manager.
    The scope of this event will be the offering component's realm.
-   `parent`: An event capability that was offered to the parent by another
    component. The scope of the offered capability matches the component that
    originally routed the capability from `framework`.

## Consuming event capabilities {#consume}

To consume an event capability, the component must request the
[`fuchsia.sys2.EventSource`][event-source] protocol and the set of
[`fuchsia.sys2.EventType`][event-type] for subscription.

To request the capability, add a `use` declaration for it:

```json5
{
    use: [
        { protocol: "fuchsia.sys2.EventSource" },
        { 
            event: [
                "started",
                "stopped",
            ],
            from: "framework",
        },
    ]
}
```

Events can be used from two sources:

-   `framework`: The event capability provided by component manager.
    The scope of this event will be the consuming component's realm.
-   `parent`: An event capability that was offered to the parent by another
    component. The scope of the offered capability matches the component that
    originally routed the capability from `framework`.

Caution: The default source for capabilities is `parent` unless otherwise
specified.

## Static event streams {#event-streams}

Event subscriptions can be set up statically in the component manifest using
*static event streams*.
Static event streams are similar to event streams created through the
`fuchsia.sys2.EventSource/Subscribe` FIDL method but they are set up by the
framework during the resolution of a component's manifest.

The following is an example of the syntax of a static event stream declaration:

```json5
use: [
    {
        event: "started",
        from: "parent",
    },
    {
        event_stream: "MyEventStream",
        subscriptions: [
            {
                event: "started",
            }
        ],
    },
]
```

To connect to a static event stream, call the
`fuchsia.sys2.EventSource/TakeStaticEventStream` FIDL method and provide the
name of the event stream defined in the manifest. In the above example, the
name is `MyEventStream`.

Static event streams are limited to the events listed in the `subscriptions`
section of the manifest. Attempting to subscribe to a different event produces a
validation error. In the above example, the only events available to the stream
are `started` events.

## Event routing example {#example}

Consider the following example of a component topology:

![A visual tree representation of the declarations explained below][example-img]

Notice the following key aspects of this example:

-   `core/archivist`: Receives `started` events for the entire topology through
    an events capability routed from `root`.

    ```json5
    // root.cml
    {
        offer: [
            {
                protocol: "fuchsia.sys2.EventSource",
                from: "parent",
                to: "#core",
            },
            {
                event: "started",
                from: "parent",
                to: "#core",
            },
        ]
    }

    // core.cml
    {
        offer: [
            {
                protocol: "fuchsia.sys2.EventSource",
                from: "parent",
                to: [ "#archivist", "#test_manager" ],
            },
            {
                event: "started",
                from: "parent",
                to: [ "#archivist", "#test_manager" ],
            },
        ]
    }

    // archivist.cml
    {
        use: [
            { protocol: "fuchsia.sys2.EventSource" },
            { event: "started" },
        ]
    }
    ```

-   `core/test_manager/archivist`: Receives `started` events for all test
    components through an events capability routed from `test_manager`.

    ```json5
    // test_manager.cml
    {
        offer: [
            {
                protocol: "fuchsia.sys2.EventSource",
                from: "parent",
                to: [ "#tests", "#archivist" ],
            },
            {
                event: "started",
                from: "framework",
                to: "#archivist",
            },
        ]
    }

    // archivist.cml
    {
          use: [
              { protocol: "fuchsia.sys2.EventSource" },
              { event: "started" },
        ]
    }
    ```

-   `core/test_manager/tests:test-12345`: Receives started events for `foo` and
    `bar` through the events capability provided by `framework`.

    ```json5
    // test-12345.cml
    {
        use: [
            { protocol: "fuchsia.sys2.EventSource" },
            {
                event: "started",
                from: "framework",
            },
        ]
    }
    ```

[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[doc-lifecycle]: /concepts/components/v2/lifecycle.md
[doc-realms]: /concepts/components/v2/realms.md
[event-source]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#EventSource
[event-type]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#EventType
[example-img]: ../images/event-example.png
