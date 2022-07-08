# Implement an LLCPP FIDL server

## Prerequisites

This tutorial builds on the [Compiling FIDL][fidl-intro] tutorial. For the
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

```posix-terminal
rm -r examples/fidl/llcpp/server/*
```

## Create the component {#component}

To create a component:

1. Add a `main()` function to `examples/fidl/llcpp/server/main.cc`:

   ```cpp
   int main(int argc, const char** argv) {
     return 0;
   }
   ```

1. Declare a target for the server in `examples/fidl/llcpp/server/BUILD.gn`:

   ```gn
   import("//build/components.gni")

   # Declare an executable for the server. This produces a binary with the
   # specified output name that can run on Fuchsia.
   executable("bin") {
     output_name = "fidl_echo_llcpp_server"
     sources = [ "main.cc" ]
   }

   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/BUILD.gn" region_tag="rest" %}
   ```

   <!-- TODO(fxbug.dev/58758) <<../../common/server/packages.md>> -->

   To get the server component up and running, there are three targets that are
   defined:

   * The raw executable file for the server that is built to run on Fuchsia.
   * A component that is set up to simply run the server executable,
     which is described using the component's manifest file.
   * The component is then put into a package, which is the unit of software
     distribution on Fuchsia. In this case, the package just contains a
     single component.

   For more details on packages, components, and how to build them, refer to
   the [Building components][building-components] page.

1. Add a component manifest in `examples/fidl/llcpp/server/meta/server.cml`:

   Note: The binary name in the manifest must match the output name of the `executable`
   defined in the previous step.

   ```json5
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/meta/server.cml" region_tag="example_snippet" %}
   ```

   <!-- TODO(fxbug.dev/58758) <<../../common/server/qemu.md>> -->

1. Add the server to your build configuration:

   ```posix-terminal
   fx set core.qemu-x64 --with //examples/fidl/llcpp/server:echo-llcpp-server
   ```

   Note: This build configuration assumes your device target is the emulator.
   To run the example on a physical device, select the appropriate
   [product configuration][products] for your hardware.

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

## Implement the server

### Add a dependency on the FIDL library

1.  Add the `fuchsia.examples` FIDL library target as a dependency of your
    `executable` in `examples/fidl/llcpp/server/BUILD.gn`:

    ```gn
    executable("bin") {
      output_name = "fidl_echo_llcpp_server"
      sources = [ "main.cc" ]
      {{ '<strong>' }}deps = [ "//examples/fidl/fuchsia.examples:fuchsia.examples_llcpp" ]{{ '</strong>' }}
    }
    ```

1.  Import the LLCPP bindings at the top of `examples/fidl/llcpp/server/main.cc`:

    ```cpp
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="fidl_includes" %}
    ```

### Add an implementation for the protocol {#impl}

Add the following to `main.cc`, above the `main()` function:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="impl" %}
```

The implementation contains the following elements:

* The class subclasses the [generated protocol class][bindings-iface] and
  overrides its pure virtual methods corresponding to the protocol methods.
* It contains a `ServerBindingRef` in order to be able to send events to the
  client.
* The constructor method binds the implementation to a given `request`.
* The method for `EchoString` replies synchronously with the request value by using the
  completer (for asynchronous replies, see
  [responding to requests asynchronously in
  LLCPP](/development/languages/fidl/tutorials/llcpp/topics/async-completer.md)).
* The method for `SendString` uses the `binding_` member (if defined) to send
  an `OnString` event containing the request value.

You can verify that the implementation builds by running:

```posix-terminal
fx build
```

## Serve the protocol {#main}

When running a component that implements a FIDL protocol, you must make a
request to the [component manager][component-manager] to expose that FIDL
protocol to other components. The component manager then routes any requests for
the echo protocol to our server.

To fulfill these requests, the component manager requires the name of the
protocol as well as a handler that it should call when it has any incoming
requests to connect to a protocol matching the specified name.

The handler passed to it is a function that takes a channel (whose remote
end is owned by the client), and binds it to our server implementation.
The resulting `fidl::ServerBindingRef` is reference to a server binding
that takes a FIDL protocol implementation and a channel,
and then listens on the channel for incoming requests. The binding then decodes
the requests, dispatches them to the correct method on our server class, and
writes any response back to the client. Our main method will keep listening
for incoming requests on an [async loop][async-loop].

This complete process is described in further detail in the
[Life of a protocol open][protocol-open].

### Add new dependencies {#deps}

This new code requires the following additional dependencies:

* `"//zircon/system/ulib/async-loop:async-loop-cpp"`: This library contains the
  asynchronous event loop code.
* `"//sdk/lib/sys/component/cpp"`: This library is used to publish
  capabilities, e.g. protocols, to the component's outgoing directory.
* `"//sdk/lib/syslog/cpp"`: This library is used to log messages.

1.  Add the library targets as dependencies of your `executable` in
    `examples/fidl/llcpp/server/BUILD.gn`:

    ```gn
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/BUILD.gn" region_tag="bin" highlight="6,7,8" %}
    ```

1.  Import these dependencies at the top of `examples/fidl/llcpp/server/main.cc`:

    ```cpp
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="includes" %}
    ```

### Initialize the event loop

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="main" highlight="2,3,4,5,30" %}
```

