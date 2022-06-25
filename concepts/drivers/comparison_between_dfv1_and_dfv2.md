# Comparison between DFv1 and DFv2

This page provides a quick overview of the differences between the two versions
of Fuchsia’s driver framework: [DFv1][dfv1] (legacy) and [DFv2][dfv2].

Key differences between DF1 and DFv2 are:

* [Device vs. node](#device-vs-node)
* [libDDK vs. capabilities](#libddk-vs-capabilities)
* [Banjo vs. FIDL](#banjo-vs-fidl)

## Device vs. node {:#device-vs-node}

In DFv1, we use the terms “[device][device]” and “device graph.” A device can be thought
of as a piece of hardware. Drivers are bound to devices and operate on them.
Drivers can also create child devices. In DFv1, when a driver binds to a device,
it must create a child device. The device is then owned by the driver that
created it.

In DFv2, we use the terms “[node][node]” and “node graph” (or "node topology”).
A node can be thought of as something that exposes capabilities in a Fuchsia system.
It could be a physical hardware device or a virtual representation of hardware.
Drivers are bound to nodes and use their capabilities. Drivers can also create
child nodes. The node is then owned by the driver that is bound to it.

## libDDK vs. capabilities {:#libddk-vs-capabilities}

In DFv1, drivers are not components. They do not have an incoming or outgoing
namespace to use capabilities in a Fuchsia system. Drivers communicate with the
driver framework using [`libDDK`][device-driver-lifecycle], which is a shared library
that exposes functions. Drivers create a messageable device if they want to be placed
in the `/dev` directory (a virtual file system). Then the driver framework will forward
FIDL messages back to the driver.

In DFv2, drivers are [components][components]. They have capabilities in their incoming
namespace. Some of these capabilities let them speak FIDL to the driver framework.
Drivers can use the [`DevfsExporter`][devfs] FIDL protocol to expose a channel to the
`/dev` directory. Using this channel, drivers and other components can speak FIDL
directly to each other.

## Banjo vs. FIDL {:#banjo-vs-fidl}

In DFv1, drivers speak [Banjo][banjo] to each other. To get the Banjo protocol, a
driver requests it from its bound device.

In DFv2, drivers speak [FIDL][fidl] to each other, like any other component in a
Fuchsia system. A driver gets a FIDL channel from its incoming component namespace.

[dfv1]: development/drivers/concepts/fdf.md
[dfv2]: concepts/drivers/driver_framework.md
[device]: development/drivers/concepts/device_driver_model/device-model.md
[node]: concepts/drivers/drivers_and_nodes.md
[node-topology]: concepts/drivers/drivers_and_nodes.md#node_topology
[device-driver-lifecycle]: development/drivers/concepts/device_driver_model/device-lifecycle.md
[devfs]: concepts/drivers/driver_communication.md
[components]: concepts/components/v2/README.md
[banjo]: development/drivers/concepts/device_driver_model/banjo.md
[fidl]: concepts/fidl/overview.md

