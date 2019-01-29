<!-- # Developing with Fuchsia on a NUC -->


# 在 NUC 上开发 Fuchsia

<!--This document describes how to get a NUC up and running with Fuchsia.-->
本文档介绍如何使用 NUC 启动和运行 Fuchsia。

<!--[1. Get Parts](#parts)<br/>-->
[1. 需要的硬件](#parts)<br/>
<!--[2. Prepare the NUC](#nuc)<br/>-->
[2. 准备 NUC](#nuc)<br/>
<!--[3. Install Fuchsia](#install)<br/>-->
[3. 安装 Fuchsia](#install)<br/>
<!--[4. Update NUC BIOS to allow netbooting](#bios)<br/>-->
[4. 更新 NUC 的 BIOS 允许网络引导](#bios)<br/>

-----

<!--## 1. Get Parts <a name="parts"/>-->
## 1. 需要的硬件 <a name="parts"/>

<!--You’ll need the following:-->
你将会需要以下硬件：

<!--- USB 3.0 Drive-->
<!--
	- Keyboard
	- Mouse
	- Monitor that supports HDMI
	- HDMI cable
	- ethernet cable
	- Magnetic tip phillips head screwdriver.
-->
- USB 3.0 驱动器
- NUC
- RAM
- m.2 SSD
- 键盘
- 鼠标
- 支持HDMI的显示器
- HDMI连接线
- 网线
- 磁头十字螺丝刀


<!--This table shows what I bought from Amazon.-->
下表是我从亚马逊上买到的设备

<!--| Item | Link | Notes: |-->

| 设备 | 链接 | 备注 |
| ---- | ---- | ------ |
| NUC | [B01MSZLO9P](https://www.amazon.com/gp/product/B01MSZLO9P) | <!--Get a NUC7 or NUC6 for gpu support.--> 需要 NUC6 或者 NUC7 支持 GPU |
| RAM | [B01BIWKP58](https://www.amazon.com/gp/product/B01BIWKP58) | <!--Works fine.--> 工作正常 |
| SSD <!--(Only need one,--> （ 只需要一个， | [B01IAGSDJ0](https://www.amazon.com/gp/product/B01IAGSDJ0) | <!--Works fine.--> 工作正常 |
| <!--I bought some of each)--> 我买了很多个） | [B00TGIVZTW](https://www.amazon.com/gp/product/B00TGIVZTW) | <!--Works fine.--> 工作正常 |
| | [B01M9K0N8I](https://www.amazon.com/gp/product/B01M9K0N8I) | <!--Works fine.--> 工作正常 |
| | | |
| <!--**Optional:**--> **可选** | | |
| <!--Keyboard and Mouse--> 键盘、鼠标 | [B00B7GV802](https://www.amazon.com/gp/product/B00B7GV802) | <!--Works fine.  Next time I'd get a keyboard with a smaller foot print.-->工作正常。下次我要买个小巧的键盘。 |
| <!--Monitor--> 显示器 | [B015WCV70W](https://www.amazon.com/gp/product/B015WCV70W) | <!--Works fine.--> 工作正常 |
| <!--HDMI Cable--> HDMI连接线 | [B014I8SIJY](https://www.amazon.com/gp/product/B014I8SIJY) | <!--Works fine.--> 工作正常 |
| <!--USB 3.0 drive--> USB 3.0 驱动器 | [B01BGTG41W](https://www.amazon.com/gp/product/B01BGTG41W) | <!--Works fine.--> 工作正常 |

-----

<!--## 2. Prepare the NUC <a name="nuc"/>-->
## 2. 准备 NUC <a name="nuc"/>
<!--NUCs don’t come with RAM or an SSD so you need to install them.-->
因为 NUCs 没有自带 RAM 和 SSD ，需要自己安装。
<br/><center><img width="50%" src="images/developing_on_nuc/parts.jpg"/></center><br/>
<!--1. Remove the phillips screws in the bottom feet of the NUC.-->
<!--1. Install the RAM.-->
<!--1. Remove the phillips screw that will hold the SSD in place (phillips screwdriver with magnetic tip is useful here).-->
<!--1. Install the SSD.-->
<!--1. Screw the SSD in place using screw from 3.-->
<!--1. Replace bottom and screw feet back in.1.(Optional) Apply fuchsia logo.-->
<!--1. Plug power, ethernet, HDMI, keyboard, and mouse into NUC.-->
1. 取下 NUC 底部的十字螺丝。
<br/><center><img width="50%" src="images/developing_on_nuc/nuc_bottom.jpg"/></center>
<br/><center><img width="50%" src="images/developing_on_nuc/nuc_inside.jpg"/></center><br/><br/>
1. 安装 RAM。
1. 卸下将SSD固定到位的十字螺丝（带磁性的十字螺丝刀在这里很有用）。
1. 安装 SSD。
1. 使用步骤 3 卸下的十字螺丝将SSD固定到位。
<br/><center><img width="50%" src="images/developing_on_nuc/parts_installed.jpg"/></center><br/><br/>
1. 更换底部，并用按照步骤 1 将螺丝固定回去（可选）喷涂 Fuchsia 的 LOGO。
<br/><center><img width="50%" src="images/developing_on_nuc/nuc_fuchsia.jpg"/></center><br/><br/>
1. 将电源，以太网，HDMI，键盘和鼠标接入NUC。

-----

<!--## 3. Build Fuchsia <a name="build"/>-->
## 3. 构建 Fuchsia <a name="build"/>

<!--1. Follow the [getting started guidelines](../../getting_started.md)-->
<!--1. Plug in your USB key to your build workstation-->
<!--1. Identify the path to your USB key by running `fx list-usb-disks`-->
<!--1. Create a Zedboot USB by running `fx mkzedboot /path/to/usb/disk`-->
<!--1. Plug the Zedboot USB key into the NUC and boot it-->
<!--1. Run `fx pave` on your workstation-->
1. 遵循[入门指南](../../getting_started.md)
1. 在你的构建工作站插入 UBS 密钥。
1. 通过运行 `fx list-usb-disks` 来确定 USB 密钥的的路径。
1. 通过运行 `fx mkzedboot /path/to/usb/disk` 来创建一个 Zedboot USB
1. 将 Zedboot USB 密钥插入 NUC 然后启动。
1. 在你的工作站运行 `fx pave`。

<!--## 4. Install Fuchsia <a name="install"/>-->
## 4. 安装 Fuchsia <a name="install"/>

<!--1. Plug in your installable fuchsia usb drive into NUC.-->
<!--1. Turn on NUC.-->
<!--1. Wait for NUC to boot into fuchsia.-->
<!--1. Alt-tab to a terminal if you don’t boot into a terminal.-->
<!--1. Run ‘lsblk’.  This should say there’s a ‘block’ device at 003.-->
<!--1. Run ‘gpt init /dev/class/block/003’.  Say ‘y’ to the warning.-->
<!--1. Run ‘install-fuchsia’.-->
<!--1. Run ‘dm reboot’.-->
<!--1. Remove usb drive.-->
1. 将 Fuchsia 的安装 USB 驱动器插入 NUC。
1. 开启 NUC。
1. 等待 NUC 启动进入 Fuchsia。
1. 如果启动起来没有进入终端，按 Alt-tab 可以打开终端。
<br/><center><img width="50%" src="images/developing_on_nuc/terminal.jpg"/></center><br/><br/>
1. 运行 `lsblk`。这里应该说明在 003 有一个“块”设备
<br/><center><img width="50%" src="images/developing_on_nuc/lsblk.jpg"/></center><br/><br/>
1. 运行 `gpt init /dev/class/block/003` 。警告输入“y”。
1. 运行 `install-fuchsia`。
1. 运行 `dm reboot`。
1. 移除 USB 驱动器。

<!--At this point the NUC should boot to fuchsia without the usb drive.  It’s using the internal SSD.  But it won’t work with netbooting.  Let’s fix that.-->
此时，NUC应在不使用USB驱动器的情况下可以引导至 Fuchsia。它使用的是内部的 SSD。但它不适用于网络启动。我们来解决这个问题。

-----

<!--## 5. Update NUC BIOS to allow netbooting <a name="bios"/>-->
## 5. 更新 NUC BIOS 用来支持网络启动 <a name="bios"/>

<!--1. Reboot NUC.-->
<!--1. Press F2 while booting to enter BIOS.-->
<!--1. In the Boot Order window on the left click the Legacy tab.-->
<!--1. Uncheck ‘Legacy Boot’.-->
<!--1. Press the X in the top right to leave the BIOS.  Ensure you save before exiting.-->
1. 重启 NUC。
1. 当启动时按 F2 进入 BIOS。
1. 在左侧的“Boot Order” 选项中，点击 “Legacy” 选项。
1. 取消选中 “Legacy Boot”。
<br/><center><img width="50%" src="images/developing_on_nuc/bios.jpg"/></center><br/><br/>
1. 点击右上角的叉关闭 BIOS。确保关闭之前已经保存。

-----


<!--All done!-->
完成！
