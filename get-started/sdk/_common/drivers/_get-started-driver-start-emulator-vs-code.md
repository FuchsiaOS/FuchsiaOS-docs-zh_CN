Start the [Fuchsia emulator][femu] on the host machine and configure the
emulator instance to use Fuchsia’s new [driver framework][driver-framework]
(DFv2).

The tasks include:

*   Download Fuchsia's Workstation prebuilt image from Google Cloud Storage.
*   Start the Fuchsia emulator.
*   Set the emulator instance as your host machine’s default target device.
*   Start the Fuchsia package server.
*   Register the system package repository to the emulator instance.

In VS Code, do the following:

1. Click **Terminal > New Terminal**.

1. In the terminal, download the latest Workstation image for the emulator:

   ```posix-terminal
   tools/ffx product-bundle get workstation_eng.qemu-x64 --force-repo --repository workstation-packages
   ```

   This command may take a few minutes to download the image and product
   metadata.

   Once the download is finished, the `ffx product-bundle get` command creates
   a local Fuchsia package repository named `workstation-packages` on your host machine.
   This package repository hosts additional system packages for this Workstation prebuilt image.
   Later in step 9 you’ll register this package repository to the emulator instance.

1. Stop all emulator instances:

   ```posix-terminal
   tools/ffx emu stop --all
   ```

1. Start the Fuchsia emulator:

   ```posix-terminal
   tools/ffx emu start workstation_eng.qemu-x64 --headless \
     --kernel-args "driver_manager.use_driver_framework_v2=true" \
     --kernel-args "driver_manager.root-driver=fuchsia-boot:///#meta/platform-bus.cm" \
     --kernel-args "devmgr.enable-ephemeral=true"
   ```

   This command starts a headless emulator instance running the Workstation prebuilt image.

   When the instance is up and running, the command prints output similar to
   the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx emu start workstation_eng.qemu-x64 --headless \
     --kernel-args "driver_manager.use_driver_framework_v2=true" \
     --kernel-args "driver_manager.root-driver=fuchsia-boot:///#meta/platform-bus.cm" \
     --kernel-args "devmgr.enable-ephemeral=true"
   ...
   Logging to "/home/alice/.local/share/Fuchsia/ffx/emu/instances/fuchsia-emulator/emulator.log"
   Waiting for Fuchsia to start (up to 60 seconds).
   Emulator is ready.
   ```

1. Click the **Not connected** icon at the bottom of VS Code.

   ![Not connected](/docs/get-started/sdk/images/get-started-vscode-not-connected.png "The Not connect icon at the bottom of VS Code"){: .screenshot}

   This opens the Command Palette at the top of VS Code.

1. Click **Set fuchsia-emulator as default** in the Command Palette.

   This sets `fuchsia-emulator` as the default target at the bottom of VS Code:

   ![Connected](/docs/get-started/sdk/images/get-started-vscode-connected-to-fuchsia-emulator.png "The fuchsia-emualtor icon at the bottom of VS Code"){: .screenshot}

1. In the terminal, start the Fuchsia package server:

   ```posix-terminal
   tools/ffx repository server start
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx repository server start
   ffx repository server is listening on [::]:8083
   ```

1. Check the list of Fuchsia package repositories available on
   your host machine:

   ```posix-terminal
   tools/ffx repository list
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx repository list
   +-----------------------+------+-------------------------------------------------------------------------------------------------+
   | NAME                  | TYPE | EXTRA                                                                                           |
   +=======================+======+=================================================================================================+
   | workstation-packages* | pm   | /home/alice/.local/share/Fuchsia/ffx/pbms/4751486831982119909/workstation_eng.qemu-x64/packages |
   +-----------------------+------+-------------------------------------------------------------------------------------------------+
   ```

   Notice a package repository named `workstation-packages` is created
   for the Workstation prebuilt image.

1. Register the `workstation-packages` package repository to the target device:

   ```posix-terminal
   tools/ffx target repository register -r workstation-packages --alias fuchsia.com --alias chromium.org
   ```

   This command exits silently without output.

<!-- Reference links -->

[driver-framework]: /docs/concepts/drivers/driver_framework.md
[femu]: /docs/development/sdk/ffx/start-the-fuchsia-emulator.md
