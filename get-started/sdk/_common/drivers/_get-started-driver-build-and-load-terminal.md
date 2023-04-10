The Fuchsia emulator (launched in the [Start the emulator](#start-the-emulator)
section above) is configured to create a virtual device named
[`edu`][edu-device]{:.external}, which  is an educational device for writing
drivers. In the previous section, when the emulator started, Fuchsia’s driver
framework detected this `edu` device in the system, but it wasn’t able to find
a driver that could serve the `edu` device. So the `edu` device was left unmatched.

In this section, we'll build and publish the [`qemu_edu`][qemu-edu]{:.external}
sample driver (which is a Fuchsia component). Upon detecting a new driver, the
driver framework will discover that this new `qemu_edu` driver is a match for
the `edu` device. Once matched, the `qemu_edu` driver starts providing the `edu`
device’s services (capabilities) to other components in the system – one of the
services provided by the `edu` device is that it computes a factorial given
an integer.

The tasks include:

*   View the drivers that are currently loaded in the emulator instance.
*   Build and publish the `qemu_edu` driver component.
*   Verify that the `qemu_edu` driver is loaded to the emulator instance.
*   View detailed information on the `qemu_edu` component.

Do the following:

1. View the list of the currently loaded drivers:

   ```posix-terminal
   tools/ffx driver list --loaded
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx driver list --loaded
   fuchsia-boot:///#meta/block.core.cm
   fuchsia-boot:///#meta/bus-pci.cm
   fuchsia-boot:///#meta/display.cm
   fuchsia-boot:///#meta/fvm.cm
   fuchsia-boot:///#meta/goldfish-display.cm
   fuchsia-boot:///#meta/goldfish.cm
   fuchsia-boot:///#meta/goldfish_address_space.cm
   fuchsia-boot:///#meta/goldfish_control.cm
   fuchsia-boot:///#meta/goldfish_sensor.cm
   fuchsia-boot:///#meta/goldfish_sync.cm
   fuchsia-boot:///#meta/hid-input-report.cm
   fuchsia-boot:///#meta/hid.cm
   fuchsia-boot:///#meta/intel-hda.cm
   fuchsia-boot:///#meta/intel-rtc.cm
   fuchsia-boot:///#meta/netdevice-migration.cm
   fuchsia-boot:///#meta/network-device.cm
   fuchsia-boot:///#meta/pc-ps2.cm
   fuchsia-boot:///#meta/platform-bus-x86.cm
   fuchsia-boot:///#meta/platform-bus.cm
   fuchsia-boot:///#meta/qemu-audio-codec.cm
   fuchsia-boot:///#meta/ramdisk.cm
   fuchsia-boot:///#meta/sysmem.cm
   fuchsia-boot:///#meta/virtio_block.cm
   fuchsia-boot:///#meta/virtio_ethernet.cm
   fuchsia-boot:///#meta/virtio_input.cm
   fuchsia-pkg://fuchsia.com/virtual_audio#meta/virtual_audio_driver.cm
   fuchsia-boot:///#meta/ahci.cm
   ```

2. Build and publish the `qemu_edu` driver component:

   ```posix-terminal
   tools/bazel run //src/qemu_edu/drivers:pkg.component
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel run //src/qemu_edu/drivers:pkg.component
   ...
   INFO: Build completed successfully, 1045 total actions
   Running workflow: pkg.component_base
   Running task: pkg.debug_symbols_base (step 1/2)
   Running task: pkg.component.run_base (step 2/2)
   added repository bazel.pkg.component.runnable
   Registering fuchsia-pkg://bazel.pkg.component.runnable/qemu_edu#meta/qemu_edu.cm
   Successfully bound:
   Node 'root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl', Driver 'fuchsia-pkg://bazel.pkg.component.runnable/qemu_edu#meta/qemu_edu.cm'.
   ```

3. Verify that the `qemu_edu` driver is now loaded to the Fuchsia emulator
   instance:

   ```posix-terminal
   tools/ffx driver list --loaded
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx driver list --loaded
   fuchsia-boot:///#meta/bus-pci.cm
   fuchsia-boot:///#meta/display.cm
   fuchsia-boot:///#meta/fvm.cm
   fuchsia-boot:///#meta/goldfish-display.cm
   fuchsia-boot:///#meta/goldfish.cm
   fuchsia-boot:///#meta/goldfish_address_space.cm
   fuchsia-boot:///#meta/goldfish_control.cm
   fuchsia-boot:///#meta/goldfish_sensor.cm
   fuchsia-boot:///#meta/goldfish_sync.cm
   fuchsia-boot:///#meta/hid-input-report.cm
   fuchsia-boot:///#meta/hid.cm
   fuchsia-boot:///#meta/intel-hda.cm
   fuchsia-boot:///#meta/intel-rtc.cm
   fuchsia-boot:///#meta/netdevice-migration.cm
   fuchsia-boot:///#meta/network-device.cm
   fuchsia-boot:///#meta/pc-ps2.cm
   fuchsia-boot:///#meta/platform-bus-x86.cm
   fuchsia-boot:///#meta/platform-bus.cm
   fuchsia-boot:///#meta/qemu-audio-codec.cm
   fuchsia-boot:///#meta/ramdisk.cm
   fuchsia-boot:///#meta/sysmem.cm
   fuchsia-boot:///#meta/virtio_block.cm
   fuchsia-boot:///#meta/virtio_ethernet.cm
   fuchsia-boot:///#meta/virtio_input.cm
   fuchsia-pkg://fuchsia.com/virtual_audio#meta/virtual_audio_driver.cm
   {{ '<strong>' }}fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm{{ '</strong>' }}
   fuchsia-boot:///#meta/ahci.cm
   fuchsia-boot:///#meta/block.core.cm
   ```

   Notice that the `qemu_edu` driver is shown in the loaded drivers list.

4. View the `qemu_edu` component information:

   ```posix-terminal
   tools/ffx component show qemu_edu.cm
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx component show qemu_edu.cm
                  Moniker:  /bootstrap/full-pkg-drivers:root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl
                      URL:  fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm
              Instance ID:  None
                     Type:  CML Component
          Component State:  Resolved
    Incoming Capabilities:  /svc/fuchsia.device.fs.Exporter
                            /svc/fuchsia.driver.compat.Service
                            /svc/fuchsia.logger.LogSink
     Exposed Capabilities:  examples.qemuedu.Service
              Merkle root:  ca337aa579388a7335c8fa53e47ba111b6a58c0b9af7519731e9942dec31f7ef
          Execution State:  Running
             Start reason:  Instance is in a single_run collection
    Outgoing Capabilities:  examples.qemuedu.Service
   ```

5. View the device logs of the `qemu-edu` driver:

   ```posix-terminal
   tools/ffx log --tags qemu-edu dump
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --tags qemu-edu dump
   2022-10-27 21:19:30.189][<ffx>]: logger started.
   [184.040][full-pkg-drivers:root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl][qemu-edu,driver][I]: [src/qemu_edu/drivers/qemu_edu.cc:65] edu device version major=1 minor=0
   [184.073][full-pkg-drivers:root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl][qemu-edu,driver][I]: [src/qemu_edu/drivers/qemu_edu.cc:117] Exported devfs_path=sys/platform/pt/PCI0/bus/00:06.0_/qemu-edu service_path=examples.qemuedu.Service/default/device
   ```

<!-- Reference links -->

[edu-device]: https://fuchsia.googlesource.com/third_party/qemu/+/refs/heads/main/docs/specs/edu.txt
[qemu-edu]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/
