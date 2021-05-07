# Prepare a USB flash drive to be a bootable disk

You can use a USB flash drive to make your device to boot from the freshly-built
OS on your network-connected host development machine. Alternatively, you can also
direct your device to boot from the OS on the flash drive itself.

## Automatic configuration

To prepare a USB flash drive to be a bootable disk for your device, complete the
following steps:

Note: This procedure only enables you to netboot or pave your device, it won't put
anything on your internal storage.

1. Run the following command to set the build configuration:
  <pre class="prettyprint">
  <code class="devsite-terminal">fx set core.x64</code>
  </pre>

1. Run the following command to build the fuchsia image:
  <pre class="prettyprint">
  <code class="devsite-terminal">fx build</code>
  </pre>

1. Run the following command to create a zedboot key, replacing `DEVICE-PATH`
   with the path to your target device:

  Note: To find the `device-path` to your USB drive, you can run `lsblk`.
  If you identify your USB drive as `sda`, your `device-path` is `/dev/sda/`.

  <pre class="prettyprint">
  <code class="devsite-terminal">fx mkzedboot <b>DEVICE-PATH</b></code>
  </pre>

  This command requires that you `sudo` into your machine. As a result, you will
  need to enter your password after running `fx mkzedboot`.

    The `mkzedboot` command does the following:

    + Creates a File Allocation Table (FAT) partition that contains an Extensible
    Firmware Interface (EFI) System Partition. The EFI System Partition contains
    the Gigaboot EFI bootloader and a configuration that specifies that your
    device always boot into Zedboot.
    + Creates a ChromeOS bootable partition with a developer key signed Zedboot
    kernel partition.

1. Connect your device to your host through built-in ethernet.

1. (Optional) To pave your target device with Fuchsia, run:
  <pre class="prettyprint">
  <code class="devsite-terminal">fx pave</code>
  </pre>

1. (Optional) To netboot your target device, run:
  <pre class="prettyprint">
  <code class="devsite-terminal">fx netboot</code>
  </pre>

1. Power on your device.

## Manual configuration

Manually creating an EFI boot key is no longer supported.
