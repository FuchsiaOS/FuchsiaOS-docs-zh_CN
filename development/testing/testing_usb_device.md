<!--
    (C) Copyright 2019 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

# Testing a USB Device

The [USB Virtual Bus](/src/devices/usb/drivers/usb-virtual-bus/) framework is a
helpful framework to connect a USB Function driver to a
USB device driver for testing.

The following files are involved in testing a USB Device driver. All three of
these files should be stored in the same directory:

* `{driver}.cc`: the USB device driver that will be tested.
* `{driver}-function.cc`: A USB function driver that fakes the underlying
   USB device hardware.
* `{driver}-test.cc`: The test program, which sets up the driver and runs tests.

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

## Write a USB-function driver {#write-usb-function-driver}

A usb-function driver makes the current host appear like a peripheral
USB device. For example, the USB mass storage (ums) function device allows
the host to appear as a block device when it is plugged into another machine.
If it makes sense for your host to have a USB function driver for your class
of device, then a real usb-function driver should be created. Otherwise,
writing a usb-function that fakes the hardware of a usb device is the easiest
way to test your usb driver.

The usb-virtual-bus connects your usb-function driver to the actual USB device
driver you are trying to test. This allows the device driver to be run in a test
mode with no modifications to the device driver.

Examples of usb-function drivers:

* [one-endpoint-hid-function driver](/src/ui/input/drivers/usb-hid/function/one-endpoint-hid-function.cc)
* [two-endpoint-hid-function driver](/src/ui/input/drivers/usb-hid/function/two-endpoint-hid-function.cc)
* [ftdi-function driver](/src/devices/serial/drivers/ftdi/ftdi-function.cc)

The usb-function driver needs to implement the
[UsbFunctionInterface](/sdk/banjo/fuchsia.hardware.usb.function/usb-function.fidl#49)
banjo interface. These are the functions that are called from the
usb-virtual-bus library as it sets up the driver in the USB stack.

A usb-function driver binds on top of the
[UsbFunction](/sdk/banjo/fuchsia.hardware.usb.function/usb-function.fidl#12)
protocol.  These are the calls that allow the function driver to allocate
endpoints, register interface callbacks, queue USB requests, and more.

### Bind rules

The usb-function driver needs to bind to the `ZX_PROTOCOL_USB_FUNCTION`
protocol. There can be additional bind rules for the USB class, USB subclass,
and USB protocol.

This example shows a bind rule where `{}` represents an area that should be
replaced with your information:


```
using fuchsia.usb;

fuchsia.BIND_PROTOCOL == fuchsia.usb.BIND_PROTOCOL.FUNCTION;
fuchsia.BIND_USB_CLASS == {usb_class}
fuchsia.BIND_USB_SUBCLASS == {usb_subclass}
fuchsia.BIND_USB_PROTOCOL == {usb_protocol}
```

## Writing the usb-virtual-bus test

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
    .interface_protocol = {usb_protocol},
};
function_descs.push_back(function_desc);

ASSERT_NO_FATAL_FAILURE(SetupPeripheralDevice(device_desc, std::move(function_descs)));
```

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
