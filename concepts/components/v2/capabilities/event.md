# event_stream capabilities

event_stream capabilities allow components to subscribe to specific events that
occur during the various stages of the [component lifecycle][doc-lifecycle].
This includes information about component state changes as well as capability
routing.

For a complete list of supported events and their descriptions, see the
[`fuchsia.component.EventType`][event-type] reference documentation. See [RFC-121]
for the original RFC for event_stream capabilities.

## Providing event_stream capabilities {#provide}

event_stream capabilities always originate from component manager. They are
provided to components from AboveRoot. Individual components cannot declare
events in their manifest `capabilities`, nor can they be used `from:
"framework"`

Each event_stream capability has an optional `scope` which determines the
subtree of events that a connecting client would receive. `scope` may be either
a single sub-tree or refer to multiple children.

## Routing event_stream capabilities {#route}

Components can [offer](#offer) event_stream capabilities that they receive from
their parent to other components. event_stream capabilities cannot be exposed.

For more details on how the framework routes component capabilities, see
[capability routing][capability-routing].

### Offering {#offer}

Offering an event capability gives a child component access to that capability:

```json5
{
    offer: [
        {
            event_stream: "started",
            from: "parent",
            scope: ["#child_a"]
            to: [ "#child-a", "#child_b" ],
        },
        {
            event_stream: "stopped",
            from: "parent",
            to: "#child-c",
        }
    ]
}
```

Events can only be offered from parent.

## Consuming event_stream capabilities {#consume}

To consume an event capability, the component must use the `event_stream`
capability. Event streams may be merged by specifying multiple streams in a
single `use`.

To request the capability, add a `use` declaration for it:

```json5
{
    use: [
        { event_stream: ["started", "stopped"] },
    ]
}
```

event_streams may only be used `from: "parent"`.

## event_stream routing example {#example}

Consider the following example of a component topology:

![event_stream example][example-img]

Notice the following key aspects of this example:

-   `core/archivist`: Receives `started` events for the entire topology through
    an events capability routed from `root`.

    ```json5
    // root.cml
    {
        offer: [
            {
                event_stream: "started",
                from: "parent",
                to: "#core",
            },
        ]
    }

    // core.cml
    {
        offer: [
            {
                event_stream: "started",
                from: "parent",
                to: "#archivist",
            },
            {
                event_stream: "started",
                from: "parent",
                to: "#test_manager",
                scope: ["#test_manager"],
            }
        ]
    }

    // archivist.cml
    {
        use: [
            { event_stream: "started" },
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
                event_stream: "started",
                from: "parent",
                to: "#archivist",
            },
            {
                event_stream: "started",
                from: "parent",
                to: "#tests",
                // Downscopes the event to the tests collection
                scope: ["#tests"],
            }
        ]
    }

    - The root Archivist gets `started` events for all components in the
    topology from the root, whereas the embedded Archivist only gets events from
    its own test collection.
    NOTE: An event_stream must be routed through the entire topology from the root
    component_manager all the way down to the component that wants to use the event.
    event_streams cannot be used `from: "framework"`, and they are not
    automatically made available in every component's environment.

    // archivist.cml
    {
          use: [
            {
                event_stream: "started",
                from: "parent",
            },
        ]
    }
    ```

-   `core/test_manager/tests:test-12345`: Receives started events for things
    under its collection through an `event_stream` capability routed from
    `test_manager`.

    ```json5
    // test-12345.cml
    {
        use: [
            {
                event_stream: "started",
                from: "parent",
            },
        ]
    }
    ```

[RFC-121]: /contribute/governance/rfcs/0121_component_events.md
[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[doc-lifecycle]: /concepts/components/v2/lifecycle.md
[doc-realms]: /concepts/components/v2/realms.md
[event-source]: https://fuchsia.googlesource.com/fuchsia/+/refs/changes/44/656544/21/sdk/fidl/fuchsia.sys2/events.fidl#283
[event-type]: https://fuchsia.googlesource.com/fuchsia/+/refs/changes/44/656544/21/sdk/fidl/fuchsia.sys2/events.fidl#21
[example-img]: ../images/example_topology.svg
