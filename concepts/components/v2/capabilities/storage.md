# Storage capabilities

[Storage capabilities][glossary.storage-capability] allocate per-component
*isolated* storage within a filesystem directory. This prevents component
instances from accessing files belonging to other components, including their
own children.

Different storage capabilities may be backed by different filesystems. A
component should not assume atomic IO operations are possible across storage
capabilities.

For information on directories that can be shared between components, see
[directory capabilities][directory-capabilities].

## Standard storage capability names {#standard-names}

Standard names are commonly used for storage capabilities. Each of these
standard names implies the storage capability should be used for a particular
purpose and provides a particular behavior. Any component that receives a
storage capability with one of these standard names may assume it provides the
behavior described below.

Note that a storage capability name does **not** necessarily globally identify a
storage capability. For example, on some products several different storage
capabilities named `data` exist at different locations in the component instance
topology. These storage capabilities are backed by different directories on
different storage volumes, but they all serve the same purpose for the component
instances using them.

Not all storage capabilities use one of these standard names. In these cases any
expectations about the behavior of the storage capability should be documented
where the storage capability is defined and at every place in the component
instance topology that the capability is renamed.

Note that during tests storage capabilities may be created that do not match
these behaviors. For example, an integration test may provide a `data`
capability that is wiped between test cases.

### `data`

Storage capabilities named "data" are intended to store general purpose
persistent data.

A component may assume that files in these storage capabilities
will not be deleted by the system. Components must be conservative in their use
of `data` because the contract does not let system delete files when the
limited disk space is exhausted. In many cases using `cache` is preferable.

### `cache`

Storage capabilities named "cache" are intended to store data that could be
discarded or regenerated if necessary. For example, a downloaded picture that
could be re-fetched.

Files stored in `cache` are usually persisted between different runs of same
component instance but this is not guaranteed. Files may be deleted by the
system at any time, even while the component is running.

### `tmp`

Storage capabilities named "tmp" are intended to store temporary or intermediate
data.

Files stored in `tmp` may be deleted by the system between runs of a component.
Files will not be deleted by the system while the component is running. `tmp`
will often be empty when a component is started but this is not guaranteed.
Components must not assume `tmp` will be empty on start but also should not use
any files that are present on start.

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
256-bit globally unique identifier listed in a component ID index file.

The following is an example entry in a component ID index file containing a
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
Using a moniker is a good secondary option for tests or other use
cases where storage does not need to be durable.

For more details on instance IDs, see [Component ID index][component-id-index].

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

### Consuming optional storage capabilities

See [Connect Components: Consuming optional capabilities][consuming-optional-capabilities].

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

[glossary.directory-capability]: /docs/glossary/README.md#directory-capability
[glossary.component-instance-identifier]: /docs/glossary/README.md#component-instance-identifier
[glossary.moniker]: /docs/glossary/README.md#moniker
[glossary.namespace]: /docs/glossary/README.md#namespace
[glossary.storage-capability]: /docs/glossary/README.md#storage-capability
[capability-routing]: /docs/concepts/components/v2/capabilities/README.md#routing
[consuming-optional-capabilities]: /docs/development/components/connect.md#consuming-optional-capabilities
[component-reference]: https://fuchsia.dev/reference/cml#references
[directory-capabilities]: /docs/concepts/components/v2/capabilities/directory.md
[component-id-index]: /docs/development/components/component_id_index.md
