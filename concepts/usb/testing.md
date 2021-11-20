<!--
    (C) Copyright 2019 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

<!--
# Testing a USB Device
-->

# 测试一个USB设备

<!--
Note: This is for the non-component version of USB Virtual
Bus. The component version is located at
[/src/lib/isolated_devmgr/usb-virtual-bus.h](/src/lib/isolated_devmgr/usb-virtual-bus.h).
There is currently no documentation about the component version.
-->

注意：这篇文档适用于非组件（non-component）版本的USB虚拟总线（USB Virtual Bus）。
组件版本位于[/src/lib/isolated_devmgr/usb-virtual-bus.h](/src/lib/isolated_devmgr/usb-virtual-bus.h)。
目前还没有对组建版本进行介绍的文档。

<!--
The [USB Virtual Bus](/src/devices/usb/drivers/usb-virtual-bus/) framework is a
helpful framework to connect a USB Function driver to a
USB device driver for testing.
-->

[USB虚拟总线](/src/devices/usb/drivers/usb-virtual-bus/)框架是一个实用框架，能够将
一个USB功能驱动（Function driver）连接至一个USB设备驱动，以对该USB设备驱动进行测试。

<!--
The following files are involved in testing a USB Device driver. All three of
these files should be stored in the same directory:

* `{driver}.cc`: the USB device driver that will be tested.
* `{driver}-function.cc`: A USB function driver that fakes the underlying
   USB device hardware.
* `{driver}-test.cc`: The test program, which sets up the driver and runs tests.
-->

在测试USB设备时，下列文件会被调用。这三个文件都应当被储存在同一个目录下：

* `{driver}.cc`: 待测试的USB设备驱动。
* `{driver}-function.cc`: 一个USB功能驱动，能够伪装成下层的USB设备硬件。
* `{driver}-test.cc`: 测试程序，能够设置好驱动并进行测试。

<!--
The usb-virtual-bus connects the USB Peripheral bus to the USB bus, as seen
in the below graphic:

```
usb-peripheral-bus -> {Your usb-function driver}
^
|
usb-virtual-bus
|
v
usb-bus -> {Your usb-driver}
```
-->

USB虚拟总线将USB外围总线连接至USB总线，如下图所示：

```
USB外围总线 -> {您的USB功能驱动}
^
|
USB虚拟总线
|
v
USB总线 -> {您的USB驱动}

<!--
## Write a USB-function driver {#write-usb-function-driver}
-->

## 编写一个USB功能驱动 {#write-usb-function-driver}

<!--
A usb-function driver makes the current host appear like a peripheral
USB device. For example, the USB mass storage (ums) function device allows
the host to appear as a block device when it is plugged into another machine.
If it makes sense for your host to have a USB function driver for your class
of device, then a real usb-function driver should be created. Otherwise,
writing a usb-function that fakes the hardware of a usb device is the easiest
way to test your usb driver.
-->

一个USB功能驱动让现在的主设备表现为一个外围设备。例如，当主设备
接入另外一台机器时，USB大容量存储（USB mass storage，ums）功能设备允许
主设备表现为一个块设备（block device）。对于您的设备的类，如果您觉得您的主设备
具有USB功能驱动是合理的，那么您应该创建一个真正的USB功能驱动。另外，要测试您的
USB驱动，编写一个伪装成USB设备硬件的USB功能是最容易的方式。

<!--
The usb-virtual-bus connects your usb-function driver to the actual USB device
driver you are trying to test. This allows the device driver to be run in a test
mode with no modifications to the device driver.
-->

USB虚拟总线将您的USB功能驱动连接至实际的您将要测试的USB设备驱动。这允许设备
驱动在测试模式下运行，而不需要对设备驱动进行改动。

<!--
Examples of usb-function drivers:

* [one-endpoint-hid-function driver](/src/ui/input/drivers/usb-hid/function/one-endpoint-hid-function.cc)
* [two-endpoint-hid-function driver](/src/ui/input/drivers/usb-hid/function/two-endpoint-hid-function.cc)
* [ftdi-function driver](/src/devices/serial/drivers/ftdi/ftdi-function.cc)
-->

USB功能驱动的例子：

* [单端HID功能驱动](/src/ui/input/drivers/usb-hid/function/one-endpoint-hid-function.cc)
* [双端HID功能驱动](/src/ui/input/drivers/usb-hid/function/two-endpoint-hid-function.cc)
* [ftdi功能驱动](/src/devices/serial/drivers/ftdi/ftdi-function.cc)

<!--
The usb-function driver needs to implement the
[UsbFunctionInterface](/sdk/banjo/fuchsia.hardware.usb.function/usb-function.fidl#49)
banjo interface. These are the functions that are called from the
usb-virtual-bus library as it sets up the driver in the USB stack.
-->

USB功能驱动需要实现[UsbFunctionInterface](/sdk/banjo/fuchsia.hardware.usb.function/usb-function.fidl#49)
banjo接口（interface）。当USB虚拟总线库设置USB栈中的驱动时，这些函数就被USB
虚拟总线库调用。

<!--
A usb-function driver binds on top of the
[UsbFunction](/sdk/banjo/fuchsia.hardware.usb.function/usb-function.fidl#12)
protocol.  These are the calls that allow the function driver to allocate
endpoints, register interface callbacks, queue USB requests, and more.
-->

USB功能驱动与[UsbFunction](/sdk/banjo/fuchsia.hardware.usb.function/usb-function.fidl#12)
协议的顶层绑定。这些调用允许功能驱动分配断点（endpoint），登记接口回调（interface callback），
维护USB请求队列，等等。

<!--
### Bind rules
-->

### 绑定规则

<!--
The usb-function driver needs to bind to the `ZX_PROTOCOL_USB_FUNCTION`
protocol. There can be additional bind rules for the USB class, USB subclass,
and USB protocol.
-->

USB功能驱动需要与 `ZX_PROTOCOL_USB_FUNCTION` 协议绑定。对于USB类、
USB子类和USB协议，有额外的绑定规则。

<!--
This example shows a bind rule where `{}` represents an area that should be
replaced with your information:


```
using fuchsia.usb;

