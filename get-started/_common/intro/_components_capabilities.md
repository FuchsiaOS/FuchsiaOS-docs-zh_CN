## Component capabilities

Components obtain privileges to access various parts of the wider system
through **capabilities**. Each component can declare new capabilities that
they offer to the system and capabilities provided by other components
(or the framework) that they require to function.

As you just saw, `runner` is an example of a capability declaring the runtime
used by the component. Other examples of common capability types are
`directory` to access filesystem resources and `protocol` for communicating
with other components.

Developers declare the capability types required by the component using the
component manifest. Below is an example of a component manifest requesting
two capabilities: read access to an `example-data` directory and a service
described by the `fuchsia.example.Foo` FIDL protocol.

```json5
use: [
    {
        directory: "example-data",
        rights: [ "r*" ],
        path: "/example/data",
    },
    {
        protocol: "fuchsia.example.Foo",
    },
]
```

Component manager uses the capability declarations to populate each component's
namespace with the necessary directory handles. For this example, the component
would receive `/example/data` and `/svc/fuchsia.example.Foo` in their namespace.
