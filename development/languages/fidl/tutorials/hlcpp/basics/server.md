# Implement an HLCPP FIDL server

## Prerequisites

This tutorial builds on the [Compiling FIDL][fidl-intro] tutorial. For the
full set of FIDL tutorials, refer to the [overview][overview].

## Overview

This tutorial shows you how to implement a FIDL protocol
(`fuchsia.examples.Echo`) and run it on Fuchsia. This protocol has one method
of each kind: a fire and forget method, a two-way method, and an event:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="echo" %}
```

This document covers how to complete the following tasks:

* Implement a FIDL protocol.
* Build and run a package on Fuchsia.
* Serve a FIDL protocol.

The tutorial starts by creating a component that is served to a Fuchsia device
and run. Then, it gradually adds functionality to get the server up and running.

If you want to write the code yourself, delete the following directories:

```posix-terminal
rm -r examples/fidl/hlcpp/server/*
```

## Create the component {#component}

To create a component:

1. Add a `main()` function to `examples/fidl/hlcpp/server/main.cc`:

   ```cpp
   #include <stdio.h>

   int main(int argc, const char** argv) {
     printf("Hello, world!\n");
     return 0;
   }
   ```

1. Declare a target for the server in `examples/fidl/hlcpp/server/BUILD.gn`:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/BUILD.gn" region_tag="imports" %}

   # Declare an executable for the server. This produces a binary with the
   # specified output name that can run on Fuchsia.
   executable("bin") {
     output_name = "fidl_echo_hlcpp_server"
     sources = [ "main.cc" ]
   }

   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/BUILD.gn" region_tag="rest" %}
   ```

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

1. Add a component manifest in `examples/fidl/hlcpp/server/meta/server.cml`:

   Note: The binary name in the manifest must match the output name of the `executable`
   defined in the previous step.

   ```json5
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/meta/server.cml" region_tag="example_snippet" %}
   ```

   <!-- TODO(fxbug.dev/58758) <<../../common/server/qemu.md>> -->

1. Add the server to your build configuration:

   ```posix-terminal
   fx set core.qemu-x64 --with //examples/fidl/hlcpp/server:echo-hlcpp-server
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
    `executable` in `examples/fidl/hlcpp/server/BUILD.gn`:

    ```gn
    executable("bin") {
      output_name = "fidl_echo_hlcpp_server"
      sources = [ "main.cc" ]
      {{ '<strong>' }}deps = [ "//examples/fidl/fuchsia.examples" ]{{ '</strong>' }}
    }
    ```

1.  Import the HLCPP bindings at the top of `examples/fidl/hlcpp/server/main.cc`:

    ```cpp
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/main.cc" region_tag="fidl_includes" %}
    ```

### Add an implementation for the protocol {#impl}

Add the following to `main.cc`, above the `main()` function:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/main.cc" region_tag="server" %}
```

The implementation contains the following elements:

* The class subclasses the [generated protocol class][bindings-iface] and
  overrides its pure virtual methods corresponding to the protocol methods.
* The method for `EchoString` replies with the request value by calling the
  callback on it.
* The method for `SendString` does not take a callback since this method does
  not have a response. Instead, the implementation sends an `OnString` event
  using an `Echo_EventSender`.
* The class contains a pointer to an `Echo_EventSender`. This will be set
  later in the `main()` function.

You can verify that the implementation is correct by running:

```posix-terminal
fx build
```

## Serve the protocol {#main}

To run a component that implements a FIDL protocol, make a request to the
[component manager][component-manager] to expose that FIDL protocol to other
components. The component manager then routes any requests for the echo protocol
to our server.

To fulfill these requests, the component manager requires the name of the protocol
as well as a handler that it should call when it has any incoming requests to
connect to a protocol matching the specified name.

The handler passed to it is a function that takes a channel (whose remote
end is owned by the client), and binds it to a `fidl::Binding` that has been
initialized using the server implementation. The `fidl::Binding` is a class
from the FIDL runtime that takes a FIDL protocol implementation and a channel,
and then listens on the channel for incoming requests. It will then decode
the requests, dispatch them to the correct method on our server class, and
write any response back to the client. Our main method will keep listening
for incoming requests on an [async loop][async-loop].

This complete process is described in further detail in the
[Life of a protocol open][protocol-open].

### Add new dependencies {#deps}

This new code requires the following additional dependencies:

* `"//zircon/system/ulib/async-loop:async-loop-cpp"` and
  `"//zircon/system/ulib/async-loop:async-loop-default"`: These libraries contain
  the async loop code.
* `"//sdk/lib/sys/cpp"`: The component framework C++ runtime, which contains
  utility code for interacting with the component's environment.

1.  Add the library targets as dependencies of your `executable` in
    `examples/fidl/hlcpp/server/BUILD.gn`:

    ```gn
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/BUILD.gn" region_tag="bin" highlight="6,7,8" %}
    ```

1.  Import these dependencies at the top of `examples/fidl/hlcpp/server/main.cc`:

    ```cpp
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/main.cc" region_tag="includes" %}
    ```

### Initialize the event loop

The first aspect is the use of an async loop:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/main.cc" region_tag="main" highlight="2,15" %}
```

The code first initializes the loop and registers it as the default dispatcher
for the current thread. This comes first, as the async code in the `main()`
function will register itself with the default dispatcher, which is a static
thread local variable (which is why it does not need to be passed explicitly in
the rest of the code). At the end of the main function, the code runs the async loop.

### Initialize the binding

Then, the code initializes the `fidl::Binding` as mentioned above:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/main.cc" region_tag="main" highlight="4,5,6" %}
```

In order to run, a binding needs two things:

* An implementation of a protocol.
* A channel that the binding will listen for messages for that protocol on.
  The binding is first initialized using the echo implementation, and will be
  bound to a channel later.

The code also sets the event sender that is used to send events to the client.
The event sender is obtained using the `events()` method on the `Binding`, and then passed to
the `EchoImpl` class.

### Define a protocol request handler {#handler}

Next, the code defines a handler for incoming requests from a client:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/main.cc" region_tag="main" highlight="7,8,9,10" %}
```

* An "incoming request" is not a request for a specific method of `Echo`
  protocol, but rather a general request from a client to connect to an
  implementation of the `Echo` protocol.
* The request is defined as a `fidl::InterfaceRequest<Echo>`. This is a
  type-safe wrapper around a channel that indicates two things:
  * `InterfaceRequest` indicates that this is the server end of a channel (i.e.
    a client is connected to the remote end of the channel)
  * The template parameter `Echo` means that the client expects that a server
    implementing the `Echo` protocol binds itself to this channel. The client
    analog of this (i.e. the type that is being used on the client side to
    represent the other end of this channel) is a `fidl::InterfaceHandle<Echo>`.
* The handler simply takes the channel sent from the client, and binds it to the
  `Echo` binding.
* Once this happens, the `Binding` starts handling messages on the channel
  according to the `Echo` protocol. This is an example of [protocol request
  pipelining][pipeline], which is explored in a [later tutorial][pipeline-tut].

### Register the protocol request handler

Finally, the code registers the handler with the component manager:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/main.cc" region_tag="main" highlight="11,12" %}
```

The first line initializes and serves the outgoing directory, which contains
protocols that this component exposes to other components, and the second line
adds the handler to the outgoing directory.

An implicit second parameter besides the handler is the name that this handler
should be registered to. By default, this parameter is the name of the protocol
being passed in, which is generated because of the presence [`[Discoverable]`
attribute][discoverable] on the `Echo` protocol. In other words, after executing
this line you should be able to call `ls` on the component's `/out` directory
and see an entry called `fuchsia.examples.Echo`.

## Test the server

Rebuild:

```posix-terminal
fx build
```

Then run the server component:

```posix-terminal
ffx component run fuchsia-pkg://fuchsia.com/echo-hlcpp-server#meta/echo_server.cm
```

Note: Components are resolved using their [component URL][glossary.component-url],
which is determined with the [`fuchsia-pkg://`][glossary.fuchsia-pkg-url] scheme.

You should see output similar to the following in the device logs
(`ffx log`):

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
[glossary.component-instance-tree]: glossary/README.md#component-instance-tree
[glossary.component-url]: glossary/README.md#component-url
[glossary.fuchsia-pkg-url]: glossary/README.md#fuchsia-pkg-url
[glossary.moniker]: glossary/README.md#moniker
[fidl-intro]: development/languages/fidl/tutorials/hlcpp/basics/using-fidl.md
[building-components]: development/components/build.md
[products]: development/build/build_system/boards_and_products.md
[declaring-fidl]: development/languages/fidl/tutorials/fidl.md
[depending-fidl]: development/languages/fidl/tutorials/hlcpp/basics/using-fidl.md
[component-manager]: concepts/components/v2/component_manager.md
[protocol-open]: concepts/components/v2/capabilities/life_of_a_protocol_open.md
[discoverable]: reference/fidl/bindings/hlcpp-bindings.md#discoverable
[bindings-iface]: reference/fidl/bindings/hlcpp-bindings.md#protocols
[pipeline]: development/api/fidl.md#request-pipelining
[pipeline-tut]: development/languages/fidl/tutorials/hlcpp/topics/request-pipelining.md
[compiling-fidl]: development/languages/fidl/tutorials/fidl.md
[async-loop]: /zircon/system/ulib/async-loop/include/lib/async-loop/cpp/loop.h
[overview]: development/languages/fidl/tutorials/overview.md
