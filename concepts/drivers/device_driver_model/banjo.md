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
