# Implement a C++ FIDL server

## Prerequisites

This tutorial builds on the [domain objects][domain-objects] tutorial. For the
full set of FIDL tutorials, refer to the [overview][overview].

## Overview

<!-- TODO(fxbug.dev/58758) <<../../common/server/overview.md>> -->

This tutorial shows you how to implement a server for a FIDL protocol
(`fuchsia.examples/Echo`) and run it on Fuchsia. This protocol has one method
of each kind: a one-way method, a two-way method, and an event:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="echo" %}
```

For more on FIDL methods and messaging models, refer to the
[FIDL concepts][concepts] page.

This document covers how to complete the following tasks:

* [Implement the FIDL protocol](#protocol).
* [Publish the protocol implementation](#serve).
* [Run the server](#run).

## Structure of the server example

The example code accompanying this tutorial is located in your Fuchsia checkout
at [`//examples/fidl/cpp/server`][cpp-server-src]. It consists of a server
component and its containing package. For more information about building
components, see [Build components][build-components].

To get the server component up and running, there are three targets that are
defined in `//examples/fidl/cpp/server/BUILD.gn`:

1. The raw executable file for the server. This produces a binary with the
   specified output name that can run on Fuchsia:

    ```gn
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/BUILD.gn" region_tag="bin" %}
    ```

1. A component that is set up to run the server executable.
   Components are the units of software execution on Fuchsia. A component is
   described by its manifest file. In this case `meta/server.cml`
   configures `echo-server` as an executable component which runs
   `fidl_echo_cpp_server` in `:bin`.

    ```gn
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/BUILD.gn" region_tag="component" %}
    ```

    The server component manifest is located at
    `//examples/fidl/cpp/server/meta/server.cml`. The binary name in the
    manifest must match the output name of the `executable` defined in
    `BUILD.gn`.

    ```json5
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/meta/server.cml" region_tag="example_snippet" %}
    ```

1. The component is then put into a package, which is the unit of software
   distribution on Fuchsia. In this case, the package just contains a
   single component.

    ```gn
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/BUILD.gn" region_tag="package" %}
    ```

### Building the server {#build}

You may build the server package via the following:

1. Add the server to your build configuration. This only needs to be done once:

    ```posix-terminal
    fx set core.qemu-x64 --with //examples/fidl/cpp/server
    ```

1. Build the server package:

    ```posix-terminal
    fx build examples/fidl/cpp/server
    ```

Note: This build configuration assumes your device target is the emulator.
To run the example on a physical device, select the appropriate
[product configuration][products] for your hardware.

## Implement the FIDL protocol {#protocol}

`EchoImpl` implements the server request handler for the `fuchsia.examples/Echo`
protocol. To do so, `EchoImpl` inherits from the generated pure virtual server
interface `fidl::Server<fuchsia_examples::Echo>`, and overrides its pure virtual
methods corresponding to every one way and two way call:

```cpp
class EchoImpl : public fidl::Server<fuchsia_examples::Echo> {
 public:
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/main.cc" region_tag="impl" %}

  // ... other methods from examples/fidl/cpp/server/main.cc omitted, to be covered later.

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/main.cc" region_tag="binding_ref" %}
};
```

Note: in this tutorial the `EchoString` handler replies synchronously. For
asynchronous replies, see
[responding to requests asynchronously][responding-asynchronously].

### Bind the implementation to a server endpoint

Implementing the request handlers is only half the story. The server needs to be
able to monitor new messages that arrives on a
[server endpoint][server-endpoint]. To do this, `EchoImpl` defines two more
methods: a `BindSelfManagedServer` static factory function that creates a
new `EchoImpl` instance to handle requests on a new server endpoint
`fidl::ServerEnd<fuchsia_examples::Echo>`, and an `OnUnbound` method that
is called when the connection is torn down:

