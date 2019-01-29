<!--# Build and Pave Quickstart-->

# 快速入门与应用

<!--This document captures the common-case workflow for building and deploying
Fuchsia onto a device using `fx` development commands. Most such commands
have options for less common situations; see `fx help <command>` for details.-->
这篇文档介绍了一般情况下使用 `fx` 开发命令在设备上构建和部署 Fuchsia。大多数此类命令都有不常见情况的选项；有关详细信息，请参阅 `fx help <command>`。

<!--## Initial Build and Deploy-->

## 构建与部署

<!--The initial build and deploy workflow using `fx` is as follows:-->
使用 `fx` 的初始构建和部署工作流程如下：

<!--1. `fx set <arch>`
       Configures the build for <arch>: one of [x64, arm64].-->

<!--1.  `fx full-build`
    Builds Zircon, then the rest of Fuchsia.-->
<!--1.  `fx mkzedboot <usb_drive_device_path>`
    Builds the Zedboot media and installs to the USB drive target. See below
    for notes on obtaining the USB drive device path.-->
<!--1.  `fx pave`
    Starts the bootserver.-->
<!--1.  Attach Zedboot USB to device and reboot.
    Zedboot will connect to the host, download the pave image, and pave the
    device.-->

1. `fx set <arch>`
   为 [x64, arm64] 其中一个配置<arch>构建内容。

1. `fx full-build`

   构建 Zircon，然后是 Fuchsia 的其余部分。

1. `fx mkzedboot <usb_drive_device_path>`

   构建 Zedboot 媒体然后安装到 USB 驱动器中。请参见下边的内容说明，获取 USB 驱动器的路径。

1. `fx pave`

   启动 bootserver。

1.  将 Zedboot USB 连接到设备并重新启动。
   Zedboot 会连接主机，并下载安装镜像，然后安装到设备中。

   

<!--### USB drive device path-->
### USB 驱动器的路径

<!--ructions for determining the correct path to your USB drive are as follows,
depending on the host OS. In either case, you can run the command once with the
USB drive disconnected, then run again with it connected, to see the
difference.-->
根据操作系统的不同，确定 USB 驱动器的正确路径的说明如下。你可以运行这些命令观察在连接 USB 驱动器和未连接 USB 驱动器时的区别。

<!-- Linux users:-->
* Linux 用户：
  - `sudo fdisk -l`
    <!--Drives are usually of the form /dev/sd[x], e.g. '/dev/sdc'. Select
    the drive rather than a specific partition.-->
    <!--* Mac users:-->

    设备通常是 /dev/sd[x] 这种格式，例如：“/dev/sdc”。选择驱动器而不是特定分区。
* Mac 用户：
  - `diskutil list | grep external`
    <!--Drives are usually of the form /dev/disk[n], e.g. '/dev/disk2'.-->

    设备通常是 /dev/disk[n] 这种格式，例如：“/dev/disk2”。

  <!-- - If you see 'ERROR: Can't open /dev/disk[n]: Resource busy'
    then you will have to unmount the usb drive.
    For this run `hdiutil unmount /dev/disk[n]`.
    If this does not fix the error, try reformating the drive:
    `diskutil eraseDisk JHFSX <name_of_the_usb_stick> /dev/disk[n]`.-->

  -  如果看到错误 “ERROR: Can't open /dev/disk[n]: Resource busy” 那么你需要解除 USB 挂载。运行 `hdiutil unmount /dev/disk[n]` 。如果没有修复这个错误，尝试重新配置驱动：`diskutil eraseDisk JHFSX <name_of_the_usb_stick> /dev/disk[n]`。

<!--## Subsequent Build and Deploy-->
## 后续构建和部署

<!--The workflow for re-building and re-deploying using `fx` is slightly different:-->
使用 `fx` 重新构建和重新部署的工作流略有不同：

<!--1.  Check the [build waterfall dashboard](https://fuchsia-dashboard.appspot.com/).
    Helps ensure that HEAD is in a good state to pull.-->
<!--1.  `jiri update`
    Fetches the latest code.-->
<!--1.  `fx full-build`
    Builds Zircon, then the rest of Fuchsia.-->
<!--1.  `fx serve`
    Starts a development package server on the host.-->
<!--1.  Boot the device *without* Zedboot USB attached.
    Boots the device into its last-paved state.-->
<!--1.  `fx ota`
    Pushes updated packages to the device.-->

1. 检查[瀑布式仪表盘](https://fuchsia-dashboard.appspot.com/)。
   帮助确保 HEAD 处于可拉取状态。

1. `jiri update`
   获取最新的源码。
   
1. `fx full-build`

   构建 Zircon，和 Fuchsia 的剩余部分。
   
1. `fx serve`
    在主机上启动开发包服务器。

1. 移除 Zedboot USB 启动设备。
    将设备引导至最后安装的状态。

1. `fx ota`

    将更新的包推送到设备。

    

<!--NOTE: If desired, the device can be re-paved using Zedboot USB as per steps 4-5 in the previous section. This is slower, but may be necessary in some cases
where the system handles the OTA less than gracefully.-->

注意:如果需要，可以按照上一节中的步骤 4-5 使用 Zedboot USB 重新安装。虽然这比较慢，但是在系统处理 OTA 的方式不够优雅的情况下这可能又是必须的。

<!--## Troubleshooting-->
## 常见问题

<!--1.  Having '.' in your PATH may cause `fx full-build` to fail.  The script will
    change the working directory such that it may create conflicts between the
    commands it uses (e.g. `touch`) and the binaries in the working directory.-->
1. 在你的路径中含有 “.” 有可能导致 `fx full-build` 运行失败。脚本会改变工作目录，这有可能导致使用的命令（例如：`touch`）和在工作目录下的二进制文件产生冲突。
