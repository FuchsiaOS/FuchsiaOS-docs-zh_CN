# Implement a Dart FIDL client

<!-- TODO(fxbug.dev/58758) <<../../common/client/overview.md>> -->

## Prerequisites

This tutorial builds on the [FIDL server][server-tut] tutorial. For the
full set of FIDL tutorials, refer to the [overview][overview].

## Overview

This tutorial implements a client for a FIDL protocol and runs it
against the server created in the [previous tutorial][server-tut]. The client in this
tutorial is asynchronous. There is an [alternate tutorial][sync-client] for
synchronous clients.

If you want to write the code yourself, delete the following directories:

```
rm -r examples/fidl/dart/client/*
```

## Create a stub component

1. Set up a hello world `dart_app` in `examples/fidl/dart/client`, with a name of `echo-dart-client`.

1. Once you have created your component, ensure that the following works:

   ```
   fx set core.x64 --with //examples/fidl/dart/client
   ```

   Note: If necessary, refer back to the [previous tutorial][server-tut].

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
   fx shell run fuchsia-pkg://fuchsia.com/echo-dart-client#meta/echo-dart-client.cmx
   ```

## Edit GN dependencies

1. Add the following dependencies:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/client/BUILD.gn" region_tag="deps" %}
   ```

1. Then, import them in `lib/main.dart`:

   ```dart
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/client/lib/main.dart" region_tag="imports" %}
   ```

These dependencies are explained in the [server tutorial][server-tut].

## Edit component manifest

1. Include the `Echo` protocol in the client component's sandbox by
   editing the component manifest in `client.cmx`.

   ```cmx
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/client/meta/client.cmx" %}
   ```

## Connect to the server {#main}

The steps in this section explain how to add code to the `main()` function
that connects the client to the server and makes requests to it.

### Bind a client object

The FIDL bindings generate a [class][proxy] for each protocol that can be used to make requests
to a server, called a proxy class. To connect to the server, the client needs to initialize
a proxy class and then bind it to the server:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/client/lib/main.dart" region_tag="main" highlight="4,5,6,7" %}
```

Similar to the server code, the client uses `ComponentContext` to access the component's
context. The difference is that the `incoming` property is used instead of the `outgoing`
property, since the client is connecting to a protocol rather than offering one. Additionally,
since no outgoing services are added, it uses the `ComponentContext.createAndServe()`
convenience method.

The `connectToService` call does a number of things under the hood:

* First, it creates a channel and binds one end to the `EchoProxy`. `EchoProxy`, similarly to `EchoBinding`,
  binds to a channel and listens for incoming messages (and sends messages back) on the channel.
  The channel end bound to the `EchoProxy` is a `fidl.InterfaceHandle<Echo>`, whereas the other end
  of the channel is a `fidl.InterfaceRequest<Echo>`.
* It then makes a request to the component manager to connect to the `Echo` protocol. Specifically,
  it requests the other end of the channel (from the previous step) to be connected to the protocol
  located at the [service name][service-name] of the `Echo` protocol.

In the background, this request triggers the follow sequence of events:

* The component framework routes this request to the server, where the requested service name
  [matches][server-handler] the service offered by the server.
* The connection request handler defined in the server code is invoked on the channel end (the
  `fidl.InterfaceRequest<Echo>`) that was provided by the client.
* The handler code binds the server implementation to the channel, and starts handling any incoming
  messages on the channel. If the client started making requests before this point, these requests
  are buffered until the server is bound and starts reading from the channel. This is a process
  called request [pipelining][pipeline], which is covered in more depth in a
  [separate tutorial][pipeline-tut].

### Send requests to the server

The code makes two requests to the server:

* An `EchoString` request
* A `SendString` request

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/client/lib/main.dart" region_tag="main" highlight="9,10,11,12,13,14" %}
```

The call to `EchoString` returns a future, which resolves to the response
returned by the server. The returned future will resolve to an error if there is
either an error sending the request or receiving the response (e.g. when
decoding the message, or if an epitaph was received).

The call to `SendString` returns a `Future<void>` since it is a fire and forget
method.

The [bindings reference][bindings-ref] describes how these proxy methods are generated.

### Handle incoming events

The code then waits for a single `OnString` event from the server:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/client/lib/main.dart" region_tag="main" highlight="15,16,17" %}
```

This is done by [taking the event stream][events] from the client object, then waiting
for a single event from it.

## Run the client

If you run the client directly, it will not connect to the server correctly because the
client does not automatically get the `Echo` protocol provided in its
sandbox (in `/svc`). To get this to work, a launcher tool is provided
that launches the server, creates a new [`Environment`][environment] for
the client that provides the server's protocol, then launches the client in it.

1. Configure your GN build as follows:

    ```
    fx set core.x64 --with //examples/fidl/dart/server --with //examples/fidl/dart/client --with //examples/fidl/dart/launcher_bin
    ```

2. Build the Fuchsia image:

   ```
   fx build
   ```

3. Run the launcher by passing it the client URL, the server URL, and
   the protocol that the server provides to the client:

    ```
    fx shell run fuchsia-pkg://fuchsia.com/example-launcher-dart#meta/example-launcher-dart.cmx fuchsia-pkg://fuchsia.com/echo-dart-client#meta/echo-dart-client.cmx fuchsia-pkg://fuchsia.com/echo-dart-server#meta/echo-dart-server.cmx fuchsia.examples.Echo
    ```

You should see output similar to the following in the QEMU console
(or using `ffx log`):

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
[bindings-ref]: /reference/fidl/bindings/dart-bindings.md
[proxy]: /reference/fidl/bindings/dart-bindings.md#proxy
[events]: /reference/fidl/bindings/dart-bindings.md#protocol-events-client
[server-tut]: /development/languages/fidl/tutorials/dart/basics/server.md
[server-handler]: /development/languages/fidl/tutorials/dart/basics/server.md#handler
[overview]: /development/languages/fidl/tutorials/overview.md
[environment]: /concepts/components/v2/environments.md
[service-name]: /reference/fidl/bindings/dart-bindings.md#discoverable
[pipeline]: /development/api/fidl.md#request-pipelining
[pipeline-tut]: /development/languages/fidl/tutorials/hlcpp/topics/request-pipelining.md
