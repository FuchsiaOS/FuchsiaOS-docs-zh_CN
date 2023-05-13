<!-- 
# Install Fuchsia on a Khadas VIM3 board
 -->
# 在 Khadas VIM3 开发板上安装 Fuchsia

<!-- 
This guide shows you how to install Fuchsia on a
[Khadas VIM3](https://www.khadas.com/vim3). The installation
process will probably take between 1 to 3 hours.
 -->
本指南向您展示在 [Khadas VIM3](https://www.khadas.com/vim3) 上安装 Fuchsia 的方法。安装过程可能需要 1 至 3 个小时。

<!-- 
Running Fuchsia on VIM3 is useful if you want to explore how Fuchsia works on
relatively low-cost real hardware that supports many kinds of peripheral devices.
See [Appendix: Feature support](#features) for details on which VIM3 features
Fuchsia supports.
 -->
如果您想在相对低成本、支持多种外设的实际硬件上了解 Fuchsia 的工作原理，那么在 VIM3 上运行 Fuchsia 非常有用。有关 Fuchsia 所支持的 VIM3 功能的详细信息，请参阅[附录：功能支持](#features)。

<!-- 
If you just want to explore Fuchsia with the lowest friction possible, check out
[Get started with the Fuchsia SDK](/get-started/sdk/index.md) instead.
 -->
如果您只想尽量低阻力地探索 Fuchsia，请转而查看[开始使用 Fuchsia SDK](/get-started/sdk/index.md)。

<!-- 
See [Appendix: Support](#support) if you have any trouble completing
this guide.
 -->
如果您在完成本指南时遇到任何问题，请参阅[附录：支持](#support)。

<!-- 
## Audience {#audience}
 -->
## 受众群体 {#audience}

<!-- 
If you've never tinkered with electronics you might find this
guide difficult to complete. For example, this guide assumes that you know
how to hook up serial cable wires to GPIOs to read logs and send commands
over a serial communication program.
 -->
如果您从未维修过电子产品，那么您可能会发现本指南难以完成。例如，本指南假设您知道如何将串行电缆连接到 GPIO 以读取日志并通过串口通信程序发送命令。

<!-- 
This guide also assumes that you're comfortable
with CLI workflows such as building Fuchsia from source.
 -->
本指南亦假设您熟悉命令行界面（CLI）工作流，例如从源代码构建 Fuchsia。

<!-- 
## Prerequisites {#prerequisites}
 -->
## 前提条件 {#prerequisites}

<!-- 
You'll need all of the following hardware and software to complete this guide:
 -->
您需要以下所有硬件和软件来完成本指南：

<!-- 
* A [Khadas VIM3](https://www.khadas.com/vim3) single-board computer.
 -->
* 一台 [Khadas VIM3](https://www.khadas.com/vim3) 单板计算机。

<!-- 
  Caution: It's unknown whether Fuchsia will run on the Basic model VIM3.
  This guide was validated with the Pro model VIM3.
 -->
  注意：Fuchsia 能否在 Basic 款 VIM3 上运行尚不清楚。本指南使用了 Pro 款 VIM3 进行验证。

<!-- 
* A desktop or laptop computer that's running Linux and has 2 USB ports
  available.
 -->
* 一台运行 Linux 并具有 2 个可用 USB 端口的台式机或笔记本电脑。

<!-- 
  Key Term: This desktop or laptop is called the **host**
  throughout the rest of this guide.
 -->
  关键术语：该台式机或笔记本电脑在本指南的其余部分中称为**主机**（host）。

<!-- 
  Caution: A macOS host may work but these instructions have not been validated
  with macOS. Building Fuchsia on a remote Linux computer and then attempting to flash
  Fuchsia onto the VIM3 with a local macOS host is known to not work.
 -->
  注意：macOS 作为主机可能可行，但这些说明并未在 macOS 上得到验证。已知在远程 Linux 计算机上构建 Fuchsia 并尝试使用本地 macOS 主机将 Fuchsia 刷入到 VIM3 上是不可行的。

  <!-- Context from kayce@: Local macOS host + remote Linux workstation doesn't work because
       when you run `fx pave` you get an error about a mismatch between the local
       host OS and the remote workstation OS. -->

<!-- 
  Note: This guide assumes that your Linux distribution has Debian commands
  like `apt`.
 -->
  注意：本指南假设您的 Linux 发行版具有例如 `apt` 的 Debian 命令。

<!-- 
* A power supply of at least 24W to your host. The VIM3 can draw that much power when
  [DVFS](https://en.wikipedia.org/wiki/Dynamic_frequency_scaling) is enabled.
 -->
* 一个能为您主机提供至少 24W 的电源。启用 [DVFS](https://en.wikipedia.org/wiki/Dynamic_frequency_scaling) 时，VIM3 可以消耗此数额的功率。

<!-- 
* A working Fuchsia development environment on your host. In other words you
  should be able to build Fuchsia from its source code on your host. See
  [Build Fuchsia](#build).
 -->
* 您主机上可用的 Fuchsia 开发环境。亦即，您应当能够从主机上的源代码构建 Fuchsia。请参阅[构建 Fuchsia](#build)。

<!-- 
* A [USB to TTL serial cable](https://www.adafruit.com/product/954).
 -->
* 一条 [USB 转 TTL 串行电缆](https://www.adafruit.com/product/954)。

<!-- 
* A USB-C to USB-\* cable that supports both data and power delivery.
  The USB-C side is for the VIM3. The other side can be whatever USB
  type your host supports.
 -->
* 一条支持数据和电力传输的 USB-C 转 USB-\* 线缆。USB-C 端用于 VIM3，另一端可以是您主机支持的任何 USB 类型。

<!-- 
The following is optional:
 -->
以下为可选项：

<!-- 
* A [heatsink](https://www.khadas.com/product-page/new-vim-heatsink).
  This enables running 2 CPU cores on the VIM3 at full speed without
  reaching 80°C, the critical temperature beyond which cores are throttled
  down.
 -->
* 一个[散热器](https://www.khadas.com/product-page/new-vim-heatsink)。这使得 VIM3 上的 2 个 CPU 核心能够全速运行而不会达到 80°C，超过该临界温度时，核心会降频。

<!-- 
See the [VIM3 collection](https://www.khadas.com/shop?Collection=VIM3&sort=price_descending)
in the Khadas shop for examples of compatible accessories.
 -->
要获取关于兼容配件的示例，请参阅 Khadas 商店中的 [VIM3 系列](https://www.khadas.com/shop?Collection=VIM3&sort=price_descending)。

<!-- 
Note: All the links in this section are only for your convenience. You
don't need to buy from these exact stores or these exact parts.
 -->
注意：本节中的所有链接均仅出于您的方便考虑而提供。您并不一定非要从这些商店购买这些配件。

<!-- 
## Build Fuchsia {#build}
 -->
## 构建 Fuchsia {#build}

<!-- 
If you don't already have an [in-tree][glossary.in-tree] environment
set up, you should start the process now because it can take a while to
complete:
 -->
如果您还没有设置[树内][glossary.in-tree]环境，您应当现在就开始这一步骤，因为它可能需要一段时间才能完成：

<!--
1. [Download the Fuchsia source code](/get-started/get_fuchsia_source.md).
 -->
1. [下载 Fuchsia 源代码](/get-started/get_fuchsia_source.md)。

<!--
1. [Configure and build Fuchsia](/get-started/build_fuchsia.md).
 -->
1. [配置并构建 Fuchsia](/get-started/build_fuchsia.md)。

<!--
   * When building Fuchsia, use `fx set core.vim3` instead.
 -->
   * 构建 Fuchsia 时，请转而使用 `fx set core.vim3`。

<!-- 
Note: The rest of this guide assumes that your Fuchsia source code directory
is located at `~/fuchsia`.
 -->
注意: 本指南的其余部分假设您的 Fuchsia 源代码目录位于 `~/fuchsia`。

<!--
Important: Whenever you see an `fx` command this guide assumes that your
working directory is within your in-tree Fuchsia source code checkout.
In other words this guide assumes that you run `cd ~/fuchsia` before
running any `fx` commands.
 -->
重要提示：每当您看到 `fx` 命令时，本指南都假设您的工作目录位于您的树内 Fuchsia 源代码检出（checkout）内。亦即，本指南假设您在运行任何 `fx` 命令前都先运行了 `cd ~/fuchsia`。

<!--
You'll use the Fuchsia development environment to build the Fuchsia image
for VIM3 and run an in-tree CLI tool for flashing the Fuchsia image onto
the VIM3.
 -->
您将使用 Fuchsia 开发环境为 VIM3 构建 Fuchsia 镜像，并运行一个树内 CLI 工具将 Fuchsia 镜像刷入到 VIM3。

<!-- 
## Set up the hardware {#hardware}
 -->
## 设置硬件 {#hardware}

<!-- 
Set up the VIM3 to communicate with your host:
 -->
设置 VIM3 以与您的主机通信：

<!-- 
1. Connect the VIM3 and your host to each other with the USB-C to USB-\* cable.
   The white LED on the VIM3 should turn on.
 -->
1. 使用 USB-C 转 USB-\* 线缆将 VIM3 和您的主机相互连接。VIM3 上的白色 LED 灯会亮起。

<!-- 
   Caution: Don't put a [USB hub](https://en.wikipedia.org/wiki/USB_hub)
   between the VIM3 and your host. The hub may make it harder for your
   VIM3 and host to detect and communicate with each other.
 -->
   注意：不要在 VIM3 和主机之间使用 [USB 集线器](https://en.wikipedia.org/wiki/USB_hub)。集线器可能会使您的 VIM3 和主机更难相互检测与通信。

<!-- 
   This connection is used to power and flash the VIM3 with
   [`fastboot`](https://en.wikipedia.org/wiki/Fastboot).
 -->
   该连接用于供电及使用 [`fastboot`](https://en.wikipedia.org/wiki/Fastboot) 刷写 VIM3。

<!-- 
1. Connect the serial cable wires to the VIM3's GPIOs:
 -->
1. 将串行电缆连接到 VIM3 的 GPIO：

<!-- 
   * GND to pin 17.
 -->
   * GND 接引脚 17。

<!-- 
   * RX (in to VIM3) to pin 18.
 -->
   * RX（输入至 VIM3）接引脚 18。

<!-- 
   * TX (out from VIM3) to pin 19.
 -->
   * TX（输出自 VIM3）接引脚 19。

<!-- 
   * Don't connect the power wire of your serial cable to any VIM3 GPIO.
     The VIM3 is getting power through the USB cable.
 -->
   * 不要将您串行电缆的电源线连接到 VIM3 的任何 GPIO 上。VIM3 是通过 USB 线缆供电的。

<!-- 
   Tip: Pins 1, 20, 21, and 40 are labeled on the circuit board.
 -->
   提示：引脚 1、20、21 和 40 在电路板上是标出的。

<!-- 
   Caution: In general the colors for TX and RX wires are not standardized.
   For example your RX wire may be blue or green.
 -->
   注意：通常 TX 和 RX 线的颜色不是标准化的。例如，您的 RX 线可能是蓝色或绿色。

<!--
   See [Serial Debugging Tool](https://docs.khadas.com/products/sbc/vim3/development/setup-serial-tool)
   for an example image of how your serial wires should be connected to the VIM3.
 -->
   要获取关于将串行线连接到 VIM3 方法的示例图像，请参阅[串行调试工具](https://docs.khadas.com/products/sbc/vim3/development/setup-serial-tool)。

<!-- 
### Verify the serial connection {#serial}
 -->
### 验证串行连接 {#serial}

<!-- 
Make sure that you can view the logs being sent over the serial cable:
 -->
请确保您可以查看通过串行电缆发送的日志：

<!--
1. Open Fuchsia's serial console:
 -->
1. 打开 Fuchsia 的串行控制台：

   ```posix-terminal
   fx serial
   ```

<!--
   Note: If `fx serial` detects multiple USB devices and you don't know which one to use,
   try disconnecting the serial cable from your host, running `ls /dev/ttyUSB*`,
   then re-connecting the serial cable and running the command again. If you see
   no difference when running `ls /dev/ttyUSB*` try `ls /dev/tty*` or `ls /dev/*` instead.
 -->
   注意：如果 `fx serial` 检测到多个 USB 设备，而您不知道要使用哪个，请尝试从主机上断开串行电缆，运行 `ls /dev/ttyUSB*`，然后重新连接串行电缆并再次运行命令。如果您在运行 `ls /dev/ttyUSB*` 时没有反应，则请尝试使用 `ls /dev/tty*` 或 `ls /dev/*`。

<!-- 
1. Press the reset button on the VIM3. The reset button is the one with the **R**
   printed next to it on the circuit board.
   See [VIM3/3L Hardware](https://docs.khadas.com/products/sbc/vim3/hardware/start) for
   a diagram. In your serial console you should see human-readable logs.
 -->
1. 按下 VIM3 上的复位按钮。复位按钮是电路板上旁边印有 **R** 的按钮。要获取图示，请参阅 [VIM3/3L 硬件](https://docs.khadas.com/products/sbc/vim3/hardware/start)。在串行控制台中，您应该看到具有可读性的日志。

<!-- 
## Erase the eMMC {#emmc}
 -->
## 擦除 eMMC {#emmc}

<!-- 
In later sections of this guide you'll update the bootloader and
OS on the VIM3. These updates don't work unless you
completely erase the eMMC first:
 -->
在本指南之后的部分，您将更新 VIM3 上的引导加载程序（bootloader）和操作系统。除非您先完全擦除 eMMC，否则这些更新不起作用：

<!-- 
1. Press the reset button on your VIM3.
 -->
1. 按下您 VIM3 上的重置按钮。

<!-- 
1. Right after you press the reset button, start repeatedly pressing the
   <kbd>Space</kbd> key as your VIM3 boots up. Make sure that your cursor
   is focused on your serial console. The bootloader process should pause
   and your serial console should show a `kvim3#` prompt. Your serial
   console is now providing you access to the **U-Boot shell**.
 -->
1. 按下复位按钮之后，在 VIM3 启动时立即重复按下 <kbd>Space</kbd>（空格）键。请确保您的光标聚焦在串行控制台上。引导加载程序的过程应会暂停，并且串行控制台上应会显示 `kvim3#` 提示符。您的串行控制台现在向您提供了访问 **U-Boot shell** 的权限。

<!-- 
1. Run the following command in the U-Boot shell:
 -->
1. 在 U-Boot shell 中运行以下命令：

   ```posix-terminal
   store init 3
   ```

<!-- 
   Your serial console logs should verify that the eMMC was correctly erased.
 -->
   您的串行控制台日志应当证实 eMMC 已正确擦除。

<!--
See [Erase eMMC](https://docs.khadas.com/products/sbc/vim3/development/erase-emmc)
for more details.
 -->
要获取更多细节，请参阅[擦除 eMMC](https://docs.khadas.com/products/sbc/vim3/development/erase-emmc)。

<!-- 
## Update the Android image on the VIM3 {#android}
 -->
## 更新 VIM3 上的 Android 镜像 {#android}

<!-- Context: https://forum.khadas.com/t/unable-to-change-bootloader-for-vim3/12708/6 -->

<!-- 
The Android image that ships by default on the VIM3 does
not support Fuchsia installation. If you just received your VIM3
from Khadas you must update your Android image:
 -->
VIM3 默认自带的 Android 镜像不支持 Fuchsia 安装。如果您刚收到来自 Khadas 的 VIM3，那么您必须更新您的 Android 镜像：

<!-- 
1. Click the following URL to download the updated Android image:
   <https://dl.khadas.com/firmware/vim3/android/VIM3_Pie_V211220.7z>
 -->
1. 点击以下网址下载更新的 Android 镜像：<https://dl.khadas.com/firmware/vim3/android/VIM3_Pie_V211220.7z>

<!-- 
1. Extract the compressed archive file (`VIM3_Pie_V211220.7z`).
   After the extraction you should have a `VIM3_Pie_V211220` directory
   with an `update.img` file in it.
 -->
1. 解压压缩存档文件（`VIM3_Pie_V211220.7z`）。解压后您应当拥有 `VIM3_Pie_V211220` 目录，其中包含一个 `update.img` 文件。

<!-- 
1. Follow the instructions in [Install OS into
   eMMC](https://docs.khadas.com/products/sbc/vim3/install-os/install-os-into-emmc-via-usb-tool).
   When running `aml-burn-tool` the value for the `-i` flag should be the
   path to your `update.img` file. Your command should look similar to this:
 -->
1. 按照[将操作系统安装至 eMMC](https://docs.khadas.com/products/sbc/vim3/install-os/install-os-into-emmc-via-usb-tool) 中的说明进行操作。运行 `aml-burn-tool` 时，`-i` 标志的值应为 `update.img` 文件的路径。您的命令应类似于：

   ```posix-terminal
   aml-burn-tool -b VIM3 -i ~/Downloads/VIM3_Pie_V211220/update.img
   ```

<!-- 
   Caution: Make sure that you're following the instructions for Ubuntu
   and VIM3 by clicking the **Install on Ubuntu** and **VIM3/VIM3L** tabs.
   These instructions are not shown by default.
 -->
   注意：请通过点击 **Install on Ubuntu**（在 Ubuntu 上安装）和 **VIM3/VIM3L** 选项卡，确保您遵循了 Ubuntu 和 VIM3 的说明。这些说明默认不显示。

<!-- 
   Tip: The `TST Mode` workflow is probably the easiest and fastest way to get
   your VIM3 into Upgrade Mode.
 -->
   提示：`TST Mode` 工作流可能是使您的 VIM3 进入更新模式（Update Mode）的最简单和最快的方法。

<!-- 
1. If the white and red LEDs on your VIM3 are off and the blue LED is on,
   it means that your VIM3 is in sleep mode. Try putting your VIM3
   back into [Upgrade Mode](https://docs.khadas.com/products/sbc/vim3/install-os/boot-into-upgrade-mode)
   and then pressing the reset button again.
 -->
1. 如果您的 VIM3 上的白色和红色 LED 灯熄灭，而蓝色 LED 灯亮起，则表示您的 VIM3 处于睡眠模式。请尝试使您的 VIM3 回到[更新模式](https://docs.khadas.com/products/sbc/vim3/install-os/boot-into-upgrade-mode)，并再次按下复位按钮。

<!-- 
At this point the white LED on your VIM3 should be on and you should see
logs in your serial console after you press the reset button on your VIM3.
 -->
此时，您 VIM3 上的白色 LED 灯应当亮起，并且在按下 VIM3 上的重置按钮后，您应当在串行控制台中看到一些日志。

<!-- 
## Update the bootloader {#bootloader}
 -->
## 更新引导加载程序 {#bootloader}

<!-- 
Flash Fuchsia's custom bootloader onto the VIM3:
 -->
将 Fuchsia 的自定义引导加载程序刷入到 VIM3：

<!--
1. Access the U-Boot shell again by pressing the reset button and
   then repeatedly pressing the <kbd>Space</kbd> key in your serial
   console. When your serial console shows the `kvim3#` prompt, you're
   in the U-Boot shell.
 -->
1. 按下重置按钮，然后在串行控制台中反复按 <kbd>Space</kbd>（空格）键，将会再次访问 U-Boot shell。当您的串行控制台显示 `kvim3#` 提示符时，您就进入了 U-Boot shell。

<!-- 
1. In your U-Boot shell run the following command:
 -->
1. 在 U-Boot shell 中运行以下命令：

   ```posix-terminal
   fastboot
   ```

<!-- 
   You should see the following logs in your serial console:
 -->
   您应当在串行控制台中看到以下日志：

   ```
   g_dnl_register: g_dnl_driver.name = usb_dnl_fastboot

   USB RESET
   SPEED ENUM

   USB RESET
   SPEED ENUM
   ```

<!-- 
   If you see the first line (`g_dnl_register: g_dnl_driver.name = usb_dnl_fastboot`)
   but not the lines after that, try using a different USB-C to USB-\* cable and make
   sure that it supports both data and power delivery.
 -->
   如果您看到了第一行文字（`g_dnl_register: g_dnl_driver.name = usb_dnl_fastboot`），但之后没有其他文字，则请尝试使用不同的 USB-C 转 USB-\* 线缆并确保它同时支持数据和电力传输。

<!-- 
1. Open a new terminal window in your host and run the following commands:
 -->
1. 在您的主机中打开一个新的终端窗口并运行以下命令：

   ```posix-terminal
   cd ~/fuchsia/prebuilt/third_party/fastboot

   ./fastboot flashing unlock

   ./fastboot flashing unlock_critical

   ./fastboot flash bootloader ~/fuchsia/prebuilt/third_party/firmware/vim3/u-boot.bin.unsigned

   ./fastboot reboot
   ```

<!-- 
   Caution: Installing the Android SDK Platform Tools probably installed another
   instance of `fastboot` on your host. When working with Fuchsia, remember to use
   the [in-tree][glossary.in-tree] version of `fastboot` at
   `~/fuchsia/prebuild/third_party/fastboot/fastboot`. The `fastboot` protocol allows arbitrary
   vendor protocol extensions and Fuchsia may rely on this functionality in the future.
 -->
   注意：安装 Android SDK 平台工具可能会在您的主机上安装另一个 `fastboot` 实例。使用 Fuchsia 时，请记住使用位于 `~/fuchsia/prebuild/third_party/fastboot/fastboot` 的[树内][glossary.in-tree]版本的 `fastboot`。`fastboot` 协议允许供应商进行任意的协议扩展，Fuchsia 未来可能会依赖此功能。

<!-- 
Note: You can also build the custom bootloader (`u-boot.bin.unsigned`) from source:
<https://third-party-mirror.googlesource.com/u-boot/+/refs/heads/vim3>
 -->
注意：您也可以从源代码构建自定义引导加载程序（`u-boot.bin.unsigned`）：<https://third-party-mirror.googlesource.com/u-boot/+/refs/heads/vim3>

<!-- 
## Flash Fuchsia into the eMMC {#fuchsia}
 -->
## 将 Fuchsia 刷入 eMMC {#fuchsia}

<!--
Complete these steps to install Fuchsia onto your VIM3 for the first time. You only
need to do these steps once. Once you have Fuchsia running on your VIM3, the
[Update your Fuchsia image](#update) workflow is a faster way to update the Fuchsia
image on your VIM3.
 -->
请完成以下步骤，以将 Fuchsia 首次安装到您的 VIM3 上。您只需执行这些步骤一次。VIM3 上运行 Fuchsia 后，[更新您的 Fuchsia 镜像](#update) 工作流是更新 VIM3 上 Fuchsia 镜像较快的方法。

<!--
1. If you just ran the `./fastboot reboot` command from the last section then
   your VIM3 should already be in `fastboot` mode. You can check your `fx serial`
   logs to confirm. Otherwise press the reset button and then repeatedly press
   the <kbd>F</kbd> in your `fx serial` console until you see `USB RESET` and
   `SPEED ENUM` again.
 -->
如果您刚刚运行了上一节中的 `./fastboot reboot` 命令，那么您的 VIM3 应当已经处于 `fastboot` 模式。您可以通过检查您的 `fx serial` 日志来确认。否则请按下重置按钮，然后在您的 `fx serial` 控制台中反复按下 <kbd>F</kbd> 键，直到您再次看到 `USB RESET` 和 `SPEED ENUM`。

<!--
   Important: You have to press the <kbd>F</kbd> key now to enter <code>fastboot</code>
   mode. Previously you pressed the <kbd>Space</kbd> key.
 -->
   重要提示：您现在必须按下 <kbd>F</kbd> 键才能进入 <code>fastboot</code> 模式。之前按下的是 <kbd>Space</kbd>（空格）键。

<!--
1. From a separate terminal on your host run the following command:
 -->
1. 从您主机上的另一终端运行以下命令：

   ```posix-terminal
   fx flash
   ```

<!-- 
Your VIM3 is now running Fuchsia!
 -->
您的 VIM3 现在运行 Fuchsia 了！

<!--
## Update your Fuchsia image {#update}
 -->
## 更新您的 Fuchsia 镜像 {#update}

<!--
Complete these steps when you already have Fuchsia running on your VIM3
and want to update the Fuchsia image on your VIM3.
 -->
当您已经在 VIM3 上运行 Fuchsia，并想更新其上的 Fuchsia 镜像时，请完成以下步骤。

<!--
1. Run the following command from a terminal on your host:
 -->
1. 从主机上的终端运行以下命令：

   ```posix-terminal
   fx serve
   ```

<!--
   Leave this command running.
 -->
   请让该命令自行运行。

<!--
1. Make some changes in your in-tree Fuchsia checkout and build the changes.
 -->
1. 对树内 Fuchsia 检出进行一些更改，并构建更改。

<!--
1. Open a new terminal window and perform an OTA update of the Fuchsia image on your VIM3:
 -->
1. 打开一个新的终端窗口，并在 VIM3 上执行 Fuchsia 镜像的 OTA 更新：

   ```posix-terminal
   fx ota
   ```

<!--
## Appendix: Fix a bricked VIM3 {#bricks}
 -->
## 附录：修复变砖的 VIM3 {#bricks}

<!-- 
Do these steps if you've [bricked](https://en.wikipedia.org/wiki/Brick_(electronics))
your VIM3 and need to "factory reset" it:
 -->
如果您的 VIM3 已[变砖](https://en.wikipedia.org/wiki/Brick_(electronics))，需要“恢复出厂设置”，则请执行以下步骤：

<!-- 
1. [Erase the eMMC](#emmc).
1. [Update the Android image](#android).
1. [Update the bootloader](#bootloader).
1. [Flash Fuchsia into the eMMC](#fuchsia).
 -->
1. [擦除 eMMC](#emmc).
1. [更新 Android 镜像](#android).
1. [更新引导加载程序](#bootloader).
1. [将 Fuchsia 刷入 eMMC](#fuchsia).

<!-- 
## Appendix: Support {#support}
 -->
## 附录：支持 {#support}

<!-- 
* For issues that seem related to VIM3 hardware or firmware, try the
  [VIM3 official docs](https://docs.khadas.com/linux/vim3/index.html) and
  [Khadas VIM3 official forum](https://forum.khadas.com/c/khadas-vim3/30).
* For issues that seem related to Fuchsia, try the
  [Fuchsia mailing lists and chat rooms](/contribute/community/mailing-lists.md).
 -->
* 对于可能与 VIM3 硬件或固件有关的问题，请尝试使用 [VIM3 官方文档](https://docs.khadas.com/linux/vim3/index.html)和 [Khadas VIM3 官方论坛](https://forum.khadas.com/c/khadas-vim3/30)。
* 对于可能与 Fuchsia 有关的问题，请尝试使用 [Fuchsia 邮件列表和聊天室](/contribute/community/mailing-lists.md)。

<!-- 
## Appendix: Feature support {#features}
 -->
## 附录：功能支持 {#features}

<!-- 
Fuchsia currently supports these features of the VIM3:
 -->
Fuchsia 目前支持 VIM3 的下列功能：

<!-- 
* UART Serial Debugger
* Paving over ethernet and USB
* Storage (eMMC)
* HDMI Display and Framebuffer
* GPU (Mali) and Vulkan graphics
* Ethernet
* SDIO
* I2C
* GPIO
* Temperature Sensors and DVFS
* RTC
* Clock
* Fan
* NNA
* USB-C in peripheral mode
* USB-A
 -->
* UART 串行调试器
* 通过以太网和 USB 铺设（pave）
* 存储（eMMC）
* HDMI 显示和帧缓冲
* GPU（Mali）和 Vulkan 图形
* 以太网
* SDIO
* 集成电路总线（I2C）
* GPIO
* 温度传感器和动态电压频率调整（DVFS）
* 实时时钟（RTC）
* 时钟
* 风扇
* 神经网络加速器（NNA）
* 外设模式（peripheral mode）下的 USB-C
* USB-A

<!-- 
These features are under development and may not be supported:
 -->
下列功能正在开发中，可能不受支持：

<!-- 
* Video decoder
* SPI
* Audio
 -->
* 视频解码器
* 串行外设接口（SPI）
* 音频

<!-- 
The following features are not supported, but might be added by future
contributions:
 -->
下列功能不受支持，但未来可能通过代码贡献添加：

<!-- 
* SPI Flash
* USB-C in host mode
* Power management and PMIC
* Wake on LAN
* UART BT
 -->
* SPI 闪存（SPI Flash）
* 主机模式（host mode）下的 USB-C
* 电源管理和 PMIC
* 局域网唤醒
* UART 蓝牙（UART BT）

<!-- 
These features are not supported and are unlikely to be added:
 -->
下列功能不受支持，且不太可能添加：

<!-- 
* Video encoding (due to non-public firmware)
* Trusted Execution Environment / secure boot
 -->
* 视频编码（受非公开固件限制）
* 可信执行环境/安全引导

<!-- 
## Appendix: Update the boot splash screen {#splash}
 -->
## 附录：更新开机启动画面 {#splash}

<!-- 
To update the boot splash screen to be the Fuchsia logo, run the following command
from a host terminal while the VIM3 is in `fastboot` mode:
 -->
要将开机启动画面更新为 Fuchsia 标志，请在 VIM3 处于 `fastboot` 模式时从主机终端运行以下命令：

```posix-terminal
fastboot flash logo ~/fuchsia/zircon/kernel/target/arm64/board/vim3/firmware/logo.img
```