fuchsia.BIND_PROTOCOL == fuchsia.usb.BIND_PROTOCOL.FUNCTION;
fuchsia.BIND_USB_CLASS == {usb_class}
fuchsia.BIND_USB_SUBCLASS == {usb_subclass}
fuchsia.BIND_USB_PROTOCOL == {usb_protocol}
```
-->

这个例子展示了一个绑定规则，`{}`表示了应当替换成您自己的信息的地方：


```
using fuchsia.usb;

fuchsia.BIND_PROTOCOL == fuchsia.usb.BIND_PROTOCOL.FUNCTION;
fuchsia.BIND_USB_CLASS == {USB类}
fuchsia.BIND_USB_SUBCLASS == {USB子类}
fuchsia.BIND_USB_PROTOCOL == {USB协议}
```

<!--
## Writing the usb-virtual-bus test
-->

## 编写USB虚拟总线测试程序

<!--
The test should be written using the
[usb virtual bus launcher library](/zircon/system/ulib/usb-virtual-bus-launcher).

The first thing the test launches is the usb-function driver described in
[Write a USB-function driver](#write-usb-function-driver). You can launch this
test by adding the bind rules to a `usb_peripheral::FunctionDescriptor` and
using the `SetupPeripheralDevice()` function. For example:

```c++
// Set up your USB Device Descriptor.
usb_peripheral::DeviceDescriptor device_desc = {};

/ Set up your USB Function descriptors.
std::vector<usb_peripheral::FunctionDescriptor> function_descs;
usb_peripheral::FunctionDescriptor function_desc = {
    .interface_class = {usb_class},
    .interface_subclass = {usb_subclass},
    .interface_protocol = {usb_protocl},
};
function_descs.push_back(function_desc);

ASSERT_NO_FATAL_FAILURES(SetupPeripheralDevice(device_desc, std::move(function_descs)));
```
-->

测试程序应当使用[USB虚拟总线启动器库](/zircon/system/ulib/usb-virtual-bus-launcher)
来编写。

测试程序首先启动的是在[编写一个USB功能驱动](#write-usb-function-driver)
中描述的USB功能驱动。要启动测试程序，您可以通过添加绑定规则至一个
`usb_peripheral::FunctionDescriptor`或者使用`SetupPeripheralDevice()`函数。例如：

```c++
// Set up your USB Device Descriptor.
usb_peripheral::DeviceDescriptor device_desc = {};

/ Set up your USB Function descriptors.
std::vector<usb_peripheral::FunctionDescriptor> function_descs;
usb_peripheral::FunctionDescriptor function_desc = {
    .interface_class = {usb_class},
    .interface_subclass = {usb_subclass},
    .interface_protocol = {usb_protocl},
};
function_descs.push_back(function_desc);

ASSERT_NO_FATAL_FAILURES(SetupPeripheralDevice(device_desc, std::move(function_descs)));
```

<!--
Once the `SetupPeripheralDevice` function has succeeded, the usb-function driver
binds.

The USB virtual bus connects the function driver into the system, and then the
real device driver binds. Your test can then connect to the USB device driver
through `devfs`. Binding happens asynchronously, so you have to wait for the
driver to be detected by `devfs`. The east way to watch for a file is
the `fdio_watch_directory` function.

Now that you've connected to your device, FIDL calls can be made normally.
Your driver handles those FIDL calls and makes calls to your USB function driver
as if it were real hardware.
-->

一旦 `SetupPeripheralDevice` 函数完成，USB功能驱动就绑定了。

USB虚拟总线将功能驱动连接至系统，然后真正的设备驱动绑定。您的测试程序接下来
能够通过 `devfs` 连接至USB设备驱动。绑定是异步进行的，所以您需要等待驱动被 `devfs`
检测到。要查看文件，最简单的方式是使用 `fdio_watch_directory` 函数。

既然您已经连接到了您的设备，就能够正常地进行FIDL调用。您的驱动处理这些FIDL调用，
并且，如果该设备是真正的硬件，您的驱动将会对USB功能驱动进行调用。
