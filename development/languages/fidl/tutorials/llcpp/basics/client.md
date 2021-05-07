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

```
rm -r examples/fidl/llcpp/client/*
```

## Create a stub component

1. Set up a hello world component in `examples/fidl/llcpp/client`.
   You can name the component `echo-client`, and give the package a name of
   `echo-llcpp-client`.

   Note: If necessary, refer back to the [previous tutorial][server-tut].

1. Once you have created your component, ensure that the following works:

   ```
   fx set core.x64 --with //examples/fidl/llcpp/client
   ```

1. Build the Fuchsia image:

   ```
   fx build
   ```

1. In a separate terminal, run:

   ```
   fx serve
   ```

1. In a separate terminal, run:

   ```
   fx shell run fuchsia-pkg://fuchsia.com/echo-llcpp-client#meta/echo-client.cmx
   ```

## Edit GN dependencies

1. Add the following dependencies:

   ```gn
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/BUILD.gn" region_tag="deps" %}
   ```

1. Then, include them in `main.cc`:

   ```cpp
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/main.cc" region_tag="includes" %}

These dependencies are explained in the  [server tutorial][server-tut].

## Edit component manifest

1. Include the `Echo` protocol in the client component's sandbox by
   editing the component manifest in `client.cmx`.

   ```cmx
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/client.cmx" %}
   ```

## Connect to the server {#main}

The steps in this section explain how to add code to the `main()` function
that connects the client to the server and makes requests to it.

### Initialize the event loop

As in the server, the code first sets up an async loop so that the client can
listen for incoming responses from the server without blocking.

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/main.cc" region_tag="main" highlight="2,3,20,25,29,40,43,44,59" %}
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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/main.cc" region_tag="main" highlight="5,6,7,8,9,11,12,13,14,15" %}
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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/main.cc" region_tag="main" highlight="17,18,22" %}
```

The event handler delegate should be an object that implements the
`Echo::AsyncEventHandler` virtual interface, which has methods corresponding to
the events offered by the protocol (see [LLCPP event handlers][event-handlers]).
In this case, a local class is defined with a single method corresponding to the
handler for the `OnString` event. The handler prints the string and quits the
event loop.

### Send requests to the server

The code makes three requests to the server:

* An asynchronous `EchoString` request.
* A synchronous `EchoString` request.
* A `SendString` request (async vs sync is not relevant for this case because it
  is a fire and forget method).

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client/main.cc" region_tag="main" highlight="35,36,37,38,39,40,41,42,46,47,48,49,50,51,54,55,56,57,58" %}
```

The client object works by overriding the dereference operator to return a
[protocol specific client implementation][client-impl], allowing calls such as
`client->EchoString()`.

The asynchronous method call requires the request parameters followed by a
response handler callback, which is called when the response is received.

The client object also allows synchronous calls, which will block until the
response is received and return the response object. These are suffixed with
`_Sync` (e.g. `client->EchoString_Sync()`).

In the synchronous case, a [result object][resultof] is returned, since the
method call can fail. In the asynchronous or fire-and-forget case, a lightweight
status object is returned, which communicates any synchronous errors. The
response callback takes the response message pointer as argument directly, since
the handler is only called in the case of a successful method call.

## Run the client {#run}

If you run the client directly, it will not connect to the server correctly
because the client does not automatically get the `Echo` protocol provided in
its sandbox (in `/svc`). To get this to work, a launcher tool is provided
that launches the server, creates a new [`Environment`][environment] for
the client that provides the server's protocol, then launches the client in it.

1. Configure your GN build as follows:

    ```
    fx set core.x64 --with //examples/fidl/llcpp/server --with //examples/fidl/llcpp/client --with //examples/fidl/test:echo-launcher
    ```

2. Build the Fuchsia image:

   ```
   fx build
   ```

3. Run the launcher by passing it the client URL, the server URL, and
   the protocol that the server provides to the client:

    ```
    fx shell run fuchsia-pkg://fuchsia.com/echo-launcher#meta/launcher.cmx fuchsia-pkg://fuchsia.com/echo-llcpp-client#meta/echo-client.cmx fuchsia-pkg://fuchsia.com/echo-llcpp-server#meta/echo-server.cmx fuchsia.examples.Echo
    ```

You should see the print output in the QEMU console (or using `fx log`).

```
[166633.167] 757796:757798> Running echo server
[166633.489] 757796:757798> echo_server_llcpp: Incoming connection for fuchsia.examples.Echo
[166633.528] 758101:758103> Got synchronous response: hello
[166633.531] 758101:758103> Got response: hello
[166633.531] 758101:758103> Got event: hi"
```

<!-- xrefs -->
[bindings-ref]: /docs/reference/fidl/bindings/llcpp-bindings.md
[event-handlers]: /docs/reference/fidl/bindings/llcpp-bindings.md#events
[resultof]: /docs/reference/fidl/bindings/llcpp-bindings.md#resultof
[client-impl]: /docs/reference/fidl/bindings/llcpp-bindings.md#async-client
[server-handler]: /docs/development/languages/fidl/tutorials/llcpp/basics/server.md#server-handler
[server-tut]: /docs/development/languages/fidl/tutorials/llcpp/basics/server.md
[sync-client]: /docs/development/languages/fidl/tutorials/llcpp/basics/sync-client.md
[overview]: /docs/development/languages/fidl/tutorials/overview.md
[environment]: /docs/concepts/components/v2/environments.md
[pipelining-tut]: /docs/development/languages/fidl/tutorials/llcpp/topics/request-pipelining.md
