# Protocol request pipelining in LLCPP

## Prerequisites

This tutorial builds on the [LLCPP getting started tutorials][overview].

## Overview

<!-- TODO(fxbug.dev/58758) <<../../common/pipelining/overview.md>> -->

A common aspect of using FIDL on Fuchsia is passing protocol endpoints across
protocols. Many FIDL messages include either the client end or the server end of
a channel, where the channel is used to communicate over a different FIDL
protocol. In this case, the client end allows making requests to the specified
protocol, whereas the server end must implement the specified protocol. An
alternate set of terms for client end and server end are protocol and protocol
request.

This tutorial covers:

* The usage of these client and server ends, both in FIDL and in the LLCPP
  FIDL bindings.
* The request pipelining pattern and its benefits.

The full example code for this tutorial is located at
[//examples/fidl/llcpp/request_pipelining][src].

### The FIDL protocol

<!-- TODO(fxbug.dev/58758) <<../../common/pipelining/launcher.md>> -->

This tutorial implements the `EchoLauncher` protocol from the
[fuchsia.examples library][examples-fidl]:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="launcher" %}
```

This is a protocol that lets clients retrieve an instance of the `Echo`
protocol. Clients can specify a prefix, and the resulting `Echo` instance
adds that prefix to every response.

There are two methods that can be used to accomplish this:

* `GetEcho`: Takes the prefix as a request, and responds with the client end of
  a channel connected to an implementation of the `Echo` protocol. After
  receiving the client end in the response, the client can start making requests
  on the `Echo` protocol using the client end.
* `GetEchoPipelined`: Takes the server end of a channel as one of the request
  parameters and binds an implementation of `Echo` to it. The client that
  made the request is assumed to already hold the client end, and will
  start making `Echo` requests on that channel after calling `GetEchoPipelined`.

As the name suggests, the latter uses a pattern called protocol request
pipelining, and is the preferred approach. This tutorial implements both
approaches.

## Implement the server

### Implement the Echo protocol

This implementation of `Echo` allows specifying a prefix in order to
distinguish between the different instances of `Echo` servers:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/request_pipelining/server/main.cc" region_tag="echo-impl" %}
```

The `SendString` handler is empty as the client just uses `EchoString`.

### Implement the EchoLauncher protocol

This class responds to either method by launching an instance of an `Echo`
server, and then stores the `EchoImpl` instance in a member variable to ensure
that its lifetime matches that of the launcher. The code for running an `Echo`
server given a specific prefix and channel is abstracted into a helper
`RunEchoServer` method:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/request_pipelining/server/main.cc" region_tag="launcher-impl" %}
```

For `GetEcho`, the code first needs to instantiate both ends of the channel. It
then launches an `Echo` instance using the server end, and then sends a response
back with the client end. For `GetEchoPipelined`, the client has already done
the work of creating both ends of the channel. It keeps one end and has passed
the other to the server, so all the code needs to do is call `RunEchoServer`.

### Serve the EchoLauncher protocol

The main loop should is the same as in the
[server tutorial][server-tut-main] but serves an `EchoLauncher` instead of `Echo`.

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/request_pipelining/server/main.cc" region_tag="main" %}
```

## Build the server

Optionally, to check that things are correct, try building the server:

1. Configure your GN build to include the server:

   ```
   fx set core.x64 --with //examples/fidl/llcpp/request_pipelining/server
   ```
2. Build the Fuchsia image:

   ```
   fx build
   ```

## Implement the client

Note: Most of the client code in `client/main.cc` should be familiar after
having followed the [client tutorial][client-tut]. The different parts of the
code are covered in more detail here.

After connecting to the `EchoLauncher` server, the client
code connects to one instance of `Echo` using `GetEcho` and another using
`GetEchoPipelined` and then makes an `EchoString` request on each instance.

This is the non-pipelined code:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/request_pipelining/client/main.cc" region_tag="main" highlight="11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29" %}
```

This code has two layers of callbacks:

* The outer layer handles the launcher request.
* The inner layer handles the `EchoString` request.

Also, the code instantiates the `fidl::Client<Echo>` in the outer scope then
`Bind`s it inside of the callback, so that the client's lifetime matches the
lifetime of the component. This client needs to be in scope when the echo
response is received, which will most likely be after the top level callback
returns.

Despite having to initialize the channel first, the pipelined code is much
simpler:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/request_pipelining/client/main.cc" region_tag="main" highlight="31,32,33,34,35,36,37,38,39,40,41,42,43,44,45" %}
```

Unlike in the [client tutorial][client-tut], the async loop is run to completion
once, which runs both non-pipelined and pipelined code concurrently in order to
observe the order of operations. The client keeps track of the number of
responses being received, so that it can quit the loop once no more messages
from the server are expected.

## Build the client

Optionally, to check that things are correct, try building the client:

1. Configure your GN build to include the server:

   ```
   fx set core.x64 --with //examples/fidl/llcpp/request_pipelining/client`
   ```

2. Build the Fuchsia image:

   ```
   fx build
   ```

## Run the example code

To run the example code:

1. Configure your GN build as follows:

   ```
   fx set core.x64 --with //examples/fidl/llcpp/request_pipelining/client --with //examples/fidl/llcpp/request_pipelining/server --with //examples/fidl/test:echo-launcher
   ```

2. Run the example:

   ```
   fx shell run fuchsia-pkg://fuchsia.com/echo-launcher#meta/launcher.cmx fuchsia-pkg://fuchsia.com/echo-launcher-llcpp-client#meta/echo-client.cmx fuchsia-pkg://fuchsia.com/echo-launcher-llcpp-server#meta/echo-server.cmx fuchsia.examples.EchoLauncher
   ```

You should see the following print output in the QEMU console (or using `fx log`):

```
[190179.987] 864900:864902> Running echo launcher server
[190180.007] 864900:864902> echo_server_llcpp: Incoming connection for fuchsia.examples.EchoLauncher
[190180.028] 864900:864902> Got non pipelined request
[190180.040] 864900:864902> Got pipelined request
[190180.040] 864900:864902> Got echo request for prefix pipelined:
[190180.049] 864900:864902> Got echo request for prefix non pipelined:
[190180.049] 864810:864812> Got echo response pipelined: hello!
[190180.049] 864810:864812> Got echo response non pipelined: hello!
```

Based on the print order, you can see that the pipelined case is faster. The
echo response for the pipelined case arrives first, even though the non
pipelined request is sent first, since request pipelining saves a roundtrip
between the client and server and allows clients to enqueue messages on the
protocol request channel before the server has proceeded any requests. Servers
then handle the requests as soon as they are ready. Request pipelining also
simplifies the code.


For further reading about protocol request pipelining, including how to handle
protocol requests that may fail, see the [FIDL API rubric][rubric].

<!-- xrefs -->
[src]: /examples/fidl/llcpp/request_pipelining
[server-tut]: /docs/development/languages/fidl/tutorials/llcpp/basics/server.md
[server-tut-main]: /docs/development/languages/fidl/tutorials/llcpp/basics/server.md#main
[client-tut]: /docs/development/languages/fidl/tutorials/llcpp/basics/client.md
[rubric]: /docs/concepts/api/fidl.md#request-pipelining
[overview]: /docs/development/languages/fidl/tutorials/llcpp/README.md
[examples-fidl]: /examples/fidl/fuchsia.examples/
