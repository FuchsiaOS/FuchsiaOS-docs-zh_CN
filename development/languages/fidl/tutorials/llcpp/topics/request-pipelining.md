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
[`//examples/fidl/llcpp/request_pipelining`][src].

### The FIDL protocol

<!-- TODO(fxbug.dev/58758) <<../../common/pipelining/launcher.md>> -->

This tutorial implements the `EchoLauncher` protocol from the
[fuchsia.examples library][examples-fidl]:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="launcher" %}
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
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/request_pipelining/server/main.cc" region_tag="echo-impl" %}
```

The `SendString` handler is empty as the client just uses `EchoString`.

### Implement the EchoLauncher protocol

This class responds to either method by launching an instance of an `Echo`
server, and then stores the `EchoImpl` instance in a member variable to ensure
that its lifetime matches that of the launcher. The code for running an `Echo`
server given a specific prefix and channel is abstracted into a helper
`RunEchoServer` method:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/request_pipelining/server/main.cc" region_tag="launcher-impl" %}
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
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/request_pipelining/server/main.cc" region_tag="main" %}
```

## Build the server

Optionally, to check that things are correct, try building the server:

1. Configure your GN build to include the server:

   ```posix-terminal
   fx set core.x64 --with //examples/fidl/llcpp/request_pipelining/server:echo-server
   ```
2. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

## Implement the client

Note: Most of the client code in `client/main.cc` should be familiar after
having followed the [client tutorial][client-tut]. The different parts of the
code are covered in more detail here.

After connecting to the `EchoLauncher` server, the client
code connects to one instance of `Echo` using `GetEcho` and another using
`GetEchoPipelined` and then makes an `EchoString` request on each instance.

### Non-pipelined client

This is the non-pipelined code:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/request_pipelining/client/main.cc" region_tag="main" highlight="11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33" %}
```

This code has two layers of callbacks:

* The outer layer handles the launcher `GetEcho` response.
* The inner layer handles the `EchoString` response.

Inside the `GetEcho` response callback, the code binds the returned client end
to a `fidl::WireSharedClient<Echo>`, and places a clone into the `EchoString`
callback, so that the client's lifetime is extended until when the echo response
is received, which will most likely be after the top level callback returns.

### Pipelined client

Despite having to create a pair of endpoints first, the pipelined code is much
simpler:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/request_pipelining/client/main.cc" region_tag="main" highlight="35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51" %}
```

Unlike in the [client tutorial][client-tut], the async loop is run to completion
once, which runs both non-pipelined and pipelined code concurrently in order to
observe the order of operations. The client keeps track of the number of
responses being received, so that it can quit the loop once no more messages
from the server are expected.

## Build the client

Optionally, to check that things are correct, try building the client:

1. Configure your GN build to include the server:

   ```posix-terminal
   fx set core.x64 --with //examples/fidl/llcpp/request_pipelining/client:echo-client
   ```

2. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

## Run the example code

For this tutorial, a [realm][glossary.realm] component is
provided to declare the appropriate capabilities and routes for
`fuchsia.examples.Echo` and `fuchsia.examples.EchoLauncher`.

Note: You can explore the full source for the realm component at
[`//examples/fidl/echo-realm`](/examples/fidl/echo-realm)

1. Configure your build to include the provided package that includes the
   echo realm, server, and client:

    ```posix-terminal
    fx set core.qemu-x64 --with //examples/fidl/llcpp:echo-launcher-llcpp
    ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

    ```posix-terminal
    ffx component run fuchsia-pkg://fuchsia.com/echo-launcher-llcpp#meta/echo_realm.cm
    ```

1. Start the `echo_client` instance:

    ```posix-terminal
    ffx component start /core/ffx-laboratory:echo_realm/echo_client
    ```

The server component starts when the client attempts to connect to the
`EchoLauncher` protocol. You should see output similar to the following
in the device logs (`ffx log`):

```none {:.devsite-disable-click-to-copy}`
[echo_server][][I] Running echo launcher server
[echo_server][][I] echo_server_llcpp: Incoming connection for fuchsia.examples.EchoLauncher
[echo_server][][I] Got non pipelined request
[echo_server][][I] Got pipelined request
[echo_server][][I] Got echo request for prefix pipelined:
[echo_client][][I] Got echo response pipelined: hello!
[echo_server][][I] Got echo request for prefix non pipelined:
[echo_client][][I] Got echo response non pipelined: hello!
```

Based on the print order, you can see that the pipelined case is faster. The
echo response for the pipelined case arrives first, even though the non
pipelined request is sent first, since request pipelining saves a roundtrip
between the client and server. Request pipelining also simplifies the code.

For further reading about protocol request pipelining, including how to handle
protocol requests that may fail, see the [FIDL API rubric][rubric].

Terminate the realm component to stop execution and clean up the component
instances:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo_realm
```

<!-- xrefs -->
[glossary.realm]: /glossary/README.md#realm
[src]: /examples/fidl/llcpp/request_pipelining
[server-tut]: /development/languages/fidl/tutorials/llcpp/basics/server.md
[server-tut-main]: /development/languages/fidl/tutorials/llcpp/basics/server.md#main
[client-tut]: /development/languages/fidl/tutorials/llcpp/basics/client.md
[rubric]: /development/api/fidl.md#request-pipelining
[overview]: /development/languages/fidl/tutorials/llcpp/README.md
[examples-fidl]: /examples/fidl/fuchsia.examples/
