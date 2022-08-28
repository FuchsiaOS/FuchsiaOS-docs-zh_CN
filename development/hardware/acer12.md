<!--
# Acer Switch Alpha 12
 -->
# 宏碁 Switch Alpha 12

<!--
WARNING:  These are directions to configure the machine and boot an experimental, in-development OS on it.
 -->
警告：这些说明是用于配置本机器并在其上启动一个实验性、开发中的操作系统。

<!--
## Powering the Machine On
 -->
## 打开机器电源
<!--
To power on you must hold the power button (lefthand side, above the Volume rocker) for several seconds, then let go.  You can safely let go when the tiny blue light on the power button itself turns on (yes, this is really hard to see when you’re holding the power button), or when the display backlight turns on.  If you hold too long it may end up power right back off again.
 -->
您必须按住电源按钮（左侧，音量摇杆上方）几秒钟，然后松开，从而启动。电源按钮本身上的微小蓝灯亮起时（是的，当您按住电源按钮时很难看到这个灯），或者当显示屏背光亮起时，您可以放心地松开。如果您按得太久，它可能会再次关闭电源。

<!--
## Powering the Machine Off
 -->
## 关闭机器电源
<!--
If you boot into Windows 10 or something hangs or crashes and you need to power things off, Press and Hold the power button until the display shuts off.  To be sure, hold for about 10 seconds total.
 -->
如果您启动到了 Windows 10，或者某些东西挂起或崩溃了并且您需要关闭电源，请按住电源按钮直到显示屏关闭。请确保总共按住大约 10 秒钟。

<!--
## Entering the BIOS
 -->
## 进入 BIOS
<!--
With the machine off, Press and hold Volume Up, then continue to hold while pressing and holding the Power button.  Let go of the Power button when the display backlight turns on.  Alternatively, hold F2 on an attached keyboard while powering the machine on.
 -->
在机器关闭的情况下，按住音量增加键并保持住，同时按住电源按钮并保持住。当显示屏背光打开时松开电源按钮。另一种方法是，在打开机器电源时按住外接键盘上的 F2 键。

<!--
## Enabling Zircon Boot
 -->
## 启用 Zircon 引导
 <!--
1. Boot the machine and enter the BIOS
 -->
1. 启动机器并进入 BIOS（基本输入输出系统）
<!--
2. Select “Security” from the tabs at the left
 -->
2. 从左侧的选项卡中选择“Security（安全）”
<!--
3. Tap the “[clean]” gray bar under “Supervisor Password Is”
 -->
3. 点击“Supervisor Password Is（管理员密码是）”下的“[clean]”灰色条
<!--
4. Enter a supervisor password, enter it again, press OK
 -->
4. 输入管理员密码，并再次输入，然后按OK
<!--
5. Select “Boot” from the tabs at the left
 -->
5. 从左侧的选项卡中选择“Boot（启动）”
<!--
6. Tap the “[Enabled]” gray bar under “Secure Boot”
    (if there’s no gray bar, you have not set a supervisor password, go back and do that now)
 -->
6. 点击“Secure Boot（安全启动）”下的“[Enabled]”灰色条（如果没有灰色条，则表示您尚未设置管理员密码，请返回并立即设置）
<!--
7. Select “Disabled” from the menu
 -->
7. 从菜单中选择“Disabled（已禁用）”
<!--
8. The “Boot priority order” list may be adjusted using the up/down arrows to the right of each item
 -->
8. 可以在每项右侧使用向上/向下箭头调整“Boot priority order（启动优先级顺序）”列表
<!--
9. Order the list like so:
 -->
9. 排列该列表成如下：
   - USB HDD
   - USB FDD
   - USB CDROM
   - HDD: \<MFG\> \<SERIALNO\>
   - Network Boot-IPV4
   - Network Boot-IPV6
   - Windows Boot Manager
<!--
10. Select the “Main” tab on the left and set the time and date by pressing “[SetTime]” and “[SetDate]” buttons respectfully. This is necessary for proper network operation.
 -->
10. 选择左侧的“Main（主菜单）”选项卡，分别按“[SetTime]”和“[SetDate]”按钮设置时间​​和日期。这是想要正常使用网络所必需的操作。
<!--
11. (Optional)  Go back to the “Security” tab and set the supervisor password back to nothing.
Otherwise you’ll need to enter the password every time you use the BIOS.
A password is required to modify the secure boot setting, but “disabled” will persist without one.
 -->
