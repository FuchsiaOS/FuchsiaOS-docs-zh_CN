# Implement a C++ FIDL client

<!-- TODO(fxbug.dev/58758) <<../../common/client/overview.md>> -->

## Prerequisites

This tutorial builds on the [FIDL server][server-tut] tutorial. For the
full set of FIDL tutorials, refer to the [overview][overview].

## Overview

This tutorial implements a client for a FIDL protocol and runs it against the
server created in the [previous tutorial][server-tut]. The client in this
tutorial is asynchronous. There is an [alternate tutorial][sync-client] for
synchronous clients.

## Structure of the client example

The example code accompanying this tutorial is located in your Fuchsia checkout
at [`//examples/fidl/cpp/client`][cpp-client-src]. It consists of a client
component and its containing package. For more information about building
components, see [Build components][build-components].

To get the client component up and running, there are three targets that are
defined in `//examples/fidl/cpp/client/BUILD.gn`:

1. The raw executable file for the client. This produces a binary with the
   specified output name that can run on Fuchsia:

    ```gn
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/BUILD.gn" region_tag="bin" %}
    ```

1. A component that is set up to run the client executable.
   Components are the units of software execution on Fuchsia. A component is
   described by its manifest file. In this case `meta/client.cml`
   configures `echo-client` as an executable component which runs
   `fidl_echo_cpp_client` in `:bin`.

    ```gn
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/BUILD.gn" region_tag="component" %}
    ```

    The server component manifest is located at
    `//examples/fidl/cpp/client/meta/client.cml`. The binary name in the
    manifest must match the output name of the `executable` defined in
    `BUILD.gn`.

    ```json5
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/meta/client.cml" region_tag="example_snippet" %}
    ```

1. The component is then put into a package, which is the unit of software
   distribution on Fuchsia. In this case, the package contains a client and
   a server component, and [realm][glossary.realm] component to to declare the
   appropriate capabilities and routes.

    ```gn
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/BUILD.gn" region_tag="package" %}
    ```

### Building the client {#build}

1. Add the client to your build configuration. This only needs to be done once:

    ```posix-terminal
    fx set core.qemu-x64 --with //examples/fidl/cpp/client
    ```

1. Build the client:

    ```posix-terminal
    fx build examples/fidl/cpp/client
    ```

Note: This build configuration assumes your device target is the emulator.
To run the example on a physical device, select the appropriate
[product configuration][products] for your hardware.

## Connect to the protocol {#connect}

In its main function, the client component connects to the
`fuchsia.examples/Echo` protocol in its [namespace][glossary.namespace].

```cpp
int main(int argc, const char** argv) {
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="connect" %}

  // ...
```

Note: There may still be asynchronous errors that causes peer of `client_end` to
be closed, because the [open][open] operation uses
[protocol request pipelining][pipelining]: a pattern of making a request to
connect the server end of the channel to an implementation, then immediately
beginning to use the client endpoint. This topic is covered further in a
separate [tutorial][pipelining-tut].

In parallel, the component manager will route the request to the server
component. The [server handler][server-handler] implemented in the server
tutorial will be called with the server endpoint, binding the channel to the
server implementation.

An important point to note here is that this code assumes that the component's
namespace already contains an instance of the `Echo` protocol. When
[running the example](#run) at the end of the tutorial, a
[realm][glossary.realm] component is used to route the protocol from the server
and offer it to the client component.

## Initialize the event loop {#event-loop}

An asynchronous client needs an `async_dispatcher_t*` to asynchronously monitor
messages from a channel. The `async::Loop` provides a dispatcher implementation
backed by an event loop.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="async-loop" %}
```

The dispatcher is used to run pieces of asynchronous code. It is first used to
run the `EchoString` method, and quits when the response is received. It is then
run after calling the `SendString` in order to listen for events, and quits when
an `OnString` event is received. The call to `ResetQuit()` in between these two
instances allows the client to reuse the loop.

## Initialize the client {#client}

In order to make `Echo` requests to the server, initialize a client using the
client endpoint from the previous step, the loop dispatcher, as well as an event
handler delegate:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="event-handler" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="init-client" %}
```

## Make FIDL calls {#call}

The methods to make FIDL calls are exposed behind a dereference operator, such
that FIDL calls look like `client->EchoString(...)`.

An asynchronous `EchoString` call takes a request object, and accepts a callback
which is invoked with the result of the call, indicating success or failure:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="two_way_natural_result" %}
```

You may also use the designated initialization style double braces syntax
supported by [natural structs][natural-structs] and [tables][natural-tables]:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="two_way_designated_natural_result" %}
        // ... callback ...
```

A one way `SendString` call doesn't have a reply, so callbacks are not needed.
The returned result represents any errors occurred when sending the request.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="one_way_natural" %}
```

### Make calls using wire domain objects {#using-wire}

The above tutorial makes client calls with
[natural domain objects][natural-types]: each call consumes request messages
represented using natural domain objects, and returns back replies also in
natural domain objects. When optimizing for performance and heap allocation, one
may make calls using [wire domain objects][wire-types]. To do that, insert a
`.wire()` before the dereference operator used when making calls, i.e.
`client.wire()->EchoString(...)`.

Make a `EchoString` two way call with wire types:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="two_way_wire_result" %}
```

