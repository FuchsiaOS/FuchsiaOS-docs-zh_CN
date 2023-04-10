# fidlcat: Monitor and debug your fidl calls

## Overview

fidlcat is a tool that allows users to monitor FIDL connections. Currently, it
can attach to or launch a process on a Fuchsia device, and will report its FIDL
traffic.

## Enabling it

The `fidlcat` tool is included with the SDK.  In `fuchsia.git`, an invocation of
`fx build` will build it automatically.

To run `fidlcat`, networking must be enabled on your Fuchsia target. To boot an
emulator with networking enabled. Follow the instructions to [start the emulator
with access to external networks on the FEMU
page](/get-started/set_up_femu.md).

## Running it

When your environment is properly set up, and fidlcat is built, you should be
able to use it to monitor FIDL messages from processes on the target. There are
several ways to do this.  Note that `fidlcat` must be invoked via the `ffx debug
fidl` command, which automatically sets up a network tunnel and finds some of
the prerequisite artifacts (e.g., debug symbols).

### Attaching to a running process

If you run the `ps` command in the shell, you can get a pid you want to monitor,
and run:

```sh
ffx debug fidl --remote-pid <pid>
```

If your code is executed by a runner, you are likely to want to attach to the
runner. For Dart JIT-executed code, run `ps` on the target, and look for the process named `dart_jit_runner`:

```sh
host$ fx shell ps
[...]
        j:21102           17.6M   17.6M
          p:21107         17.6M   17.6M     32k         dart_jit_runner.cmx
```

You can then attach directly to that process, and view all FIDL messages sent by
Dart programs:

```sh
host$ ffx debug fidl --remote-pid 21107
```

You can use the `--remote-pid` flag multiple times to connect to multiple processes:

```sh
ffx debug fidl --remote-pid <pid1> --remote-pid <pid2>
```

### Launching a component with fidlcat

Alternatively, you can launch a component directly using its URL:

```sh
ffx debug fidl -- run fuchsia-pkg://fuchsia.com/echo_client_rust#meta/echo_client_rust.cmx
```

You can also specify the URL with a bash regex that matches a unique URL known to the build:

```sh
ffx debug fidl -- run "echo_client_cpp_synchronous.*"
ffx debug fidl -- run echo_client_cpp.cmx
```

### Attaching to a program on startup

You can also attach to programs with their names by passing a regex to
match their names. Fidlcat will attach to all currently running and
subsequently started programs that satisfy the regex. If you issue the following
command, fidlcat will connect to the system, and attach to all programs with the
substring "echo_client".

```sh
ffx debug fidl --remote-name echo_client
```

### Mixed use

All three options --remote-pid, --remote-name and run can be used together.
However, run must always be the last one.

When --remote-name and run are used together, only processes that match
--remote-name are monitored.

Examples (echo_server is launched by echo_client):

Run and monitor echo_client.
```sh
ffx debug fidl -- run echo_client_cpp.cmx
```

Run and monitor echo_client.
```sh
ffx debug fidl --remote-name echo_client -- run echo_client_cpp.cmx
```

Run echo_client and monitor echo_server.
```sh
ffx debug fidl --remote-name echo_server -- run echo_client_cpp.cmx
```

Run echo_client and monitor echo_client and echo_server.
```sh
ffx debug fidl --remote-name echo -- run echo_client_cpp.cmx
```

Run echo_client and monitor echo_client and echo_server.
```sh
ffx debug fidl --remote-name echo_client --remote-name echo_server -- run echo_client_cpp.cmx
```

### Monitoring a service

If you want to monitor a service, you should use --extra-name instead of --remote-name. The option
--extra-name also monitors some processes. However, for these processes, monitoring starts only
when one of of the "--remote-name" process is launched. Also, fidlcat stops when the last
"--remote-name" process stops (even if some "--extra-name" processes are still monitored).

```sh
ffx debug fidl --remote-name echo --extra-name appmgr -- run echo_client_cpp.cmx
```

### Input options

You have two input options:

 * --from=device This is the default option, which monitors a device in real time.

 * --from=&lt;path&gt; Playback. With this option, fidlcat replays a session previously saved with
   --to=&lt;path&gt; (protobuf format).

### Session save

The option --to=&lt;path&gt; The session is saved to the specified file (binary protobuf format).
When a session is saved, you can replay it using "--from=&lt;path&gt;". The raw data is saved.
That means that the data saved is independent from what is displayed.

### Format (output) options

You have the following output options:

 * --format=pretty This is the default output. The session is pretty printed (with colors).

 * --format=json The session is printed using a json format.

 * --format=textproto The session is printed using a text protobuf format.

 * --format=summary   At the end of the session, a summary of the session is displayed.

 * --format= Nothing is displayed on the standard output (this option only makes sense when used
 with --to=&lt;path&gt;). When there is no output, fidlcat is much faster (this is better when you
 want to monitor real time components).


## Troubleshooting

If you have problems running `ffx debug fidl`, see the [troubleshooting guide
for zxdb](/development/debugger/running.md),

The `fidlcat` tool needs two sources of information to work.  If either are
missing, you will not be able to decode fidl messages:

 * First, it needs the symbols for the executable. In practice, if you are
   running in-tree, the symbols should be provided to fidlcat automatically.
   Otherwise, you can provide fidlcat either a) a `.build-id` directory using
   `--build-id-dir` flag, b) an `ids.txt` file using `--ids-txt` flag, or c)
   an arbitrary ELF file or a directory of ELF files using `--symbol-path`
   flag. These flags can be combined and specified multiple times.

 * Second, it needs the intermediate representation for the FIDL it ingests, so
   it can produce readable output. If you are running in-tree, the IR should be
   provided to fidlcat automatically. Otherwise, you can provide fidlcat an IR
   path, which can be an explicit IR file path, a directory it will scan for IR
   files, or an argument file containing explicit paths. This can be provided
   to fidlcat with the `--fidl-ir-path` flag. The argument files need to be
   prepended with a `@` character: `--fidl-ir-path @argfile`.

Developers with other concerns can [file a
bug](https://bugs.fuchsia.dev/p/fuchsia/issues/entry).  Use the Tools>fidlcat
component.

## Read the guide

The [fidlcat guide](fidlcat_usage.md) describes all the flags that modify the
output. It also gives some examples of display interpretation.

## Where is the code?

The code is located in `//tools/fidlcat`.
