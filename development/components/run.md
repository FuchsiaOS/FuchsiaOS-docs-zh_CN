# Run components

This document demonstrates how to add components directly to the component
instance tree during development and interact with them at runtime.

Fuchsia provides a few abstractions on top of [component framework][glossary.component-framework]
for specific use cases. If you are building components using one of the
following frameworks, refer to the corresponding guides instead:

*   **Session components:** [Building and running a session][run-session]
*   **Test components:** [Run Fuchsia tests][run-test]

Note: For more details on the commands described in this guide, see the
[`ffx component` reference][ffx-reference].

## Concepts {#concepts}

You should understand the following concepts before running a component:

*   At runtime, the [component instance tree][glossary.component-instance-tree]
    connects individual [component instances][glossary.component-instance]
    together in a hierarchy of parent and child relationships.
*   Component instances progress through four major lifecycle states: create,
    start, stop, and destroy.
*   A [component moniker][glossary.moniker] identifies component instances
    within the tree using their topological path.
*   Component instances are declared **statically** as a
    [child][manifest-children] of another component in their
    [component manifest][glossary.component-manifest] or created **dynamically**
    at runtime in a [component collection][manifest-collections]. Each instance
    consists of a component `name` and `url`.
*   A [component URL][glossary.component-url] identifies a component. Component
    URLs are resolved by the component framework, often to a resource inside a
    package.

For more details on component execution, see
[Component lifecycle][lifecycle-doc].

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

Replace `COMPONENT_NAME` with the name of a component. The following example
shows the command output for the `pkg-resolver` component:

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

Dynamic components are *created* at runtime inside of a collection. You can use
`ffx component create` to create a new component instance, providing a target
moniker within an existing collection and a component URL for resolving the
component:

```posix-terminal
ffx component create {{ '<var label="moniker">TARGET_MONIKER</var>' }} {{ '<var label="url">COMPONENT_URL</var>' }}
```

Replace `TARGET_MONIKER` with the destination moniker of the new component
inside an existing collection and `COMPONENT_URL` with the location where the
component is being served. For example, the following command creates a new
component instance named `hello-world` inside the `ffx-laboratory` collection:

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

Replace `TARGET_MONIKER` with the moniker of the component to destroy. The
following example destroys the `hello-world` component created above:

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

Replace `TARGET_MONIKER` with the moniker of the component to start. The
following example starts the `hello-world` component created previously:

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

Replace `TARGET_MONIKER` with the moniker of the component to stop. The
following example stops to the `hello-world` component started above:

```none {:.devsite-disable-click-to-copy}
$ ffx component stop /core/ffx-laboratory:hello-world
Moniker: /core/ffx-laboratory:hello-world
Stopping component instance...
```

Note: You can add the `--recursive` flag to stop all child components. For more
details, see the [`ffx component` reference][ffx-reference].

### Run a component {#run}

The `ffx component run` command provides a quickstart to run basic components
during development. It is a shortcut for `ffx component create` followed by
`ffx component start`:

```posix-terminal
ffx component run {{ '<var label="moniker">TARGET_MONIKER</var>' }} {{ '<var label="url">COMPONENT_URL</var>' }}
```

Replace `TARGET_MONIKER` with the destination moniker of the new component
inside an existing collection and `COMPONENT_URL` with the location where the
component is being served. For example, the following command creates a new
component instance named `hello-world` inside the `ffx-laboratory` collection:

```none {:.devsite-disable-click-to-copy}
$ ffx component run /core/ffx-laboratory:hello-world fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
Moniker: /core/ffx-laboratory:hello-world-rust
Creating component instance...
Starting component instance...
```

The example above is equivalent to running the following individual `ffx`
commands:

```none {:.devsite-disable-click-to-copy}
$ ffx component create /core/ffx-laboratory:hello-world-rust fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
$ ffx component start /core/ffx-laboratory:hello-world-rust
```

# Ways to update a component {#updating}

When you make changes to your component, you'll often want to update one of its
instances running on the device. For example, you may change the component's
binary, and restart the component to run with the new binary. Or you may change
its manifest to add new capability routes and want to make those capability
routes available on the device.

The `ffx component reload` command is the fastest and most complete way to
reload a component. But it's not the only way and understanding the other
methods may allow more precise operations for special use cases.

The methods are summarized here and explained in full below.

### Summary

| Command              | Description | Updates | Updates  | Preserves |
:                      :             : package : manifest : resources :
| -------------------- | ----------- | ------- | -------- | --------- |
| ffx component reload | stops,      | yes     | yes      | yes       |
:                      : updates,    :         :          :           :
:                      : and starts  :         :          :           :
| ffx component        | destroys,   | yes     | yes      | no        |
: destroy/create/start : then starts :         :          :           :
| ffx component run    | destroys,   | yes     | yes      | no        |
: --recreate           : then starts :         :          :           :
| ffx component        | stops and   | yes     | no       | yes       |
: stop/start           : starts      :         :          :           :
:                      : without     :         :          :           :
:                      : destroying  :         :          :           :

> *   "Updates package" means that the code is updated when the package is
>     reloaded.
> *   "Updates manifest" means that the manifest cache is reloaded, updating the
>     routing and other information contained in the FIDL files.
> *   "Preserves resources" means that resources such as storage that would be
>     released by a destroy command are instead preserved.

### Reload {#reloading}

Update your component's code and manifest while retaining resources with:

```posix-terminal
ffx component reload {{ '<var label="moniker">TARGET_MONIKER</var>' }}
```

