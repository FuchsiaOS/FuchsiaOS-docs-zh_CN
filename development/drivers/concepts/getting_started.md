

<!--
    (C) Copyright 2018 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

# Getting Started

Caution: The pages in this section may contain information that is specific to the
legacy version of the driver framework (DFv1).

Writing a device driver is often viewed as a daunting task, fraught with complexities
and requiring arcane knowledge of little-known kernel secrets.

The goal of this section is to demystify the process; you'll learn everything you
need to know about how to write device drivers, starting with what they do, how
they work, and how they fit into the overall system.

List of documents to get started -

* [Fuchsia Driver Framework][fdf] - Overview of driver manager and driver host
* [Device and driver model][device-driver-model] - Documents that explain device and driver model
* [Driver development][driver-development] - Documents related to interrupts, DMA and other concepts
for developing drivers
* [Drivers Rubric][driver-rubric] - Rules for writing new drivers
* [Driver architectures][driver-architectures] - Documents related to architecture of specific
driver types

The sections are listed above in default reading order, but it's perfectly fine to jump around and
read them in order of interest or applicability.

<!-- Reference links -->

[fdf]: /development/drivers/concepts/fdf.md
[driver-rubric]: /development/drivers/developer_guide/rubric.md
[device-driver-model]: /development/drivers/concepts/device_driver_model/README.md
[driver-development]: /development/drivers/concepts/driver_development/README.md
[driver-architectures]: /development/drivers/concepts/driver_architectures/README.md
