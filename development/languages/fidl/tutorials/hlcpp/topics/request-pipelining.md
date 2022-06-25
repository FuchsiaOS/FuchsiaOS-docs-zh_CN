# Passing FIDL protocols

## Prerequisites

This tutorial builds on the [HLCPP getting started tutorials][overview].

## Overview

A common aspect of using FIDL on Fuchsia is passing protocols themselves across
protocols. More precisely, many messages include either the client end or
the server end of a channel, where the channel is used to communicate over a
specific protocol. In this case, client end means that the remote end of the
channel implements the specified protocol, whereas server end means that the
remote end is making requests for the specified protocol. An alternate set of
terms for client end and server end are protocol and protocol request.

This tutorial covers:

* The usage of these client and server ends, both in FIDL and in the HLCPP
  FIDL bindings.
* The request pipelining pattern and its benefits.

The full example code for this tutorial is located at
[`//examples/fidl/hlcpp/request_pipelining`][src].

### The FIDL protocol

To do so, this tutorial implements the `EchoLauncher` protocol from the
[fuchsia.examples library][examples-fidl]:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="launcher" %}
```

This is a protocol that lets clients retrieve an instance of the `Echo`
protocol. Clients can specify a prefix, and the resulting `Echo` instance
adds that prefix to every response.

There are two methods that can be used to accomplish this:

* `GetEcho`: Takes the prefix as a request, and responds with the *client end* of
  a channel connected to an implementation of the `Echo` protocol. After
  receiving the client end in the response, the client can start making requests
  on the `Echo` protocol using the client end.
* `GetEchoPipelined`: Takes the *server end* of a channel as one of the request
  parameters and binds an implementation of `Echo` to it. The client that
  made the request is assumed to already hold the client end, and will
  start making `Echo` requests on that channel after calling `GetEchoPipeliend`.

As the name suggests, the latter uses a pattern called protocol request
pipelining, and is the preferred approach. This tutorial implements both
approaches.

## Implement the server

### Implement the Echo protocol

This implementation of `Echo` allows specifying a prefix in order to
distinguish between the different instances of `Echo` servers:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/request_pipelining/server/main.cc" region_tag="echo-impl" %}
```

The `SendString` handler is empty as the client just uses `EchoString`.

### Implement the EchoLauncher protocol

This class uses
a binding set to keep track of all of the instances of `Echo` that it launches:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/request_pipelining/server/main.cc" region_tag="launcher-impl" highlight="1,17,18" %}
```

The code explicitly specifies not just the protocol that the binding set is
templated on, but also the pointer type of the bindings that it stores. The code uses
`unique_ptr` instead of raw pointers so that the binding set owns the
instances of `EchoImpl`.

This is the implentation of the two methods:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/request_pipelining/server/main.cc" region_tag="launcher-impl" highlight="3,4,5,6,7,8,9,11,12,13,14,15" %}
```

For `GetEcho`, the code first needs to instantiate both ends of the
channel. It creates a `Binding` using the server end, and then sends a response
back with the client end. For `GetEchoPipelined`, the client has already done
the work of creating both ends of the channel. It keeps one end and has passed
the other to the server, so all the code needs to do is bind it to an `Echo`
implementation.

### Serve the EchoLauncher protocol

The main loop is the same as in the
[server tutorial][server-tut-main] but serves an `EchoLauncher` instead of `Echo`.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/request_pipelining/server/main.cc" region_tag="main" %}
```

## Build the server

Optionally, o check that things are correct, try building the server:

1. Configure your GN build to include the server:

   ```posix-terminal
   fx set core.x64 --with //examples/fidl/hlcpp/request_pipelining/server:echo-server
   ```
2. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

## Implement the client

Note: Most of the client code in `client/main.cc` should be familiar after having
followed the [client tutorial][client-tut]. The different parts of the code
are covered in more detail here.

After connecting to the `EchoLauncher` server, the client
code connects to one instance of `Echo` using `GetEcho` and another using
`GetEchoPipelined` and then makes an `EchoString` request on each instance.

This is the non-pipelined code:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/request_pipelining/client/main.cc" region_tag="non-pipelined" %}
```

This code has two layers of callbacks:

* The outer layer handles the launcher request.
* The inner layer handles the `EchoString` request.

Also, the code instantiates the `EchoPtr` in the outer scope then `Bind`s it
inside of the callback instead of calling `fidl::InterfaceRequest<T>::Bind`.
This is because the proxy needs to be in scope when the echo response is
received, which will most likely be after the top level callback returns.

Despite having to initialize the channels, the pipelined code is
much simpler:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/request_pipelining/client/main.cc" region_tag="pipelined" %}
```

## Build the client

Optionally, to check that things are correct, try building the client:

1. Configure your GN build to include the client:

   ```posix-terminal
   fx set core.x64 --with //examples/fidl/hlcpp/request_pipelining/client:echo-client
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
    fx set core.qemu-x64 --with //examples/fidl/hlcpp:echo-launcher-hlcpp
    ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

    ```posix-terminal
    ffx component run fuchsia-pkg://fuchsia.com/echo-launcher-hlcpp#meta/echo_realm.cm
    ```

1. Start the `echo_client` instance:

    ```posix-terminal
    ffx component start /core/ffx-laboratory:echo_realm/echo_client
    ```

The server component starts when the client attempts to connect to the
`EchoLauncher` protocol. You should see output similar to the following
in the device logs (`ffx log`):

```none {:.devsite-disable-click-to-copy}
[echo_server][][I] Running echo launcher server
[echo_server][][I] Got non pipelined request
[echo_server][][I] Got pipelined request
[echo_server][][I] Got echo request for prefix pipelined:
[echo_client][][I] Got non pipelined response
[echo_client][][I] Got echo response pipelined: hello!
[echo_server][][I] Got echo request for prefix not pipelined:
[echo_client][][I] Got echo response not pipelined: hello!
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
[glossary.realm]: glossary/README.md#realm
[src]: /examples/fidl/hlcpp/request_pipelining
[server-tut]: development/languages/fidl/tutorials/hlcpp/basics/server.md
[server-tut-main]: development/languages/fidl/tutorials/hlcpp/basics/server.md#main
[client-tut]: development/languages/fidl/tutorials/hlcpp/basics/client.md
[rubric]: development/api/fidl.md#request-pipelining
[overview]: development/languages/fidl/tutorials/hlcpp/README.md
[examples-fidl]: /examples/fidl/fuchsia.examples/
