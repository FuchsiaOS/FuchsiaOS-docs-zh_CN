# Implement a synchronous C++ FIDL client

<!-- <<../../common/client/overview.md>> -->

## Prerequisites

This tutorial builds on the [FIDL server][server-tut] tutorial. For the
full set of FIDL tutorials, refer to the [overview][overview].

## Overview

This tutorial implements a client for a FIDL protocol and runs it
against the server created in the [previous tutorial][server-tut]. The client in
this tutorial is synchronous. There is an [alternate tutorial][async-client] for
asynchronous clients.

## Structure of the client example

The example code accompanying this tutorial is located in your Fuchsia checkout
at [`//examples/fidl/cpp/client_sync`][cpp-client-src]. It consists of a client
component and its containing package. For more information about building
components, see [Build components][build-components].

To get the client component up and running, there are three targets that are
defined in `//examples/fidl/cpp/client_sync/BUILD.gn`:

1. The raw executable file for the client. This produces a binary with the
   specified output name that can run on Fuchsia:

    ```gn
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/BUILD.gn" region_tag="bin" %}
    ```

1. A component that is set up to run the client executable.
   Components are the units of software execution on Fuchsia. A component is
   described by its manifest file. In this case `meta/client.cml`
   configures `echo-client` as an executable component which runs
   `fidl_echo_cpp_client_sync` in `:bin`.

    ```gn
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/BUILD.gn" region_tag="component" %}
    ```

    The server component manifest is located at
    `//examples/fidl/cpp/client_sync/meta/client.cml`. The binary name in the
    manifest must match the output name of the `executable` defined in
    `BUILD.gn`.

    ```json5
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/meta/client.cml" region_tag="example_snippet" %}
    ```

1. The component is then put into a package, which is the unit of software
   distribution on Fuchsia. In this case, the package contains a client and
   a server component, and [realm][glossary.realm] component to to declare the
   appropriate capabilities and routes.

    ```gn
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/BUILD.gn" region_tag="package" %}
    ```

### Building the client {#build}

1. Add the client to your build configuration. This only needs to be done once:

    ```posix-terminal
    fx set core.qemu-x64 --with //examples/fidl/cpp/client_sync
    ```

1. Build the client:

    ```posix-terminal
    fx build examples/fidl/cpp/client_sync
    ```

Note: This build configuration assumes your device target is the emulator.
To run the example on a physical device, select the appropriate
[product configuration][products] for your hardware.

## Connect to the protocol {#connect}

In its main function, the client component connects to the
`fuchsia.examples/Echo` protocol in its [namespace][glossary.namespace].

```cpp
int main(int argc, const char** argv) {
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="connect" %}

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

## Initialize the client {#client}

In order to make `Echo` requests to the server, initialize a client using the
client endpoint from the previous step.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="init-client" adjust_indentation="auto" %}
```

## Make FIDL calls {#call}

The methods to make FIDL calls are exposed behind a dereference operator, such
that FIDL calls look like `client->EchoString(...)`.

A two way call such as `EchoString` takes a request object, and returns a result
object indicating success or failure:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="echo-string" adjust_indentation="auto" %}
```

You may also use the designated initialization style double braces syntax
supported by [natural structs][natural-structs] and [tables][natural-tables]:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="echo-string-designated-first-line" adjust_indentation="auto" %}
```

A one way `SendString` call doesn't have a reply. The returned result represents
any errors that occurred when sending the request.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="send-string" adjust_indentation="auto" %}
```

## Handle events {#events}

Define an [event handler][event-handlers]:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="event-handler" adjust_indentation="auto" %}
```

Call `client.HandleOneEvent` to block until an event is received. If the event
was recognized and successfully decoded, `HandleOneEvent` returns
`fidl::Status::Ok()`. Otherwise, it returns an appropriate error:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="handle-one-event" adjust_indentation="auto" %}
```

## Make calls using wire domain objects {#using-wire}

The above tutorial makes client calls with
[natural domain objects][natural-types]: each call consumes request messages
represented using natural domain objects, and returns back replies also in
natural domain objects. When optimizing for performance and heap allocation, one
may make calls using [wire domain objects][wire-types]. To do that, insert a
`.wire()` before the dereference operator used when making calls, i.e.
`client.wire()->EchoString(...)`.

Make a `EchoString` two way call with wire types:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="echo-string-wire" adjust_indentation="auto" %}
```

