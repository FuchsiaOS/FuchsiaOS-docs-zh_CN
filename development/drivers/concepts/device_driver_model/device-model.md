# Device model

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

## Introduction

In Fuchsia, device drivers are implemented as ELF shared libraries (DSOs), which are
loaded into Driver Host processes. The Driver Manager process,
contains the Device Coordinator which keeps track of drivers and devices, manages
the discovery of drivers, the creation and direction of Driver Host processes, and
maintains the Device Filesystem (devfs), which is the mechanism through which userspace
services and applications (constrained by their namespaces) gain access to devices.

The Driver Manager views devices as part of a single unified tree.
The branches (and sub-branches) of that tree consist of some number of
devices within a Driver Host process. The decision as to how to sub-divide
the overall tree among Driver Hosts is based on system policy for isolating
drivers for security or stability reasons and colocating drivers for performance
reasons.


## Devices, Drivers, and Driver Hosts

Here's a (slightly trimmed for clarity) dump of the tree of devices in
Fuchsia running on Qemu x86-64:

```sh
$ driver dump
[root]
   <root> pid=1509
      [null] pid=1509 /boot/driver/builtin.so
      [zero] pid=1509 /boot/driver/builtin.so
   [misc]
      <misc> pid=1645
         [console] pid=1645 /boot/driver/console.so
         [dmctl] pid=1645 /boot/driver/dmctl.so
         [ptmx] pid=1645 /boot/driver/pty.so
         [i8042-keyboard] pid=1645 /boot/driver/pc-ps2.so
            [hid-device-001] pid=1645 /boot/driver/hid.so
         [i8042-mouse] pid=1645 /boot/driver/pc-ps2.so
            [hid-device-002] pid=1645 /boot/driver/hid.so
   [sys]
      <sys> pid=1416 /boot/driver/bus-acpi.so
         [acpi] pid=1416 /boot/driver/bus-acpi.so
         [pci] pid=1416 /boot/driver/bus-acpi.so
            [00:00:00] pid=1416 /boot/driver/bus-pci.so
            [00:01:00] pid=1416 /boot/driver/bus-pci.so
               <00:01:00> pid=2015 /boot/driver/bus-pci.proxy.so
                  [bochs_vbe] pid=2015 /boot/driver/bochs-vbe.so
                     [framebuffer] pid=2015 /boot/driver/framebuffer.so
            [00:02:00] pid=1416 /boot/driver/bus-pci.so
               <00:02:00> pid=2052 /boot/driver/bus-pci.proxy.so
                  [e1000] pid=4628 /boot/driver/e1000.so
                     [ethernet] pid=2052 /boot/driver/ethernet.so
            [00:1f:00] pid=1416 /boot/driver/bus-pci.so
            [00:1f:02] pid=1416 /boot/driver/bus-pci.so
               <00:1f:02> pid=2156 /boot/driver/bus-pci.proxy.so
                  [ahci] pid=2156 /boot/driver/ahci.so
            [00:1f:03] pid=1416 /boot/driver/bus-pci.so
```

The names in square brackets are devices. The names in angle brackets are
proxy devices, which are instantiated in the "lower" driver host, when process
isolation is being provided. The pid= field indicates the process object
id of the driver host process that device is contained within. The path indicates
which driver implements that device.

Above, for example, the pid 1416 driver host contains the pci bus driver, which has
created devices for each PCI device in the system. PCI device 00:02:00 happens
to be an intel ethernet interface, which we have a driver for (e1000.so).
A new driver host (pid 2052) is created, set up with a proxy device for PCI 00:02:00,
and the intel ethernet driver is loaded and bound to it.

Proxy devices are invisible within the Device filesystem, so this ethernet device
appears as `/dev/sys/platform/pci/00:02:00/e1000`.

### Driver Framework Version 2 (DFv2)
In driver framework version 2, devices are referred to as nodes and the dump of
the tree of nodes will look slightly different:

