# View device information

The [`ffx target`][ffx-target] commands can retrieve various types of
information from Fuchsia devices.

## Concepts

The [`ffx target list`][ffx-target-list] command prints the list of all
Fuchsia devices connected to the host machine. This command is often used
to quickly scan the following information of connected Fuchsia devices:

* A device name
* A device's product type
* A device's state
* A device's IP address

The [`ffx target show`][ffx-target-show] command, unlike `ffx target list`,
targets only a single Fuchsia device and prints much detailed information
about the device. To do so, `ffx target show` requires that `ffx` can
establish a [SSH connection][ssh-connection] to the target device. Below
is some of the information you can obtain from `ffx target show`:

* The board name of the target device
* The version of the Fuchsia product running on the target device
* The product configuration on the target device

Lastly, the [`ffx target snapshot`][ffx-target-snapshot] command
generates a ZIP archive file that captures the current state
of the target device. A snapshot from a device contains a generous
amount of information helpful for debugging.

## Get the list of devices

To get the list of all connected Fuchsia devices,
run the following command:

```posix-terminal
ffx target list
```

This command prints output similar to the following:

``` none {:.devsite-disable-click-to-copy}
$ ffx target list
NAME                      SERIAL       TYPE       STATE      ADDRS/IP                                             AGE     RCS
fuchsia-5254-0063-5e7a    <unknown>    Unknown    Unknown    [172.16.241.43, fe80::7594:7308:4168:9fb1%brqemu]    0m8s    N
```

## Get detailed information from a device

To get the detailed information of your target Fuchsia device,
run the following command:

```posix-terminal
ffx target show
```

This command prints output similar to the following:

``` none {:.devsite-disable-click-to-copy}
$ ffx target show
Target:
    Name: "fuchsia-4102-0ba9-8a3b"
    SSH Address: "[fe80::ae21:e7fa:8e1f:6c46%17]:22"
Board:
    Name: "<BOARD_NAME>"
    Revision: "<REVISION_NUMBER>"
    ...
Device:
    Serial number: "<SERIAL_NUMBER>"
    ...
Product:
    ...
    Build date: "<DATE>"
    Build name: "<BUILD>"
    ...
    Model: "<MODEL>"
    Name: "<NAME>"
    ...
Update:
    Current channel: "<CHANNEL>"
    Next channel: "<CHANNEL>"
Build:
    Version: "<VERSION>"
    Product: "<PRODUCT>"
    Board: "<BOARD>"
    Commit: "<DATE>"
...
```

## Generate a snapshot from a device

To generate a snapshot from your target Fuchsia device,
run the following command:

```posix-terminal
ffx target snapshot
```

This command generates a ZIP archive file and prints its location,
for example:

``` none {:.devsite-disable-click-to-copy}
$ ffx target snapshot
Exported /tmp/snapshots/20210616_183136/snapshot.zip
```

By default, the command stores the archive file in the `/tmp` directory of
the host machine. To change this directory,
run `ffx target snapshot --dir <PATH_TO_DIR>`.

<!-- Reference links -->

[ffx-target]: https://fuchsia.dev/reference/tools/sdk/ffx#target
[ffx-target-list]: https://fuchsia.dev/reference/tools/sdk/ffx#list_17
[ffx-target-show]: https://fuchsia.dev/reference/tools/sdk/ffx#show_8
[ffx-target-snapshot]: https://fuchsia.dev/reference/tools/sdk/ffx#snapshot
[ssh-connection]: ./create-ssh-keys-for-devices.md
