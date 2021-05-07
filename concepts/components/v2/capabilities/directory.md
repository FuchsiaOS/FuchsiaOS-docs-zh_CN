# Directory capabilities (Components v2)

<<../../_v2_banner.md>>

[Directory capabilities][glossary-directory] allow components to connect to
directories provided by other components.

## Providing directory capabilities

To provide a directory capability, a component must define the capability and
[route](#routing-directory-capabilities) it from `self`. The component hosts the
directory capability in its [outgoing directory][glossary-outgoing].

To define the capability, add a `capabilities` declaration for it:

```json5
{
    capabilities: [
        {
            directory: "data",
            rights: ["r*"],
            path: "/published-data",
        },
    ],
}
```

This defines a capability hosted by this component whose outgoing directory path
is `/published-data`, and whose maximum usable
[rights](#directory-capability-rights) are `r*`.

## Routing directory capabilities

Components route directory capabilities by either:

-   [exposing](#routing-directory-capability-expose) them,
-   or [offering](#routing-directory-capability-offer) them.

When a component wants to make one of its directories available to other
components, it specifies the path of that directory in its
[outgoing directory][glossary-outgoing] in one of the following ways:

### Exposing {#routing-directory-capability-expose}

To [expose][expose] the directory to a parent:

```json5
{
    expose: [
        {
            directory: "data",
            from: "#child-a",
        },
    ],
}
```

Optionally, you may narrow the [rights](#directory-capability-rights) on the
directory:

```json5
{
    expose: [
        {
            directory: "data",
            from: "#child-a",
            rights: ["r*"],
        },
    ],
}
```

### Offering {#routing-directory-capability-offer}

To [offer][offer] a directory to a child:

```json5
{
    offer: [
        {
            directory: "data",
            from: "self",
            to: [ "#child-a", "#child-b" ],
        },
    ],
}
```

Optionally, you may narrow the [rights](#directory-capability-rights) on the
directory:

```json5
{
    offer: [
        {
            directory: "data",
            from: "self",
            rights: ["rw*"],
            to: [ "#child-a", "#child-b" ],
        },
    ],
}
```

## Consuming directory capabilities

When a component wants to make use of a directory from its parent, it does so by
[using][use] the directory. This will make the directory accessible from the
component's [namespace][glossary-namespace].

This example shows a directory named `data` that is included in the component's
namespace. If the component instance accesses this directory during its
execution, the component framework performs
[capability routing][capability-routing] to find the component that provides it.
Then, the framework connects the directory from the component's namespace to
this provider.

```json5
{
    use: [
        {
            directory: "data",
            rights: ["r*"],
            path: "/data",
        },
    ],
}
```

`rights` must be a subset of the [rights](#directory-capability-rights) attached
to the directory.

See [`//examples/components/routing`][routing-example] for a working example of
routing a directory capability from one component to another.

## Directory capability rights {#directory-capability-rights}

As directories are [offered][offer] and [exposed][expose] throughout the system
a user may want to restrict what components who have access to this directory
may do. For example, a component could expose a directory as read-write to its
parent realm, which could expose that directory it to its children as read-write
but to its parent as read-only.

[Directory rights][directory-rights] allow any directory declaration to specify
a rights field that indicates the set of rights that the directory would like to
[offer][offer], [expose][expose] or [use][use].

### Example

This example shows component `A` requesting access to `data` with read-write
rights:

```json5
// A.cml
{
    use: [
        {
            directory: "data",
            rights: ["rw*"],
            path: "/data",
        },
    ],
}
```

Furthermore, parent component `B` offers the directory `data` to component A but
with only read-only rights. In this case the routing fails and `data` wouldn't
be present in A's namespace.

```json5
// B.cml
{
    capabilities: [
        {
            directory: "data",
            rights: ["r*"],
            path: "/published-data",
        },
    ],
    offer: [
        {
            directory: "data",
            from: "self",
            to: [ "#A" ],
        },
    ],
}
```

### Inference Rules

Directory rights are required in the following situations:

-   [use][use] - All directories use statements must specify their directory
    rights.
-   [capability][capability] - All `directory` `capability` declarations must
    specify rights.

If an expose or offer directory declaration does not specify optional rights, it
will inherit the rights from the source of the expose or offer. Rights specified
in a `use`, `offer`, or `expose` declaration must be a subset of the rights set
on the capability's source.

### Framework directory capabilities

Some directory capabilities are available to all components through the
framework. When a component wants to use one of these directories, it does so by
[using][use] the directory with a source of `framework`.

```
{
    use: [
        {
            directory: "hub",
            from: "framework",
            rights: ["r*"],
            path: "/hub",
        },
    ],
}
```

[capability-routing]: ../component_manifests.md#capability-routing
[directory-rights]: ../component_manifests.md#directory-rights
[expose]: ../component_manifests.md#expose
[glossary-directory]: /docs/glossary.md#directory-capability
[glossary-fidl]: /docs/glossary.md#fidl
[glossary-namespace]: /docs/glossary.md#namespace
[glossary-outgoing]: /docs/glossary.md#outgoing-directory
[offer]: ../component_manifests.md#offer
[routing-example]: /examples/components/routing
[use]: ../component_manifests.md#use
[capability]: ../component_manifests.md#capability
