# Install Fuchsia on a NUC

This document describes how to get a NUC up and running with Fuchsia.

[TOC]

## 1. Get Parts {#get-parts}

You need the following:

- USB 3.0 Drive
- NUC
- RAM
- m.2 SSD
- Keyboard
- Monitor that supports HDMI
- HDMI cable
- Ethernet cable
- Magnetic tip phillips head screwdriver

This table shows some example parts you can get from Amazon.

| Item | Link | Notes: |
| ---- | ---- | ------ |
| NUC | [B01MSZLO9P](https://www.amazon.com/gp/product/B01MSZLO9P) | Get a NUC7 (Kaby Lake) or NUC6 (Skylake) for GPU support. |
| RAM | [B01BIWKP58](https://www.amazon.com/gp/product/B01BIWKP58) | Works fine. |
| SSD | [B01IAGSDJ0](https://www.amazon.com/gp/product/B01IAGSDJ0) | Works fine. You only need one of these SSDs. |
| SSD | [B00TGIVZTW](https://www.amazon.com/gp/product/B00TGIVZTW) | Works fine. |
| SSD | [B01M9K0N8I](https://www.amazon.com/gp/product/B01M9K0N8I) | Works fine. |
| Monitor | [B015WCV70W](https://www.amazon.com/gp/product/B015WCV70W) | Works fine. |
| HDMI Cable | [B014I8SIJY](https://www.amazon.com/gp/product/B014I8SIJY) | Works fine. |
| Keyboard | [B00B7GV802](https://www.amazon.com/gp/product/B00B7GV802) | Works fine. It also includes a mouse. |
| USB 3.0 drive | [B01BGTG41W](https://www.amazon.com/gp/product/B01BGTG41W) | Works fine. |

## 2. Prepare the NUC {#prepare-the-nuc}

NUCs don’t come with RAM or an SSD, so you need to install them.

<img width="50%" src="/docs/images/developing_on_nuc/parts.jpg"/>

Follow the instructions to install the RAM and SSD on the NUC:

1. Remove the phillips screws in the bottom feet of the NUC.

   <img width="50%" src="/docs/images/developing_on_nuc/nuc_bottom.jpg"/>
   <img width="50%" src="/docs/images/developing_on_nuc/nuc_inside.jpg"/>
1. Install the RAM.
1. Remove the phillips screw that will hold the SSD in place (phillips screwdriver with magnetic tip is useful here).
1. Install the SSD.
1. Screw the SSD in place using screw from Step 3.

   <img width="50%" src="/docs/images/developing_on_nuc/parts_installed.jpg"/>
1. Replace bottom and screw feet back in.
1. Plug power, ethernet cable, HDMI, and keyboard into the NUC.
1. Plug the other end of the ethernet cable into your build workstation or the router/switch that connects to your build workstation.

## 3. Enable EFI booting {#enable-efi-booting}

1. Reboot NUC.
1. Press F2 while booting to enter BIOS.
1. In the Boot Order window on the left, click the Legacy tab.
1. Uncheck ‘Legacy Boot’.

   <img width="50%" src="/docs/images/developing_on_nuc/bios.jpg"/>
1. Click the `Advanced` button and confirm the following boot configuration:
    1. Select the `Boot Priority` tab.
       1. Check `UEFI Boot`.
       1. Set `USB` the first entry in the boot order.
    1. Select the `Boot configuration` tab.
       1. Check `Boot Network Devices Last`.
       1. Check `Unlimited Network Boot Attempts`.
       1. Check `USB boot devices`.
       1. Set `Network boot` to `UEFI PXE & iSCSI`.
2. Select the `Secure Boot` tab and uncheck `Secure Boot`.
3. Press F10 to save the changes and exit BIOS.

Note: Network booting only works with the NUC's *built-in* ethernet, netbooting via
USB-ethernet dongle is unsupported.

If you want to remotely manage the device, see [Remote Management for NUC](nuc-remote-management.md).

## 4. Build Fuchsia {#build-fuchsia}

1. Follow the [getting started guidelines](/docs/get-started/README.md). Make sure to
use the board configuration `x64` when running `fx set`. For example `fx set core.x64`.

## 5. Pave Fuchsia {#pave-fuchsia}

1. Plug your USB key into your build workstation.
1. Identify the path to your USB key by running `fx list-usb-disks`.
1. Create a Zedboot USB by running `fx mkzedboot /path/to/usb/disk`.
1. Plug the Zedboot USB key into the NUC and boot it.
1. When Zedboot is started, press Alt+F3 to switch to a command line prompt.
1. Run `lsblk` on the device. Take note of the HDD or SSD's block device path.
    1. An example path looks like `/dev/sys/pci/00:17.0/ahci/sata0/block`
1. Run `install-disk-image init-partition-tables --block-device <BLOCK_DEVICE_PATH>` on the device to
   wipe and initialize the partition tables on the NUC. Use the block device path from the previous
   step.
1. Run `fx serve` on your workstation to install Fuchsia on the NUC.
1. After paving is completed, disconnect the USB key.
