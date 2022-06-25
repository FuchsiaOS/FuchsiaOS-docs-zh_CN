# Drivers

Drivers provide software interfaces for communicating with hardware (or virtual)
devices that are embedded in or connected to a system. In Fuchsia, drivers are
user-space [components][components]. Like any other Fuchsia component, a driver
is software that exposes and receives FIDL capabilities to and from other
components in the system. Using these FIDL calls, Fuchsia components interact
with drivers, which are bound to specific devices in the system.

Similar to Fuchsia’s component framework, which manages Fuchsia components, the
[driver framework][driver-framework] manages the lifecycle and topology of
all devices (known as [nodes][nodes]) and drivers in a Fuchsia system.

## Table of contents

Important: These pages contain information that is specific to the new
version of the driver framework (DFv2).

*  [Driver framework (DFv2)][driver-framework]
   *  [Driver manager][driver-manager]
   *  [Driver host][driver-host]
   *  [Driver index][driver-index]
   *  [Driver runtime][driver-runtime]
   *  [FIDL interface][fidl-interface]
*  [Comparison between DFv1 and DFv2][dfv1-and-dfv2]
*  [Drivers and nodes][nodes]
   *  [Node properties][node-properties]
   *  [Node capabilities][node-capabilities]
   *  [Node topology][node-topology]
   *  [Node lifecycle][node-lifecycle]
   *  [Composite nodes][composite-nodes]
* [Driver binding][driver-binding]
   *  [Binding sequence][binding-sequence]
   *  [Board driver and USB devices][board-driver-and-usb-devices]
* [Driver communication][driver-communication]
   *   [Service discovery (using `devfs`)][service-discovery]
   *   [FIDL communication][fidl-communication]

<!-- Reference links -->

[components]: concepts/components/v2/README.md
[driver-framework]: driver_framework.md
[driver-manager]: driver_framework.md#driver_manager
[driver-host]: driver_framework.md#driver_host
[driver-index]: driver_framework.md#driver_index
[driver-runtime]: driver_framework.md#driver_runtime
[fidl-interface]: driver_framework.md#fidl_interface
[dfv1-and-dfv2]: comparison_between_dfv1_and_dfv2.md
[nodes]: drivers_and_nodes.md
[node-properties]: drivers_and_nodes.md#node_properties
[node-capabilities]: drivers_and_nodes.md#node_capabilities
[node-topology]: drivers_and_nodes.md#node_topology
[node-lifecycle]: drivers_and_nodes.md#node_lifecycle
[composite-nodes]: drivers_and_nodes.md#composite_nodes
[driver-binding]: driver_binding.md
[binding-sequence]: driver_binding.md#binding_sequence
[board-driver-and-usb-devices]: driver_binding.md#board_driver_and_usb_devices
[driver-communication]: driver_communication.md
[service-discovery]: driver_communication.md#service_discovery_using_devfs
[fidl-communication]: driver_communication.md#fidl_communication
