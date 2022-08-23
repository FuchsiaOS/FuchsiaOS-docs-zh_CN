<!-- 
# Install Fuchsia on a Chromebook
 -->
# 在 Chromebook 上安装 Fuchsia


<!-- 
## Supported Chromebooks
 -->
## 支持的 Chromebook

<!-- 
These Chromebooks are used regularly by developers and should be stable.
 -->
这些 Chromebook 开发者经常使用，应当是稳定支持的。

<!-- 
* Google Pixelbook Go (_atlas_)
 -->
* Google Pixelbook Go（_atlas_）

<!-- 
### Formerly supported Chromebooks
 -->
### 以前支持的 Chromebook

<!-- 
These Chromebooks are supported on a best-effort basis, and are not regularly tested.
 -->
这些 Chromebook 受到“尽力而为”的支持，并且不会受到定期测试。

<!-- 
* Google Pixelbook (_eve_)
 -->
* Google Pixelbook（_eve_）

<!-- 
### Other ChromeOS devices
 -->
### 其他 ChromeOS 设备

<!-- 
Other x86-based ChromeOS devices may or may not work. ARM-based ChromeOS devices will not work out of the box.
 -->
其他基于 x86 的 ChromeOS 设备可能工作，也可能不会工作。基于 ARM 的 ChromeOS 设备无法开箱即用。

<!-- 
## Prerequisites
 -->
## 前提条件

<!-- 
Ensure that you have a `chromebook-x64` build for Fuchsia.
 -->
请确保您为 Fuchsia 进行了 `chromebook-x64` 构建。

<!-- 
1.  Complete the [Download the Fuchsia source code][get-fuchsia-source]
    guide.
2.  As part of [Configure and Build Fuchsia][build-fuchsia], set your build
    configuration to use the following Chromebook product:
 -->
1.  完成[下载 Fuchsia 源代码][get-fuchsia-source]指南。
2.  请在[配置和构建 Fuchsia][build-fuchsia]中设置您的构建配置，以使用如下的 Chromebook 产品：

    ```posix-terminal
    fx set workstation_eng.chromebook-x64 --release
    ```

<!-- 
## Update ChromeOS
 -->
## 更新 ChromeOS

<!-- 
If your Chromebook has never been booted, it is best to boot it normally to check
for any critical updates, as follows:
 -->
如果您的 Chromebook 从未启动过，请您最好将其启动，以检查重要的更新，步骤如下：

<!-- 
1. Boot the Chromebook normally. Opening the lid usually powers on the device.
If this doesn't work, the power button is on the left side of the device, near
the front of the wrist rest.
 -->
1. 正常启动 Chromebook。通常，打开盖子会启动设备。如果没有启动，请使用位于设备左侧、靠近腕托位置的电源按钮。
<!-- 
2. Tap the "Let's go" button.
 -->
2. 点按“开始使用”（Let's go）按钮。
<!-- 
3. Connect to a wired or wireless network.
 -->
3. 连接到有线或无线网络。
<!-- 
4. Accept the terms to proceed to the update check step.
 -->
4. 接受条款，以进入更新检查步骤。
<!-- 
5. The device should check for updates, install any found.
 -->
5. 设备应当检查并安装找到的更新。

<!-- 
6. After rebooting from any updates, tap 'Browse as Guest' in the lower left
corner.
 -->
6. 更新重启之后，点按左下角的“以访客身份浏览”（Browse as Guest）。

<!-- 
7. From the browser UI, go into "Settings->About Chrome OS" or "Help->About Chrome
OS" and confirm the newly installed version.
 -->
7. 通过浏览器用户界面，前往“设置->关于 Chrome OS”（Settings->About Chrome OS）或“帮助->关于 Chrome OS”（Help->About Chrome
OS），确认新安装的版本。

<!-- 
## Put your device into developer mode
 -->
## 为您的设备开启开发者模式

<!-- 
Caution: This will erase any state stored locally on your Chromebook.
 -->
注意：这将会擦除保存在您 Chromebook 本地的任何状态。

<!-- 
1. Power off the Chromebook.
 -->
1. 关闭 Chromebook 电源。
<!-- 
2. Go into Recovery Mode.
Hold down Esc+Refresh (first and third buttons on the top row of the keyboard).
Then press the Power button (bottom left side of the device).
 -->
2. 进入恢复模式。同时按住 Esc+Refresh（键盘顶行的第一和第三个按钮）。接着按下电源按钮（设备的左下方）。
<!-- 
3. Start by disabling OS verification by pressing Ctrl+D. You should see "To turn OS verification OFF, press ENTER". Press Enter to confirm.
 -->
3. 按下 Ctrl+D，以在禁用 OS 验证的情况下启动。您应当看到“要关闭 OS 验证，请按下 ENTER。”（To turn OS verification OFF, press ENTER）
<!-- 
4. When your device reboots, you'll get confirmation that OS verification is OFF. Press Ctrl+D again to enter Developer Mode.
 -->
