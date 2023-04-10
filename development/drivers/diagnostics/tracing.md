# Add tracing to a driver

This document describes how to add tracing to a driver in a Fuchsia system.

## Overview

For an overview of tracing, see
[Fuchsia tracing system][fuchsia-tracing-system]. However, drivers don't need
to specify a [trace provider][trace-provider]. The driver host provides one
for its drivers.

For a tutorial on tracing, see
[Adding tracing in your code][adding-tracing-in-your-code].
And for the tracing API reference, see
[Tracing: C and C++ macros][trace-c-cpp-macros].

## Add trace records {:#add-trace-records}

To add tracing a driver, you need to update the driver's source code
and `BUILD.gn` file.

### Source code update {:#source-code-update}

To add trace records in a driver component, update the source code to invoke
the `TRACE_*()` macros from `lib/trace/event.h`, for example:

```c++ {:.devsite-disable-click-to-copy}
#include <lib/trace/event.h>

void DoSomething(int a, std::string b) {
  TRACE_DURATION("example", "DoSomething", "a", a, "b", b);

  // Do something
}
```

The first two arguments to most macros are the trace category and the
event name. In this example, they are `example` and `DoSomething`
respectively.

Trace categories let you specify what types of data the tracing system
collects. If a category is not requested, no data is collected. Categories
don't need to be unique across the driver. One typically groups several
events under the same category.

An event name is included in the trace to describe the event. It is
typically unique for each event.

### BUILD.gn update {:#buildgn-update}

To pick up tracing support, add the following target to your driver's
`BUILD.gn`:

```gn
fuchsia_driver("my_driver") {
  deps = [
    ...
    "//zircon/system/ulib/trace",
  ]
}
```

## Build with tracing {:#build-with-tracing}

To be able to trace drivers that are loaded during boot,
set your Fuchsia build configuration with the additional
`--with-base=//bundles/packages/prod:tracing` option, for example:

```sh {:.devsite-disable-click-to-copy}
$ fx set workstation_eng.x64 --with-base=//bundles/packages/prod:tracing
$ fx build
```

Without this option, `TraceManager` is not present when the driver starts.
Thus the driver is not be able to participate in tracing when `TraceManager`
is started later.

For more information on `fx set`, see [fx workflows][fx-workflows]
(or run `fx help set`).

## Boot with tracing {:#boot-with-tracing}

Caution: The information in this section is only specific to the legacy
version of the driver framework (DFv1).

Fuchsia uses a kernel command-line flag to enable tracing in drivers
during boot:

```none
driver.tracing.enable=1
```

This is the default setting in Fuchsia devices.

To disable participation of drivers in Fuchsia tracing, boot the kernel with
the following command-line flag:

```none
driver.tracing.enable=0
```

For instructions on booting a specific Fuchsia device, see documentation for
your hardware or QEMU. Tracing doesn't require anything special during boot.

## Use tracing {:#use-tracing}

Use [`ffx trace`][ffx-trace] to record a trace and view the result with the
[Perfetto viewer][perfetto-viewer]{:.external}.

The example command below uses the `example` category described in the
[Source code update](#source-code-update) section above:

```none {:.devsite-disable-click-to-copy}
$ ffx trace start --categories "example,kernel:sched,kernel:meta"
```

The `kernel:sched,kernel:meta` categories need to be present if you
want to visualize the result. The visualizer wants to associate trace data
with threads and processes, and it needs the data provided by the kernel
through these categories.

For additional details, as well as instructions on tracing directly on a
Fuchsia device without `ffx`, see
[Recording a Fuchsia trace][recording-a-fuchsia-trace].

## Further reading

See [Fuchsia tracing guides][fuchsia-tracing-guides] for more information.

<!-- Reference links -->

[fuchsia-tracing-system]: /docs/concepts/kernel/tracing-system.md
[trace-provider]: /docs/concepts/kernel/tracing-system.md#trace-providers
[adding-tracing-in-your-code]: /docs/development/tracing/tutorial/adding-tracing-in-code.md
[trace-c-cpp-macros]: /docs/reference/tracing/c_cpp_macros.md
[fx-workflows]: /docs/development/build/fx.md
[ffx-trace]: /docs/development/sdk/ffx/record-traces.md
[perfetto-viewer]: https://ui.perfetto.dev/#!/
[recording-a-fuchsia-trace]: /docs/development/tracing/tutorial/recording-a-fuchsia-trace.md
[fuchsia-tracing-guides]: /docs/development/tracing/README.md

