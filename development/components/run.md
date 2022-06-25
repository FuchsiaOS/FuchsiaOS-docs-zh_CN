# Run components

This document demonstrates how to add components directly to the component
instance tree during development and interact with them at runtime.

Fuchsia provides a few abstractions on top of
[component framework][glossary.component-framework] for specific
use cases. If you are building components using one of the following frameworks,
refer to the corresponding guides instead:

*   **Session components:** [Building and running a session][run-session]
*   **Test components:** [Run Fuchsia tests][run-test]

Note: For more details on the commands described in this guide, see the
[`ffx component` reference][ffx-reference].

## Concepts {#concepts}

You should understand the following concepts before running a component:

*   At runtime,
    the [component instance tree][glossary.component-instance-tree]
    connects individual [component instances][glossary.component-instance]
    together in a hierarchy of parent and child relationships.
*   Component instances progress through four major lifecycle states:
    create, start, stop, and destroy.
*   A [component moniker][glossary.moniker] identifies component instances
    within the tree using their topological path.
*   Component instances are declared **statically** as a
    [child][manifest-children] of another component in their
    [component manifest][glossary.component-manifest] or created **dynamically**
    at runtime in a [component collection][manifest-collections].
    Each instance consists of a component `name` and `url`.
*   A [component URL][glossary.component-url] identifies a component.
    Component URLs are resolved by the component framework, often to a resource
    inside a package.

For more details on component execution, see [Component lifecycle][lifecycle-doc].

## Component instances {#instances}

The first step to running a component is adding a new component instance to the
tree. The position of the component instance within the tree determines its
available [capabilities][glossary.capability].

### Discover static components

Static components are *declared* as children of another component instance in
the tree. You can use `ffx component show` to determine the moniker and
component URL of a static component instance:

```posix-terminal
ffx component show {{ '<var label="component">COMPONENT_NAME</var>' }}
```

Replace `COMPONENT_NAME` with the name of a component.
The following example shows the command output for the `pkg-resolver` component:

```none {:.devsite-disable-click-to-copy}
$ ffx component show pkg-resolver
               {{ '<strong>' }}Moniker: /core/pkg-resolver{{ '</strong>' }}
                   {{ '<strong>' }}URL: fuchsia-pkg://fuchsia.com/pkg-resolver#meta/pkg-resolver.cm{{ '</strong>' }}
                  Type: CML static component
       Component State: Resolved
       Execution State: Running
...
```

Static component instances cannot be created or destroyed at runtime.

### Manage dynamic components

Dynamic components are *created* at runtime inside of a collection.
You can use `ffx component create` to create a new component instance, providing
a target moniker within an existing collection and a component URL for resolving
the component:

```posix-terminal
ffx component create {{ '<var label="moniker">TARGET_MONIKER</var>' }} {{ '<var label="url">COMPONENT_URL</var>' }}
```

Replace `TARGET_MONIKER` with the destination moniker of the new component
inside an existing collection and `COMPONENT_URL` with the location where the
component is being served.
For example, the following command creates a new component instance inside the
`ffx-laboratory` collection named `hello-world`:

```none {:.devsite-disable-click-to-copy}
$ ffx component create /core/ffx-laboratory:hello-world fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
Moniker: /core/ffx-laboratory:hello-world
Creating component instance...
```

Similarly, use `ffx component destroy` to destroy a dynamic component instance
by providing its moniker:

```posix-terminal
ffx component destroy {{ '<var label="moniker">TARGET_MONIKER</var>' }}
```

Replace `TARGET_MONIKER` with the moniker of the component to destroy.
The following example destroys the `hello-world` component created above:

```none {:.devsite-disable-click-to-copy}
$ ffx component destroy /core/ffx-laboratory:hello-world
Moniker: /core/ffx-laboratory:hello-world
Destroying component instance...
```

## Component execution {#execute}

Once a component instance exists in the tree, you can start and stop the target
instance using `ffx component`.

### Start the instance

Use `ffx component start` to explicitly start a component instance:

```posix-terminal
ffx component start {{ '<var label="moniker">TARGET_MONIKER</var>' }}
```

Replace `TARGET_MONIKER` with the moniker of the component to start.
The following example starts the `hello-world` component created previously:

```none {:.devsite-disable-click-to-copy}
$ ffx component start /core/ffx-laboratory:hello-world
Moniker: /core/ffx-laboratory:hello-world
Starting component instance...
```

### Stop the instance

Use `ffx component stop` to terminate execution of a running component instance
using its moniker:

```posix-terminal
ffx component stop {{ '<var label="moniker">TARGET_MONIKER</var>' }}
```

Replace `TARGET_MONIKER` with the moniker of the component to stop.
The following example stops to the `hello-world` component started above:

```none {:.devsite-disable-click-to-copy}
$ ffx component stop /core/ffx-laboratory:hello-world
Moniker: /core/ffx-laboratory:hello-world
Stopping component instance...
```

Note: You can add the `--recursive` flag to stop all child components.
For more details, see the [`ffx component` reference][ffx-reference].

## Run a component {#run}

The `ffx component run` command provides a quickstart to run basic components
during development:

```posix-terminal
ffx component run {{ '<var label="url">COMPONENT_URL</var>' }}
```

Replace `COMPONENT_URL` with the location where the component is being served.
The following example creates a component instance using the `hello-world-rust`
component:

