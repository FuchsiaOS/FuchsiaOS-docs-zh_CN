# Service capabilities (Components v2)

<<../../_v2_banner.md>>

[Service capabilities][glossary-service] allow components to connect to
[FIDL services][fidl-service] provided either by other components or the
component framework itself.

Note: _Protocol_ and _service_ capabilities are distinct types of capabilities.
A protocol represents a single instance of a
[FIDL protocol][glossary-fidl-protocol], while a service represents zero or more
instances of a [FIDL service][glossary-fidl-service]. See the documentation on
[protocol capabilities][protocol-capability] for more details.

## Providing service capabilities

To provide a service capability, a component must define the capability and
[route](#routing-service-capabilities) it from `self`. The component hosts the
service capability in its [outgoing directory][glossary-outgoing].

To define the capability, add a `capabilities` declaration for it:

```json5
{
    capabilities: [
        {
            service: "fuchsia.example.ExampleService",
        },
    ],
}
```

This defines a capability hosted by this component whose outgoing directory path
is `/svc/fuchsia.example.ExampleService`. You can also customize the path:

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

## Routing service capabilities

Components route service capabilities by either:

-   [exposing](#routing-service-capability-expose) them,
-   or [offering](#routing-service-capability-offer) them.

### Exposing {#routing-service-capability-expose}

Exposing a service capability gives the component's parent access to that
capability. This is done through an [`expose`][expose] declaration.

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

The `from: "self"` directive means that the service capability is provided by
this component. In this case the service must have a corresponding
[definition](#providing-service-capability).

### Offering {#routing-service-capability-offer}

Offering a service capability gives a child component access to that capability.
This is done through an [`offer`][offer] declaration.

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

## Consuming service capabilities

When a component [uses][use] a service capability that has been [offered][offer]
to it, that service is made available through the component's
[namespace][glossary-namespace].

Consider a component with the following manifest declaration:

```
{
    use: [
        {
            service: "fuchsia.example.ExampleService",
        },
    ],
}
```

When the component attempts to open the path
`/svc/fuchsia.example.ExampleService`, the component framework performs
[capability routing][capability-routing] to find the component that provides
this service. Then, the framework connects the newly opened channel to this
provider.

You can also customize the namespace path:

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

For more information about the open request, see
[life of a protocol open][life-of-a-protocol-open].

[capability-routing]: /docs/concepts/components/v2/component_manifests.md#capability-routing
[expose]: /docs/concepts/components/v2/component_manifests.md#expose
[fidl-service]: /docs/concepts/components/v2/services.md
[framework-services]: /docs/concepts/components/v2/component_manifests.md#framework-services
[glossary-fidl]: /docs/glossary.md#fidl
[glossary-fidl-protocol]: /docs/glossary.md#protocol
[glossary-fidl-service]: /docs/glossary.md#service
[glossary-namespace]: /docs/glossary.md#namespace
[glossary-outgoing]: /docs/glossary.md#outgoing-directory
[glossary-protocol]: /docs/glossary.md#protocol-capability
[glossary-service]: /docs/glossary.md#service-capability
[life-of-a-protocol-open]: /docs/concepts/components/v2/capabilities/life_of_a_protocol_open.md
[offer]: /docs/concepts/components/v2/component_manifests.md#offer
[protocol-capability]: /docs/concepts/components/v2/capabilities/protocol.md
[routing-example]: /examples/components/routing
[use]: /docs/concepts/components/v2/component_manifests.md#use
