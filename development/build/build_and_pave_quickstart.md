<!--
# Build and pave quickstart

This document shows you how to build and deploy Fuchsia on a target device with
`fx` development commands. Most of these commands have additional commands, see
`fx help <command>` for details.
-->
# 构建快速入门

本文档向您展示如何使用开发命令`fx`在目标设备上构建和运行Fuchsia。大多数命令含有额外的子命令。可以通过`fx help <command>`了解更多细节。

<!--
## Determine USB drive device path {#usb-drive-device-path}

Before you attempt to build and pave Fuchsia on a target device, you need to
know the path of your USB drive.

Note: For either operating system, you can run the command once with the USB
drive disconnected, then run again with the USB drive connected, to see the
difference.
-->
# 指定USB设备驱动路径{#usb-drive-device-path}

在目标设备上构建和部署Fuchsia之前，需要了解您所使用的USB设备的路径。

注意：在您所使用的任何操作系统上都可以在USB设备断开之后执行以下命令，之后在USB设备重新连接之后再执行一遍命令以查看二者的差异。

<!--
### fx

To determine the correct path to your USB drive:

Note: The `fx` tool is platform agnostic and lists available USB drives.

```posix-terminal
fx mkzedboot
```
-->

### fx

通过以下命令确定USB设备的确切路径：
注意：`fx`工具用于列出可用的USB设备，与具体的操作系统平台无关。

```posix-terminal
fx mkzedboot

<!--
### Linux

To determine the correct path to your USB drive:

```posix-terminal
sudo fdisk -l
```

Drives are usually in the form `/dev/sd[x]` such as `/dev/sdc`.

Make sure that you select the drive rather than a specific partition. For
example, a specific partition has a number at the end of the path such as
`/dev/sdc1`.
-->
### Linux系统

可以通过以下命令确定USB设备的确切路径：
```posix-terminal
sudo fdisk -l
```

设备路径通常形如`/dev/sd[x]`，比如`/dev/sdc`。

确保选择的是一个设备而不是某一个分区。一个分区路径信息的末尾通常有一个数字，如`/dev/sdc1`。

<!--
### macOS

To determine the correct path to your USB drive:

```posix-terminal
diskutil list | grep external
```

Drives are usually in the form `/dev/disk[n]` such as `/dev/disk2`.

Note: If you see `ERROR: Can't open /dev/disk[n]: Resource busy` then you will
have to unmount the USB drive. To do this, run:

```posix-terminal
hdiutil unmount /dev/disk[n]
```

If this does not fix the error, try reformatting the drive:

```posix-terminal
diskutil eraseDisk JHFSX <name_of_the_usb_stick> /dev/disk[n]
```
-->
### macOS

可以通过以下命令确定USB设备的确切路径：

```posix-terminal
diskutil list | grep external
```
设备路径通常形如：`/dev/disk[n]`，比如：`/dev/disk2`。

注意：如果执行了以上命令后出现`ERROR: Can't open /dev/disk[n]: Resource busy`，可以先尝试断开USB设备：

```posix-terminal
hdiutil unmount /dev/disk[n]
```
如果错误还未消除，可以尝试格式化它：

```posix-terminal
diskutil eraseDisk JHFSX <name_of_the_usb_stick> /dev/disk[n]
```

<!--
## Build and deploy Fuchsia

To perform an initial build and deploy of Fuchsia with `fx`, do the following:

1.  Set your build type:

    Note: Configures the build to build the `core` product on a generic x64
    board. For a list of available products and boards, see `fx list-products`
    and `fx list-boards` for lists of available products, respectively.

    ```posix-terminal
    fx set core.x64
    ```
-->
## 构建并部署Fuchsia

要使用`fx`从头构建Fuchsia，跟着如下步骤来：

1. 设定构建类型：
   注：本设定为在通用的x64架构主板上构建`core`核心产品。要想查看支持的产品和主板，可以相应的用`fx list-products`
    和`fx list-boards`命令查看。
    ```posix-terminal
    fx set core.x64
    ```

<!--
1.  Build a Fuchsia image:

    ```posix-terminal
    fx build
    ```

    This command builds Zircon and then the rest of Fuchsia.

-->
1. 使用如下命令构建Fuchsia镜像：
    ```posix-terminal
    fx build
    ```
    
    以上命令首先构建Zircon，然后构建Fuchsia的其他组件。
    
<!--
    