The event loop is used to asynchronously listen for incoming connections and
requests from the client. This code initializes the loop, and obtains the
dispatcher, which will be used when binding the server implementation to a
channel.

At the end of the main function, the code runs the loop to completion.

### Serve component's outgoing directory

The `component::OutgoingDirectory` class serves the outgoing directory for a
given component. This directory is where the outgoing FIDL protocols are
installed so that they can be provided to other components. The
`ServeFromStartupInfo()` function sets up the outgoing directory with the
startup handle. The startup handle is a handle provided to every component by
the system, so that they can serve capabilities (e.g. FIDL protocols) to other
components.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="main" highlight="7,8,9,10,11,12,13,14" %}
```

### Serve the protocol {#server-handler}

The server then registers the Echo protocol using `outgoing.AddProtocol`.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="main" highlight="16,17,18,19,20,21,22,23,24,25,26,27" %}
```

The call to `AddProtocol` installs a handler for the name of the FIDL protocol
(`fidl::DiscoverableProtocolName<fuchsia_examples::Echo>`, which is the string
`"fuchsia.examples.Echo"`). The handler will call the lambda function that we
created, and this lambda function will construct an `EchoImpl` with the
`fidl::ServerEnd<fuchsia_examples::Echo>`, which internally wraps a
`zx::channel`, that represents a request from a client. The `EchoImpl` stays
alive the connection is torn down, at which point it deletes itself.

When the handler is called (i.e. when a client has requested to connect to
`/svc/fuchsia.examples.Echo`), it binds the incoming channel to our
`Echo` implementation, which will start listening for `Echo` requests on that
channel and dispatch them to the `EchoImpl` instance. `EchoImpl`'s constructor
populates a `fidl::ServerBindingRef` which is used to send events back to the
client.

## Test the server

Rebuild:

```posix-terminal
fx build
```

Then run the server component:

```posix-terminal
ffx component run fuchsia-pkg://fuchsia.com/echo-llcpp-server#meta/echo_server.cm
```

Note: Components are resolved using their [component URL][glossary.component-url],
which is determined with the [`fuchsia-pkg://`][glossary.fuchsia-pkg-url] scheme.

You should see output similar to the following in the device logs (`ffx log`):

```none {:.devsite-disable-click-to-copy}
[ffx-laboratory:echo_server][][I] Running echo server
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
[glossary.component-instance-tree]: /glossary/README.md#component-instance-tree
[glossary.component-url]: /glossary/README.md#component-url
[glossary.fuchsia-pkg-url]: /glossary/README.md#fuchsia-pkg-url
[glossary.moniker]: /glossary/README.md#moniker
[fidl-intro]: /development/languages/fidl/tutorials/llcpp/basics/using-fidl.md
[building-components]: /development/components/build.md
[products]: /development/build/build_system/boards_and_products.md
[declaring-fidl]: /development/languages/fidl/tutorials/fidl.md
[depending-fidl]: /development/languages/fidl/tutorials/llcpp/basics/using-fidl.md
[component-manager]: /concepts/components/v2/component_manager.md
[protocol-open]: /concepts/components/v2/capabilities/life_of_a_protocol_open.md#binding_to_a_component_and_sending_a_protocol_channel
[bindings-iface]: /reference/fidl/bindings/llcpp-bindings.md#protocols
[compiling-fidl]: /development/languages/fidl/tutorials/fidl.md
[async-loop]: /zircon/system/ulib/async-loop/include/lib/async-loop/cpp/loop.h
[overview]: /development/languages/fidl/tutorials/overview.md
[concepts]: /concepts/fidl/overview.md
