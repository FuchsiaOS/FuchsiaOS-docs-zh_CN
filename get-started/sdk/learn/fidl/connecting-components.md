# Connecting components

<<../../../_common/fidl/_connecting_intro.md>>

## Publishing a protocol implementation

Components that implement a FIDL protocol **declare** and **expose** that
protocol as a capability in their component manifest. This enables the component
framework to perform capability routing from this component to others in the
topology that request the capability.

```json5
{
    // ...
    capabilities: [
        { protocol: "fuchsia.example.Foo" },
    ],
    expose: [
        {
            protocol: "fuchsia.example.Foo",
            from: "self",
        },
    ],
}
```

Capability routing describes the access rights for the protocol, but it does
not establish the necessary endpoints for a connection. Components must publish
the implementation as an `/svc/` handle in the outgoing directory using the
[fuchsia.io](https://fuchsia.dev/reference/fidl/fuchsia.io) protocol. The
generated FIDL bindings wrap this handle and enable the provider to connect a
request handle to begin receiving FIDL messages.

```cpp
// Serve the protocol
FooImplementation instance;
fidl::Binding<fuchsia::example::Foo> binding(&instance);
instance.event_sender_ = &binding.events();
fidl::InterfaceRequestHandler<fuchsia::example::Foo> handler =
    [&](fidl::InterfaceRequest<fuchsia::example::Foo> request) {
      binding.Bind(std::move(request));
    };
context->outgoing()->AddPublicService(std::move(handler));
```

## Connecting to a protocol implementation

Client components declare the protocol as a required capability in their
component manifest. This allows the component framework to determine whether
the component has the rights to access protocol implementation. If a valid route
exists, the component's namespace contains a corresponding `/svc/` handle.

```json5
{
    // ...
    use: [
        { protocol: "fuchsia.example.Foo" },
    ],
}
```

<aside class="key-point">
Recall that capabilities are routed explicitly between components, so the
topology must include a connected set of offers between components for routing
to succeed.
</aside>

The client component uses the
[fuchsia.io](https://fuchsia.dev/reference/fidl/fuchsia.io) protocol to
establish a connection to the protocol implementation and open a channel. The
generated FIDL bindings wrap this channel and enable the client to begin sending
messages to the provider.

```cpp
// Connect to FIDL protocol
fuchsia::example::FooSyncPtr proxy;
auto context = sys::ComponentContext::Create();
context->svc()->Connect(proxy.NewRequest());
```

## Exercise: Echo server and client

In this section, you'll use the generated FIDL bindings for
`examples.routing.echo` to implement client and server components.

<<../_common/_start_femu_with_packages.md>>

### Create the server component

Begin by creating a new component project to implement the echo server. This
component will serve the `Echo` protocol and handle incoming requests.

Create a new project directory in your Bazel workspace for a new component
called `echo_server`:

```posix-terminal
mkdir -p fuchsia-codelab/echo-server
```

After you complete this section, the project should have the following directory
structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/echo-server
                  |- BUILD.bazel
                  |- meta
                  |   |- echo_server.cml
                  |
                  |- main.cc
```

Create the `echo-server/meta/echo_server.cml` component manifest, declare the
`Echo` protocol as a capability provided by the server component, and expose it
for use by the parent realm:

`echo-server/meta/echo_server.cml`:

```json5
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_server/meta/echo_server.cml" region_tag="example_snippet" adjust_indentation="auto" %}
```

Add the following `BUILD.bazel` rules to build and package the server component:

`echo-server/BUILD.bazel`:

{% set build_bazel_snippet %}
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_server/BUILD.bazel" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_server/BUILD.bazel" region_tag="cc_binary" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_server/BUILD.bazel" region_tag="component" adjust_indentation="auto" %}
{% endset %}

```bazel
{{ build_bazel_snippet|replace("//src/routing/fidl","//fuchsia-codelab/echo-fidl")|trim() }}
```

### Implement the server

Open the main source file and replace the import statements with the
following code:

`echo-server/main.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_server/main.cc" region_tag="imports" adjust_indentation="auto" %}
```

Add the following code to implement the protocol request handler:

`echo-server/main.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_server/main.cc" region_tag="handler" adjust_indentation="auto" %}
```

Each `Echo` protocol method has a corresponding override function
(`EchoString()`) and includes a callback interface to send back the return value.

This implementation simply "echoes" the same string value from the request back
in the response payload.

Add the following code to `main()` to serve the `Echo` protocol:

`echo-server/main.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_server/main.cc" region_tag="main_body" adjust_indentation="auto" %}
```

This code performs the following steps to serve the `Echo` protocol:

1.  Initialize `ComponentContext` and add an entry under
    `/svc/examples.routing.echo.Echo` in the outgoing directory.
1.  Serve the directory and begin listening for incoming connections.
1.  Attach the `EchoImplementation` instance as a request handler for any
    matching `Echo` requests.

### Create the client component

Create another new component project to implement the echo client. This
component will connect to the protocol implementation and send requests.

Create a new project directory in your Bazel workspace for a new component
called `echo_client`:

```posix-terminal
mkdir -p fuchsia-codelab/echo-client
```

After you complete this section, the project should have the following directory
structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/echo-client
                  |- BUILD.bazel
                  |- meta
                  |   |- echo_client.cml
                  |
                  |- main.cc
```

Create the `echo-client/meta/echo_client.cml` component manifest and configure
the client component to request the `examples.routing.echo.Echo` capability
exposed by the server:

`echo-client/meta/echo_client.cml`:

```json5
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_client/meta/echo_client.cml" region_tag="example_snippet" adjust_indentation="auto" %}
```

Add the following `BUILD.bazel` rules to build and package the client component:

`echo-client/BUILD.bazel`:

{% set build_bazel_snippet %}
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_client/BUILD.bazel" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_client/BUILD.bazel" region_tag="cc_binary" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_client/BUILD.bazel" region_tag="component" adjust_indentation="auto" %}
{% endset %}

```bazel
{{ build_bazel_snippet|replace("//src/routing/fidl","//fuchsia-codelab/echo-fidl")|trim() }}
```

### Implement the client

Similar to `echo`, the client passes the program arguments as a message
to the server. Recall that the arguments are described in the `program` block of
`echo_client.cml`:

`echo-client/meta/echo_client.cml`:

```json5 {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_client/meta/echo_client.cml" region_tag="program_args" adjust_indentation="auto" highlight="9,10" %}
```

Open the main source file and replace the import statements with the
following code:

`echo-client/main.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_client/main.cc" region_tag="imports" adjust_indentation="auto" %}
```

Add the following code to `main()` to connect to the `Echo` protocol and send
a request:

`echo-client/main.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/cpp/echo_client/main.cc" region_tag="main_body" adjust_indentation="auto" %}
```

The `EchoSyncPtr` provides a wrapper to connect to the exposed capability by
name and returns a handle to the open proxy interface. This proxy contains
the `EchoString()` FIDL protocol method.

### Integrate the components

The capabilities provided by the server must be routed to the client through
the component framework. To enable this, you will implement a realm component
to act as the parent and manage capability routing.

Create a new project directory for the realm component definition:

```posix-terminal
mkdir -p fuchsia-codelab/echo-realm
```

After you complete this section, the project should have the following directory
structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/echo-realm
                  |- BUILD.bazel
                  |- meta
                      |- echo_realm.cml
```

Create a new component manifest file `echo-realm/meta/echo_realm.cml` with the
following contents:

`echo-realm/meta/echo_realm.cml`:

```json5
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/meta/echo_realm.cml" region_tag="example_snippet" adjust_indentation="auto" %}
```

This creates a component realm with the server and client as child components,
and routes the `examples.routing.echo.Echo` protocol capability to the
client.

Add a `BUILD.bazel` file to create a build target for the realm component and a
Fuchsia package containing the server and client:

`echo-realm/BUILD.bazel`:

{% set build_bazel_snippet %}
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/BUILD.bazel"  region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/BUILD.bazel"  region_tag="component" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/BUILD.bazel"  region_tag="package" adjust_indentation="auto" %}
{% endset %}

```bazel
{{ build_bazel_snippet
    |replace("components_routing_example","echo-realm")
    |replace("//src/routing/cpp/echo_client","//fuchsia-codelab/echo-client")
    |replace("//src/routing/cpp/echo_server","//fuchsia-codelab/echo-server")
    |trim() }}
```

Build and publish the package to the `fuchsiasamples.com` repository:

```posix-terminal
bazel run //fuchsia-codelab/echo-realm:pkg.publish -- \
    --repo_name fuchsiasamples.com
```

### Add the components to the topology

You will add your component to the `ffx-laboratory` — a restricted collection
used for development inside the product's **core realm**. Collections enable
components to be dynamically created and destroyed at runtime.

Create the component instances by passing the `echo-realm` component URL and
an appropriate moniker inside `ffx-laboratory` to `ffx component create`:

```posix-terminal
ffx component create /core/ffx-laboratory:echo-realm \
    fuchsia-pkg://fuchsiasamples.com/echo-realm#meta/echo_realm.cm
```

Then, resolve the `echo-realm` component with `ffx component resolve`:

```posix-terminal
ffx component resolve /core/ffx-laboratory:echo-realm
```

Verify that instances of the server and client were also created as child
components using `ffx component show`:

```posix-terminal
ffx component show echo
```

```none {:.devsite-disable-click-to-copy}
               Moniker: /core/ffx-laboratory:echo-realm/echo_client
                   URL: #meta/echo_client.cm
                  Type: CML static component
       Component State: Unresolved
       Execution State: Stopped

               Moniker: /core/ffx-laboratory:echo-realm/echo_server
                   URL: #meta/echo_server.cm
                  Type: CML static component
       Component State: Unresolved
       Execution State: Stopped

               Moniker: /core/ffx-laboratory:echo-realm
                   URL: fuchsia-pkg://fuchsiasamples.com/echo-realm#meta/echo_realm.cm
                  Type: CML dynamic component
       Component State: Resolved
       Execution State: Stopped
           Merkle root: 666c40477785f89b0ace22b30d65f1338f1d308ecceacb0f65f5140baa889e1b
```

### Verify the component interactions

Start the existing client component instance using `ffx component start`:

```posix-terminal
ffx component start /core/ffx-laboratory:echo-realm/echo_client
```
Open another terminal window and verify the log output from the client component:

```posix-terminal
ffx log --filter echo
```

You should see the following output in the device logs:

```none {:.devsite-disable-click-to-copy}
[echo_client][I] Server response: Hello, Fuchsia!
```

The server component starts once the client makes a connection to the
`examples.routing.echo.Echo` capability and continues running to serve
additional FIDL requests.

Use `ffx component show` the see the echo server running in the component
instance tree:

```posix-terminal
ffx component show echo_server
```

```none {:.devsite-disable-click-to-copy}
               Moniker: /core/ffx-laboratory:echo-realm/echo_server
                   URL: #meta/echo_server.cm
                  Type: CML static component
       Component State: Resolved
 Incoming Capabilities: fuchsia.logger.LogSink
  Exposed Capabilities: diagnostics
                        examples.routing.echo.Echo
       Execution State: Running
                Job ID: 474691
            Process ID: 474712
           Running for: 2026280474361 ticks
           Merkle root: 666c40477785f89b0ace22b30d65f1338f1d308ecceacb0f65f5140baa889e1b
 Outgoing Capabilities: diagnostics
                        examples.routing.echo.Echo
```

### Destroy the instance

Clean up the `echo-realm` instance using the following command:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo-realm
```
