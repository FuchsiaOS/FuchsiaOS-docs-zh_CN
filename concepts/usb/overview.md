 <!--
     (C) Copyright 2019 The Fuchsia Authors. All rights reserved.
     Use of this source code is governed by a BSD-style license that can be
     found in the LICENSE file.
 -->

# USB system overview
# USB系统概述

Zircon provides a full featured USB subsystem enabling the development of USB
host and peripheral devices. Low, full, high, and super-speed devices are
supported as well as various standard auto-negotiation mechanisms.

Zircon提供了一个功能完备的USB子系统，它能够使您开发USB主设备（USB host）
和外围设备。它支持低速、全速、高速和超高速传输USB设备，以及各种标准的
自动协商（auto-negotiation）机制。

In the host role, Zircon's USB subsystem assumes a tiered approach facilitating
the lifetime management of devices as they are attached or removed from the bus.
In the device role, the subsystem marshals USB packets in and out of a
class-specific driver (or hierarchy of drivers).

在Zircon的USB子系统作为主设备（host）时，随着USB设备连接到总线（bus）或者从
总线中移除，Zircon的USB子系统会使用一种层级式的方式，以便于USB设备的生命周期
管理。在Zircon的USB子系统作为从属设备时，子系统借助特定种类的驱动（或者多重等级
的USB驱动）发送或者接收USB数据包。

A target hardware platform may contain numerous USB controllers. As a result,
Zircon may be acting as either a host or device on each respective physical bus.
However, each role is unique to a particular bus topology. This document will
assume that only a single bus is present unless noted otherwise.

目标硬件平台可能会包含许多个USB控制器。因此，Zircon在各个物理总线上的工作模式
既有可能是主设备（host）模式，也有可能是从属设备模式。但是，对于一个特定的
总线拓扑，每个工作模式是唯一的。除非另外提及，这篇文档一般考虑的是只有一个总线
的情况。

The USB subsystem components are summarized as:

- Class-specific hardware driver(s)
- USB hub driver (special case of a class-specific driver)
- Bus driver
- Host or device controller interface driver

USB子系统的组件概括如下：

- 特定种类的硬件驱动
- USB集线器驱动（特定种类的硬件驱动的一种特殊情形）
- 总线驱动
- 主设备或从属设备控制器的接口驱动

## Host role

## Zircon的USB主设备模式

When operating as a USB host, Zircon acts as the authoritative bus arbiter. The
tree of attached USB devices is rooted at a root USB hub. The presence of this
root hub is required regardless of whether any actual hub hardware exists. For
systems that contain a host-capable controller, but no actual hub hardware, this
root hub must be emulated in software.

当作为一个USB主设备时，Zircon是一个权威的仲裁者。所连接的USB设备形成的
树状拓扑的根部（root）是一个USB根集线器（USB root hub）。无论总线设备
是否实际存在，这个USB根集线器都是必须要有的。对于包含具有主设备功能的
控制器，但是没有实际的总线设备的系统，必须使用软件来模拟一个这样的USB
根集线器。

To facilitate bus arbitration, Zircon operates the following drivers:

- USB root hub driver
- Bus driver
- Host controller interface (HCI) driver

为了便于总线仲裁（bus arbitration），Zircon会使用下面的驱动进行操作：

- USB根集线器驱动
- 总线驱动
- 主机控制器接口（Host controller interface，HCI）驱动

These drivers operate together to respond to bus attachment and manage the
lifetime of the attached devices.

这些驱动协同工作，来对接入总线的设备作出反应，以及管理接入的设备的生命周期。

## Device role

## Zircon的USB从属设备模式

When operating as a USB device, Zircon transports USB packet data between the
bus and the class-specific driver (or hierarchy of drivers). In this role, the
bus driver facilitates communication between the DCI driver and the upper layers
of the class-specific driver(s).

当作为一个USB从属设备时，Zircon在总线和特定种类的驱动（或者多重等级的驱动）
传输USB数据包。在这个模式下，总线驱动使得DCI驱动和特定种类的驱动的顶部
抽象层之间能够进行通信。

## Class-specific driver

## 特定种类的驱动

Class-specific drivers implement the logic necessary to fulfill a specific USB
function (e.g. HID-class device) while remaining agnostic of the hardware
details necessary to read and write physical packets from or to the actual bus.

为了实现特定的USB功能（例如，HID种类的设备），特定种类的驱动里面会相应地
实现必要的逻辑。与此同时，它不会关心向实际的总线读取或写入物理数据包所需要的
硬件信息。

Note: that the hub driver is one example of a class-specific device driver.

注意：集线器驱动是特定种类的驱动的一个例子。

In general, USB device drivers encode transfer requests into a `usb_request_t`
structure. These request structs generally have an asynchronous callback
associated with them to be executed upon transfer completion. For the most
part, the USB stack functions by the higher order device drivers publishing
requests to a queue of outstanding requests. As these requests are serviced,
their respective callbacks are invoked notifying the upper layers that the
request is complete.

一般而言，USB设备的驱动将数据传输请求编码成一个“usb_request_t”结构体。这些
请求结构体一般具有相应的一个异步的回调（callback）函数，在数据传输完成时，这个
回调函数会被执行。通常而言，USB协议栈的运作是通过高阶（higher order）设备驱动
向尚未处理的请求队列发送请求来实现的。当这些请求被处理，与它们对应的回调函数会
被调用，以通知顶部抽象层：该请求已经处理完成。

## Hub driver

## 集线器驱动

The purpose of the hub driver is to manage a hub device according to CH11 of the
[USB 2.0 specification][USB 2.0 spec]. In brief, having undergone device
enumeration, USB hubs use two interfaces to achieve their function:

1. IN-type interrupt endpoint for port status change events
2. IN-type control transfers for port status queries

