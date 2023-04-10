# Connecting components

<<../../_common/fidl/_connecting_intro.md>>

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

* {Rust}

  ```rust
  let mut service_fs = ServiceFs::new_local();

  // Serve the protocol
  service_fs.dir("svc").add_fidl_service(PROTOCOL_NAME);
  service_fs.take_and_serve_directory_handle().context("failed to serve outgoing namespace")?;
  ```

* {C++}

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

* {Rust}

  ```rust
  // Connect to FIDL protocol
  let protocol = connect_to_protocol::<FooMarker>().expect("error connecting to echo");
  ```

* {C++}

  ```cpp
  // Connect to FIDL protocol
  fuchsia::example::FooSyncPtr proxy;
  auto context = sys::ComponentContext::Create();
  context->svc()->Connect(proxy.NewRequest());
  ```

## Exercise: Echo server and client

In this section, you'll use the generated FIDL bindings for
`fidl.examples.routing.echo` to implement client and server components in Rust.

<<../_common/_start_femu_with_packages.md>>

### Create the server component

Begin by creating a new component project to implement the echo server. This
component will serve the `Echo` protocol and handle incoming requests.

Create a project scaffold for a new component called `echo-server` in the
`//vendor/fuchsia-codelab` directory:

```posix-terminal
mkdir -p vendor/fuchsia-codelab/echo-server
```

Create the following file and directory structure in the new project directory:

* {Rust}

  ```none {:.devsite-disable-click-to-copy}
  //vendor/fuchsia-codelab/echo-server
                          |- BUILD.gn
                          |- meta
                          |   |- echo.cml
                          |
                          |- src
                              |- main.rs
  ```

* {C++}

  ```none {:.devsite-disable-click-to-copy}
  //vendor/fuchsia-codelab/echo-server
                          |- BUILD.gn
                          |- meta
                          |   |- echo.cml
                          |
                          |- main.cc
  ```

Add the following build rules to your `BUILD.gn` file to build and package the server component:

