<!--# Preparing to install Fuchsia on Pixelbook-->
# 在 Pixelbook 上安装 Fuchsia 的准备工作
<!--## Update ChromeOS-->
## 更新 ChromeOS

<!--If your Pixelbook has never been booted, it is best to boot it normally to check
for any critical updates, as follows:-->
如果你的 Pixelbook 从来没有被启动过, 那么最好的方式就是正常启动检查所有关键的更新，方法如下：

<!--1. Boot the Pixelbook normally. Opening the lid usually powers on the device.
If this doesn't work, the power button is on the left side of the device, near
the front of the wrist rest.-->
1. 正常启动 Pixelbook， 通常翻开盖子就会给 Pixelbook 供电
如果没有启动， 电源键就在设备的左侧，靠近腕托前面。
<!--2. Tap the "Let's go" button.-->
2. 点击 “Let's go” 按钮。
<!--3. Connect to a wired or wireless network.-->
3. 连接到有线或无线网络。
<!--4. Accept the terms to proceed to the update check step.-->
4. 接受条款以继续进行更新检查步骤。
<!--5. The device should check for updates, install any found.-->
5. 设备应该会检查更新，并安装所有更新。
<!--6. After rebooting from any updates, tap 'Browse as Guest' in the lower left
corner.-->
6. 安装所有更新后重启设备，点击左下角的 “Browse as Guest（以访客身份访问）”。
<!--7. From the browser UI, go into “Settings->About Chrome OS” or “Help->About Chrome
OS” and confirm the version is &gt;=62.-->
7. 在 ChromeOS 的主界面，点击 “Settings->About Chrome OS（设置->关于 Chrome OS）” 或 “Help->About Chrome
OS（帮助->关于 Chrom OS）” 确定版本&gt;=62

<!--## Put your device into developer mode-->
## 设置开发者模式

<!--***WARNING: This will erase any state stored locally on your Pixelbook***
-->***注意: 这个操作会清除本地所有的设置***

<!--1. Power off the Pixelbook.-->
1. 关闭 Pixelbook 的电源。
<!--2. Go into Recovery Mode.
Hold down Esc+Refresh (first and third buttons on the top row of the keyboard).
Then press the Power button (bottom left side of the device).-->
2. 进入 Recovery 模式。
按住 Esc+Refresh (键盘上第一行的第一个和第三个按钮)
然后再按电源键 (设备左侧的按钮)。
<!--3. Start by disabling OS verification by pressing Ctrl+D. You should see "To turn OS verification OFF, press ENTER". Press Enter to confirm.-->
3. 首先按 Ctrl+D 关闭系统验证。你会看到 “To turn OS verification OFF, press ENTER（关闭系统验证，请按 ENTER ）”。回车确认选择。
<!--4. When your device reboots, you'll get confirmation that OS verification is OFF. Press Ctrl+D again to enter Developer Mode.-->
4. 如果 Pixelbook 重启，可以确定系统验证已经被关闭。再次按 Ctrl+D 进入开发者模式。
<!--5. Wait for the device to re-configure itself, which will take several minutes.
Initially it may not appear to be doing anything. Let the device sit for a
minute or two. You will hear two loud &lt;BEEP&gt;s early in the process. The
process is complete when you hear two more loud &lt;BEEP&gt;s.-->
5. 等待设备自己配置，这将会花费几分钟时间。最初设备好像什么都没有做。静置设备一两分钟。配置开始会听到两声哔哔声。当适配完成会发出两声更加响亮的哔哔声。
<!--6. The device should reboot itself when the Developer Mode transition is
complete. You can now jump to Step #2 in the "Boot from USB" section.-->
6. 开发者模式配置完成，设备会重起。可以进入第二步“从 USB 启动”。

<!--## Boot from USB
-->
## 从 USB 启动

