# Overview

At the highest level, a device driver's job is to provide a uniform interface to
a particular device, while hiding details specific to the device's implementation.

Two different ethernet drivers, for example, both allow a client to send packets
out an interface, using the exact same C language function.
Each driver is responsible for managing its own hardware in a way that makes the
client interfaces identical, even though the hardware is different.

Note that the interfaces that are provided by the driver may be "intermediate" &mdash;
that is, they might not necessarily represent the "final" device in the chain.

Consider a PCI-based ethernet device.
First, a base PCI driver is required that understands how to talk to the PCI bus itself.
This driver doesn't know anything about ethernet, but it does know
how to deal with the specific PCI chipset present on the machine.

It enumerates the devices on that bus, collects information
from the various registers on each device, and provides functions that allow
its clients (such as the PCI-based ethernet driver) to perform PCI operations
like allocating an interrupt or a DMA channel.

Thus, this base PCI driver provides services to the ethernet driver, allowing
the ethernet driver to manage its associated hardware.

At the same time, other devices (such as a video card) could also use the base PCI
driver in a similar manner to manage their hardware.

# The Fuchsia model

In order to provide maximum flexibility, drivers in the Fuchsia world are allowed
to bind to matching "parent" devices, and publish "children" of their own.
This hierarchy extends as required: one driver might publish a child, only to have
another driver consider that child their parent, with the second driver publishing
its own children, and so on.

In order to understand how this works, let's follow the PCI-based ethernet example.

The system starts by providing a special "PCI root" parent.
Effectively, it's saying "I know that there's a PCI bus on this system, when you
find it, bind it *here*."

Drivers are evaluated by the system (a directory is searched), and drivers that
match are automatically bound.

In this case, a driver that binds to a "PCI root" parent is found, and bound.

This is the base PCI driver.
It's job is to configure the PCI bus, and enumerate the peripherals on the bus.

The PCI bus has specific conventions for how peripherals are identified:
a combination of a Vendor ID (**VID**) and Device ID (**DID**) uniquely identifies
all possible PCI devices.
During enumeration, these values are read from the peripheral, and new parent
nodes are published containing the detected VID and DID (and a host of other
information).

Every time a new device is published, the same process as described above (for
the initial PCI root device publication) repeats;
that is, drivers are evaluated by the system, searching for drivers that match
up with the new parents' characteristics.

Whereas with the PCI root device we were searching for a driver that matched
a certain kind of functionality (called a "protocol," we'll see this shortly), in
this case, however, we're searching for drivers that match a different
protocol, namely one that satisfies the requirements of "is a PCI device and
has a given VID and DID."

If a suitable driver is found (one that matches the required protocol, VID and
DID), it's bound to the parent.

As part of binding, we initialize the driver &mdash; this involves such operations
as setting up the card for operation, bringing up the interface(s), and
publishing a child or children of this device.
In the case of the PCI ethernet driver, it publishes the "ethernet" interface,
which conforms to yet another protocol, called the "ethernet implementation" protocol.
This protocol represents a common protocol that's close to the functions that
clients use (but is one step removed; we'll come back to this).