{% set gn_rust_binary %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_server/BUILD.gn" region_tag="executable" adjust_indentation="auto" %}
{% endset %}

{% set gn_rust_component %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_server/BUILD.gn" region_tag="component" adjust_indentation="auto" %}
{% endset %}

{% set gn_cpp_binary %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_server/BUILD.gn" region_tag="executable" adjust_indentation="auto" %}
{% endset %}

{% set gn_cpp_component %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_server/BUILD.gn" region_tag="component" adjust_indentation="auto" %}
{% endset %}

* {Rust}

  `echo-server/BUILD.gn`:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_server/BUILD.gn" region_tag="imports" adjust_indentation="auto" %}

  {{ gn_rust_binary|replace("echo_server_rust","echo-server")|replace("//examples/components/routing/fidl","//vendor/fuchsia-codelab/echo-fidl")|trim() }}

  {{ gn_rust_component|replace("echo_server_component","component")|trim() }}

  fuchsia_package("echo-server") {
    package_name = "echo-server"
    deps = [ ":component" ]
  }
  ```

* {C++}

  `echo-server/BUILD.gn`:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_server/BUILD.gn" region_tag="imports" adjust_indentation="auto" %}

  {{ gn_cpp_binary|replace("echo_server_cpp","echo-server")|replace("//examples/components/routing/fidl","//vendor/fuchsia-codelab/echo-fidl")|trim() }}

  {{ gn_cpp_component|replace("echo_server_component","component")|trim() }}

  fuchsia_package("echo-server") {
    package_name = "echo-server"
    deps = [ ":component" ]
  }
  ```

Declare the `Echo` protocol as a capability provided by the server component,
and expose it for use by the parent realm:

{% set cml_rust %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_server/meta/echo_server.cml" region_tag="example_snippet" adjust_indentation="auto" %}
{% endset %}

{% set cml_cpp %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_server/meta/echo_server.cml" region_tag="example_snippet" adjust_indentation="auto" %}
{% endset %}

* {Rust}

  `echo-server/meta/echo_server.cml`:

  ```json5
  {{ cml_rust|replace("echo_server_rust","echo-server")|trim() }}
  ```

* {C++}

  `echo-server/meta/echo_server.cml`:

  ```json5
  {{ cml_cpp|replace("echo_server_cpp","echo-server")|trim() }}
  ```

### Implement the server

Open the main source file and replace the import statements with the
following code:

* {Rust}

  `echo-server/src/main.rs`:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_server/src/main.rs" region_tag="imports" adjust_indentation="auto" %}
  ```

  <aside class="key-point">
  FIDL bindings for Rust generate a Rust library crate named after the FIDL
  <code>library</code>. For the <code>fidl.examples.routing.echo</code> library,
  the generated crate name is <code>fidl_fidl_examples_routing_echo</code>. For
  more details, see
  <a href="/docs/reference/fidl/bindings/rust-bindings">Rust bindings</a>.
  </aside>

* {C++}

  `echo-server/main.cc`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_server/main.cc" region_tag="imports" adjust_indentation="auto" %}
  ```

Add the following code to `main()` to serve the `Echo` protocol:

* {Rust}

  `echo-server/src/main.rs`:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_server/src/main.rs" region_tag="main_body" adjust_indentation="auto" %}
  ```

  This code performs the following steps to serve the `Echo` protocol:

  1.  Initialize `ServiceFs` and add an entry under
      `/svc/fidl.examples.routing.echo.Echo` in the outgoing directory.
  1.  Serve the directory and begin listening for incoming connections.
  1.  Attach the `handle_echo_request()` function as a request handler for any
      matching `Echo` requests.

* {C++}

  `echo-server/main.cc`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_server/main.cc" region_tag="main_body" adjust_indentation="auto" %}
  ```

  This code performs the following steps to serve the `Echo` protocol:

  1.  Initialize `ComponentContext` and add an entry under
      `/svc/fidl.examples.routing.echo.Echo` in the outgoing directory.
  1.  Serve the directory and begin listening for incoming connections.
  1.  Attach the `EchoImplementation` instance as a request handler for any
      matching `Echo` requests.

Add the following code to implement the protocol request handler:

* {Rust}

  `echo-server/src/main.rs`:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_server/src/main.rs" region_tag="handler" adjust_indentation="auto" %}
  ```

  Each request in the `EchoRequestStream` is typed by the method name
  (`EchoString`) and includes a responder interface to send back the return value.


* {C++}

  `echo-server/main.cc`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_server/main.cc" region_tag="handler" adjust_indentation="auto" %}
  ```

  Each `Echo` protocol method has a corresponding override function
  (`EchoString()`) and includes a callback interface to send back the return value.

This implementation simply "echoes" the same string value from the request back
in the response payload.

### Create the client component

Create another new component project to implement the echo client. This
component will connect to the protocol implementation and send requests.

Create a project scaffold for a new component called `echo-client` in the
`//vendor/fuchsia-codelab` directory:

```posix-terminal
mkdir -p vendor/fuchsia-codelab/echo-client
```

Create the following file and directory structure in the new project directory:

* {Rust}

  ```none {:.devsite-disable-click-to-copy}
  //vendor/fuchsia-codelab/echo-client
                          |- BUILD.gn
                          |- meta
                          |   |- echo.cml
                          |
                          |- src
                              |- main.rs
  ```

* {C++}

  ```none {:.devsite-disable-click-to-copy}
  //vendor/fuchsia-codelab/echo-client
                          |- BUILD.gn
                          |- meta
                          |   |- echo.cml
                          |
                          |- main.cc
  ```

Add the following build rules to your `BUILD.gn` file to build and package the client component:

{% set gn_rust_binary %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_client/BUILD.gn" region_tag="executable" adjust_indentation="auto" %}
{% endset %}

{% set gn_rust_component %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_client/BUILD.gn" region_tag="component" adjust_indentation="auto" %}
{% endset %}

{% set gn_cpp_binary %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_client/BUILD.gn" region_tag="executable" adjust_indentation="auto" %}
{% endset %}

{% set gn_cpp_component %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_client/BUILD.gn" region_tag="component" adjust_indentation="auto" %}
{% endset %}

* {Rust}

  `echo-client/BUILD.gn`:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_client/BUILD.gn" region_tag="imports" adjust_indentation="auto" %}

  {{ gn_rust_binary|replace("echo_client_rust","echo-client")|replace("//examples/components/routing/fidl","//vendor/fuchsia-codelab/echo-fidl")|trim() }}

  {{ gn_rust_component|replace("echo_client_component","component")|trim() }}

  fuchsia_package("echo-client") {
    package_name = "echo-client"
    deps = [ ":component" ]
  }
  ```

* {C++}

  `echo-client/BUILD.gn`:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_client/BUILD.gn" region_tag="imports" adjust_indentation="auto" %}

  {{ gn_cpp_binary|replace("echo_client_cpp","echo-client")|replace("//examples/components/routing/fidl","//vendor/fuchsia-codelab/echo-fidl")|trim() }}

  {{ gn_cpp_component|replace("echo_client_component","component")|trim() }}

  fuchsia_package("echo-client") {
    package_name = "echo-client"
    deps = [ ":component" ]
  }
  ```

Configure the client's component manifest to request the
`fidl.examples.routing.echo.Echo` capability exposed by the server:

{% set cml_rust %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_client/meta/echo_client.cml" region_tag="example_snippet" adjust_indentation="auto" %}
{% endset %}

{% set cml_cpp %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_client/meta/echo_client.cml" region_tag="example_snippet" adjust_indentation="auto" %}
{% endset %}

* {Rust}

  `echo-client/meta/echo_client.cml`:

  ```json5
  {{ cml_rust|replace("echo_client_rust","echo-client")|trim() }}
  ```

* {C++}

  `echo-client/meta/echo_client.cml`:

  ```json5
  {{ cml_cpp|replace("echo_client_cpp","echo-client")|trim() }}
  ```

### Implement the client

Similar to `echo-args`, the client passes the program arguments as a message
to the server. Add the following program arguments to `echo_client.cml`:

{% set cml_rust %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_client/meta/echo_client.cml" region_tag="program_args" adjust_indentation="auto" highlight="9,10" %}
{% endset %}

{% set cml_cpp %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_client/meta/echo_client.cml" region_tag="program_args" adjust_indentation="auto" highlight="9,10" %}
{% endset %}

* {Rust}

  `echo-client/meta/echo_client.cml`:

  ```json5
  {{ cml_rust|replace("echo_client_rust","echo-client")|trim() }}
  ```

* {C++}

  `echo-client/meta/echo_client.cml`:

  ```json5
  {{ cml_cpp|replace("echo_client_cpp","echo-client")|trim() }}
  ```

Open the main source file and replace the import statements with the following code:

* {Rust}

  `echo-client/src/main.rs`:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_client/src/main.rs" region_tag="imports" adjust_indentation="auto" %}
  ```

* {C++}

  `echo-client/main.cc`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_client/main.cc" region_tag="imports" adjust_indentation="auto" %}
  ```

Add the following code to `main()` to connect to the `Echo` protocol and send
a request:

* {Rust}

  `echo-client/src/main.rs`:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_client/src/main.rs" region_tag="main_body" adjust_indentation="auto" %}
  ```

  The `EchoMarker` provides a wrapper to connect to the exposed capability by
  name and returns a handle to the open `EchoProxy` interface. This proxy contains
  the `echo_string()` FIDL protocol method.

* {C++}

  `echo-client/main.cc`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_client/main.cc" region_tag="main_body" adjust_indentation="auto" %}
  ```

  The `EchoSyncPtr` provides a wrapper to connect to the exposed capability by
  name and returns a handle to the open proxy interface. This proxy contains
  the `EchoString()` FIDL protocol method.

### Integrate the components

The capabilities provided by the server must be routed to the client through
the component framework. To enable this, you will implement a realm component
to act as the parent and manage capability routing.

Create a new project directory for the realm product definition:

```posix-terminal
mkdir -p vendor/fuchsia-codelab/echo-realm
```

Create the following file and directory structure in the new project directory:

```none {:.devsite-disable-click-to-copy}
//vendor/fuchsia-codelab/echo-realm
                        |- BUILD.gn
                        |- meta
                        |   |- echo_realm.cml
```

Create a new component manifest file `meta/echo_realm.cml` with the
following contents:

`echo-realm/meta/echo_realm.cml`:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/meta/echo_realm.cml" region_tag="example_snippet" adjust_indentation="auto" %}
```

This creates a component realm with the server and client as child components, and routes the `fidl.examples.routing.echo.Echo` protocol capability to the client.

Add a `BUILD.gn` file to create a build target for the realm component:

`echo-realm/BUILD.gn`:

```gn
import("//build/components.gni")

fuchsia_component("echo_realm") {
  manifest = "meta/echo_realm.cml"
}

fuchsia_package("echo-realm") {
  deps = [
    ":echo_realm",
    "//vendor/fuchsia-codelab/echo-server:component",
    "//vendor/fuchsia-codelab/echo-client:component",
  ]
}
```

Update the build configuration to include the new components:

```posix-terminal
fx set workstation_eng.qemu-x64 \
    --with //vendor/fuchsia-codelab/echo-server \
    --with //vendor/fuchsia-codelab/echo-client \
    --with //vendor/fuchsia-codelab/echo-realm
```

Run `fx build` again to build the components:

```posix-terminal
fx build
```

### Add the components to the topology

You will add your component to the `ffx-laboratory` — a restricted collection
used for development inside the product's **core realm**. Collections enable
components to be dynamically created and destroyed at runtime.

Create the component instances by passing the `echo-realm` component URL and
an appropriate moniker inside `ffx-laboratory` to `ffx component create`:

```posix-terminal
ffx component create /core/ffx-laboratory:echo-realm \
    fuchsia-pkg://fuchsia.com/echo-realm#meta/echo_realm.cm
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
                   URL: fuchsia-pkg://fuchsia.com/echo-realm#meta/echo_realm.cm
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
`fidl.examples.routing.echo.Echo` capability and continues running to serve
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
                        fidl.examples.routing.echo.Echo
       Execution State: Running
                Job ID: 474691
            Process ID: 474712
           Running for: 2026280474361 ticks
           Merkle root: 666c40477785f89b0ace22b30d65f1338f1d308ecceacb0f65f5140baa889e1b
 Outgoing Capabilities: diagnostics
                        fidl.examples.routing.echo.Echo
```

### Destroy the instance

Clean up the `echo-realm` instance using the following command:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo-realm
```
