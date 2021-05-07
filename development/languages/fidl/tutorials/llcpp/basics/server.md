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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="echo" %}
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
rm -r examples/fidl/llcpp/server/*
```

## Create and run a component {#component}

### Create the component

To create a component:

1. Add a `main()` function to `examples/fidl/llcpp/server/main.cc`:

   ```c++
   #include <stdio.h>

   int main(int argc, const char** argv) {
     printf("Hello, world!\n");
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

   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/BUILD.gn" region_tag="rest" %}
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
  the [Building components](/docs/development/components/build.md) page.

1. Add a component manifest in `examples/fidl/llcpp/server/server.cmx`:

   Note: The binary name in the manifest must match the output name of the
   `executable` defined in the previous step.

   ```cmx
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/server.cmx" %}

### Run the component

<!-- TODO(fxbug.dev/58758) <<../../common/server/qemu.md>> -->

Note: The instructions in this section are geared towards running the component
on QEMU, as this is the simplest way to get started with running Fuchsia, but
it is also possible to pick a different [product configuration][products] and
run on actual hardware if you are familiar with running components on
other product configurations.

1. Add the server to your configuration and build:

   ```
   fx set core.x64 --with //examples/fidl/llcpp/server && fx build
   ```

1. Ensure `fx serve` is running in a separate tab and connected to an instance
   of Fuchsia (e.g. running in QEMU using `fx qemu`), then run the server:

   Note: The component should be referenced by its [URL][glossary-url], which
   is determined with the [`fuchsia-pkg://` scheme][glossary-scheme]. The
   package name in the URL matches the `package_name` field in the
   `fuchsia_package` declaration, and the manifest path in `meta/` matches the
   target name of the `fuchsia_component`.

   ```
   fx shell run fuchsia-pkg://fuchsia.com/echo-llcpp-server#meta/echo-server.cmx
   ```

## Implement the server

### Add a dependency on the FIDL library

1. Add `"//examples/fidl/fuchsia.examples:fuchsia.examples_llcpp"` to the `deps` of the `executable`
2. Include the bindings into the main file with `#include <fuchsia/examples/llcpp/fidl.h>`

The full `bin` target declaration should now look like this:

```
executable("bin") {
  output_name = "fidl_echo_llcpp_server"
  sources = [ "main.cc" ]
  deps = [ "//examples/fidl/fuchsia.examples:fuchsia.examples_llcpp" ]
}
```

### Add an implementation for the protocol {#impl}

Add the following to `main.cc`, above the `main()` function:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="impl" %}
```

The implementation contains the following elements:

* The class subclasses the [generated protocol class][bindings-iface] and
  overrides its pure virtual methods corresponding to the protocol methods.
* It contains an optional `ServerBindingRef` in order to be able to send events
  to the client. It gets set later in the class's `Bind()`  function.
* The `Bind` method binds the implementation to a given `request`.
* The method for `EchoString` replies with the request value by using the
  completer.
* The method for `SendString` uses the `binding_` member (if defined) to send
  an `OnString` event containing the request value.

You can verify that the implementation builds by running:

```
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

### Initialize the event loop

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="main" highlight="2,3,4,5,30" %}
```

The event loop is used to asynchronously listen for incoming connections and
requests from the client. This code initializes the loop, and obtains the
dispatcher, which will be used when binding the server implementation to a
channel.

At the end of the main function, the code runs the loop to completion.

### Serve component's service directory

The `svc::Outgoing` class serves the service directory ("/svc") for a given
component. This directory is where the outgoing FIDL protocols are installed so
that they can be provided to other components. The `ServeFromStartupInfo()`
function sets up the service directory with the startup handle. The startup
handle is a handle provided to every component by the system, so that they can
serve capabilities (e.g. FIDL protocols) to other components.

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="main" highlight="7,8,9,10,11,12,13,14,15,16,17" %}
```

### Serve the protocol {#server-handler}

The server then registers the Echo protocol using `ougoing.svc_dir()->AddEntry()`.

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="main" highlight="19,20,21,22,23,24,25,26,27,28,29,30,31" %}
```

The call to `AddEntry` installs a handler for the name of the FIDL protocol
(`fuchsia_examples::Echo::Name`, which is the string
`"fuchsia.examples.Echo"`). The handler will call the lambda function that we
created, and this lambda function will call `server.Bind()` with the `fidl::ServerEnd<fuchsia_examples::Echo>`, which internally wraps a
`zx::channel`, that represents a request from a client.

When a client requests access to `/svc/fuchsia.examples.Echo`, this function
will be called with a channel that represents the request. This channel is bound
to our server via the `Bind()` function, and future requests from this client
will call.

When the handler is called (i.e. when a client has requested to connect to the
`/svc/fuchsia.examples.Echo` protocol), it binds the incoming channel to our
`Echo` implementation, which will start listening for `Echo` requests on that
channel and dispatch them to the `EchoImpl` instance. `EchoImpl`'s call to
`fidl::BindServer` returns a `fidl::ServerBindingRef`, which is then stored so
the instance can be able to send events back to the client.

### Add new dependencies {#deps}

This new code requires the following additional dependencies:

* `"//zircon/system/ulib/async-loop:async-loop-cpp"` and `"//zircon/system/ulib/async-loop:async-loop-default"`, which contain the async loop code.
* `"//sdk/lib/fdio"` and `"//zircon/system/ulib/svc"`: These are libraries used
  to interact with the components environment (e.g. for serving protocols).
* `"//zircon/public/lib/fidl"`: The LLCPP runtime, which contains utility code for
  using the FIDL bindings, such as the `BindServer` function.

The full `bin` target declaration should now look like this:

```
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/BUILD.gn" region_tag="bin" %}
```

Import the dependencies by including them at the top of `examples/fidl/llcpp/server/main.cc`:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="includes" %}
```

## Run the server

Rebuild:

```
fx build
```

Then run the server:

```
fx shell run fuchsia-pkg://fuchsia.com/echo-llcpp-server#meta/echo-server.cmx
```

You should see the `std::cout` output from the `main()` function followed by the
server hanging. This is expected. Instead of exiting right away, the server
keeps waiting for incoming requests. The next step will be to write a client for
the server.

<!-- xrefs -->
[fidl-intro]: /docs/development/languages/fidl/tutorials/llcpp/basics/using-fidl.md
[building-components]: /docs/development/components/build.md
[products]: /docs/concepts/build_system/boards_and_products.md
[getting-started]: /docs/getting_started.md
[glossary-url]: /docs/glossary.md#component-url
[glossary-scheme]: /docs/glossary.md#fuchsia-pkg-url
[declaring-fidl]: /docs/development/languages/fidl/tutorials/fidl.md
[depending-fidl]: /docs/development/languages/fidl/tutorials/llcpp/basics/using-fidl.md
[component-manager]: /docs/concepts/components/v2/component_manager.md
[protocol-open]: /docs/concepts/components/v2/capabilities/life_of_a_protocol_open.md#binding_to_a_component_and_sending_a_protocol_channel
[bindings-iface]: /docs/reference/fidl/bindings/llcpp-bindings.md#protocols
[compiling-fidl]: /docs/development/languages/fidl/tutorials/fidl.md
[async-loop]: /zircon/system/ulib/async-loop/include/lib/async-loop/cpp/loop.h
[overview]: /docs/development/languages/fidl/tutorials/overview.md
[concepts]: /docs/concepts/fidl/overview.md
