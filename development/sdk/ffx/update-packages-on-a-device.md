# Update Fuchsia packages on a device

The [`ffx target update`][ffx-target-update] commands can check
and perform software updates on a Fuchsia device.

## Concepts

During development, you can trigger a Fuchsia device to check and perform
updates on all Fuchsia packages known to the device. To check for updates,
the device needs at least one running
[Fuchsia package repository][start-a-package-repo], which serves as
an [update channel](#list-update-channels) for the device.

The device compares its Fuchsia packages to the Fuchsia packages that are
available in the update channel. For each Fuchsia package, when the device
detects that the [Merkle root][merkle-root] of the package does not
match the Merkle root of the same package in the update channel,
the device performs a software update for that Fuchsia package.

## Check and perform software updates {:#check-and-perform-software-updates}

To check and perform software updates on a Fuchsia device,
run the following command:

```posix-terminal
ffx target update check-now
```

The device may reboot first before it starts updating.

## List update channels {:#list-update-channels}

To view the list of update channels, run the following command:

```posix-terminal
ffx target update channel list
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx target update channel list
known channels:
fuchsiasamples.com
```

<!-- Reference links -->

[ffx-target-update]: https://fuchsia.dev/reference/tools/sdk/ffx#update_3
[flash-fuchsia]: ./flash-a-device.md
[start-a-package-repo]: ./create-a-package-repository.md
[merkle-root]: /concepts/packages/merkleroot.md