```none {:.devsite-disable-click-to-copy}
$ ffx component run fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
Moniker: /core/ffx-laboratory:hello-world-rust
Creating component instance...
Starting component instance...
```

The `ffx component run` command automates the following steps:

1.  Create a new component instance in the [`ffx-laboratory`](#ffx-laboratory)
    collection, using the component name as the target moniker.
1.  Start the new instance to begin execution.

The example above is equivalent to running the following individual `ffx` commands:

```none {:.devsite-disable-click-to-copy}
$ ffx component create /core/ffx-laboratory:hello-world-rust fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
$ ffx component start /core/ffx-laboratory:hello-world-rust
```

### `ffx-laboratory` {#ffx-laboratory}

The `ffx-laboratory` is a component collection that provides a restricted set of
capabilities for development. The following capabilities are offered to
components in this collection:

*   [Protocol capabilities][capability-protocol]
    *   [`fuchsia.logger.LogSink`][fidl-logger]: Record log messages
    *   [`fuchsia.process.Launcher`][fidl-launcher]: Create new processes
*   [Storage capabilities][capability-storage]
    *   `tmp`: Temporary storage (non-persistent)
    *   `data`: Emulated persistent storage backed by `/tmp`
    *   `cache`: Emulated cache storage backed by `/tmp`
*   [Directory capabilities][capability-directory]
    *   `/dev`: Device driver `devfs` provided by Driver Manager
    *   `/boot`: Read-only `bootfs` provided by Component Manager

The `ffx-laboratory` is a [`transient`][manifest-collections] collection.
Component instances in this collection will persist even after they stop.
To destroy a component instance in this collection, use the `ffx component destroy` command.

## Troubleshooting

This section contains common issues you may encounter while running your
components during development.

### Unable to resolve the component

When using `ffx component start` or `ffx component run` you may encounter the
following error if component framework cannot resolve the component instance:

```none {:.devsite-disable-click-to-copy}
$ ffx component run fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm
Moniker: /core/ffx-laboratory:hello-world
Creating component instance...
Starting component instance...
Lifecycle protocol could not bind to component instance: InstanceCannotResolve
```

This occurs when the component URL does not resolve to a valid component
manifest.

To address this issue, verify the following:

*   The [component URL][component-url] is formatted correctly.
*   You have a [package server running][package-server].
*   Your package server is registered with the target.
*   Your [component is published][package-updates] to the package server.

### Component instance already exists

When using `ffx component create` or `ffx component run` you may encounter the
following error if the component instance already exists:

```none {:.devsite-disable-click-to-copy}
$ ffx component run fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm
Moniker: /core/ffx-laboratory:hello-world
Creating component instance...
Component instance already exists. Use --recreate to destroy and recreate a new instance, or --name to create a new instance with a different name.
```

This occurs when the target moniker is already in use by another component
instance.

To address this issue, manually destroy the instance using the `ffx component destroy` command:

```none {:.devsite-disable-click-to-copy}
$ ffx component destroy /core/ffx-laboratory:hello-world
Moniker: /core/ffx-laboratory:hello-world
Destroying component instance...
```

If you are using `ffx component run`, add the `--recreate` flag to destroy the instance and
recreate it:

```none {:.devsite-disable-click-to-copy}
$ ffx component run fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm --recreate
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm
Moniker: /core/ffx-laboratory:hello-world
Creating component instance...
Component instance already exists. Destroying...
Recreating component instance...
Starting component instance...
```

Alternatively, add the `--name` flag to create a new instance with a different name:

```none {:.devsite-disable-click-to-copy}
$ ffx component run fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm --name hello-world-2
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm
Moniker: /core/ffx-laboratory:hello-world-2
Creating component instance...
Starting component instance...
```

[capability-directory]: /docs/concepts/components/v2/capabilities/directory.md
[capability-protocol]: /docs/concepts/components/v2/capabilities/protocol.md
[capability-storage]: /docs/concepts/components/v2/capabilities/storage.md
[component-select]: /docs/development/tools/ffx/commands/component-select.md
[component-url]: /docs/reference/components/url.md
[fidl-launcher]: https://fuchsia.dev/reference/fidl/fuchsia.process#Launcher
[fidl-logger]: https://fuchsia.dev/reference/fidl/fuchsia.logger#LogSink
[ffx-reference]: https://fuchsia.dev/reference/tools/sdk/ffx.md#component
[glossary.capability]: /docs/glossary/README.md#capability
[glossary.component-instance]: /docs/glossary/README.md#component-instance
[glossary.component-instance-tree]: /docs/glossary/README.md#component-instance-tree
[glossary.component-manifest]: /docs/glossary/README.md#component-manifest
[glossary.component-url]: /docs/glossary/README.md#component-url
[glossary.moniker]: /docs/glossary/README.md#moniker
[lifecycle-doc]: /docs/concepts/components/v2/lifecycle.md
[manifest-children]: https://fuchsia.dev/reference/cml#children
[manifest-collections]: https://fuchsia.dev/reference/cml#collections
[package-server]: /docs/concepts/packages/package_update.md#connecting_host_and_target
[package-updates]: /docs/concepts/packages/package_update.md#triggering_package_updates
[run-session]: /docs/development/sessions/building-and-running-a-session.md
[run-test]: /docs/development/testing/run_fuchsia_tests.md