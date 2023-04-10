# Implement a FIDL server in Dart

## Prerequisites

This tutorial builds on the [Dart FIDL packages][fidl-packages] tutorial. For the
full set of FIDL tutorials, refer to the [overview][overview].

## Overview

<!-- TODO(fxbug.dev/58758) <<../../common/server/overview.md>> -->

This tutorial shows you how to implement a FIDL protocol
(`fuchsia.examples.Echo`) and run it on Fuchsia. This protocol has one method
of each kind: a fire and forget method, a two-way method, and an event:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="echo" %}
```

For more on FIDL methods and messaging models, refer to the [FIDL concepts][concepts] page.

This document covers how to complete the following tasks:

* Implement a FIDL protocol.
* Build and run a package on Fuchsia.
* Serve a FIDL protocol.

The tutorial starts by creating a component that is served to a Fuchsia device
and run. Then, it gradually adds functionality to get the server up and running.

If you want to write the code yourself, delete the following directories:

```
rm -r examples/fidl/dart/server/*
```

## Create and run a component {#component}

### Create the component

To create a component:

1. Add a `main()` function to `examples/fidl/dart/server/lib/main.dart`:

   ```dart
   void main(List<String> args) {
     print("Hello, world!");
   }
   ```

1. Declare a target for the server in `examples/fidl/dart/server/BUILD.gn`:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/server/BUILD.gn" %}
   ```

   The `dart_library` template specifies sources and dependencies for a Dart package. Since this example will build an executable component, the Dart package includes a `main.dart` source and `main()` method.

   The `dart_component` template depends on the `dart_library` target and a
   component `manifest` file.

   Finally, the `fuchsia_package` declares a package containing the component.
   Packages are the unit of software distribution on Fuchsia.

   For more details on packages, components, and how to build them, refer to
   the [Building components][building-components] page.

1. Add a component manifest in `examples/fidl/dart/server/meta/server.cml`:

   ```json5
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/server/meta/server.cml" region_tag="example_snippet" %}
   ```

   <!-- TODO(fxbug.dev/58758) <<../../common/server/qemu.md>> -->

1. Add the server to your build configuration:

   ```posix-terminal
   fx set core.x64 --with //examples/fidl/dart/server:echo-dart-server --with-base //src/dart \
     --args='core_realm_shards += [ "//src/dart:dart_runner_core_shard" ]'
   ```

   Note: This build configuration assumes your device target is the emulator.
   To run the example on a physical device, select the appropriate
   [product configuration][products] for your hardware.

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

## Implement the server

### Add dependencies

Import the required dependencies in `lib/main.dart`:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/server/lib/main.dart" region_tag="imports" %}
```

### Implement an Echo server

Add the following to `lib/main.dart`, above the `main()` function:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/server/lib/main.dart" region_tag="impl" %}
```

The implementation consists of the following elements:

* The class inherits from the [generated protocol class][bindings-iface] and overrides its abstract
  methods to define the protocol method handlers.
  * The method for `EchoString` replies with the request value by returning it.
  * The method for `SendString` returns `void` since this method does not have a response. Instead,
    the implementation sends an `OnString` event containing the request data.
* The class contains an `_onStringStreamController`, which is used to implement the
  [abstract `onString` method][events].
  The FIDL runtime will subscribe to the stream returned by this method, sending
  incoming events to the client. The server can therefore send an `OnString` event by sending an
  event on the stream.

You can verify that the implementation is correct by running:

```
fx build
```

## Serve the protocol {#main}

To run a component that implements a FIDL protocol, you must make a request to
the [component manager][component-manager] to expose that FIDL protocol to other
components. The component manager then routes any requests for the echo protocol
to our server.

To fulfill these requests, the component manager requires the name of the protocol
as well as a handler that it should call when it has any incoming requests to
connect to a protocol matching the specified name.

The handler passed to it is a function that takes a channel (whose remote
end is owned by the client), and binds it to an `EchoBinding`.

The `EchoBinding` is a class that takes a FIDL protocol implementation and a channel,
and then listens on the channel for incoming requests. It will then decode
the requests, dispatch them to the correct method on our server class, and
write any response back to the client.

This complete process is described in further detail in the
[Life of a protocol open][protocol-open].

### Initialize the binding

First, the code initializes the `EchoBinding` as mentioned above:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/server/lib/main.dart" region_tag="main" highlight="4,5,6,7" %}
```

In order to run, a binding needs two things:

* An implementation of a protocol.
* A channel that the binding will listen for messages for that protocol on.

The binding binds itself to a channel and implementation when the server receives a request to
connect to an `Echo` server.

### Register the protocol request handler {#handler}

Then, the code calls the component manager to expose the `Echo` FIDL protocol to other components:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/server/lib/main.dart" region_tag="main" highlight="10,11,12,13,14,15,16,17" %}
```

It does so using the `fuchsia_services` package, which provides an API to access the startup context
of the component. Specifically, each component receives a `ComponentContext` that the component can
use to both access capabilties *from* other components and expose capabilities *to* other components.
The call to `sys.ComponentContext.create()` obtains an instance of the component's context, and the
`outgoing` property is used to expose the `Echo` protocol and later `serveFromStartupInfo()`.

In order to add a service, the outgoing context needs to know:

* The name of the service, so that clients are able to locate it using the correct path.
* What to do with an incoming request to connect to the service.
  * A connection request here is defined as a `fidl.InterfaceRequest<Echo>`.
    This is a type-safe wrapper around a channel.
    * `InterfaceRequest` indicates that this is the server end of a channel
      (i.e. a client is connected to the remote end of the channel)
    * The template parameter `Echo` means that the client expects that a server
      implementing the `Echo` protocol binds itself to this channel. The client
      analog of this (i.e. the type that is being used on the client side to
      represent the other end of this channel) is a
      `fidl.InterfaceHandle<Echo>`.

The name of the service is specified as the associated [service name][service-name], and the
handler is just a function that takes channel sent from the client and binds it to the
`EchoBinding`.

## Logging

The server uses the [`fuchsia_logger`][logging] to log information. The logger needs to be
initialized first using `setupLogger()`, then information can be logged using `log.info` or
other methods corresponding to the various log levels.

## Run the server

Build:

```
fx build
```

Then run the server component:

```posix-terminal
ffx component run /core/ffx-laboratory:echo_server fuchsia-pkg://fuchsia.com/echo-dart-server#meta/echo_server.cm
```

Note: Components are resolved using their [component URL][glossary.component-url],
which is determined with the [`fuchsia-pkg://`][glossary.fuchsia-pkg-url] scheme.

You should see output similar to the following in the device logs (`ffx log`):

```none {:.devsite-disable-click-to-copy}
[echo-server, main.dart(64)] INFO: Running Echo server
```

The server is now running and waiting for incoming requests.
The next step will be to write a client that sends `Echo` protocol requests.
For now, you can simply terminate the server component:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo_server
```

Note: Component instances are referenced by their
[component moniker][glossary.moniker], which is determined by their location in
the [component instance tree][glossary.component-instance-tree]

<!-- xrefs -->
[glossary-url]: /docs/glossary/README.md#component-url
[glossary-scheme]: /docs/glossary/README.md#fuchsia-pkg-url
[fidl-packages]: /docs/development/languages/fidl/tutorials/dart/basics/using-fidl.md
[building-components]: /docs/development/components/build.md
[products]: /docs/development/build/build_system/boards_and_products.md
[getting-started]: /docs/get-started/index.md
[declaring-fidl]: /docs/development/languages/fidl/tutorials/fidl.md
[depending-fidl]: /docs/development/languages/fidl/tutorials/dart/basics/using-fidl.md
[component-manager]: /docs/concepts/components/v2/component_manager.md
[protocol-open]: /docs/concepts/components/v2/capabilities/life_of_a_protocol_open.md#binding_to_a_component_and_sending_a_protocol_channel
[compiling-fidl]: /docs/development/languages/fidl/tutorials/fidl.md
[overview]: /docs/development/languages/fidl/tutorials/overview.md
[building-components]: /docs/development/components/build.md
[bindings-iface]: /docs/reference/fidl/bindings/dart-bindings.md#protocols
[events]: /docs/reference/fidl/bindings/dart-bindings.md#events
[service-name]: /docs/reference/fidl/bindings/dart-bindings.md#discoverable
[logging]: /docs/development/languages/dart/logging.md
[concepts]: /docs/concepts/fidl/overview.md
