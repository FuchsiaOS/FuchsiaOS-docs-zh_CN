# Implement a Dart FIDL server

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

   The `dart_app` template defines multiple parts:

     * A Dart binary based on the specified sources and dependencies.
     * A component that is set up to simply run the binary, which is described using
       the specified manifest file. `path` refers to the location of the file in the
       tree, and `dest` refers to the target location of the manifest within the
       component.
     * A package containing the component. Packages are the unit of software distribution on
       Fuchsia.

    For more details on packages, components, and how to build them, refer to
    [Building components][components].

    The dependencies will used later when implementing the FIDL server, and are not needed yet
    at this step.

1. Add a component manifest in `examples/fidl/dart/server/server.cmx`:

   Note: The binary name in the manifest must match the name of the `dart_app`, which is used
   to define the Dart executable.

   ```cmx
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/server/meta/server.cmx" %}
   ```

### Run the component

<!-- TODO(fxbug.dev/58758) <<../../common/server/qemu.md>> -->

Note: The instructions in this section are geared towards running the component
on QEMU, as this is the simplest way to get started with running Fuchsia, but
it is also possible to pick a different [product configuration][products] and
run on actual hardware if you are familiar with running components on
other product configurations.

1. Add the server to your configuration and build:

   ```
   fx set core.x64 --with //examples/fidl/dart/server && fx build
   ```

1. Ensure `fx serve` is running in a separate tab and connected to an instance of
   Fuchsia (e.g. running in QEMU using `fx qemu`), then run the server:

   Note: The component should be referenced by its
   [URL][glossary.component url], which
   is determined with the `[fuchsia-pkg://][glossary.fuchsia-pkg URL]` scheme. The
   package name in the URL matches the `package_name` field in the `fuchsia_package`
   declaration, and the manifest path in `meta/` matches the target name of the
   `fuchsia_component`.

   ```
   fx shell run fuchsia-pkg://fuchsia.com/echo-dart-server#meta/echo-server.cmx
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

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/server/lib/main.dart" region_tag="main" highlight="4,5,6,7" %}
```

In order to run, a binding needs two things:

* An implementation of a protocol.
* A channel that the binding will listen for messages for that protocol on.

The binding binds itself to a channel and implementation when the server receives a request to
connect to an `Echo` server.

### Register the protocol request handler {#handler}

Then, the code calls the component manager to expose the `Echo` FIDL protocol to other components:

```cpp
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

Then run the server:

```
fx shell run fuchsia-pkg://fuchsia.com/echo-dart-server#meta/echo-server.cmx
```

You should see server hanging and the startup log using `ffx log`.
This is expected because an
[event loop](https://dart.dev/tutorials/language/futures) to handle incoming
requests is running. The next step will be to write a client for the server.

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
[components]: /docs/development/components/build.md
[bindings-iface]: /docs/reference/fidl/bindings/dart-bindings.md#protocols
[events]: /docs/reference/fidl/bindings/dart-bindings.md#events
[service-name]: /docs/reference/fidl/bindings/dart-bindings.md#discoverable
[logging]: /docs/development/languages/dart/logging.md
[concepts]: /docs/concepts/fidl/overview.md