4. 当您的设备重启时，您将收到 OS 验证已经关闭的确认信息。再次按下 Ctrl+D 进入开发者模式（Developer Mode）。
<!-- 
5. Wait for the device to re-configure itself, which will take several minutes.
Initially it may not appear to be doing anything. Let the device sit for a
minute or two. You will hear two loud &lt;BEEP&gt;s early in the process. The
process is complete when you hear two more loud &lt;BEEP&gt;s.
 -->
5. 等待设备自行完成重新配置，可能需要几分钟时间。设备最初可能看起来没有任何反应。请允许设备静置一到两分钟。在此过程的前期，您可能听到两次响亮的“哔”声。该过程在您再次听到两次响亮的“哔”声时完成。
<!-- 
6. The device should reboot itself when the Developer Mode transition is
complete. You can now jump to Step #2 in the "Boot from USB" section.
 -->
6. 当开发者模式转换完成时，设备应当自行重启。现在您可以跳至“从 USB 启动”一节的步骤 #2 了。

<!-- 
## Boot from USB
 -->
## 从 USB 启动

<!-- 
1. Boot into ChromeOS.
 -->
1. 启动至 ChromeOS。
<!-- 
2. You should see a screen that says "OS verification is OFF" and approximately
30 seconds later the boot will continue. Wait for the Welcome or Login screen
to load. **Ignore** any link for "Enable debugging features".
 -->
2. 您应当看到显示“OS 验证已关闭”（OS verification is OFF）的画面，在大约 30 秒后，启动会继续。请等待欢迎或登录界面加载。请**忽略**任何用于“启用调试功能”（Enable debugging features）的链接。
<!-- 
3. Press Ctrl+Alt+Refresh/F3 to enter a command shell. If pressing this key
combination has no effect, try rebooting the Chromebook once more.
 -->
3. 按下 Ctrl+Alt+Refresh/F3 进入命令行界面。如果该组合键无效，请尝试再次重启 Chromebook。
<!-- 
4. Enter 'chronos' as the user with a blank password
 -->
4. 进入“chronos”用户，密码留空。
<!-- 
5. Enable USB booting by running `sudo crossystem dev_boot_usb=1`
 -->
5. 运行 `sudo crossystem dev_boot_usb=1`，启用 USB 启动。
<!-- 
6. _(optional)_ Default to USB booting by running `sudo crossystem dev_default_boot=usb`.
 -->
6. （**可选**）运行 `sudo crossystem dev_default_boot=usb`，将 USB 启动设为默认。
<!-- 
7. Plug the USB drive into the Chromebook.
 -->
7. 将 USB 驱动器插入 Chromebook。
<!-- 
8. Reboot by typing `sudo reboot`
 -->
