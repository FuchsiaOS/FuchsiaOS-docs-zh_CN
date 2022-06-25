# Implement a sync LLCPP FIDL client

<!-- <<../../common/client/overview.md>> -->

## Prerequisites

This tutorial builds on the [FIDL server][server-tut] tutorial. For the
full set of FIDL tutorials, refer to the [overview][overview].

## Overview

This tutorial implements a client for a FIDL protocol and runs it
against the server created in the [previous tutorial][server-tut]. The client in
this tutorial is synchronous. There is an [alternate tutorial][async-client] for
asynchronous clients.

If you want to write the code yourself, delete the following directories:

```posix-terminal
rm -r examples/fidl/llcpp/client_sync/*
```

## Create the component

Create a new component project at `examples/fidl/llcpp/client_sync`:

1. Add a `main()` function to `examples/fidl/llcpp/client_sync/main.cc`:

   ```cpp
   int main(int argc, const char** argv) {
     std::cout << "Hello, world!" << std::endl;
   }
   ```

1. Declare a target for the client in `examples/fidl/llcpp/client_sync/BUILD.gn`:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/BUILD.gn" region_tag="imports" %}

   # Declare an executable for the client.
   executable("bin") {
     output_name = "fidl_echo_llcpp_client_sync"
     sources = [ "main.cc" ]
   }

   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/BUILD.gn" region_tag="rest" %}
   ```

1. Add a component manifest in `examples/fidl/llcpp/client_sync/meta/client.cml`:

   Note: The binary name in the manifest must match the output name of the
   `executable` defined in the previous step.

   ```json5
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/meta/client.cml" region_tag="example_snippet" %}
   ```

1. Once you have created your component, ensure that you can add it to the
   build configuration:

   ```posix-terminal
   fx set core.qemu-x64 --with //examples/fidl/llcpp/client_sync:echo-client
   ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

## Edit GN dependencies

1. Add the following dependencies:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/BUILD.gn" region_tag="deps" %}
   ```

1. Then, include them in `main.cc`:

   ```cpp
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/main.cc" region_tag="includes" %}
   ```

These dependencies are explained in the [server tutorial][server-tut]. The
client requires far fewer dependencies because it does not need to run any
asynchronous code.

## Connect to the server {#main}

The steps in this section explain how to add code to the `main()` function
that connects the client to the server and makes requests to it.

### Connect to the server

The client then connects to the service directory `/svc`, and uses it to connect
to the server.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/main.cc" region_tag="main" highlight="2,3,4,5,6,8,9,10" %}
```

The `service::OpenServiceRoot` function initializes a channel, then passes the
server end to `fdio_service_connect` to connect to the `/svc` directory,
returning the client end wrapped in a `zx::status` result type. We should check
for the `is_ok()` value on the result to determine if any synchronous error
occurred.

Connecting to a protocol relative to the service directory is done by calling
`fdio_service_connect_at`, passing it the service directory, the name of the
service to connect to, as well as the channel that should get passed to the
server. The `service::ConnectAt` function wraps the low level `fdio` call,
providing the user with a typed client channel endpoint to the requested
protocol.

In parallel, the component manager will route the requested service name and
channel to the server component, where the [`connect` function][server-handler]
implemented in the server tutorial is called with these arguments, binding the
channel to the server implementation.

An important point to note here is that this code assumes that `/svc` already
contains an instance of the `Echo` protocol. This is not the case by default
because of the sandboxing provided by the component framework. A workaround will
be when [running the example](#run) at the end of the tutorial.

Note: This pattern of making a request to connect the server end of the channel
to a service, then immediately using the client end to communicate with the
service is known as request pipelining. This topic is covered further in a
separate [tutorial][pipelining-tut].

### Send requests to the server

The code makes two requests to the server:

* An `EchoString` request
* A `SendString` request

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/main.cc" region_tag="main" highlight="12,13,16,17,18,19,20,24,25,26,27" %}
```

The protocol methods on the client object (`EchoString` and `SendString`) return
a [result object][resultof], which will contain either an error or the contents
of the response (if any). When a response is expected, the client will block
until the response is received.

A client object is generated for each protocol, which is described further in
the [LLCPP bindings reference][sync-client].

### Handle events

The client object allows handling events by specifying an
[event delegate][event-handlers], where each method corresponds to one of the
events of the protocol, plus a `Unknown` handler for when an unknown event
is received.

The code defines a handler, which prints the contents of an `OnString` event,
then calls `client.HandleOneEvent()` to block until an event is received. If a
recognized event was received and successfully decoded, `HandleOneEvent` returns
`fidl::Status::Ok()`. Otherwise, it returns an appropriate error:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/main.cc" region_tag="main" highlight="29,30,31,32,33,34,35,36,37,38,39,41,42,43,44" %}
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
    fx set core.qemu-x64 --with //examples/fidl/llcpp:echo-llcpp-client-sync
    ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

    ```posix-terminal
    ffx component run fuchsia-pkg://fuchsia.com/echo-llcpp-client-sync#meta/echo_realm.cm
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
[echo_server][][I] Incoming connection for fuchsia.examples.Echo
[echo_client][][I] Got response: hello
[echo_client][][I] Got event: hi
```

Terminate the realm component to stop execution and clean up the component
instances:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo_realm
```

<!-- xrefs -->
[glossary.realm]: glossary/README.md#realm
[server-tut]: development/languages/fidl/tutorials/llcpp/basics/server.md
[server-handler]: development/languages/fidl/tutorials/llcpp/basics/server.md#server-handler
[async-client]: development/languages/fidl/tutorials/llcpp/basics/client.md
[overview]: development/languages/fidl/tutorials/overview.md
[environment]: concepts/components/v2/environments.md
[pipelining-tut]: development/languages/fidl/tutorials/llcpp/topics/request-pipelining.md
[resultof]: reference/fidl/bindings/llcpp-bindings.md#resultof
[sync-client]: reference/fidl/bindings/llcpp-bindings.md#sync-client
[event-handlers]: reference/fidl/bindings/llcpp-bindings.md#events
