<!---

# SDMMC drivers architecture

The SDMMC driver stack is divided into two main components: platform drivers
that talk directly to controller hardware, and the core driver that handles
protocol-specific device initialization and communication. The core driver is
further divided into an SDIO driver and a block driver (for SD and eMMC). Each
SDMMC controller has a different platform driver, while the core driver is
used on all platforms.

--->

# SDMMC 驱动架构

 SDMMC 驱动栈分为两个主要的模块：平台驱动用于直接和控制硬件对话，而核心驱动用于处理特定协议设备的初始化和通信。核心驱动又可以进一步分为一个 SDIO 驱动和一个块驱动（对于 SD 和 eMMC）。每一个 SDMMC 控制器都有一个不同的平台驱动，而核心驱动则能在所有平台上使用。

<!---

## Bringup

Bringing up an SoC with a new SDMMC controller requires writing a new platform
driver. If the controller implements the SDHCI specification then this driver
should implement
[fuchsia.hardware.sdhci](/sdk/banjo/fuchsia.hardware.sdhci/sdhci.fidl), otherwise it
should implement
[fuchsia.hardware.sdmmc](/sdk/banjo/fuchsia.hardware.sdmmc/sdmmc.fidl). It may be
helpful to disable DMA and higher speed modes through `SdmmcHostInfo` and
`SdmmcHostPrefs` until the basic functionality of the hardware has been
validated. See the SDHCI and SDMMC protocol definitions for more information.

--->

## 启动

启动一个有新 SDMMC 控制器的 SoC 需要写入新的平台驱动。如果控制器实现了 SDHCI 规范，那么这个驱动也应该实现[fuchsia.hardware.sdhci](/sdk/banjo/fuchsia.hardware.sdhci/sdhci.fidl)，否则就要实现[fuchsia.hardware.sdmmc](/sdk/banjo/fuchsia.hardware.sdmmc/sdmmc.fidl)。这对禁用 DMA 和`SdmmcHostInfo` 和`SdmmcHostPrefs`提高速度模式直到硬件的基础功能验证，都是非常有用的。参考 SDHCI 和 SDMMC 协议定义获取更多信息。

<!---

## SD/eMMC core driver

The SD/eMMC block driver creates a device that implements
[fuchsia.hardware.block.BlockImpl](/sdk/banjo/fuchsia.hardware.block/block.fidl) and
[fuchsia.hardware.block.partition](/sdk/banjo/fuchsia.hardware.block.partition/partition.fidl)
for the user data partition, as well as devices for the boot0 and boot1
partitions if enabled (eMMC only). A device implementing
[fuchsia.hardware.rpmb](/sdk/banjo/fuchsia.hardware.rpmb/rpmb.fidl) is created if the
device supports it (eMMC only, based on JEDEC standard JESD84-B51 section 6.6.22).

--->

## SD/eMMC 核心驱动

 SD/eMMC 块驱动对于用户数据分区创建一个设备实现[fuchsia.hardware.block.BlockImpl](/sdk/banjo/fuchsia.hardware.block/block.fidl)和[fuchsia.hardware.block.partition](/sdk/banjo/fuchsia.hardware.block.partition/partition.fidl)，如果使能的话（仅限 eMMC ），则同样创建一个设备有 boot0 和 boot1 分区。如果设备支持的话（仅限 eMMC , 基于 JEDEC 标准 JESD84-B51的章节 6.6.22），那么需要设备实现[fuchsia.hardware.rpmb](/sdk/banjo/fuchsia.hardware.rpmb/rpmb.fidl)。

<!---

## SDIO core driver

The SDIO core driver creates devices that implement
[fuchsia.hardware.sdio](/sdk/banjo/fuchsia.hardware.sdio/sdio.fidl), one for
each IO function. Whereas the only expected client of the SD/eMMC block driver
is the storage stack, the SDIO driver will have different clients depending on
what kind of SDIO card is detected. Client drivers bind to the SDIO core driver
using the bind variables specified in the table below. Client drivers that use
more than one IO function should bind to a composite device that has each
function device as a fragment. Note that there could be multiple concurrent SDIO
client drivers for combo cards, e.g. for Bluetooth and WiFi, in which case
access to the bus will be shared through the core driver. Clients also cannot
directly access function 0 to prevent possibly disrupting other clients. See the
SDIO protocol definition for more information.

--->

## SDIO 核心驱动

 SDIO 核心驱动创建设备实现[fuchsia.hardware.sdio](/sdk/banjo/fuchsia.hardware.sdio/sdio.fidl)，对于每个 IO 功能各一个。而 SD/eMMC 块驱动的唯一预期客户端是存储栈， SDIO 驱动将依据是哪一种 SDIO 卡被检测到而拥有不同的客户端。客户端驱动使用在下述表中的绑定变量定义绑定在 SDIO 核心驱动上。使用多余一种 IO 功能的客户端驱动需要绑定在复合设备上，这样对于每一个功能设备都有一个分块。注意这可能对于组合卡有多个同时存在的 SDIO 客户端驱动，例如蓝牙和 WIFI， 在某种情景下，通过核心驱动的共享实现访问总线。客户端同样不能直接访问功能0，这样用于避免其他客户端的干扰。参考 SDIO 协议定义来获取更多信息。

<!---

### SDIO client binding

--->

### SDIO 客户端绑定

| Bind variable        | Meaning                                               |
| ---------------------| ------------------------------------------------------|
| `BIND_SDIO_VID`      | The IO function's manufacturer ID read from FBR       |
| `BIND_SDIO_PID`      | The IO function's product ID read from FBR            |
| `BIND_SDIO_FUNCTION` | The IO function number from 1 to 7                    |

<!---

## Device diagram

--->

## 设备框图

![SDMMC device diagram](images/sdmmc_architecture.png)
