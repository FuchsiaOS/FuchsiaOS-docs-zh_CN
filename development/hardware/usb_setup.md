<!--# Install Fuchsia from a USB flash drive-->
# 从 USB 闪存驱动器安装Fuchsia

<!--You can use a USB flash drive to make your target device to boot from
the Fuchsia installer, which then installs a freshly built Fuchsia
image on the device directly from the USB.-->
您可以使用 USB 闪存驱动器使您的目标设备从 Fuchsia 安装程序启动，然后直接从 USB 将新构建的 Fuchsia 映像安装到设备上。

<!--To prepare a USB flash drive to be a bootable disk, do the following:-->
要将 USB 闪存驱动器准备为可引导磁盘，请执行以下操作：

<!--1. Set the build configuration to `workstation_eng.x64` and include
   the recovery package (`recovery-installer`):-->
1. 请将构建配置设置为 workstation_eng.x64 并包含恢复安装包(`recovery-installer`)：   

   ```posix-terminal
   fx set workstation_eng.x64 --with //build/images/recovery:recovery-installer
   ```

<!--1. Build a new Fuchsia image and its artifacts:-->
1. 请构建一个新的 Fuchsia 映像及其源代码的生成物（artifacts）：

   ```posix-terminal
   fx build
   ```

<!--1. Plug a USB flash drive into your workstation.-->
1. 请将 USB 闪存驱动器插入您的工作站。

<!--1. Identify the path to the USB drive:-->
1. 确定 USB 驱动器的文件路径：

   ```posix-terminal
   fx list-usb-disks
   ```

   <!--This command prints output similar to the following:-->
   然后请用此命令打印出类似于以下内容的输出：

   ``` none {:.devsite-disable-click-to-copy}
   $ fx list-usb-disks
   /dev/sda - My Example USB Disk
   ```

<!--1. Create a bootable USB drive:-->
1. 请创建一个可启动的 USB 驱动器：

   ```posix-terminal
   fx mkinstaller -v --new-installer {{ "<var>" }}PATH_TO_USB_DRIVE{{ "</var>"}}
   ```

   <!--Replace `PATH_TO_USB_DRIVE` with the path to the USB drive from the step above.-->
   请将 `PATH_TO_USB_DRIVE` 替换为上述步骤中.

   <!--The example command below selects the `/dev/sda` path:-->
   请在下面的示例命令中选择 `/dev/sda` 路径：

   ``` none {:.devsite-disable-click-to-copy}
   $ fx mkinstaller -v --new-installer /dev/sda
   ```

   <!--When finished, the command prints output similar to the following in the end:-->
   完成后，该命令最后会打印出类似于以下内容的输出：


   ``` none {:.devsite-disable-click-to-copy}
   $ fx mkinstaller -v --new-installer /dev/sda
   mkinstaller: WARNING: Changing ownership of /dev/sda to alice
   [sudo] password for alice:
   ...
   mkinstaller: INFO:    Writing image fvm.sparse.blk to partition storage-sparse...
   mkinstaller: INFO:      Wrote 835.6M in 35.55s, 23.5M/s
   mkinstaller: INFO: Done.
   mkinstaller: INFO: Ejected USB disk
   ```

<!--1. Unplug the USB drive from the workstation.-->
1. 请从工作站上拔下 USB 驱动器。

<!--1. Plug the bootable USB drive into your target device.-->
1. 请将可启动 USB 驱动器插入目标设备。

<!--1. Configure the target device's BIOS to boot from a USB drive.-->
1. 请将目标设备的 BIOS 配置为从 USB 驱动器启动。

<!--1. Reboot the target device.-->
1. 请重新启动目标设备。

   <!--The device boots into the Fuchsia Workstation Installer.-->
   设备启动到 Fuchsia Workstation 安装程序。

<!--1. Press **Enter** on prompts to continue the installation process.-->
1. 请在出现提示时按 **Enter** 以继续安装过程。 

   <!--When the installation is finished, the screen displays `Success! Please restart your computer.`-->
   在安装完成后，屏幕显示`Success! Please restart your computer.`

<!--1. Unplug the USB drive from the target device.-->
1. 请从目标设备上拔下 USB 驱动器。

<!--1. Reboot the target device.-->
1. 请重新启动目标设备。

   <!--The target device is now booted into Fuchsia’s Workstation.-->
   随后目标设备现在启动到 Fuchsia 的工作站（Workstation）。
