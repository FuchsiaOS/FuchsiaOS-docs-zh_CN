# Implement a FIDL client in Rust

## Prerequisites

This tutorial assumes that you are familiar with writing and running a Fuchsia
component and with implementing a FIDL server, which are both covered in the
[FIDL server][server-tut] tutorial. For the full set of FIDL tutorials, refer
to the [overview][overview].

## Overview

This tutorial implements a client for a FIDL protocol and runs it
against the server created in the [previous tutorial][server-tut]. The client in this
tutorial is asynchronous. There is an [alternate tutorial][sync-client] for
synchronous clients.

If you want to write the code yourself, delete the following directories:

```
rm -r examples/fidl/rust/client/*
```

## Create a hello world component

1. Set up a hello world component in `examples/fidl/rust/client`.
   You can name the component `echo-client`, and give the package a name of
   `echo-rust-client`.

   Note: If necessary, refer back to the [previous tutorial][server-tut].

1. Once you have created your component, ensure that the following works:

   ```
   fx set core.x64 --with //examples/fidl/rust/client
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
   fx shell run fuchsia-pkg://fuchsia.com/echo-rust-client#meta/echo-client.cmx
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

## Edit component manifest

1. Include the `Echo` protocol in the client component's sandbox by
   editing the component manifest in `client.cmx`.

   ```cmx
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client/client.cmx" %}
   ```

## Connect to the server {#main}

The steps in this section explain how to add code to the `main()` function
that connects the client to the server and makes requests to it.

### Connect to the server

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client/src/main.rs" region_tag="main" highlight="3,4" %}
```

Under the hood, this call triggers a sequence of events that starts on the client and traces
through the server code from the previous tutorial.

* Initialize a client object, as well as a channel. The client object is bound to one end of the
  channel.
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

### Send requests to the server

The code makes two requests to the server:

* An `EchoString` request
* A `SendString` request

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client/src/main.rs" region_tag="main" highlight="6,7,8,9,10,11" %}
```

The call to `EchoString` returns a future, which resolves to the response returned by the server.
The returned future will resolve to an error if there is either an error sending the request or
receiving the response (e.g. when decoding the message, or if an epitaph was received).

On the other hand, the call to `SendString` returns a `Result`, since it is a fire and forget
method. This method call will return an error if there was an issue sending the request.

The [bindings reference][bindings-ref] describes how these proxy methods are generated, and the
[Fuchsia rustdoc][rustdoc] includes documentation for the generated FIDL crates.

### Handle incoming events

The code then waits for a single `OnString` event from the server:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/client/src/main.rs" region_tag="main" highlight="12,13,14,15" %}
```

This is done by [taking the event stream][events] from the client object, then waiting
for a single event from it.

## Run the client

If you run the client directly, it will not connect to the server correctly because the
client does not automatically get the `Echo` protocol provided in its
sandbox (in `/svc`). To get this to work, a launcher tool is provided
that launches the server, creates a new [`Environment`][environment] for
the client that provides the server's protocol, then launches the client in it.

1. Configure your GN build:

    ```
    fx set core.x64 --with //examples/fidl/rust/server --with //examples/fidl/rust/client --with //examples/fidl/test:echo-launcher
    ```

2. Build the Fuchsia image:

   ```
   fx build
   ```

3. Run the launcher by passing it the client URL, the server URL, and
   the protocol that the server provides to the client:

    ```
    fx shell run fuchsia-pkg://fuchsia.com/echo-launcher#meta/launcher.cmx fuchsia-pkg://fuchsia.com/echo-rust-client#meta/echo-client.cmx fuchsia-pkg://fuchsia.com/echo-rust-server#meta/echo-server.cmx fuchsia.examples.Echo
    ```

You should see the print output in the QEMU console (or using `fx log`).

```
[105541.570] 489493:489495> Listening for incoming connections...
[105541.573] 489493:489495> Received EchoString request for string "hello"
[105541.574] 489493:489495> Response sent successfully
[105541.574] 489272:489274> response: "hello"
[105541.575] 489493:489495> Received SendString request for string "hi"
[105541.575] 489493:489495> Event sent successfully
[105541.575] 489272:489274> Received OnString event for string "hi"
```

<!-- xrefs -->
[bindings-ref]: /docs/reference/fidl/bindings/rust-bindings.md
[events]: /docs/reference/fidl/bindings/rust-bindings.md#protocol-events-client
[rustdoc]: https://fuchsia-docs.firebaseapp.com/rust/
[server-tut]: /docs/development/languages/fidl/tutorials/rust/basics/server.md
[sync-client]: /docs/development/languages/fidl/tutorials/rust/basics/sync-client.md
[overview]: /docs/development/languages/fidl/tutorials/overview.md
[environment]: /docs/concepts/components/v2/environments.md