```cpp
/* Inside `class EchoImpl {`... */

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/main.cc" region_tag="bind_server" %}
```

## Publish the protocol implementation {#server}

A component that implements a FIDL protocol can expose that FIDL
protocol to other components. This is done by publishing the protocol
implementation to the component's
[outgoing directory][glossary.outgoing-directory]. This complete process is
described in further detail in the [Life of a protocol open][protocol-open].
We can use `component::OutgoingDirectory` from the C++ component runtime library
to perform the heavy lifting.

To depend on the component runtime library:

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/BUILD.gn" region_tag="bin" highlight="12" %}
```

Import the library at the top of `examples/fidl/cpp/server/main.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/main.cc" region_tag="includes" highlight="3" %}
```

Serve the component's outgoing directory:

```cpp
int main(int argc, const char** argv) {
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/main.cc" region_tag="serve-out-dir" %}

  // ...
```

### Serve the protocol {#server-handler}

The server then registers the Echo protocol using `outgoing.AddProtocol`.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/main.cc" region_tag="main" highlight="27,28,29,30,31,32,33,34,35,36" %}
```

The call to `AddProtocol` installs a handler at the name of the FIDL protocol
(`fidl::DiscoverableProtocolName<fuchsia_examples::Echo>`, which is the string
`"fuchsia.examples.Echo"`). When a client component connects to
`fuchsia.examples.Echo`, `outgoing` will call the lambda function that we
created with a server endpoint corresponding to the client endpoint from the
client, and this lambda function will call the `EchoImpl::BindSelfManagedServer`
detailed above to bind the server endpoint to a new instance of `EchoImpl`.

Our main method will keep listening for incoming requests on the
[async loop][async-loop].

## Test the server {#run}

After [building the server](#build), you may run the example on a running
instance of Fuchsia emulator via

```posix-terminal
ffx component run /core/ffx-laboratory:echo-server fuchsia-pkg://fuchsia.com/echo-cpp-server#meta/echo_server.cm
```

Note: Components are resolved using their [component URL][glossary.component-url],
which is determined with the [`fuchsia-pkg://`][glossary.fuchsia-pkg-url] scheme.

You should see output similar to the following in the device logs (`ffx log`):

```none {:.devsite-disable-click-to-copy}
[ffx-laboratory:echo_server][][I] Running C++ echo server with natural types
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

## Serve requests using wire domain objects {#using-wire}

The above tutorial implements a server with
[natural domain objects][natural-types]: the server receives requests
represented in natural domain objects, and sends replies encoded from natural
domain objects. When optimizing for performance and heap allocation, one may
implement a server that speaks [wire domain objects][wire-types], i.e. a wire
server. Here is the `EchoImpl` rewritten to use wire domain objects:

```cpp
class EchoImpl final : public fidl::WireServer<fuchsia_examples::Echo> {
 public:
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/wire/main.cc" region_tag="handlers" %}

  // ... |BindSelfManagedServer| etc omitted. Those stay the same.
};
```

The relevant classes and functions used in a wire server have similar shapes to
those used in a natural server. When a different class or function is called
for, the wire counterpart is usually prefixed with `Wire`. There are also
small differences in pointers vs references and argument structure:

* The server interface implemented by a natural server is
  `fidl::Server<fuchsia_examples::Echo>`. The server interface implemented by a
  wire server is `fidl::WireServer<fuchsia_examples::Echo>`.

* The handler function in a natural server takes a reference to the request
  message. The `Reply` method takes a single argument that is the response
  payload domain object:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/main.cc" region_tag="impl-echo-string" adjust_indentation="auto" %}
  ```

  Whereas the handler function in a wire server takes a view (akin to a pointer)
  of the request message. When the response payload is a struct, the `Reply`
  method flattens the list of struct fields in the response payload into
  separate arguments (here, a single `fidl::StringView` argument):

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/wire/main.cc" region_tag="impl-echo-string" adjust_indentation="auto" %}
  ```

* The function to send events with natural types is `fidl::SendEvent`. The
  function to send events with wire types is `fidl::WireSendEvent`. Struct
  fields are also flattened into separate arguments when sending an event.

The same `fidl::BindServer` function may be used to bind either a natural server
or a wire server.

The full example code for a wire server is located in your Fuchsia checkout
at [`//examples/fidl/cpp/server/wire`][cpp-wire-server-src].

<!-- xrefs -->
[glossary.component-instance-tree]: /glossary/README.md#component-instance-tree
[glossary.component-url]: /glossary/README.md#component-url
[glossary.fuchsia-pkg-url]: /glossary/README.md#fuchsia-pkg-url
[glossary.moniker]: /glossary/README.md#moniker
[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[cpp-server-src]: /examples/fidl/cpp/server
[cpp-wire-server-src]: /examples/fidl/cpp/server/wire
[domain-objects]: /development/languages/fidl/tutorials/cpp/basics/domain-objects.md
[build-components]: /development/components/build.md
[products]: /development/build/build_system/boards_and_products.md
[protocol-open]: /concepts/components/v2/capabilities/life_of_a_protocol_open.md#binding_to_a_component_and_sending_a_protocol_channel
[compiling-fidl]: /development/languages/fidl/tutorials/fidl.md
[async-loop]: /zircon/system/ulib/async-loop/include/lib/async-loop/cpp/loop.h
[overview]: /development/languages/fidl/tutorials/overview.md
[concepts]: /concepts/fidl/overview.md
[responding-asynchronously]: /development/languages/fidl/tutorials/cpp/topics/async-completer.md
[server-endpoint]: /reference/fidl/language/language.md#protocols-use
[natural-types]: /development/languages/fidl/tutorials/cpp/basics/domain-objects.md#using-natural
[wire-types]: /development/languages/fidl/tutorials/cpp/basics/domain-objects.md#using-wire
