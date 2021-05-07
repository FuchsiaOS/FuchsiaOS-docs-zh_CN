# Device model

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
$ dm dump
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
appears as `/dev/sys/pci/00:02:00/e1000`.
