A protocol handle is a well-known object that provides an implementation of a
FIDL protocol that is discoverable using component namespaces. The component
framework facilitates protocol discovery between
[components](/glossary/README.md#component) using capabilities.
Capability routing describes which component should act as the provider for any
given client. Once the proper components are identified, the
[Component Manager](/glossary/README.md#component-manager)
initiates connections between components using handles found in each
component's namespace.

Consider the following example for a `fuchsia.example.Foo` protocol:

![Diagram showing how connecting components is a combination of capability
routing and protocol serving. Components must serve the implementation of a
protocol they offer to other components.](
    /get-started/images/fidl/protocol-serving.png){: width="629"}

The diagram highlights the main elements involved in performing the connection:

1.  The provider component statically **declares** the protocol in the
    `capabilities` section of the manifest. This enables the component framework
    to perform capability routing.
1.  A client component statically **requests** the protocol in the `use` section
    of the manifest. This creates the `/svc/fuchsia.example.Foo` protocol entry
    in the client's namespace if capability routing is successful.
1.  The provider code **publishes** the implementation at runtime. This creates
    a protocol entry at `/svc/fuchsia.example.Foo` in the provider's outgoing
    directory.
1.  The client code **connects** to the protocol handle at runtime. This opens a
    FIDL connection to the implementation running in the provider component.
