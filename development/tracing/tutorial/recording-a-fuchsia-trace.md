# Recording a trace

This document describes how to record a trace with Fuchsia's tracing system.

Note: For more information on the Fuchsia tracing system, see
[Fuchsia tracing system](/docs/concepts/tracing/README.md).

## Prerequisites

Before you attempt to record a trace, make sure you have
done the following:

* Registered your component as a trace provider. See
  [Registering a trace provider](/docs/development/tracing/tutorial/registering-a-trace-provider.md).
* Added tracing in your code. See
  [Adding tracing in your code](/docs/development/tracing/tutorial/adding-tracing-in-code.md).
* Included the `tools` to your Fuchsia build. The `core` product and most other
  products include `tools` by default. If your build configuration does not
  include `tools` bundle by default, then you can manually add it with `fx set`:

  <pre class="prettyprint">
  <code class="devsite-terminal">fx set <var>PRODUCT</var>.<var>BOARD</var> --with-base '//bundles:tools'</code>
  </pre>

## Use the utilities

Traces are recorded with the `trace` utility on a Fuchsia target.
The `fx traceutil` command, which you can run from your development
host, calls the `trace` utility on your Fuchsia target.

You can record a trace from your Fuchsia target from your development host
or directly from the Fuchsia target.

* [From a development host](#from-a-development-host)
* [From a Fuchsia target](#from-a-fuchsia-target)

### From a development host {#from-a-development-host}

To record a trace for a Fuchsia target from a development host,
run the following:

Note: You can also configure parameters of tracing with a
tracing specification file. For more information, see
[Tracing specification file](/docs/reference/tracing/trace-specification-file.md).

<pre class="prettyprint">
<code class="devsite-terminal">fx traceutil record <var>component</var> <var>arg1 ...</var> --with-base</code>
</pre>

`fx traceutil record` does the following:

 * Takes a 10 second trace on the Fuchsia target with the default options.
   This records basic graphics data and thread CPU usage.
 * Copies the trace from the Fuchsia target device to your development host.

If you specify a component, the component runs after tracing has started.
This behavior is intended to not miss any early trace events in the component.

For a full list of the `record` option, run `fx traceutil record --help`.

Once you have trace file, you can now convert and analyze that
trace file. For more information, see
[Converting and visualizing a trace](/docs/development/tracing/tutorial/converting-visualizing-a-trace.md).

### From a Fuchsia target {#from-a-fuchsia-target}

To record a trace directly from a Fuchsia target, run the following in
a shell on your target:

Note: You can also configure parameters of tracing with a tracing specification
file. For more information, see
[Tracing specification file](/docs/reference/tracing/trace-specification-file.md).

<pre class="prettyprint">
<code class="devsite-terminal">trace record</code>
</pre>

This saves your trace in `/data/trace.json` on your Fuchsia target by default.
For more information on the `trace` utility, run `trace --help` at a Fuchsia shell.

Once you have trace file, you can now convert and analyze that
trace file. For more information, see
[Converting and visualizing a trace](/docs/development/tracing/tutorial/converting-visualizing-a-trace.md).

