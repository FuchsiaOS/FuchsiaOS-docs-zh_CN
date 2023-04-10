# Recording a boot trace

The Zircon kernel's internal tracing system can be active on boot. This means that one can trace at
least the kernel side of booting by setting a boot option. Then, once booting has finished, the data
is already there, one just needs to collect it.

## Enable the Kernel Tracing Boot Parameter

The size of the kernel's trace buffer can be changed at boot time
with the `ktrace.bufsize=N` command line option, where `N` is the size
of the buffer in megabytes.

The choice of data to collect is controlled with the `ktrace.grpmask=0xNNN'
command line option. The 0xNNN value is a bit mask of *KTRACE\_GRP\_\**
values from
//zircon/kernel/lib/boot-options/include/lib/boot-options/options.inc.
The default is 0x000, which disables all trace categories (or groups in
ktrace parlance).

Set the boot option as a GN variable via your `fx set` command:

```posix-terminal
fx set ... --args='dev_kernel_cmdline=["ktrace.grpmask=0xFFF"]'
```

You'll then need to rebuild and redeploy.

For more information on Zircon command line options see:
- [kernel_cmdline](/docs/reference/kernel/kernel_cmdline.md)
- [kernel_build](/docs/development/kernel/build.md)

## Including kernel boot trace data in trace results

Once you enable the kernel tracing boot parameter, as long as the kernel's internal trace buffer is
not rewound, after boot, the data is available to be included in the trace. This is achieved by
passing category `kernel:retain` to the `ffx trace` or `trace` program. Note that the moment a trace
is made without passing `kernel:retain` then the ktrace buffer is rewound and the data is lost.

Example:

```posix-terminal
ffx trace start --categories "kernel,kernel:retain" --buffer-size 32 --duration 1
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

The third important thing to note is that in this example we just want
to grab the current contents of the trace buffer, and aren't interested
in tracing anything more. That is why a duration of one second is used.
