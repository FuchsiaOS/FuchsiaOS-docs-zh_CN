# Install Fuchsia from a USB flash drive

You can use a USB flash drive to make your target device to boot from
the Fuchsia installer, which then installs a freshly built Fuchsia
image on the device directly from the USB.

To prepare a USB flash drive to be a bootable disk, do the following:

1. Set the build configuration to `workstation_eng.x64` and include
   the recovery package (`recovery-installer`):

   ```posix-terminal
   fx set workstation_eng.x64 --with //build/images/recovery:recovery-installer --release
   ```

1. Build a new Fuchsia image and its artifacts:

   ```posix-terminal
   fx build
   ```

1. Plug a USB flash drive into your workstation.

1. Identify the path to the USB drive:

   ```posix-terminal
   fx list-usb-disks
   ```

   This command prints output similar to the following:

   ``` none {:.devsite-disable-click-to-copy}
   $ fx list-usb-disks
   /dev/sda - My Example USB Disk
   ```

1. Create a bootable USB drive:

   ```posix-terminal
   fx mkinstaller -v {{ "<var>" }}PATH_TO_USB_DRIVE{{ "</var>"}}
   ```

   Replace `PATH_TO_USB_DRIVE` with the path to the USB drive from the step above.

   The example command below selects the `/dev/sda` path:

   ``` none {:.devsite-disable-click-to-copy}
   $ fx mkinstaller -v /dev/sda
   ```

   When finished, the command prints output similar to the following in the end:


   ``` none {:.devsite-disable-click-to-copy}
   $ fx mkinstaller -v /dev/sda
   mkinstaller: WARNING: Changing ownership of /dev/sda to alice
   [sudo] password for alice:
   ...
   mkinstaller: INFO:    Writing image fvm.sparse.blk to partition storage-sparse...
   mkinstaller: INFO:      Wrote 835.6M in 35.55s, 23.5M/s
   mkinstaller: INFO: Done.
   mkinstaller: INFO: Ejected USB disk
   ```

1. Unplug the USB drive from the workstation.

1. Plug the bootable USB drive into your target device.

1. Configure the target device's BIOS to boot from a USB drive.

1. Reboot the target device.

   The device boots into the Fuchsia Workstation Installer.

1. Press **Enter** on prompts to continue the installation process.

   When the installation is finished, the screen displays `Success! Please restart your computer.`

1. Unplug the USB drive from the target device.

1. Reboot the target device.

   The target device is now booted into Fuchsia’s Workstation.
