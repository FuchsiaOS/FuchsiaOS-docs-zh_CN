# Start the Fuchsia debugger

The [`ffx debug connect`][ffx-debug-connect] command starts the Fuchsia debugger
(which is called [`zxdb`][zxdb-user-guide]) for debugging Fuchsia components
on a device.

Important: `zxdb` only supports C++ and Rust at the moment.

## Concepts

`zxdb` is Fuchsia’s own debugger that allows you to attach a Fuchsia component
running on a device. Once a component is attached to `zxdb`, you can  perform
interactive debugging operations, such as adding breakpoints, stepping through
code, and inspecting stack traces and variables of the component.

For `zxdb` to understand and unpack the code of a Fuchsia component,
the component's debug symbols must be available in your development
environment. (For more information on debug symbols, see
[Register debug symbols][register-debug-symbols].)

When you run the `ffx debug connect` command, it establishes a connection to
the Fuchsia device and starts the `zxdb` terminal on the host machine.
In this terminal, you can use the [`zxdb` commands][zxdb-user-guide] to
interactively debug Fuchsia components running on the device.

## Run the Fuchsia debugger

To start the Fuchsia debugger, run the following command:

```posix-terminal
ffx debug connect
```

Once successfully connected to a Fuchsia device, this command starts the
`zxdb` terminal, for example:

```none {:.devsite-disable-click-to-copy}
$ ffx debug connect
Connecting (use "disconnect" to cancel)...
Connected successfully.
👉 To get started, try "status" or "help".
[zxdb]
```

In the `zxdb` terminal, you can start performing interactive debugging
operations. The example below shows that `zxdb` is attached to the
`memory_monitor` component and a breakpoint is created at the
component's `main` function:

```none {:.devsite-disable-click-to-copy}
[zxdb] attach memory_monitor.cm
Waiting for process matching "memory_monitor.cm".
Type "filter" to see the current filters.
Attached Process 1 state=Running koid=47467 name=memory_monitor.cm
[zxdb] process
  # State    Koid Name
▶ 1 Running 47467 memory_monitor.cm
[zxdb] break $main
Created Breakpoint 1 @ $main
   48
 ◉ 49 int main(int argc, const char** argv) {
   50   auto command_line = fxl::CommandLineFromArgcArgv(argc, argv);
[zxdb]
```

To exit the `zxdb` terminal, type `exit` or press `Ctrl-D`.

For more information on usages and best practices on `zxdb`,
see the [`zxdb` user guide][zxdb-user-guide].

<!-- Reference links -->

[ffx-debug-connect]: https://fuchsia.dev/reference/tools/sdk/ffx#connect
[register-debug-symbols]: ./register-debug-symbols.md
[zxdb-user-guide]: /development/debugger/README.md
