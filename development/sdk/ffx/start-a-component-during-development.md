# Start a component during development

For testing and debugging purposes, the
[`ffx component`][ffx-component] commands can quickly
start a [dynamic component instance][dynamic-children] on a device.

## Concepts

Component instances on a device are usually declared using
[component manifests][component-manifests], which statically define the topology
and capabilities of the components on the device. However, statically declaring
a component isn't the only way to create a component instance
on a device. You can also use the `ffx component` commands to create a dynamic
component instance on the device at runtime.

One important difference is that Fuchsia restricts all dynamic
component instances to run under a [component collection][component-collection].
A collection serves as a container for dynamic component instances.
Consequently, the capabilities of component instances
under a collection are limited by the capabilities that the collection is able
to expose and offer.

While a new component collection can be declared using a component manifest
(similar to declaring a static component instance), Fuchsia
provides a number of predefined collections for general usage.
For instance, [`ffx-laboratory`][ffx-laboratory] is one of those predefined
collections.

## Start a component {:#start-a-component}

To create a new dynamic component instance, you first need to run
[`ffx component create`][ffx-component-create] to add a new component
to the component instance tree on a device. Once added,
you can run [`ffx component start`][ffx-component-start] to start the component
on the device.

To start a new dynamic component instance on a device, do the following:

1. Create a new component instance:

   ```posix-terminal
   ffx component create <TARGET_MONIKER> <COMPONENT_URL>
   ```

   Replace the following:

   * `TARGET_MONIKER`: The destination moniker of a new component instance. The
   moniker must include a component collection on the path.
   * `COMPONENT_URL`:  The resource location of a component.

   The example below creates a new component instance for the
   `hello-world-cpp.cm` component and assigns its moniker to be
   `/core/ffx-laboratory:hello-world`:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx component create /core/ffx-laboratory:hello-world fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-cpp.cm
   URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-cpp.cm
   Moniker: /core/ffx-laboratory:hello-world
   Creating component instance...
   ```

   Note: To remove this component instance from the tree, see
   [Destroy a component](#destroy-a-component).

2. Start the component instance:

   ```posixe-terminal
   ffx component start <TARGET_MONIKER>
   ```

   Replace `TARGET_MONIKER` with the moniker used in Step 1.

   The example below starts the component instance at
   `/core/ffx-laboratory:hello-world`:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx component start /core/ffx-laboratory:hello-world
   Moniker: /core/ffx-laboratory:hello-world
   Starting component instance...
   ```

   Note: To stop this component instance, see
   [Stop a component](#stop-a-component).

## Start a component under ffx-laboratory

Starting a dynamic component instance would normally require running a
sequence of the `ffx component create` and `ffx component start` commands
(see [Start a component](#start-a-component)). However,
the [`ffx component run`][ffx-component-run] command can start a dynamic
component instance in a single command line.

Of course, there is a catch to this convenience: the `ffx component run` command
can only start its component instance under the
[`ffx-laboratory`][ffx-laboratory] collection. Keep in mind that the
`ffx-laboratory` collection might not offer all the capabilities required by
your component.

To quick start a component under the `ffx-laboratory` collection,
run the following command:

```posix-terminal
ffx component run <COMPONENT_URL>
```

Replace `COMPONENT_URL` with the resource location of a component.

The example below starts the `hello-world-cpp.cm` component on the device:

```none {:.devsite-disable-click-to-copy}
$ ffx component run fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-cpp.cm
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-cpp.cm
Moniker: /core/ffx-laboratory:hello-world-cpp
Creating component instance...
Starting component instance...
```

In essence, the `ffx component run` command performs the following steps:

1. Run `ffx component create` to create a new component instance under the `ffx-laboratory` collection
   using the component name as the target moniker.
2. Run `ffx component start` to start the component instance on the device.

For instance, running `ffx component run fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-cpp.cm`
in the example above is equivalent to running the following commands:

```none {:.devsite-disable-click-to-copy}
$ ffx component create /core/ffx-laboratory:hello-world-cpp fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-cpp.cm
$ ffx component start /core/ffx-laboratory:hello-world-cpp
```

## Stop a component {:#stop-a-component}

To stop a running component instance on a device, run the following command:

```posix-terminal
ffx component stop <TARGET_MONIKER>
```

Replace `TARGET_MONIKER` with the moniker of a component instance.

The example below stops the component instance at
`/core/ffx-laboratory:hello-world`:

```none {:.devsite-disable-click-to-copy}
$ ffx component stop /core/ffx-laboratory:hello-world
Moniker: /core/ffx-laboratory:hello-world
Stopping component instance...
```

## Destroy a component {:#destroy-a-component}

To remove a dynamic component instance from the component instance tree
on a device, run the following command:

```posix-terminal
ffx component destroy <TARGET_MONIKER>
```

Replace `TARGET_MONIKER` with the moniker of a component instance.

The example below removes the component instance at
`/core/ffx-laboratory:hello-world`:

```none {:.devsite-disable-click-to-copy}
$ ffx component destroy /core/ffx-laboratory:hello-world
Moniker: /core/ffx-laboratory:hello-world
Destroying component instance...
```


<!-- Reference links -->

[dynamic-children]: concepts/components/v2/realms.md#dynamic-children
[component-manifests]: concepts/components/v2/component_manifests.md
[component-collection]: concepts/components/v2/realms.md#collections
[ffx-component-run]: https://fuchsia.dev/reference/tools/sdk/ffx#run
[ffx-laboratory]: development/components/run.md#ffx-laboratory
[ffx-component]: https://fuchsia.dev/reference/tools/sdk/ffx#component
[component-lifecycle]: concepts/components/v2/lifecycle.md
[ffx-component-create]: https://fuchsia.dev/reference/tools/sdk/ffx#create_2
[ffx-component-start]: https://fuchsia.dev/reference/tools/sdk/ffx#start
[ffx-component-stop]: https://fuchsia.dev/reference/tools/sdk/ffx#stop
[ffx-component-destory]: https://fuchsia.dev/reference/tools/sdk/ffx#destroy
[get-the-list-of-components]: ./view-component-information.md#get-the-list-of-components
