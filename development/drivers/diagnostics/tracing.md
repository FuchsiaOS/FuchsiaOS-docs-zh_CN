# Adding tracing to device drivers

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

This document describes how to add tracing to device drivers.

## Overview

Please read [Fuchsia tracing system](/concepts/kernel/tracing-system.md)
for an overview of tracing.

## Trace Provider

Drivers don't have to specify a Trace Provider, the driver host process
provides it through `libdriver.so`. It is mentioned here in case the topic
comes up.

## Adding trace records

### Source additions

Trace records are easiest to add by invoking the `TRACE_*()` macros
from `ddk/trace/event.h`.

There are various kinds of trace records that can be emitted.
Please see `trace/internal/event_common.h` for a description
of the various macros.

Looking up macro documentation from internal implementation files
is a temporary situation. Ultimately such documentation will live
in a more appropriate place.

Example:

```c++
#include <ddk/trace/event.h>

void DoSomething(int a, std::string b) {
  TRACE_DURATION("example", "DoSomething", "a", a, "b", b);

  // Do something
}
```

The first two arguments to most macros are the "category" and the
event name. Here they are "example" and "DoSomething" respectively.

Trace categories are how the tracing system lets the user specify
what data to collect. If a category is not requested by the user
then the data is not collected.

Categories don't need to be unique across the driver.
One typically groups several events under the same category.

The event name is included in the trace to describe what the event
is about. It is typically unique for each event.

### BUILD.gn additions

The following addition to your driver's `BUILD.gn` target is needed to
pick up tracing support:

```gn
fuchsia_driver("my_driver") {
  deps = [
    ...
    "//zircon/system/ulib/trace",
    "//zircon/system/ulib/trace:trace-driver",
  ]
}
```

## Building with tracing

The following needs to be passed to fx set in order to trace drivers
that are loaded during boot: `--with-base=//bundles/packages/prod:tracing`.

```sh
$ fx set ${PRODUCT}.${BOARD} --with-base=//bundles/packages/prod:tracing
$ fx build
```

The issue is that without this option then TraceManager won't be present
when the driver starts and thus the driver won't be able to participate
in tracing when TraceManager is started later.

See the documentation for [fx](/development/build/fx.md)
or even just the output of `fx help` and especially `fx help set` for further
documentation of running `fx` in general and `fx set` specifically.

## Booting with tracing

To be conservative, tracing uses a kernel command line flag to enable it:
`driver.tracing.enable=1`.
`driver.tracing.enable=1` is the default. To disable participation
of drivers in Fuchsia tracing, boot the kernel with `driver.tracing.enable=0`.

Then boot. See the documentation for your hardware or qemu for instructions
for booting the device. Tracing doesn't require anything special during boot.

## Using tracing

Once the system is booted you can collect traces on the target and
then manually copy them to your development host.
These examples use the category from the source additions described above.

Example:

<pre class="devsite-click-to-copy">
<span class="no-select">fuchsia$ </span>trace record --categories=example,kernel:sched,kernel:meta
</pre>

<pre class="devsite-click-to-copy">
<span class="no-select">host$ </span>fx cp --to-host /data/trace.json trace.json
</pre>

However, it's easier to invoke the `ffx trace` program on your development
host. It will copy the output file directly to your host and prepare them for
viewing with the [Perfetto viewer][perfetto-viewer]{:.external}.

<pre class="devsite-click-to-copy">
<span class="no-select">host$ </span>ffx trace start --categories "example,kernel:sched,kernel:meta"
</pre>

The categories `kernel:sched,kernel:meta` should always be present if you
want to visualize the results. The visualizer wants to associate trace data
with threads and processes, and thus it needs the data provided by the kernel
through these categories.

## Further Reading

See the [Tracing Documentation](/development/tracing/README.md)
for more information.

<!-- Reference links -->

[perfetto-viewer]: https://ui.perfetto.dev/#!/
