# Explore the `edu` device

The `edu` device connects to the system PCI bus and identifies itself using the
following configuration register values:

*   **Vendor ID (VID):** `0x1234`
*   **Device ID (DID):** `0x11E8`

Device drivers interact with the device using Interrupts and Memory-Mapped I/O
(MMIO) registers.

Note: For complete details on the `edu` device and its MMIO regions, see the
[device specification][edu-device-spec].

In this codelab, you'll write a Fuchsia driver that provides the features of
this `edu` device to other system components.

<<_common/_start_femu.md>>

## Explore the device nodes

Use the `ffx driver list-devices` command to list all the device nodes known to
the system:

```posix-terminal
ffx driver list-devices
```

The command outputs a list similar to the following:

```none {:.devsite-disable-click-to-copy}
root
root.sys
root.sys.platform
root.sys.cpu-trace
root.sys.platform.pt
root.sys.platform.00_00_2d
root.sys.platform.00_00_2e
root.sys.platform.00_00_2f
root.sys.platform.00_00_30
root.sys.platform.00_00_1b
root.sys.cpu-trace.perfmon
root.sys.platform.pt.acpi
root.sys.platform.pt.PCI0
...
```

The `edu` device is on the PCI bus, so use the following command to narrow down
the list of devices:

```posix-terminal
ffx driver list-devices | grep 'PCI'
```

You'll see a shorter list similar to the following output:

```none {:.devsite-disable-click-to-copy}
root.sys.platform.pt.PCI0
root.sys.platform.pt.PCI0.bus
root.sys.platform.pt.PCI0.bus.00_00_0
root.sys.platform.pt.PCI0.bus.00_00_0_
root.sys.platform.pt.PCI0.bus.00_01_0
root.sys.platform.pt.PCI0.bus.00_01_0_
root.sys.platform.pt.PCI0.bus.00_02_0
root.sys.platform.pt.PCI0.bus.00_02_0_
root.sys.platform.pt.PCI0.bus.00_03_0
root.sys.platform.pt.PCI0.bus.00_03_0_
root.sys.platform.pt.PCI0.bus.00_04_0
root.sys.platform.pt.PCI0.bus.00_04_0_
root.sys.platform.pt.PCI0.bus.00_05_0
root.sys.platform.pt.PCI0.bus.00_05_0_
root.sys.platform.pt.PCI0.bus.00_06_0
root.sys.platform.pt.PCI0.bus.00_06_0_
root.sys.platform.pt.PCI0.bus.00_07_0
root.sys.platform.pt.PCI0.bus.00_07_0_
root.sys.platform.pt.PCI0.bus.00_0b_0
root.sys.platform.pt.PCI0.bus.00_0b_0_
root.sys.platform.pt.PCI0.bus.00_1f_0
root.sys.platform.pt.PCI0.bus.00_1f_0_
root.sys.platform.pt.PCI0.bus.00_1f_2
root.sys.platform.pt.PCI0.bus.00_1f_2_
root.sys.platform.pt.PCI0.bus.00_1f_3
root.sys.platform.pt.PCI0.bus.00_1f_3_
```

These are the PCI device nodes in the current emulator instance.

Note: The driver framework team is currently migrating drivers from
[Banjo][drivers-banjo] to [FIDL][drivers-fidl] interfaces. During the migration,
each device node appears twice to represent each interface type.

## Discover the correct device

In order to determine which of these device nodes is the `edu` device, use the
`lspci` command to find the device with the matching VID (`0x1234`) and
DID (`0x11e8`) of the `edu` device:

```posix-terminal
ffx driver lspci
```

The command prints a list similar to the following, indicating the matching PCI
device node is `00:06.0`:

```none {:.devsite-disable-click-to-copy}
00:00.0 Host bridge: Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller (rev 00)
00:01.0 Audio device: Intel Corporation 82801FB/FBM/FR/FW/FRW (ICH6 Family) High Definition Audio Controller (rev 01)
00:02.0 SCSI storage controller: Red Hat, Inc. Virtio block device (rev 00)
00:03.0 Mouse controller: Red Hat, Inc. Virtio input (rev 01)
00:04.0 Ethernet controller: Red Hat, Inc. Virtio network device (rev 00)
00:05.0 Keyboard controller: Red Hat, Inc. Virtio input (rev 01)
{{ '<strong>' }}00:06.0 Unclassified device: [1234:11e8] (rev 10){{ '</strong>' }}
00:07.0 Mouse controller: Red Hat, Inc. Virtio input (rev 01)
00:0b.0 Unclassified device: [607d:f153] (rev 01)
...
```

From the device list in the previous section, this means the `edu` device maps
to the PCI device node `root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl`.

Explore the properties of this device node using the following command:

```posix-terminal
ffx driver list-devices root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl --verbose
```

The command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
Name     : 0-fidl
{{ '<strong>' }}Moniker  : root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl{{ '</strong>' }}
Driver   : None
11 Properties
[ 1/ 11] : Key fuchsia.BIND_FIDL_PROTOCOL     Value 0x000004
{{ '<strong>' }}[ 2/ 11] : Key fuchsia.BIND_PCI_VID           Value 0x001234{{ '</strong>' }}
{{ '<strong>' }}[ 3/ 11] : Key fuchsia.BIND_PCI_DID           Value 0x0011e8{{ '</strong>' }}
[ 4/ 11] : Key fuchsia.BIND_PCI_CLASS         Value 0x000000
[ 5/ 11] : Key fuchsia.BIND_PCI_SUBCLASS      Value 0x0000ff
[ 6/ 11] : Key fuchsia.BIND_PCI_INTERFACE     Value 0x000000
[ 7/ 11] : Key fuchsia.BIND_PCI_REVISION      Value 0x000010
[ 8/ 11] : Key fuchsia.BIND_PCI_TOPO          Value 0x000030
[ 9/ 11] : Key "fuchsia.hardware.pci.Device"  Value true
[10/ 11] : Key fuchsia.BIND_PROTOCOL          Value 0x000000
[11/ 11] : Key "fuchsia.driver.framework.dfv2" Value true
```

Notice that this device does not currently have a driver loaded (the `Driver`
field is set to `None`). These properties describe the values that the driver
framework considers in determining whether a driver matches the device node.
In the next section, you'll declare a driver component that binds to this device
using these properties.

<!-- Reference links -->

[drivers-banjo]: /docs/development/drivers/concepts/device_driver_model/banjo.md
[drivers-fidl]: /docs/development/drivers/concepts/device_driver_model/fidl.md
[edu-device-spec]: https://fuchsia.googlesource.com/third_party/qemu/+/refs/heads/main/docs/specs/edu.txt
