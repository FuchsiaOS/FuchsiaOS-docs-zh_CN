# Install Fuchsia on a Khadas VIM3 board

This guide shows you how to install Fuchsia on a
[Khadas VIM3](https://www.khadas.com/vim3). The installation
process will probably take between 1 to 3 hours.

Running Fuchsia on VIM3 is useful if you want to explore how Fuchsia works on
relatively low-cost real hardware that supports many kinds of peripheral devices.
See [Appendix: Feature support](#features) for details on which VIM3 features
Fuchsia supports.

If you just want to explore Fuchsia with the lowest friction possible, check out
[Get started with the Fuchsia SDK](/docs/get-started/sdk/index.md) instead.

See [Appendix: Support](#support) if you have any trouble completing
this guide.

## Audience {#audience}

If you've never tinkered with electronics you might find this
guide difficult to complete. For example, this guide assumes that you know
how to hook up serial cable wires to GPIOs to read logs and send commands
over a serial communication program like `minicom`.

This guide also assumes that you're comfortable
with CLI workflows such as building Fuchsia from source.

## Prerequisites {#prerequisites}

You'll need all of the following hardware and software to complete this guide:

* A [Khadas VIM3](https://www.khadas.com/vim3) single-board computer.

  Caution: It's unknown whether Fuchsia will run on the Basic model VIM3.
  This guide was validated with the Pro model VIM3.

* A desktop or laptop computer that's running Linux and has 2 USB ports
  available.

  Key Term: This desktop or laptop is called the **host**
  throughout the rest of this guide.

  Caution: A macOS host may work but these instructions have not been validated
  with macOS. Building Fuchsia on a remote Linux computer and then attempting to flash
  Fuchsia onto the VIM3 with a local macOS host is known to not work.

  <!-- Context from kayce@: Local macOS host + remote Linux workstation doesn't work because
       when you run `fx pave` you get an error about a mismatch between the local
       host OS and the remote workstation OS. -->

  Note: This guide assumes that your Linux distribution has Debian commands
  like `apt`.

* A power supply of at least 24W to your host. The VIM3 can draw that much power when
  [DVFS](https://en.wikipedia.org/wiki/Dynamic_frequency_scaling) is enabled.

* A working Fuchsia development environment on your host. In other words you
  should be able to build Fuchsia from its source code on your host. See
  [Build Fuchsia](#build).

* A [USB to TTL serial cable](https://www.adafruit.com/product/954).

* A USB-C to USB-\* cable that supports both data and power delivery.
  The USB-C side is for the VIM3. The other side can be whatever USB
  type your host supports.

The following is optional:

* A [heatsink](https://www.khadas.com/product-page/new-vim-heatsink).
  This enables running 2 CPU cores on the VIM3 at full speed without
  reaching 80°C, the critical temperature beyond which cores are throttled
  down.

See the [VIM3 collection](https://www.khadas.com/shop?Collection=VIM3&sort=price_descending)
in the Khadas shop for examples of compatible accessories.

Note: All the links in this section are only for your convenience. You
don't need to buy from these exact stores or these exact parts.

## Build Fuchsia {#build}

If you don't already have an [in-tree][glossary.in-tree] environment
set up, you should start the process now because it can take a while to
complete:

1. [Download the Fuchsia source code](/docs/get-started/get_fuchsia_source.md).

1. [Configure and build Fuchsia](/docs/get-started/build_fuchsia.md).

   * When building Fuchsia, use `fx set core.vim3` instead.

Note: The rest of this guide assumes that your Fuchsia source code directory
is located at `~/fuchsia`.

You'll use the Fuchsia development environment to build the Fuchsia image
for VIM3 and run an in-tree CLI tool for flashing the Fuchsia image onto
the VIM3.

## Set up the hardware {#hardware}

Set up the VIM3 to communicate with your host:

1. Connect the VIM3 and your host to each other with the USB-C to USB-\* cable.
   The white LED on the VIM3 should turn on.

   Caution: Don't put a [USB hub](https://en.wikipedia.org/wiki/USB_hub)
   between the VIM3 and your host. The hub may make it harder for your
   VIM3 and host to detect and communicate with each other.

   This connection is used to power and flash the VIM3 with
   [`fastboot`](https://en.wikipedia.org/wiki/Fastboot).

1. Connect the serial cable wires to the VIM3's GPIOs:

   * GND to pin 17.

   * RX (in to VIM3) to pin 18.

   * TX (out from VIM3) to pin 19.

   * Don't connect the power wire of your serial cable to any VIM3 GPIO.
     The VIM3 is getting power through the USB cable.

   Tip: Pins 1, 20, 21, and 40 are labeled on the circuit board.

   Caution: In general the colors for TX and RX wires are not standardized.
   For example your RX wire may be blue or green.

   See [Serial Debugging Tool](https://docs.khadas.com/linux/vim3/SetupSerialTool.html)
   for an example image of how your serial wires should be connected to the VIM3.

### Verify the serial connection {#serial}

Make sure that you can view the logs being sent over the serial cable:

1. Open a terminal in your host and run `ls /dev/ttyUSB*` before connecting the
   serial cable to a USB port on your host.

1. Connect the serial cable to your host and run `ls /dev/ttyUSB*` again.
   There should be 1 more result than the first time you ran the command,
   such as `/dev/ttyUSB0`. That is the USB connection between your VIM3
   and your host. You'll provide this result for the `Serial Device`
   value in the next step.

   If you see no difference when running `ls /dev/ttyUSB*` before and after
   connecting the serial cable, try `ls /dev/tty*` or `ls /dev/*` instead.

1. Install, set up, and launch `minicom` on your host as explained in [Set Up Serial Communication
   Program](https://docs.khadas.com/linux/vim3/SetupSerialTool.html#Setup-Serial-Communication-Program).

   Key Term: In the rest of this guide the terminal window running `minicom` is called
   the **serial console**.

   Note: This guide assumes that you're using `minicom` for your serial communication
   program but you can use whatever program you prefer.

1. Press the reset button on the VIM3. The reset button is the one with the **R**
   printed next to it on the circuit board.
   See [VIM3/3L Hardware](https://docs.khadas.com/linux/vim3/Hardware.html) for
   a diagram. In your serial console you should see human-readable logs.

## Erase the eMMC {#emmc}

In later sections of this guide you'll update the bootloader and
OS on the VIM3. These updates don't work unless you
completely erase the eMMC first:

1. Press the reset button on your VIM3.

1. Right after you press the reset button, start repeatedly pressing the
   <kbd>Space</kbd> key as your VIM3 boots up. Make sure that your cursor
   is focused on your serial console. The bootloader process should pause
   and your serial console should show a `kvim3#` prompt. Your serial
   console is now providing you access to the **U-Boot shell**.

1. Run the following command in the U-Boot shell:

   ```posix-terminal
   store init 3
   ```

   Your serial console logs should verify that the eMMC was correctly erased.

See [Erase eMMC](https://docs.khadas.com/linux/vim3/EraseEmmc.html)
for more details.

## Update the Android image on the VIM3 {#android}

<!-- Context: https://forum.khadas.com/t/unable-to-change-bootloader-for-vim3/12708/6 -->

The Android image that ships by default on the VIM3 does
not support Fuchsia installation. If you just received your VIM3
from Khadas you must update your Android image:

1. Click the following URL to download the updated Android image:
   <https://dl.khadas.com/Firmware/VIM3/Android/VIM3_Pie_V210527.7z>

1. Extract the compressed archive file (`VIM3_Pie_V210527.7z`).
   After the extraction you should have a `VIM3_Pie_V210527` directory
   with an `update.img` file in it.

1. Follow the instructions in [Install OS into
   eMMC](https://docs.khadas.com/linux/vim3/InstallOsIntoEmmc.html).
   When running `aml-burn-tool` the value for the `-i` flag should be the
   path to your `update.img` file. Your command should look similar to this:

   ```posix-terminal
   aml-burn-tool -b VIM3 -i ~/Downloads/VIM3_Pie_V210527/update.img
   ```

   Caution: Make sure that you're following the instructions for Ubuntu
   and VIM3 by clicking the **Install on Ubuntu** and **VIM3/VIM3L** tabs.
   These instructions are not shown by default.

   Tip: The `TST Mode` workflow is probably the easiest and fastest way to get
   your VIM3 into Upgrade Mode.

1. If the white and red LEDs on your VIM3 are off and the blue LED is on,
   it means that your VIM3 is in sleep mode. Try putting your VIM3
   back into [Upgrade Mode](https://docs.khadas.com/linux/vim3/BootIntoUpgradeMode.html)
   and then pressing the reset button again.

At this point the white LED on your VIM3 should be on and you should see
logs in your serial console after you press the reset button on your VIM3.

## Update the bootloader {#bootloader}

Flash Fuchsia's custom bootloader onto the VIM3:

1. Install the [Android SDK Platform
   Tools](https://developer.android.com/studio/releases/platform-tools).

   Installing these tools gives you access to `adb`.

1. Verify that you can now run `adb`:

   ```posix-terminal
   adb --version
   ```

1. Access the U-Boot shell again by pressing the reset button and
   then repeatedly pressing the <kbd>Space</kbd> key in your serial
   console. When your serial console shows the `kvim3#` prompt, you're
   in the U-Boot shell.

1. In your U-Boot shell run the following command:

   ```posix-terminal
   fastboot
   ```

   You should see the following logs in your serial console:

   ```
   g_dnl_register: g_dnl_driver.name = usb_dnl_fastboot

   USB RESET
   SPEED ENUM

   USB RESET
   SPEED ENUM
   ```

   If you see the first line (`g_dnl_register: g_dnl_driver.name = usb_dnl_fastboot`)
   but not the lines after that, try using a different USB-C to USB-\* cable and make
   sure that it supports both data and power delivery.

1. Open a new terminal window in your host and run the following commands:

   ```posix-terminal
   cd ~/fuchsia/prebuilt/third_party/fastboot

   ./fastboot flashing unlock

   ./fastboot flashing unlock_critical

   ./fastboot flash bootloader ~/fuchsia/prebuilt/third_party/firmware/vim3/u-boot.bin.unsigned

   ./fastboot reboot
   ```

   Caution: Installing the Android SDK Platform Tools probably installed another
   instance of `fastboot` on your host. When working with Fuchsia, remember to use
   the [in-tree][glossary.in-tree] version of `fastboot` at
   `~/fuchsia/prebuild/third_party/fastboot/fastboot`. The `fastboot` protocol allows arbitrary
   vendor protocol extensions and Fuchsia may rely on this functionality in the future.

Note: You can also build the custom bootloader (`u-boot.bin.unsigned`) from source:
<https://third-party-mirror.googlesource.com/u-boot/+/refs/heads/vim3>

## Flash Fuchsia into the eMMC {#fuchsia}

Install Fuchsia onto your VIM3:

1. Put your VIM3 into `fastboot` mode by pressing the reset button
   and then immediately pressing the <kbd>F</kbd> key.

1. From a separate terminal on your host run the following command:

   ```posix-terminal
   cd ~/fuchsia

   fx flash --pave
   ```

Your VIM3 is now running Fuchsia!

Repeat the steps in this section whenever you want to flash a new Fuchsia
image onto your VIM3.

## Appendix: Fix a bricked VIM3 {#bricks}

Do these steps if you've [bricked](https://en.wikipedia.org/wiki/Brick_(electronics))
your VIM3 and need to "factory reset" it:

1. [Erase the eMMC](#emmc).
1. [Update the Android image](#android).
1. [Update the bootloader](#bootloader).
1. [Flash Fuchsia into the eMMC](#fuchsia).

## Appendix: Support {#support}

* For issues that seem related to VIM3 hardware or firmware, try the
  [VIM3 official docs](https://docs.khadas.com/linux/vim3/index.html) and
  [Khadas VIM3 official forum](https://forum.khadas.com/c/khadas-vim3/30).
* For issues that seem related to Fuchsia, try the
  [Fuchsia mailing lists and chat rooms](/docs/contribute/community/mailing-lists.md).

## Appendix: Feature support {#features}

Fuchsia currently supports these features of the VIM3:

* UART Serial Debugger
* Paving over ethernet and USB
* Storage (eMMC)
* HDMI Display and Framebuffer
* GPU (Mali) and Vulkan graphics
* Ethernet
* SDIO
* I2C
* GPIO
* Temperature Sensors and DVFS
* RTC
* Clock
* Fan
* NNA
* USB-C in peripheral mode
* USB-A

These features are under development and may not be supported:

* Video decoder
* SPI
* Audio

The following features are not supported, but might be added by future
contributions:

* SPI Flash
* USB-C in host mode
* Power management and PMIC
* Wake on LAN
* UART BT

These features are not supported and are unlikely to be added:

* Video encoding (due to non-public firmware)
* Trusted Execution Environment / secure boot

## Appendix: Update the boot splash screen {#splash}

To update the boot splash screen to be the Fuchsia logo, run the following command
from a host terminal while the VIM3 is in `fastboot` mode:

```posix-terminal
fastboot flash logo ~/fuchsia/zircon/kernel/target/arm64/board/vim3/firmware/logo.img
```