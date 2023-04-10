Start the Fuchsia debugger ([`zxdb`][fuchsia-debugger]) and debug the
sample component, which is now updated to crash when it runs.

The tasks include:

- Start the Fuchsia debugger and connect it to the emulator instance.
- Attach the debugger to the sample component.
- Set a breakpoint.
- Run the sample component and step through the code.

Do the following:

1. Start the Fuchsia debugger:

   ```posix-terminal
   tools/ffx debug connect
   ```

   This command automatically connects the debugger to the default target
   device, which is the emulator instance.

   When connected to the device, this command starts the `zxdb` terminal, for
   example:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx debug connect
   Connecting (use "disconnect" to cancel)...
   Connected successfully.
   👉 To get started, try "status" or "help".
   [zxdb]
   ```

1. In the `zxdb` terminal, attach the debugger to the `hello_world.cm`
   component:

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>attach hello_world.cm
   </pre>

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] attach hello_world.cm
   Waiting for process matching "hello_world.cm".
   Type "filter" to see the current filters.
   ```

1. In the `zxdb` terminal, set a breakpoint at the `main()` method:

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>break main
   </pre>

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] break main
   Created Breakpoint 1 @ main
   Pending: No current matches for location. It will be matched against new
            processes and shared libraries.
   ```

1. In a different terminal, run the sample component:

   Note: In this new terminal, make sure that you change to the same work
   directory (for instance, `cd $HOME/fuchsia-getting-started`).

   ```posix-terminal
   tools/bazel run //src/hello_world:pkg.component
   ```

   In the `zxdb` terminal, the sample component is paused at the breakpoint:

   ```none {:.devsite-disable-click-to-copy}
   Attached Process 1 state=Running koid=17658651 name=hello_world.cm
   Downloading symbols...
   Breakpoint 1 now matching 1 addrs for main
   Could not load symbols for "<vDSO>" because there was no mapping for build ID "1dbd2861a642d61b".
   Symbol downloading complete. 0 succeeded, 1 failed.
   🛑 on bp 1, 2 main() • hello_world.cc:8
       6
       7 int main() {
    ▶  8   std::cout << "Hello again, World!\n";
       9   abort();
      10   return 0;
   [zxdb]
   ```

   Note: You can re-build and re-run your component as many times as you want,
   but do not need to restart the debugger or run `attach` again. The debugger
   will preserve your breakpoints and continue watching for future processes
   called `hello_world.cm`.

1. In the new terminal, monitor device logs for the `hello_world` component:

   ```posix-terminal
   tools/ffx log --filter hello_world
   ```

   This comment prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --filter hello_world
   ...
   [215.904][pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://bazel.pkg.component.runnable/hello_world as fuchsia-pkg://bazel.pkg.component.runnable/hello_world to 4c1ba90570cc92a62fab8e718a2fad4599583c5b49a684b97469452c9c9387a8 with TUF
   [215.952][pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://bazel.pkg.component.runnable/hello_world as fuchsia-pkg://bazel.pkg.component.runnable/hello_world to 4c1ba90570cc92a62fab8e718a2fad4599583c5b49a684b97469452c9c9387a8 with TUF
   ```

   Notice the `Hello again, World!` line is not printed yet.

1. In the `zxdb` terminal, use `next` to step through the code:

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>next
   </pre>

   In the `zxdb` terminal, the code get executed line by line, for example:

   ```none {:.devsite-disable-click-to-copy}
   ...
   🛑 on bp 1 main() • hello_world.cc:8
       6
       7 int main() {
    ▶  8   std::cout << "Hello again, World!\n";
       9   abort();
      10   return 0;
   [zxdb] {{ '<strong>'}}next{{ '</strong>'}}
   🛑 main() • hello_world.cc:9
       7 int main() {
       8   std::cout << "Hello again, World!\n";
    ▶  9   abort();
      10   return 0;
      11 }
   ```

   In the device logs terminal, verify that the `Hello again, World!` line is now
   printed:

   ```none {:.devsite-disable-click-to-copy}
   [ffx-laboratory:hello_world.cm][I] Hello again, World!
   ```

1. To exit the `zxdb` terminal, type `exit` or press `Ctrl-D`.

   This causes the component to finish the execution of the rest of the code.

   Note: For more information on usages and best practices on `zxdb`, see the
   [zxdb user guide][zxdb-user-guide].

<!-- Reference links -->

[fuchsia-debugger]: /docs/development/debugger/README.md
[zxdb-user-guide]: /docs/development/debugger/README.md