```sh
$ driver dump
[root] pid=4766 fuchsia-boot:///#meta/platform-bus.cm
   [sys] pid=4766
      [platform] pid=4766
         [pt] pid=4766 fuchsia-boot:///#meta/platform-bus-x86.cm
            [acpi] pid=4766
               [acpi-pwrbtn] pid=4766 fuchsia-boot:///#meta/hid.cm
               [acpi-_SB_] pid=4766
               [acpi-PCI0] pid=4766
               [acpi-ISA_] pid=4766
               [acpi-RTC_] pid=7264 fuchsia-boot:///#meta/intel-rtc.cm
                  [rtc] pid=7264
               [acpi-KBD_] pid=7359 fuchsia-boot:///#meta/pc-ps2.cm
                  [i8042] pid=7359
                     [i8042-keyboard] pid=7359
                     [i8042-mouse] pid=7359
               [acpi-MOU_] pid=4766
               [acpi-COM1] pid=4766
               [acpi-PRES] pid=4766
               [acpi-GPE0] pid=4766
               [acpi-FWCF] pid=4766
               [acpi-S00_] pid=4766
               [acpi-S08_] pid=4766
               [acpi-S10_] pid=4766
               [acpi-S18_] pid=4766
               [acpi-S20_] pid=4766
               [acpi-S28_] pid=4766
               [acpi-S30_] pid=4766
               [acpi-S38_] pid=4766
               [acpi-S58_] pid=4766
               [acpi-HPET] pid=4766
               [acpi-LNKE] pid=4766
               [acpi-LNKF] pid=4766
               [acpi-LNKG] pid=4766
               [acpi-LNKH] pid=4766
               [acpi-GSIE] pid=4766
               [acpi-GSIF] pid=4766
               [acpi-GSIG] pid=4766
               [acpi-GSIH] pid=4766
               [acpi-GFBY] pid=4766
               [acpi-GFEV] pid=4766
               [acpi-GFPP] pid=4766
               [acpi-GFFB] pid=4766
               [acpi-GFAU] pid=4766
               [acpi-GFSK] pid=4766
               [acpi-GFRT] pid=4766
               [acpi-GFRO] pid=4766
               [acpi-CPUS] pid=4766
               [acpi-_TZ_] pid=4766
            [PCI0] pid=4766 fuchsia-boot:///#meta/bus-pci.cm
               [bus] pid=4766
                  [00_00_0] pid=4766
                  [00_00_0_] pid=4766
                  [00_01_0] pid=4766
                  [00_01_0_] pid=4766
                  [00_02_0] pid=4766 fuchsia-boot:///#meta/virtio_block.cm
                     [virtio-block] pid=4766 fuchsia-boot:///#meta/block.core.cm
                        [block] pid=4766 fuchsia-boot:///#meta/fvm.cm
                           [fvm] pid=4766
                              [blobfs-p-1] pid=4766 fuchsia-boot:///#meta/block.core.cm
                                 [block] pid=4766
                              [data-p-2] pid=4766 fuchsia-boot:///#meta/block.core.cm
                                 [block] pid=4766 fuchsia-boot:///#meta/zxcrypt.cm
                                    [zxcrypt] pid=4766
                                       [unsealed] pid=4766 fuchsia-boot:///#meta/block.core.cm
                                          [block] pid=4766
                  [00_02_0_] pid=4766
                  [00_03_0] pid=4766
                  [00_03_0_] pid=4766
                  [00_04_0] pid=4766 fuchsia-boot:///#meta/virtio_ethernet.cm
                     [virtio-net] pid=4766 fuchsia-boot:///#meta/netdevice-migration.cm
                        [netdevice-migration] pid=4766 fuchsia-boot:///#meta/network-device.cm
                           [network-device] pid=4766
                  [00_04_0_] pid=4766
                  [00_05_0] pid=4766
                  [00_05_0_] pid=4766
                  [00_06_0] pid=4766
                  [00_06_0_] pid=4766
                  [00_07_0] pid=4766
                  [00_07_0_] pid=4766
                  [00_0b_0] pid=4766
                  [00_0b_0_] pid=4766
                  [00_1f_0] pid=4766
                  [00_1f_0_] pid=4766
                  [00_1f_2] pid=4766
                  [00_1f_2_] pid=4766
                  [00_1f_3] pid=4766
                  [00_1f_3_] pid=4766
         [00_00_2d] pid=4766 fuchsia-boot:///#meta/ramdisk.cm
            [ramctl] pid=4766
         [00_00_2e] pid=4766
         [00_00_2f] pid=4766 fuchsia-pkg://fuchsia.com/virtual_audio#meta/virtual_audio_driver.cm
            [virtual_audio] pid=4766
         [00_00_30] pid=4766
         [00_00_1b] pid=4766 fuchsia-boot:///#meta/sysmem.cm
            [sysmem] pid=4766
```

The names in square brackets are nodes. The pid= field indicates the process
object id of the driver host process that device is contained within. The URL is
the component manifest of the driver that is bound to the node. Nodes that do
not have a URL do not have a driver bound to them.
