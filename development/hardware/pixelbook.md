# 在 Pixelbook 上安装 Fuchsia 的准备工作

## 更新 ChromeOS

如果你的 Pixelbook 从来没有被启动过, 那么最好的方式就是正常启动检查所有关键的更新，方法如下：

1. 正常启动 Pixelbook， 通常翻开盖子就会给 Pixelbook 供电
如果没有启动， 电源键就在设备的左侧，靠近腕托前面。
2. 点击“Let's go”按钮。
3. 连接到有线或无线网络。
4. 接受条款以继续进行更新检查步骤。
5. 设备应该会检查更新，并安装所有更新。
6. 安装所有更新后重启设备，点击左下角的”Browse as Guest（以访客身份访问）“。
7. 在ChromeOS的主界面，点击”Settings->About Chrome OS（设置->关于Chrome OS）“或”Help->About Chrome
OS（帮助->关于Chrom OS）“确定版本&gt;=62

## 讲设备设置为开发者模式
***注意: 这个操作会清除本地所有的设置***

1. 关闭 Pixelbook 的电源。
2. 进入Recovery模式。
按住 Esc+Refresh (键盘上第一行的第一个和第三个按钮)
然后再按电源键 (设备左侧的按钮)。
3. 首先按Ctrl+D关闭系统验证。你会看到"To turn OS verification OFF, press ENTER（关闭系统验证，请按ENTER）"。回车确认选择。
4. 如果 Pixelbook 重启，可以确定系统验证已经被关闭。再次按 Ctrl+D 进入开发者模式。
5. 等待设备自己配置，这将会花费几分钟时间。最初设备好像什么都没有做。静置设备一两分钟。配置开始会听到两声哔哔声。当适配完成会发出两声更加响亮的哔哔声。
6. 开发者模式配置完成，设备会重起。可以进入第二步”从USB启动“。

## 从USB启动

1. 进入ChromeOS。
2. 你会看到屏幕显示"OS verification is OFF（系统验证已经关闭）"然后大约30秒会继续加载系统。等欢迎界面或者登陆页面加载。 **忽略** 所有的"Enable debugging features（启用调试功能）"的链接。
3. 按 Ctrl+Alt+Refresh/F3 打开命令行窗口。 如果按下组合键没有效果，重启一次之后再尝试。
4. 输入'chronos'作为用户名，密码为空。
5. 通过运行 `sudo crossystem dev_boot_usb=1` 启动USB。
6. (可选) 运行 `sudo crossystem dev_default_boot=usb` 设置默认USB启动。
7. 插入USB驱动器（U盘）。
8. 运行 `sudo reboot` 重启设备。
9. 在"OS verification is OFF（系统验证已经关闭）"页面，按 Ctrl+U 可以立即进入USB启动。 ( [提示 & 技巧](#提示-&-技巧)<a href="#提示-&-技巧">提示 & 技巧</a> 查看其他简单操作)

Th USB drive is only needed for booting when you want to re-pave or otherwise
netboot the device. If you didn't make USB booting the default (Step #6), you
will need to press Ctrl+U at the grey 'warning OS-not verified' screen to boot
from USB when you power on your device. If the device tries to boot from USB,
either because that is the default or you pressed Ctrl+U, and the device fails
to boot from USB you'll hear a fairly loud &lt;BEEP&gt;. Note that ChromeOS
bootloader USB enumeration during boot has been observed to be slow. If you're
having trouble booting from USB, it may be helpful to remove other USB devices
until the device is through the bootloader and also avoid using a USB hub.

## 提示 & 技巧

By default the ChromeOS bootloader has a long timeout to allow you to press
buttons. To shortcut this you can press Ctrl+D or Ctrl+U when on the grey screen
that warns that the OS will not be verified. Ctrl+D will cause the device to
skip the timeout and boot from its default source. Ctrl+U will skip the timeout
and boot the device from USB.

### 回到ChromeOS

To go back to ChromeOS you must modify the priority of the Fuchsia kernel
partition to be lower than that of at least one of the two ChromeOS kernel
partitions.

1. Press Alt+Esc to get to a virtual console
2. Find the disk that contains the KERN-A, KERN-B, and KERN-C partitions with
the `lsblk` command. Below this is device 000, note how the device path of the
kernel partitions is an extension of that device.

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
3. Use the `gpt` command to look at the device's (eg. 000) partition map.

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
4. KERN-C typically hosts the Zircon kernel. KERN-A and KERN-B typically have
ChromeOS kernels. To go to ChromeOS we need to lower the priority of KERN-C
here by referencing the **partition** index on the **disk** that has that
partition.

        $ gpt edit_cros 5 -P 0 /dev/class/block/000
5. Reboot

To go back to the Fuchsia kernel, just re-pave the device.
