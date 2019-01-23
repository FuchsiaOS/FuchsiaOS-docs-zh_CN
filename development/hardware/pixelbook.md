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

## 设置开发者模式
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
9. 在"OS verification is OFF（系统验证已经关闭）"页面，按 Ctrl+U 可以立即进入USB启动。 ( [提示 & 技巧](#提示--技巧) 查看其他简单操作)

只有你想重新安装或网络启动设备时才需要USB驱动器。如果你没有设置USB默认启动（第6步）， 你需要在设备开启电源后，在界面显示”warning OS-not verified（警告系统没有验证）“时按 Ctrl+U。如果设备默认USB启动或者你按了 Ctrl+U ，那么设备会尝试从USB启动，如果启动失败，设备会发出很大得哔哔声。注意，ChromeOS引导程序在引导期间的USB总线枚举过程被观察到很慢。如果你USB启动遇到困难，你可以尝试移除其他所有的USB驱动设备并且不使用USB集线器连接USB驱动器，直到通过引导程序。

## 提示 & 技巧

默认情况下，ChromeOS引导加载程序有很长的超时时间允许您按下按钮。 再灰色的界面上按快捷键 Ctrl+D 或 CTRL +U 可以跳过系统不会被验证的警告。Ctrl+D 可以跳过等待时间然后从默认资源启动。Ctrl+U 可以跳过等待时间然后从USB启动。

### 设置启动ChromeOS

安装Fuchsia后要启动Chromeos，必须将Fuchsia内核分区的优先级修改为低于至少两个Chromeos内核分区中的一个。

1. 按 Alt+Esc 打开虚拟控制台
2. 使用`lsblk` 找到包含KERN-A， KERN-B 和 KERN-C 的磁盘分区。下面是设备000 的示例，注意，内核分区的路径是该设备的扩展。

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
4. KERN-C 含有Zircon内核。KERN-A 和 KERN-B 代表 ChromeOS 内核。设置启动进入ChromeOS 我们需要通过引用具有该分区的磁盘上的分区索引来降低 KERN-C 的优先级。

        $ gpt edit_cros 5 -P 0 /dev/class/block/000
5. 重启

启动进入Fuchsia内核，只需要重设设备的分区索引。
