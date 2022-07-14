# Record traces for performance analysis

The [`ffx trace`][ffx-trace] commands can record tracing information from processes
on a Fuchsia device.

## Concepts

The [Fuchsia tracing system][fuchsia-tracing-system] provides a mechanism to collect,
aggregate, and visualize diagnostic tracing information from user space processes and
the Zircon kernel on a Fuchsia device. A [trace provider][trace-providers] is a
component that generates trace data on the device, and a system can have
[many trace providers](#view-trace-providers). (To register your component as a trace
provider, see [Registering a trace provider][register-a-trace-provider].)

The `ffx trace start` command stores the output of tracing as a  [`.fxt`][fxt] file on
the host machine. You can upload this file to the
[Perfetto viewer][perfetto-viewer]{:.external} to render the trace results for
performance analysis. (For more information on Perfetto, see this
[Perfetto documentation][perfetto-docs]{:.external} site.)

By default, the `ffx trace start` command attempts to collect trace data from a
predefined set of trace categories (run `ffx trace start --help` to see the default
categories). However, `ffx trace start` also allows you to
[select trace categories](#view-trace-categories) for collecting trace data.

Only one trace session can be running on a Fuchsia device at a time, and only a single
trace can be recorded on an output file. In the examples below, all output files default
to `trace.fxt` in the directory where `ffx trace` is run, and all target devices default
to an available Fuchsia device connected to the host machine at the time.

## Run a trace interactively {:#run-a-trace-interactively}

With an interactive trace, you can press the `Enter` key to decide when to end
the tracing in real time. However, if the `--duration` flag is specified,
the tracing stops automatically when the duration is reached.

To start an interactive trace, run the following command:

```posix-terminal
ffx trace start [--duration <SECONDS>]
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx trace start
Tracing started successfully on "fuchsia-5254-0063-5e7a".
Writing to /Users/alice/trace.fxt
Press <enter> to stop trace.

```

To stop the trace, press the `Enter` key.

The command exits with output similar to the following:

```none {:.devsite-disable-click-to-copy}
Shutting down recording and writing to file.
Tracing stopped successfully on "fuchsia-5254-0063-5e7a".
Results written to /Users/alice/trace.fxt
Upload to https://ui.perfetto.dev/#!/  to view.
```

Once the tracing is finished, do the following to view the results:

1. Visit the [Perfetto viewer][perfetto-viewer]{:.external} site on a web browser.
2. Click **Open trace file** on the navigation bar.
3. Upload the `trace.fxt` file from the host machine.

## Run a trace in the background {:#run-a-trace-in-the-background}

A background trace runs indefinitely, as long as a duration is not specified.
To stop a trace running in the background, you need to run
[`ffx trace stop`](#stop-a-trace).

To start a background trace, run the following command:

Note: Unlike an interactive trace, a background trace can be given a duration in
fractional seconds (for example, `–duration 1.5` runs for 1.5 seconds).

```posix-terminal
ffx trace start --background [--duration <SECONDS>]
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx trace start --background
Tracing started successfully on "fuchsia-5254-0063-5e7a".
Writing to /Users/alice/trace.fxt
Current tracing status:
- fuchsia-5254-0063-5e7a:
  - Output file: /Users/alice/trace.fxt
  - Duration: indefinite
  - Config:
    - Categories:
      - app,audio,benchmark,blobfs,gfx,input,kernel:meta
```

To stop this tracing, see [Stop a trace](#stop-a-trace).

## Run a trace in the background with a trigger {:#run-a-trace-in-the-background-with-a-trigger}

If a trace is run with a trigger, the tracing stops when the specified event
is detected.

To run a trace with a trigger, run the following command:

Note: Traces with triggers can only be run in the background.
At the moment, the only available action is `terminate`.

```posix-terminal
ffx trace start --background --trigger <TRIGGER>
```

Replace `TRIGGER` with an action using the syntax `alert:action`, for example:

```none {:.devsite-disable-click-to-copy}
$ ffx trace start --background --trigger "foo:terminate"
```

This example command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx trace start --background --trigger "foo:terminate"
Tracing started successfully on "fuchsia-5254-0063-5e7a".
Writing to /Users/alice/trace.fxt
Current tracing status:
- fuchsia-5254-0063-5e7a:
  - Output file: /Users/alice/trace.fxt
  - Duration: indefinite
  - Config:
    - Categories:
      - app,audio,benchmark,blobfs,gfx,input,kernel:meta
  - Triggers:
    - foo : Terminate
```

## Check the status of traces in the background {:#check-the-status-of-traces-in-the-background}

To check the status of background traces, run the following command:

```posix-terminal
ffx trace status
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx trace status
- fuchsia-5254-0063-5e7a:
  - Output file: /Users/alice/trace.fxt
  - Duration: indefinite
  - Config:
    - Categories:
      - app,audio,benchmark,blobfs,gfx,input,kernel:meta
```

If there are no traces in the background, the command prints
the following:

```none {:.devsite-disable-click-to-copy}
$ ffx trace status
No active traces running.
```

## Stop a trace {:#stop-a-trace}

The `ffx trace stop` command stops a trace running in the background.

Note: To see all traces running in the background, run
[`ffx trace status`](#check-the-status-of-traces-in-the-background).

To stop a trace, run the following command:

```posix-terminal
ffx trace stop [--output <file>]
```

By default, the command stops a trace that matches the default target device.
However, you can also select which trace to stop by using the `–output` flag,
which then stops the trace that is associated with the output file.

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx trace stop
Tracing stopped successfully on "fuchsia-5254-0063-5e7a".
Results written to /Users/alice/trace.fxt
Upload to https://ui.perfetto.dev/#!/ to view.
```

Once the tracing is finished, do the following to view the results:

1. Visit the [Perfetto viewer][perfetto-viewer]{:.external} site on a web browser.
2. Click **Open trace file** on the navigation bar.
3. Upload the `trace.fxt` file from the host machine.

## View trace categories {:#view-trace-categories}

The `ffx trace start` command allows you to select categories which
are used to collect trace data, for example:

```none {:.devsite-disable-click-to-copy}
$ ffx trace start --categories "kernel,kernel:arch"
```

To see all available trace categories on a Fuchsia device,
run the following command:

```posix-terminal
ffx trace list-categories
```
This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx trace list-categories
Known Categories:
- app - Generic application traces
- benchmark - Benchmark traces
- cpu - several, run xyz for the list
- gfx - Graphics & Compositor
- input - Input system
- kernel - All kernel trace events
- kernel:arch - Kernel arch events

Default Categories:
- app
- audio
- benchmark
- blobfs
```

For more information on trace categories, check out the following advanced
guides on tracing:

* [Recording a boot trace][record-a-boot-trace]
* [Recording a CPU performance trace][record-a-cpu-trace]

## View trace providers {:#view-trace-providers}

To see all available trace providers on a Fuchsia device,
run the following command:

```posix-terminal
ffx trace list-providers
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx trace list-providers
Trace providers:
- ktrace_provider
```

<!-- Reference links -->

[ffx-trace]: https://fuchsia.dev/reference/tools/sdk/ffx#trace
[fuchsia-tracing-system]: /concepts/kernel/tracing-system.md
[register-a-trace-provider]: /development/tracing/tutorial/registering-a-trace-provider.md
[fxt]: /reference/tracing/trace-format.md
[perfetto-viewer]: https://ui.perfetto.dev/#!/
[perfetto-docs]: https://perfetto.dev/docs/
[record-a-boot-trace]: /development/tracing/advanced/recording-a-boot-trace.md
[record-a-cpu-trace]: /development/tracing/advanced/recording-a-cpu-performance-trace.md
[trace-providers]: /concepts/kernel/tracing-system.md#trace-providers