<!--1. Boot into ChromeOS.-->
1. 进入 ChromeOS。
<!--2. You should see a screen that says "OS verification is OFF" and approximately
30 seconds later the boot will continue. Wait for the Welcome or Login screen
to load. **Ignore** any link for "Enable debugging features".-->
2. 你会看到屏幕显示 "OS verification is OFF（系统验证已经关闭）"然后大约 30 秒会继续加载系统。等欢迎界面或者登陆页面加载。 **忽略** 所有的 “Enable debugging features（启用调试功能）” 的链接。
<!--3. Press Ctrl+Alt+Refresh/F3 to enter a command shell. If pressing this key
combination has no effect, try rebooting the Pixelbook once more.-->
3. 按 Ctrl+Alt+Refresh/F3 打开命令行窗口。 如果按下组合键没有效果，重启一次之后再尝试。
<!--4. Enter 'chronos' as the user with a blank password-->
4. 输入 'chronos' 作为用户名，密码为空。
<!--5. Enable USB booting by running `sudo crossystem dev_boot_usb=1`-->
5. 通过运行 `sudo crossystem dev_boot_usb=1` 启动 USB。
<!--6. (optional) Default to USB booting by running `sudo crossystem dev_default_boot=usb`.-->
6. (可选) 运行 `sudo crossystem dev_default_boot=usb` 设置默认 USB 启动。
<!--7. Plug the USB drive into the Pixelbook.-->
7. 插入 USB 驱动器。
<!--8. Reboot by typing `sudo reboot`-->
8. 运行 `sudo reboot` 重启设备。
<!--9. On the "OS verification is OFF" screen press Ctrl+U to bypass the timeout and
boot from USB immediately. (See [Tips and Tricks](#tips-and-tricks) for other
short circuit options)-->
9. 在 “OS verification is OFF（系统验证已经关闭）” 页面，按 Ctrl+U 可以立即进入 USB 启动。 ( [提示 & 技巧](#提示--技巧) 查看其他简单操作)

<!--The USB drive is only needed for booting when you want to re-pave or otherwise
netboot the device. If you didn't make USB booting the default (Step #6), you
will need to press Ctrl+U at the grey 'warning OS-not verified' screen to boot
from USB when you power on your device. If the device tries to boot from USB,
either because that is the default or you pressed Ctrl+U, and the device fails
to boot from USB you'll hear a fairly loud &lt;BEEP&gt;. Note that ChromeOS
bootloader USB enumeration during boot has been observed to be slow. If you're
having trouble booting from USB, it may be helpful to remove other USB devices
until the device is through the bootloader and also avoid using a USB hub.-->
只有你想重新安装或网络启动设备时才需要 USB 驱动器。如果你没有设置 USB 默认启动（第 6 步）， 你需要在设备开启电源后，在界面显示 “warning OS-not verified（警告系统没有验证）” 时按 Ctrl+U。如果设备默认 USB 启动或者你按了 Ctrl+U ，那么设备会尝试从 USB 启动，如果启动失败，设备会发出很大得哔哔声。注意，ChromeOS 引导程序在引导期间的 USB 总线枚举过程被观察到很慢。如果你 USB 启动遇到困难，你可以尝试移除其他所有的 USB 驱动设备并且不使用 USB 集线器连接 USB 驱动器，直到通过引导程序。

<!--## Tips and Tricks-->
## 提示 & 技巧

<!--By default the ChromeOS bootloader has a long timeout to allow you to press
buttons. To shortcut this you can press Ctrl+D or Ctrl+U when on the grey screen
that warns that the OS will not be verified. Ctrl+D will cause the device to
skip the timeout and boot from its default source. Ctrl+U will skip the timeout
and boot the device from USB.
-->
一般情况下，ChromeOS 引导加载程序有很长的超时时间允许您按下按钮。 再灰色的界面上按快捷键 Ctrl+D 或 CTRL+U 可以跳过系统不会被验证的警告。Ctrl+D 可以跳过等待时间然后从默认资源启动。Ctrl+U 可以跳过等待时间然后从 USB 启动。

<!--### Going back to ChromeOS-->
### 设置启动 ChromeOS

<!--To go back to ChromeOS you must modify the priority of the Fuchsia kernel
partition to be lower than that of at least one of the two ChromeOS kernel
partitions.
-->
安装 Fuchsia 后要启动 Chromeos，必须将 Fuchsia 内核分区的优先级修改为低于两个 Chromeos 内核分区中的一个。

<!--1. Press Alt+Esc to get to a virtual console-->
1. 按 Alt+Esc 打开虚拟控制台
<!--2. Find the disk that contains the KERN-A, KERN-B, and KERN-C partitions with
the `lsblk` command. Below this is device 000, note how the device path of the
kernel partitions is an extension of that device.-->
2. 使用 `lsblk` 命令找到包含 KERN-A， KERN-B 和 KERN-C 的磁盘分区。下面是设备 000 的示例，注意，内核分区的路径是该设备的扩展。

        $ lsblk
        ID  SIZE TYPE             LABEL                FLAGS  DEVICE
        000 232G                                              /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block
        001   5G data             STATE                       /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-000/block
        002  16M cros kernel      KERN-A                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-001/block
        003   4G cros rootfs      ROOT-A                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-002/block
        004  16M cros kernel      KERN-B                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-003/block
        005   4G cros rootfs      ROOT-B                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-004/block
        006  64M cros kernel      KERN-C                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-005/block
        007   4G cros rootfs      ROOT-C                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-006/block
<!--3. Use the `gpt` command to look at the device's (eg. 000) partition map.-->
3. 使用 `gpt` 命令查看设备（例如：000）的分区表。

        $ gpt dump /dev/class/block/000
        blocksize=0x200 blocks=488554496
        Partition table is valid
        GPT contains usable blocks from 34 to 488554462 (inclusive)
        Paritition 0: STATE
            Start: 478035968, End: 488521727 (10485760 blocks)
            id:   51E8D442-0419-2447-96E5-49CB60CF0B25
            type: EBD0A0A2-B9E5-4433-87C0-68B6B72699C7
            flags: 0x0000000000000000
        Paritition 1: KERN-A
            Start: 20480, End: 53247 (32768 blocks)
            id:   054CD627-F23C-5C40-8035-C188FA57DE9C
            type: FE3A2A5D-4F32-41A7-B725-ACCC3285A309
            flags: priority=2 tries=0 successful=1
        Paritition 2: ROOT-A
            Start: 8704000, End: 17092607 (8388608 blocks)
            id:   936E138F-1ACF-E242-9C5B-3667FAA3C10C
            type: 3CB8E202-3B7E-47DD-8A3C-7FF2A13CFCEC
            flags: 0x0000000000000000
        Paritition 3: KERN-B
            Start: 53248, End: 86015 (32768 blocks)
            id:   A8667891-8209-8648-9D5E-63DC9B8D0CB3
            type: FE3A2A5D-4F32-41A7-B725-ACCC3285A309
            flags: priority=1 tries=0 successful=1
        Paritition 4: ROOT-B
            Start: 315392, End: 8703999 (8388608 blocks)
            id:   8B5D7BB4-590B-E445-B596-1E7AA1BB501F
            type: 3CB8E202-3B7E-47DD-8A3C-7FF2A13CFCEC
            flags: 0x0000000000000000
        Paritition 5: KERN-C
            Start: 17092608, End: 17223679 (131072 blocks)
            id:   C7D6B203-C18F-BC4D-9160-A09BA8970CE1
            type: FE3A2A5D-4F32-41A7-B725-ACCC3285A309
            flags: priority=3 tries=15 successful=1
        Paritition 6: ROOT-C
            Start: 17223680, End: 25612287 (8388608 blocks)
            id:   769444A7-6E13-D74D-B583-C3A9CF0DE307
            type: 3CB8E202-3B7E-47DD-8A3C-7FF2A13CFCEC
            flags: 0x0000000000000000
<!--4. KERN-C typically hosts the Zircon kernel. KERN-A and KERN-B typically have
ChromeOS kernels. To go to ChromeOS we need to lower the priority of KERN-C
here by referencing the **partition** index on the **disk** that has that
partition.-->
4. KERN-C 含有 Zircon 内核。KERN-A 和 KERN-B 代表 ChromeOS 内核。设置启动进入 ChromeOS 我们需要通过引用具有该分区的磁盘上的分区索引来降低 KERN-C 的优先级。

        $ gpt edit_cros 5 -P 0 /dev/class/block/000
<!--5. Reboot-->
5. 重启
<!--To go back to the Fuchsia kernel, just re-pave the device.-->
启动进入 Fuchsia 内核，只需要重设设备的分区索引。
