# Implement a synchronous FIDL client in Rust

## Prerequisites

This tutorial assumes that you are familiar with writing and running a Fuchsia
component and with implementing a FIDL server, which are both covered in the
[FIDL server][server-tut] tutorial. For the full set of FIDL tutorials, refer
to the [overview][overview].

## Overview

This tutorial implements a client for a FIDL protocol and runs it
against the server created in the [previous tutorial][server-tut]. The client in this
tutorial is synchronous. There is an [alternate tutorial][async-client] for
asynchronous clients.

If you want to write the code yourself, delete the following directories:

```posix-terminal
rm -r examples/fidl/rust/client_sync/*
```

## Create the component

Create a new component project at `examples/fidl/rust/client_sync`:

1. Add a `main()` function to `examples/fidl/rust/client_sync/src/main.rs`:

   ```rust
   fn main() {
     println!("Hello, world!");
   }
   ```

1. Declare a target for the client in `examples/fidl/rust/client_sync/BUILD.gn`:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client_sync/BUILD.gn" region_tag="imports" %}

   # Declare an executable for the client.
   rustc_binary("bin") {
     name = "fidl_echo_rust_client_sync"
     edition = "2018"

     sources = [ "src/main.rs" ]
   }

   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client_sync/BUILD.gn" region_tag="rest" %}
   ```

1. Add a component manifest in `examples/fidl/rust/client_sync/meta/client.cml`:

   Note: The binary name in the manifest must match the output name of the
   `executable` defined in the previous step.

   ```json5
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client_sync/meta/client.cml" region_tag="example_snippet" %}
   ```

1. Once you have created your component, ensure that you can add it to the
   build configuration:

   ```posix-terminal
   fx set core.qemu-x64 --with //examples/fidl/rust/client_sync:echo-client
   ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

## Edit GN dependencies

1. Add the following dependencies to the `rustc_binary`:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client_sync/BUILD.gn" region_tag="deps" %}
   ```

1. Then, import them in `main.rs`:

   ```rust
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client_sync/src/main.rs" region_tag="imports" %}
   ```

These dependencies are explained in the [server tutorial][server-tut].

The one new dependency is `fuchsia-zircon`, which is a crate containing type safe
bindings for making Zircon kernel syscalls. In this example, the crate is used to
create a channel.

## Connect to the server {#main}

The steps in this section explain how to add code to the `main()` function
that connects the client to the server and makes requests to it.

### Initialize a channel

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client_sync/src/main.rs" region_tag="main" highlight="2,3" %}
```

This channel will be used to communicate between the client and server.

### Connect to the server

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client_sync/src/main.rs" region_tag="main" highlight="4,5,6,7,8" %}
```

`connect_channel_to_service` will bind the provided channel end to the specified
service. Under the hood, this call triggers a sequence of events that starts on the client and traces through the server code from the previous tutorial:

* Makes a request to the component framework containing the name of the service to connect to, and the
  other end of the channel. The name of the service is obtained implicitly using the `SERVICE_NAME`
  of `EchoMarker` template argument, similarly to how the service path is determined on the server
  end.
* This client object is returned from `connect_to_protocol`.

In the background, the request to the component framework gets routed to the server:

* When this request is received in the server process,
  it wakes up the `async::Executor` executor and tells it that the `ServiceFs` task can now make
  progress and should be run.
* The `ServiceFs` wakes up, sees the request available on the startup handle of the process, and
  looks up the name of the requested service in the list of `(service_name, service_startup_func)`
  provided through calls to `add_service`, `add_fidl_service`, etc. If a matching `service_name`
  exists, it calls `service_startup_func` with the provided channel to connect to the new service.
* `IncomingService::Echo` is called with a `RequestStream`
  (typed-channel) of the `Echo` FIDL protocol that is registered with `add_fidl_service`. The
  incoming request channel is stored in `IncomingService::Echo` and is added to the stream of
  incoming requests. `for_each_concurrent` consumes the `ServiceFs` into a [`Stream`] of type
  `IncomingService`. A handler is run for each entry in the stream, which matches over the incoming
  requests and dispatches to the `run_echo_server`. The resulting futures from each call to
  `run_echo_server` are run concurrently when the `ServiceFs` stream is `await`ed.
* When a request is sent on the channel, the channel the `Echo` service is becomes readable, which
  wakes up the asynchronous code in the body of `run_echo_server`.

Note: If any requests are sent before the server end of the channel is bound to
the server, these requests are buffered by the kernel. The server then reads
these requests as soon as the server is initialized.

### Send requests to the server

The code makes two requests to the server:

* An `EchoString` request
* A `SendString` request

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client/src/main.rs" region_tag="main" highlight="10,11,12,13,14,15" %}
```

The call to `echo_string` will block until a response is received from the server, and therefore
it takes a timeout argument as the last parameter.

On the other hand, the call to `send_string` does not have a timeout parameter since `SendString`
does not have a response. With the [current server implementation][server-tut], an `OnString` event
will be sent to the client after this request is received. However, the synchronous Rust bindings
do not have support for handling events.

The [bindings reference][bindings-ref] describes how these methods are generated, and the
[Fuchsia rustdoc][rustdoc] includes documentation for the generated FIDL crates.

## Run the client

In order for the client and server to communicate using the `Echo` protocol,
component framework must route the `fuchsia.examples.Echo` capability from the
server to the client. For this tutorial, a [realm][glossary.realm] component is
provided to declare the appropriate capabilities and routes.

Note: You can explore the full source for the realm component at
[`//examples/fidl/echo-realm`](/examples/fidl/echo-realm)

1. Configure your build to include the provided package that includes the
   echo realm, server, and client:

    ```posix-terminal
    fx set core.qemu-x64 --with //examples/fidl/rust:echo-rust-client-sync
    ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

    ```posix-terminal
    ffx component run fuchsia-pkg://fuchsia.com/echo-rust-client-sync#meta/echo_realm.cm
    ```

1. Start the `echo_client` instance:

    ```posix-terminal
    ffx component start /core/ffx-laboratory:echo_realm/echo_client
    ```

The server component starts when the client attempts to connect to the `Echo`
protocol. You should see output similar to the following in the device logs
(`ffx log`):

```none {:.devsite-disable-click-to-copy}
[echo_server][][I] Listening for incoming connections...
[echo_server][][I] Received EchoString request for string "hello"
[echo_server][][I] Response sent successfully
[echo_client][][I] response: "hello"
[echo_server][][I] Received SendString request for string "hi"
[echo_server][][I] Event sent successfully
[echo_client][][I] Received OnString event for string "hi"
```

Terminate the realm component to stop execution and clean up the component
instances:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo_realm
```

<!-- xrefs -->
[glossary.realm]: /glossary/README.md#realm
[server-tut]: /development/languages/fidl/tutorials/rust/basics/server.md
[async-client]: /development/languages/fidl/tutorials/rust/basics/client.md
[overview]: /development/languages/fidl/tutorials/overview.md
[environment]: /concepts/components/v2/environments.md
[request-pipelining]: /development/languages/fidl/tutorials/rust/topics/request-pipelining.md
