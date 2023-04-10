Examine the [symbolized logs][symbolize-logs] (that is, human readable stack
traces) of a crashed component.

The tasks include:

- Update the sample component to crash when it's started.
- Build and run the sample component, which generates and registers the debug
  symbols of the component.
- Verify that the crashed component's logs are in symbolized format.

In VS Code, do the following:

1. Select the `src/hello_world/hello_world.cc` file from the **OPEN EDITORS**
   view at the top of VS Code.

1. Above the line `return 0;`, add the following line:

   ```
   abort();
   ```

   The `main()` method now should look like below:

   ```none {:.devsite-disable-click-to-copy}
   int main() {
     std::cout << "Hello again, World!\n";
     {{ '<strong>' }}abort();{{ '</strong>' }}
     return 0;
   }
   ```

   This update will cause the component to crash immediately after printing a
   message.

1. To save the file, press `CTRL+S` (or `Command+S` on macOS).

1. Click **Run > Run Without Debugging**.

   Building a component automatically generates and registers its debug symbols
   in the development environment.

1. In the debug toolbar at the top of VS Code, click the **Stop** icon.

1. In the terminal, restart the `ffx` daemon:

   Note: Today, this workaround is required for newly registered symbols to be
   discovered in the environment. This issue is being tracked in
   [Issue 94614][ticket-94614]{:.external}.

   ```posix-terminal
   tools/ffx daemon stop
   ```

   A new instance of the `ffx `daemon starts the next time you run a `ffx`
   command or view device logs in VS Code.

1. Click the **FUCHSIA LOGS** tab on the VS Code panel.

1. In the **Filter logs** text box, type `moniker:klog` and press
   **Enter**.

1. Verify that the sample component's crash stack is symbolized in the kernel
   logs.

   ![Symbolized logs](/get-started/sdk/images/get-started-vscode-symbolized-logs.png "Symbolized Fuchsia logs shown in VS Code"){: .screenshot}

   The symbolized logs above show the exact filenames and line numbers
   (for example, `main() src/hello_world/hello_world.cc:9`) that might have
   caused the component to crash.

<!-- Reference links -->

[symbolize-logs]: /development/sdk/ffx/symbolize-logs.md
[ticket-94614]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=94614
