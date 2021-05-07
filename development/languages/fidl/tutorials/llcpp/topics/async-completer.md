# Responding to requests asynchronously in LLCPP

## Prerequisites

This tutorial builds on the [LLCPP getting started tutorials][overview].

## Overview

In the `Echo` implementation from the [server tutorial][server-tut], the server code responded
to `EchoString` requests using the completer.

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="impl" highlight="14,15,16" %}
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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="echo" %}
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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/async_completer/client/main.cc" region_tag="main" highlight="14,16,17,18,19,20,21,22,23,24,25,26" %}
```

The loop is run `kNumEchoes` times (which is by default 3), and will print the time elapsed since
the first request every time it receives a response. After it receives `kNumEchoes` responses, the
code quits from the loop.

## Implement the server

The `main()` function from the server code is the same as in the [server tutorial][server-tut].
The difference lies in the implementation of `Echo`:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/async_completer/server/main.cc" region_tag="impl" %}
```

When an `EchoString` request is received, the server calls `async::PostDelayedTask`. This function
takes a dispatcher, a callback, and a duration, and will execute the callback at the end of the
duration. This call emulates a long running task that returns its result through a callback when it
is finished. The handler uses `PostDelayedTask` to wait 5 seconds before echoing the
request value back to the client.

A key point is that the completer being moved into the lambda capture is the async completer. A
`::Sync` completer can be converted to the `::Async` counterpart by using the `ToAsync()` method.

For further information on the completer API, refer to the [LLCPP bindings reference][bindings-ref].

## Run the example

First, build the code:

```
fx set core.x64 --with //examples/fidl/llcpp/async_completer/client --with //examples/fidl/llcpp/async_completer/server --with //examples/fidl/test:echo-launcher

fx build
```

Then run the example:

```
fx shell run fuchsia-pkg://fuchsia.com/echo-launcher#meta/launcher.cmx fuchsia-pkg://fuchsia.com/echo-llcpp-client-async#meta/echo-client.cmx fuchsia-pkg://fuchsia.com/echo-llcpp-server-async#meta/echo-server.cmx fuchsia.examples.Echo
```

You should see the following print output in the QEMU console (or using `fx log`):

```
[193539.863] 884542:884544> Running echo server
[193539.871] 884542:884544> echo_server_llcpp: Incoming connection for fuchsia.examples.Echo
[193544.899] 884632:884636> Got response after 5 seconds
[193544.899] 884632:884636> Got response after 5 seconds
[193544.899] 884632:884636> Got response after 5 seconds
```

By using the async completer, the client receives all 3 responses after 5 seconds, rather than in
5/10/15 seconds.

<!-- xrefs -->
[src]: /examples/fidl/llcpp/async_completer
[server-tut]: /docs/development/languages/fidl/tutorials/llcpp/basics/server.md
[client-tut]: /docs/development/languages/fidl/tutorials/llcpp/basics/client.md
[overview]: /docs/development/languages/fidl/tutorials/llcpp/README.md
[bindings-ref]: /docs/reference/fidl/bindings/llcpp-bindings.md#server-completers