11. （可选）返回“Security（安全）”选项卡并将管理员密码设置为空。否则每次使用 BIOS 时都需要输入密码。修改安全启动设置时需要密码，在没有密码时会持续显示“disabled（已禁用）”。
<!--
12. Select “Exit” from the tabs at the left
 -->
12. 从左侧的选项卡中选择“Exit（退出）”
<!--
13. Select “Exit Saving Changes”
 -->
13. 选择“Exit Saving Changes（退出并保存更改）”
<!--
14. Continue to [Setup with USB flash drive](usb_setup.md)
 -->
14. 继续[使用 USB 闪存驱动器进行设置](usb_setup.md)

<!--
## What if you end up in the Windows 10 Setup?
 -->
## 如果您最终进入 Windows 10 设置怎么办？
<!--
If you don’t enter the BIOS and haven’t installed another OS, You’ll end up on a blue background “Hi there” screen asking you to select country, language, etc.
 -->
如果您没有进入 BIOS 并且没有安装其他操作系统，您最终会出现蓝色背景的“Hi there”屏幕，要求您选择国家、语言等。

<!--
1. Press Power and Hold it for about 10 seconds (the screen will turn off after 2-3 seconds).
 -->
1. 请按住电源约 10 秒（屏幕将在 2-3 秒后关闭）。
<!--
2. Boot into the BIOS as described above.
 -->
2. 然后按上所述引导进入 BIOS。

<!--
## What if you get stuck in Windows 10 Recovery?
 -->
## 如果您卡在 Windows 10 恢复中怎么办？
<!--
It’s possible to end up in a situation where the machine *really* wants to help you recover your failed boots into Windows 10 and dumps you into a recovery screen -- blue background, “Recovery” in the upper left, and some text saying “It looks like Windows didn’t load correctly”.
 -->
有可能最终出现这样的情况：机器真的想帮助您恢复失败的启动进入到Windows 10，并将您跳入一个恢复屏幕--蓝色背景，"Recovery"显示在屏幕左上方，还有一些文字说 "It looks like Windows didn’t load correctly"。

<!--
1. Select “See advanced repair options”
 -->
1. 选择“See advanced repair options（查看高级修复选项）”
<!--
2. Select “Troubleshoot” (screwdriver and wrench icon)
 -->
2. 选择“Troubleshoot（疑难解答）”（螺丝刀和扳手图标）
<!--
3. Select “Advanced options” (checkmarks and lines icon)
 -->
3. 选择“Advanced options（高级选项）”（复选标记和线条图标）
<!--
4. Select “UEFI Firmware Settings” (integrated circuit and gear icon)
 -->
4. 选择“UEFI Firmware Settings（UEFI固件设置）”（集成电路和齿轮图标）
<!--
5. When prompted “Restart to change UEFI firmware settings”, select “Restart”
 -->
5. 当提示“Restart to change UEFI firmware settings（重新启动以更改 UEFI 固件设置）”时，选择“Restart（重新启动）”
<!--
6. The machine should now reboot into the BIOS
 -->
6. 机器现在应该重新启动进入 BIOS
<!--
7. Check that “Windows Boot Manager” didn’t get moved to the top of the boot order, fix it if it did
 -->
7. 检查“Windows Boot Manager（Windows 启动管理器）”没有被移到启动顺序的顶部，如果在顶部，请修改掉

<!--
## Quirks
 -->
## 怪症
<!--
It has been observed that USB initialization is racy on a cold boot.  So if you're starting from a cold boot and trying to boot to USB, you may find that you boot to disk instead.
 -->
据观察，USB 初始化在冷启动时很竞争。因此，如果您从冷启动开始并尝试启动到 USB，您可能会发现您启动的是磁盘。

<!--
Mitigations:
 -->
缓解措施：

<!--
- It's useful to use a `cmdline` file to set `zircon.nodename=foo` to know during the boot screen whether you're booting from USB or disk.
 -->
- 一个很有用的技巧，使用 `cmdline` 文件设置 `zircon.nodename=foo` 以在启动屏幕期间了解您是从 USB 启动还是从磁盘启动。
<!--
- If the Acer is booting from disk and you want to boot from USB, remove and reinsert the USB drive, then reboot with `ctrl-alt-del` (not the power button.)
 -->
- 如果宏碁从磁盘启动但您想从 USB 启动，请移除并重新插入 USB 驱动器，然后使用 `ctrl-alt-del`（不是电源按钮）重新启动。
<!--
- You can tell from the bios whether USB has been initialized because it will name the USB device.
 -->
- 您可以从 bios 中判断 USB 是否已初始化，因为它会命名 USB 设备。