3.  Build the Zedboot media and install to a USB device target:

    Note: For information on obtaining the USB drive device path, see
    [USB drive device path](#usb-drive-device-path).

    ```posix-terminal
    fx mkzedboot <usb_drive_device_path>
    ```
-->
1. 构建Zedboot组件并安装到目标USB设备上：
    
    ```posix-terminal
    fx mkzedboot <usb设备路径>
    ```
    
    注意：如何获取USB设备路径，参考[USB设备路径](#usb-drive-device-path).

<!--
3.  Attach Zedboot USB drive to your target device and reboot that device.

1.  On your target device, run:

    ```posix-terminal
    lsblk
    ```
-->
1. 把上一步构建好的带有Zedboot信息的USB设备连接到目标设备上并重启。
2. 在目标设备上执行：

    ```posix-terminal
    lsblk
    ```

<!--
3.  Take note of the HDD or SSD's device path from the output of `lsblk`. An
    example path looks like `/dev/sys/pci/00:17.0/ahci/sata0/block`.

1.  On your target device, run:

    ```posix-terminal
    install-disk-image init-partition-tables --block-device <BLOCK_DEVICE_PATH>
    ```
-->
1. 留意该命令的输出中的HDD或SSD设备的路径。该路径形如：`/dev/sys/pci/00:17.0/ahci/sata0/block`。
2. 在目标设备上执行：

    ```posix-terminal
    install-disk-image init-partition-tables --block-device <HDD或SSD块设备路径>
    ```

<!--
3.  To start the bootserver, from your host, run:

    Note: The bootserver connects to the target device to upload the Fuchsia
    image and then paves your target device.

    ```posix-terminal
    fx pave
    ```
-->
3. 在本机上开始 bootserver
    ```posix-terminal
    fx pave
    ```
    
    注意：bootserver 会连接到目标设备上，并上传Fuchsia镜像，之后铺设。

<!--
## Rebuild and redeploy Fuchsia

To rebuild and redeploy with `fx`:

1.  Ensure that HEAD is in a good state to pull at the
    [build dashboard](https://luci-milo.appspot.com/p/fuchsia).
1.  Fetch the latest code:

    ```posix-terminal
    jiri update
    ```

1.  Build a Fuchsia image:

    ```posix-terminal
    fx build
    ```

    This command builds Zircon and then the rest of Fuchsia.

1.  (Only for macOS users) Set up firewall rules:

    ```posix-terminal
    fx setup-macos
    ```

1.  From your host, start a development package server:

    ```posix-terminal
    fx serve
    ```

1.  Boot your target device without the Zedboot USB attached.

1.  From your host, push updated Fuchsia packages to the target device:

    ```posix-terminal
    fx ota
    ```

    In some cases, if `fx ota` does not complete successfully, consider repaving
    with `fx pave`.
-->

## Fuchsia的重建和重部署

采用`fx`进行重建和重部署：

1.  确保版本状态正常，并指向[构建面板](https://luci-milo.appspot.com/p/fuchsia)

1. 使用以下命令下载最新代码：

    ```posix-terminal
    jiri update
    ```
1. 构建Fuchsia镜像：
1.  Build a Fuchsia image:

    ```posix-terminal
    fx build
    ```
    
本命令构建Zircon并构建Fuchsia的其余部分。

1. 设定防火墙规则（仅限于使用macOS的用户）

    ```posix-terminal
    fx setup-macos
    ```
    
1. 从你构建的宿主机上启动一个开发打包服务器:

    ```posix-terminal
    fx serve
    ```
1. 在Zedboot USB设备未连接时启动目标设备.
1. 从宿主机上推送Fuchsia更新包到目标设备:

    ```posix-terminal
    fx ota
    ```
某些情况下，如果`fx ota`命令没有成功执行，考虑用`fx pave`重发。
    
<!--
## Troubleshooting

*   If `fx build` fails, make sure that your `PATH` environment variable is set
    correctly.

    Note: The `fx` script changes the working directory in a way that may create
    conflicts between the commands it uses (such as `touch`) and the binaries in
    the working directory.

    To check the value of your `PATH` variable:

    ```posix-terminal
    echo $PATH
    ```

    Make that sure that the output of your `PATH` variable is a list of
    directories separated by colons. Make sure that none of the directories are
    separated by `.`.
-->

## 问题排查

* 若 `fx build` 失败，请检查PATH环境变量是否正确设置。
    使用如下命令确认PATH环境变量的值：
    
    ```posix-terminal
    echo $PATH
    ```
    
    确保输出的信息是一系列由冒号`:`分隔的目录名，注意不是`.`分隔。
    注意：`fx`相关命令会以某种方式重设工作目录，该方式可能会导致它所使用的命令（如touch）与在工作路径下的可执行程序产生冲突。
