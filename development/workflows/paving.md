# Putting Fuchsia on a Device

One of the best ways to experience Fuchsia is by running it on actual hardware.
This guide will help you get Fuchsia installed on your device. Fuchsia has good
support for a few different hardware platforms including the Acer Switch 12,
Intel NUC, and Google Pixelbook (not to be confused with the Chromebook Pixel).
The install process is not currently compatible with ARM-based targets. The
Fuchsia install process, called 'paving', requires two machines, the machine on
which you want to run Fuchsia ("target") and the machine on which you build
Fuchsia ("host"). Host and target must be able to communicate over a local area
network. On your host system you will build Fuchsia, create a piece of install
media, and stream a large portion of the system over the network to the target.

The `fx` command will be used throughout these instructions. If you have fx
mapped into your command path you can follow the instructions verbatim. If you
don't have fx in your path, it can be found at `//scripts/fx` and you'll need
to use the appropriate relative path in the supplied commands. Many of fx
commands are relatively thin wrappers around build actions in GN coupled with
tool invocations. If your use case isn't quite served by what's currently
available there may a few GN targets you can build or some GN templates you can
extend to allow you to build what you need.

## TL;DR

Read this all before? See the
[quickstart guide](https://fuchsia.googlesource.com/fuchsia/+/master/docs/development/workflows/build_and_pave_quickstart.md)
for a workflow summary.

## Building

Detailed instructions for obtaining and building Fuchsia are available from the
[Getting Started](/getting_started.md) guide, but we'll assume here that the
target system is x86-based and that you want to build a complete system. To
configure our build for this we can run `fx set x64` and then build with
`fx full-build`.

## Creating install media

To create your install media we recommend using a USB drive since these are
well-supported as boot media by most systems. Note that the install media
creation process **will wipe everything** from the USB drive being used. Insert the
USB drive and then run `fx mkzedboot <device_path>`, which on Linux is
typically something like /dev/sd&lt;X&gt; where X is a letter and on Mac is typically
something like /dev/disk&lt;N&gt; where 'N' is a number. **Be careful not to select
the wrong device**. Once this is done, remove the USB drive.

## Paving

Now we'll build the artifacts to transfer over the network during the paving
process. What is transferred is dependent on the target device. For UEFI based
systems (like Intel NUC or Acer Switch 12) our output target type is 'efi'. For
ChromeOS-based systems (like Pixelbook) that use vboot-format images, the target
type is 'vboot'. To start the bootserver with the correct image just run `fx pave`.

Insert the install media into the target device that you want to pave. The target
device's boot settings may need to be changed to boot from the USB device and
this is typically device-specific. For the guides listed below, **only** go
through the steps to set the boot device, don't continue with any instructions on
creating install media.
* [Acer Switch Alpha 12](https://fuchsia.googlesource.com/zircon/+/master/docs/targets/acer12.md)
* [Intel NUC](https://fuchsia.googlesource.com/zircon/+/master/docs/targets/nuc.md)
* [Google Pixelbook](/development/hardware/pixelbook.md)

Paving should occur automatically after the device is booted into Zedboot from the
USB drive. After the paving process completes, the system should boot into the
Zircon kernel. After paving, the whole system is installed on internal storage. At
this point the USB key can be removed since the system has everything it needs
stored locally. If you plan to re-pave frequently it may be useful to keep the
USB drive inserted so your system boots into Zedboot by default where paving
will happen automatically. After the initial pave on UEFI systems that use
Gigaboot, another option for re-paving is to press 'z' while in Gigaboot to
select Zedboot. For vboot-based systems using the USB drive is currently the
only option for re-paving. In all cases the bootserver needs to have been
started with `fx pave`

## Troubleshooting

In some cases paving may fail because you have a disk layout that is incompatible.
In these cases you will see a message that asks you to run
'install-disk-image wipe'. If it is incompatible because it contains an older
Fuchsia layout put there by installer (vs the paver) you can fix this by killing
the fx pave process on the host, switching to a different console (Alt+F3) on
the target, and running `install-disk-image wipe`. Then reboot the target,
re-run `fx pave` on the host, and the pave should succeed.

In some cases paving may fail on an Acer with some error indicating "couldn't
find space in gpt". In these cases (as long as you don't want to keep the other
OS, i.e. Windows, parts) run `lsblk` and identify the partition that isn't your
USB (it shouldn't have RE in the columns). Identify the number in the first
column for your partition (likely to be either 000 or 003). Then run
`gpt init /dev/class/block/N` where N is the number previously identified. This
will clear all Windows partitions from the disk. Once this is done, reboot into
zedboot and paving should work.

## Changing boot target (localboot, netboot, etc) default

For EFI-based systems, it is possible to change the default boot option of the
system paved on the target between local booting and Zedboot for network
booting. By default the system boots locally with a 1-second delay in Gigaboot
to allow you to select a different mode. To change this default to Zedboot,
supply the `always_zedboot` argument when calling your set command, for example
`fx set <target_type> --args "always_zedboot=true"`.
