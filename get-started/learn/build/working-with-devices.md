# Working with devices

This codelab series focuses on the
[Fuchsia emulator](/development/build/emulator.md) (FEMU) as the target
device, which is built and distributed with the source tree and runs on your
development machine. However, you can also build Fuchsia for supported hardware
platforms, such as an [Intel NUC](/development/hardware/intel_nuc.md).

This section describes some specifics related to working with Fuchsia on
physical devices.

## Configure

Fuchsia defines support for hardware devices by the **board name** used to
configure the build. This includes any hardware-specific packages such as
drivers. Recall the `fx set` command used previously:

```posix-terminal
fx set workstation_eng.qemu-x64
```

In this example, `qemu-x64` is the board name for FEMU. To build the same
**product** for the Intel NUC, you can modify the `set` command to use the
`x64` board.

Note: To determine the Fuchsia board name for your supported hardware, see the
[device documentation](/development/hardware/README.md).

```posix-terminal
fx set workstation_eng.x64
```

Running `fx build` will now generate an image for the target device.

## Bootstrap

Before flashing the operating system, a supported device must have a
Fuchsia-compatible bootloader installed. This process is known as
**bootstrapping** the device. Many devices have a compatible bootloader
installed from the factory, others may require manufacturer-specific tools to
update the bootloader to a compatible version.
See the [device documentation](/development/hardware/README.md) for more
details regarding your specific device.

<aside class="key-point">
Most Fuchsia-compatible devices use a bootloader that supports the
<a href="/contribute/governance/rfcs/0081_fastboot_boot">Fastboot protocol</a>.
</aside>

## Flash

The process of loading the operating system onto the device is known as
**flashing**. With a device in bootloader mode connected to your workstation,
you can use the `flash` command to flash Fuchsia onto the device.

```posix-terminal
fx flash
```

For devices that have already been flashed, you can reboot them from Fuchsia
into bootloader mode if you need to flash them again using `ffx`:

```posix-terminal
ffx target reboot --bootloader
```

## Discover

You can discover and interact with Fuchsia devices from a development machine
connected over USB or a local IPv6 network. Fuchsia enables automatic device
discovery using DNS Service Discovery (DNS-SD) over multicast DNS (mDNS) and
the [Overnet mesh protocol](/src/connectivity/overnet/).

Host tools such as `ffx` discover advertising devices and enable host-target
interaction with both physical devices and FEMU.


```posix-terminal
ffx target list
```

```none {:.devsite-disable-click-to-copy}
NAME                      SERIAL       TYPE    STATE      ADDRS/IP                            RCS
fuchsia-5254-0063-5e7a    <unknown>    .       Product    [fe80::c357:53e7:aedf:ed95%qemu]    Y
```


If a target device does not advertise discovery packets or `ffx` is unable to
detect them, you can manage those targets manually using the `add` and `remove`
commands:

```posix-terminal
ffx target add {{ "<var>" }}device-ip{{ "</var>" }}:{{ "<var>" }}device-port{{ "</var>" }}

ffx target remove {{ "<var>" }}device-ip{{ "</var>" }}:{{ "<var>" }}device-port{{ "</var>" }}
```

Once a device is tracked in the target list, `ffx` interacts with the Remove
Control Service (RCS) on the target to enable you to send additional commands.

![Diagram showing how "ffx" is a developer tool that communicates with the
 Remote Control Service (RCS) on the Fuchsia Device.](
    /get-started/images/build/ffx-rcs.png){: width="591"}

Note: For a complete list of the developer commands supported by `ffx`, see
the [`ffx` reference](https://fuchsia.dev/reference/tools/sdk/ffx.md).

## What's Next?

Congratulations! You've successfully customized and built Fuchsia from source,
and have a better understanding for where the key system components live in the
source tree.

In the next module, you'll learn more about building Fuchsia's fundamental unit
of software:

<a class="button button-primary"
    href="/get-started/learn/components">Fuchsia components</a>
