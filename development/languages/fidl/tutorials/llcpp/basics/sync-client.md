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

```
rm -r examples/fidl/llcpp/client_sync/*
```

## Create a stub component

1. Set up a hello world component in `examples/fidl/llcpp/client_sync`.
   You can name the component `echo-client`, and give the package a name of
   `echo-llcpp-client-sync`.

   Note: If necessary, refer back to the [previous tutorial][server-tut].

1. Once you have created your component, ensure that the following works:

   ```
   fx set core.x64 --with //examples/fidl/llcpp/client_sync
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
   fx shell run fuchsia-pkg://fuchsia.com/echo-llcpp-client-sync#meta/echo-client.cmx
   ```

## Edit GN dependencies

1. Add the following dependencies:

   ```gn
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/BUILD.gn" region_tag="deps" %}
   ```

1. Then, include them in `main.cc`:

   ```cpp
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/main.cc" region_tag="includes" %}
   ```

These dependencies are explained in the [server tutorial][server-tut]. The
client requires far fewer dependencies because it does not need to run any
asynchronous code.

## Edit component manifest

1. Include the `Echo` protocol in the client component's sandbox by
   editing the component manifest in `client.cmx`.

   ```cmx
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/client.cmx" %}
   ```

## Connect to the server {#main}

The steps in this section explain how to add code to the `main()` function
that connects the client to the server and makes requests to it.

### Connect to the server

The client then connects to the service directory `/svc`, and uses it to connect
to the server.

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/main.cc" region_tag="main" highlight="2,3,4,5,6,8,9,10" %}
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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/main.cc" region_tag="main" highlight="12,13,16,17,18,19,20,24,25,26,27" %}
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
then calls `client.HandleOneEvent()` to block until an event is received. If an
unknown event is received, its return value becomes the return value of the
`HandleOneEvent` call:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_sync/main.cc" region_tag="main" highlight="29,30,31,32,33,34,35,36,37,38,39,41,42,43,44" %}
```

## Run the client

If you run the client directly, it will not connect to the server correctly
because the client does not automatically get the `Echo` protocol provided in
its sandbox (in `/svc`). To get this to work, a launcher tool is provided
that launches the server, creates a new [`Environment`][environment] for
the client that provides the server's protocol, then launches the client in it.

1. Configure your GN build:

   ```
   fx set core.x64 --with //examples/fidl/llcpp/server --with
   //examples/fidl/client/client_sync --with //examples/fidl/test:echo-launcher
   ```

2. Build the Fuchsia image:

   ```
   fx build
   ```

3. Run the launcher by passing it the client URL, the server URL, and
   the protocol that the server provides to the client:

   ```
   fx shell run fuchsia-pkg://fuchsia.com/echo-launcher#meta/launcher.cmx fuchsia-pkg://fuchsia.com/echo-llcpp-client-sync#meta/echo-client.cmx fuchsia-pkg://fuchsia.com/echo-llcpp-server#meta/echo-server.cmx fuchsia.examples.Echo
   ```

You should see the print output in the QEMU console (or using `fx log`).

```
[189209.659] 859216:859218> Running echo server
[189209.778] 859216:859218> echo_server_llcpp: Incoming connection for fuchsia.examples.Echo
[189209.803] 859554:859556> Got response: hello
[189209.804] 859554:859556> Got event: hi
```
<!-- xrefs -->
[server-tut]: /docs/development/languages/fidl/tutorials/llcpp/basics/server.md
[server-handler]: /docs/development/languages/fidl/tutorials/llcpp/basics/server.md#server-handler
[async-client]: /docs/development/languages/fidl/tutorials/llcpp/basics/client.md
[overview]: /docs/development/languages/fidl/tutorials/overview.md
[environment]: /docs/concepts/components/v2/environments.md
[pipelining-tut]: /docs/development/languages/fidl/tutorials/llcpp/topics/request-pipelining.md
[resultof]: /docs/reference/fidl/bindings/llcpp-bindings.md#resultof
[sync-client]: /docs/reference/fidl/bindings/llcpp-bindings.md#sync-client
[event-handlers]: /docs/reference/fidl/bindings/llcpp-bindings.md#events
