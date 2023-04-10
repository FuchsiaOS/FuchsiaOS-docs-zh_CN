All components in the system are composed into a rooted
**component instance tree**. Parent components in the tree are responsible for
declaring instances of other components as their children and providing them
with capabilities. At the same time, child components can expose capabilities
back to the parent. These component instance and capability relationships make
up the **component topology**.

Any parent component and all its children form a group within the tree called a
**realm**. Realms enable a parent to control which capabilities flow into and
out of its sub-tree of components, creating a capability boundary. This
encapsulation allows the realm to be reorganized internally without affecting
external components dependent on its exposed capabilities.

![Diagram showing how component instances are organized into a tree and parent
components determine the capabilities available to each child through
"capability routing."](/get-started/images/components/component-topology.png){: width="616"}

In the above diagram, a protocol capability for `fuchsia.example.Foo` is routed
through the component instance tree from the provider to the client. Components
declare the capabilities they **require** with the `use` keyword:

```json5
{
    // Information about the program to run.
    program: {
        // Use the built-in ELF runner to run core binaries.
        runner: "elf",
        // The binary to run for this component.
        binary: "bin/client",
    },

    // Capabilities required by this component.
    use: [
        { protocol: "fuchsia.example.Foo" },
    ],
}
```

Components declare the capabilities they implement, or **provide**, using the
`capabilities` section of the component manifest. This makes the capability and
its provider known to the component framework. See the following `provider.cml`
example:

```json5
{
    // Information about the program to run.
    program: {
        // Use the built-in ELF runner to run core binaries.
        runner: "elf",
        // The binary to run for this component.
        binary: "bin/provider",
    },

    // Capabilities provided by this component.
    capabilities: [
        { protocol: "fuchsia.example.Foo" },
    ],
    // Capabilities routed through this component.
    expose: [
        {
            protocol: "fuchsia.example.Foo",
            from: "self",
        },
    ],
}
```

The `expose` keyword makes the capability available from this component to other
realms through its parent, which may also include capabilities provided by this
component's children. In this case, the source of the capability is `self`
because this component is the provider.

Parent components control **capability routing** within the realm, creating
explicit pathways from the client component to a provider. See the following
example `parent.cml` manifest:

```json5
{
    children: [
        {
            name: "provider",
            url: "fuchsia-pkg://fuchsia.com/foo-package#meta/provider.cm",
        },
        {
            name: "client",
            url: "fuchsia-pkg://fuchsia.com/foo-package#meta/client.cm",
        },
    ],
    offer: [
        {
            protocol: "fuchsia.example.Foo",
            from: "#provider",
            to: [ "#client" ],
        },
    ],
}
```

<aside class="key-point">
<b>Tip:</b> Strings prefixed with <code>#</code> in the manifest are
<a href="https://fuchsia.dev/reference/cml#references">references</a>
to a child component instance.
</aside>

The parent component declares the set of child components in the realm and
routes capabilities to them using the `offer` keyword. In this way, the parent
determines both the scope and the source of each child's capabilities. This also
enables multiple components in the topology to provide the same capability, as
the component framework relies on explicit routes to determine how to resolve
the requests from each client.

Note: For more details on component organization, see
[Component topology](/concepts/components/v2/topology.md).
