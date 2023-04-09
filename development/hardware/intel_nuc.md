<!--
# Install Fuchsia on a NUC
 -->
# 在 NUC 迷你电脑上安装 Fuchsia

<!--
This guide provides instructions on how to install Fuchsia on an
Intel [NUC][nuc-wiki]{:.external} (Next Unit of Computing) device.
 -->
本指南提供了有关在 Intel [NUC][nuc-wiki]{:.external}（Next Unit of Computing）迷你电脑设备上 Fuchsia 安装方法的说明。

<!--
The steps are:
 -->
安装步骤为：

<!--
1. [Prerequisites](#prerequisites).
1. [Build Fuchsia](#build-fuchsia).
1. [Prepare a USB drive](#prepare-usb).
1. [Enable EFI booting on the NUC](#enable-efi-booting).
1. [Install Fuchsia on the NUC](#install-fuchsia).
 -->
1. [前提条件](#prerequisites)。
1. [构建 Fuchsia](#build-fuchsia)。
1. [准备 USB 驱动器](#prepare-usb)。
1. [启用 NUC 迷你电脑上的 EFI 引导](#enable-efi-booting)。
1. [将 Fuchsia 安装至 NUC 迷你电脑](#install-fuchsia)。

<!--
## 1. Prerequisites {#prerequisites}
 -->
## 1. 前提条件 {#prerequisites}

<!--
Before you start installing Fuchsia on a NUC device, make sure that
you've completed the following tasks:
 -->
开始在 NUC 迷你电脑设备上安装 Fuchsia 之前，请确保您完成了以下工作：

<!--
* [Set up the Fuchsia development environment](#set-up-fuchsia-env)
* [Get parts](#get-parts)
 -->
* [设置 Fuchsia 开发环境](#set-up-fuchsia-env)
* [获取配件](#get-parts)

<!--
### Set up the Fuchsia development environment {#set-up-fuchsia-env}
 -->
### 设置 Fuchsia 开发环境 {#set-up-fuchsia-env}

<!--
To set up the Fuchsia development environment on your workstation,
complete the [Get started with Fuchsia][get-started-with-fuchsia] guide.
 -->
要在工作站上建立 Fuchsia 开发环境，请您完成[开始使用 Fuchsia][get-started-with-fuchsia] 指南。

<!--
### Get parts {#get-parts}
 -->
### 获取配件 {#get-parts}

<!--
Note: Fuchsia only supports the specific system configurations listed in
[Supported system configurations][supported-sys-config].
 -->
注意：Fuchsia 仅支持列入[支持的系统配置][supported-sys-config]清单的特定系统配置。

<!--
The following parts are required for this guide:
 -->
本指南需要使用以下配件：

<!--
*  A NUC device (see [example models](#supported-nuc-models))
*  A USB 3.0 flash drive
*  A keyboard
*  A mouse (Optional)
*  A monitor with an HDMI port
*  An HDMI cable
*  An Ethernet cable
*  A Phillips-head screwdriver (with a magnetic tip)
 -->
*  一台 NUC 迷你电脑设备（请参阅[示例型号](#supported-nuc-models)）
*  一个 USB 3.0 闪存驱动器
*  一个键盘
*  一只鼠标（可选）
*  一台带有 HDMI 端口的显示器
*  一条 HDMI 线缆
*  一条以太网线缆
*  一把十字头螺丝刀（带有磁性尖端）

<!--
Note: The [_2. Build Fuchsia_](#build-fuchsia) and
[_3. Prepare a USB drive_](#prepare-usb) sections do not require a NUC
device, so you can complete these sections prior to obtaining a NUC device.
However, you will need a USB flash drive for the _3. Prepare a USB drive_
section.
 -->
注意：“[2. 构建 Fuchsia](#build-fuchsia)”和“[3. 准备 USB 驱动器](#prepare-usb)”两节不需要 NUC 迷你电脑设备，因此您可以在获得 NUC 迷你电脑设备之前完成这些章节。但是，“3. 准备 USB 驱动器”一节需要 USB 闪存驱动器。

<!--
## 2. Build Fuchsia {#build-fuchsia}
 -->
## 2. 构建 Fuchsia {#build-fuchsia}

<!--
Installing Fuchsia on a NUC device requires that you build a Workstation
image (`workstation_eng.x64`) and generate build artifacts (which include
the Fuchsia installer) on your workstation.
 -->
要在 NUC 迷你电脑设备上安装 Fuchsia，您需要构建工作站镜像（`workstation_eng.x64`），并在工作站上生成构建内容（包括 Fuchsia 安装程序）。

<!--
To build Fuchsia for NUC installation, do the following:
 -->
要构建安装在 NUC 迷你电脑上的 Fuchsia，请执行以下操作：

<!--
1. Set your build configuration to `workstation_eng.x64` and include the
   recovery package (`recovery-installer`):
 -->
1. 将构建配置设置为 `workstation_eng.x64` 恢复包（`recovery-installer`）：

   ```posix-terminal
   fx set workstation_eng.x64 --with //build/images/recovery:recovery-installer
   ```

<!--
1.  Build Fuchsia:
 -->
1. 构建 Fuchsia ：

   ```posix-terminal
   fx build
   ```

<!--
    Building Fuchsia can take up to 90 minutes.
 -->
   Fuchsia 构建可能长达 90 分钟。

<!--
## 3. Prepare a USB drive {#prepare-usb}
 -->
## 3. 准备 USB 驱动器 {#prepare-usb}

<!--
You need to prepare a bootable USB drive that runs the Fuchsia installer.
Later in the [Install Fuchsia on the NUC](#install-fuchsia) section,
you will use this USB drive to boot your NUC into the Fuchsia installer.
 -->
您需要准备一个运行 Fuchsia 安装程序的可引导 USB 驱动器。稍后，在[将 Fuchsia 安装至 NUC 迷你电脑](#install-fuchsia)一节，您将使用该 USB 驱动器将 NUC 迷你电脑引导至 Fuchsia 安装程序。

<!--
Note: The instructions below require that you've completed the
build in the previous [Build Fuchsia](#build-fuchsia) section.
 -->
注意：下面的说明要求您已经在先前的[构建 Fuchsia](#build-fuchsia) 一节完成了构建。

<!--
To prepare a bootable USB drive, do the following:
 -->
要准备可引导 USB 驱动器，请执行以下操作：

<!--
1. Plug the USB drive into **your workstation**.
 -->
1. 将 USB 驱动器插入**您的工作站**。

<!--
1. Identify the path to the USB drive:
 -->
1. 确定 USB 驱动器路径：

   ```posix-terminal
   fx list-usb-disks
   ```

<!--
   This command prints output similar to the following:
 -->
   该命令打印的输出内容形如：

   ```none {:.devsite-disable-click-to-copy}
   $ fx list-usb-disks
   /dev/sda - My Example USB Disk
   ```

<!--
1. Create a bootable USB drive:
 -->
1. 创建可引导 USB 驱动器：

   ```posix-terminal
   fx mkinstaller -v --new-installer {{ "<var>" }}PATH_TO_USB_DRIVE{{ "</var>" }}
   ```

<!--
   Replace `PATH_TO_USB_DRIVE` with the path to the USB drive from the step
   above.
 -->
   在上述步骤中将 `PATH_TO_USB_DRIVE` 替换为 USB 驱动器路径。

<!--
   The example command below selects the `/dev/sda` path:
 -->
   下面的示例命令选择了 `/dev/sda` 路径：

   ```none {:.devsite-disable-click-to-copy}
   $ fx mkinstaller -v --new-installer /dev/sda
   ```

<!--
   When finished, the command prints output similar to the following
   in the end:
 -->
   完成后，命令最终的输出结果形如：

   ```none {:.devsite-disable-click-to-copy}
   $ fx mkinstaller -v --new-installer /dev/sda
   mkinstaller: WARNING: Changing ownership of /dev/sda to alice
   [sudo] password for alice:
   ...
   mkinstaller: INFO:    Writing image fvm.sparse.blk to partition storage-sparse...
   mkinstaller: INFO:      Wrote 835.6M in 35.55s, 23.5M/s
   mkinstaller: INFO: Done.
   mkinstaller: INFO: Ejected USB disk
   ```

<!--
1. Unplug the USB drive from the workstation.
 -->
1. 从工作站上拔下 USB 驱动器。

<!--
## 4. Enable EFI booting on the NUC {#enable-efi-booting}
 -->
## 4. 启用 NUC 迷你电脑上的 EFI 引导 {#enable-efi-booting}

<!--
Update your NUC's BIOS setup so that it can boot from
a USB drive.
 -->
请更新 NUC 迷你电脑的 BIOS 设置，以便其从 USB 驱动器启动。

<!--
To enable EFI (Extensible Firmware Interface) booting on your NUC,
do the following:
 -->
要启用 NUC 迷你电脑上的 EFI（Extensible Firmware Interface，可扩展固件接口）引导，请执行以下操作：

<!--
1. Reboot your NUC.
1. To enter the BIOS setup, press `F2` while booting.
1. In the **Boot Order** window on the left, click the **Legacy** tab.
1. Uncheck **Legacy Boot**.
 -->
1. 重新启动您的 NUC 迷你电脑。
1. 要进入 BIOS 设置，请在启动时按下 `F2`。
1. 在左侧的 **Boot Order**（启动顺序）窗口中，点击 **Legacy**（旧版）选项卡。
1. 取消勾选 **Legacy Boot**（旧版启动）。

   <img width="40%" src="/images/developing_on_nuc/bios.jpg"/>
<!--
1. Click the **Advanced** button.
1. Confirm the following boot configuration:
    * Under the **Boot Priority** tab:
       * **UEFI Boot** is checked.
    * Under the **Boot Configuration** tab:
       * In the **UEFI Boot** window:
         * **Boot USB Devices First** is checked.
         * **Boot Network Devices Last** is checked.
         * **Unlimited Network Boot Attempts** is checked.
       * In the **Boot Devices** window:
         * **USB** is checked.
         * **Network Boot** is set to **UEFI PXE & iSCSI**.
    * Under the **Secure Boot** tab:
       * **Secure Boot** is unchecked.
1. To save and exit BIOS, press `F10` and click **Yes**.
 -->
1. 点击 **Advanced**（高级）按钮。
1. 确认以下引导配置：
    * 在 **Boot Priority**（引导优先级）选项卡下：
       * **UEFI Boot**（UEFI 引导）已勾选。
    * 在 **Boot Configuration**（引导配置）选项卡下：
       * 在 **UEFI Boot**（UEFI 引导）窗口中：
         * **Boot USB Devices First**（首先使用 USB 设备引导）已勾选。
         * **Boot Network Devices Last**（最后使用网络设备引导）已勾选。
         * **Unlimited Network Boot Attempts**（网络引导尝试不限次数）已勾选。
       * 在 **Boot Devices**（引导设备）窗口中：
         * **USB** 已勾选。
         * **Network Boot**（网络引导）已设置为 **UEFI PXE & iSCSI**。
    * 在 **Secure Boot**（安全引导）选项卡下：
       * **Secure Boot**（安全引导）已取消勾选。
1. 要保存并退出 BIOS，请按下 `F10`，然后点击 **Yes**（是）。

<!--
## 5. Install Fuchsia on the NUC {#install-fuchsia}
 -->
## 5. 将 Fuchsia 安装至 NUC 迷你电脑 {#install-fuchsia}

<!--
Use the [bootable USB drive](#prepare-usb) to boot your NUC into
the Fuchsia installer. It then flashes the
[Workstation prebuilt image](#build-fuchsia) from your workstation
to the NUC to install Fuchsia for the first time.
 -->
请使用[可引导 USB 驱动器](#prepare-usb)将您的 NUC 迷你电脑引导到 Fuchsia 安装程序中。之后程序会将[预构建的工作站镜像](#build-fuchsia)从您的工作站刷入 NUC 迷你电脑，以进行 Fuchsia 的首次安装。

<!--
To install Fuchsia on your NUC, do the following:
 -->
要在您的 NUC 迷你电脑上安装 Fuchsia，请执行以下操作：

<!--
1. Plug the bootable USB drive into the NUC.
 -->
1. 将可引导 USB 驱动器插入您的 NUC 迷你电脑中。

<!--
1. Reboot your NUC.
 -->
1. 重新启动您的 NUC 迷你电脑。

<!--
   The NUC boots into the Fuchsia Workstation Installer (with a pink background).
 -->
   NUC 迷你电脑引导至 Fuchsia 工作站安装程序（背景为粉红色）。

<!--
1. Press **Enter** to select the `Install from USB` option.
 -->
1. 按下 **Enter**（回车）键选择 `Install from USB`（从 USB 安装）选项。

<!--
1. Press **Enter** on other prompts to continue.
 -->
1. 在出现其他提示时按下 **Enter**（回车）键继续。

<!--
   When the installation is finished, the screen displays
   `Success! Please restart your computer`.
 -->
   安装完成后，屏幕显示 `Success! Please restart your computer`（成功！请重新启动您的计算机）。

<!--
1. Unplug the USB drive from the NUC device.
 -->
1. 从 NUC 迷你电脑设备上拔下 USB 驱动器。

<!--
1. Reboot the NUC device.
 -->
1. 重新启动 NUC 迷你电脑设备。

<!--
   The NUC is now booted into Fuchsia’s Workstation.
 -->
   现在将 NUC 迷你电脑引导至 Fuchsia 工作站。

<!--
1. Set your login password to start the Fuchsia Workstation.
 -->
1. 设置登录密码以启动 Fuchsia 工作站。

<!--
Later, if you need to install a new version of Fuchsia (for instance, after
re-building a new Workstation image using `fx build`), see the
[Flash a new Fuchsia image to the NUC](#flash-fuchsia) section in Appendices.
 -->
之后，如果您需要安装新版本的 Fuchsia（例如，在使用 `fx build` 重新构建新的工作站镜像后），请参见附录中[将 Fuchsia 新镜像刷入 NUC 迷你电脑](#flash-fuchsia) 一节。

<!--
Important: If you plan on using this NUC device for Fuchsia development,
you must complete the steps in  the
[Flash a new Fuchsia image to the NUC](#flash-fuchsia) section at least once
after installing Fuchsia from a bootable USB drive. Running `fx flash` will
upload [Fuchsia-specific SSH keys][fuchsia-ssh-keys] to the NUC device, which
then enables other useful [`ffx` workflows][ffx-workflows].
 -->
重要提示：您如果打算将此 NUC 迷你电脑设备用于 Fuchsia 开发，则在通过可引导 USB 驱动器安装 Fuchsia 后，必须完成[将 Fuchsia 新镜像刷入 NUC 迷你电脑](#flash-fuchsia)一节至少一次。运行 `fx flash` 将向 NUC 迷你电脑设备上传[Fuchsia 特定的 SSH 键][Fuchsia-specific SSH keys]，这将启用其他有用的[`ffx` 工作流][ffx-workflows]。

<!--
## Appendices
 -->
## 附录

<!--
### Supported NUC models {#supported-nuc-models}
 -->
### 支持的 NUC 迷你电脑型号 {#supported-nuc-models}

<!--
For GPU support, get a NUC7 (Kaby Lake) or NUC8 (Coffee Lake), or a higher
generation.
 -->
为获得 GPU 支持，请使用 NUC 迷你电脑 7（Kaby Lake）、NUC 迷你电脑 8（Coffee Lake）或更新版本。

<!--
The list below shows some example models:
 -->
下面的列表展示了一些示例型号：

<!--
 * [Intel® NUC Kit NUC7i5DNKE][NUC7i5DNKE]{:.external}
 * [Intel® NUC Kit NUC7i5DNHE][NUC7i5DNHE]{:.external}
 * [Intel® NUC Kit NUC7i3DNKE][NUC7i3DNKE]{:.external}
 * [Intel® NUC Kit NUC7i3DNHE][NUC7i3DNHE]{:.external}
 * [Intel® NUC Kit NUC8i5BEK][NUC8i5BEK]{:.external}
 * [Intel® NUC Kit NUC8i5BEH][NUC8i5BEH]{:.external}
 * [Intel® NUC Kit NUC8i3BEK][NUC8i3BEK]{:.external}
 * [Intel® NUC Kit NUC8i3BEH][NUC8i3BEH]{:.external}
 -->
 * [Intel® NUC 套件 NUC7i5DNKE][NUC7i5DNKE]{:.external}
 * [Intel® NUC 套件 NUC7i5DNHE][NUC7i5DNHE]{:.external}
 * [Intel® NUC 套件 NUC7i3DNKE][NUC7i3DNKE]{:.external}
 * [Intel® NUC 套件 NUC7i3DNHE][NUC7i3DNHE]{:.external}
 * [Intel® NUC 套件 NUC8i5BEK][NUC8i5BEK]{:.external}
 * [Intel® NUC 套件 NUC8i5BEH][NUC8i5BEH]{:.external}
 * [Intel® NUC 套件 NUC8i3BEK][NUC8i3BEK]{:.external}
 * [Intel® NUC 套件 NUC8i3BEH][NUC8i3BEH]{:.external}

<!--
### Flash a new Fuchsia image to the NUC {#flash-fuchsia}
 -->
### 将 Fuchsia 新镜像刷入 NUC 迷你电脑 {#flash-fuchsia}

<!--
Once a NUC is running Fuchsia, you can use Fuchsia's flashing
mechanism to provision a new Fuchsia image to the NUC.
 -->
NUC 迷你电脑运行 Fuchsia 之后，您就可以使用 Fuchsia 的刷入机制（flashing mechanism）来为 NUC 迷你电脑提供 Fuchsia 新镜像了。

<!--
To flash a Fuchsia image to your NUC, do the following:
 -->
要将 Fuchsia 镜像刷入您的 NUC 迷你电脑，请执行以下操作：

<!--
1. Connect the NUC directly to the workstation using an Ethernet cable
   (or connect the NUC to a router or WiFi modem in the same
   Local Area Network as the workstation).
 -->
1. 使用以太网线缆将 NUC 迷你电脑直接连接到工作站（或将 NUC 迷你电脑连接到与工作站相同本地网络下的路由器或 WiFi 调制解调器）。

<!--
   Note: Network booting only works with the NUC's built-in Ethernet port –
   netbooting with an USB port (via an Ethernet-to-USB adapter) is not supported.
 -->
   注意：网络引导仅适用于 NUC 迷你电脑的内置以太网端口——（通过以太网转 USB 适配器）使用 USB 端口进行网络引导是不支持的。

<!--
1. Reboot your NUC.
 -->
1. 重新启动您的 NUC 迷你电脑。

<!--
1. To boot the NUC into Fastboot mode, press the `f` key at the Fuchsia boot screen.
 -->
1. 要将 NUC 迷你电脑引导至 Fastboot 模式，请在 Fuchsia 引导屏幕下按 `f` 键。

<!--
   Once the NUC is in Fastboot mode, you can see `entering fastboot mode` printed on the
   screen.
 -->
   当 NUC 迷你电脑处于 Fastboot 模式时，您可以看到屏幕上输出的 `entering fastboot mode`（正在进入 fastboot 模式）。

<!--
1. **On your workstation**, detect the NUC in Fastboot mode:
 -->
1. **在您的工作站上**，检测 NUC 迷你电脑处于 Fastboot 模式：

   ```posix-terminal
   ffx target list
   ```

<!--
   This command prints output similar to the following:
 -->
该命令打印的输出形如：

   ```none {:.devsite-disable-click-to-copy}
   $ ffx target list
   NAME                      SERIAL       TYPE       STATE       ADDRS/IP                           RCS
   fuchsia-54b2-0389-644b    <unknown>    Unknown    Fastboot    [fe81::55b1:2ff:fe34:567b%en10]    N
   ```

<!--
   Verify that the device's state is in `Fastboot`.
 -->
验证设备状态处于 `Fastboot`。

<!--
1. Flash a new Fuchsia image to the NUC:
 -->
1. 将 Fuchsia 新镜像刷入 NUC 迷你电脑：

<!--
   Note: To build a new Fuchsia image, see the [Build Fuchsia](#build-fuchsia) section above.
 -->
   注意：要构建 Fuchsia 新镜像，请参阅上述[构建 Fuchsia](#build-fuchsia) 一节。

   ```posix-terminal
   fx flash
   ```

<!--
   When finished, the NUC reboots and starts running the new Fuchsia image.
 -->
   完成后，NUC 迷你电脑重新启动并开始运行 Fuchsia 新镜像。

<!--
   Important: When using this NUC device for Fuchsia development, currently
   for other [`ffx` workflows][ffx-workflows], you can only use USB ports to
   connect the NUC to your host machine. In other words, undo the cable setup in
   Step 1 above, and use **2 Ethernet-to-USB adapters** and an Ethernet cable to
   establish a connection between the NUC and your host machine using only USB ports.
 -->
   重要提示：在使用该 NUC 迷你电脑设备进行 Fuchsia 开发时，对于其他[`ffx`工作流][ffx-workflows]，您目前只能使用 USB 端口将 NUC 迷你电脑连接到您的主机。亦即，不进行步骤 1 中的线缆设置，而是使用**两个以太网转 USB 适配器**和一条以太网线缆，在仅使用 USB 端口的情况下，建立 NUC 迷你电脑和您的主机之间的连接。


<!-- Reference links -->

[nuc-wiki]: https://en.wikipedia.org/wiki/Next_Unit_of_Computing
[get-started-with-fuchsia]: /get-started/README.md
[usb-setup]: /development/hardware/usb_setup.md
[supported-sys-config]: /reference/hardware/support-system-config.md
[NUC7i5DNKE]: https://ark.intel.com/content/www/us/en/ark/products/122486/intel-nuc-kit-nuc7i5dnke.html
[NUC7i5DNHE]: https://ark.intel.com/content/www/us/en/ark/products/122488/intel-nuc-kit-nuc7i5dnhe.html
[NUC7i3DNKE]: https://ark.intel.com/content/www/us/en/ark/products/122495/intel-nuc-kit-nuc7i3dnke.html
[NUC7i3DNHE]: https://ark.intel.com/content/www/us/en/ark/products/122498/intel-nuc-kit-nuc7i3dnhe.html
[NUC8i5BEK]: https://ark.intel.com/content/www/us/en/ark/products/126147/intel-nuc-kit-nuc8i5bek.html
[NUC8i5BEH]: https://ark.intel.com/content/www/us/en/ark/products/126148/intel-nuc-kit-nuc8i5beh.html
[NUC8i3BEK]: https://ark.intel.com/content/www/us/en/ark/products/126149/intel-nuc-kit-nuc8i3bek.html
[NUC8i3BEH]: https://ark.intel.com/content/www/us/en/ark/products/126150/intel-nuc-kit-nuc8i3beh.html
[ffx]: https://fuchsia.dev/reference/tools/sdk/ffx
[ffx-workflows]: /development/sdk/ffx/index.md
[fuchsia-ssh-keys]: /development/sdk/ffx/create-ssh-keys-for-devices.md
