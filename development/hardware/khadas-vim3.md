<!-- 
# Install Fuchsia on a Khadas VIM3 board
 -->
# 在 Khadas VIM3 板型上安装 Fuchsia

<!-- 
This guide shows you how to install Fuchsia on a
[Khadas VIM3](https://www.khadas.com/vim3). The installation
process will probably take between 1 to 3 hours.
 -->
本指南向您展示如何在 [Khadas VIM3](https://www.khadas.com/vim3) 上安装 Fuchsia。安装过程可能需要 1 到 3 个小时。

<!-- 
Running Fuchsia on VIM3 is useful if you want to explore how Fuchsia works on
relatively low-cost real hardware that supports many kinds of peripheral devices.
See [Appendix: Feature support](#features) for details on which VIM3 features
Fuchsia supports.
 -->
如果您想了解 Fuchsia 如何在相对低成本、支持多种外围设备的实际硬件上工作，那么在 VIM3 上运行 Fuchsia 非常有用。有关 Fuchsia 支持的 VIM3 功能的详细信息，请参阅[附录：支持的功能](#features)。

<!-- 
If you just want to explore Fuchsia with the lowest friction possible, check out
[Get started with the Fuchsia SDK](/docs/get-started/sdk/index.md) instead.
 -->
如果您只想以尽可能低的分歧探索 Fuchsia，请查看[开始使用 Fuchsia SDK](/docs/get-started/sdk/index.md)。

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
over a serial communication program like `minicom`.
 -->
如果您从未维修过电子产品，您可能会发现本指南难以完成。例如，本指南假设您知道如何将串行电缆连接到 GPIO 以读取日志并通过像 `minicom` 这样的串口通信程序发送命令。

<!-- 
This guide also assumes that you're comfortable
with CLI workflows such as building Fuchsia from source.
 -->
本指南还假设您熟悉 CLI 工作流程，例如从源代码构建 Fuchsia。

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
* 一个 [Khadas VIM3](https://www.khadas.com/vim3) 单板计算机。

<!-- 
  Caution: It's unknown whether Fuchsia will run on the Basic model VIM3.
  This guide was validated with the Pro model VIM3.
 -->
  注意：Fuchsia 是否能在 Basic 型号 VIM3 上运行尚不清楚。本指南已使用 Pro 型号 VIM3 进行了验证。

<!-- 
* A desktop or laptop computer that's running Linux and has 2 USB ports
  available.
 -->
* 一台运行 Linux 并具有 2 个 USB 端口可用的台式机或笔记本电脑。

<!-- 
  Key Term: This desktop or laptop is called the **host**
  throughout the rest of this guide.
 -->
  关键术语：此台式机或笔记本电脑在本指南的其余部分中称为**主机**。

<!-- 
  Caution: A macOS host may work but these instructions have not been validated
  with macOS. Building Fuchsia on a remote Linux computer and then attempting to flash
  Fuchsia onto the VIM3 with a local macOS host is known to not work.
 -->
  注意：macOS 主机可能会运行，但这些说明没有在 macOS 上得到验证。在远程 Linux 计算机上构建 Fuchsia，然后尝试使用本地 macOS 主机将 Fuchsia 刷入到 VIM3 上是无法运行的。

  <!-- Context from kayce@: Local macOS host + remote Linux workstation doesn't work because
       when you run `fx pave` you get an error about a mismatch between the local
       host OS and the remote workstation OS. -->

<!-- 
  Note: This guide assumes that your Linux distribution has Debian commands
  like `apt`.
 -->
  注意：本指南假设您的 Linux 发行版具有 Debian 命令，例如 `apt`。

<!-- 
* A power supply of at least 24W to your host. The VIM3 can draw that much power when
  [DVFS](https://en.wikipedia.org/wiki/Dynamic_frequency_scaling) is enabled.
 -->
* 为您的主机提供至少 24W 的电源。启用 [DVFS](https://en.wikipedia.org/wiki/Dynamic_frequency_scaling) 后，VIM3 可以消耗那么多功率 。

<!-- 
* A working Fuchsia development environment on your host. In other words you
  should be able to build Fuchsia from its source code on your host. See
  [Build Fuchsia](#build).
 -->
* 在您的主机上工作的 Fuchsia 开发环境。换句话说，您应该能够从主机上的源代码构建 Fuchsia。请参阅[构建 Fuchsia](#build)。

<!-- 
* A [USB to TTL serial cable](https://www.adafruit.com/product/954).
 -->
* 一条 [USB 转 TTL 串行电缆](https://www.adafruit.com/product/954)。

<!-- 
* A USB-C to USB-\* cable that supports both data and power delivery.
  The USB-C side is for the VIM3. The other side can be whatever USB
  type your host supports.
 -->
* 支持数据和电力传输的 USB-C 到 USB-\* 电缆。USB-C 端用于 VIM3。另一端可以是您的主机支持的任何 USB 类型。

<!-- 
The following is optional:
 -->
以下是可选的：

<!-- 
* A [heatsink](https://www.khadas.com/product-page/new-vim-heatsink).
  This enables running 2 CPU cores on the VIM3 at full speed without
  reaching 80°C, the critical temperature beyond which cores are throttled
  down.
 -->
* 一个[散热器](https://www.khadas.com/product-page/new-vim-heatsink)。这使得 VIM3 上的 2 个 CPU 内核能够全速运行，而不会达到 80°C，超过该临界温度，内核就会被节流。

<!-- 
See the [VIM3 collection](https://www.khadas.com/shop?Collection=VIM3&sort=price_descending)
in the Khadas shop for examples of compatible accessories.
 -->
有关兼容配件的示例，请参阅 Khadas 商店中的 [VIM3 系列](https://www.khadas.com/shop?Collection=VIM3&sort=price_descending)。

<!-- 
Note: All the links in this section are only for your convenience. You
don't need to buy from these exact stores or these exact parts.
 -->
注意：本节中的所有链接仅是为了您的方便。您不需要从这些确切的商店或这些确切的零件购买。

<!-- 
## Build Fuchsia {#build}
 -->
## 构建 Fuchsia {#build}

<!-- 
If you don't already have an [in-tree][glossary.in-tree] environment
set up, you should start the process now because it can take a while to
complete:
 -->
如果您还没有设置 [in-tree][glossary.in-tree] 环境，您应该现在就开始这个步骤，因为它可能需要一段时间才能完成：

<!-- 
1. [Download the Fuchsia source code](/docs/get-started/get_fuchsia_source.md).
 -->
1. [下载 Fuchsia 源代码](/docs/get-started/get_fuchsia_source.md)。

<!-- 
1. [Configure and build Fuchsia](/docs/get-started/build_fuchsia.md).
 -->
1. [配置和构建 Fuchsia](/docs/get-started/build_fuchsia.md)。

<!-- 
   * When building Fuchsia, use `fx set core.vim3` instead.
 -->
   * 构建 Fuchsia 时，请改用 `fx set core.vim3`。

<!-- 
Note: The rest of this guide assumes that your Fuchsia source code directory
is located at `~/fuchsia`.
 -->
注意: 本指南的其余部分假设你的 Fuchsia 源代码目录位于 `~/fuchsia`。

<!-- 
You'll use the Fuchsia development environment to build the Fuchsia image
for VIM3 and run an in-tree CLI tool for flashing the Fuchsia image onto
the VIM3.
 -->
您将使用 Fuchsia 开发环境为 VIM3 构建 Fuchsia 镜像，并运行一个树内 CLI 工具来将 Fuchsia 镜像刷入到 VIM3。

<!-- 
## Set up the hardware {#hardware}
 -->
## 设置硬件 {#hardware}

<!-- 
Set up the VIM3 to communicate with your host:
 -->
设置 VIM3 来与您的主机通信：

<!-- 
1. Connect the VIM3 and your host to each other with the USB-C to USB-\* cable.
   The white LED on the VIM3 should turn on.
 -->
1. 使用 USB-C 到 USB-\* 电缆将 VIM3 和您的主机相互连接。VIM3 上的白色 LED 会亮起。

<!-- 
   Caution: Don't put a [USB hub](https://en.wikipedia.org/wiki/USB_hub)
   between the VIM3 and your host. The hub may make it harder for your
   VIM3 and host to detect and communicate with each other.
 -->
   注意：不要在 VIM3 和主机之间使用 [USB 集线器](https://en.wikipedia.org/wiki/USB_hub)连接。集线器可能会使您的 VIM3 和主机更难检测和相互通信。

<!-- 
   This connection is used to power and flash the VIM3 with
   [`fastboot`](https://en.wikipedia.org/wiki/Fastboot).
 -->
   这个连接用于供电和用 [`fastboot`](https://en.wikipedia.org/wiki/Fastboot) 刷写 VIM3。

<!-- 
1. Connect the serial cable wires to the VIM3's GPIOs:
 -->
1. 将串行电缆连接到 VIM3 的 GPIO：

<!-- 
   * GND to pin 17.
 -->
   * GND 连接到引脚 17。

<!-- 
   * RX (in to VIM3) to pin 18.
 -->
   * RX（输入到 VIM3）连接到引脚 18。

<!-- 
   * TX (out from VIM3) to pin 19.
 -->
   * TX（从 VIM3 输出）连接到引脚 19。

<!-- 
   * Don't connect the power wire of your serial cable to any VIM3 GPIO.
     The VIM3 is getting power through the USB cable.
 -->
   * 不要把您串行电缆的电源线连接到 VIM3 的任何 GPIO 上。VIM3 是通过 USB 线获得电源的。

<!-- 
   Tip: Pins 1, 20, 21, and 40 are labeled on the circuit board.
 -->
   提示：引脚 1、20、21 和 40 标记在电路板上。

<!-- 
   Caution: In general the colors for TX and RX wires are not standardized.
   For example your RX wire may be blue or green.
 -->
   注意：通常 TX 和 RX 线的颜色不是标准化的。例如，您的 RX 线可能是蓝色或绿色。

<!-- 
   See [Serial Debugging Tool](https://docs.khadas.com/linux/vim3/SetupSerialTool.html)
   for an example image of how your serial wires should be connected to the VIM3.
 -->
   有关如何将串行线连接到 VIM3 的示例图像，请参阅[串行调试工具](https://docs.khadas.com/linux/vim3/SetupSerialTool.html)。

<!-- 
### Verify the serial connection {#serial}
 -->
### 验证串行连接 {#serial}

<!-- 
Make sure that you can view the logs being sent over the serial cable:
 -->
确保您可以查看通过串行电缆发送的日志：

<!-- 
1. Open a terminal in your host and run `ls /dev/ttyUSB*` before connecting the
   serial cable to a USB port on your host.
 -->
1. 在您的主机中打开一个终端，并且在将串行电缆连接到主机的 USB 端口之前，运行 `ls /dev/ttyUSB*`。

<!-- 
1. Connect the serial cable to your host and run `ls /dev/ttyUSB*` again.
   There should be 1 more result than the first time you ran the command,
   such as `/dev/ttyUSB0`. That is the USB connection between your VIM3
   and your host. You'll provide this result for the `Serial Device`
   value in the next step.
 -->
1. 将串行电缆连接到主机，然后再次运行 `ls /dev/ttyUSB*`。应该比第一次运行命令多出 1 个结果，例如 `/dev/ttyUSB0`。这是 VIM3 和主机之间的 USB 连接。您将在下一步中为 `Serial Device` 值提供这个结果。

<!-- 
   If you see no difference when running `ls /dev/ttyUSB*` before and after
   connecting the serial cable, try `ls /dev/tty*` or `ls /dev/*` instead.
 -->
   如果在连接串行电缆前后运行 `ls /dev/ttyUSB*` 没有区别，可以尝试 `ls /dev/tty*` 或 `ls /dev/*`。

<!-- 
1. Install, set up, and launch `minicom` on your host as explained in [Set Up Serial Communication
   Program](https://docs.khadas.com/linux/vim3/SetupSerialTool.html#Setup-Serial-Communication-Program).
 -->
1. 如[设置串行通信程序](https://docs.khadas.com/linux/vim3/SetupSerialTool.html#Setup-Serial-Communication-Program)中所述，在您的主机上安装、设置和启动 `minicom`。

<!-- 
   Key Term: In the rest of this guide the terminal window running `minicom` is called
   the **serial console**.
 -->
   关键术语：在本指南的其余部分中，运行的终端窗口 `minicom` 称为**串行控制台**。

<!-- 
   Note: This guide assumes that you're using `minicom` for your serial communication
   program but you can use whatever program you prefer.
 -->
   注意：本指南假设您使用的串行通信程序是 `minicom`，但您可以使用您喜欢的任何程序。

<!-- 
1. Press the reset button on the VIM3. The reset button is the one with the **R**
   printed next to it on the circuit board.
   See [VIM3/3L Hardware](https://docs.khadas.com/linux/vim3/Hardware.html) for
   a diagram. In your serial console you should see human-readable logs.
 -->
1. 按下 VIM3 上的复位按钮。复位按钮是电路板上旁边印有 **R** 的按钮。请参见 [VIM3/3L 硬件](https://docs.khadas.com/linux/vim3/Hardware.html) 获得图表。在串行控制台中，您应该看到人类可读的日志。

<!-- 
## Erase the eMMC {#emmc}
 -->
## 擦除 eMMC {#emmc}

<!-- 
In later sections of this guide you'll update the bootloader and
OS on the VIM3. These updates don't work unless you
completely erase the eMMC first:
 -->
在本指南的后面部分，您将更新 VIM3 上的引导加载程序和操作系统。除非您先完全擦除 eMMC，否则这些更新不起作用：

<!-- 
1. Press the reset button on your VIM3.
 -->
1. 按下 VIM3 上的重置按钮。

<!-- 
1. Right after you press the reset button, start repeatedly pressing the
   <kbd>Space</kbd> key as your VIM3 boots up. Make sure that your cursor
   is focused on your serial console. The bootloader process should pause
   and your serial console should show a `kvim3#` prompt. Your serial
   console is now providing you access to the **U-Boot shell**.
 -->
1. 在按下复位按钮之后，在 VIM3 启动时开始重复按 <kbd>Space</kbd> 键。确保光标聚焦在串行控制台上。引导加载程序过程应该会暂停，并且串行控制台应该显示 `kvim3#` 提示符。您的串行控制台现在提供了访问 **U-Boot shell** 的权限。

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
   您的串行控制台日志应验证 eMMC 被正确地擦除。

<!-- 
See [Erase eMMC](https://docs.khadas.com/linux/vim3/EraseEmmc.html)
for more details.
 -->
有关详细信息，请参阅[擦除 eMMC](https://docs.khadas.com/linux/vim3/EraseEmmc.html)。

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
VIM3 默认自带的 Android 镜像不支持 Fuchsia 安装。如果您刚从 Khadas 收到您的 VIM3，您必须更新您的 Android 镜像：

<!-- 
1. Click the following URL to download the updated Android image:
   <https://dl.khadas.com/Firmware/VIM3/Android/VIM3_Pie_V210527.7z>
 -->
1. 点击以下 URL 下载更新的 Android 镜像：<https://dl.khadas.com/Firmware/VIM3/Android/VIM3_Pie_V210527.7z>

<!-- 
1. Extract the compressed archive file (`VIM3_Pie_V210527.7z`).
   After the extraction you should have a `VIM3_Pie_V210527` directory
   with an `update.img` file in it.
 -->
1. 提取压缩存档文件（VIM3_Pie_V210527.7z）。提取后您应该有一个 `VIM3_Pie_V210527` 目录，其中包含一个 `update.img` 文件。

<!-- 
1. Follow the instructions in [Install OS into
   eMMC](https://docs.khadas.com/linux/vim3/InstallOsIntoEmmc.html).
   When running `aml-burn-tool` the value for the `-i` flag should be the
   path to your `update.img` file. Your command should look similar to this:
 -->
1. 按照[安装操作系统到 eMMC](https://docs.khadas.com/linux/vim3/InstallOsIntoEmmc.html) 中的说明进行操作。运行 `aml-burn-tool` 时，`-i` 标记的值应该是 `update.img` 文件的路径。您的命令应类似于以下内容：

   ```posix-terminal
   aml-burn-tool -b VIM3 -i ~/Downloads/VIM3_Pie_V210527/update.img
   ```

<!-- 
   Caution: Make sure that you're following the instructions for Ubuntu
   and VIM3 by clicking the **Install on Ubuntu** and **VIM3/VIM3L** tabs.
   These instructions are not shown by default.
 -->
   注意：通过单击**在 Ubuntu 上安装**和 **VIM3/VIM3L** 选项卡，确保您遵循 Ubuntu 和 VIM3 的说明。默认情况下不显示这些说明。

<!-- 
   Tip: The `TST Mode` workflow is probably the easiest and fastest way to get
   your VIM3 into Upgrade Mode.
 -->
   提示：`TST Mode` 工作流可能是使您的 VIM3 进入升级模式的最简单和最快的方法。

<!-- 
1. If the white and red LEDs on your VIM3 are off and the blue LED is on,
   it means that your VIM3 is in sleep mode. Try putting your VIM3
   back into [Upgrade Mode](https://docs.khadas.com/linux/vim3/BootIntoUpgradeMode.html)
   and then pressing the reset button again.
 -->
1. 如果您的 VIM3 上的白色和红色 LED 熄灭，而蓝色 LED 亮起，这意味着您的 VIM3 处于睡眠模式。尝试将您的 VIM3 回到[升级模式](https://docs.khadas.com/linux/vim3/BootIntoUpgradeMode.html)，然后再次按下复位按钮。

<!-- 
At this point the white LED on your VIM3 should be on and you should see
logs in your serial console after you press the reset button on your VIM3.
 -->
此时，您的 VIM3 上的白色 LED 应该亮起，并且在您按下 VIM3 上的重置按钮后，您应该会在串行控制台中看到一些日志。

<!-- 
## Update the bootloader {#bootloader}
 -->
## 更新引导加载程序 {#bootloader}

<!-- 
Flash Fuchsia's custom bootloader onto the VIM3:
 -->
将 Fuchsia 的自定义引导加载程序刷入到 VIM3：

<!-- 
1. Install the [Android SDK Platform
   Tools](https://developer.android.com/studio/releases/platform-tools).
 -->
1. 安装 [Android SDK 平台工具](https://developer.android.com/studio/releases/platform-tools)。

<!-- 
   Installing these tools gives you access to `adb`.
 -->
   安装这些工具可以让您访问 `adb`。

<!-- 
1. Verify that you can now run `adb`:
 -->
1. 验证您现在是否可以运行 `adb`：

   ```posix-terminal
   adb --version
   ```

<!-- 
1. Access the U-Boot shell again by pressing the reset button and
   then repeatedly pressing the <kbd>Space</kbd> key in your serial
   console. When your serial console shows the `kvim3#` prompt, you're
   in the U-Boot shell.
 -->
1. 按重置按钮，然后在串行控制台中反复按 <kbd>Space</kbd> 键，将会再次访问 U-Boot shell。当您的串行控制台显示 `kvim3#` 提示符时，您就进入了 U-Boot shell。

<!-- 
1. In your U-Boot shell run the following command:
 -->
1. 在您的 U-Boot shell 中运行以下命令：

   ```posix-terminal
   fastboot
   ```

<!-- 
   You should see the following logs in your serial console:
 -->
   您应该在串行控制台中看到以下日志：

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
   如果您看到第一行（`g_dnl_register: g_dnl_driver.name = usb_dnl_fastboot`）但之后没有其他行，请尝试使用不同的 USB-C 到 USB-\* 电缆并确保它同时支持数据和电力传输。

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
   注意：安装 Android SDK 平台工具可能会在您的主机上安装另一个 `fastboot` 实例。使用 Fuchsia 时，请记住使用在 `~/fuchsia/prebuild/third_party/fastboot/fastboot` 中 `fastboot` 的 [in-tree][glossary.in-tree] 版本。`fastboot` 协议允许任意供应商协议扩展，Fuchsia 将来可能会依赖此功能。

<!-- 
Note: You can also build the custom bootloader (`u-boot.bin.unsigned`) from source:
<https://third-party-mirror.googlesource.com/u-boot/+/refs/heads/vim3>
 -->
注意：您也可以从源代码中构建自定义引导程序（`u-boot.bin.unsigned`）：<https://third-party-mirror.googlesource.com/u-boot/+/refs/heads/vim3>

<!-- 
## Flash Fuchsia into the eMMC {#fuchsia}
 -->
## 将 Fuchsia 刷入 eMMC {#fuchsia}

<!-- 
Install Fuchsia onto your VIM3:
 -->
在你的 VIM3 上安装 Fuchsia：

<!-- 
1. Put your VIM3 into `fastboot` mode by pressing the reset button
   and then immediately pressing the <kbd>F</kbd> key.
 -->
1. 按下重置按钮，然后立即按下 <kbd>F</kbd> 键，使您的 VIM3 进入 `fastboot` 模式。

<!-- 
1. From a separate terminal on your host run the following command:
 -->
1. 从您主机上的单独终端运行以下命令：

   ```posix-terminal
   cd ~/fuchsia

   fx flash --pave
   ```

<!-- 
Your VIM3 is now running Fuchsia!
 -->
你的 VIM3 现在正在运行 Fuchsia！

<!-- 
Repeat the steps in this section whenever you want to flash a new Fuchsia
image onto your VIM3.
 -->
每当您想刷入新的 Fuchsia 镜像到 VIM3 时，请重复本节中的步骤。

<!-- 
## Appendix: Fix a bricked VIM3 {#bricks}
 -->
## 附录：修复一个变砖的 VIM3 {#bricks}

<!-- 
Do these steps if you've [bricked](https://en.wikipedia.org/wiki/Brick_(electronics))
your VIM3 and need to "factory reset" it:
 -->
如果您的 VIM3 已[变砖](https://en.wikipedia.org/wiki/Brick_(electronics))并需要“恢复出厂设置”，请执行以下步骤：

<!-- 
1. [Erase the eMMC](#emmc).
1. [Update the Android image](#android).
1. [Update the bootloader](#bootloader).
1. [Flash Fuchsia into the eMMC](#fuchsia).
 -->
1. [擦除 eMMC](#emmc).
1. [更新安卓镜像](#android).
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
  [Fuchsia mailing lists and chat rooms](/docs/contribute/community/mailing-lists.md).
 -->
* 对于似乎与 VIM3 硬件或固件相关的问题，请尝试 [VIM3 官方文档](https://docs.khadas.com/linux/vim3/index.html)和 [Khadas VIM3 官方论坛](https://forum.khadas.com/c/khadas-vim3/30)。
* 对于似乎与 Fuchsia 相关的问题，请尝试 [Fuchsia 邮件列表和聊天室](/docs/contribute/community/mailing-lists.md)。

<!-- 
## Appendix: Feature support {#features}
 -->
## 附录：功能支持 {#features}

<!-- 
Fuchsia currently supports these features of the VIM3:
 -->
Fuchsia 目前支持 VIM3 的这些特性：

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
* UART 串​​行调试器
* 通过以太网和 USB 铺路
* 存储 (eMMC)
* HDMI 显示和帧缓冲（Framebuffer）
* GPU（Mali）和 Vulkan 图形
* 以太网
* 安全数字输入输出（SDIO）
* 集成电路总线（I2C）
* 通用输入输出接口（GPIO）
* 温度传感器和 DVFS
* 实时时钟（RTC）
* 时钟
* 风扇
* NNA
* 外围模式下的 USB-C
* USB-A

<!-- 
These features are under development and may not be supported:
 -->
这些功能正在开发中，可能不受支持：

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
不支持以下功能，但未来的贡献可能会添加：

<!-- 
* SPI Flash
* USB-C in host mode
* Power management and PMIC
* Wake on LAN
* UART BT
 -->
* SPI 闪光灯
* 主机模式下的 USB-C
* 电源管理和电源管理 IC（PMIC）
* 局域网唤醒
* UART 蓝牙

<!-- 
These features are not supported and are unlikely to be added:
 -->
这些功能不受支持且不太可能添加：

<!-- 
* Video encoding (due to non-public firmware)
* Trusted Execution Environment / secure boot
 -->
* 视频编码（由于非公开固件）
* 可信执行环境/安全启动

<!-- 
## Appendix: Update the boot splash screen {#splash}
 -->
## 附录：更新开机启动画面 {#splash}

<!-- 
To update the boot splash screen to be the Fuchsia logo, run the following command
from a host terminal while the VIM3 is in `fastboot` mode:
 -->
要将开机启动屏幕画面更新为 Fuchsia 标识，请在 VIM3 处于 `fastboot` 模式时从主机终端运行以下命令：

```posix-terminal
fastboot flash logo ~/fuchsia/zircon/kernel/target/arm64/board/vim3/firmware/logo.img
```