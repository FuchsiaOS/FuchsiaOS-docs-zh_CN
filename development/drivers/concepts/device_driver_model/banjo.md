# Banjo in drivers

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

Devices may implement protocols, which are Banjo ABIs used by child devices
to interact with parent devices in a device-specific manner. The
[USB Protocol](/sdk/banjo/fuchsia.hardware.usb/usb.fidl),
[Block Core Protocol](/sdk/banjo/fuchsia.hardware.block/block.fidl), and
[Ethernet Protocol](/sdk/banjo/fuchsia.hardware.ethernet/ethernet.fidl), are
examples of these. Protocols are usually in-process interactions between
devices in the same driver host, but in cases of driver isolation, they may take
place through RPC to another driver host (through proxy).

See [Banjo Tutorial](/development/drivers/tutorials/banjo-tutorial.md) to learn how to use Banjo.
