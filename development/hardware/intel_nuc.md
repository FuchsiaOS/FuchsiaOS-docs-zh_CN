# Install Fuchsia on a NUC

This guide provides instructions on how to install Fuchsia on an
Intel [NUC][nuc-wiki]{:.external} (Next Unit of Computing) device.

The steps are:

1. [Prerequisites](#prerequisites).
1. [Build Fuchsia](#build-fuchsia).
1. [Prepare a USB drive](#prepare-usb).
1. [Enable EFI booting on the NUC](#enable-efi-booting).
1. [Install Fuchsia on the NUC](#install-fuchsia).

## 1. Prerequisites {#prerequisites}

Before you start installing Fuchsia on a NUC device, make sure that
you've completed the following tasks:

* [Set up the Fuchsia development environment](#set-up-fuchsia-env)
* [Get parts](#get-parts)

### Set up the Fuchsia development environment {#set-up-fuchsia-env}

To set up the Fuchsia development environment on your workstation,
complete the [Get started with Fuchsia][get-started-with-fuchsia] guide.

### Get parts {#get-parts}

Note: Fuchsia only supports the specific system configurations listed in
[Supported system configurations][supported-sys-config].

The following parts are required for this guide:

*  A NUC device (see [example models](#supported-nuc-models))
*  A USB 3.0 flash drive
*  A keyboard
*  A mouse (Optional)
*  A monitor with an HDMI port
*  An HDMI cable
*  An Ethernet cable
*  A Phillips-head screwdriver (with a magnetic tip)

Note: The [_2. Build Fuchsia_](#build-fuchsia) and
[_3. Prepare a USB drive_](#prepare-usb) sections do not require a NUC
device, so you can complete these sections prior to obtaining a NUC device.
However, you will need a USB flash drive for the _3. Prepare a USB drive_
section.

## 2. Build Fuchsia {#build-fuchsia}

Installing Fuchsia on a NUC device requires that you build a Workstation
image (`workstation.x64`) and generate build artifacts (which include
the Fuchsia installer) on your workstation.

To build Fuchsia for NUC installation, do the following:

1. Set your build configuration to `workstation.x64` and include the
   recovery package (`recovery-installer`):

   ```posix-terminal
   fx set workstation.x64 --with //build/images/recovery:recovery-installer
   ```

1.  Build Fuchsia:

    ```posix-terminal
    fx build
    ```

    Building Fuchsia can take up to 90 minutes.

## 3. Prepare a USB drive {#prepare-usb}

You need to prepare a bootable USB drive that runs the Fuchsia installer.
Later in the [Install Fuchsia on the NUC](#install-fuchsia) section,
you will use this USB drive to boot your NUC into the Fuchsia installer.

Note: The instructions below require that you've completed the
build in the previous [Build Fuchsia](#build-fuchsia) section.

To prepare a bootable USB drive, do the following:

1. Plug the USB drive into **your workstation**.

1. Identify the path to the USB drive:

   ```posix-terminal
   fx list-usb-disks
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ fx list-usb-disks
   /dev/sda - My Example USB Disk
   ```

1. Create a bootable USB drive:

   ```posix-terminal
   fx mkinstaller -v --new-installer {{ "<var>" }}PATH_TO_USB_DRIVE{{ "</var>" }}
   ```

   Replace `PATH_TO_USB_DRIVE` with the path to the USB drive from the step
   above.

   The example command below selects the `/dev/sda` path:

   ```none {:.devsite-disable-click-to-copy}
   $ fx mkinstaller -v --new-installer /dev/sda
   ```

   When finished, the command prints output similar to the following
   in the end:

   ```none {:.devsite-disable-click-to-copy}
   $ fx mkinstaller -v --new-installer /dev/sda
   mkinstaller: WARNING: Changing ownership of /dev/sda to alice
   [sudo] password for alice:
   ...
   mkinstaller: INFO:    Writing image fvm.sparse.blk to partition storage-sparse...
   mkinstaller: INFO:      Wrote 835.6M in 35.55s, 23.5M/s
   mkinstaller: INFO: Done.
   mkinstaller: INFO: Ejected USB disk
   ```

1. Unplug the USB drive from the workstation.

## 4. Enable EFI booting on the NUC {#enable-efi-booting}

Update your NUC's BIOS setup so that it can boot from
a USB drive.

To enable EFI (Extensible Firmware Interface) booting on your NUC,
do the following:

1. Reboot your NUC.
1. To enter the BIOS setup, press `F2` while booting.
1. In the **Boot Order** window on the left, click the **Legacy** tab.
1. Uncheck **Legacy Boot**.

   <img width="40%" src="/docs/images/developing_on_nuc/bios.jpg"/>
1. Click the **Advanced** button.
1. Confirm the following boot configuration:
    * Under the **Boot Priority** tab:
       * **UEFI Boot** is checked.
    * Under the **Boot Configuration** tab:
       * In the **UEFI Boot** window:
         * **Boot USB Devices First** is checked.
         * **Boot Network Devices Last** is checked.
         * **Unlimited Network Boot Attempts** is checked.
       * In the **Boot Devices** window:
         * **USB** is checked.
         * **Network Boot** is set to **UEFI PXE & iSCSI**.
    * Under the **Secure Boot** tab:
       * **Secure Boot** is unchecked.
1. To save and exit BIOS, press `F10` and click **Yes**.

## 5. Install Fuchsia on the NUC {#install-fuchsia}

Use the [bootable USB drive](#prepare-usb) to boot your NUC into
the Fuchsia installer. It then flashes the
[Workstation prebuilt image](#build-fuchsia) from your workstation
to the NUC to install Fuchsia for the first time.

To install Fuchsia on your NUC, do the following:

1. Plug the bootable USB drive into the NUC.

1. Reboot your NUC.

   The NUC boots into the Fuchsia Workstation Installer (with a pink background).

1. Press **Enter** to select the `Install from USB` option.

1. Press **Enter** on other prompts to continue.

   When the installation is finished, the screen displays
   `Success! Please restart your computer`.

1. Unplug the USB drive from the NUC device.

1. Reboot the NUC device.

   The NUC is now booted into Fuchsia’s Workstation.

1. Set your login password to start the Fuchsia Workstation.

Later, if you need to install a new version of Fuchsia (for instance, after
re-building a new Workstation image using `fx build`), see the
[Flash a new Fuchsia image to the NUC](#flash-fuchsia) section in Appendices.

Important: If you plan on using this NUC device for Fuchsia development,
you must complete the steps in  the
[Flash a new Fuchsia image to the NUC](#flash-fuchsia) section at least once
after installing Fuchsia from a bootable USB drive. Running `fx flash` will
upload [Fuchsia-specific SSH keys][fuchsia-ssh-keys] to the NUC device, which
then enables other useful [`ffx` workflows][ffx-workflows].

## Appendices

### Supported NUC models {#supported-nuc-models}

For GPU support, get a NUC7 (Kaby Lake) or NUC8 (Coffee Lake), or a higher
generation.

The list below shows some example models:

 * [Intel® NUC Kit NUC7i5DNKE][NUC7i5DNKE]{:.external}
 * [Intel® NUC Kit NUC7i5DNHE][NUC7i5DNHE]{:.external}
 * [Intel® NUC Kit NUC7i3DNKE][NUC7i3DNKE]{:.external}
 * [Intel® NUC Kit NUC7i3DNHE][NUC7i3DNHE]{:.external}
 * [Intel® NUC Kit NUC8i5BEK][NUC8i5BEK]{:.external}
 * [Intel® NUC Kit NUC8i5BEH][NUC8i5BEH]{:.external}
 * [Intel® NUC Kit NUC8i3BEK][NUC8i3BEK]{:.external}
 * [Intel® NUC Kit NUC8i3BEH][NUC8i3BEH]{:.external}

### Flash a new Fuchsia image to the NUC {#flash-fuchsia}

Once a NUC is running Fuchsia, you can use Fuchsia's flashing
mechanism to provision a new Fuchsia image to the NUC.

To flash a Fuchsia image to your NUC, do the following:

1. Connect the NUC directly to the workstation using an Ethernet cable
   (or connect the NUC to a router or WiFi modem in the same
   Local Area Network as the workstation).

   Note: Network booting only works with the NUC's built-in Ethernet port –
   netbooting with an USB port (via an Ethernet-to-USB adapter) is not supported.

1. Reboot your NUC.

1. To boot the NUC into Fastboot mode, press the `f` key at the Fuchsia boot screen.

   Once the NUC is in Fastboot mode, you can see `entering fastboot mode` printed on the
   screen.

1. **On your workstation**, detect the NUC in Fastboot mode:

   ```posix-terminal
   ffx target list
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx target list
   NAME                      SERIAL       TYPE       STATE       ADDRS/IP                           RCS
   fuchsia-54b2-0389-644b    <unknown>    Unknown    Fastboot    [fe81::55b1:2ff:fe34:567b%en10]    N
   ```

   Verify that the device's state is in `Fastboot`.

1. Flash a new Fuchsia image to the NUC:

   Note: To build a new Fuchsia image, see the [Build Fuchsia](#build-fuchsia) section above.

   ```posix-terminal
   fx flash
   ```

   When finished, the NUC reboots and starts running the new Fuchsia image.

   Important: When using this NUC device for Fuchsia development, currently
   for other [`ffx` workflows][ffx-workflows], you can only use USB ports to
   connect the NUC to your host machine. In other words, undo the cable setup in
   Step 1 above, and use **2 Ethernet-to-USB adapters** and an Ethernet cable to
   establish a connection between the NUC and your host machine using only USB ports.

<!-- Reference links -->

[nuc-wiki]: https://en.wikipedia.org/wiki/Next_Unit_of_Computing
[get-started-with-fuchsia]: /docs/get-started/README.md
[usb-setup]: /docs/development/hardware/usb_setup.md
[supported-sys-config]: /docs/reference/hardware/support-system-config.md
[NUC7i5DNKE]: https://ark.intel.com/content/www/us/en/ark/products/122486/intel-nuc-kit-nuc7i5dnke.html
[NUC7i5DNHE]: https://ark.intel.com/content/www/us/en/ark/products/122488/intel-nuc-kit-nuc7i5dnhe.html
[NUC7i3DNKE]: https://ark.intel.com/content/www/us/en/ark/products/122495/intel-nuc-kit-nuc7i3dnke.html
[NUC7i3DNHE]: https://ark.intel.com/content/www/us/en/ark/products/122498/intel-nuc-kit-nuc7i3dnhe.html
[NUC8i5BEK]: https://ark.intel.com/content/www/us/en/ark/products/126147/intel-nuc-kit-nuc8i5bek.html
[NUC8i5BEH]: https://ark.intel.com/content/www/us/en/ark/products/126148/intel-nuc-kit-nuc8i5beh.html
[NUC8i3BEK]: https://ark.intel.com/content/www/us/en/ark/products/126149/intel-nuc-kit-nuc8i3bek.html
[NUC8i3BEH]: https://ark.intel.com/content/www/us/en/ark/products/126150/intel-nuc-kit-nuc8i3beh.html
[ffx]: https://fuchsia.dev/reference/tools/sdk/ffx
[ffx-workflows]: /docs/development/sdk/ffx/index.md
[fuchsia-ssh-keys]: /docs/development/sdk/ffx/create-ssh-keys-for-devices.md
