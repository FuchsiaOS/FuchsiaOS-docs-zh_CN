 <!--
     (C) Copyright 2019 The Fuchsia Authors. All rights reserved.
     Use of this source code is governed by a BSD-style license that can be
     found in the LICENSE file.
 -->

# USB system overview

Zircon provides a full featured USB subsystem enabling the development of USB
host and peripheral devices. Low, full, high, and super-speed devices are
supported as well as various standard auto-negotiation mechanisms.

In the host role, Zircon's USB subsystem assumes a tiered approach facilitating
the lifetime management of devices as they are attached or removed from the bus.
In the device role, the subsystem marshals USB packets in and out of a
class-specific driver (or hierarchy of drivers).

A target hardware platform may contain numerous USB controllers. As a result,
Zircon may be acting as either a host or device on each respective physical bus.
However, each role is unique to a particular bus topology. This document will
assume that only a single bus is present unless noted otherwise.

The USB subsystem components are summarized as:

- Class-specific hardware driver(s)
- USB hub driver (special case of a class-specific driver)
- Bus driver
- Host or device controller interface driver

## Host role

When operating as a USB host, Zircon acts as the authoritative bus arbiter. The
tree of attached USB devices is rooted at a root USB hub. The presence of this
root hub is required regardless of whether any actual hub hardware exists. For
systems that contain a host-capable controller, but no actual hub hardware, this
root hub must be emulated in software.

To facilitate bus arbitration, Zircon operates the following drivers:

- USB root hub driver
- Bus driver
- Host controller interface (HCI) driver

These drivers operate together to respond to bus attachment and manage the
lifetime of the attached devices.

## Device role

When operating as a USB device, Zircon transports USB packet data between the
bus and the class-specific driver (or hierarchy of drivers). In this role, the
bus driver facilitates communication between the DCI driver and the upper layers
of the class-specific driver(s).

## Class-specific driver

Class-specific drivers implement the logic necessary to fulfill a specific USB
function (e.g. HID-class device) while remaining agnostic of the hardware
details necessary to read and write physical packets from or to the actual bus.

Note: that the hub driver is one example of a class-specific device driver.

In general, USB device drivers encode transfer requests into a `usb_request_t`
structure. These request structs generally have an asynchronous callback
associated with them to be executed upon transfer completion. For the most
part, the USB stack functions by the higher order device drivers publishing
requests to a queue of outstanding requests. As these requests are serviced,
their respective callbacks are invoked notifying the upper layers that the
request is complete.

## Hub driver

The purpose of the hub driver is to manage a hub device according to CH11 of the
[USB 2.0 specification][USB 2.0 spec]. In brief, having undergone device
enumeration, USB hubs use two interfaces to achieve their function:

1. IN-type interrupt endpoint for port status change events
2. IN-type control transfers for port status queries

The Zircon USB stack (which the hub driver is part of) issues a request awaiting
a port status change interrupt event. USB hubs report port status change events
using an N-bit bitmap where bit-1 corresponds to port#1, bit-2 port#2, etc...
Note that bit-0 is reserved for hub status change events, and is currently
unsupported. Thus, a 4-port hub writes a 5-bit value for each of the 4 ports
using an IN-type interrupt endpoint.

Note: While not a requirement, most hubs only produce an interrupt transfer when
there is an actual port status change event.

When the hub device detects a change to one of its ports, it issues an interrupt
transfer encoding the port number. This interrupt transfer unblocks the hub
driver, which reads the port status change bitmap and determines which port(s)
have relevant activity.

Given a port status change event, the USB stack uses the control interface
of the hub device to query the individual port status and proceed as per the
spec. For example, if a port's status changed due to a connection event, the
port would be powered, reset, and enumeration would proceed.

For more information about the specifics of hub lifetime, see CH11 of the [USB
2.0 specification][USB 2.0 spec].

## Bus driver

The purpose of the bus driver is to announce the presence (or removal) of
devices to the bus, and to register the presence of a hub device with the rest
of the USB stack. For the most part, the bus driver simply facilitates
communication between the different parts of the USB stack.

## HCI driver (host only)

The host controller interface (HCI) driver exists at the bottom layer of the USB
stack when operating in host mode. This is the entity responsible for
translating outstanding `usb_request_t` into the necessary hardware directives
capable of servicing the request.

The HCI driver is distinguished from the DCI driver in that it contains
functionality to facilitate device enumeration. If general enumeration is
separated into two phases:

- Bus enumeration (up through the `set_address` command).
- Device enumeration (everything to follow an addressable device).

The HCI driver performs the former half while the USB stack takes over and
performs the rest of the device enumeration.

## DCI driver (device only)

The device controller interface(DCI) driver exists at the bottom layer of the
USB stack when operating in device mode. This is the entity responsible for
translating outstanding `usb_request_t` into the necessary hardware directives
capable of servicing the request.

The DCI driver is distinguished from the HCI driver in that it serves to present
incoming OUT-type transfer requests to the device as well as set up outgoing
IN-type transfer requests to the bus. In both cases, an individual transfer may
result in multiple packets going each direction.

## See also

+ [USB 2.0 spec]

<!-- xref -->

[USB 2.0 spec]: https://www.usb.org/document-library/usb-20-specification
