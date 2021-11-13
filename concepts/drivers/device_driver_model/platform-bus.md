<!---

# Platform Bus

## Introduction

The term **platform bus** refers to a specific Fuchsia driver with source code located at
[//fuchsia/src/devices/bus/drivers/platform/](/src/devices/bus/drivers/platform/).
However this term also refers to the framework that manages the lower level drivers in Fuchsia.
In this document, **platform bus driver** refers to a specific driver and **platform bus**
refers to the general framework.

--->

# 平台总线

## 简介

**平台总线**指代码在[//fuchsia/src/devices/bus/drivers/platform/](/src/devices/bus/drivers/platform/)中的Fuchsia驱动。
尽管这个术语同样指在Fuchsia中管理底层驱动的框架。但是在本文档中，**平台总线驱动**指特定驱动，**平台总线**指代通用框架。

<!---

The platform bus as a whole contains several types of drivers:

- The **platform bus driver**, which manages the platform bus. This is a generic driver
with no hardware specific functionality. The platform bus driver is started automatically
by the driver manager when the system boots.
- The **board driver**, which is the first driver loaded by the platform bus driver.
The board driver contains all the platform specific information needed by the platform bus
and controls what other drivers will be loaded by the platform bus.
On arm64 platforms, the platform bus driver uses information from the bootloader or boot shim
to bind the correct board driver for the platform it is running on.
On x86 platforms, the platform bus driver always loads the x86 board driver and creates platform
devices based on information from ACPI.
- The **platform device drivers** are the foundations for the higher level drivers in Fuchsia.
These drivers provide the lowest level of support for a particular feature, like USB,
eMMC or NAND storage, etc., with higher level drivers loading on top of that.
- The **protocol implementation drivers** are drivers that provide protocols that are needed
by the board driver. One common example of this is the GPIO driver, which is often needed by the
board driver for pin-muxing. In the past, the platform bus used to also proxy these drivers'
protocols to platform devices, but now we use composite devices instead.
Over time, we will likely phase out the use of protocol implementation drivers in the platform bus
and replace it with a new approach that does not require blocking to wait for drivers to load.
- Finally, the **platform proxy driver** a companion to the platform bus driver that loads
  into the platform device driver hosts. This driver supports proxying the platform device protocol
  and other resource protocols from the platform device driver to the platform bus driver and
  protocol implementation drivers. This is needed because the platform device drivers run in a
  different driver host process than the platform bus driver and the protocol implementation drivers.

--->

平台总线总体包含以下几种类型的驱动：
- **平台总线驱动**，用来管理平台总线。这是一种没有特定硬件功能的通用驱动。平台总线驱动通过系统启动时驱动管理来完成自动启动。
- **板卡驱动**，是平台总线驱动加载的第一个驱动。板卡驱动包含所有平台总线需要的平台特定信息和其他平台总线加载的驱动控制。
  在 arm64 平台上，平台总线驱动使用 bootloader 或 boot shim 中的信息来为所运行的平台绑定正确的板卡驱动。
  在 x86 平台上，平台总线驱动总是加载 x86 板卡驱动并且基于 ACPI 中的信息创建平台设备。
- **平台设备驱动**是 Fuchsia 中高层驱动的基础。这些驱动提供了特定功能的最底层支持，例如 USB ， eMMC 或者 NAND 存储等，用于高层驱动加载。
- **协议实现驱动**是提供板卡驱动所需协议的驱动。一个通用示例为 GPIO 驱动，板卡驱动经常用于引脚混用功能。在过去平台总线曾经也将这些驱动的协议代理给平台设备，但是现在我们使用复合设备来代替。
  久而久之，我们可能逐步淘汰平台总线中的协议实现驱动使用，并用一种新的方法来取代它，这种方法不需要阻塞来等待驱动加载。
- 最后，**平台代理驱动**是一个平台总线驱动的配套设备，可载入平台设备驱动主机。这个驱动支持从平台设备驱动到平台总线驱动和协议实现驱动中的代理平台设备协议和其他资源协议，之所以这样做，是因为平台设备驱动运行在与平台总线驱动程序和协议实现不同的驱动主机进程中。

![Fuchsia Platform Bus diagram](images/platform-bus.png)
Source: [https://goto.google.com/zircon-platform-bus-diagram](https://goto.google.com/zircon-platform-bus-diagram)

<!---

## Platform Bus Initialization

The platform bus driver is started automatically by the driver manager at boot.
Since the platform bus driver is a generic driver that contains no information about the
platform it is running on, it first loads the board driver, which handles platform specific logic.
To determine which board driver to load, platform bus driver reads the `ZBI_TYPE_PLATFORM_ID`
record from the [ZBI data](/zircon/system/public/zircon/boot/image.h) passed from the
bootloader. It then adds a device with protocol `ZX_PROTOCOL_PBUS` with the
`BIND_PLATFORM_DEV_VID` and `BIND_PLATFORM_DEV_PID` binding variables set to the vid and did
from the platform data record. The correct board driver will bind to this device and continue
the platform bus initialization process. On x86 platforms, the x86 board driver is loaded
automatically.

The board driver uses the platform bus protocol to communicate with the platform bus driver.
After it does its own initialization, the board driver then uses the `ProtocolDeviceAdd()`
call in the platform bus protocol to load protocol implementation drivers.
After the protocol implementation driver loads, it must register its protocol with the platform bus
driver by calling the platform bus `RegisterProtocol()` API.
`ProtocolDeviceAdd()` will block until the driver calls `RegisterProtocol()`, so the board driver
must call `RegisterProtocol()` from one of its own threads rather than a driver manager callback like
`Bind()`.

After the protocol devices are added, the board driver will call the `DeviceAdd()` call
in the platform bus protocol to create platform devices, which will result in
platform device drivers loading each in its own driver host.
After the platform devices are created, the platform bus initialization is complete.

--->

## 平台总线初始化

平台总线驱动在启动时由驱动管理自动启动。因为平台总线驱动是一个不包含平台运行信息的通用驱动，它首先加载处理平台特殊逻辑的板卡驱动。为了决定哪个驱动被加载，平台总线驱动读取从 bootloader 传递的  [ZBI data](/zircon/system/public/zircon/boot/image.h)中的`ZBI_TYPE_PLATFORM_ID`。然后使用协议 `ZX_PROTOCOL_PBUS` 中 `BIND_PLATFORM_DEV_VID` 和 `BIND_PLATFORM_DEV_PID `绑定从平台数据记录中设置变量 vid 和 did 来添加设备。正确的板卡驱动将绑定该设备然后继续平台总线初始化进程。在 x86 平台上， x86 板卡驱动自动加载。

板卡驱动使用平台总线协议来和平台总线驱动通信。当它完成自身初始化后，板卡驱动使用 `ProtocolDeviceAdd()`调用到平台总线协议加载协议实现驱动。在协议实现驱动加载后，它必须通过调用平台总线 `RegisterProtocol()` API 注册协议到平台总线驱动中。
 `RegisterProtocol()`将阻塞直到驱动调用`RegisterProtocol()`，所以板卡驱动必须从它自己的线程中调用`RegisterProtocol()`，而不是从例如`Bind()`的回调中。

当协议设备被添加后，板卡驱动将在平台总线协议中调用`DeviceAdd()` 来创建平台设备，这将导致平台设备驱动依次加载到它自己的驱动主机中。
平台设备创建后，平台总线初始化就算完成了。

<!---

## Composite Platform Devices

The platform bus also supports adding platform devices to be used as components in composite
devices. The platform bus `CompositeDeviceAdd()` call adds a composite device, with the zeroth
component being a platform device described by the provided `PBusDev` struct.
The binding rules for the remaining components are provided by the `components` parameter.
The `coresident_device_index` is used to specify which driver host the composite device
should be created in. A value of `UINT32_MAX` will result in a new driver host being created for the
composite device, while a value of 1 through n will add the composite device to the driver host of one
of the other components. Passing 0 is not allowed, since we do not want the composite device
to be added to the platform bus driver's driver host.

The internals of composite platform devices are a bit different than the non-composite case.
Instead of using the platform proxy driver, the driver manager **component** and **component proxy** drivers
proxy the platform device protocol instead. For example, in the diagram above we have a composite device
for an audio driver with a platform device as its first component and an I2C channel as its second.
The audio driver is started in a new driver host, and the driver manager component and component proxy drivers
are responsible for proxying the PDEV and I2C protocols to the audio driver.

--->

## 复合平台设备

平台总线同样支持添加平台设备作为一个复合设备中的组件来使用。平台总线`CompositeDeviceAdd()` 接口为添加一个复合设备，它的第零位组件是一个提供的 `PBusDev` 结构体描述的平台设备。对于剩余组件的绑定规则由`components`参数来提供。`coresident_device_index`被用于明确复合设备应当被创建在哪一个驱动主机中。`UINT32_MAX` 的值将导致为复合设备创建一个新的驱动主机，当值从1到n时，将添加复合设备到其他组件的驱动主机中。传递0是不允许的，因为我们不希望复合设备被添加到平台总线驱动主机中。

复合平台设备内部是结构与非复合情况有些不同。不使用平台代理驱动，而使用驱动管理**组件**和**组件代理**驱动代替使用平台代理驱动。例如在上述框架图中我们有一个音频驱动的复合设备，以平台设备作为它第一个组件， I2C 通道作为其第二个组件的复合设备。
在一个新的驱动主机中启动音频驱动，驱动管理器组件和组件代理驱动负责代理音频驱动的 PDEV 和 I2C 协议。

<!---

## Platform Device Protocol

The [platform device protocol](/sdk/banjo/fuchsia.hardware.platform.device/platform-device.fidl)
(`ZX_PROTOCOL_PDEV`) is the main protocol provided by the platform bus to
platform device drivers. This protocol provides access to resources like MMIO ranges, interrupts,
BTIs, and SMC ranges to the platform device driver. Rather than requesting MMIOs and interrupts by
physical addresses or IRQ numbers, these resource are requested by a zero-based index.
This allows us to have platform device drivers for particular IP that works across multiple
platforms, since the knowledge of the exact MMIO addresses and interrupt numbers do not need to be
known by the driver. Instead, the board driver configures the MMIO addresses and IRQ numbers in the
`PbusDev` struct passed with `AddDevice()`.

The platform device protocol is also available to protocol implementation drivers.
For example, a GPIO driver may use the platform device protocol to access its MMIO and interrupts.
This allows protocol implementation drivers to be shared among different SOC variants,
where the functionality may be identical but the MMIO addresses and interrupt numbers may be
different.

--->

## 平台设备协议

[平台设备协议](/sdk/banjo/fuchsia.hardware.platform.device/platform-device.fidl)（`ZX_PROTOCOL_PDEV`）是平台总线对平台设备驱动提供的主要协议。这个协议提供对资源例如平台设备驱动 MMIO ，中断，BTIs 和 SMC 的访问。这些资源通过基于0的指针请求，而不是通过物理地址或者 IRQ 数请求 MMIOs 和中断。这使我们能够为特定的 IP 提供多种平台的平台设备驱动，因为确切的 MMIO 地址和中断号不需要由驱动程序知道。取而代之的是，板卡驱动使用接口`AddDevice()`传递`PbusDev`结构体来配置 MMIO 地址和 IRQ 数。

<!---

## Platform Bus Protocol

The [platform bus protocol](/sdk/banjo/fuchsia.hardware.platform.bus/platform-bus.fidl)
(`ZX_PROTOCOL_PBUS`) is used by board drivers and protocol implementation drivers
to communicate with the platform bus driver. It is only available to drivers running in the
platform bus's driver host (in particular, it is not accessible to platform device drivers).
The purpose of this protocol is for the board driver to load protocol implementation drivers
and to start platform device drivers. It is also used by protocol implementation drivers to
register their protocols with the platform bus so their protocols can be made available
to platform device drivers.

--->

## 平台总线协议

[平台总线协议](/sdk/banjo/fuchsia.hardware.platform.bus/platform-bus.fidl)（`ZX_PROTOCOL_PBUS`）用于板卡驱动和协议实现驱动来和平台总线驱动通信。仅对运行在平台总线驱动主机上的驱动有效（特别的是它不能被平台设备驱动访问）。这个协议的目的在于对板卡驱动来加载协议实现驱动和启动平台设备驱动。它同样也用于协议实现驱动来使用平台总线注册它们的协议，让平台设备驱动可使用。

