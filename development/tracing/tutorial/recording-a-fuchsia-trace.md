# Recording a Fuchsia trace

This document describes how to record a trace with Fuchsia's
[tracing system](/concepts/kernel/tracing-system.md).

## Prerequisites

Before you attempt to record a trace, make sure you have
done the following:

* Registered your component as a trace provider. See
  [Registering a trace provider](/development/tracing/tutorial/registering-a-trace-provider.md).
* Added tracing in your code. See
  [Adding tracing in your code](/development/tracing/tutorial/adding-tracing-in-code.md).
* Included the `tools` to your Fuchsia build. The `core` product and most other
  products include `tools` by default. If your build configuration does not
  include `tools` bundle by default, then you can manually add it with `fx set`:

  <pre class="prettyprint">
  <code class="devsite-terminal">fx set <var>PRODUCT</var>.<var>BOARD</var> --with-base '//bundles:tools'</code>
  </pre>

## Use the utilities

Traces are recorded with the `trace` utility on a Fuchsia target.
The [`ffx trace start`][ffx-trace] command, which you can run from
your development host, calls the `trace` utility on your Fuchsia target.

You can record a trace from your Fuchsia target from your development host
or directly from the Fuchsia target.

* [From a development host](#from-a-development-host)
* [From a Fuchsia target](#from-a-fuchsia-target)

### From a development host {#from-a-development-host}

To record a trace for a Fuchsia target from a development host,
run the following command:

Note: You can also configure parameters of tracing with a
[tracing specification file](/reference/tracing/trace-specification-file.md).

```posix-terminal
ffx trace start [--duration <SECONDS>]
```

`ffx trace start` does the following:

 * Starts a trace on the Fuchsia target with the default options.
 * Runs the tracing until the `Enter` key is pressed, or the duration is
   reached if provided.
 * Prints the trace results from the Fuchsia target device to an output file
   on your development host.

For a complete list of the `ffx trace start` options, run `ffx trace start --help`.

Once you have the trace output file, you can
[convert and analyze that trace file](/development/tracing/tutorial/converting-visualizing-a-trace.md).

### From a Fuchsia target {#from-a-fuchsia-target}

To record a trace directly from a Fuchsia target, run the following
command in a shell on your target:

Note: You can also configure parameters of tracing with a
[tracing specification file](/reference/tracing/trace-specification-file.md).

<pre class="prettyprint">
<code class="devsite-terminal">trace record</code>
</pre>

This saves your trace in `/data/trace.json` on your Fuchsia target by default.
For more information on the `trace` utility, run `trace --help` at a Fuchsia shell.

Once you have the trace output file, you can
[convert and analyze that trace file](/development/tracing/tutorial/converting-visualizing-a-trace.md).

<!-- Reference links -->

[ffx-trace]: https://fuchsia.dev/reference/tools/sdk/ffx#trace
