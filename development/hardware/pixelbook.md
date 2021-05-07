# Install Fuchsia on Pixelbook

## Prerequisites
Ensure that you have a chromebook build for Fuchsia. Note that Chromebook is now a
distinct board configuration from other x64 devices. See [Paving](/docs/development/hardware/paving.md#building) for
more information.

## Update ChromeOS

If your Pixelbook has never been booted, it is best to boot it normally to check
for any critical updates, as follows:

1. Boot the Pixelbook normally. Opening the lid usually powers on the device.
If this doesn't work, the power button is on the left side of the device, near
the front of the wrist rest.
2. Tap the "Let's go" button.
3. Connect to a wired or wireless network.
4. Accept the terms to proceed to the update check step.
5. The device should check for updates, install any found.
6. After rebooting from any updates, tap 'Browse as Guest' in the lower left
corner.
7. From the browser UI, go into "Settings->About Chrome OS" or "Help->About Chrome
OS" and confirm the version is &gt;=62.

## Put your device into developer mode
***WARNING: This will erase any state stored locally on your Pixelbook***

1. Power off the Pixelbook.
2. Go into Recovery Mode.
Hold down Esc+Refresh (first and third buttons on the top row of the keyboard).
Then press the Power button (bottom left side of the device).
3. Start by disabling OS verification by pressing Ctrl+D. You should see "To turn OS verification OFF, press ENTER". Press Enter to confirm.
4. When your device reboots, you'll get confirmation that OS verification is OFF. Press Ctrl+D again to enter Developer Mode.
5. Wait for the device to re-configure itself, which will take several minutes.
Initially it may not appear to be doing anything. Let the device sit for a
minute or two. You will hear two loud &lt;BEEP&gt;s early in the process. The
process is complete when you hear two more loud &lt;BEEP&gt;s.
6. The device should reboot itself when the Developer Mode transition is
complete. You can now jump to Step #2 in the "Boot from USB" section.

## Boot from USB

1. Boot into ChromeOS.
2. You should see a screen that says "OS verification is OFF" and approximately
30 seconds later the boot will continue. Wait for the Welcome or Login screen
to load. **Ignore** any link for "Enable debugging features".
3. Press Ctrl+Alt+Refresh/F3 to enter a command shell. If pressing this key
combination has no effect, try rebooting the Pixelbook once more.
4. Enter 'chronos' as the user with a blank password
5. Enable USB booting by running `sudo crossystem dev_boot_usb=1`
6. (optional) Default to USB booting by running `sudo crossystem dev_default_boot=usb`.
7. Plug the USB drive into the Pixelbook.
8. Reboot by typing `sudo reboot`
9. On the "OS verification is OFF" screen press Ctrl+U to bypass the timeout and
boot from USB immediately. (See [Tips and Tricks](#tips-and-tricks) for other
short circuit options)

The USB drive is only needed for booting when you want to re-pave or otherwise
netboot the device. If you didn't make USB booting the default (Step #6), you
will need to press Ctrl+U at the grey 'warning OS-not verified' screen to boot
from USB when you power on your device. If the device tries to boot from USB,
either because that is the default or you pressed Ctrl+U, and the device fails
to boot from USB you'll hear a fairly loud &lt;BEEP&gt;. Note that ChromeOS
bootloader USB enumeration during boot has been observed to be slow. If you're
having trouble booting from USB, it may be helpful to remove other USB devices
until the device is through the bootloader and also avoid using a USB hub.

## Tips and Tricks {#tips-and-tricks}

By default the ChromeOS bootloader has a long timeout to allow you to press
buttons. To shortcut this you can press Ctrl+D or Ctrl+U when on the grey screen
that warns that the OS will not be verified. Ctrl+D will cause the device to
skip the timeout and boot from its default source. Ctrl+U will skip the timeout
and boot the device from USB.

### Going back to ChromeOS

To go back to ChromeOS you must modify the priority of the Fuchsia kernel
partition to be lower than that of at least one of the two ChromeOS kernel
partitions.

1. Press Alt+Esc to get to a virtual console if not already on one
2. Press Alt+Fullscreen to get to a terminal emulator on Fuchsia
3. Find the disk that contains the KERN-A, KERN-B, and KERN-C partitions with
the `lsblk` command. Below this is device 000, note how the device path of the
kernel partitions is an extension of that device.

        $ lsblk
        ID  SIZE TYPE             LABEL                FLAGS  DEVICE
        000 232G                                              /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block
        001   5G data             STATE                       /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-000/block
        002  16M cros kernel      KERN-A                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-001/block
        003   4G cros rootfs      ROOT-A                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-002/block
        004  16M cros kernel      KERN-B                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-003/block
        005   4G cros rootfs      ROOT-B                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-004/block
        006  64M cros kernel      KERN-C                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-005/block
        007   4G cros rootfs      ROOT-C                      /dev/sys/pci/00:1e.4/pci-sdhci/sdhci/sdmmc/block/part-006/block
4. Use the `gpt` command to look at the device's (eg. 000) partition map.

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
5. Note that KERN-A and KERN-B typically have ChromeOS kernels. The
   Zircon kernel appears as KERN-C as in the example above, or as
   ZIRCON-A instead in certain setups.

   To go to ChromeOS, lower the priority of KERN-C (or ZIRCON-A)
   by referencing the **partition** index on the **disk** that has
   that partition. E.g.:

        $ gpt edit_cros 5 -P 0 /dev/class/block/000

6. Reboot.

7. When the ChromeOS bootloader appears, press Space to re-enable
OS Verification. Your device will reboot. This time, it will display
a message with "Your system is repairing itself. Please wait". This operation
will take around 5 minutes, after which the PixelBook will reboot one final
time. The device will reboot to the initial setup screen.

To go back to the Fuchsia kernel, just re-pave the device.
