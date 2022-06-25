# Driver communication

Important: This page contains information that is specific to the new
version of the driver framework (DFv2).

In Fuchsia, all communication occurs over FIDL calls, for both drivers and
non-drivers. What differs is how drivers' services are discovered and how
connection is established.

For driver-to-driver communication, Fuchsia uses the
[node topology][node-topology] to place parent nodes' capabilities in a child
node's incoming FIDL namespace (that is, under `/svc` as directories and files).
This setup enables a driver (once bound to the child node) to access FIDL
services inherited from the parent nodes.

However, communication from a non-driver component to a driver takes place in
two phases:

1. [Service discovery (using devfs)](#service_discovery_using_devfs)
1. [FIDL communication](#fidl_communication)

For non-driver components, the first task is to discover which drivers' services
are available in the system. These services are provided by the drivers that are
currently bound to nodes representing hardware or virtual devices in the system.
A filesystem known as `devfs` provides a mechanism for discovering these services.

The following events take place for non-driver to driver communication:

1. To discover driver services in the system, a non-driver component scans the
   directories and files in `devfs`.
2. The non-driver component finds a file in `devfs` that represents a service
   provided by the target driver.
3. The non-driver component opens this file and contacts the target driver.
4. After the initial contact, a FIDL connection is established between the
   non-driver component and the driver.
5. From this point, all communication takes place over the FIDL channels.

Note: Fuchsia's expectation is that non-drivers will also discover
driver services using FIDL in the near future. Meanwhile, Fuchsia will continue
to support `devfs`. However, it will be deprecated at some point.

## Service discovery (using devfs)

The [driver manager][driver-manager] hosts a virtual filesystem named `devfs`
(as in "device filesystem"). This virtual filesystem provides uniform access to
all driver services in a Fuchsia system to Fuchsia’s user-space services
(that is, components external to the drivers). These non-driver components
establish initial contacts with drivers by discovering the services of the
target drivers in `devfs`.

Strictly speaking, `devfs` is a directory capability exposed by the driver
manager. Therefore, by convention, components that wish to access drivers mount
`devfs` under the `/dev` directory in their namespace (although it’s not
mandated that `devfs` to be always mounted under `/dev`).

`devfs` hosts virtual files that enable Fuchsia components to route messages to
the interfaces implemented by the drivers running in a Fuchsia system.
In other words, when a client (that is, a non-driver component) opens a file
under the `/dev` directory, it receives a channel that can be used to make
FIDL calls directly to the driver mapped to the file. For example,
a Fuchsia component can connect to an input device by opening and writing to
a file that looks like `/dev/class/input-report/000`. In this case,
the client may receive a channel that speaks the `fuchsia.input.report` FIDL.

Drivers can use the [`fuchsia.device.fs`][fuchsia-device-fs] protocol to export
themselves into `devfs`. The path under the `/dev` directory for a driver is the
direct reflection of the path in the node topology. Therefore, if a driver wants
to show up in the right place in `devfs`, it needs to make sure that its
topological path and protocol correspond correctly.

## FIDL communication

Once an initial contact is made between non-driver and driver components
using `devfs`, the components can exchange FIDL handles. From this point,
these components make use of FIDL calls for communication, just like any
other components in Fuchsia.

Drivers, as [Fuchsia components][components], have an incoming FIDL namespace
filled with capabilities the drivers can use. Some of these capabilities may be
inherited from their parent driver (for example, a PCI device will have
a `fuchsia.hardware.PCI` capability from its parent node). Drivers can use
these capabilities to make FIDL calls to their parent drivers. Similarly,
their clients (that is, non-driver components) can also use the capabilities
received from the drivers to make FIDL calls to the drivers.

These FIDL calls, by default, get routed through the Zircon kernel. However,
if the target driver is in the same process (therefore, in the same
driver host), the [driver runtime][driver-runtime] can route
the FIDL calls to remain in process, without going in and out of
the Zircon kernel.

<!-- Reference links -->

[driver-manager]: driver_framework.md#driver_manager
[driver-runtime]: driver_framework.md#driver_runtime
[node-topology]: drivers_and_nodes.md#node_topology
[fuchsia-device-fs]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.device.fs/exporter.fidl;l=12
[components]: concepts/components/v2/README.md
