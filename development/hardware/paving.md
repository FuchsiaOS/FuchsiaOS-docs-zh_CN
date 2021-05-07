# Installing Fuchsia on a device

This guide helps you understand the device installation process for Fuchsia.
The Fuchsia install process, called 'paving', requires two machines, the machine
on which you build Fuchsia ("host") and the device on which you want to run
Fuchsia ("target"). The host and target must be able to communicate over a local
area network.

This guide covers at a high level how to use your host system
to build Fuchsia, create a piece of install media, and stream
the system over the network to the target.


## Building {#building}

The `fx` command is used throughout these instructions. Before you can
install Fuchsia on a device, you must have Fuchsia source on your host by
following the instructions in these two documents:

 * [Fuchsia source installed and environment variables created](/docs/get-started/get_fuchsia_source.md).
 * [Configured and built Fuchsia](/docs/get-started/build_fuchsia.md)

If you have `fx` mapped into your command path you can follow these instructions. If you
don't have fx in your path, it can be found at `//scripts/fx` and you'll need
to use the appropriate relative path in the supplied commands.

Many `fx` commands are wrappers around build actions in [GN](/docs/concepts/build_system/intro.md)
coupled with tool invocations.

Assuming that the target system is x86-based and that you want to build a
complete Fuchsia system, you can set Fuchsia with the following command:

```
fx set {product_name}.x64`
```

Note: For more information on supported products, see [product and board](/docs/concepts/build_system/boards_and_products.md).

Then build Fuchsia:

```
fx build
```

## Creating install media {#creating-install-media}

To create your install media you should use a USB drive since these are
well-supported as boot media by most systems.

Note: Creating a USB drive **wipes everything** from the USB drive being used.

To create a USB drive, insert a USB drive into the host machine and run:

```
fx mkzedboot <device_path>
```

On Linux, `device_path` is typically `/dev/sd<X>` where X is a letter. On macOS
it is typically `/dev/disk<N>` where 'N' is a number. If you don't specify a
device path, `mkzedboot` lists the drives it detects. **Be careful
not to select the wrong device**. Once this is done, remove the USB drive.

## Paving {#paving}

Next, you must build the artifacts to transfer over the network during the paving
process. What is transferred is dependent on the target device. For UEFI based
systems (like the Intel NUC) our output target type is 'efi'.

To start the bootserver with the correct image just run `fx pave`.

Insert the install media into the target device that you want to pave. The
target device's boot settings may need to be changed to boot from the USB device
and this is typically device-specific. For the guides listed below, **only** go
through the steps to set the boot device, don't continue with any instructions
on creating install media.

* [Intel NUC](/docs/development/hardware/intel_nuc.md)

Paving should occur automatically after the device is booted into Zedboot from
the USB drive. After the paving process completes, the system should boot into
the Zircon kernel. After paving, the whole system is installed on internal
storage.

At this point the USB key can be removed since the system has
everything it needs stored locally. If you plan to re-pave frequently it may be
useful to keep the USB drive inserted so your system boots into Zedboot by
default where paving happens automatically. After the initial pave on UEFI
systems that use Gigaboot, another option for re-paving is to press 'z' while in
Gigaboot to select Zedboot.


## Troubleshooting {#troubleshooting}

In some cases paving may fail because you have a disk layout that is
incompatible. In these cases you will see a message that asks you to run
'install-disk-image wipe'. If it is incompatible because it contains an older
Fuchsia layout put there by installer (vs the paver) you can fix this by killing
the fx pave process on the host, switching to a different console (Alt+F3) on
the target, and running `install-disk-image wipe`. Then reboot the target,
re-run `fx pave` on the host, and the pave should succeed.

In some cases paving may fail with an error indicating "couldn't find space in
gpt". In these cases (as long as you don't want to keep the other OS, i.e.
Windows, parts) run `lsblk` and identify the partition that isn't your USB (it
shouldn't have RE in the columns). Identify the number in the first column for
your partition (likely to be either 000 or 003). Then, run:

Note: `N` is the number you just identified.

```
gpt Init /dev/class/block/N
```

This clears all Windows partitions from the disk. Once this is done, reboot into
zedboot and paving should work.

## Changing boot target (localboot, netboot, etc) default

For EFI-based systems, it is possible to change the default boot option of the
system paved on the target between local booting and Zedboot for network
booting. By default the system boots locally with a 1-second delay in Gigaboot
to allow you to select a different mode. To change this default to Zedboot,
supply the `always_zedboot` argument when calling your set command, for example
`fx set <goal> --args "always_zedboot=true"`.

## More information

 * [Creating a Fuchsia USB flash drive](/docs/development/hardware/usb_setup.md)
 * [Step-by-step build and pave guide](/docs/development/build/build_and_pave_quickstart.md)
 * [More information on fx workflows](/docs/development/build/fx.md)
