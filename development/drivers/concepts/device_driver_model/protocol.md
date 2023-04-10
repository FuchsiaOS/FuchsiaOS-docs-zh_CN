# Protocols in drivers

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

## What is a protocol?

A protocol is a strict interface definition.

The ethernet driver published an interface that conforms to `ZX_PROTOCOL_ETHERNET_IMPL`.
This means that it must provide a set of functions defined in a data structure
(in this case, `ethernet_impl_protocol_ops_t`).

These functions are common to all devices implementing the protocol &mdash; for example,
all ethernet devices must provide a function that queries the MAC address of the
interface.

Other protocols will of course have different requirements for the functions they
must provide.
For example a block device will publish an interface that conforms to the
"block implementation protocol" (`ZX_PROTOCOL_BLOCK_IMPL`) and
provide functions defined by `block_protocol_ops_t`.
This protocol includes a function that returns the size of the device in blocks,
for example.

In many cases a Protocol is used to allow drivers to be simpler by taking advantage
of a common implementation of an Interface. For example, the "block" driver implements
the common block interface, and binds to devices implementing the Block Core Protocol,
and the "ethernet" driver does the same thing for the Ethernet Interface and Ethermac
Protocol. Some protocols, such as the two cited here, make use of shared memory, and
non-rpc signaling for more efficient, lower latency, and higher throughput than could
be achieved otherwise.

Classes represent a promise that a device implements an Interface or Protocol.
Devices exist in the Device Filesystem under a topological path, like
`/sys/platform/pci/00:02:00/e1000`. If they are a specific class, they also appear
as an alias under `/dev/class/CLASSNAME/...`. The `e1000` driver implements
the Ethermac interface, so it also shows up at `/dev/class/ethermac/000`. The names
within class directories are unique but not meaningful, and are assigned on demand.

Note: Currently names in class directories are 3 digit decimal numbers, but they
are likely to change form in the future. Clients should not assume there is any
specific meaning to a class alias name.

Example protocols:

*   the PCI root protocol (`ZX_PROTOCOL_PCIROOT`),
*   the PCI device protocol (`ZX_PROTOCOL_PCI`), and
*   the ethernet implementation protocol (`ZX_PROTOCOL_ETHERNET_IMPL`).

The names in brackets are the C language constants corresponding to the protocols, for reference.


## Platform dependent vs platform independent

Above, we mentioned that `ZX_PROTOCOL_ETHERNET_IMPL` was "close to" the functions used
by the client, but one step removed.
That's because there's one more protocol, `ZX_PROTOCOL_ETHERNET`, that sits between
the client and the driver.
This additional protocol is in place to handle functionality common to all ethernet
drivers (in order to avoid code duplication).
Such functionality includes buffer management, status reporting, and administrative
functions.

This is effectively a "platform dependent" vs "platform independent" decoupling;
common code exists in the platform independent part (once), and driver-specific code
is implemented in the platform dependent part.

This architecture is repeated in multiple places.
With block devices, for example, the hardware driver binds to the bus (e.g., PCI)
and provides a `ZX_PROTOCOL_BLOCK_IMPL` protocol.
The platform independent driver binds to `ZX_PROTOCOL_BLOCK_IMPL`, and publishes the
client-facing protocol, `ZX_PROTOCOL_BLOCK`.

You'll also see this with the display controllers, I<sup>2</sup>C bus, and serial drivers.

<!--- More content? -->

## Process / protocol mapping

In order to keep the discussions above simple, we didn't talk about process separation
as it relates to the drivers.
To understand the issues, let's see how other operating systems deal with them,
and compare that to the Fuchsia approach.

In a monolithic kernel, such as Linux, many drivers are implemented within the kernel.
This means that they share the same address space, and effectively live in the same
"process."

The major problem with this approach is fault isolation / exploitation.
A bad driver can take out the entire kernel, because it lives in the same address
space and thus has privileged access to all kernel memory and resources.
A compromised driver can present a security threat for the same reason.

The other extreme, that is, putting each and every driver service into its own
process, is used by some microkernel operating systems.
Its major drawback is that if one driver relies on the services of another driver,
the kernel must effect at least a context switch operation (if not a data transfer
as well) between the two driver processes.
While microkernel operating systems are usually designed to be fast at these
kinds of operations, performing them at high frequency is undesirable.

