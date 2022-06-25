# Responding to requests asynchronously in LLCPP

## Prerequisites

This tutorial builds on the [LLCPP getting started tutorials][overview].

## Overview

In the `Echo` implementation from the [server tutorial][server-tut], the server code responded
to `EchoString` requests using the completer.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="impl" highlight="32,33,34,35,36" %}
```

Notice that the type for the completer has `::Sync`. This indicates the default mode of operation:
the server must synchronously make a reply before returning from the handler function. Enforcing
this allows optimizations since the bookkeeping metadata for making a reply can be stack-allocated.

This tutorial provides an example of how to respond to requests asynchronously, by converting the
sync completer into an async completer.

The full example code for this tutorial is located at
[//examples/fidl/llcpp/async_completer][src].

### The Echo protocol

This example uses the `Echo` protocol from the examples library:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="echo" %}
```

As part of this tutorial, you will implement a client that makes multiple `EchoString` requests in
succession. The server will respond to these requests asynchronously, emulating a scenario where
the server must execute a long running task before sending a response. By using an
async completer, these long running tasks can be completed asynchronously.

## Implement the client

The client code is mostly similar to the code from the [client tutorial][client-tut]. The differences
are highlighted in this section.

After connecting to the server, the client will make multiple `EchoString` requests inside of a
for loop:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/async_completer/client/main.cc" region_tag="main" highlight="14,15,16,17,18,19,20,21,22,23,24,25,26" %}
```

The loop is run `kNumEchoes` times (which is by default 3), and will print the time elapsed since
the first request every time it receives a response. After it receives `kNumEchoes` responses, the
code quits from the loop.

## Implement the server

The `main()` function from the server code is the same as in the [server tutorial][server-tut].
The difference lies in the implementation of `Echo`:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/async_completer/server/main.cc" region_tag="impl" %}
```

When an `EchoString` request is received, the server calls `async::PostDelayedTask`. This function
takes a dispatcher, a callback, and a duration, and will execute the callback at the end of the
duration. This call emulates a long running task that returns its result through a callback when it
is finished. The handler uses `PostDelayedTask` to wait 5 seconds before echoing the
request value back to the client.

A key point is that the completer being moved into the lambda capture is the async completer. A
`::Sync` completer can be converted to the `::Async` counterpart by using the `ToAsync()` method.
For further information on the completer API, refer to the [LLCPP bindings reference][bindings-ref].

Another noteworthy aspect is that the request views provided to method handlers
do not own the request message. In order to use the request parameters after the
`EchoString` method returns, we need to copy relevant fields to an owned type,
here `value_owned` in the lambda captures. For further information on the memory
ownership, refer to the [LLCPP Memory Management][memory-management].

## Run the example

In order for the client and server to communicate using the `Echo` protocol,
component framework must route the `fuchsia.examples.Echo` capability from the
server to the client. For this tutorial, a [realm][glossary.realm] component is
provided to declare the appropriate capabilities and routes.

Note: You can explore the full source for the realm component at
[`//examples/fidl/echo-realm`](/examples/fidl/echo-realm)

1. Configure your build to include the provided package that includes the
   echo realm, server, and client:

    ```posix-terminal
    fx set core.qemu-x64 --with //examples/fidl/llcpp:echo-llcpp-async
    ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

    ```posix-terminal
    ffx component run fuchsia-pkg://fuchsia.com/echo-llcpp-async#meta/echo_realm.cm
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
[echo_server][][I] echo_server_llcpp: Incoming connection for fuchsia.examples.Echo
...
[echo_client][][I] Got response after 5 seconds
[echo_client][][I] Got response after 5 seconds
[echo_client][][I] Got response after 5 seconds
```

By using the async completer, the client receives all 3 responses after 5 seconds,
rather than individually at 5 second intervals.

Terminate the realm component to stop execution and clean up the component
instances:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo_realm
```

<!-- xrefs -->
[src]: /examples/fidl/llcpp/async_completer
[server-tut]: development/languages/fidl/tutorials/llcpp/basics/server.md
[client-tut]: development/languages/fidl/tutorials/llcpp/basics/client.md
[overview]: development/languages/fidl/tutorials/llcpp/README.md
[bindings-ref]: reference/fidl/bindings/llcpp-bindings.md#server-completers
[memory-management]: development/languages/fidl/tutorials/llcpp/topics/memory-ownership.md
