<!--
    (C) Copyright 2020 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

<!---

# Fuchsia input drivers

Fuchsia's input drivers implement the
[fuchsia.input.report](/sdk/fidl/fuchsia.input.report) FIDL API. Input drivers
cover a range of input devices like mice, keyboards, touchscreens, consumer
controls (media buttons), and sensors. Input drivers cover a range of protocols including USB,
I2C, and Bluetooth.

--->

# Fuchsia 输入驱动

Fuchsia 的输入驱动实现[fuchsia.input.report](/sdk/fidl/fuchsia.input.report) 的 FIDL API。输入驱动包含一系列输入设备，就像鼠标， 键盘，触摸屏，消费者控制器（多媒体按键）和传感器。输入驱动覆盖一系列协议包括 USB， I2C 和蓝牙。

<!---

## Overview

The
[`fuchsia.input.report`](https://fuchsia.dev/reference/fidl/fuchsia.input.report)
FIDL API represents the lowest level of structured input in a Fuchsia system.
This API should try to represent the underlying input hardware as close as
possible. No additional state should be added to this API that doesn't exist in
hardware. FIDL tables are used to ensure that future input devices and future
features can be added while maintaining compatibility. The
`fuchsia.input.report` API is based on the Human Interface Device (HID) Protocol
which is a hardware protocol for Input devices. Basing this API on the HID
Protocol helps ensure that the API will support most HID devices.

This API is consumed by the next layer of the input stack, which
provides IME support, access control, and statefulness. This API is also
consumed by trusted system programs that are unable or unwilling to be a part of
the input stack.

--->

## 概述

[`fuchsia.input.report`](https://fuchsia.dev/reference/fidl/fuchsia.input.report) FIDL API 代表了 Fuchsia 系统的输入架构的最低等级。这个 API 应该尽可能地代表底层的输入硬件。 API 中不应添加硬件中不存在的额外状态。 FIDL 列表被用于确保将来的输入设备，和维护兼容性时追加的新特性。`fuchsia.input.report`  API 是基于人机接口设备（ HID ）协议，这是一个对于输入设备的硬件协议。基于这个 API 使用 HID 协议可以帮助确保 API 将支持大多数 HID 设备。

这个 API 是被输入栈的下一层使用，用于提供 IME 支持，访问控制和状态化。这个 API 也被那些不能或不愿意成为输入栈部分的可信任的系统程序使用。

<!---

## Background

The Human Interface Device or
[HID standard](https://www.usb.org/sites/default/files/documents/hid1_11.pdf) is a
hardware protocol for Input devices that is standardized across USB, I2C, and
bluetooth. It supports a wide variety of input devices such as Mice, Keyboards,
Touchscreens, Styluses, Sensors, LEDs, Gamepads, and more. At its core, the
HID standard provides a way to assign meaning to binary reports being sent
from the input device to the host computer.

The devices that HID can describe are infinitely flexible, but this makes the
API difficult to consume. 95% of HID devices fit cleanly into a well defined
group of Input Devices and can easily be described in generic terms. An API
layer like `fuchsia.input.report` is necessary in order to abstract away from the
HID protocol and allow clients to easily use “typical” devices.

--->

## 背景

人机界面设备或者[HID standard](https://www.usb.org/sites/default/files/documents/hid1_11.pdf) 是一个输入设备的硬件协议，它标准化了 USB， I2C 和蓝牙协议。支持各种各样的输入设备例如鼠标，键盘，触摸屏，探针，传感器，LEDs，手柄和更多设备。其核心是 HID 标准提供了一种方式来为从输入设备发送到主机的二进制报告分配意义。

HID 可以描述的设备是极其灵活的，但是这会对 API 使用变得困难。95 % 的 HID 设备完全适合于一个定义明确的输入设备组，这样可以用通用术语很容易的来描述。 为了抽象 HID 协议和允许客户端能简单的使用“经典”设备，形如`fuchsia.input.report` 的API 层就是有必要的。

<!---

## Design Decisions

### Clients and Typical Usage

--->

## 设计决策

### 客户端和典型用法





![Figure: General Usage of fuchsia.input.report in the system](images/input-pipeline.png)

<!---

`fuchsia.input.report` is meant to be the lowest level of the input stack. This
API should be consumed by the next stage of the input stack, and a modified
version of these reports should be passed to higher-level clients. The clients
for these Reports should be programs that are responsible for the rest of the
input stack (e.g: `RootPresenter`, `SessionFramework`).

These reports are not suitable for direct consumption by generic apps in the
system. Generic apps should use the full input stack, which provides
InputEvents that have full context and have valid access control.

Some lower-level system programs want to access `fuchsia.input.report` in
order to read sensors and other simplified input. These are programs that
are trusted by the system and that require access to input at this level. The
system terminal Virtcon requires low level access to the keyboard. The
Screen-Brightness program requires access to the brightness sensor. These are
the types of programs that should be using `fuchsia.input.report` directly. The
[`print-input-report`](/src/ui/tools/print-input-report) binary can be used as
an example program for using low level input.

--->

`fuchsia.input.report`意味着输入栈中的最底层。这个 API 将被下一个阶段的输入栈使用，这些上报的修改版本也应当被传递给更高级的客户端。针对这些上报，客户端应当负责其剩余的输入栈（例如： `RootPresenter`, `SessionFramework`）。

这些上报并不适用于在系统中的通用应用直接使用。一般的应用应当使用提供拥有完整上下文和有可访问权限控制的 InputEvents 这样完整输入栈。

一些系统底层程序想要访问`fuchsia.input.report`，来读取传感器和其他简单的输入。这些被系统信任的程序想要在这一层获取访问输入的权限。系统终端 Virtcon 需要底层访问键盘。屏幕亮度程序想要访问亮度传感器。类似这样的程序可以直接使用`fuchsia.input.report` 。[`print-input-report`](/src/ui/tools/print-input-report) 库则是一个使用底层输入的示例程序。

<!---

### Stateless Reports

`fuchsia.input.report` should be as faithful as possible to the
hardware reports that are sent by the physical device. This means that the
`fuchsia.input.report` API should not add additional state that is not present in
the hardware device. This means that some reports are missing context and state
from the rest of the system. For example, the mouse reports give relative X and
Y movements, but do not keep track of global position on a screen. The touch
reports give finger id’s but do give “Hover”, “Press”, “Release” events
directly. Keyboard reports have no concept of keyboard layout. This information
will need to be filled in by the higher levels of the input stack.

--->

### 无状态上报

`fuchsia.input.report` 应当对物理设备上报的硬件状态无条件信任。这意味着`fuchsia.input.report` 的 API 不应该添加硬件设备中没有的额外状态。这也就是说某些上报缺少上下文和从系统其他部分的状态。例如鼠标上报给定相关 X 和 Y 的移动信息，但是并不追踪其在屏幕上的全局位置。触摸板上报给定指纹 id ，但是不会直接上报 “ 悬停”，”按压“，”释放“事件。键盘上报信息对键盘布局也没有概念。这个信息将由更高层的输入栈来填入。

<!---

### Fuchsia-specific Enums

The InputReport API will explicitly not use HID enums for descriptions (e.g: LED
enums, Keyboard Key enums). It will use Fuchsia specific enums. This grants
the API an extra layer of flexibility for declaring additional uses and removing
usages that we choose not to support. (One example is that HID does not have a
Factory Data Reset (FDR) button enum, which we need to support).

It is easier to be consistent by explicitly not using HID enums than to be
inconsistent and use HID enums for some cases but add additional values in
others.

--->

### Fuchsia 特有枚举

InputReport API 明确不使用 HID 枚举进行叙述（例如 LED 枚举，键盘键值枚举）。它将使用 Fuchsia 特有的枚举。这样给予了 API 对于定义额外的使用和移除我们不想支持的功能更好的灵活性。（一个示例就是 HID 是没有工厂数据复位（ FDR ）键枚举，但是我们又需要支持该功能）。

明确不使用 HID 枚举来保持一致，要比在某些情况下使用 HID 枚举但是需要追加额外的值来保持一致要容易的多。