This command will first shut down the component, then reload and restart it. The
command updates the package and manifest without destroying the component or
releasing resources.

The `reload` command preserves your component's resources such as storage. This
preservation can be helpful if it is slow to initialize, acquire, or recreate
resources in a specific state for debugging.

Reloading is also faster when destroying the component is expensive, such as
when it requires shutting down and restarting a session or the target
device/emulator.

### Destroy/create/start {#destroy_create_stop}

To fully reload the component and drop acquired resources, you can first destroy
the existing component instance, then restart it. Use:

```none {:.devsite-disable-click-to-copy}
$ ffx component destroy {{ '<var label="moniker">TARGET_MONIKER</var>' }}
$ ffx component create {{ '<var label="moniker">TARGET_MONIKER</var>' }} {{ '<var label="url">COMPONENT_URL</var>' }}
$ ffx component start {{ '<var label="moniker">TARGET_MONIKER</var>' }}
```

This sequence will reload both the package and the manifest, so code and
capability changes will be updated. However, destroying the component will also
free any resources it uses. This full reset may be what you want if your goal is
to start with a completely reinitialized component.

### Run --recreate {#run_recreate}

A convenient command that is analogous to the destroy/create/start sequence is
the `run` command with `--recreate`:

```posix-terminal
ffx component run {{ '<var label="moniker">TARGET_MONIKER</var>' }} {{ '<var label="url">COMPONENT_URL</var>' }} --recreate
```

### Stop/start {#start_stop}

Although it's not primarily a way to do updates, a side effect of just stopping,
then starting your component is that it will be partially updated.

```none {:.devsite-disable-click-to-copy}
$ ffx component stop {{ '<var label="moniker">TARGET_MONIKER</var>' }}
$ ffx component start {{ '<var label="moniker">TARGET_MONIKER</var>' }}
```

Assuming a package manager such as `ffx serve` is running, the latest version of
the component's code will be loaded and run. However, due to the way caching
works in the Fuchsia component framework, the manifest will not be updated. The
manifest contains your component's routing and other information as defined in
the `*.cm` files. So if you change your component's capability routes, stopping
and starting the component will not pick up these changes.

## `ffx-laboratory` {#ffx-laboratory}

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
Component instances in this collection will persist even after they stop. To
destroy a component instance in this collection, use the `ffx component destroy`
command.

## Troubleshooting

This section contains common issues you may encounter while running your
components during development.

### Unable to resolve the component

When using `ffx component start` or `ffx component run` you may encounter the
following error if component framework cannot resolve the component instance:

```none {:.devsite-disable-click-to-copy}
$ ffx component run /core/ffx-laboratory:hello-world fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm
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
$ ffx component run /core/ffx-laboratory:hello-world fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm
Moniker: /core/ffx-laboratory:hello-world
Creating component instance...
Component instance already exists. Use --recreate to destroy and recreate a new instance, or --name to create a new instance with a different name.
```

This occurs when the target moniker is already in use by another component
instance.

To address this issue, manually destroy the instance using the `ffx component
destroy` command:

```none {:.devsite-disable-click-to-copy}
$ ffx component destroy /core/ffx-laboratory:hello-world
Moniker: /core/ffx-laboratory:hello-world
Destroying component instance...
```

If you are using `ffx component run`, add the `--recreate` flag to destroy the
instance and recreate it:

```none {:.devsite-disable-click-to-copy}
$ ffx component run /core/ffx-laboratory:hello-world fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm --recreate
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm
Moniker: /core/ffx-laboratory:hello-world
Creating component instance...
Component instance already exists. Destroying...
Recreating component instance...
Starting component instance...
```

Alternatively, add the `--name` flag to create a new instance with a different
name:

```none {:.devsite-disable-click-to-copy}
$ ffx component run /core/ffx-laboratory:hello-world fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm --name hello-world-2
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world.cm
Moniker: /core/ffx-laboratory:hello-world-2
Creating component instance...
Starting component instance...
```

[capability-directory]: /concepts/components/v2/capabilities/directory.md
[capability-protocol]: /concepts/components/v2/capabilities/protocol.md
[capability-storage]: /concepts/components/v2/capabilities/storage.md
[component-select]: /development/tools/ffx/commands/component-select.md
[component-url]: /reference/components/url.md
[fidl-launcher]: https://fuchsia.dev/reference/fidl/fuchsia.process#Launcher
[fidl-logger]: https://fuchsia.dev/reference/fidl/fuchsia.logger#LogSink
[ffx-reference]: https://fuchsia.dev/reference/tools/sdk/ffx.md#component
[glossary.capability]: /glossary/README.md#capability
[glossary.component-framework]: /glossary/README.md#component-framework
[glossary.component-instance]: /glossary/README.md#component-instance
[glossary.component-instance-tree]: /glossary/README.md#component-instance-tree
[glossary.component-manifest]: /glossary/README.md#component-manifest
[glossary.component-url]: /glossary/README.md#component-url
[glossary.moniker]: /glossary/README.md#moniker
[lifecycle-doc]: /concepts/components/v2/lifecycle.md
[manifest-children]: https://fuchsia.dev/reference/cml#children
[manifest-collections]: https://fuchsia.dev/reference/cml#collections
[package-server]: /concepts/packages/package_update.md#connecting_host_and_target
[package-updates]: /concepts/packages/package_update.md#triggering_package_updates
[run-session]: /development/sessions/building-and-running-a-session.md
[run-test]: /development/testing/run_fuchsia_tests.md
