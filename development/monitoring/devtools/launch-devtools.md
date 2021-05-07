# Launch Fuchsia DevTools

With [Fuchsia DevTools](/docs/concepts/monitoring/fuchsia-devtools.md), you can:

* View CPU and memory usage on your device.
* [Focus on relevant log messages](/docs/development/monitoring/devtools/devtools-logging.md) with keyword filtering
  and scoping logs by process, severity, or time frame
* Observe package updates on your device as you rebuild and restart components
* Monitor a component by watching for changes in its Inspect tree

Follow these instructions to install and start Fuchsia DevTools.

## Before you start Fuchsia DevTools

Fuchsia DevTools is a Flutter Desktop application that runs on macOS and Linux.

Make sure you have a package server running by following the
instructions to set up Fuchsia for
[in-tree development](/docs/get-started/README.md).

## Start Fuchsia DevTools

In a new terminal, start Fuchsia DevTools from your `fuchsia` directory:

<pre class="prettyprint">
  <code class="devsite-terminal">fx fdt</code>
</pre>

If this is your first time running Fuchsia DevTools,
you will be prompted to run the following command in a terminal window
before proceeding:

<pre class= "prettyprint"><code class="devsite-terminal">jiri init -fetch-optional=fuchsia_devtools && jiri fetch-packages --local-manifest=true</code></pre>

After running the command, start Fuchsia DevTools again:

<pre class="prettyprint">
  <code class="devsite-terminal">fx fdt</code>
</pre>

If you don't want to update your Fuchsia directory with `jiri update`, or if you
want to run the very latest version of Fuchsia DevTools, use the following
command in a terminal window:

<pre class="prettyprint">
  <code class="devsite-terminal">fx fdt latest</code>
</pre>

## Targeting a specific Fuchsia instance

If you have one device or emulator running, it's generally not necessary to
specify the name or address of the Fuchsia device or the emulator you want to
run Fuchsia DevTools against. However, if Fuchsia DevTools can't find your
target, you can set the `$FDT_DEVICE` environment variable to your desired
device name or address.

Example of an IPV4 address:

```
FDT_DEVICE="192.168.1.2" fx fdt
```

Example of an IPV6 address with an SSH port:

```
FDT_DEVICE="[2001:db8::10]:8022" fx fdt
```

Example of a device name:

```
FDT_DEVICE="step-atom-yard-juicy" fx fdt
```

## Use Fuchsia DevTools

You can use Fuchsia DevTools to do the following:

<p>
<a href="/docs/development/monitoring/devtools/devtools-cpu.md">Examine CPU Usage</a>
</p>
<p>
<a href="/docs/development/monitoring/devtools//devtools-memory.md">Examine memory usage</a>
</p>
<p>
<a href="/docs/development/monitoring/devtools/devtools-logging.md">Search and filter log messages from a device</a>
</p>
<p>
<a href="{/docs/development/monitoring/devtools/devtools-inspect.md">Inspect component properties for debugging</a>
</p>
<p>
<a href="/docs/development/monitoring/devtools/devtools-check-package.md">Check if an updated package exists on your device</a>
</p>

