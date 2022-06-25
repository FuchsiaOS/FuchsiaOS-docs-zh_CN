# Directory capabilities

<<../../_v2_banner.md>>

[Directory capabilities][glossary.directory-capability] allow components
to connect to a directory provided by another component.

For information on directories that are isolated per-component, see
[storage capabilities][storage-capabilities].

## Providing directory capabilities {#provide}

To provide a directory capability, a component must declare the capability and
[route](#route) it from `self`. The component hosts the directory capability in
its [outgoing directory][glossary.outgoing-directory].

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
[rights](#directory-capability-rights) are "read-only".

## Routing directory capabilities {#route}

Components route directory capabilities by [exposing](#expose) them to their
parent and [offering](#offer) them to their children.

For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].

### Exposing {#expose}

Exposing a directory capability gives the component's parent access to that
capability:

```json5
{
    expose: [
        {
            directory: "data",
            from: "self",
        },
    ],
}
```

You may optionally specify:

* [`as`](#renaming)
* [`rights`](#directory-capability-rights)
* [`subdir`](#subdirectories)

### Offering {#offer}

Offering a storage capability gives a child component access to that
capability:

```json5
{
    offer: [
        {
            directory: "data",
            from: "parent",
            to: [ "#child-a", "#child-b" ],
        },
    ],
}
```

You may optionally specify:

* [`as`](#renaming)
* [`rights`](#directory-capability-rights)
* [`subdir`](#subdirectories)

## Consuming directory capabilities {#consume}

To consume a storage capability, the component must request the capability and
open the corresponding path in its [namespace][glossary.namespace].

To request the capability, add a `use` declaration for it:

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

This populates the component's namespace with a directory at the provided `path`
containing the shared directory contents.

You must specify [`rights`](#directory-capability-rights).
You may optionally specify [`subdir`](#subdirectories).

## Directory capability rights {#directory-capability-rights}

Directory rights enable components to control access to directories as they are
routed throughout the system. Directory rights are applied as follows:

-   [`capabilities`][manifest-capabilities]: *Required*.
    Provides the base set of rights available for the directory. Any rights
    specified in a `use`, `offer`, or `expose` must be a subset of what is
    declared here.
-   [`use`][manifest-use]: *Required*.
    Describes the access rights requested by the consuming component.
-   [`offer`][manifest-offer]: *Optional*.
    Modified rights available to the destination component. Rights are inherited
    from the `offer` source if not present.
-   [`expose`][manifest-expose]: *Optional*.
    Modified rights available to the destination component. Rights are inherited
    from the `expose` source if not present.

The `rights` field can contain any combination of the following
[`fuchsia.io.Rights`][fidl-io-rights] tokens:

```json5
rights: [
  "connect",
  "enumerate",
  "traverse",
  "read_bytes",
  "write_bytes",
  "execute_bytes",
  "update_attributes",
  "get_attributes",
  "modify_directory",
]
```

The framework provides a simplified form for declaring `rights` using *aliases*.
Each alias represents the combination of FIDL rights tokens to provide common
read, write, or execute access:

| Alias | FIDL rights                                                |
| :---: | ---------------------------------------------------------- |
| `r*`  | `connect, enumerate, traverse, read_bytes,`                |
:       : `get_attributes`                                           :
| `w*`  | `connect, enumerate, traverse, write_bytes,`               |
:       : `update_attributes, modify_directory`                      :
| `x*`  | `connect, enumerate, traverse, execute_bytes`              |
| `rw*` | `connect, enumerate, traverse, read_bytes, write_bytes,`   |
:       : `get_attributes, update_attributes, modify_directory`      :
| `rx*` | `connect, enumerate, traverse, read_bytes, execute_bytes,` |
:       : `get_attributes`                                           :

The `rights` field may only contain one alias. Additional FIDL rights may be
appended as long as they do not duplicate rights expressed by the alias.

### Example

Consider the following example where component `A` requests *read-write* access
to the `data` directory:

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

However, the parent component `B` offers the directory `data` to component `A`
with only *read-only* rights. In this case the routing fails and `data` wouldn't
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

## Subdirectories {#subdirectories}

You may `expose`, `offer`, or `use` a subdirectory of a directory capability:

```json5
{
    offer: [
        {
            directory: "data",
            from: "parent",
            to: [ "#child-a", "#child-b" ],
            subdir: "children",
        },
    ],
}
```

## Renaming directories {#renaming}

You may `expose` or `offer` a directory capability by a different name:

```json5
{
    offer: [
        {
            directory: "data",
            from: "#child-a",
            to: [ "#child-b" ],
            as: "a-data",
        },
    ],
}
```

## Framework directories {#framework}

A *framework directory* is a directory provided by the component framework.
Any component may `use` these capabilities by setting `framework` as the source
without an accompanying `offer` from its parent.
Fuchsia supports the following framework directories:

-   [hub][doc-hub]: Allows a component to perform runtime introspection of
    itself and its children.

```json5
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

[glossary.directory-capability]: glossary/README.md#directory-capability
[glossary.outgoing-directory]: glossary/README.md#outgoing-directory
[capability-routing]: concepts/components/v2/capabilities/README.md#routing
[doc-hub]: concepts/components/v2/hub.md
[fidl-io-rights]: /sdk/fidl/fuchsia.io/rights-abilities.fidl
[manifest-capabilities]: https://fuchsia.dev/reference/cml#capabilities
[manifest-expose]: https://fuchsia.dev/reference/cml#expose
[manifest-offer]: https://fuchsia.dev/reference/cml#offer
[manifest-use]: https://fuchsia.dev/reference/cml#use
[routing-example]: /examples/components/routing
[storage-capabilities]: concepts/components/v2/capabilities/storage.md
