Build and run the [C++ Hello World component][hello-world-component]{:.external}
included in the SDK samples repository. [Components][fuchsia-component] are the
basic unit of executable software on Fuchsia.

The tasks include:

- Build and run the sample Hello World component.
- Make a change to the component.
- Repeat the build and run steps.
- Verify the change.

In VS Code, do the following:

1. Click the **Explorer** icon on the left side of VS Code.

   ![Explorer](/get-started/sdk/images/get-started-vscode-explorer-icon.png "The Explorer icon in VS Code"){: .screenshot width="50"}

1. Open the `getting-started.code-workspace` file.

1. Verify that this file includes the following launch configuration:

   ```none {:.devsite-disable-click-to-copy}
   "launch": {
     "version": "0.2.0",
     "configurations": [
       {
         "name": "Fuchsia Hello World",
         "type": "zxdb",
         "request": "launch",
         "launchCommand": "tools/bazel run //src/hello_world:pkg.component",
         "process": "hello_world"
       }
     ]
   }
   ```

   This configuration is set to build and run the `hello_world` sample
   component.

1. Click the **Run and Debug** icon on the left side of VS Code.

   ![Run and Debug](/get-started/sdk/images/get-started-vscode-run-and-debug-icon-01.png "The Run and Debug icon in VS Code"){: .screenshot width="50"}

1. At the top of the **Run and Debug** panel, select the **Fuchsia Hello World**
   option in the dropdown memu.

   ![Run and Debug dropdown](/get-started/sdk/images/get-started-vscode-run-and-debug-dropdown.png "The dropdown menu in the Run and Drop panel of VS Code"){: .screenshot width="350"}

1. Click **Run > Run Without Debugging**.

   This starts a debug session (but without actually running a debugger)
   that launches the `hello_world` component. The `bazel run` command used to
   launch the component prints output similar to the following in the terminal:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel run //src/hello_world:pkg.component
   ...
   INFO: Build completed successfully, 155 total actions
   Running workflow: pkg.component_base
   Running task: pkg.debug_symbols_base (step 1/2)
   Running task: pkg.component.run_base (step 2/2)
   added repository bazel.pkg.component.runnable
   URL: fuchsia-pkg://bazel.pkg.component.runnable/hello_world#meta/hello_world.cm
   Moniker: /core/ffx-laboratory:hello_world.cm
   Creating component instance...
   Resolving component instance...
   Starting component instance...
   Started component instance!
   ```

1. In the terminal, check the status of the `hello_world` component:

   ```posix-terminal
   tools/ffx component show hello_world
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx component show hello_world
                  Moniker:  /core/ffx-laboratory:hello_world.cm
                      URL:  fuchsia-pkg://bazel.pkg.component.runnable/hello_world#meta/hello_world.cm
              Instance ID:  None
                     Type:  CML Component
          Component State:  Resolved
    Incoming Capabilities:  /svc/fuchsia.logger.LogSink
     Exposed Capabilities:
              Merkle root:  eebd529bd8ac6d2fd8a467279719f74c76643ebee2e94ebf594ffcbaac02fe8f
          Execution State:  Stopped
   ```

   The output shows that the `hello_world` component has run and is now
   terminated (`Stopped`).

1. In the debug toolbar at the top of VS Code, click the **Stop** icon
   to close the current debug session.

   ![Debug stop](/get-started/sdk/images/get-started-vscode-debug-stop-icon.png "The Stop icon in VS Code"){: .screenshot width="250"}

   Note: You can safely ignore the `Error: connection closed` pop-up message
   at the bottom of VS Code for now.

1. Click the **fuchsia-emulator** icon at the bottom of VS Code.

   ![Connected](/get-started/sdk/images/get-started-vscode-connected-to-fuchsia-emulator.png "The fuchsia-emualtor icon at the bottom of VS Code"){: .screenshot}

   This opens the Command Palette at the top of VS Code.

1. Click **Show log for fuchsia-emulator** in the Command Palette.

   This opens the **FUCHSIA LOGS** panel and streams the device logs of
   your current Fuchsia target.

   ![Fuchsia logs](/get-started/sdk/images/get-started-vscode-fuchsia-logs-panel.png "The Fuchsia logs panel in VS Code"){: .screenshot}

   Note: It may take a few minutes to load all the logs cached on the host
   machine. To stop the streaming of logs, click the
   <span class="material-icons">pause</span> icon at the top right corner of
   the **FUCHSIA LOGS** panel.

1. To fit the messages on the panel, click the **Wrap logs** icon
   at the top right corner of the **FUCHSIA LOGS** panel.

   ![Fuchsia logs](/get-started/sdk/images/get-started-vscode-wrap-logs-icon.png "The Wrap logs icon in VS Code"){: .screenshot width="200"}

1. In the **Filter logs** text box, type `hello_world` and
   press **Enter**.

   ![Hello World](/get-started/sdk/images/get-started-vscode-hello-world.png "Hello, World! shown in the Fuchsia logs panel of VS Code"){: .screenshot}

   Notice that `Hello, World!` is printed from the `hello_world` component.

   Note: For more information on filtering syntax, see
   [Filter Fuchsia logs][filter-vscode-logs].

1. Click the **Explorer** icon on the left side of VS Code.

1. Open the `src/hello_world/hello_world.cc` file.

1. Edit the message to `"Hello again, World!"`.

   The `main()` method now should look like below:

   ```none {:.devsite-disable-click-to-copy}
   int main() {
     {{ '<strong>' }}std::cout << "Hello again, World!\n";{{ '</strong>' }}
     return 0;
   }
   ```

1. To save the file, press `CTRL+S` (or `Command+S` on macOS).

1. Click **Run > Run Without Debugging**.

   This builds and runs the `hello_world` component again.

1. Click the **FUCHSIA LOGS** tab on the VS Code panel.

1. Verify that `Hello again, World!` is printed in the logs.

   ![Hello again, World](/get-started/sdk/images/get-started-vscode-hello-again-world.png "Hello again, World! shown in the Fuchsia logs panel of VS Code"){: .screenshot}

1. In the debug toolbar at the top of VS Code, click the **Stop** icon.

<!-- Reference links -->

[filter-vscode-logs]: /reference/tools/editors/vscode/fuchsia-ext-using.md#filter_fuchsia_logs
[fuchsia-component]: /concepts/components/v2/README.md
[hello-world-component]: https://fuchsia.googlesource.com/sdk-samples/getting-started/+/refs/heads/main/src/hello_world/
