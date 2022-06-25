# Implement an LLCPP FIDL client

<!-- TODO(fxbug.dev/58758) <<../../common/client/overview.md>> -->

## Prerequisites

This tutorial builds on the [FIDL server][server-tut] tutorial. For the
full set of FIDL tutorials, refer to the [overview][overview].

## Overview

This tutorial implements a client for a FIDL protocol and runs it against the
server created in the [previous tutorial][server-tut]. The client in this
tutorial is asynchronous. There is an [alternate tutorial][sync-client] for
synchronous clients.

If you want to write the code yourself, delete the following directories:

```posix-terminal
rm -r examples/fidl/llcpp/client/*
```

## Create the component

Create a new component project at `examples/fidl/llcpp/client`:

1. Add a `main()` function to `examples/fidl/llcpp/client/main.cc`:

   ```cpp
   int main(int argc, const char** argv) {
     std::cout << "Hello, world!" << std::endl;
   }
   ```

1. Declare a target for the client in `examples/fidl/llcpp/client/BUILD.gn`:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/BUILD.gn" region_tag="imports" %}

   # Declare an executable for the client.
   executable("bin") {
     output_name = "fidl_echo_llcpp_client"
     sources = [ "main.cc" ]
   }

   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/BUILD.gn" region_tag="rest" %}
   ```

1. Add a component manifest in `examples/fidl/llcpp/client/meta/client.cml`:

   Note: The binary name in the manifest must match the output name of the
   `executable` defined in the previous step.

   ```json5
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/meta/client.cml" region_tag="example_snippet" %}
   ```

1. Once you have created your component, ensure that you can add it to the
   build configuration:

   ```posix-terminal
   fx set core.qemu-x64 --with //examples/fidl/llcpp/client:echo-client
   ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

## Edit GN dependencies

1. Add the following dependencies:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/BUILD.gn" region_tag="deps" %}
   ```

1. Then, include them in `main.cc`:

   ```cpp
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/main.cc" region_tag="includes" %}

These dependencies are explained in the [server tutorial][server-tut].

## Connect to the server {#main}

The steps in this section explain how to add code to the `main()` function
that connects the client to the server and makes requests to it.

### Initialize the event loop

As in the server, the code first sets up an async loop so that the client can
listen for incoming responses from the server without blocking.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/main.cc" region_tag="main" highlight="2,3,20,25,34,36,52,53,67" %}
```

The dispatcher is used to run two pieces of async code. It is first used to run
the `EchoString` method, and quits when the response is received. It is then run
after calling the `SendString` in order to listen for events, and quits when an
`OnString` event is received. The call to `ResetQuit()` in between these two
instances allows the client to reuse the loop.

### Connect to the server

The client then connects to the service directory `/svc`, and uses it to connect
to the server.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/main.cc" region_tag="main" highlight="5,6,7,8,9,11,12,13,14,15" %}
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

### Initialize the client {#proxy}

In order to make `Echo` requests to the server, initialize a client using the
client end of the channel from the previous step, the loop dispatcher, as well
as an event handler delegate:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/main.cc" region_tag="main" highlight="17,18,22,28,36,38,39" %}
```

The event handler delegate should be an object that implements the
`fidl::AsyncEventHandler<Echo>` virtual interface, which has methods
corresponding to the events offered by the protocol (see
[LLCPP event handlers][event-handlers]). In this case, a local class is defined
with a method corresponding to the `OnString` event. The handler prints the
string and quits the event loop. The class also overrides the `on_fidl_error`
method, which is called when the client encounters an error and is going to
teardown.

### Send requests to the server

The code makes four requests to the server:

* An asynchronous `EchoString` call taking a result callback.
* An asynchronous `EchoString` call taking a response callback.
* A synchronous `EchoString` call.
* A one way `SendString` request (async vs sync is not relevant for this case
  because it is a fire and forget method).

The client object works by overriding the dereference operator to return a
[protocol specific client implementation][client-impl], allowing calls such as
`client->EchoString()`.

#### Asynchronous call with result callback

The asynchronous method call requires the request parameters followed by
a *result callback*, which is called either when the method succeeds or an error
happens.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/main.cc" region_tag="main" highlight="41,42,43,44,45,46,47,48,49,50,51" %}
```

#### Synchronous call

The client object also allows synchronous calls, which will block until the
response is received and return the response object. These may be selected
using the `.sync()` accessor. (e.g. `client.sync()->EchoString()`).


```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/main.cc" region_tag="main" highlight="55,56,57" %}
```

In the synchronous case, a [result object][resultof] is returned, since the
method call can fail. In the asynchronous or fire-and-forget case, a lightweight
status object is returned, which communicates any synchronous errors.

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
    fx set core.qemu-x64 --with //examples/fidl/llcpp:echo-llcpp-client
    ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

    ```posix-terminal
    ffx component run fuchsia-pkg://fuchsia.com/echo-llcpp-client#meta/echo_realm.cm
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
[echo_client][][I] Got response (result callback): hello
[echo_client][][I] Got response (response callback): hello
[echo_client][][I] Got synchronous response: hello
[echo_client][][I] Got event: hi
```

Terminate the realm component to stop execution and clean up the component
instances:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo_realm
```

<!-- xrefs -->
[glossary.realm]: glossary/README.md#realm
[bindings-ref]: reference/fidl/bindings/llcpp-bindings.md
[event-handlers]: reference/fidl/bindings/llcpp-bindings.md#events
[resultof]: reference/fidl/bindings/llcpp-bindings.md#resultof
[client-impl]: reference/fidl/bindings/llcpp-bindings.md#async-client
[server-handler]: development/languages/fidl/tutorials/llcpp/basics/server.md#server-handler
[server-tut]: development/languages/fidl/tutorials/llcpp/basics/server.md
[sync-client]: development/languages/fidl/tutorials/llcpp/basics/sync-client.md
[overview]: development/languages/fidl/tutorials/overview.md
[environment]: concepts/components/v2/environments.md
[pipelining-tut]: development/languages/fidl/tutorials/llcpp/topics/request-pipelining.md
