<!--# Build and Pave Quickstart-->
# 快速入门与应用

This document captures the common-case workflow for building and deploying
Fuchsia onto a device using `fx` development commands. Most such commands
have options for less common situations; see `fx help <command>` for details.

<!--## Initial Build and Deploy-->
## 创建与部署

The initial build and deploy workflow using `fx` is as follows:

1.  `fx set <arch>`
    Configures the build for <arch>: one of [x64, arm64].
1.  `fx full-build`
    Builds Zircon, then the rest of Fuchsia.
1.  `fx mkzedboot <usb_drive_device_path>`
    Builds the Zedboot media and installs to the USB drive target. See below
    for notes on obtaining the USB drive device path.
1.  `fx pave`
    Starts the bootserver.
1.  Attach Zedboot USB to device and reboot.
    Zedboot will connect to the host, download the pave image, and pave the
    device.

### USB drive device path

Instructions for determining the correct path to your USB drive are as follows,
depending on the host OS. In either case, you can run the command once with the
USB drive disconnected, then run again with it connected, to see the
difference.

* Linux users:
  - `sudo fdisk -l`
    Drives are usually of the form /dev/sd[x], e.g. '/dev/sdc'. Select
    the drive rather than a specific partition.
* Mac users:
  - `diskutil list | grep external`
    Drives are usually of the form /dev/disk[n], e.g. '/dev/disk2'.
  - If you see 'ERROR: Can't open /dev/disk[n]: Resource busy'
    then you will have to unmount the usb drive.
    For this run `hdiutil unmount /dev/disk[n]`.
    If this does not fix the error, try reformating the drive:
    `diskutil eraseDisk JHFSX <name_of_the_usb_stick> /dev/disk[n]`.

## Subsequent Build and Deploy

The workflow for re-building and re-deploying using `fx` is slightly different:

1.  Check the [build waterfall dashboard](https://fuchsia-dashboard.appspot.com/).
    Helps ensure that HEAD is in a good state to pull.
1.  `jiri update`
    Fetches the latest code.
1.  `fx full-build`
    Builds Zircon, then the rest of Fuchsia.
1.  `fx serve`
    Starts a development package server on the host.
1.  Boot the device *without* Zedboot USB attached.
    Boots the device into its last-paved state.
1.  `fx ota`
    Pushes updated packages to the device.

NOTE: If desired, the device can be re-paved using Zedboot USB as per steps 4-5
in the previous section. This is slower, but may be necessary in some cases
where the system handles the OTA less than gracefully.

## Troubleshooting

1.  Having '.' in your PATH may cause `fx full-build` to fail.  The script will
    change the working directory such that it may create conflicts between the
    commands it uses (e.g. `touch`) and the binaries in the working directory.
