
# Diagnostics and monitoring

<<../../../_common/fidl/_diagnostics_intro.md>>

<<../../../_common/fidl/_diagnostics_fidlcat.md>>

<<../../../_common/fidl/_diagnostics_inspect.md>>

## Exercise: Monitoring provider components

In this section, you'll use the diagnostics tools to monitor the health and
behavior of the echo server component.

<<../_common/_start_femu_with_packages.md>>

### Monitor FIDL traffic

Create a new instance of the `echo-realm` containing the server and client
components:

```posix-terminal
ffx component run /core/ffx-laboratory:echo-realm fuchsia-pkg://fuchsiasamples.com/echo-realm#meta/echo_realm.cm
```

You can use `fidlcat` to monitor and debug the FIDL connections in your
components. Launch `ffx debug fidl` and configure it to monitor the echo server
component:

```posix-terminal
ffx debug fidl --remote-name echo_server.cm --fidl-ir-path bazel-out/
```

```none {:.devsite-disable-click-to-copy}
Checking for debug agent on [fe80::d6c5:4526:c282:fb6%qemu]:2345.
Debug agent not found. Starting one.
INFO: [main.cc(238)] Connected to symbol server gs://fuchsia-artifacts-release/debug
INFO: [main.cc(122)] Connecting to port 2345 on fe80::d6c5:4526:c282:fb6%qemu...
INFO: [main.cc(92)] Connected!
```

<aside class="key-point">
The <code>--remote-name</code> parameter configures <code>fidlcat</code> to
watch for a matching process name and attach as soon as it is discovered. You
can use <code>--remote-pid</code> to attach to an existing process.
</aside>

Initiate a FIDL connection to the server by starting an echo client instance:

```posix-terminal
ffx component start /core/ffx-laboratory:echo-realm/echo_client
```

The client binds to the server component and communicates using the `Echo`
FIDL protocol. Review the `ffx debug fidl` output to see a list of the FIDL
transactions handled by echo server:

```none {:.devsite-disable-click-to-copy}
Monitoring echo_server.cm

echo_server.cm 58694:58696 zx_channel_read_etc(handle: handle = fb9b5273, options: uint32 = 0, num_bytes: uint32 = 512, num_handles: uint32 = 4)
  -> ZX_OK
    received request fuchsia.io/Directory.Open = { flags: uint32 = 3, mode: uint32 = 493, path: string = "svc/examples.routing.echo.Echo", object: handle = Channel:f93b597b(ZX_RIGHT_TRANSFER | ZX_RIGHT_READ | ZX_RIGHT_WRITE | ZX_RIGHT_SIGNAL | ZX_RIGHT_SIGNAL_PEER | ZX_RIGHT_WAIT | ZX_RIGHT_INSPECT)(channel:0:svc/examples.routing.echo.Echo) }

echo_server.cm 58694:58696 zx_channel_read_etc(handle: handle = Channel:f93b597b(channel:0:svc/examples.routing.echo.Echo), options: uint32 = 0, num_bytes: uint32 = 512, num_handles: uint32 = 4)
  -> ZX_OK
    received request examples.routing.echo/Echo.EchoString = { value: string = "Hello, Fuchsia" }

echo_server.cm 58694:58696 zx_channel_write_etc(handle: handle = Channel:f93b597b(channel:0:svc/examples.routing.echo.Echo), options: uint32 = 0)
  sent response examples.routing.echo/Echo.EchoString = { response: string = "Hello, Fuchsia" }
  -> ZX_OK

echo_server.cm 58694:58696 zx_channel_read_etc(handle: handle = Channel:f93b597b(channel:0:svc/examples.routing.echo.Echo), options: uint32 = 0, num_bytes: uint32 = 512, num_handles: uint32 = 4)
  -> ZX_ERR_PEER_CLOSED

echo_server.cm 58694:58696 zx_handle_close(handle: handle = Channel:f93b597b(channel:0:svc/examples.routing.echo.Echo))
  -> ZX_OK
```

Notice the sequence of events:

1.  A channel to the protocol implementation opens at
    `svc/examples.routing.echo.Echo`.
1.  The server receives an `Echo.EchoString` request over the open channel,
    containing the string payload sent by the client.
