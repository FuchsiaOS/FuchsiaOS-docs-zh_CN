# Install Fuchsia on a NUC

This guide provides instructions on how to install Fuchsia on an
Intel [NUC][nuc-wiki]{:.external} (Next Unit of Computing) device.

The steps are:

1. [Prerequisites](#prerequisites).
1. [Build Fuchsia](#build-fuchsia).
1. [Prepare a USB drive](#prepare-usb).
1. [Enable EFI booting on the NUC](#enable-efi-booting).
1. [Install Fuchsia on the NUC](#install-fuchsia).
1. [(Optional) Upload Fuchsia SSH keys to the NUC](#upload-fuchsia-ssh-keys).

## Prerequisites {:#prerequisites .numbered}

Before you start installing Fuchsia on a NUC device, make sure that
you've completed the following tasks:

* [Set up the Fuchsia development environment](#set-up-fuchsia-env)
* [Get parts](#get-parts)

### Set up the Fuchsia development environment {#set-up-fuchsia-env}

To set up the Fuchsia development environment on your workstation,
complete the [Get started with Fuchsia][get-started-with-fuchsia] guide.

### Get parts {#get-parts}

Note: Fuchsia only supports the NUC configurations listed in
[Supported system configurations][supported-sys-config]. However,
unsupported NUC configurations may also work with Fuchsia. For more information
on experimental setups, see [Experimental hardware][experimental-hardware].

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

## Build Fuchsia {:#build-fuchsia .numbered}

Installing Fuchsia on a NUC device requires that you build a Workstation
image (`workstation_eng.x64`) and generate build artifacts (which include
the Fuchsia installer) on your workstation.

To build Fuchsia for NUC installation, do the following:

1. Set your build configuration to `workstation_eng.x64` and include the
   recovery package (`recovery-installer`):

   ```posix-terminal
   fx set workstation_eng.x64 --with //build/images/recovery:recovery-installer
   ```

1.  Build Fuchsia:

    ```posix-terminal
    fx build
    ```

    Building Fuchsia can take up to 90 minutes.

## Prepare a USB drive {:#prepare-usb .numbered}

You need to prepare a bootable USB drive that runs the Fuchsia installer.
Later in the [Install Fuchsia on the NUC](#install-fuchsia) section,
you will use this USB drive to boot your NUC into the Fuchsia installer.

Important: The instructions below require that you've completed the
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
   fx mkinstaller -v {{ "<var>" }}PATH_TO_USB_DRIVE{{ "</var>" }}
   ```

   Replace `PATH_TO_USB_DRIVE` with the path to the USB drive from the step
   above.

   The example command below selects the `/dev/sda` path:

   ```none {:.devsite-disable-click-to-copy}
   $ fx mkinstaller -v /dev/sda
   ```

   When finished, the command prints output similar to the following
   in the end:

   ```none {:.devsite-disable-click-to-copy}
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

## Enable EFI booting on the NUC {:#enable-efi-booting .numbered}

Update your NUC's BIOS setup so that it can boot from
a USB drive.

Two versions of BIOS are available on NUC devices: **Visual BIOS** (for instance, NUC7)
and **Aptio V BIOS** (for instance, NUC11). The steps are slightly different depending
on which BIOS is included in your system.

Important: To be able to enter the BIOS setup mode, you may need to unplug the
Ethernet cable from the NUC device if it's already connected to the host machine.

* {Visual BIOS}

   ![Visual BIOS](images/visual_bios.png "Screenshot showing Visual BIOS"){: width="700"}

   **Figure 1**. A screenshot of Visual BIOS

   To enable EFI (Extensible Firmware Interface) booting on your NUC,
   do the following:

   1. Reboot your NUC.
   1. To enter the BIOS setup, press `F2` while booting.
   1. Click the **Advanced** button at the top.
   1. Click the **Boot** tab.
   1. Click the **Boot Priority** tab and confirm the following settings:

      -  **UEFI Boot** is checked.
      -  **Legacy Boot** is unchecked.

     ![Visual BIOS Boot Priority tab](images/visual_bios_boot_priority.png "Screenshot showing the Boot Priority tab in Visual BIOS"){: width="700"}

   1. Click the **Boot Configuration** tab and confirm the following settings:

      -  In the **UEFI Boot** window:

         -  **Boot USB Devices First** is checked.
         -  **Boot Network Devices Last** is checked.
         -  **Unlimited Network Boot Attempts** is checked.

      -  In the **Boot Devices** window:

         -  **USB** is checked.
         -  **Network Boot** is set to `UEFI PXE & iSCSI`.

      ![Visual BIOS Boot Configuration](images/visual_bios_boot_configuration.png "Screenshot showing the Boot Configuration tab in Visual BIOS"){: width="400"}

   1. Click the **Secure Boot** tab and confirm the following settings:

      -  **Secure Boot** is unchecked.

      ![Visual BIOS Secure Boot](images/visual_bios_secure_boot.png "Screenshot showing the Secure Boot tab Visual BIOS"){: width="400"}

   1. To save and exit BIOS, press `F10` and click **Yes**.

* {Aptio V BIOS}

   ![Aptio V BIOS](images/aptio_v_bios.png "Screenshot showing Aptio V BIOS"){: width="700"}

   **Figure 2**. A screenshot of Aptio V BIOS

   To enable EFI (Extensible Firmware Interface) booting on your NUC,
   do the following:

   1. Reboot your NUC.
   1. To enter the BIOS setup, press `F2` while booting.
   1. Click the **Boot** tab.
   1. Click **Secure Boot** and confirm the following settings:

      -  **Secure Boot** is set to `Disabled`.

      ![Aptio V BIOS Secure Boot](images/aptio_v_bios_secure_boot.png "Screenshot showing the Secure Boot tab in Aptio V BIOS"){: width="500"}

   1. To return, click the **<** button on the left.
   1. Click **Boot Priority** and confirm the following settings:

      -  **UEFI Boot** is checked.
      -  **Legacy Boot** is unchecked.

         Note: If you don't see the **UEFI Boot** and **Legacy Boot** options, it means that
         your system does not support legacy boot. Skip these first two checks.

      -  **Boot USB Devices First** is checked.
      -  **Boot Network Devices Last** is checked.
      -  **Unlimited Boot to Network Attempts** is checked.
      -  **USB** is checked.
      -  **Network Boot** is set to `UEFI PXE & iSCSI`.

      ![Aptio V BIOS Boot Priority](images/aptio_v_bios_boot_priority.png "Screenshot showing the Boot priority tab in Aptio V BIOS"){: width="500"}

   1. To save and exit BIOS, press `F10` and click **Ok**.

## Install Fuchsia on the NUC {:#install-fuchsia .numbered}

Use the [bootable USB drive](#prepare-usb) to boot your NUC into
the Fuchsia installer. It then installs the Workstation image
(which was built in the [Build Fuchsia](#build-fuchsia) section) to the NUC.

To install Fuchsia on your NUC, do the following:

1. Plug the bootable USB drive into the NUC.

1. Reboot your NUC.

   The NUC boots into the Fuchsia Workstation Installer (with a pink background).

1. Press **Enter** to select the `Install from USB` option.

1. Press **Enter** on other prompts to continue.

1. Once the installation completes, unplug the USB drive from the NUC device.

1. Reboot the NUC device.

   The NUC is now booted into Fuchsia’s Workstation.

Note: Later, if you need to install a new version of Fuchsia (for instance, after
re-building a new Workstation image using `fx build`), see
[Flash a new Fuchsia image to the NUC](#flash-fuchsia) in Appendices.

## (Optional) Upload Fuchsia SSH keys to the NUC {:#upload-fuchsia-ssh-keys .numbered}

If you plan on using this NUC device **for Fuchsia development**, you need
to flash a Fuchsia image to the NUC device from your host machine, which
in turn uploads the [Fuchsia-specific SSH keys][fuchsia-ssh-keys] to the NUC.
Once those Fuchsia-specific SSH keys are uploaded to the NUC, you can perform
[`ffx`-based  workflows][ffx-workflows] on the NUC from your host machine.

To upload Fuchsia SSH keys to the NUC, do the following:

1. Complete the steps in the
   [Flash a new Fuchsia image to the NUC](#flash-fuchsia) section
   in Appendices.

1. To verify that you can connect to the NUC from the host machine,
   run the following command:

   ```posix-terminal
   ffx target show
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx target show
   Target:
       Name: "fuchsia-54b2-0389-644b"
       SSH Address: "fe81::55b1:2ff2:fe34:567b%en10:22"
   Board:
       Name: "default-board"
       Revision: "1"
       Instruction set: "x64"
   ...
   ```

## Appendices

### Supported NUC models {#supported-nuc-models}

For GPU support, get a NUC7 (Kaby Lake) or NUC8 (Coffee Lake), or a higher
generation.

The list below shows some example models:

 * [Intel® NUC Kit NUC7i5DNKE][NUC7i5DNKE]{:.external}
 * [Intel® NUC Kit NUC7i5DNHE][NUC7i5DNHE]{:.external} (Best choice)
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

1. Connect the NUC directly to the workstation using an Ethernet cable.

   (Or you can also connect the NUC to a router or WiFi modem in the same
   Local Area Network as the workstation.)

   Note: Network booting only works with the NUC's built-in Ethernet port.
   Netbooting with an USB port (via an Ethernet-to-USB adapter) is not supported.

1. Reboot your NUC.

1. On Fuchsia's boot screen, press the `f` key to select the `fastboot` option.

   Once the NUC is in Fastboot mode, it prints `Fastboot TCP is ready`
   on the screen.

1. **On your workstation**, discover the NUC in Fastboot mode:

   ```posix-terminal
   ffx target list
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx target list
   NAME                      SERIAL       TYPE       STATE       ADDRS/IP                           RCS
   fuchsia-54b2-0389-644b    <unknown>    Unknown    Fastboot    [fe81::55b1:2ff2:fe34:567b%en10]    N
   ```

   Verify that the device's state is `Fastboot`.

1. Flash a new Fuchsia image to the NUC:

   Note: To build a new Fuchsia image, see the [Build Fuchsia](#build-fuchsia) section above.

   ```posix-terminal
   fx flash
   ```

   If you have multiple devices connected to the host machine previously, you may need to
   explicitly specify the name of the NUC device, for example:

   ```posix-terminal
   fx flash -s fuchsia-54b2-0389-644b
   ```

   When finished, the NUC reboots and starts running the new Fuchsia image.

### Remote management of NUC devices {:#remote-management-of-nuc-devices}

To enable remote management, including KVM, you need to configure
Intel [AMT][amt]{:.external} (Active Management Technology).

Note: This assumes you're using NUC connected to the EdgeRouter. If
your networking setup is different, you may need a different network
configuration.

First, configure Intel ME on your NUC:

1. Reboot your NUC.
1. Enter Intel ME settings by pressing `Ctrl+P` on the boot screen.
1. Select **MEBx Login**
1. Set up a new password, the default one is `admin`.

   Note: The password must be at least 8 characters long, contain both lowercase and
   uppercase characters, at least one digit and at least one non alphanumeric
   character ("_" is considered alphanumeric).

   Tip: If you choose a password that is exactly 8 characters long, you can use the same password
   as the VNC password below.

1. Configure network:

   1. Select **Intel(R) AMT Configuration**.
   1. Unconfigure existing network settings:

      1. Select **Unconfigure Network Access**
      1. Select **Full Unprovision**
      1. Press `Y` to confirm.
   1. Select **Network Setup** > **TCP/IP Settings** > **Wired LAN IPV4 Configuration**.
   1. Set **DHCP Mode** to **Disabled**.
   1. Set **IPV4 Address** to an address reachable from your host machine via the EdgeRouter.

      On your host machine, run `ifconfig` and find the entry that corresponds to the EdgeRouter, for example:

      ``` none {:.devsite-disable-click-to-copy}
      $ ifconfig
      enx00e04c0c13ba: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500
              inet 192.168.42.86  netmask 255.255.255.0  broadcast 192.168.42.255
              ...
      ```

      In this case, you could try using the address **192.168.42.20**

   1. Set **Subnet Mask Address** to the netmask of your host machine to EdgeRouter connection, for example **255.255.255.0**.
   1. Press `Esc` until you return to **Intel(R) AMT Configuration**.
   1. Select **Activate Network Access** and press `Y` to confirm.
   1. Exit Intel ME settings and save your changes.

Now, configure the [`amtctrl`][amtctrl]{:.external} command-line utility on your host machine:

These instructions assume you have set some environment variables:

 * `AMT_HOST`: The IPv4 address you configured in the Intel ME settings.
 * `AMT_PASSWORD`: The password you chose for Intel ME.
 * `VNC_PASSWORD`: A password for accessing the NUC over VNC.

Note: The password used for `VNC_PASSWORD` must be _exactly_ 8 characters long,
must contain both lowercase and uppercase characters, at least one digit and
at least one non alphanumeric character.

1. Clone the `amtctrl` repository:

   ```posix-terminal
   git clone https://github.com/sdague/amt
   ```

1. Install `amtctrl`:

   ```posix-terminal
   cd amt && sudo ./setup.py install
   ```

1. Configure NUC IP address and passwords:

   ```posix-terminal
   amtctrl set -V $VNC_PASSWORD nuc $AMT_HOST $AMT_PASSWORD
   ```

1. Enable VNC:

   ```posix-terminal
   amtctrl nuc vnc
   ```

Now, you can access the NUC from your host machine using any VNC client by connecting to
the IP address set in `AMT_HOST`. Enter the password set in `VNC_PASSWORD` when prompted.

Note: The NUC needs to be plugged in to a monitor with a HDMI cable to accept VNC connections.

You can also turn on, turn off or reboot the NUC with the following terminal commands:

 * To turn on the NUC:

   ```posix-terminal
   amtctrl nuc on
   ```

 * To turn off the NUC:

   ```posix-terminal
   amtctrl nuc off
   ```

 * To reboot the NUC:

   ```posix-terminal
   amtctrl nuc reboot
   ```

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
[experimental-hardware]: /docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#experimental-hardware
[amt]: https://www.intel.com/content/www/us/en/architecture-and-technology/intel-active-management-technology.html
[amtctrl]: https://github.com/sdague/amt
