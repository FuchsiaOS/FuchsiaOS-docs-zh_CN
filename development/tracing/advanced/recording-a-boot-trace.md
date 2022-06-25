# Recording a boot trace

The Zircon kernel's internal tracing system can be active on boot
(and in fact is currently the default). This means that one can trace
at least the kernel side of booting without extra effort. The data
is already there, one just needs to collect it.

## Including kernel boot trace data in trace results

As long as the kernel's internal trace buffer is not rewound the data
is available to be included in the trace. This is achieved by passing
category `kernel:retain` to the `ffx trace` or `trace` program.
Note that the moment a trace is made without passing `kernel:retain`
then the ktrace buffer is rewound and the data is lost.

Example:

```posix-terminal
ffx trace start --categories "kernel,kernel:retain" --buffer-size 64 --duration 1
```

There are a few important things to note here.

The first thing to note is the categories passed: `kernel` and `kernel:retain`.
The `kernel` category tells the kernel to trace everything.
In this example the kernel has already been tracing everything: that is
the default on boot. It is specified here as a simple way to
tell `ktrace_provider`, which is the interface between the Fuchsia tracing
system and the kernel, that kernel data is being collected.
The `kernel:retain` category tells `ktrace_provider` not to rewind the
kernel trace buffer at the start of tracing.

The second is the buffer size. The kernel's default trace buffer size
is 32MB whereas the Fuchsia trace default buffer size is 4MB.
Using a larger Fuchsia trace buffer size means there is enough space
to hold the contents of the kernel's trace buffer.
There are some implementation quirks at play here.
The kernel currently has its own trace format called "ktrace". When
tracing stops the `ktrace_provider` program reads the kernel trace buffer
and converts it to Fuchsia's trace format. Depending on circumstances
the ktrace buffer format is a little more compact. That is why the
above example provides a 64MB buffer even though the kernel's buffer
size was 32MB.

The third important thing to note is that in this example we just want
to grab the current contents of the trace buffer, and aren't interested
in tracing anything more. That is why a duration of one second is used.

## Changing kernel trace parameters at boot

The size of the kernel's trace buffer can be changed at boot time
with the `ktrace.bufsize=N` command line option, where `N` is the size
of the buffer in megabytes.

The choice of data to collect is controlled with the `ktrace.grpmask=0xNNN'
command line option. The 0xNNN value is a bit mask of *KTRACE\_GRP\_\**
values from
//zircon/system/ulib/zircon-internal/include/lib/zircon-internal/ktrace.h.
The default is 0xfff, which enables all trace categories (or groups in
ktrace parlance).

For more information on Zircon command line options see
[kernel\_cmdline](reference/kernel/kernel_cmdline.md).
