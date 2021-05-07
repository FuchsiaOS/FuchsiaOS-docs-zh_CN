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

```
rm -r examples/fidl/rust/client_sync/*
```

## Create a hello world component

1. Set up a hello world component in `examples/fidl/rust/client_sync`.
   You can name the component `echo-client`, and give the package a name of
   `echo-rust-client-sync`.

   Note: If necessary, refer back to the [previous tutorial][server-tut].

1. Once you have created your component, ensure that the following works:

   ```
   fx set core.x64 --with //examples/fidl/rust/client_sync
   ```

1. Build the Fuchsia image:

   ```
   fx build
   ```

1. In a separate terminal, run:

   ```
   fx serve
   ```

1. In a separate terminal, run:

   ```
   fx shell run fuchsia-pkg://fuchsia.com/echo-rust-client-sync#meta/echo-client.cmx
   ```

## Edit GN dependencies

1. Add the following dependencies to the `rustc_binary`:

   ```gn
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client/BUILD.gn" region_tag="deps" %}
   ```

1. Then, import them in `main.rs`:

   ```rust
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client/src/main.rs" region_tag="imports" %}
   ```

These dependencies are explained in the  [server tutorial][server-tut].

The one new dependency is `fuchsia-zircon`, which is a crate containing type safe
bindings for making Zircon kernel syscalls. In this example, the crate is used to
create a channel.

## Edit component manifest

1. Include the `Echo` protocol in the client component's sandbox by
   editing the component manifest in `client.cmx`.

   ```cmx
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client_sync/client.cmx" %}
   ```

## Connect to the server {#main}

The steps in this section explain how to add code to the `main()` function
that connects the client to the server and makes requests to it.

### Initialize a channel

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client_sync/src/main.rs" region_tag="main" highlight="2,3" %}
```

This channel will be used to communicate between the client and server.

### Connect to the server

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client_sync/src/main.rs" region_tag="main" highlight="4,5,6,7,8" %}
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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client/src/main.rs" region_tag="main" highlight="10,11,12,13,14,15" %}
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

If you run the client directly, it will not connect to the server correctly because the
client does not automatically get the `Echo` protocol provided in its
sandbox (in `/svc`). To get this to work, a launcher tool is provided
that launches the server, creates a new [`Environment`][environment] for
the client that provides the server's protocol, then launches the client in it.

1. Configure your GN build:

   ```
   fx set core.x64 --with //examples/fidl/rust/server --with
   //examples/fidl/rust/client_sync --with //examples/fidl/test:echo-launcher
   ```

2. Build the Fuchsia image:

   ```
   fx build
   ```

3. Run the launcher by passing it the client URL, the server URL, and
   the protocol that the server provides to the client:

   ```
   fx shell run fuchsia-pkg://fuchsia.com/echo-launcher#meta/launcher.cmx fuchsia-pkg://fuchsia.com/echo-rust-client-sync#meta/echo-client.cmx fuchsia-pkg://fuchsia.com/echo-rust-server#meta/echo-server.cmx fuchsia.examples.Echo
   ```

You should see the print output in the QEMU console (or using `fx log`).

```
[109100.005] 505615:505617> Listening for incoming connections...
[109100.099] 505615:505617> Received EchoString request for string "hello"
[109100.100] 505615:505617> Response sent successfully
[109100.100] 505864:505866> response: "hello"
[109100.100] 505615:505617> Received SendString request for string "hi"
[109100.100] 505615:505617> error sending event
[109100.100] 505615:505617>
[109100.100] 505615:505617> Caused by:
[109100.100] 505615:505617>     0: A server encountered an IO error writing a FIDL response to a channel: PEER_CLOSED
[109100.100] 505615:505617>     1: PEER_CLOSED
```

The server prints a `PEER_CLOSED` error when it tries to send the event, because the client
terminates immediately after sending the `SendString` request without waiting for the `OnString`
event. This is expected since synchronous clients cannot handle events. To see how to handle
events, try following the [async client tutorial instead][async-client]

<!-- xrefs -->
[server-tut]: /docs/development/languages/fidl/tutorials/rust/basics/server.md
[async-client]: /docs/development/languages/fidl/tutorials/rust/basics/client.md
[overview]: /docs/development/languages/fidl/tutorials/overview.md
[environment]: /docs/concepts/components/v2/environments.md
[request-pipelining]: /docs/development/languages/fidl/tutorials/rust/topics/request-pipelining.md