8. 键入 `sudo reboot` 进行重启。
<!-- 
9. On the "OS verification is OFF" screen press Ctrl+U to bypass the timeout and
boot from USB immediately. (See [Tips and Tricks](#tips-and-tricks) for other
short circuit options)
 -->
9. 在“OS 验证已关闭”画面，按下 Ctrl+U 跳过超时等待并立即从 USB 启动。（要获取其他短路选项，请参阅[提示和技巧](#tips-and-tricks)）

<!-- 
The USB drive is only needed for booting when you want to re-pave or otherwise
netboot the device.
 -->
仅当您想要重新铺设（re-pave）或另行从网络启动设备时，才需要 USB 驱动器进行引导。

<!-- 
If you didn't make USB booting the default (Step #6), you will need to press Ctrl+U
at the grey 'warning OS-not verified' screen to boot from USB when you power on your device.
 -->
如果您未将 USB 启动设为默认（步骤 #6），那么在开机时，您将需要在灰色的“警告 OS 未验证”（warning OS-not verified）画面按下 Ctrl+U，以从 USB 启动。

<!-- 
If the device tries to boot from USB, either because that is the default or you
pressed Ctrl+U, and the device fails to boot from USB you'll hear a fairly loud &lt;BEEP&gt;.
 -->
如果设备试图从 USB 启动（可能是因为您将其设为了默认，或按下了 Ctrl+U）失败，那么您将听到一次相当响亮的“哔”声。

<!-- 
Note that ChromeOS bootloader USB enumeration during boot has been observed to be slow. If you're
having trouble booting from USB, it may be helpful to remove other USB devices
until the device is through the bootloader and also avoid using a USB hub.
 -->
注意，已经观察到启动过程中 ChromeOS 引导加载程序 USB 枚举速度很慢。如果您在从 USB 启动时遇到问题，那么在设备通过引导加载程序前，移除其他 USB 设备可能会有所帮助；另外，也请避免使用 USB 集线器。

<!-- 
## Tips and Tricks {#tips-and-tricks}
 -->
## 提示和技巧 {#tips-and-tricks}

<!-- 
By default the ChromeOS bootloader has a long timeout to allow you to press
buttons. To shortcut this you can press Ctrl+D or Ctrl+U when on the grey screen
that warns that the OS will not be verified. Ctrl+D will cause the device to
skip the timeout and boot from its default source. Ctrl+U will skip the timeout
and boot the device from USB.
 -->
默认情况下，ChromeOS 引导加载程序的超时等待时间很长，以便您能够按下按键。要跳过此过程，您可以在警告 OS 无法验证的灰色画面中按下 Ctrl+D 或 Ctrl+U。Ctrl+D 将使设备跳过超时等待，并从其默认源启动。Ctrl+U将跳过超时等待，并从 USB 启动设备。

<!-- 
### Configuring boot source from Fuchsia
 -->
### 配置 Fuchsia 引导源

<!-- 
Fuchsia has an equivalent to `crossystem` called `cros_nvtool`.
You can run `cros_nvtool set dev_boot_default <usb|disk>` to modify the default boot source of
the system to USB or disk, respectively.
 -->
Fuchsia 具有称为 `cros_nvtool` 的 `crossystem` 等价物。您可以运行 `cros_nvtool set dev_boot_default <usb|disk>` 将系统的默认引导源分别修改为 USB 或硬盘。

<!-- 
### Going back to ChromeOS
 -->
### 返回 ChromeOS

<!-- 
To go back to ChromeOS you must modify the priority of the Fuchsia kernel
partition to be lower than that of at least one of the two ChromeOS kernel
partitions.
 -->
要返回 ChromeOS，您必须修改 Fuchsia 内核分区的优先级低于两个 ChromeOS 内核分区中的至少一个。

<!-- 
1. Press Alt+Esc to get to a virtual console if not already on one
2. Press Alt+Fullscreen to get to a terminal emulator on Fuchsia
3. Find the disk that contains the KERN-A, KERN-B, and KERN-C partitions with
the `lsblk` command. Below this is device 000, note how the device path of the
kernel partitions is an extension of that device.
 -->
1. 如果您未处于虚拟控制台中，请按下 Alt+Esc 进入其中
1. 按下 Alt+Fullscreen 进入 Fuchsia 中的终端模拟器
1. 使用 `lsblk` 命令查找含有 KERN-A、KERN-B 和 KERN-C 分区的硬盘。下例中为设备 000，请注意，内核分区的设备路径是其设备本身路径的扩展。

        $ lsblk
        ID  SIZE TYPE             LABEL                FLAGS  DEVICE
        000 232G                                              /dev/sys/platform/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block
        001   5G data             STATE                       /dev/sys/platform/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-000/block
        002  16M cros kernel      KERN-A                      /dev/sys/platform/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-001/block
        003   4G cros rootfs      ROOT-A                      /dev/sys/platform/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-002/block
        004  16M cros kernel      KERN-B                      /dev/sys/platform/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-003/block
        005   4G cros rootfs      ROOT-B                      /dev/sys/platform/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-004/block
        006  64M cros kernel      KERN-C                      /dev/sys/platform/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-005/block
        007   4G cros rootfs      ROOT-C                      /dev/sys/platform/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-006/block
<!-- 
4. Use the `gpt` command to look at the device's (eg. 000) partition map.
 -->
4. 使用 `gpt` 命令查看设备（例，000）的分区表。

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
<!-- 
5. Note that KERN-A and KERN-B typically have ChromeOS kernels. The
   Zircon kernel appears as KERN-C as in the example above, or as
   ZIRCON-A instead in certain setups.
 -->
5. 注意，KERN-A 和 KERN-B 通常具有 ChromeOS 内核。Zircon 内核在上例中显示为 KERN-C，或者在某些设置中为 ZIRCON-A。

<!-- 
   To go to ChromeOS, lower the priority of KERN-C (or ZIRCON-A)
   by referencing the **partition** index on the **disk** that has
   that partition. E.g.:
 -->
   要进入 ChromeOS，请通过引用**硬盘**上**分区**的索引，来降低 KERN-C（或 ZIRCON-A）的优先级。例如：

        $ gpt edit_cros 5 -P 0 /dev/class/block/000

<!-- 
6. Reboot.
 -->
6. 重启。

<!-- 
7. When the ChromeOS bootloader appears, press Space to re-enable
OS Verification. Your device will reboot. This time, it will display
a message with "Your system is repairing itself. Please wait". This operation
will take around 5 minutes, after which the Chromebook will reboot one final
time. The device will reboot to the initial setup screen.
 -->
7. 当 ChromeOS 引导加载程序出现时，请按下空格键重新启用 OS 验证。您的设备将会重启。这一次，消息会显示“您的系统正在进行自我修复。请稍候。”（Your system is repairing itself. Please wait.）该操作将大约需要 5 分钟，之后 Chromebook 将最后重启一次。设备将重启至初始设置界面。

<!-- 
To go back to the Fuchsia kernel, just re-pave the device.
 -->
要返回 Fuchsia 内核，请重新铺设设备。

[get-fuchsia-source]: /get-started/get_fuchsia_source.md
[build-fuchsia]: /get-started/build_fuchsia.md
