# Protocol capabilities

A [protocol capability][glossary.protocol-capability] is a capability backed
by a [channel][glossary.channel] that speaks a particular
[FIDL protocol][glossary.protocol].

```fidl
library fuchsia.examples;

const MAX_STRING_LENGTH uint64 = 32;

@discoverable
protocol Echo {
    EchoString(struct {
        value string:MAX_STRING_LENGTH;
    }) -> (struct {
        response string:MAX_STRING_LENGTH;
    });
    SendString(struct {
        value string:MAX_STRING_LENGTH;
    });
    -> OnString(struct {
        response string:MAX_STRING_LENGTH;
    });
};
```

Note: For more details on FIDL protocol syntax, see the
[FIDL language reference][fidl-reference].

Protocol implementations are served from provider components using the
[outgoing directory][glossary.outgoing-directory] and consumed from another
component's [namespace][glossary.namespace].

## Providing protocol capabilities {#provide}

To provide a protocol capability, a component must declare the capability and
[route](#route) it from `self`. The component hosts the protocol capability in
its [outgoing directory][glossary.outgoing-directory].

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

## Routing protocol capabilities {#route}

Components route protocol capabilities by [exposing](#expose) them to their
parent and [offering](#offer) them to their children.

For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].

### Exposing {#expose}

Exposing a protocol capability gives the component's parent access to that
capability:

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

The `from: "self"` directive means that the protocol capability is
[provided](#provide) by this component.

### Offering {#offer}

Offering a protocol capability gives a child component access to that
capability:

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

## Consuming protocol capabilities {#consume}

To consume a protocol capability, the component must request the capability and
open the corresponding path in its [namespace][glossary.namespace].

To request the capability, add a `use` declaration for it:

```json5
{
    use: [
        {
            protocol: "fuchsia.example.ExampleProtocol",
        },
    ],
}
```

This populates the protocol in the component's namespace at the well-known path
`/svc/fuchsia.example.ExampleProtocol`. You can also customize the path:

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

Note: For a working example of routing a protocol capability between components,
see [`//examples/components/routing`][routing-example].

### Consuming optional protocol capabilities

See [Connect Components: Consuming optional capabilities][consuming-optional-capabilities].

## Framework protocols {#framework}

A *framework protocol* is a protocol provided by the component framework.
Any component may `use` these capabilities by setting `framework` as the source
without an accompanying `offer` from its parent.
Fuchsia supports the following framework protocols:

-   [`fuchsia.component.Realm`][fidl-realm]: Allows a component to manage and bind to
    its children. Scoped to the component's realm.
-   [`fuchsia.component.Binder`][fidl-binder]: Allows a component to start
    another component.

```json5
{
    use: [
        {
            protocol: "fuchsia.component.Realm",
            from: "framework",
        },
    ],
}
```

[glossary.namespace]: /docs/glossary/README.md#namespace
[glossary.outgoing-directory]: /docs/glossary/README.md#outgoing-directory
[glossary.channel]: /docs/glossary/README.md#channel
[glossary.protocol]: /docs/glossary/README.md#protocol
[glossary.protocol-capability]: /docs/glossary/README.md#protocol-capability
[capability-routing]: /docs/concepts/components/v2/capabilities/README.md#routing
[consuming-optional-capabilities]: /docs/development/components/connect.md#consuming-optional-capabilities
[fidl-reference]: /docs/reference/fidl/language/language.md
[fidl-binder]: /sdk/fidl/fuchsia.component/binder.fidl
[fidl-realm]: /sdk/fidl/fuchsia.component/realm.fidl
[life-of-a-protocol-open]: /docs/concepts/components/v2/capabilities/life_of_a_protocol_open.md
[routing-example]: /examples/components/routing
