## Component organization

All components in the system are composed into a single rooted
**component instance tree**. This tree structure governs several important
aspects of component behavior.

![Tree diagram illustrating how component instances are organized. These parent
and child relationships govern several aspects of component behavior.]
(/docs/get-started/images/intro/component-organization.png){: width="712"}

Parent components in the tree are responsible for creating instances of
other components as their children and providing them with the necessary
capabilities. At the same time, child components can expose capabilities back
to the parent. Child components can be created one of two ways:

* **Statically**: The parent declares the existence of the child in its own
  component declaration.
* **Dynamically**: The parent adds the child to a component collection at
  runtime using the `fuchsia.component.Realm` protocol.

Any parent component and all its children form a group within the tree called
a **realm**. Realms enable a parent to control which capabilities flow into
and out of its sub-tree of components, creating a capability boundary.
Components decide whether to export capabilities outside their realm using the
`expose` keyword:

```json5
expose: [
    {
        protocol: "fuchsia.example.Foo",
        from: "self",
    },
],
```

Once a capability is exposed to the realm, the parent can share it with other
components within the same realm. This is done using the `offer` keyword:

```json5
offer: [
    {
        protocol: "fuchsia.example.Foo",
        from: "self",
    },
],
```

[Component manager][glossary.component-manager] is responsible for resolving
requests to access a capability (such as a directory or protocol) with the
component providing that capability. This is known as **capability routing**.
Component Manager can only resolve capabilities that are **exposed** and
**offered** within the same realm.

![Diagram showing how components share capabilities through
"capability routing," which describes how resources are made available within
a particular realm.]
(/docs/get-started/images/intro/capability-routing.png){: width="614"}

<aside class="key-point">
You will explore more about capabilities and building components later on.
You can also find more of the technical details in the
<a href="/docs/concepts/components/v2/introduction.md">component documentation</a>.
</aside>

[glossary.component-manager]: /docs/glossary/README.md#component-manager
