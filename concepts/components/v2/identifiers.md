# Component identifiers

The Component Framework uses different identifiers to describe components.
This section describes the relationship between the following component
identifiers, and their usage:

-   [Component URLs](#component-urls): Identifies a component as a resource to
    be fetched by a component resolver.
-   [Monikers](#monikers): Identifies specific component instances in the
    component instance tree.

## Component URLs {#component-urls}

A [component URL][glossary.component-url] is a [URL][wiki-url] that locates a
component, including its declaration, program, and assets. Component Framework
uses [component resolvers][doc-resolvers] to resolve a component URL into a
[component declaration][doc-manifests-declaration].

### Usage

The primary use of component URLs is to identify a component in the definition
of a component instance, as part of a [child declaration][doc-manifests-children]:

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

The above example declares the `logger` component as an absolute resource
in a [Fuchsia package][doc-package] hosted in a package repository.

Component Framework also supports relative URLs.

To identify a component built into the same package as the parent component,
specify only the URL fragment:

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

To identify a component in a [subpackage][doc-subpackaging] of the parent
component's package, include the subpackage name followed by the component
manifest path (via URL fragment):

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

Relative component URLs are often used in tests, where the best practice is to
re-package production components in a test-specific package to promote
[hermeticity][test-hermeticity].

For more details on component URL syntax, see the
[component URL reference][url-reference].

## Monikers {#monikers}

A [component moniker][glossary.moniker] identifies a specific component instance
in the component instance tree using a topological path.

### Design principles

#### Stability

Monikers are stable identifiers. Assuming the component topology does not
change, the monikers used to identify component instances in the topology
will remain the same.

#### Uniqueness

Each time a component instance is destroyed and a new component instance with
the same name is created in its place in the component topology (as a child
of the same parent), the new instance is assigned a unique instance identifier
to distinguish it from prior instances in that place.

Monikers include unique instance identifiers to prevent confusion of old
component instances with new component instances of the same name as the
tree evolves.

#### Privacy

Monikers may contain privacy-sensitive information about other components that
the user is running.

To preserve the encapsulation of the system, components should be unable to
determine the identity of other components running outside of their own
realm. Accordingly, monikers are only transmitted on a need-to-know basis
or in an obfuscated form.

For example, components are not given information about their own absolute
moniker because it would also reveal information about their parents and
ancestors.

Monikers may be collected in system logs. They are also used to implement the
component framework's persistence features.

### Usage

The primary use of monikers is to identify component instances at runtime.
There are three types of component monikers:

-   Absolute moniker: Denotes the path from the root of the component instance
    tree to a target component instance.
-   Child moniker: Denotes the path of a child of a component instance relative
    to its parent.
-   Relative moniker: Denotes the path from a source component instance to a
    target component instance.

Every component instance has a unique absolute moniker. Consider the following
example component instance tree:

<br>![Diagram of Absolute Monikers](/reference/components/images/monikers_absolute.png)<br>

-   `/alice:0/carol:0/sandy:0`: Uniquely identifies the component instance
    "sandy" as the descendent of "alice" and "carol".
-   `/alice:0/support:dan:0`: Uniquely identifies the component instance "dan"
    as an element in the "support" collection descended from "alice".

Note: Both components could resolve from the same **component URL**, but since
they are two different instances at runtime they have different **monikers**.

Monikers are used by [developer tools][component-select] to interact with
component instances on a target device.

For more details on component moniker syntax, see the
[component moniker reference][moniker-reference].

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
