<!---

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

--->

# 设备模型

## 简介

在 Fuchsia 中，设备驱动实现成 ELF 共享库（DSOs），用于加载到驱动主机进程中。驱动管理进程包含设备协调程序，用来追踪驱动和设备状态，驱动管理进程管理驱动程序发现，创建和指导驱动程序主机进程，并维护设备文件系统（ devfs ），设备文件系统则是用户空间服务和应用程序（受其命名空间的限制）获得设备访问的机制。

驱动管理器把设备看做单一统一树的部分。在驱动主机进程中的树的分支（次级分支）包含一些设备。关于如何在驱动主机间细分整个树的决定是基于系统策略，即出于安全或稳定的原因隔离驱动，以及出于性能原因搭配驱动。

<!---

## Devices, Drivers, and Driver Hosts

Here's a (slightly trimmed for clarity) dump of the tree of devices in
Fuchsia running on Qemu x86-64:

--->

## 设备，驱动和驱动主机

这里有一个（为清晰起见，略作修饰）在 Qemu x86-64 上运行的 Fuchsia 设备树的转储。

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

<!---

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

--->

在方括号内的名称就是设备。在尖括号内的名称就是代理设备，当提供进程隔离时，代理设备通常在“低级”驱动主机内被实例化。pid= 字段表明了该设备所在的驱动主机进程的对象ID。path 表明了哪个驱动实现了该设备。

在上述示例中，例如进程1416驱动主机包含了 pci 总线驱动，它对每一个系统内的 PCI 设备创建了设备。 PCI 设备在 00:02:00 时开始作为一个网络以太网接口，并且有一个驱动程序（e1000.so）。

一个新的驱动主机（进程2052）被创建，在 00:02:00 时设置为 PCI 的代理设备，然后由以太网驱动加载并绑定。

代理设备在设备文件系统内是不可见的，所以这个以太网设备以`/dev/sys/pci/00:02:00/e1000`形式出现。

