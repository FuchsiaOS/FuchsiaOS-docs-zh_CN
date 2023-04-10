# Copy files to and from a component

The `ffx component copy` command can copy files to and from Fuchsia components running
on a target device.

## Concepts

The `ffx component copy` command can transfer files in the following ways:

-  From a component on the device to the host machine.
-  From the host machine to a component on the device.
-  Between components on the  device.

Capabilities such as [storage][storage-capabilities] and
[directory][directory-capabilities] provide persistent or ephemeral storage in a Fuchsia
system where files belong to a specific component can be stored and accessed. Every Fuchsia
component has a [namespace][namespace] where a component can access all of its available
capabilities. A component's [moniker][component-moniker] indicates its position in
the component topology, which often functions as an identifier for the component and its
namespace.

Therefore, to access files in a component's namespace, you first need to identify the
[absolute moniker][absolute-moniker] of your target component. For example, the absolute
moniker of the `stash_secure` component is `/core/stash_secure`. (To discover a component's
absolute moniker, see the [`ffx component show`][ffx-component-show] command.)

## Copy a file to and from a component's namespace {:#copy-a-file-to-and-from-a-components-namespace}

To copy a file to and from a component running on a Fuchsia device, run the following
command:

```posix-terminal
ffx component copy {{ '<var>' }}SOURCE{{ '</var>' }} {{ '<var>' }}DESTINATION{{ '</var>'}}
```

Replace the following:

*  <var>SOURCE</var>: The path of the file(s) to be copied. This path can refer
   to either the host machine or a Fuchsia component on the target device.

*  <var>DESTINATION</var>: The destination path for the file(s). This path can
   refer to either the host machine or a Fuchsia component on the target device.

   If <var>SOURCE</var> or <var>DESTINATION</var> is a Fuchsia component,
   use the syntax below to represent the component and path:

   ```none {:.devsite-disable-click-to-copy}
   {{ '<var>' }}MONIKER{{ '</var>' }}::{{ '<var>' }}PATH_IN_NAMESPACE{{ '</var>' }}
   ```

   For example, `/core/stash_secure::/data/examples.txt` means that the `example.txt` file
   is located in the `/data` directory of the `/core/stash_secure` component.

See example commands below:

*  Download the `stash_secure.store` file from the `/core/stash_secure`
   component to the host machine:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx component copy /core/stash_secure::/data/stash_secure.store ./copy_of_stash_secure.store
   ```

*  Upload the `example.txt` file from the host machine to the `/core/stash_secure`
   component:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx component copy ./my_example_dir/example.txt /core/stash_secure::/data/copy_of_example.txt
   ```