集线器驱动的目的是根据《USB 2.0规格（USB 2.0 spec）》中的第11章来
管理集线器设备。简单而言，在进行设备枚举之后，USB集线器使用两种接口来实现
它们的功能：

1. 对于端口状态改变的事件，使用IN类型中断端点（interrupt endpoint）
2. 对于端口状态查询，使用IN类型控制传输

The Zircon USB stack (which the hub driver is part of) issues a request awaiting
a port status change interrupt event. USB hubs report port status change events
using an N-bit bitmap where bit-1 corresponds to port#1, bit-2 port#2, etc...
Note that bit-0 is reserved for hub status change events, and is currently
unsupported. Thus, a 4-port hub writes a 5-bit value for each of the 4 ports
using an IN-type interrupt endpoint.

Zircon的USB协议栈（集线器驱动是其中的一个部分）发出一个请求，等待端口状态
改变的中断事件。USB集线器使用一个包含N个比特的位图（bitmap）来报告端口状态
改变的中断事件，这个位图中，第1个比特对应端口#1，第二个比特对应端口#2，等等。
请注意，第0个比特是为集线器状态改变的事件而保留的，此功能目前还不支持。因此，
一个包含4个端口的集线器会使用IN类型中断端点向每一个端口写入一个包含5个比特的值。

Note: While not a requirement, most hubs only produce an interrupt transfer when
there is an actual port status change event.

请注意：虽然没有做出要求，但是大多数集线器只在有端口状态改变事件实际发生的时候
产生一个中断传输（interrupt transfer）。

When the hub device detects a change to one of its ports, it issues an interrupt
transfer encoding the port number. This interrupt transfer unblocks the hub
driver, which reads the port status change bitmap and determines which port(s)
have relevant activity.

当集线器设备侦测到它的一个端口的状态发生了改变，它对端口号进行编码并产生
一个中断传输。这个中断传输使集线器驱动进行下面的工作：读取端口状态改变位图，
并且判断决定哪个端口有相关的活动。

Given a port status change event, the USB stack uses the control interface
of the hub device to query the individual port status and proceed as per the
spec. For example, if a port's status changed due to a connection event, the
port would be powered, reset, and enumeration would proceed.

在一个端口状态改变事件发生的情况下，USB协议栈使用集线器设备的控制接口
（control interface）来查询各个端口的状态，并且按照USB规格说明进行工作。
例如，如果一个端口的状态因为有了连接事件（connection event）而发生了改变，
这个端口将会通电，重置，并且继续进行这种枚举查询工作。

For more information about the specifics of hub lifetime, see CH11 of the [USB
2.0 specification][USB 2.0 spec].

要获取关于集线器生命周期的细节的信息，参考[USB 2.0 规格]的第11章。

## Bus driver

## 总线驱动

The purpose of the bus driver is to announce the presence (or removal) of
devices to the bus, and to register the presence of a hub device with the rest
of the USB stack. For the most part, the bus driver simply facilitates
communication between the different parts of the USB stack.

总线驱动的目的是向总线通知设备的接入（或移除），并且向USB协议栈的其他
部分登记集线器设备的接入。通常来说，总线驱动只是方便USB协议栈的不同部分
之间的通信。

## HCI driver (host only)

## 主机控制器接口（Host controller interface，HCI）驱动（只有USB主设备模式）

The host controller interface (HCI) driver exists at the bottom layer of the USB
stack when operating in host mode. This is the entity responsible for
translating outstanding `usb_request_t` into the necessary hardware directives
capable of servicing the request.

当运行在USB主设备模式时，主机控制器接口（Host controller interface，HCI）驱动
存在于USB协议栈的底层。它负责将尚未处理的“usb_request_t”翻译成必要的、能够
为请求提供服务的硬件指令。

The HCI driver is distinguished from the DCI driver in that it contains
functionality to facilitate device enumeration. If general enumeration is
separated into two phases:

- Bus enumeration (up through the `set_address` command).
- Device enumeration (everything to follow an addressable device).

HCI驱动与DCI驱动的区别在于它包含了便于设备枚举的功能。如果将一般的枚举
划分成两个阶段：

- 总线枚举（通过“set_address”命令来做到）。
- 设备枚举（所有设备都跟随一个能够寻址的设备）。

The HCI driver performs the former half while the USB stack takes over and
performs the rest of the device enumeration.

HCI驱动执行前半部分，然后USB协议栈对该枚举任务进行接管，并执行设备
枚举任务的其余部分。

## DCI driver (device only)

## DCI驱动（只有USB从属设备模式）

The device controller interface(DCI) driver exists at the bottom layer of the
USB stack when operating in device mode. This is the entity responsible for
translating outstanding `usb_request_t` into the necessary hardware directives
capable of servicing the request.

当运行在USB从属设备模式时，设备控制器接口（Device controller interface，DCI）
驱动存在于USB协议栈的底层。它负责将尚未处理的“usb_request_t”翻译成必要的、能够
为请求提供服务的硬件指令。

The DCI driver is distinguished from the HCI driver in that it serves to present
incoming OUT-type transfer requests to the device as well as set up outgoing
IN-type transfer requests to the bus. In both cases, an individual transfer may
result in multiple packets going each direction.

DCI驱动与HCI驱动的区别在于它用于向设备提供传入的OUT类型传输请求，
并且向总线设置传出的IN类型的传输请求。在这两种情况下，一个独立的传输可能都会
造成多个数据包在传入和传出方向上同时进行传输。

## See also

## 另见

+ [USB 2.0 spec]

+ [USB 2.0 规格]

<!-- xref -->

[USB 2.0 spec]: https://www.usb.org/document-library/usb-20-specification

[USB 2.0 规格]: https://www.usb.org/document-library/usb-20-specification
