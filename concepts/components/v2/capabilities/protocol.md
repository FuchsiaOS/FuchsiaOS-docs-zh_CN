# Protocol capabilities (Components v2)

<<../../_v2_banner.md>>

[Protocol capabilities][glossary-protocol] allow components to
connect to [FIDL protocols][glossary-fidl-protocol] provided either by other
components or the component framework itself.

Note: _Protocol_ and _service_ capabilities are distinct types of
capabilities. A protocol represents a single instance of a
[FIDL protocol][glossary-fidl-protocol], while a service represents zero or
more instances of a [FIDL service][glossary-fidl-service].
See the documentation on [service capabilities][service-capability]
for more details.

## Providing protocol capabilities

To provide a protocol capability, a component must define the capability and
[route](#routing-protocol-capabilities) it from `self`. The component hosts the
protocol capability in its [outgoing directory][glossary-outgoing].

To define the capability, add a `capabilities` declaration for it:

```json5
{
    capabilities: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
        },
    ],
}
```

This defines a capability hosted by this component whose outgoing directory path
is `/svc/fuchsia.example.ExampleProtocol`. You can also customize the path:

```json5
{
    capabilities: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
            path: "/my_svc/fuchsia.example.MyExampleProtocol",
        },
    ],
}
```

## Routing protocol capabilities

Components route protocol capabilities by either:

- [exposing](#routing-protocol-capability-expose) them,
- or [offering](#routing-protocol-capability-offer) them.

### Exposing {#routing-protocol-capability-expose}

Exposing a protocol capability gives the component's parent access to that
capability. This is done through an [`expose`][expose] declaration.

```json5
{
    expose: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
            from: "self",
        },
    ],
}
```

The `from: "self"` directive means that the protocol capability is provided by
this component. In this case the protocol must have a corresponding
[definition](#providing-protocol-capability).

### Offering {#routing-protocol-capability-offer}

Offering a protocol capability gives a child component access to that capability.
This is done through an [`offer`][offer] declaration.

```json5
{
    offer: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
            from: "self",
            to: [ "#child-a", "#child_b" ],
        },
    ],
}
```

## Consuming protocol capabilities

When a component [uses][use] a protocol capability that has been [offered][offer]
to it, that protocol is made available through the component's
[namespace][glossary-namespace].

Consider a component with the following manifest declaration:

```
{
    use: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
        },
    ],
}
```

When the component attempts to open the path
`/svc/fuchsia.example.ExampleProtocol`, the component framework performs
[capability routing][capability-routing] to find the component that provides
this protocol. Then, the framework connects the newly opened channel to this
provider.

You can also customize the namespace path:

```json5
{
    use: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
            path: "/my_svc/fuchsia.example.MyExampleProtocol",
        },
    ],
}
```

For more information about the open request, see
[life of a protocol open][life-of-a-protocol-open].

For a working example of routing a protocol capability from one component to
another, see [`//examples/components/routing`][routing-example].

## Consuming protocol capabilities provided by the framework

Some protocol capabilities are provided by the component framework, and thus
can be [used][use] by components without their parents [offering][offer] them.

For a list of these protocols and what they can be used for, see
[framework protocols][framework-protocols].

```json5
{
    use: [
        {
            protocol: "fuchsia.sys2.Realm",
            from: "framework",
        },
    ],
}
```

[capability-routing]: /docs/concepts/components/v2/component_manifests.md#capability-routing
[expose]: /docs/concepts/components/v2/component_manifests.md#expose
[framework-protocols]: /docs/concepts/components/v2/component_manifests.md#framework-protocols
[glossary-fidl]: /docs/glossary.md#fidl
[glossary-fidl-protocol]: /docs/glossary.md#protocol
[glossary-fidl-service]: /docs/glossary.md#service
[glossary-namespace]: /docs/glossary.md#namespace
[glossary-outgoing]: /docs/glossary.md#outgoing-directory
[glossary-protocol]: /docs/glossary.md#protocol-capability
[life-of-a-protocol-open]: /docs/concepts/components/v2/capabilities/life_of_a_protocol_open.md
[offer]: /docs/concepts/components/v2/component_manifests.md#offer
[routing-example]: /examples/components/routing
[service-capability]: /docs/concepts/components/v2/capabilities/service.md
[use]: /docs/concepts/components/v2/component_manifests.md#use