The approach taken by Fuchsia is based on the concept of a driver host.
A driver host is a process that contains a protocol stack &mdash; that is, one or
more protocols that work together.
The driver host loads drivers from ELF shared libraries (called Dynamic Shared Objects,
or **DSO**s).

The protocol stack effectively allows the creation of a complete "driver" for
a device, consisting of platform dependent and platform independent components,
in a self-contained process container.

For the advanced reader, take a look at the `driver dump` command available from
the Fuchsia command line. It displays a tree of devices, and shows you the
process ID, DSO name, and other useful information.

Here's a highly-edited version showing just the PCI ethernet driver parts:

```
1. [root]
2.    [sys]
3.       <sys> pid=1416 /boot/driver/bus-acpi.so
4.          [acpi] pid=1416 /boot/driver/bus-acpi.so
5.          [pci] pid=1416 /boot/driver/bus-acpi.so
            ...
6.             [00:02:00] pid=1416 /boot/driver/bus-pci.so
7.                <00:02:00> pid=2052 /boot/driver/bus-pci.proxy.so
8.                   [e1000] pid=2052 /boot/driver/e1000.so
9.                      [ethernet] pid=2052 /boot/driver/ethernet.so
```

From the above, you can see that process ID `1416` (lines 3 through 6)
is the Advanced Configuration and Power Interface (**ACPI**) driver, implemented
by the DSO `bus-acpi.so`.

During primary enumeration, the ACPI DSO detected a PCI bus.
This caused the publication of a parent with `ZX_PROTOCOL_PCI_ROOT` (line 5,
causing the appearance of the `[pci]` entry),
which then caused the driver host to load the `bus-pci.so` DSO and bind to it.
That DSO is the "base PCI driver" to which we've been referring throughout the
discussions above.

During its binding, the base PCI driver enumerated the PCI bus, and found an ethernet
card (line 6 detects bus 0, device 2, function 0, shown as `[00:02:00]`).
(Of course, many other devices were found as well, but we've removed them from
the above listing for simplicity).

The detection of this device then caused the base PCI driver to publish a new parent
with `ZX_PROTOCOL_PCI` and the device's VID and DID.
Additionally, a new driver host (process ID `2052`) was created and loaded with the
`bus-pci.proxy.so` DSO (line 7).
This proxy serves as the interface from the new driver host (pid `2052`) to the base PCI
driver (pid `1416`).

> This is where the decision was made to "sever" the device driver into its own
> process &mdash; the new driver host and the base PCI driver now live in two
> different processes.

The new driver host `2052` then finds a matching child (the `e1000.so`
DSO on line 8; it's considered a match because it has `ZX_PROTOCOL_PCI` and the correct
VID and DID).
That DSO publishes a `ZX_PROTOCOL_ETHERNET_IMPL`, which binds to a matching
child (the `ethernet.so` DSO on line 9; it's considered a match because it has a
`ZX_PROTOCOL_ETHERNET_IMPL` protocol).

What's not shown by this chain is that the final DSO (`ethernet.so`) publishes
a `ZX_PROTOCOL_ETHERNET` &mdash; that's the piece that clients can use, so of
course there's no further "device" binding involved.

### Driver Framework Version 2 (DFv2)

If driver framework version 2 is enabled, `driver dump` will show a slightly
different tree.

```sh
$ driver dump
[root] pid=4766 fuchsia-boot:///#meta/platform-bus.cm
   [sys] pid=4766
      [platform] pid=4766
         [pt] pid=4766 fuchsia-boot:///#meta/platform-bus-x86.cm
            [acpi] pid=4766
               [acpi-pwrbtn] pid=4766 fuchsia-boot:///#meta/hid.cm
               ...
            [PCI0] pid=4766 fuchsia-boot:///#meta/bus-pci.cm
               [bus] pid=4766
                 ...
                 [00_04_0] pid=4766 fuchsia-boot:///#meta/virtio_ethernet.cm
                    [virtio-net] pid=4766 fuchsia-boot:///#meta/netdevice-migration.cm
                       [netdevice-migration] pid=4766 fuchsia-boot:///#meta/network-device.cm
                          [network-device] pid=4766
        ...
```

It is important to point out that nodes (devices are referred to as nodes in
DFv2) do not have an `.so` file associated with them. Instead, there is a URL of
the component manifest of the driver that is attached to a given node.
