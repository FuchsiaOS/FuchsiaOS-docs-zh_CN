# Implement a FIDL client in Dart

<!-- TODO(fxbug.dev/58758) <<../../common/client/overview.md>> -->

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

```posix-terminal
rm -r examples/fidl/dart/client/*
```

## Create the component

Create a new component project at `examples/fidl/dart/client`:

1. Add a `main()` function to `examples/fidl/dart/client/lib/main.dart`:

   ```dart
   import 'dart:async';

   Future<void> main(List<String> args) async {
     print('hello world!');
   }
   ```

1. Declare a target for the client in `examples/fidl/dart/client/BUILD.gn`:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/client/BUILD.gn" region_tag="imports" %}

   # Declare a `dart_library` for the client executable.
   dart_library("lib") {
     package_name = "echo_client"
     null_safe = true

     sources = [ "main.dart" ]
   }

   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/client/BUILD.gn" region_tag="rest" %}
   ```

1. Add a component manifest in `examples/fidl/dart/client/meta/client.cml`:

   ```json5
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/client/meta/client.cml" region_tag="example_snippet" %}
   ```

1. Once you have created your component, ensure that you can add it to the
   build configuration:

   ```posix-terminal
   fx set core.x64 --with //examples/fidl/dart/client:echo-client
   ```

1. Build the Fuchsia image:

   ```
   fx build
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

In order for the client and server to communicate using the `Echo` protocol,
component framework must route the `fuchsia.examples.Echo` capability from the
server to the client. For this tutorial, a [realm][glossary.realm] component is
provided to declare the appropriate capabilities and routes.

Note: You can explore the full source for the realm component at
[`//examples/fidl/echo-realm`](/examples/fidl/echo-realm)

1. Configure your build to include the provided package that includes the
   echo realm, server, and client:

   ```posix-terminal
   fx set core.x64 --with examples/fidl/dart:echo-launcher-dart --with-base //src/dart \
     --args='core_realm_shards += [ "//src/dart:dart_runner_core_shard" ]'
   ```

   NOTE: The flag `--with-base //src/dart` adds the required dart runner to the
   base packages; and the `core_realm_shards` argument updates the
   `laboratory-env` component environment (the environment provided to the
   `ffx-laboratory` realm, used in `ffx component start`) to include the
   required dart runner.

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

    ```posix-terminal
    ffx component run /core/ffx-laboratory:echo_realm fuchsia-pkg://fuchsia.com/echo-dart-client#meta/echo_realm.cm
    ```

1. Start the `echo_client` instance:

    ```posix-terminal
    ffx component start /core/ffx-laboratory:echo_realm/echo_client
    ```

The server component starts when the client attempts to connect to the `Echo`
protocol. You should see output similar to the following in the device logs
(`ffx log`):

```none {:.devsite-disable-click-to-copy}
[echo-server, main.dart(64)] INFO: Running Echo server
[echo-server, main.dart(33)] INFO: Received EchoString request: hello
[echo-server, main.dart(41)] INFO: Received SendString request: hi
[echo-client, main.dart(27)] INFO: Got response: hello
[echo-client, main.dart(33)] INFO: Got event: hi
```

<!-- xrefs -->
[bindings-ref]: /reference/fidl/bindings/dart-bindings.md
[proxy]: /reference/fidl/bindings/dart-bindings.md#proxy
[events]: /reference/fidl/bindings/dart-bindings.md#protocol-events-client
[server-tut]: /development/languages/fidl/tutorials/dart/basics/server.md
[server-handler]: /development/languages/fidl/tutorials/dart/basics/server.md#handler
[overview]: /development/languages/fidl/tutorials/overview.md
[service-name]: /reference/fidl/bindings/dart-bindings.md#discoverable
[pipeline]: /development/api/fidl.md#request-pipelining
[pipeline-tut]: /development/languages/fidl/tutorials/hlcpp/topics/request-pipelining.md
[sync-client]: /development/languages/fidl/tutorials/cpp/basics/sync-client.md