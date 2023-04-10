# Recording a Fuchsia trace

This document describes how to record a trace with Fuchsia's
[tracing system](/concepts/kernel/tracing-system.md).

## Prerequisites

Tracing is only enabled for `core` and `eng` based products. Ensure that your
build type is not `user` or `userdebug`.

Many existing components already have trace points included and ready to go
which will give you a reasonable overview of the system. If you are recording a
general trace, such as for adding additional detail to a bug report, continue on
to the next step.

However, if you are adding additional trace points, ensure that you have:

* Registered your component as a trace provider. See
  [Registering a trace provider](/development/tracing/tutorial/registering-a-trace-provider.md).
* Added tracing in your code. See
  [Adding tracing in your code](/development/tracing/tutorial/adding-tracing-in-code.md).

## Use the utilities

The most convenient and straight forward way to trace is from your development
host via `ffx trace`.

* [From a development host](#from-a-development-host)

If you run into a situation where you wish to trace, but do not have access to
ffx, you can also trace directly from the Fuchsia target using the `trace`
shell package.

* [From a Fuchsia target](#from-a-fuchsia-target)

### From a development host {#from-a-development-host}

To record a trace for a Fuchsia target from a development host, run the
following command:

```posix-terminal
ffx trace start [--duration <SECONDS>]
```

This will start a trace with the default settings, which will capture a general
overview.

The trace will continue for the given duration, or until `<ENTER>` is pressed
if duration is left unspecified. Once the trace finishes, the trace data will
automatically be saved to `trace.fxt` in the current directory. This can be
changed by specifying the flag `ffx trace start --output <file_path>`.

Then, you can upload your trace file to
[ui.perfetto.dev](https://ui.perfetto.dev) to view it. Alternatively, see
[Converting and Visualizing a
Trace](/development/tracing/tutorial/converting-visualizing-a-trace.md)
for additional options.

#### Categories and category groups

You can control what kinds of data is collected during the trace session by specifying
a `--categories` argument to `ffx trace start`.  The full list of categories can be
accessed by running the following command:

```posix-terminal
ffx trace list-categories
```

`ffx trace start` also supports "category groups" - predefined lists of categories which
are notated with a `#` prefix. For example, `#chrome_nav`, which expands to
`loading,net,netlog,neavigation,browser`, can be used to quickly specify all the
events relating to resource loading and page navigation. The default categories
are also represented as a category group called `#default`. Custom category groups may be set
using `ffx config set`. To see the full list of category groups, run:

```posix-terminal
ffx config get -s all trace.category_groups
```

If you would like to define a custom category group for an often-used set of categories,
you can do so by running a command similar to the one below, which defines a category group
called `#audiovisual`:

```posix-terminal
ffx config set trace.category_groups.audiovisual '["audio", "gfx"]'
```

For a complete list of the `ffx trace start` options, run `ffx trace start --help`.

Once you have the trace output file, you can
[convert and analyze that trace file](/development/tracing/tutorial/converting-visualizing-a-trace.md).

#### Tracing Chrome and WebEngine events

Tracing information can be collected from Chrome and WebEngine by calling `ffx trace start`
with a list of Chrome-specific categories or category groups. You may also specify categories
outside of Chrome as well. The list of Chrome category groups are:

 * `#chrome_input`: Input handling events.
 * `#chrome_ipc_flows`: Mojo IPC routing events.
 * `#chrome_js_exec`: JavaScript (V8) events.
 * `#chrome_nav`: Resource loading, page navigation, browser events.
 * `#chrome_task_sched`: Asynchronous task scheduling/dispatch events.
 * `#chrome_ui_render`: Chrome UI (browser UX, browser widgets, compositor, GPU) events.
 * `#chrome_web_content_render`: Content rendering (Blink, compositor, GPU) events.

All Chrome category groups include the categories `toplevel` and `toplevel.flow` which cover
a variety of basic Chrome events such as async task scheduling.

You may also combine Chrome categories with Fuchsia categories. For example, if you would like to
see trace data for Chrome content rendering alongside Zircon kernel scheduler activity, you can run
the following command:

```posix-terminal
ffx trace start --categories kernel:sched,#chrome_web_content_render
```

### From a Fuchsia target {#from-a-fuchsia-target}

To record a trace directly from a Fuchsia target, you will need to include the
`trace` shell component in your build with `fx set ... --with
//src/performance/trace`.

Then run the following command in a shell on your target:

<pre class="prettyprint">
<code class="devsite-terminal">trace record</code>
</pre>

This saves your trace in `/data/trace.json` on your Fuchsia target by default.
For more information on the `trace` utility, run `trace --help` at a Fuchsia shell.

Once you have the trace output file, you can
[convert and analyze that trace file](/development/tracing/tutorial/converting-visualizing-a-trace.md).

<!-- Reference links -->

[ffx-trace]: https://fuchsia.dev/reference/tools/sdk/ffx#trace
