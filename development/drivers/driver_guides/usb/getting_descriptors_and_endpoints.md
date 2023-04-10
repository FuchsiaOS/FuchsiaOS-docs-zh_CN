<!--
    (C) Copyright 2019 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

# Getting descriptors and endpoints from USB

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

The `usb` class contains several subclasses providing access to the interfaces, descriptors,
and endpoints of the usb device. The subclasses included are:

*   [`InterfaceList`](/src/devices/usb/lib/usb/include/usb/usb.h#311)
*   [`Interface`](/src/devices/usb/lib/usb/include/usb/usb.h#290)
*   [`DescriptorList`](/src/devices/usb/lib/usb/include/usb/usb.h#166)
*   [`EndpointList`](/src/devices/usb/lib/usb/include/usb/usb.h#266)

USB descriptor report all of the device's attributes. An endpoint is a specific type of descriptor
that describes the terminus of a communication flow between the host and the device.

The `InterfaceList` class iterates over each `Interface` within the usb device.
Each `Interface` then contains:

*   `DescriptorList` through `GetDescriptorList()`
*   `EndpointList` through `GetEndpointList()`

These methods allow access to all the descriptors and endpoints of the interface.

Note: Endpoints are still considered descriptors and therefore can also be
accessible through the `GetDescriptorList()` method.

The hierarchy of these subclasses can be seen in **Figure 1.**

![Diagram of USB class hierarchy](images/usbstructure.jpg)

**Figure 1**

## Examples

### Receiving Descriptors from USB

These examples iterate through all the descriptors in a USB device. The example iterates through all
of the USB Interfaces and then iterates through all the descriptors in each interface.

Note: To iterate through the endpoints instead, replace `interface.getDescriptorList()` with `interface.getEndpointList()`.

#### Range-based for loop

    std::optional<InterfaceList> interface_list;

    status = InterfaceList::Create(my_client, true, &interface_list);

    if (status != ZX_OK) {
        ...
    }

    for (auto& interface : *interface_list) {

        for (auto& descriptor : interface.GetDescriptorList()) {
            ...
        }
    }

#### Manual for loop

    std::optional<InterfaceList> interface_list;

    status = InterfaceList::Create(my_client, true, &interface_list);

    if (status != ZX_OK) {
        ...
    }

    for (auto& interface : *interface_list) {

        auto dList_itr = interface.GetDescriptorList().begin(); // or cbegin().

        do {
            ...
        } while (++dList_itr != interface.GetDescriptorList().end());
    }
