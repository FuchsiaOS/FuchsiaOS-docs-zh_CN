# Fuchsia DevTools

Fuchsia DevTools provides real-time analytics on your Fuchsia
device and its components, allowing you to make informed inferences about
the state of your device.

## Available tools

Fuchsia DevTools provides the following information:

*   **Overview**
    *   Review a summary of your device's health, including CPU usage,
        memory usage, and process count.
*   **Processes**
    *   View a list of processes currently running on your device.
    *   View how much CPU and memory are being used by those processes running
    on your device.
    *   View custom graphs by selecting specific processes.
*   **CPU**
    *   View CPU usage by individual process or thread.
    *   Select multiple processes to build custom graphs.
    *   Select a time range in the graph to see processes running during
        that time and their CPU usage.
*   **Memory**
    *   View memory usage by individual process.
    *   Select multiple processes to build custom graphs
    *   Select a time range in the graph to see processes running during that
        time and their memory usage.
*   **Logging**
    *   Filter logs by severity, process, and keyword.
    *   Use negative search to filter out certain logs and reduce log spam.
    *   Review color-coded warning and error messages.
    *   Correlate logs to CPU usage.
*   **Packages**
    Note: This is hidden by default, but can be enabled through the Settings menu.
    *   Confirm that the package you've just built has been installed in your
        package server and cached on your device.
    *   Inspect the contents and sizes of packages in your package server or
        device.
    *   Confirm that the most recent version of a package exists on your device.
*   **Iquery**
    *   View components that are publishing data through the Inspect API.
    *   Search for and filter the nodes within a component’s Inspect tree.
    *   Display the raw JSON for any Inspect tree node.
    *   View histograms for components that publish histogram data to Inspect.


## Launch Fuchsia DevTools

Get started using [Fuchsia DevTools](/docs/development/monitoring/devtools/launch-devtools.md).

## Use Fuchsia DevTools

You can use Fuchsia DevTools to do the following:

<ul>
  <li><a href="/docs/development/monitoring/devtools/devtools-cpu.md">Examine CPU Usage</a></li>
  <li><a href="/docs/development/monitoring/devtools/devtools-memory.md">Examine memory usage</a></li>
  <li><a href="/docs/development/monitoring/devtools/devtools-logging.md">Search and filter log messages from a device</a></li>
  <li><a href="/docs/development/monitoring/devtools/devtools-inspect.md">Inspect component properties for debugging</a></li>
  <li><a href="/docs/development/monitoring/devtools/devtools-check-package.md">Check if an updated package exists on your device</a></li>
</ul>