1.  The server sends a corresponding response with the same string payload.
1.  The channel closes.

By tracing the FIDL connections between your components, `fidlcat` enables you
to find and diagnose potential issues such as failed connections or invalid
data payloads.

<aside class="key-point">
<b>Extra credit</b>
<p>Monitor the FIDL connections in <code>echo-client.cm</code> using
<code>fidlcat</code>. Do you see anything different from the messages recorded
from echo server?</p>
</aside>

### Add request tracking

Component inspection allows you to publish diagnostic information from your
components to assist in debugging. You'll use the Inspect API to track some
usage statistics for the echo server component.

Update the `echo_server` request handler to accept a new struct containing
numeric Inspect properties for request count and bytes processed.
The handler increments these properties on each incoming request:

`echo-server/main.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/inspect_server/main.cc" region_tag="handler" adjust_indentation="auto" %}
```

Add the following code to `main()` to initialize the Inspect propertes and pass
them to the updated handler:

`echo-server/main.cc`:

```cpp
int main(int argc, const char** argv) {
  // ...

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/inspect_server/main.cc" region_tag="echo_instance" %}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/inspect_server/main.cc" region_tag="add_properties" %} {{ '</strong>' }}

  // ...
}
```

<aside class="key-point">
<b>Health checks</b>
<p>The component template includes code that sets the component health status.
This is a standardized inspection metric for component health, which provides a
convenient way to report whether your component is running well or experiencing
an issue. You'll find this status reported under
<code>fuchsia.inspect.Health</code> in the Inspect tree.</p>
<p>For more details on this metric, see
<a href="/docs/concepts/diagnostics/inspect/health">Health check</a>.</p>
</aside>

Finally, update the imports to include the new Inspect libraries:

`echo-server/main.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/inspect_server/main.cc" region_tag="imports" adjust_indentation="auto" %}
```

Build and publish the updated package to the `fuchsiasamples.com` repository:

```posix-terminal
bazel run //fuchsia-codelab/echo-realm:pkg.publish -- \
    --repo_name fuchsiasamples.com
```

### Verify the Inspect data

Create a new `echo-realm` component containing the updated `echo-server`:

```posix-terminal
ffx component run /core/ffx-laboratory:echo-realm fuchsia-pkg://fuchsiasamples.com/echo-realm#meta/echo_realm.cm \
    --recreate
```

Run the echo client component multiple times. This causes the request count in
`echo-server` to increment with each connection:

```posix-terminal
ffx component start /core/ffx-laboratory:echo-realm/echo_client

ffx component start /core/ffx-laboratory:echo-realm/echo_client

ffx component start /core/ffx-laboratory:echo-realm/echo_client
```

View the available Inspect data for the echo server component with
`ffx inspect`. You'll see the values for request count and bytes processed in
the tree under the `root` node alongside the component health status:

```posix-terminal
ffx inspect show 'core/ffx-laboratory\:echo-realm/echo_server'
```

```none {:.devsite-disable-click-to-copy}
core/ffx-laboratory\:echo-realm/echo_server:
  metadata:
    filename = fuchsia.inspect.Tree
    component_url = #meta/echo_server.cm
    timestamp = 1476246046122
  payload:
    root:
      bytes_processed = 42
      total_requests = 3
      fuchsia.inspect.Health:
        start_timestamp_nanos = 1467828507317
        status = OK
```

Publishing health and behavior information using Inspect enables you to observe
the current state of your components and diagnose issues on production devices.

<aside class="key-point">
<b>Extra credit</b>
<p>Use <code>ffx target snapshot</code> tool to capture a debug snapshot from
the target device. This archive also contains the Inspect data in a JSON
format. Can you locate the same echo server data returned using
<code>ffx inspect</code>?</p>
</aside>

### Destroy the instance

Clean up the `echo-realm` instance using the following command:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo-realm
```

## What's next?

Congratulations! You've successfully built a Fuchsia IPC interface using FIDL,
and connected two components together using that interface.

You have completed all the modules in this course! Take your newfound
understanding to the next level and dive deeper into the:

<a class="button button-primary"
    href="/docs/concepts">Fuchsia concepts</a>
