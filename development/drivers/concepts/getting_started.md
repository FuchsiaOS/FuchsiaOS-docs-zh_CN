

<!--
    (C) Copyright 2018 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

<!---

# Getting Started

Writing a device driver is often viewed as a daunting task, fraught with complexities
and requiring arcane knowledge of little-known kernel secrets.

The goal of this section is to demystify the process; you'll learn everything you
need to know about how to write device drivers, starting with what they do, how
they work, and how they fit into the overall system.

--->

## 入门指南

编写设备驱动通常是被认为是一件充满复杂性，艰巨的任务，并且需要了解神秘且鲜为人知的内核秘密。

本章的目的在于揭开这个过程的神秘面纱；你将会从它们的作用是什么，如何工作以及怎样融入整个系统开始，学习到关于如何编写设备驱动程序的一切知识。

<!---

List of documents to get started -

* [Fuchsia Driver Framework][fdf] - Overview of driver manager and driver host
* [Fuchsia drivers][fuchsia-drivers-overview] - Overview of fuchsia driver model.
* [Device and driver model][device-driver-model] - Documents that explain device and driver model
* [Driver development][driver-development] - Documents related to interrupts, DMA and other concepts
  for developing drivers
* [Driver architectures][driver-architectures] - Documents related to architecture of specfic driver
  types

The sections are listed above in default reading order, but it's perfectly fine to jump around and
read them in order of interest or applicability.

--->

下面为开始指南的相关文档

* [Fuchsia驱动框架][fdf] - 驱动管理器和驱动主机的概述
* [Fuchsia驱动][fuchsia-drivers-overview] - Fuchsia驱动模型概述
* [设备和驱动模型][device-driver-model] - 设备和驱动模型解释
* [驱动实现][driver-development] - 中断，DMA和其他开发驱动相关文档
* [驱动框架][driver-architectures] - 特定驱动类型架构相关文档

上述所列章节为默认阅读顺序，但也可以完全按照兴趣和使用的顺序跳读。

<!---

<!-- xrefs -->

[fdf]: /docs/concepts/drivers/fdf.md
[fuchsia-drivers-overview]: /docs/concepts/drivers/device_driver_model/introduction.md
[device-driver-model]: /docs/concepts/drivers/device_driver_model
[driver-development]: /docs/concepts/drivers/driver_development
[driver-architectures]: /docs/concepts/drivers/driver_architectures

--->

<!-- xrefs -->

[fdf]: /concepts/drivers/fdf.md
[fuchsia-drivers-overview]: /concepts/drivers/device_driver_model/introduction.md
[device-driver-model]: /concepts/drivers/device_driver_model
[driver-development]: /concepts/drivers/driver_development
[driver-architectures]: /concepts/drivers/driver_architectures



