# Storage capabilities

<<../../_v2_banner.md>>

[Storage capabilities][glossary.storage-capability] allocate per-component
*isolated* storage within a filesystem directory. This prevents component
instances from accessing files belonging to other components, including their
own children.

For information on directories that can be shared between components, see
[directory capabilities][directory-capabilities].

## Backing directories {#backing-dir}

Each storage capability must be backed by a corresponding
[directory capability][glossary.directory-capability] to host an isolated
subdirectory for each component. When a component instance attempts to access
the directory provided to it through a storage capability, the framework
generates a unique subdirectory inside the backing directory for that component.

Caution: The backing directory capability can also be routed directly to other
components. Providing this capability allows components to access all the
isolated storage directories it contains.

The framework allocates storage subdirectories based on either the component
instance's [moniker][glossary.moniker] or a static
[instance ID][glossary.component-instance-identifier]. Each instance ID is a
256-bit globally unique identifier listed in a component storage index file.

The following is an example entry in a component storage index file containing a
stable instance ID:

```json5
{
    instances: [
        {
            instance_id: "47c3bf08f3e560c4dee659c28fa8d863dbdc0b1dbb74065e6cb1f38441ac759c",
            moniker: "/core/my_component",
        },
    ],
}
```

Instance IDs allow a component's storage to persist across changes to the
component's moniker, such as moving the component instance to a different realm.
Storage IDs based on moniker are a good secondary option for tests or other use
cases where storage does not need to be durable.

For more details on instance IDs, see [Component storage index][storage-index].

## Providing storage capabilities {#provide}

To provide a storage capability, a component must declare the capability and
[route](#route) it from `self`.

```json5
{
    capabilities: [
        {
            storage: "tmp",
            from: "self",
            backing_dir: "memfs",
            storage_id: "static_instance_id",
        },
    ],
}
```

You must specify [`backing_dir`](#backing-dir) with a valid directory capability
name.

The `from` field declares the component providing the backing directory.
You may supply a [component reference][component-reference] if the provider is
another component.

## Routing storage capabilities {#route}

Storage capabilities cannot be exposed to a parent component. Components should
route the [backing directory](#backing-dir) to an appropriate parent component
where storage can be [declared](#provide) and [offered](#offer) to the necessary
children.

For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].

### Offering {#offer}

Offering a storage capability gives a child component access to that
capability:

```json5
{
    offer: [
        {
            storage: "data",
            from: "self",
            to: [ "#storage-user" ],
        },
    ],
}
```

## Consuming storage capabilities {#consume}

To consume a storage capability, the component must request the capability and
open the corresponding path in its [namespace][glossary.namespace].

To request the capability, add a `use` declaration for it:

```json5
{
    use: [
        {
            storage: "data",
            path: "/example_dir",
        },
    ],
}
```

This populates the component's namespace with a directory at the provided `path`
containing the isolated storage contents.

## Storage example {#example}

Consider the following example where component `A` requests isolated storage
`tmp` from its parent:

```json5
// A.cml
{
    use: [
        {
            storage: "tmp",
            path: "/example_dir",
        },
    ],
}
```

This provides an isolated storage directory at `/example_dir` in the namespace
of component `A`.
The parent component `B` offers this capability to `A` using a backing directory
provided by the `memfs` component in the same realm:

```json5
// B.cml
{
    capabilities: [
        {
            storage: "tmp",
            from: "#memfs",
            backing_dir: "memfs",
        },
    ],
    offer: [
        {
            storage: "tmp",
            from: "self",
            to: [ "#A" ],
        },
    ],
    children: [
        { name: "A", url: "fuchsia-pkg://...", },
        { name: "memfs", url: "fuchsia-pkg://..." },
    ],
}
```

For more details on implementing directories, see
[directory capabilities][directory-capabilities].

[glossary.directory-capability]: /glossary/README.md#directory-capability
[glossary.component-instance-identifier]: /glossary/README.md#component-instance-identifier
[glossary.moniker]: /glossary/README.md#moniker
[glossary.namespace]: /glossary/README.md#namespace
[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[glossary.storage-capability]: /glossary/README.md#storage-capability
[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[component-reference]: https://fuchsia.dev/reference/cml#references
[directory-capabilities]: /concepts/components/v2/capabilities/directory.md
[storage-index]: /development/components/component_id_index.md
