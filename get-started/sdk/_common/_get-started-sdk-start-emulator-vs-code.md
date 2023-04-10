Start the [Fuchsia emulator][femu] on the host machine. This guide uses an
instance of the Fuchsia emulator as the target device for running and testing
Fuchsia software. And to be able to supply new Fuchsia software to the target
device, you also need to start the
[Fuchsia package server][fuchsia-package-server] on the host machine.

The tasks include:

- Download a Fuchsia prebuilt image from Google Cloud Storage.
- Start the Fuchsia emulator to run the downloaded Fuchsia prebuilt image.
- Set the emulator instance as the default target device.
- Verify that various `ffx` commands can connect to the emulator instance.
- Start the Fuchsia package server.
- Register the system package repository to the emulator instance.

In VS Code, do the following:

1. Click **Terminal > New Terminal**.

1. In the terminal, download the latest Fuchsia Workstation prebuilt image
   for the emulator:

   ```posix-terminal
   tools/ffx product-bundle get workstation_eng.qemu-x64 --force-repo --repository workstation-packages
   ```

   This command may take a few minutes to download the image and product
   metadata.

   Once the download is finished, the `ffx product-bundle get` command creates a
   local Fuchsia package repository named `workstation-packages` on your host
   machine. This package repository hosts additional system packages for this
   Workstation prebuilt image. Later in step 10 you’ll register this package
   repository to the emulator instance.

1. Stop all running emulator instances:

   ```posix-terminal
   tools/ffx emu stop --all
   ```

1. Start a new Fuchsia emulator instance:

   Note: If your Linux machine does not support
   [KVM hardware virtualization](#check-if-your-linux-machine-supports-kvm-virtualization),
   start the emulator with the following command instead:
   `tools/ffx emu start workstation_eng.qemu-x64 --engine qemu --startup-timeout 720 --accel none --device qemu-x64-emu-min --headless`

   ```posix-terminal
   tools/ffx emu start workstation_eng.qemu-x64 --headless
   ```

   This command starts a headless emulator instance running the Workstation
   prebuilt image.

   When the instance is up and running, the command prints output similar to the
   following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx emu start workstation_eng.qemu-x64 --headless
   ...
   Logging to "/home/alice/.local/share/Fuchsia/ffx/emu/instances/fuchsia-emulator/emulator.log"
   Waiting for Fuchsia to start (up to 60 seconds)...........
   Emulator is ready.
   ```

1. Click the **Not connected** icon at the bottom of VS Code.

   ![Not connected](/get-started/sdk/images/get-started-vscode-not-connected.png "The Not connect icon at the bottom of VS Code"){: .screenshot}

   This opens the Command Palette at the top of VS Code.

1. Click **Set fuchsia-emulator as default** in the Command Palette.

   This sets `fuchsia-emulator` as the default target at the bottom of VS Code:

   ![Connected](/get-started/sdk/images/get-started-vscode-connected-to-fuchsia-emulator.png "The fuchsia-emulator icon at the bottom of VS Code"){: .screenshot}

1. In the terminal, to verify that you can establish an SSH connection
   to the emulator instance, run the following command:

   ```posix-terminal
   tools/ffx target show
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx target show
   Target:
       Name: "fuchsia-emulator"
       SSH Address: "127.0.0.1:37787"
   Board:
       Name: "default-board"
       Revision: "1"
       Instruction set: "x64"
   Device:
       ...
   Build:
       Version: "11.20230109.3.1"
       Product: "workstation_eng"
       Board: "qemu-x64"
       Commit: "2023-01-09T20:03:45+00:00"
   Last Reboot:
       Graceful: "false"
       Reason: "Cold"
   ...
   ```

   The example output above shows that the target device is running a
   `workstation_eng.qemu-x64` prebuilt image.

1. Start the Fuchsia package server:

   ```posix-terminal
   tools/ffx repository server start
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx repository server start
   ffx repository server is listening on [::]:8083
   ```

1. Check the list of Fuchsia package repositories available on your host
   machine:

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

   Notice a package repository named `workstation-packages` is created for the
   Workstation prebuilt image.

1. Register the `workstation-packages` package repository to the target device:

   ```posix-terminal
   tools/ffx target repository register -r workstation-packages --alias fuchsia.com --alias chromium.org
   ```

   This command exits silently without output.

<!-- Reference links -->

[femu]: /development/sdk/ffx/start-the-fuchsia-emulator.md
[fuchsia-package-server]: /development/sdk/ffx/create-a-package-repository.md
