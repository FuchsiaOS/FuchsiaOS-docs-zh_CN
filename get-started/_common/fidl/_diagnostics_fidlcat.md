## Monitoring FIDL connections

The `fidlcat` tool allows you to monitor and debug FIDL connections to trace
individual FIDL messages sent and received by your component. Similar to the
Fuchsia debugger (`zxdb`), `fidlcat` connects to a running `debug_agent`
component on the target device and monitors running processes.

![Diagram showing how "fidlcat" interacts with the debug_agent service running
on a Fuchsia device to monitor and debug FIDL calls for a given process.]
(/docs/get-started/images/fidl/fidlcat.png){: width="592"}

Setting up the monitoring session requires the following high-level steps:

1.  Run the `debug_agent` component on the target device.
1.  Run the `fidlcat` client and connect to the target device.

The simplest method to start a FIDL debug session is to use the `ffx debug fidl`
command, which does all of these in the context of your local Fuchsia build.
However, these steps can also be performed manually if you need to configure
them separately.

Below is an example `fidlcat` message for a FIDL protocol request. The trace
output contains helpful information for each translation, including:

*   The component or process name
*   The system call invoked
*   The FIDL library, protocol, and method name
*   Message payload containing parameters or return values

```none {:.devsite-disable-click-to-copy}
echo-client.cm 256109:256122 zx_channel_read(handle:handle: e4c7c57f, options:uint32: 0, num_bytes:uint32: 48, num_handles:uint32: 0)
  -> ZX_OK
    received response fidl.examples.echo/Echo.EchoString = {
      response: string = "hello world!"
    }
```
Note: For complete details on `fidlcat` usage and options, see
[Monitor and debug your FIDL calls](/docs/development/monitoring/fidlcat).
