<!-- # Developing with Fuchsia on a NUC -->


# 在 NUC 上开发 Fuchsia

This document describes how to get a NUC up and running with Fuchsia.

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
| SSD (Only need one, | [B01IAGSDJ0](https://www.amazon.com/gp/product/B01IAGSDJ0) | Works fine. |
| I bought some of each) | [B00TGIVZTW](https://www.amazon.com/gp/product/B00TGIVZTW) | Works fine. |
| | [B01M9K0N8I](https://www.amazon.com/gp/product/B01M9K0N8I) | Works fine. |
| | | |
| **Optional:** | | |
| Keyboard and Mouse | [B00B7GV802](https://www.amazon.com/gp/product/B00B7GV802) | Works fine.  Next time I'd get a keyboard with a smaller foot print. |
| Monitor | [B015WCV70W](https://www.amazon.com/gp/product/B015WCV70W) | Works fine. |
| HDMI Cable | [B014I8SIJY](https://www.amazon.com/gp/product/B014I8SIJY) | Works fine. |
| USB 3.0 drive | [B01BGTG41W](https://www.amazon.com/gp/product/B01BGTG41W) | Works fine. |

-----

## 2. Prepare the NUC <a name="nuc"/>
NUCs don’t come with RAM or an SSD so you need to install them.
<br/><center><img width="50%" src="images/developing_on_nuc/parts.jpg"/></center><br/>

1. Remove the phillips screws in the bottom feet of the NUC.
<br/><center><img width="50%" src="images/developing_on_nuc/nuc_bottom.jpg"/></center>
<br/><center><img width="50%" src="images/developing_on_nuc/nuc_inside.jpg"/></center><br/><br/>
1. Install the RAM.
1. Remove the phillips screw that will hold the SSD in place (phillips screwdriver with magnetic tip is useful here).
1. Install the SSD.
1. Screw the SSD in place using screw from 3.
<br/><center><img width="50%" src="images/developing_on_nuc/parts_installed.jpg"/></center><br/><br/>
1. Replace bottom and screw feet back in.
1.(Optional) Apply fuchsia logo.
<br/><center><img width="50%" src="images/developing_on_nuc/nuc_fuchsia.jpg"/></center><br/><br/>
1. Plug power, ethernet, HDMI, keyboard, and mouse into NUC.

-----

## 3. Build Fuchsia <a name="build"/>

1. Follow the [getting started guidelines](../../getting_started.md)
1. Plug in your USB key to your build workstation
1. Identify the path to your USB key by running `fx list-usb-disks`
1. Create a Zedboot USB by running `fx mkzedboot /path/to/usb/disk`
1. Plug the Zedboot USB key into the NUC and boot it
1. Run `fx pave` on your workstation

## 4. Install Fuchsia <a name="install"/>

1. Plug in your installable fuchsia usb drive into NUC.
1. Turn on NUC.
1. Wait for NUC to boot into fuchsia.
1. Alt-tab to a terminal if you don’t boot into a terminal.
<br/><center><img width="50%" src="images/developing_on_nuc/terminal.jpg"/></center><br/><br/>
1. Run ‘lsblk’.  This should say there’s a ‘block’ device at 003.
<br/><center><img width="50%" src="images/developing_on_nuc/lsblk.jpg"/></center><br/><br/>
1. Run ‘gpt init /dev/class/block/003’.  Say ‘y’ to the warning.
1. Run ‘install-fuchsia’.
1. Run ‘dm reboot’.
1. Remove usb drive.

At this point the NUC should boot to fuchsia without the usb drive.  It’s using the internal SSD.  But it won’t work with netbooting.  Let’s fix that.

-----

## 5. Update NUC BIOS to allow netbooting <a name="bios"/>

1. Reboot NUC.
1. Press F2 while booting to enter BIOS.
1. In the Boot Order window on the left click the Legacy tab.
1. Uncheck ‘Legacy Boot’.
<br/><center><img width="50%" src="images/developing_on_nuc/bios.jpg"/></center><br/><br/>
1. Press the X in the top right to leave the BIOS.  Ensure you save before exiting.

-----


All done!
