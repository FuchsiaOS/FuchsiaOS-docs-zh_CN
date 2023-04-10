Start the Fuchsia debugger ([`zxdb`][zxdb-user-guide]) in VS Code and
step through the sample driver’s code.

The tasks include:

*   Identify the driver host (which is a component) of the `qemu_edu`
    driver.
*   Create a launch configuration for the debugger in VS Code.
*   Set a breakpoint in the driver's source code.
*   Start the Fuchsia debugger.
*   Step through the driver’s code.

In VS Code, do the following:

1. Click the **TERMINAL** tab on the VS Code panel.

1. In the terminal, view the list of the running driver hosts:

   ```posix-terminal
   tools/ffx driver list-hosts
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx driver list-hosts
   Driver Host: 5507
       fuchsia-boot:///#meta/bus-pci.cm
       fuchsia-boot:///#meta/display.cm
       fuchsia-boot:///#meta/goldfish-display.cm
       fuchsia-boot:///#meta/goldfish.cm
       fuchsia-boot:///#meta/goldfish_control.cm
       fuchsia-boot:///#meta/goldfish_sensor.cm
       fuchsia-boot:///#meta/goldfish_sync.cm
       fuchsia-boot:///#meta/hid.cm
       fuchsia-boot:///#meta/platform-bus-x86.cm
       fuchsia-boot:///#meta/platform-bus.cm
       unbound

   ...

   Driver Host: 25673
       fuchsia-pkg://fuchsia.com/virtual_audio#meta/virtual_audio_driver.cm
       unbound

   Driver Host: {{ '<strong>' }}85211{{ '</strong>' }}
       fuchsia-pkg://bazel.pkg.component.runnable/qemu_edu#meta/qemu_edu.cm
   ```

   Make a note of the process ID (PID) of the `qemu_edu` driver host
   (`85211` in the example above).

1. Click the **Run and Debug** icon on the left side of VS Code.

   ![Run and Debug](/docs/get-started/sdk/images/get-started-vscode-run-and-debug-icon.png "The Run and Debug icon in VS Code"){: .screenshot width="350"}

1. Click the **Show all automatic debug configurations** link.

   This opens the Command Palette and displays a list of
   launch configurations:

   ![Run and Debug](/docs/get-started/sdk/images/get-started-vscode-add-config-fuchsia-drivers.png "The Add Config options in VS Code"){: .screenshot width="500"}

1. In the Command Palette, click **Add Config (fuchsia-drivers)...**.

1. Click **zxdb**.

   This opens the `.vscode/launch.json` file.

1. Update this `launch.json` file to the following configuration:

   ```json5 {:.devsite-disable-click-to-copy}
   "configurations": [
       {
           "name": "{{ '<strong>' }}Fuchsia drivers{{ '</strong>' }}",
           "type": "zxdb",
           "request": "launch",
           "launchCommand": "{{ '<strong>' }}tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- fact 12{{ '</strong>' }}",
           "process": "{{ '<var>' }}PID{{ '</var>' }}"
       }
   ]
   ```

   Replace <var>PID</var> with the PID of your `qemu_edu` driver host from
   step 2, for example:

   ```none {:.devsite-disable-click-to-copy}
           "process": "{{ '<strong>' }}85211{{ '</strong>' }}"
   ```

   This configures the debugger to attach to the driver host
   using the PID and run `eductl_tool`.

1. To save the file, press `CTRL+S` (or `Command+S` on macOS).

1. Click the **Explorer** icon on the left side of VS Code.

   ![Explorer](/docs/get-started/sdk/images/get-started-vscode-explorer-icon.png "The Explorer icon in VS Code"){: .screenshot width="50"}

1. Open the `src/qemu_edu/drivers/edu_device.cc` file.

1. To set a breakpoint at the `QemuEduDevice::HandleIrq()` method,
   click the space to the left of the line number.

   ![Breakpoint](/docs/get-started/sdk/images/get-started-vscode-qemu-edu-breakpoint.png "A breakpoint in VS Code"){: .screenshot}

   When a breakpoint is set, a red dot appears.

1. Click the **Run and Debug** icon on the left side of VS Code.

1. At the top of the **Run and Debug** panel, select the
   **Fuchsia drivers** option in the dropdown menu.

1. At the top of the **Run and Debug** panel, click
   the **Play** icon to launch the debugger.

   ![Play](/docs/get-started/sdk/images/get-started-vscode-qemu-edu-play-icon.png "The Play icon on the Run and Debug panel of VS Code"){: .screenshot width="350"}

   This builds and runs `eductl_tools`, which causes
   the debugger to pause at the line where the breakpoint is set
   in the `src/qemu_edu/drivers/edu_device.cc` file.

1. Click the **DEBUG CONSOLE** tab on the VS Code panel.

   ![Debug console](/docs/get-started/sdk/images/get-started-vscode-qemu-edu-debug-console.png "The Debug console panel in VS Code"){: .screenshot}

   This shows the console output of the Fuchsia debugger (`zxdb`).

1. Click the **TERMINAL** tab on the VS Code panel.

   The panel shows that the `eductl_tool` command is
   idling after printing output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- fact 12
   ...
   INFO: Build completed successfully, 1 total action
   Running workflow: pkg.eductl_tool_base
   Running task: pkg.debug_symbols_base (step 1/2)
   Running task: pkg.eductl_tool.run_base (step 2/2)
   added repository bazel.pkg.eductl.tool.runnable
   ```

1. In the debug toolbar at the top of VS Code, click the **Step Over**
   icon.

   ![Step over](/docs/get-started/sdk/images/get-started-vscode-step-over-icon.png "The Step Over icon in VS Code"){: .screenshot width="250"}

1. Continue to click the **Step Over** icon to step through the
   `HandleIrq()` function until the line 126 is reached.

   When the `HandleIrq()` function is finished, it replies
   with the result of the factorial.

1. In the **TERMINAL** panel, verify that the `eductl_tool` command
   has exited after printing the result of the factorial:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- fact 12
   ...
   INFO: Build completed successfully, 1 total action
   Running workflow: pkg.eductl_tool_base
   Running task: pkg.debug_symbols_base (step 1/2)
   Running task: pkg.eductl_tool.run_base (step 2/2)
   added repository bazel.pkg.eductl.tool.runnable
   {{ '<strong>' }}Factorial(12) = 479001600{{ '</strong>' }}
   $
   ```

1. To exit the debugger, click the **Stop** icon in the debug toolbar.

   ![Debug stop](/docs/get-started/sdk/images/get-started-vscode-debug-stop-icon.png "The Stop icon in VS Code"){: .screenshot width="250"}

<!-- Reference links -->

[zxdb-user-guide]: /docs/development/debugger/README.md
