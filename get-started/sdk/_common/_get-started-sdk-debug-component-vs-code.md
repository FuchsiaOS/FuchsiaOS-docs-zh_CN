Start the Fuchsia debugger ([`zxdb`][fuchsia-debugger]) in VS Code and debug
the sample component, which is now  updated to crash when it runs.

The tasks include:

- Set a breakpoint in the source code.
- Start the Fuchsia debugger.
- Step through the code.

In VS Code, do the following:

1. Select the `src/hello_world/hello_world.cc` file from the **OPEN EDITORS**
   view at the top of VS Code.

1. To set a breakpoint at the `main()` method, click the space to the left of
   the line number.

   ![Breakpoint](/get-started/sdk/images/get-started-vscode-breakpoint.png "A breakpoint in VS Code"){: .screenshot width="500"}

   When a breakpoint is set, a red dot appears.

1. Click **Run > Start Debugging**.

   This starts a debug session that launches the `hello_world` component.
   The debugger then pauses at the line where the breakpoint is set
   in the `src/hello_world/hello_world.cc` file.

1. Click the **DEBUG CONSOLE** tab on the VS Code panel.

   ![Debug console](/get-started/sdk/images/get-started-vscode-debug-console.png "The Debug console panel in VS Code"){: .screenshot}

   This shows the console output of the Fuchsia debugger (`zxdb`).

1. Click the **FUCHSIA LOGS** tab on the VS Code panel.

1. At the top right corner of the **FUCHSIA LOGS** panel,
   click the **Clear logs** icon.

1. In the **Filter logs** text box, type `hello_world` and press **Enter**.

1. In the debug toolbar at the top of VS Code, click the **Step Over** icon.

   ![Step over](/get-started/sdk/images/get-started-vscode-step-over-icon.png "The Step Over icon in VS Code"){: .screenshot width="250"}

1. In the **FUCHSIA LOGS** panel, verify that a new `Hello again, World!`
   entry is printed in the logs.

   ![Hello again](/get-started/sdk/images/get-started-vscode-debug-hello-again-world.png "Hello again, World in the Fuchsia logs panel of VS Code"){: .screenshot}

1. In the debug toolbar at the top of VS Code, click the **Stop** icon.

   This causes the component to finish the execution of the rest of the code.

<!-- Reference links -->

[fuchsia-debugger]: /development/debugger/README.md
