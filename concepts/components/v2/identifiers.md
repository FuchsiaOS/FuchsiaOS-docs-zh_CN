<!--
# Component identifiers
 -->
# 组件标识符

<!--
The Component Framework uses different identifiers to describe components.
This section describes the relationship between the following component
identifiers, and their usage:
 -->
组件框架使用不同的标识符来描述组件。本节介绍了以下组件标识符之间的关系及其用法：

<!--
-   [Component URLs](#component-urls): Identifies a component as a resource to
    be fetched by a component resolver.
-   [Monikers](#monikers): Identifies specific component instances in the
    component instance tree.
 -->
 -  [组件网址](#component-urls)（component URL）：将组件标识为由组件解析器获取的资源。
 -  [代称](#monikers)（moniker）：在组件实例树中标识特定的组件实例。

<!--
## Component URLs {#component-urls}
 -->
## 组件网址 {#component-urls}

<!--
A [component URL][glossary.component-url] is a [URL][wiki-url] that locates a
component, including its declaration, program, and assets. Component Framework
uses [component resolvers][doc-resolvers] to resolve a component URL into a
[component declaration][doc-manifests-declaration].
 -->
[组件网址][glossary.component-url]是可以定位组件的[网址][wiki-url]，包括其声明、程序和资源。组件框架使用[组件解析器][doc-resolvers]将组件网址解析为[组件声明][doc-manifests-declaration]。

<!--
### Usage
 -->
### 用法

<!--
The primary use of component URLs is to identify a component in the definition
of a component instance, as part of a [child declaration][doc-manifests-children]:
 -->
组件网址的主要用途是确定组件实例定义中的组件，作为[子组件声明][doc-manifests-children]的一部分。

```json5 {:.devsite-disable-click-to-copy}
{
    children: [
        {
            name: "logger",
            url: "fuchsia-pkg://fuchsia.com/logger#logger.cm",
        },
    ],
}
```

<!--
The above example declares the `logger` component as an absolute resource
in a [Fuchsia package][doc-package] hosted in a package repository.
 -->
上面的示例将 `logger` 组件声明为托管于包仓库中的 [Fuchsia 包][doc-package]的绝对资源。

<!--
Component Framework also supports relative URLs.
 -->
组件框架也支持相对网址。

<!--
To identify a component built into the same package as the parent component,
specify only the URL fragment:
 -->
要识别构建至与父组件同一包中的组件，请仅指定网址片段：

```json5 {:.devsite-disable-click-to-copy}
{
    children: [
        {
            name: "child",
            url: "#meta/child.cm",
        }
    ],
}
```

<!--
To identify a component in a [subpackage][doc-subpackaging] of the parent
component's package, include the subpackage name followed by the component
manifest path (via URL fragment):
 -->
要识别父组件包的[子包][doc-subpackaging]中的一个组件，请（通过网址片段）包括子包名称及组件清单路径：

```json5 {:.devsite-disable-click-to-copy}
{
    children: [
        {
            name: "child",
            url: "child#meta/default.cm",
        }
    ],
}
```

<!--
Relative component URLs are often used in tests, where the best practice is to
re-package production components in a test-specific package to promote
[hermeticity][test-hermeticity].
 -->
相对组件网址通常用于测试中，最佳做法是在特定于测试的包中重新打包生产组件，以提高[密封性][test-hermeticity]。

<!--
For more details on component URL syntax, see the
[component URL reference][url-reference].
 -->
要获取关于组件网址语法的更多细节，请参阅[组件网址参考][url-reference]。

<!--
## Monikers {#monikers}
 -->
## 代称 {#monikers}

<!--
A [component moniker][glossary.moniker] identifies a specific component instance
in the component instance tree using a topological path.
 -->
[组件代称][glossary.moniker]（component moniker）使用拓扑路径在组件实例树中标识特定的组件实例。

<!--
### Design principles
 -->
### 设计原则

<!--
#### Stability
 -->
#### 稳定性

<!--
Monikers are stable identifiers. Assuming the component topology does not
change, the monikers used to identify component instances in the topology
will remain the same.
 -->
代称是稳定标识符。假设组件拓扑不会改变，则用于标识拓扑中组件实例的代称将保持不变。

<!--
#### Uniqueness
 -->
#### 唯一性

<!--
Each time a component instance is destroyed and a new component instance with
the same name is created in its place in the component topology (as a child
of the same parent), the new instance is assigned a unique instance identifier
to distinguish it from prior instances in that place.
 -->
每当销毁组件实例并在组件拓扑中创建具有相同名称的新组件实例（作为同一父实例的子实例）时，会为新实例分配唯一的实例标识符，以将其与先前在同一位置的实例区分开。

<!--
Monikers include unique instance identifiers to prevent confusion of old
component instances with new component instances of the same name as the
tree evolves.
 -->
代称包括唯一的实例标识符，以防止随着树的发展将旧组件实例与同名的新组件实例混淆。

<!--
#### Privacy
 -->
#### 隐私性

<!--
Monikers may contain privacy-sensitive information about other components that
the user is running.
 -->
代称可能包含有关用户正在运行的其他组件的隐私敏感信息。

<!--
To preserve the encapsulation of the system, components should be unable to
determine the identity of other components running outside of their own
realm. Accordingly, monikers are only transmitted on a need-to-know basis
or in an obfuscated form.
 -->
为了保留系统的封装，组件应无法确定在其领域之外运行的其他组件的标识。因此，绰号仅在需要知道时或以混淆的形式传输。

<!--
For example, components are not given information about their own absolute
moniker because it would also reveal information about their parents and
ancestors.
 -->
例如，不会为组件提供关于其自身绝对代称的信息，因为同时也会泄露关于其父辈或祖辈的信息。

<!--
Monikers may be collected in system logs. They are also used to implement the
component framework's persistence features.
 -->
代称可以在系统日志中收集。它们还用于实现组件框架的持久性功能。

<!--
### Usage
 -->
### 用法

<!--
The primary use of monikers is to identify component instances at runtime.
There are three types of component monikers:
 -->
代称的主要用途是在运行时识别组件实例。组件代称有三种类型：

<!--
-   Absolute moniker: Denotes the path from the root of the component instance
    tree to a target component instance.
-   Child moniker: Denotes the path of a child of a component instance relative
    to its parent.
-   Relative moniker: Denotes the path from a source component instance to a
    target component instance.
 -->
-   绝对代称：表示从组件实例树根到目标组件实例的路径。
-   子代称：表示子组件实例相对于其父实例的路径。
-   相对代称：表示从源组件实例到目标组件实例的路径。

<!--
Every component instance has a unique absolute moniker. Consider the following
example component instance tree:
 -->
每个组件实例都有唯一的绝对代称。考虑以下示例组件实例树：

<!--
<br>![Diagram of Absolute Monikers](/reference/components/images/monikers_absolute.png)<br>
 -->
<br>![绝对代称图示](/reference/components/images/monikers_absolute.png)<br>

<!--
-   `/alice:0/carol:0/sandy:0`: Uniquely identifies the component instance
    "sandy" as the descendent of "alice" and "carol".
-   `/alice:0/support:dan:0`: Uniquely identifies the component instance "dan"
    as an element in the "support" collection descended from "alice".
 -->
 - `/alice:0/carol:0/sandy:0`：唯一地将组件实例“sandy”标识为“alice”和“carol”的后代。
 - `/alice:0/support:dan:0`：唯一地将组件实例“dan”标识为“alice”后代的“support”集合中的元素。

<!--
Note: Both components could resolve from the same **component URL**, but since
they are two different instances at runtime they have different **monikers**.
 -->
注意：两个组件都可以从相同的**组件网址**解析，但是由于它们在运行时是两个不同的实例，因此具有不同的**代称**。

<!--
Monikers are used by [developer tools][component-select] to interact with
component instances on a target device.
 -->
[开发人员工具][component-select]使用代称与目标设备上的组件实例进行交互。

<!--
For more details on component moniker syntax, see the
[component moniker reference][moniker-reference].
 -->
要获取关于组件代称语法的更多细节，请参阅[组件代称参考][moniker-reference]。

[glossary.component-url]: /glossary/README.md#component-url
[glossary.moniker]: /glossary/README.md#moniker
[component-select]: /development/tools/ffx/commands/component-select.md
[doc-manifests-children]: https://fuchsia.dev/reference/cml#children
[doc-manifests-declaration]: /concepts/components/v2/component_manifests.md#component-declaration
[doc-package]: /concepts/packages/package.md
[doc-subpackaging]: /concepts/components/v2/subpackaging.md
[doc-resolvers]: /concepts/components/v2/capabilities/resolvers.md
[moniker-reference]: /reference/components/moniker.md
[url-reference]: /reference/components/url.md
[test-hermeticity]: /development/testing/components/test_runner_framework.md#hermeticity
[wiki-url]: https://en.wikipedia.org/wiki/URL
