# Flash a Fuchsia image on a device

The [`ffx target flash`][ffx-target-flash] command can flash a Fuchsia image
on a device.

## Concepts

Fuchsia uses a Fastboot-based flashing mechanism to install a Fuchsia product
on a hardware device. The [Fastboot protocol][fastboot-src]{:.external}
(originally part of [Android][android-flash]{:.external}) is a mechanism for
communicating with bootloaders over USB or Ethernet. This mechanism allows you
to flash a Fuchsia prebuilt image on the device's non-volatile memory.

To be able to flash a Fuchsia image on a device, the bootloader of the device
must support Fastboot mode. Once the device can boot into Fastboot
mode, you can then use `ffx target flash` to flash a Fuchsia image on the
device. However, if your device's bootloader doesn't support Fastboot, you'd
first need to update the bootloader. Updating a device's bootloader (to support
Fastboot) typically requires instructions that are specific to the type and
maker of the device, which is not covered in this guide.

Fuchsia prebuilt images can be obtained from various sources, such as Google
Cloud Storage and project repositories. Additionally,
[custom prebuilt images][generate-a-build] can be generated from a Fuchsia
source checkout. In either case, the prebuilt image used for flashing must
match the target device.

## Flash the device {:#flash-the-device}

To flash a Fuchsia image on your device, do the following:

1. Connect the device to the host machine over USB or Ethernet.
2. [Boot the device into Fastboot
   mode](#boot-the-device-into-fastboot-mode).

3. Check the device's state:

   ```posix-terminal
   ffx target list
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx target list
   NAME         SERIAL            TYPE       STATE       ADDRS/IP    RCS
   <unknown>    01234ABCD012YZ    Unknown    Fastboot    []          N
   ```

   Verify that the device's state is `Fastboot`.

4. Flash the device:

   ```posix-terminal
   ffx target flash <FUCHSIA_IMAGE>
   ```

   Replace `FUCHSIA_IMAGE` with an archive file that contains
   a Fuchsia prebuilt image and its flash manifest file, for example:


   ```none {:.devsite-disable-click-to-copy}
   $ ffx target flash ~/Downloads/fuchsia-image-example.zip
   ```

   Once the flashing is finished, the device reboots and starts running
   Fuchsia.

## Boot the device into Fastboot mode {:#boot-the-device-into-fastboot-mode}

To trigger a Fuchsia device to boot into Fastboot mode, run the following
command:

Note: This command works only if your device is already flashed with Fuchsia.
Manually booting a device into Fastboot mode may require instructions
specific to the device.

```posix-terminal
ffx target reboot -b
```

After rebooting, the device boots into Fastboot mode.

<!-- Reference links -->

[fastboot-src]: https://android.googlesource.com/platform/system/core/+/master/fastboot/
[android-flash]: https://source.android.com/setup/build/running
[ffx-target-flash]: https://fuchsia.dev/reference/tools/sdk/ffx#flash
[generate-a-build]: /docs/development/build/fx.md#generating-a-build-archive