Make a `SendString` one way call with wire types:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="one_way_wire" %}
```

The relevant classes and functions used in a wire client call have similar
shapes to those used in a natural client call. When a different class or
function is called for, the wire counterpart is usually prefixed with `Wire`.
There are also differences in pointers vs references and argument structure:

* The `EchoString` method taking natural domain objects accepts a single
  argument that is the request domain object:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="two_way_designated_natural_result" adjust_indentation="auto" %}
  ```

  When the request payload is a struct, the `EchoString` method taking wire
  domain objects flattens the list of struct fields in the request body into
  separate arguments (here, a single `fidl::StringView` argument):

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="two_way_wire_result_first_line" adjust_indentation="auto" %}
  ```

* The callback in async natural calls accepts a `fidl::Result<Method>&`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="two_way_natural_result" adjust_indentation="auto" highlight="2" %}
  ```

  * To check for success or error, use the `is_ok()` or `is_error()` method.
  * To access the response payload afterwards, use `value()` or `->`.
  * You may move out the result or the payload since these types all implement
    hierarchical object ownership.

  The callback in async wire calls accepts a `fidl::WireUnownedResult<Method>&`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="two_way_wire_result" adjust_indentation="auto" highlight="2" %}
  ```

  * To check for success, use the `ok()` method.
  * To access the response payload afterwards, use `value()` or `->`.
  * You must synchronously use the result within the callback. The result type
    is *unowned*, meaning it only borrows the response allocated somewhere else
    by the FIDL runtime.

* One way calls also take the whole request domain object in the natural case,
  and flatten request struct fields into separate arguments in the wire case:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="one_way_natural_first_line" adjust_indentation="auto" %}

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="one_way_wire_first_line" adjust_indentation="auto" %}
  ```

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
    fx set core.qemu-x64 --with //examples/fidl/cpp/client
    ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

    ```posix-terminal
    ffx component run /core/ffx-laboratory:echo-client fuchsia-pkg://fuchsia.com/echo-cpp-client#meta/echo_realm.cm
    ```

1. Start the `echo_client` instance:

    ```posix-terminal
    ffx component start /core/ffx-laboratory:echo_realm/echo_client
    ```

The server component starts when the client attempts to connect to the `Echo`
protocol. You should see output similar to the following in the device logs
(`ffx log`):

```none {:.devsite-disable-click-to-copy}
[echo_server][][I] Running C++ echo server with natural types
[echo_server][][I] Incoming connection for fuchsia.examples.Echo
[echo_client][][I] (Natural types) got response: hello
[echo_client][][I] (Natural types) got response: hello
[echo_client][][I] (Natural types) got response: hello
[echo_client][][I] (Natural types) got event: hello
[echo_client][][I] (Wire types) got response: hello
[echo_client][][I] (Natural types) got event: hello
[echo_server][][I] Client disconnected
```

Terminate the realm component to stop execution and clean up the component
instances:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo_realm
```

## Wire domain objects only client {#wire-client}

`fidl::Client` supports making calls with both
[natural domain objects][natural-types] and [wire domain objects][wire-types].
If you only need to use wire domain objects, you may create a `WireClient` that
exposes the equivalent method call interface as the subset obtained from calling
`client.wire()` on a `fidl::Client`.

A `WireClient` is created the same way as a `Client`:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/wire/main.cc" region_tag="init-client-short" adjust_indentation="auto" %}
```

`fidl::Client` always exposes received events to the user in the form of natural
domain objects. On the other hand, `fidl::WireClient` will expose received
events in the form of wire domain objects. To do that, the event handler passed
to a `WireClient` needs to implement `fidl::WireAsyncEventHandler<Protocol>`.

* Implementing a natural event handler:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/main.cc" region_tag="event-handler-short" adjust_indentation="auto" %}

    // ...
  };
  ```

* Implementing a wire event handler:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/wire/main.cc" region_tag="event-handler-short" adjust_indentation="auto" %}

    // ...
  };
  ```

### Synchronous call

`WireClient` objects also allows synchronous calls, which will block until the
response is received and return the response object. These may be selected
using the `.sync()` accessor. (e.g. `client.sync()->EchoString()`).

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client/wire/main.cc" region_tag="sync-call" %}
```

In synchronous calls, a [result object][resultof] is returned, synchronously
communicating the success or failure of the call.

The full example code for using a wire client is located in your Fuchsia
checkout at [`//examples/fidl/cpp/client/wire`][cpp-wire-client-src].

<!-- xrefs -->
[glossary.realm]: /glossary/README.md#realm
[glossary.namespace]: /glossary/README.md#namespace
[bindings-ref]: /reference/fidl/bindings/cpp-bindings.md
[build-components]: /development/components/build.md
[cpp-client-src]: /examples/fidl/cpp/client
[cpp-wire-client-src]: /examples/fidl/cpp/client/wire
[resultof]: /reference/fidl/bindings/cpp-bindings.md#resultof
[server-handler]: /development/languages/fidl/tutorials/cpp/basics/server.md#server-handler
[server-tut]: /development/languages/fidl/tutorials/cpp/basics/server.md
[sync-client]: /development/languages/fidl/tutorials/cpp/basics/sync-client.md
[overview]: /development/languages/fidl/tutorials/overview.md
[environment]: /concepts/components/v2/environments.md
[open]: https://fuchsia.dev/reference/fidl/fuchsia.io#Directory.Open
[pipelining]: /development/api/fidl.md#request-pipelining
[pipelining-tut]: /development/languages/fidl/tutorials/cpp/topics/request-pipelining.md
[products]: /development/build/build_system/boards_and_products.md
[natural-types]: /development/languages/fidl/tutorials/cpp/basics/domain-objects.md#using-natural
[natural-structs]: /development/languages/fidl/tutorials/cpp/basics/domain-objects.md#natural_structs
[natural-tables]: /development/languages/fidl/tutorials/cpp/basics/domain-objects.md#natural_tables
[wire-types]: /development/languages/fidl/tutorials/cpp/basics/domain-objects.md#using-wire
