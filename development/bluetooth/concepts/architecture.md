# Bluetooth architecture

The Fuchsia Bluetooth system aims to provide a dual-mode implementation of the
Bluetooth Host Subsystem versions 4.2 and 5.0+. This includes

- A framework for developing Bluetooth Low Energy applications in central,
  peripheral, broadcaster, and scanner roles.

- DDK interfaces for building LE and Traditional service drivers that have
  high throughput requirements.

- A DDK surface for building vendor-specific HCI drivers to support a wide range
  of Bluetooth controllers as well as generic transport drivers.

- Services for policy and management to integrate a system with the Generic
  Access Profile.

## Device stack

Bluetooth controllers that are present on the system surface as a hierarchy of
devices. On an x86 platform this hierarchy may look like the following:

```
[pci] pid=1633 /boot/driver/bus-acpi.so
   [00:14.0] pid=1633 /boot/driver/bus-pci.so
      <00:14.0> pid=2179 /boot/driver/bus-pci.proxy.so
         [xhci] pid=2179 /boot/driver/xhci.so
            [usb] pid=2179 /boot/driver/usb-bus.so
               [005] pid=2179 /boot/driver/usb-bus.so
                  [ifc-000] pid=2179 /boot/driver/usb-bus.so
                     [bt_transport_usb] pid=2179 /boot/driver/bt-transport-usb.so
                        [bthci-passthrough] pid=2179 /system/driver/bthci-passthrough.so
                           [bt-host] pid=2179 /system/driver/bthost.so
```

### HCI

Generic HCI transport functionality is provided by the `bt-transport` protocol.
Fuchsia provides drivers that implement the HCI transport over
[USB](/src/connectivity/bluetooth/hci/transport/usb/)
and [UART](/src/connectivity/bluetooth/hci/transport/uart/).
The transport protocol abstracts the HCI control, ACL, and SCO
channels (currently as Zircon [channels](reference/kernel_objects/channel.md)).

A transport driver publishes a bt-transport device (e.g. `/dev/class/bt-transport/000`).
Each of these devices only represents the transport and not an initialized
Bluetooth controller since most Bluetooth controllers require vendor-specific protocols
for their setup (e.g. to load firmware). That logic is implemented by vendor HCI
drivers that bind to a bt-transport device.

Vendor drivers have access to the bt-transport protocol for HCI transactions, as
well as other underlying protocols that the transport device supports. Once a
Bluetooth controller has been initialized and is ready for the host subsystem,
the vendor driver publishes a `bt-hci` device.

The system provides the `bthci-passthrough` driver, which binds to bt-transport
devices that are not claimed by any vendor-specific driver. bthci-passthrough
simply publishes a bt-hci device without doing special initialization.

### Host

The `bthost` driver implements the core Bluetooth protocols that form the
Generic Access Profile. bthost binds to bt-hci devices and publishes `bt-host`
devices. A bt-host device claims the HCI control and data endpoints of the underlying
`bt-hci` and implements:

* The core dual-mode GAP bookkeeping
* Handling of FIDL messages for core services
* L2CAP and fixed channel protocols (GATT, SMP, SDP)
* Pairing protocols and delegation
* Other types of IPC (such as L2CAP sockets)
* Bus protocol for child devices for services implemented as device drivers

Host devices are managed by the
[Bluetooth system service](/src/connectivity/bluetooth/).
The service allows only one bt-host to be accessed for service requests at a given
time. This bt-host is represented as the "active Adapter".
[host_watcher.fidl](/sdk/fidl/fuchsia.bluetooth.sys/host_watcher.fidl) provides
a management interface to designate an active adapter when multiple adapters are
present.

bt-host devices implement the [host.fidl](/src/connectivity/bluetooth/fidl/host.fidl)
protocol to communicate with the Bluetooth system service.


### Host bus

TODO(armansito): child devices

## Services

Bluetooth environment services are the primary way to implement Bluetooth
services and applications.

The [Sys](/sdk/fidl/fuchsia.bluetooth.sys) FIDL library is
intended for privileged clients and is for device-level control/policy.

TODO: describe other services
