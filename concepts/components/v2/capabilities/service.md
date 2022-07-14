# Service capabilities

<<../../_v2_banner.md>>

Caution: service capabilities are experimental and in development. Their
behavior and APIs could change at any time.

A [service capability][glossary.service-capability] is a capability that
enables discovery of one or more individually named
[FIDL service][glossary.service] instances. Service capabilities are backed by
a [glossary.channel] that speaks the [`Directory`][directory.fidl] protocol,
where each entry in the directory exposes a named [service instance](#instances).

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

service EchoService {
    regular_echo client_end:Echo;
    reversed_echo client_end:Echo;
};
```

Note: For more details on FIDL service syntax, see the
[FIDL language reference][fidl-reference].

Service implementations are served from provider components using the
[outgoing directory][glossary.outgoing-directory] and consumed from another
component's [namespace][glossary.namespace].

## Service instances {#instances}

Multiple named instances of a service can be hosted by a single component.
These are present in the [namespace][glossary.namespace] of the consuming
component as subdirectories of the service.
The component framework generates an arbitrary, unique identifier for each
service instance name.

For example, if the framework generates `57dfe118a2a8` as the instance name of
the `fuchsia.examples.EchoService` service, a consuming component could connect
to the protocols in that instance using the following namespace paths:

- `/svc/fuchsia.examples.EchoService/57dfe118a2a8/regular_echo`
- `/svc/fuchsia.examples.EchoService/57dfe118a2a8/reversed_echo`

## Providing service capabilities {#provide}

To provide a service capability, a component must declare the capability and
[route](#route) it from `self`. The component hosts the service capability in
its [outgoing directory][glossary.outgoing-directory].

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

## Routing service capabilities {#route}

Components route service capabilities by [exposing](#expose) them to their
parent and [offering](#offer) them to their children.

For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].

### Exposing {#expose}

Exposing a service capability gives the component's parent access to that
capability:

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

The `from: "self"` directive means that the service capability is
[provided](#provide) by this component.

#### Dynamic collections

A service capability can be exposed from a [dynamic collection][collection]:

```json5
{
    collections: [
        {
            name: "coll",
            durability: "transient",
        },
    ],
    expose: [
        {
            service: "fuchsia.example.ExampleService",
            from: "#coll",
        },
    ],
}
```

Note: When routing services exposed from components in the collection, the
component framework renames each [service instance](#instances) with an
arbitrary, unique identifier to allow multiple components in the collection to
expose the same service.

### Offering {#offer}

Offering a service capability gives a child component access to that
capability:

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

## Consuming service capabilities {#consume}

To consume a service capability, the component must request the capability and
open the corresponding path in its [namespace][glossary.namespace].

To request the capability, add a `use` declaration for it:

```json5
{
    use: [
        {
            service: "fuchsia.example.ExampleService",
        },
    ],
}
```

This populates the service in the component's namespace at the well-known path
`/svc/fuchsia.example.ExampleService`. You can also customize the path:

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

Note: For a working example of routing a service capability between components,
see [`//examples/components/services`][routing-example].

[glossary.channel]: /glossary/README.md#channel
[glossary.namespace]: /glossary/README.md#namespace
[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[glossary.protocol]: /glossary/README.md#protocol
[glossary.service]: /glossary/README.md#service
[glossary.service-capability]: /glossary/README.md#service-capability
[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[collection]: /concepts/components/v2/realms.md#collections
[fidl-reference]: /reference/fidl/language/language.md
[life-of-a-protocol-open]: /concepts/components/v2/capabilities/life_of_a_protocol_open.md
[directory.fidl]: https://fuchsia.dev/reference/fidl/fuchsia.io#Directory
[realm.fidl]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Realm
[routing-example]: /examples/components/services
