<!-- ## Component organization -->
## 组件组织

<!-- All components in the system are composed into a single rooted
**component instance tree**. This tree structure governs several important
aspects of component behavior. -->
系统中所有组件组成一棵单根**组件实例树**。这棵树结构控制着组件行为的几个重要方面。

<!-- ![Tree diagram illustrating how component instances are organized. These parent
and child relationships govern several aspects of component behavior.]
(/get-started/images/intro/component-organization.png){: width="712"} -->
![树图展示了组件实例是如何组织的。这种父子关系控制组件行为的几个方面。]
(/get-started/images/intro/component-organization.png){: width="712"}

<!-- Parent components in the tree are responsible for creating instances of
other components as their children and providing them with the necessary
capabilities. At the same time, child components can expose capabilities back
to the parent. Child components can be created one of two ways: -->
树中的父组件负责将其他组件的实例创建为其子组件，并向其提供必要能力。与此同时，子组件向父组件公开能力。子组件可通过如下方式之一创建：

<!-- * **Statically**: The parent declares the existence of the child in its own
  component declaration.
* **Dynamically**: The parent adds the child to a component collection at
  runtime using the `fuchsia.component.Realm` protocol. -->
* **静态创建** 父组件在自身组件声明中声明子组件。
* **动态创建** 父组件在运行时通过 `fuchsia.component.Realm` 协议将子组件添加到组件集中。

<!-- Any parent component and all its children form a group within the tree called
a **realm**. Realms enable a parent to control which capabilities flow into
and out of its sub-tree of components, creating a capability boundary.
Components decide whether to export capabilities outside their realm using the
`expose` keyword: -->
任何一个父组件和它所有子组件在树中形成一个称之为**领地**的组，领地使父组件可以控制哪个功能流可以进出组件子树，
以此建立一个功能边界。组件自行决定是否通过 `expose` 关键字暴露其功能给领地外部：

```json5
expose: [
    {
        protocol: "fuchsia.example.Foo",
        from: "self",
    },
],
```

<!-- Once a capability is exposed to the realm, the parent can share it with other
components within the same realm. This is done using the `offer` keyword: -->
一旦功能暴露到领地之中，父组件可以将其分享给领地内的其它组件。这可以通过使用 `offer` 关键字达成：

```json5
offer: [
    {
        protocol: "fuchsia.example.Foo",
        from: "self",
    },
],
```

<!-- [Component manager][glossary.component-manager] is responsible for resolving
requests to access a capability (such as a directory or protocol) with the
component providing that capability. This is known as **capability routing**.
Component Manager can only resolve capabilities that are **exposed** and
**offered** within the same realm. -->
[组件管理器][glossary.component-manager]负责解析与功能提供者之间的访问请求（如目录或协议）。
这就是**功能路由**。组件管理器只能解析那些在领地内**公开**和**提供**的功能。

<!-- ![Diagram showing how components share capabilities through
"capability routing," which describes how resources are made available within
a particular realm.] -->
![组件如何通过“功能路由”共享功能的图示，“功能路由”描述了资源如何在特定领地内变得可用。]
(/get-started/images/intro/capability-routing.png){: width="614"}

<aside class="key-point">
<!-- You will explore more about capabilities and building components later on.
You can also find more of the technical details in the
<a href="/concepts/components/v2/introduction.md">component documentation</a>. -->
稍后您可能想探索更多有关功能和如何构建组件的信息。您也可以在<a href="/concepts/components/v2/introduction.md">组件文档</a>中找到更多技术细节。
</aside>

[glossary.component-manager]: /glossary/README.md#component-manager
