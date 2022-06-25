# Copy files to and from a component

The [`ffx component storage`][ffx-component-storage] commands
can copy files to and from Fuchsia components on a device.

## Concepts

A Fuchsia component can provide storage space on a device if the component
uses one of the following [storage capabilities][storage-capabilities]:
`data`, `cache` and `tmp`. All the examples in this guide select, by default,
the `data` storage capability, which provides persistent storage on the
device. For other storage types (`cache` and `tmp`), you can use
the `--provider` flag with the `ffx component storage` commands.

Before you can upload files to (or download files from) a Fuchsia component,
you first need to identify the [instance ID][component-id-index] of your target
component on the device. To discover a component's instance ID, you can use
the [`ffx component show`][ffx-component-show] command, for example:

``` none {:.devsite-disable-click-to-copy}
$ ffx component show stash_secure
               Moniker: /core/stash_secure
                   URL: fuchsia-pkg://fuchsia.com/stash#meta/stash_secure.cm
                  Type: CML static component
       Component State: Resolved
           Instance ID: c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4
           ...
```

The example above shows that the instance ID of the `stash_secure` component is
`c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4`.

Once you identify your target component's instance ID, you can use this
identifier (as a parameter to the `ffx component storage` commands) to access
the component's storage space on the device.

## Download a file from the device {:#download-a-file-from-the-device}

To download a file from a Fuchsia device to your host machine, run the
following command:

```
ffx component storage copy <INSTANCE_ID>::<PATH_TO_FILE> <DESTINATION>
```

Replace the following:

*   `INSTANCE_ID`: The instance ID of your target component.
    *   For example,
        `c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4`.
*   `PATH_TO_FILE`: The path to a file on the target component.
    *   For example, `/my-example.file` or `/my/path/my-example.file`.
*   `DESTINATION`: The path to a local directory where you want to save the
    file.

The example command below downloads `my-example.file` from the target
component to the host machine:

``` none {:.devsite-disable-click-to-copy}
$ ffx component storage copy c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4::/my-example.file ./my-example.file
```

## Upload a file to the device {:#upload-a-file-to-the-device}

To upload a file from your host machine to a Fuchsia device, run the
following command:

```
ffx component storage copy <SOURCE> <INSTANCE_ID>::<PATH_TO_FILE>
```

Replace the following:

*   `SOURCE`: The path to a file you want to copy to the device.
*   `INSTANCE_ID`: The instance ID of your target component.
    *   For example,
        `c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4`.
*   `PATH_TO_FILE`: The path and filename on the target component where you want
    to save the file.
    *   For example, `/my-example.file` or `/my/path/my-example.file`.

The example command below uploads `my-example.file` from the host machine to the
target component running on the device:

``` none {:.devsite-disable-click-to-copy}
$ ffx component storage copy ./my-example.file c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4::/my-example.file
```

## List all directories and files {:#list-all-directories-and-files}

To list all directories and files in a component's storage, run
the following command:

```
ffx component storage list <INSTANCE_ID>::<PATH>
```

Replace the following:

*   `INSTANCE_ID`: The instance ID of your target component.
    *   For example,
        `c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4`.
*   `PATH`: A path on the target component.
    *   For example, `/` or `/my/path/`.

The example command below shows all directories and files in the root (`/`)
directory of the target component:

``` none {:.devsite-disable-click-to-copy}
$ ffx component storage list c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4::/
my-example.file
dir_01
dir_02
dir_03
```

## Create a new directory {:#create-a-new-directory}

To create a new directory in a component's storage, run the
following command:

```
ffx component storage make-directory <INSTANCE_ID>::<NEW_PATH>
```

Replace the following:

*   `INSTANCE_ID`: The instance ID of your target component.
    *   For example,
        `c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4`.
*   `PATH`: The name of a new directory on the target component.
    *   For example, `/my-new-path`.

The example command below shows creates a new directory named `my-new-path` on
the target component:

``` none {:.devsite-disable-click-to-copy}
$ ffx component storage make-directory c1a6d0aebbf7c092c53e8e696636af8ec0629ff39b7f2e548430b0034d809da4::/my-new-path
```

<!-- Reference links -->

[storage-capabilities]: /docs/concepts/components/v2/capabilities/storage.md
[ffx-component-storage]: https://fuchsia.dev/reference/tools/sdk/ffx#storage
[ffx-component-show]: ./view-component-information.md#get-detailed-information-from-a-component
[component-id-index]: /docs/development/components/component_id_index.md
