# Recording a kernel trace

The kernel traces various actions by writing records to an internal buffer,
which can later be retrieved and printed.

## Kernel trace format

The kernel trace format is described in the
[ktrace.h](/zircon/system/ulib/zircon-internal/include/lib/zircon-internal/ktrace.h) and
[ktrace-def.h](/zircon/system/ulib/zircon-internal/include/lib/zircon-internal/ktrace-def.h) files under
[system/ulib/zircon-internal/include/lib/zircon-internal](/zircon/system/ulib/zircon-internal/include/lib/zircon-internal).

## Controlling what to trace

Control of what to trace is provided by a kernel command-line parameter
`ktrace.grpmask`. The value is specified as 0xNNN and is a bitmask
of tracing groups to enable. See the **KTRACE\_GRP\_\** values in
`system/ulib/zircon-internal/include/lib/zircon-internal/ktrace.h`.
The default is 0xfff, which traces everything.

What to trace can also be controlled by the `ktrace` command-line utility,
described below.

## Trace buffer size

The size of the trace buffer is fixed at boot time and is controlled by
the `ktrace.bufsize` kernel command-line parameter. Its value is the
buffer size in megabytes. The default is 32MB.

## ktrace command-line utility

Kernel tracing may be controlled with the `ktrace` command-line utility.

```
$ ktrace --help
Usage: ktrace [options] <control>
Where <control> is one of:
  start <group_mask>  - start tracing
  stop                - stop tracing
  rewind              - rewind trace buffer
  written             - print bytes written to trace buffer
    Note: This value doesn't reset on "rewind". Instead, the rewind
    takes effect on the next "start".
  save <path>         - save contents of trace buffer to <path>

Options:
  --help  - Duh.
```

## Pretty-printing a kernel trace

The host tool `ktrace-dump` can be used to pretty-print a kernel trace.

Example:

First collect the trace on the target:

```
$ ktrace start 0xfff
... do something ...
$ ktrace stop
$ ktrace save /tmp/save.ktrace
```

Then copy the file to the development host, and dump it:

```
host$ out/default/host-tools/netcp :/tmp/save.ktrace save.ktrace
host$ out/default/host-tools/ktrace-dump save.ktrace > save.dump
```

The pretty-printed output can be quite voluminous, thus it's recommended
to send it to a file and then view it in your editor or whatever.

## Use with Fuchsia Tracing

Fuchsia's tracing system supports collecting kernel trace records through
the `ktrace_provider` trace provider.
For documentation of Fuchsia's tracing system see the documentation in
[Fuchsia tracing system](concepts/kernel/tracing-system.md).

## More information

More information on `ktrace` can be found in the
[full list of kernel command line parameters](reference/kernel/kernel_cmdline.md).
