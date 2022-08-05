<!-- 
All components in the system are composed into a rooted
**component instance tree**. Parent components in the tree are responsible for
declaring instances of other components as their children and providing them
with capabilities. At the same time, child components can expose capabilities
back to the parent. These component instance and capability relationships make
up the **component topology**.
 -->
系统中的所有组件组成一个有根的**组件实例树**。树中的父组件负责将其他组件的实例声明为其子组件并为它们提供功能。同时，子组件可以向父组件公开功能。这些组件实例和功能关系构成了**组件拓扑**。

<!-- 
Any parent component and all its children form a group within the tree called a
**realm**. Realms enable a parent to control which capabilities flow into and
out of its sub-tree of components, creating a capability boundary. This
encapsulation allows the realm to be reorganized internally without affecting
external components dependent on its exposed capabilities.
 -->
任何父组件及其所有子组件在树中形成一个称为**领域**的组。领域使父级能够控制那些功能流入和流出其组件的子树，从而创建功能边界。这种封装允许领域在内部进行重组，而不会影响依赖于其公开功能的外部组件。

<!-- 
![Diagram showing how component instances are organized into a tree and parent
components determine the capabilities available to each child through
"capability routing."](/get-started/images/components/component-topology.png){: width="616"}
 -->
![图表展示了组件实例被组织成一个树，父组件通过“功能路由”确定每个子组件可用的功能。](/get-started/images/components/component-topology.png){: width="616"}

<!-- 
In the above diagram, a protocol capability for `fuchsia.example.Foo` is routed
through the component instance tree from the provider to the client. Components
declare the capabilities they **require** with the `use` keyword:
 -->
在上图中，`fuchsia.example.Foo` 协议功能通过组件实例树从提供者路由到客户端。组件使用 `use` 关键字声明它们**需要**的功能：

<!-- 
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
 -->
```json5
{
    // 程序运行的有关信息。
    program: {
        // 使用内置的 ELF 运行程序来运行二进制文件。
        runner: "elf",
        // 运行该组件的二进制文件。
        binary: "bin/client",
    },

    // 此组件所需的功能。
    use: [
        { protocol: "fuchsia.example.Foo" },
    ],
}
```

<!-- 
Components declare the capabilities they implement, or **provide**, using the
`capabilities` section of the component manifest. This makes the capability and
its provider known to the component framework. See the following `provider.cml`
example:
 -->
组件使用组件清单中的 `capabilities` 部分声明它们实现或**提供**的功能。这使得组件框架知道该功能和它的提供者。请查看下面的 `provider.cml` 示例：

<!-- 
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
 -->
```json5
{
    // 程序运行的有关信息。
    program: {
        // 使用内置的 ELF 运行程序来运行二进制文件。
        runner: "elf",
        // 运行该组件的二进制文件。
        binary: "bin/provider",
    },

    // 此组件提供的功能。
    capabilities: [
        { protocol: "fuchsia.example.Foo" },
    ],
    // 通过此组件路由的功能。
    expose: [
        {
            protocol: "fuchsia.example.Foo",
            from: "self",
        },
    ],
}
```

<!-- 
The `expose` keyword makes the capability available from this component to other
realms through its parent, which may also include capabilities provided by this
component's children. In this case, the source of the capability is `self`
because this component is the provider.
 -->
`expose` 关键字使这个组件的功能通过它的父组件向其他领域提供，这也可能包括这个组件的子组件提供的功能。在这种情况下，功能的来源是 `self`，因为这个组件是提供者。

<!-- 
Parent components control **capability routing** within the realm, creating
explicit pathways from the client component to a provider. See the following
example `parent.cml` manifest:
 -->
父组件控制领域内的**功能路由**，创建从客户端组件到提供者的显式路径。请查看以下 `parent.cml` 清单示例：

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

<!-- 
<aside class="key-point">
<b>Tip:</b> Strings prefixed with <code>#</code> in the manifest are
<a href="https://fuchsia.dev/reference/cml#references">references</a>
to a child component instance.
</aside>
 -->
<aside class="key-point">
<b>提示：</b>清单中以 <code>#</code> 为前缀的字符串是对子组件实例的<a href="https://fuchsia.dev/reference/cml#references">引用</a>。</aside>

<!-- 
The parent component declares the set of child components in the realm and
routes capabilities to them using the `offer` keyword. In this way, the parent
determines both the scope and the source of each child's capabilities. This also
enables multiple components in the topology to provide the same capability, as
the component framework relies on explicit routes to determine how to resolve
the requests from each client.
 -->
父组件在领域中声明一组子组件，并使用 `offer` 关键字将功能路由到它们。用这种方法，父组件就决定了每个子组件功能的范围和来源。这也使拓扑中的多个组件能够提供相同的功能，因为组件框架依赖于显式路由来确定如何解析来自每个客户端的请求。

<!-- 
Note: For more details on component organization, see
[Component topology](/concepts/components/v2/topology.md).
 -->
注意：要获取关于组件组织的更多详细信息，请参阅[组件拓扑](/concepts/components/v2/topology.md).