Make a `SendString` one way call with wire types:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="send-string-wire" adjust_indentation="auto" %}
```

The relevant classes and functions used in a wire client call have similar
shapes to those used in a natural client call. When a different class or
function is called for, the wire counterpart is usually prefixed with `Wire`.
There are also differences in pointers vs references and argument structure:

* The `EchoString` method taking natural domain objects accepts a single
  argument that is the request domain object:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="echo-string-designated-first-line" adjust_indentation="auto" %}
  ```

  When the request payload is a struct, the `EchoString` method taking wire
  domain objects flattens the list of struct fields in the request body into
  separate arguments (here, a single `fidl::StringView` argument):

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="echo-string-wire-first-line" adjust_indentation="auto" %}
  ```

* The two way natural calls return a `fidl::Result<Method>`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="echo-string-designated" adjust_indentation="auto" %}
  ```

  * To check for success or error, use the `is_ok()` or `is_error()` method.
  * To access the response payload afterwards, use `value()` or `->`.
  * You may move out the result or the payload since these types all implement
    hierarchical object ownership.

  The two way wire calls return a `fidl::WireResult<Method>`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="echo-string-wire" adjust_indentation="auto" %}
  ```

  * To check for success, use the `ok()` method.
  * To access the response payload afterwards, use `value()` or `->`.
  * You cannot move the result object.

* One way calls also take the whole request domain object in the natural case,
  and flatten request struct fields into separate arguments in the wire case:

  ```cpp
  // Make a SendString call using natural types.
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="send-string-first-line" adjust_indentation="auto" %}

  // Make a SendString call using wire types.
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="send-string-wire-first-line" adjust_indentation="auto" %}
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
    fx set core.qemu-x64 --with //examples/fidl/cpp/client_sync
    ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

    ```posix-terminal
    ffx component run /core/ffx-laboratory:echo-client fuchsia-pkg://fuchsia.com/echo-cpp-client-sync#meta/echo_realm.cm
    ```

1. Start the `echo_client` instance:

    ```posix-terminal
    ffx component start /core/ffx-laboratory:echo_realm/echo_client
    ```

The server component starts when the client attempts to connect to the `Echo`
protocol. You should see output similar to the following in the device logs
(`ffx log`):

```none {:.devsite-disable-click-to-copy}
[echo_server][][I] Running echo server
[echo_client][I] Got response: hello
[echo_client][I] Got event: hi
[echo_client][I] Got response: hello
[echo_server][I] Client disconnected
```

Terminate the realm component to stop execution and clean up the component
instances:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo_realm
```

## Wire domain objects only client {#wire-client}

`fidl::SyncClient` supports making calls with both
[natural domain objects][natural-types] and [wire domain objects][wire-types].
If you only need to use wire domain objects, you may create a `WireSyncClient`
that exposes the equivalent method call interface as the subset obtained from
calling `client.wire()` on a `fidl::SyncClient`.

A `WireSyncClient` is created the same way as a `SyncClient`:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/wire/main.cc" region_tag="init-client" adjust_indentation="auto" %}
```

`fidl::SyncClient` always exposes received events to the user in the form of
natural domain objects. On the other hand, `fidl::WireSyncClient` will expose
received events in the form of wire domain objects. To do that, the event
handler passed to a `WireSyncClient` needs to implement
`fidl::WireSyncEventHandler<Protocol>`.

* Implementing a natural event handler:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/main.cc" region_tag="event-handler" adjust_indentation="auto" %}
  ```

* Implementing a wire event handler:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/client_sync/wire/main.cc" region_tag="event-handler" adjust_indentation="auto" %}
  ```

The full example code for using a wire client is located in your Fuchsia
checkout at [`//examples/fidl/cpp/client_sync/wire`][cpp-wire-client-src].

<!-- xrefs -->
[glossary.realm]: /docs/glossary/README.md#realm
[glossary.namespace]: /docs/glossary/README.md#namespace
[build-components]: /docs/development/components/build.md
[cpp-client-src]: /examples/fidl/cpp/client_sync
[cpp-wire-client-src]: /examples/fidl/cpp/client_sync/wire
[server-tut]: /docs/development/languages/fidl/tutorials/cpp/basics/server.md
[server-handler]: /docs/development/languages/fidl/tutorials/cpp/basics/server.md#server-handler
[async-client]: /docs/development/languages/fidl/tutorials/cpp/basics/client.md
[overview]: /docs/development/languages/fidl/tutorials/overview.md
[open]: https://fuchsia.dev/reference/fidl/fuchsia.io#Directory.Open
[pipelining]: /docs/development/api/fidl.md#request-pipelining
[pipelining-tut]: /docs/development/languages/fidl/tutorials/cpp/topics/request-pipelining.md
[products]: /docs/development/build/build_system/boards_and_products.md
[resultof]: /docs/reference/fidl/bindings/cpp-bindings.md#resultof
[sync-client]: /docs/reference/fidl/bindings/cpp-bindings.md#sync-client
[event-handlers]: /docs/reference/fidl/bindings/cpp-bindings.md#events
[natural-types]: /docs/development/languages/fidl/tutorials/cpp/basics/domain-objects.md#using-natural
[natural-structs]: /docs/development/languages/fidl/tutorials/cpp/basics/domain-objects.md#natural_structs
[natural-tables]: /docs/development/languages/fidl/tutorials/cpp/basics/domain-objects.md#natural_tables
[wire-types]: /docs/development/languages/fidl/tutorials/cpp/basics/domain-objects.md#using-wire
