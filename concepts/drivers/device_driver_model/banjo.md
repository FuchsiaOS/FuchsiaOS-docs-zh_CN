<!---

# Banjo in drivers

Devices may implement protocols, which are Banjo ABIs used by child devices
to interact with parent devices in a device-specific manner. The
[PCI Protocol](/sdk/banjo/fuchsia.hardware.pci/pci.fidl),
[USB Protocol](/sdk/banjo/fuchsia.hardware.usb/usb.fidl),
[Block Core Protocol](/sdk/banjo/fuchsia.hardware.block/block.fidl), and
[Ethernet Protocol](/sdk/banjo/fuchsia.hardware.ethernet/ethernet.fidl), are
examples of these. Protocols are usually in-process interactions between
devices in the same driver host, but in cases of driver isolation, they may take
place through RPC to another driver host (through proxy).

See [Banjo Tutorial](/docs/development/drivers/tutorials/banjo-tutorial.md) to learn how to use Banjo.

--->

# 驱动中的Banjo

设备可以实现协议，这些协议是用于子设备和父设备以设备特定的方式交互的Banjo ABI。 其中

[PCI Protocol](/sdk/banjo/fuchsia.hardware.pci/pci.fidl),
[USB Protocol](/sdk/banjo/fuchsia.hardware.usb/usb.fidl),
[Block Core Protocol](/sdk/banjo/fuchsia.hardware.block/block.fidl)和

[Ethernet Protocol](/sdk/banjo/fuchsia.hardware.ethernet/ethernet.fidl)是一些协议示例。协议通常是同一驱动主机内设备之间的进程间通信，但是在驱动隔离的前提下，它们可能通过远程调用（RPC）到另一个驱动主机（通过代理）进行通信。

查看[Banjo Tutorial](/docs/development/drivers/tutorials/banjo-tutorial.md) 了解更多关于怎样使用Banjo。

