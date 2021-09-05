<!---
# Fuchsia Driver Framework
--->
# Fuchsia 驱动框架
<!---
## Overview
Fuchsia Driver Framework (FDF) is a collection of libraries, tools, metadata and images that enables
driver writers to develop, test and distribute drivers targeting Fuchsia. This is aimed to provide a
stable ABI allowing driver developers to potentially write a driver once and use it on multiple
versions of the Fuchsia kernel and platform.

At the moment, the FDF is composed of a driver manager, driver host, core library (libdriver),
FIDL interfaces, banjo interfaces and guidelines to develop drivers for Fuchsia.
FDF is constantly evolving and yet to achieve ABI stability.
--->

## 概述

Fuchsia驱动框架（FDF）是一整套库，工具，元数据和映像文件的集合，可以帮助驱动开发者进行开发，测试和发布针对Fuchsia的驱动。FDF想要提供一个稳定的ABI去允许驱动开发者仅需开发一次驱动后，就可以应用在Fuchsia内核和平台的多个版本上。

同时，对于Fuchsia驱动开发者而言，FDF包含一个驱动管理器，驱动主机，核心库（libdriver)，FIDL接口，banjo接口和参考。

FDF正在不断发展，尚未达到ABI的稳定性。

<!---

## Driver manager

Driver manager is a binary maintained and developed as part of FDF. It is responsible to
load drivers and manage devices on all platforms. This is one of the initial process to be started
on device bootup. It finds driver packages in pre-configured paths, tries to match a
driver for every device by running the driver's bind program, and manages the device lifecycle.
It hosts a virtual filesystem named as Device Filesystem (`devfs`), that provides
uniform access to all devices from userspace services/components external to the drivers. `devfs`
is mounted under `/dev` and contains virtual files that eventually route to interfaces
implemented by the devices.

--->

## 驱动管理器

驱动管理器是作为FDF的一部分用于维护和开发二进制文件。它负责在所有平台上加载驱动和管理设备。这是设备加载时要启动的初始程序之一。它在预先配置的路径下寻找到驱动包后，通过运行驱动绑定程序，试图通过一个驱动匹配所有的设备，然后管理设备生命周期。它管理着一个名为设备文件系统（`devfs`）的虚拟文件系统，为来自驱动外部的用户空间服务/组件提供对所有设备的统一访问接口。 `devfs`被挂载在 `/dev` 下，其中包含由设备实现的最终路由接口的虚拟文件。

<!---

## Driver host

Driver host is a binary that is launched by driver manager to host one or more drivers. It
facilitates sandboxing of drivers.

--->

## 驱动主机

驱动主机是一个由驱动管理器启动的二进制文件，用于管理一个或多个驱动程序。这样有助于对驱动程序进行沙箱管理。
