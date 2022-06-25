# Developing and debugging zxdb

## Run tests

To run the zxdb tests:

```posix-terminal
fx test zxdb_tests
```

To run the debug\_agent tests:

```posix-terminal
fx test debug_agent_unit_tests
fx test debug_agent_integration_tests
```

## Reload debug\_agent.cm after a new version is built

Since debug\_agent\_launcher is a long-running process, the system will not try to update
the debug\_agent package after the first `ffx debug connect` invocation.
To force the system to unload debug\_agent.cm, use

```posix-terminal
ffx component stop /core/debug_agent
```

## Enable debug logging in debug\_agent

Adding `--select core/debug_agent#DEBUG` to `fx log` will enable the debug logging of the
debug\_agent, e.g.

```posix-terminal
fx log --select core/debug_agent#DEBUG --tag debug_agent --hide_metadata --pretty
```

Note: `ffx log` doesn't work here because of [fxbug.dev/99937](https://fxbug.dev/99937).

## Enable debug logging in zxdb

To enable debug logging in zxdb, use

```posix-terminal
ffx debug connect -- --debug-mode
```

## Launch zxdb in another debugger

It's possible to ask `ffx debug` to launch zxdb in another debugger, e.g. lldb.

```posix-terminal
ffx debug connect --debugger lldb
```

This command will bring the lldb shell and you can use "run" to start the zxdb.

The "lldb" in the command can be substituted by "gdb".  However, using gdb might bring several
issues including

  * Older versions of gdb may not support all DWARF 5 standard, and some information might be
    missing such as source file listing.
  * Ctrl-C will not bring you back from zxdb to gdb. A workaround is to use `pkill -INT zxdb`
    in another window to stop the zxdb.

## Debug debug\_agent in another debug\_agent

It is also possible to attach a debug\_agent to another debug\_agent. This is done frequently by the
debugger team.

```none {:.devsite-disable-click-to-copy}
// Run the debugger that will attach to the "to-be-debugged" debug_agent.
$ ffx debug connect

// Within zxdb.
[zxdb] attach debug_agent
Waiting for process matching "debug_agent".
Type "filter" to see the current filters.
Attached Process 1 state=Running koid=345223 name=debug_agent.cm
Attached Process 2 state=Running koid=345403 name=/pkg/bin/debug_agent
// The first debug_agent will capture the launcher and itself. Detach to avoid any deadlock.
[zxdb] pr 1 detach
[zxdb] pr 2 detach
// Create a breakpoint on $main
[zxdb] break $main

// Launch another debugger in another window
$ ffx debug connect

// * Within the first zxdb:
Attached Process 1 state=Running koid=12345 name=/pkg/bin/debug_agent
Breakpoint 1 now matching 1 addrs for $main
🛑 process 1 on bp 1 main(int, const char**) • main.cc:101
    99
   100 int main(int argc, const char* argv[]) {
 ▶ 101   debug_agent::CommandLineOptions options;
   102   cmdline::Status status = ParseCommandLine(argc, argv, &options);
   103   if (status.has_error()) {

// Now you have two running instances of the debugger!
```

Note: Only one debugger can be attached to the main job in order to auto-attach to new processes
due to [fxbug.dev/97848](https://fxbug.dev/97848). Since you're using it for the first debugger,
you won't be able to auto-attach to new processes using a filter, or launch a component in the
second debugger. However, you should still be able to launch processes and attach to existing
processes.
