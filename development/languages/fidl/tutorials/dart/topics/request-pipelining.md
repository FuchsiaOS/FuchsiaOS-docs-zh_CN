# Protocol request pipelining in Dart

## Prerequisites

This tutorial builds on the [Dart getting started tutorials][overview].

## Overview

<!-- TODO(fxbug.dev/58758) <<../../common/pipelining/overview.md>> -->

A common aspect of using FIDL on Fuchsia is passing protocols themselves across
protocols. Many FIDL messages include either the client end or the server end of
a channel, where the channel is used to communicate over a different FIDL
protocol. In this case, client end means that the remote end of the channel
implements the specified protocol, whereas server end means that the remote end
is making requests for the specified protocol. An alternate set of terms for
client end and server end are protocol and protocol request.

This tutorial covers:

* The usage of these client and server ends, both in FIDL and in the Dart
  FIDL bindings.
* The request pipelining pattern and its benefits.

The full example code for this tutorial is located at
[//examples/fidl/dart/request_pipelining][src].

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

```dart
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/server/lib/main.dart" region_tag="echo-impl" %}
```

The `SendString` handler is empty as the client just uses `EchoString`.

Additionally, the class holds an `EchoBinding` property to simplify the process of binding the
server to a channel.

### Implement the EchoLauncher protocol

This class uses stores a list of all of the instances of `Echo` that it launches:

```dart
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/server/lib/main.dart" region_tag="launcher-impl" %}
```

Both of the `EchoLauncher` methods are handled by calling the `launchEchoServer` helper method on
the server end of the channel. The difference is that in `getEcho`, the server is responsible for
initializing the channel - it uses one end as the server end and sends the other end back to the
client. In `getEchoPipelined`, the server end is provided as part of the request, so no additional
work needs to be done by the server, and no response is necessary.

### Serve the EchoLauncher protocol

The main loop should is the same as in the
[server tutorial][server-tut-main] but serves an `EchoLauncher` instead of `Echo`.

```dart
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/server/lib/main.dart" region_tag="main" %}
```

## Build the server

Optionally, to check that things are correct, try building the server:

1. Configure your GN build to include the server:

   ```
   fx set core.x64 --with //examples/fidl/dart/request_pipelining/server
   ```
2. Build the Fuchsia image:

   ```
   fx build
   ```

## Implement the client

Note: Most of the client code in `client/lib/main.dart` should be familiar if you
followed the [client tutorial][client-tut]. The different parts of the code
are covered in more detail here.

After connecting to the `EchoLauncher` server, the client
code connects to one instance of `Echo` using `GetEcho` and another using
`GetEchoPipelined` and then makes an `EchoString` request on each instance.

This is the non-pipelined code:

```dart
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/client/lib/main.dart" region_tag="main" highlight="9,10,11,12,13,14,15,16" %}
```

This code chains together two futures. First, it makes the `GetEcho` request to the client. It then
takes the result of that future (a channel), and binds it the non pipelined client object, calls
`EchoString` on it, and then blocks on the result using `await`.

Note: You should prefer just using `await` on a future instead of chaining them together
using combinators like `then`. In this case, this is necessary to demonstrate the order in which
the futures get completed.

The pipelined code is much simpler:

```dart
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/client/lib/main.dart" region_tag="main" highlight="18,19,20,21,22,23,24,25,26" %}
```

The call to `pipelinedEcho.ctrl.request()` creates a channel, binds the client object to one end,
then returns the other. The return value in this case gets passed to the call to `GetEchoPipelined`.
After the call to `GetEchoPipelined`, the client can immediately make the `EchoString` request.

Finally, the two futures corresponding to the non-pipelined and pipelined calls are run to
completion concurrently, to see which one completes first:

```dart
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/client/lib/main.dart" region_tag="main" highlight="28,29" %}
```

## Build the client

Optionally, to check that things are correct, try building the client:

1. Configure your GN build to include the client:

   ```
   fx set core.x64 --with //examples/fidl/dart/request_pipelining/client
   ```
2. Build the Fuchsia image:

   ```
   fx build
   ```

## Run the example code

To run the example code:

1. Configure your GN build as follows:

   ```
   fx set core.x64 --with //examples/fidl/dart/request_pipelining/client --with //examples/fidl/dart/request_pipelining/server --with //examples/fidl/dart/launcher_bin
   ```

2. Run the example:

   ```
   fx shell run fuchsia-pkg://fuchsia.com/example-launcher-dart#meta/example-launcher-dart.cmx fuchsia-pkg://fuchsia.com/echo-launcher-dart-client#meta/echo-launcher-dart-client.cmx fuchsia-pkg://fuchsia.com/echo-launcher-dart-server#meta/echo-launcher-dart-server.cmx fuchsia.examples.EchoLauncher
   ```

You should see the following print output in the QEMU console (or using `fx log`):

```
[269547.480853][3][790426877][echo-launcher-server, main.dart(86)] INFO: Running EchoLauncher server
[269547.605037][3][1058778107][echo-launcher-client, main.dart(39)] INFO: Got echo response pipelined: hello
[269547.609355][3][1058778107][echo-launcher-client, main.dart(27)] INFO: Got echo response not pipelined: hello
```

Based on the print order, you can see that the pipelined case is faster. The
echo response for the pipelined case arrives first, even though the non
pipelined request is sent first, since request pipelining saves a roundtrip
between the client and server. Request pipelining also simplifies the code.

For further reading about protocol request pipelining, including how to handle
protocol requests that may fail, see the [FIDL API rubric][rubric].

<!-- xrefs -->
[src]: /examples/fidl/dart/request_pipelining
[server-tut]: /docs/development/languages/fidl/tutorials/dart/basics/server.md
[server-tut-main]: /docs/development/languages/fidl/tutorials/dart/basics/server.md#main
[client-tut]: /docs/development/languages/fidl/tutorials/dart/basics/client.md
[rubric]: /docs/concepts/api/fidl.md#request-pipelining
[overview]: /docs/development/languages/fidl/tutorials/dart/README.md
[examples-fidl]: /examples/fidl/fuchsia.examples/
