# Storage capabilities (Components v2)

<<../../_v2_banner.md>>

[Storage capabilities][glossary-storage] are a way for components to receive
_isolated_ access to a private storage directory. When a storage capability is
declared in a [component manifest][manifests-storage] it must reference a
backing [directory capability][directory-capabilities]. Each component that then
[uses][use] this storage capability receives a unique and non-overlapping
subdirectory within the backing directory. This prevents [component
instances][component-instance] from accessing files belonging to other component
instances (including their own children).

## Directory vs storage capabilities

As an example, if component instance `a` receives a _directory_ capability from
its [realm][realm] and both [uses][use] it and [offers][offer] it to `b`, which
also uses the directory, both component instances can see and interact with the
same directory.

```
<a's realm>
    |
    a
    |
    b

a.cml:
{
    use: [
        {
            directory: "example_dir",
            rights: ["rw*"],
            path: "/example_dir",
        },
    ],
    offer: [
        {
            directory: "example_dir",
            from: "parent",
            to: [ "#b" ],
        },
    ],
}

b.cml:
{
    use: [
        {
            directory: "example_dir",
            rights: ["rw*"],
            path: "/example_dir",
        },
    ],
}
```

In this example if component instance `a` creates a file named `hippos` inside
`/example_dir` then `b` will be able to see and read this file.

If the component instances use storage capabilities instead of directory
capabilities, then component instance `b` cannot see and read the `hippos` file.

```
<a's realm>
    |
    a
    |
    b

a.cml:
{
    use: [
        {
            storage: "data",
            path: "/example_dir",
        },
    ],
    offer: [
        {
            storage: "data",
            from: "parent",
            to: [ "#b" ],
        },
    ],
}

b.cml:
{
    use: [
        {
            storage: "data",
            path: "/example_dir",
        },
    ],
}
```

In this example any files that `a` creates are not be visible to `b`, as storage
capabilities provide unique non-overlapping directories to each component
instance.

## Creating storage capabilities

Storage capabilities can be created with a
[`storage` declaration][storage-syntax] in a [component manifest][manifests].
Once storage capabilities have been declared, they can then be offered to other
component instances.

A `storage` declaration must include a reference to a directory capability,
which is the directory from which the component manager will create isolated
directories for each component instance using the storage capability.

For example, the following manifest describes a new storage capability named
`temp` backed by the `memfs` directory exposed by the child named `#memfs`. From
this storage declaration a storage capability is offered to the child named
`storage_user`.

```
{
    capabilities: [
        {
            storage: "temp",
            from: "#memfs",
            backing_dir: "memfs",
        },
    ],
    offer: [
        {
            storage: "temp",
            from: "self",
            to: [ "#storage-user" ],
        },
    ],
    children: [
        { name: "memfs", url: "fuchsia-pkg://..." },
        { name: "storage-user", url: "fuchsia-pkg://...", },
    ],
}
```

## Storage capability semantics

A directory capability that backs storage capabilities can be used to access the
files of any component that uses the resulting storage capabilities. This type
of directory capability should be routed carefully to avoid exposing this
capability to too many component instances.

When a component instance attempts to access the directory provided to it
through a storage capability, the framework binds to and generates
sub-directories in the component instance that provides the backing directory
capability. Then, the framework provides the component instance access to a
unique subdirectory.

The subdirectory to which a component instance is provided access is determined
by its location in the component topology. This means that if a component
instance is renamed in its parent manifest or moved to a different parent then
it will receive a different subdirectory than it did before the change.

[component-instance]: /docs/glossary.md#component-instance
[directory-capabilities]: /docs/glossary.md#directory-capability
[glossary-storage]: /docs/glossary.md#storage-capability
[manifests]: /docs/concepts/components/v2/component_manifests.md
[manifests-storage]: /docs/concepts/components/v2/component_manifests.md#capability-storage
[offer]: /docs/glossary.md#offer
[outgoing-directory]: /docs/concepts/system/abi/system.md#outgoing_directory
[realm]: /docs/glossary.md#realm
[storage-syntax]: /docs/concepts/components/v2/component_manifests.md#storage
[use-syntax]: /docs/concepts/components/v2/component_manifests.md#use
[use]: /docs/glossary.md#use
