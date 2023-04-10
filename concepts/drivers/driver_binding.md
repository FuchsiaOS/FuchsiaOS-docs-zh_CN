# Driver binding

Important: This page contains information that is specific to the new
version of the driver framework (DFv2).

To provide services for devices in a Fuchsia system, drivers must be bound to
[nodes][nodes] that represent the devices. The [driver manager][driver-manager]
maintains the topology of nodes, where each node represents access to a hardware
or virtual device in the system. When a driver is matched to a node, the
driver can bind to the node. Once bound to the node, the driver can start
providing services for the device that the node represents. For example, a USB
keyboard driver may bind to a node representing a keyboard device.

To identify which drivers can bind to which nodes, each driver has
bind rules and each node has a set of [node properties][node-properties].
A driver’s bind rules describe the qualification of a node that the driver can
serve effectively. When the driver framework attempts to match a driver to
a node, each unbound node’s properties are compared to the driver’s
bind rules. If the node properties satisfy the driver’s bind rules,
the driver framework allows the driver to bind to the node.

## Binding sequence

When a Fuchsia system boots up, the driver manager tries to construct a node
topology that represents all the hardware and virtual devices in the system,
and the [driver index][driver-index] enumerates all the drivers known to
the system.

The following events take place during the initial booting of a Fuchsia system:

1. (Starting with the root node and its driver) A driver requests that the
   driver manager creates a new child node.
2. The driver manager asks the driver index to find out which driver
   best matches this node's properties:
    1. The driver index compares each known driver's bind rules against the
       node's properties.
    2. The driver index returns the matched driver’s URL to the driver manager.
3. The driver manager binds the driver to the node:
    1. The driver manager creates (or assigns) a driver host for the driver.
    2. The [driver host][driver-host] starts an instance of the driver.
4. The running driver may decide to create a child node.
    1. The process repeats from Step 1.

After the initial run of scanning and binding, whenever a new driver appears
(for instance, a new driver is loaded to the system), the driver manager sends
all unbound nodes in the topology to the driver index to be matched against
the new driver. When a node is matched, the driver manager
binds this new driver to the node, an instance of the driver is placed
in a driver host, and the driver host starts serving the device’s capabilities
to other Fuchsia components in the system.

For more details on bind rules, see [Driver binding][driver-binding-dfv1], which
was written previously for the driver framework version 1 (DFv1).

## Board driver and USB devices

While drivers are often bound to devices, some drivers are bound to
[boards](/docs/glossary/README.md#board) (such as PCI and ACPI) that may
have multiple devices connected to them, both statically and dynamically.

Upon the initial binding to a node, a [board driver](/docs/glossary/README.md#board-driver)
(such as `acpi`) parses a binary blob passed from the system (which can be
ACPI bytecode or a compiled device tree) and informs the driver manager of the
static set of devices connected on the board. These devices get bound to drivers
through the normal binding process orchestrated by the driver index.  From this point,
these drivers (that are bound to the child nodes of the board driver) dynamically
query the hardware for additional information. From this information, the
drivers may discover new devices to be added to the topology. This process
occurs recursively as more devices are discovered and introduced to the
topology.

<!-- Reference links -->

[components]: /docs/concepts/components/v2/README.md
[driver-framework]: driver_framework.md
[driver-manager]: driver_framework.md#driver_manager
[driver-host]: driver_framework.md#driver_host
[driver-index]: driver_framework.md#driver_index
[driver-runtime]: driver_framework.md#driver_runtime
[fidl-interface]: driver_framework.md#fidl_interface
[nodes]: drivers_and_nodes.md
[node-properties]: drivers_and_nodes.md#node_attributes
[node-capabilities]: drivers_and_nodes.md#node_capabilities
[driver-binding-dfv1]: /docs/development/drivers/concepts/device_driver_model/driver-binding.md