*  Copy the `stash_secure.store` file from the `/core/stash_secure` component to
   the `/core/feedback` component:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx component copy /core/stash_secure::/data/stash_secure.store /core/feedback::/data/copy_of_stash_secure.store
   ```

## Copy multiple files to and from a component's namespace {:#copy-multiple-files-to-and-from-a-components-namespace}

The `ffx component copy` command supports the use of the wildcard `*` to copy multiple files at once.
When the wildcard is used, the paths on the host machine are interpreted by the terminal while the
paths on the Fuchsia device are interpreted by the `ffx component copy` command.

Note: In [`Zsh`][zsh]{:.external} and [`fish`][fish]{:.external}, you may run into issues when
using `*` for the paths on the device. A workaround is to wrap the path with `*` in single quotes
(`''`). For example, replace `/data/*` with `'/data/*'`.

See example commands below:

*  Download all files in the `/data` directory of the `/core/stash_secure`
   component to the host machine:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx component copy /core/stash_secure::/data/* ./my_example_dir/
   ```

*  Upload all files in the `./my_example_dir` directory of the host machine to
   the `/core/stash_secure` component:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx component copy ./my_example_dir/* /core/stash_secure::/data/
   ```

*  Copy all files in the `/data` directory of the `/core/stash_secure` component to
   the `/core/feedback` component:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx component copy /core/stash_secure::/data/* /core/feedback::/data/
   ```

*  Copy all files in the `data` directory of the `/core/feedback/` component and the
   `example.txt` file on the host machine to the `/core/stash_secure` component:

   ``` none {:.devsite-disable-click-to-copy}
   $ ffx component copy /core/feedback::/data/* ./my_example_dir/example.txt /core/stash_secure::/data/
   ```

## Appendices

### Copy a file to and from a component's isolated storage {:#copy-a-file-to-and-from-a-components-isolated-storage}

A Fuchsia component can allocate per-component *isolated* storage on a device using
[storage capabilities][storage-capabilities]. This type of storage is isolated within
the component, preventing other components from accessing files in the storage.

Unlike `ffx component copy`, the `ffx component storage` command works exclusively with
storage capabilities. Instead of an absolute moniker, this command uses a component's
[instance ID][component-id-index] as an identifier. By default, the `ffx component storage`
command connects to the component's `data` storage.

To copy a file to and from a component's isolated storage, run the following command:

```posix-terminal
ffx component storage copy {{ '<var>' }}SOURCE{{ '</var>' }} {{ '<var>' }}DESTINATION{{ '</var>'}}
```

Replace the following:

*  <var>SOURCE</var>: The path of the file(s) to be copied. This path can refer
   to either the host machine or a Fuchsia component on the target device.

*  <var>DESTINATION</var>: The destination path for the file(s). This path can
   refer to either the host machine or a Fuchsia component on the target device.

   If <var>SOURCE</var> or <var>DESTINATION</var> is a Fuchsia component,
   use the syntax below to represent the component and path:

   ```none {:.devsite-disable-click-to-copy}
   {{ '<var>' }}INSTANCE_ID{{ '</var>' }}::{{ '<var>' }}PATH_IN_STORAGE{{ '</var>' }}
   ```

   For example, `c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4::/examples.txt`
   means that the `example.txt` file is located in the default `/data` directory of the
   `/core/stash_secure` component.

See example commands below:

*  Upload the `example.txt` file from the host machine to the `/core/stash_secure`
   component:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx component storage copy ./my_example_dir/example.txt c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4::/copy_of_example.txt
   ```

*  Download the `stash_secure.store` file from the target componnet to the host machine:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx component storage copy c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4::/stash_secure.store ./my_example_dir/copy_of_example.txt
   ```

### List all directories and files in a component's isolated storage {:#list-all-directories-and-files-in-a-components-isolated-storage}

To list all directories and files in a component's
[isolated storage](#copy-a-file-to-and-from-a-components-isolated-storage),
run the following command:

```posix-terminal
ffx component storage list {{ '<var>' }}INSTANCE_ID{{ '</var>' }}::{{ '<var>' }}PATH{{ '</var>' }}
```

Replace the following:

*  <var>INSTANCE_ID</var>: The instance ID of the target component.
   For example, `c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4`.

*  <var>PATH</var>: A path on the target component.

The example command below shows all directories and files in the root (`/`) directory
of the target component:

```none {:.devsite-disable-click-to-copy}
$ ffx component storage list c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4::/
copy_of_example.txt
stash_secure.store
```

### Create a new directory in a component's isolated storage {:#create-a-new-directory-in-a-components-isolated-storage}

To create a new directory in a component's
[isolated storage](#copy-a-file-to-and-from-a-components-isolated-storage),
run the following command:

```posix-terminal
ffx component storage make-directory {{ '<var>' }}INSTANCE_ID{{ '</var>' }}::{{ '<var>' }}NEW_PATH{{ '</var>' }}
```

Replace the following:

*  <var>INSTANCE_ID</var>: The instance ID  of the target component.
   For example, `c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4`.

*  <var>NEW_PATH</var>: The name of the new directory to be created on the target
   component.

The example command below creates a new directory named `my-new-example-dir` on
the target component:

```none {:.devsite-disable-click-to-copy}
$ ffx component storage make-directory c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4::/my-new-example-dir
```

<!-- Reference links -->

[storage-capabilities]: /docs/concepts/components/v2/capabilities/storage.md
[directory-capabilities]: /docs/concepts/components/v2/capabilities/directory.md
[component-moniker]: /docs/concepts/components/v2/identifiers.md#monikers
[ffx-component-storage]: https://fuchsia.dev/reference/tools/sdk/ffx#storage
[ffx-component-show]: ./view-component-information.md#get-detailed-information-from-a-component
[component-id-index]: /docs/development/components/component_id_index.md
[absolute-moniker]: /docs/reference/components/moniker.md#absolute
[namespace]: /docs/concepts/process/namespaces.md
[zsh]: https://zsh.sourceforge.io/
[fish]: https://fishshell.com/
