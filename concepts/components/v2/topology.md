<!--
# Component topology
 -->
# 组件拓扑

<!--
The _[component topology][glossary.component topology]_ is a general concept
that expresses the set of relationships between
[component instances](#component-instances).
 -->
[组件拓扑][glosossary.compontontoly]（_component topology_）是一个通用概念，表达了[组件实例](#component-instances)之间的一系列关系。

<!--
These relationships are the following:
 -->
这些关系如下：

<!--
-   Component instance tree: Describes how component instances are
    [composed](#composition) together (their parent-child relationships).
-   Capability routing graph: Describes how component instances gain access to
    use capabilities published by other component instances (their
    provider-consumer relationships).
 -->
-   组件实例树：描述组件实例是如何[组合](#composition)在一起的（其父子关系）。
-   能力路由图：描述组件实例是如何获得由其他组件实例发布能力的使用权限的（其提供者-消费者关系）。

<!--
## Component instances {#component-instances}
 -->
## 组件实例 {#component-instances}

<!--
A _component instance_ is a distinct embodiment of a
[component][glossary.component] running in its own sandbox that is isolated from
other component instances (including other instances of the same component).
 -->
组件实例（_component instance_）是[组件]的[glossary.component]明确实体，运行在其自身的沙箱中，该沙箱与其他组件实例（包括同一组件的其他实例）隔离。

<!--
You can often use the terms component and component instance interchangeably
when the context is clear. For example, it would be more precise to talk about
"starting a component instance" rather than "starting a component" but you can
easily infer that "starting a component" requires an instance of that component
to be created first so that the instance can be started.
 -->
上下文清晰时，您通常可以交换使用术语组件（component）和组件实例（component instance）。例如，谈论“启动一个组件实例”比“启动一个组件”更准确，但您可以很容易地推断出“启动一个组件”需要先创建该组件的一个实例，以便该实例可以启动。

<!--
While components are identified by a [URL][doc-component-urls], component
instances are identified by a [moniker](#monikers). Different instances of the
same component thus share the same URL but have different monikers.
 -->
尽管组件通过[网址][doc-component-urls]（URL）标识，但组件实例是通过[代称](#monikers)（moniker）标识的。因此，同一组件的不同实例共享相同的网址，却拥有不同的代称。

<!--
<br>![Components and component instances](images/topology_instances.png)<br>
 -->
<br>![组件和组件实例](images/topology_instances.png)<br>

<!--
## Component instance tree {#composition}
 -->
## 组件实例树 {#composition}

<!--
The _component instance tree_ expresses how components are assembled together to
make more complex components.
 -->
组件实例树（_component instance tree_）表达组件如何组装在一起来制造更为复杂的组件。

<!--
Using hierarchical composition, a parent component creates instances of other
components, which are known as its _children_. The child instances belong to the
parent and depend on the parent to provide them with the capabilities that they
need to run. Meanwhile, the parent gains access to the capabilities exposed by
its children through [capability routing](#capability-routing).
 -->
使用分层组合，父组件创建其他组件的实例，这些实例称为它的“子组件”。子实例属于父实例，并依赖父实例为它们提供运行所需的能力。同时，父组件通过[能力路由](#capability-routing)（capability routing）访问其子组件公开的能力。

<!--
Children can be created in two ways:
 -->
子组件可以通过两种方式创建：

<!--
-   Statically: The parent declares the existence of the child in its own
    [component declaration][doc-component-declaration]. The child is destroyed
    automatically if the child declaration is removed in an updated version of
    the parent's software.
-   Dynamically: The parent uses the
    [Realm framework protocol][doc-realm-framework-protocol] to add a child to a
    [component collection][doc-collections] that the parent declared. The parent
    destroys the child in a similar manner.
 -->
 - 静态创建：父组件在自己的[组件声明][doc-component-declaration]（component declaration）中声明子组件的存在。如果父组件的软件的更新版本中删除了子组件声明，则子组件会自动销毁。
 - 动态创建：父组件使用[领域框架协议][doc-realm-framework-protocol]（Realm framework protocol）将子组件添加到父组件声明的[组件集合][doc-collections]（component collection）中。父组件以类似的方式销毁子组件。

<!--
The component topology represents the structure of these parent-child
relationships as a [component instance tree][glossary.component-instance-tree].
 -->
组件拓扑将这些父子关系的结构表现为[组件实例树][glossary.component-instance-tree]。

<!--
<br>![Diagram of component instance tree](images/topology_instance_tree.png)<br>
 -->
<br>![组件实例树的图示](images/topology_instance_tree.png)<br>

<!--
## Monikers {#monikers}
 -->
## 代称 {#monikers}

<!--
A _moniker_ identifies a specific component instance in the component tree using
a topological path. There are three types of monikers, depending on how the
moniker is being used and kind of relationship it's describing: absolute,
relative, and child.
 -->
代称（_moniker_）使用拓扑路径在组件树中标识一个特定的组件实例。代称有三种类型，按照代称的使用方式及其描述关系的类型来划分：绝对（absolute）、相对（relative）和子级（child）。

<!--
See the [monikers documentation][doc-monikers] for more information.
 -->
要获取更多信息，请参阅[代称文档][doc-monikers]。

<!--
<br>![Diagram of component instance tree with absolute monikers](images/topology_monikers.png)<br>
 -->
<br>![具有绝对代称的组件实例树的图示](images/topology_monikers.png)<br>

<!--
## Realms {#realms}
 -->
## 领域 {#realms}

<!--
A _realm_ is a subtree of the component instance tree. Each realm is rooted by a
component instance and includes all of that instance's children and their
descendants. Put another way, realms express
[hierarchical composition](#composition) of component instances.
 -->
领域（_realm_）是组件实例树的子树。每个领域都由组件实例作为根，并包括该实例的所有子实例及其后代。换句话说，领域表示组件实例的[层次组成](#composition)。

<!--
Realms are important [encapsulation](#encapsulation) boundaries in the component
topology. The root of each realm receives certain privileges to influence the
behavior of components, such as:
 -->
领域是组件拓扑中的重要[封装](#encapsulation)边界。每个领域的根都会获得某些权限来影响组件的行为，例如：

<!--
-   Declaring how capabilities flow into, out of, and within the realm.
-   Binding to child components to access their capabilities.
-   Creating and destroying child components.
 -->
-   声明能力如何流入、流出领域和流动于领域内部。
-   绑定至子组件以访问其能力。
-   创建和销毁子组件。

<!--
See the [realms documentation][doc-realms] for more information.
 -->
要获取更多信息，请参阅[领域文档][doc-realms]。

<!--
<br>![Diagram of component realms](images/topology_realms.png)<br>
 -->
<br>![组件的领域图示](images/topology_realms.png)<br>

<!--
## Encapsulation {#encapsulation}
 -->
## 封装 {#encapsulation}

<!--
A component acts as an encapsulation boundary. Capabilities cannot escape a
component's [realm](#realms) unless explicitly allowed to by an
[expose][doc-expose] declaration.
 -->
组件充当封装边界。能力不能逃脱组件的[领域](#realms)，除非通过 [expose][doc-expose]（公开）声明显式允许。

<!--
Children remain forever dependent upon their parent; they cannot be reparented
and they cannot outlive their parent. When a parent is destroyed so are all of
its children.
 -->
子组件总是依赖于父组件。子组件不可被重新分配父组件，也不可晚于父组件消亡。当父组件销毁时，其所有子组件均会销毁。

<!--
This model resembles [composition][wiki-object-composition]{:.external} in
object-oriented programming languages.
 -->
该模型类似于面向对象程序设计语言中的[复合][wiki-object-composition]{:.external}（composition）。

<!--
See the [realms documentation][doc-realms] for more information.
 -->
要获取更多信息，请参阅[领域文档][doc-realms]。

<!--
<br>![Diagram of component instance encapsulation](images/topology_encapsulation.png)<br>
 -->
<br>![组件实例封装的图示](images/topology_encapsulation.png)<br>

<!--
## Capability routing graph {#capability-routing}
 -->
## 能力路由图 {#功能路由}

<!--
The _capability routing graph_ describes how components gain access to use
capabilities exposed and offered by other components in the component instance
tree. For a capability provided by a component instance to be usable by a
consumer component instance, there must be capability routing path between them.
Such capability routes are determined by `use`, `offer`, and `expose`
declarations in [component declarations][doc-component-declaration].
 -->
能力路由图（_capability routing graph_）描述了组件如何获取组件实例树中其他组件公开和提供的能力的使用权限。为了让组件实例提供的能力可供消费者组件实例使用，其间必须有能力路由路径。此类能力路由通过[组件声明][doc-component-declaration]中的 `use`（使用）、`offer`（提供）和 `expose`（公开）声明确定。

<!--
See the [capability routing documentation][doc-capability-routing] for more
information.
 -->
要获取更多信息，请参阅[功能路由文档][doc-capability-routing]。

<!--
<br>![Diagram of capability routing](images/topology_capability_routing.png)<br>
 -->
<br>![功能路由图示](images/topology_capability_routing.png)<br>

[glossary.component]: /glossary/README.md#component
[glossary.component instance tree]: /glossary/README.md#component-instance-tree
[glossary.component topology]: /glossary#component-topology
[doc-collections]: /concepts/components/v2/realms.md#collections
[doc-environments]: /concepts/components/v2/environments.md
[doc-expose]: https://fuchsia.dev/reference/cml#expose
[doc-realms]: /concepts/components/v2/realms.md
[doc-realm-framework-protocol]: /concepts/components/v2/realms.md#realm-framework-protocol
[doc-monikers]: /concepts/components/v2/identifiers.md#monikers
[doc-component-urls]: /concepts/components/v2/identifiers.md#component-urls
[doc-capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[doc-component-declaration]: /concepts/components/v2/component_manifests.md#component-declaration
[wiki-least-privilege]: https://en.wikipedia.org/wiki/Principle_of_least_privilege
[wiki-object-composition]: https://en.wikipedia.org/wiki/Object_composition
