# Request pipelining in Rust

## Prerequisites

In this tutorial, you'll learn about the request pipelining pattern and its
benefits. This tutorial expects you to already be familiar with the basics
of writing and running FIDL clients and servers, which is covered in the
[Rust getting started tutorials][overview].

## Overview

<!-- TODO(fxbug.dev/58758): <<../../common/pipelining/overview.md>> -->

A common aspect of using FIDL on Fuchsia is passing protocols themselves across
protocols. Many FIDL messages include either the client end or
the server end of a channel, where the channel is used to communicate over a
different FIDL protocol. In this case, client end means that the remote end of the
channel implements the specified protocol, whereas server end means that the
remote end is making requests for the specified protocol. An alternate set of
terms for client end and server end are protocol and protocol request.

This tutorial covers:

* The usage of these client and server ends, both in FIDL and in the Rust
  FIDL bindings.
* The request pipelining pattern and its benefits.

The full example code for this tutorial is located at
[//examples/fidl/rust/request_pipelining][src].

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

* `GetEcho`: Takes the prefix as a request, and responds with the *client end* of
  a channel connected to an implementation of the `Echo` protocol. After
  receiving the client end in the response, the client can start making requests
  on the `Echo` protocol using the client end.
* `GetEchoPipelined`: Takes the prefix and the *server end* of a channel as a request,
  and binds an implementation of `Echo` to it. The client that
  made the request is assumed to already hold the client end, and will
  start making `Echo` requests on that channel after calling `GetEchoPipeliend`.

As the name suggests, the latter uses a pattern called protocol request
pipelining, and is the preferred approach. This tutorial implements both
approaches.

## Implement the server

### Implement the Echo protocol

This implementation of `Echo` allows specifying a prefix in order to
distinguish between the different instances of `Echo` servers:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/request_pipelining/server/src/main.rs" region_tag="echo-impl" %}
```

The `SendString` handler is empty as the client just uses `EchoString`.

### Implement the EchoLauncher protocol

The general structure is similar to the `Echo` implementation, but one difference is that the
`try_for_each_concurrent` is used instead of `try_for_each`. The client in this example launches
two instances of `Echo`, so, using the concurrent version allows the two calls to
`run_echo_server` to be run concurrently:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/request_pipelining/server/src/main.rs" region_tag="launcher-impl" highlight="7,31" %}
```

Both of the `EchoLauncher` methods are handled by calling `run_echo_server` on the server end of
the channel. The difference is that in `GetEcho`, the server is responsible for initializing the
channel - it uses one end as the server end and sends the other end back to the client. In
`GetEchoPipelined`, the server end is provided as part of the request, so no additional work needs
to be done by the server, and no response is necessary.

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/request_pipelining/server/src/main.rs" region_tag="launcher-impl" highlight="8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30" %}
```

### Serve the EchoLauncher protocol

The main loop should is the same as in the
[server tutorial][server-tut-main] but serves an `EchoLauncher` instead of `Echo`.

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/request_pipelining/server/src/main.rs" region_tag="main" %}
```

## Build the server

Optionally, to check that things are correct, try building the server:

1. Configure your GN build to include the server:

   ```
   fx set core.x64 --with //examples/fidl/rust/request_pipelining/server
   ```
2. Build the Fuchsia image:

   ```
   fx build
   ```

## Implement the client

Note: Most of the client code in `client/src/main.rs` should be familiar after having
followed the [client tutorial][client-tut]. The different parts of the code
are covered in more detail here.

After connecting to the `EchoLauncher` server, the client
code connects to one instance of `Echo` using `GetEcho` and another using
`GetEchoPipelined` and then makes an `EchoString` request on each instance.

This is the non-pipelined code:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/request_pipelining/client/src/main.rs" region_tag="main" highlight="6,7,8,9,10,11,12,13,14" %}
```

This code chains together two futures. First, it makes the `GetEcho` request to the client. It then
takes the result of that future, and then uses it to create a client object (the `proxy`), calls
`EchoString`, and then blocks on the result using `await`.

Despite having to initialize the channel first, the pipelined code is much simpler:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/request_pipelining/client/src/main.rs" region_tag="main" highlight="16,17,18,19,20,21,22" %}
```

`create_proxy` is used, which is a shortcut for creating the two ends of a channel and converting
one end into a proxy. After the call to `GetEchoPipelined`, the client can immediately make the
`EchoString` request.

Finally, the two futures corresponding to the non-pipelined and pipelined calls are run to
completion concurrently, to see which one completes first:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/request_pipelining/client/src/main.rs" region_tag="main" highlight="24,25,26,27,28" %}
```

## Build the client

Optionally, to check that things are correct, try building the client:

1. Configure your GN build to include the server:

   ```
   fx set core.x64 --with //examples/fidl/rust/request_pipelining/client`
   ```

2. Build the Fuchsia image:

   ```
   fx build
   ```

## Run the example code

To run the example code:

1. Configure your GN build as follows:

   ```
   fx set core.x64 --with //examples/fidl/rust/request_pipelining/client --with //examples/fidl/rust/request_pipelining/server --with //examples/fidl/test:echo-launcher
   ```

2. Run the example:

   ```
   fx shell run fuchsia-pkg://fuchsia.com/echo-launcher#meta/launcher.cmx fuchsia-pkg://fuchsia.com/echo-launcher-rust-client#meta/echo-client.cmx fuchsia-pkg://fuchsia.com/echo-launcher-rust-server#meta/echo-server.cmx fuchsia.examples.EchoLauncher
   ```

You should see the following print output in the QEMU console (or using `fx log`):

```
[115871.934] 535502:535504> Running echo launcher server
[115871.940] 535502:535504> Got non pipelined request
[115871.942] 535502:535504> Got pipelined request
[115871.942] 535502:535504> Got echo request for prefix pipelined:
[115871.942] 535502:535504> Got echo request for prefix not pipelined:
[115871.943] 535282:535284> Got echo response pipelined: : hello
[115871.943] 535282:535284> Got echo response not pipelined: : hello`
```

Based on the print order, you can see that the pipelined case is faster. The
echo response for the pipelined case arrives first, even though the non
pipelined request is sent first, since request pipelining saves a roundtrip
between the client and server. Request pipelining also simplifies the code.


For further reading about protocol request pipelining, including how to handle
protocol requests that may fail, see the [FIDL API rubric][rubric].

<!-- xrefs -->
[src]: /examples/fidl/rust/request_pipelining
[server-tut]: /docs/development/languages/fidl/tutorials/rust/basics/server.md
[server-tut-main]: /docs/development/languages/fidl/tutorials/rust/basics/server.md#main
[client-tut]: /docs/development/languages/fidl/tutorials/rust/basics/client.md
[rubric]: /docs/concepts/api/fidl.md#request-pipelining
[overview]: /docs/development/languages/fidl/tutorials/rust/README.md
[examples-fidl]: /examples/fidl/fuchsia.examples/
