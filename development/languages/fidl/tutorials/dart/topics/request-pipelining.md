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

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/server/lib/main.dart" region_tag="echo-impl" %}
```

The `SendString` handler is empty as the client just uses `EchoString`.

Additionally, the class holds an `EchoBinding` property to simplify the process of binding the
server to a channel.

### Implement the EchoLauncher protocol

This class uses stores a list of all of the instances of `Echo` that it launches:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/server/lib/main.dart" region_tag="launcher-impl" %}
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
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/server/lib/main.dart" region_tag="main" %}
```

## Build the server

Optionally, to check that things are correct, try building the server:

1. Configure your GN build to include the server:

   ```posix-terminal
   fx set core.x64 --with //examples/fidl/dart/request_pipelining/server:echo-server
   ```

2. Build the Fuchsia image:

   ```posix-terminal
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
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/client/lib/main.dart" region_tag="main" highlight="9,10,11,12,13,14,15,16" %}
```

This code chains together two futures. First, it makes the `GetEcho` request to the client. It then
takes the result of that future (a channel), and binds it the non pipelined client object, calls
`EchoString` on it, and then blocks on the result using `await`.

Note: You should prefer just using `await` on a future instead of chaining them together
using combinators like `then`. In this case, this is necessary to demonstrate the order in which
the futures get completed.

The pipelined code is much simpler:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/client/lib/main.dart" region_tag="main" highlight="18,19,20,21,22,23,24,25,26" %}
```

The call to `pipelinedEcho.ctrl.request()` creates a channel, binds the client object to one end,
then returns the other. The return value in this case gets passed to the call to `GetEchoPipelined`.
After the call to `GetEchoPipelined`, the client can immediately make the `EchoString` request.

Finally, the two futures corresponding to the non-pipelined and pipelined calls are run to
completion concurrently, to see which one completes first:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/request_pipelining/client/lib/main.dart" region_tag="main" highlight="28,29" %}
```

## Build the client

Optionally, to check that things are correct, try building the client:

1. Configure your GN build to include the client:

   ```posix-terminal
   fx set core.x64 --with //examples/fidl/dart/request_pipelining/client:echo-client
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
   fx set core.x64 --with examples/fidl/dart:echo-launcher-dart --with-base //src/dart \
     --args='core_realm_shards += [ "//src/dart:dart_runner_core_shard" ]'
   ```

   NOTE: The flag `--with-base //src/dart` adds the required dart runner to the
   base packages; and the `core_realm_shards` argument updates the
   `laboratory-env` component environment (the environment provided to the
   `ffx-laboratory` realm, used in `ffx component start`) to include the
   required dart runner.

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Start or restart your device and package server (`fx serve` or
   `fx serve-updates`) to ensure the Dart runner package can be served.

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

   ```posix-terminal
   ffx component run /core/ffx-laboratory:echo_realm fuchsia-pkg://fuchsia.com/echo-launcher-dart#meta/echo_realm.cm
    ```

1. Start the `echo_client` instance:

    ```posix-terminal
   ffx component start /core/ffx-laboratory:echo_realm/echo_client
   ```

The server component starts when the client attempts to connect to the
`EchoLauncher` protocol. You should see output similar to the following
in the device logs (`ffx log`):


```
[echo-launcher-server][][I] Running EchoLauncher server
[echo-launcher-server][][I] Got echo response pipelined: hello
[echo-launcher-server][][I] Got echo response not pipelined: hello
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
[src]: /examples/fidl/dart/request_pipelining
[server-tut]: /development/languages/fidl/tutorials/dart/basics/server.md
[server-tut-main]: /development/languages/fidl/tutorials/dart/basics/server.md#main
[client-tut]: /development/languages/fidl/tutorials/dart/basics/client.md
[rubric]: /development/api/fidl.md#request-pipelining
[overview]: /development/languages/fidl/tutorials/dart/README.md
[examples-fidl]: /examples/fidl/fuchsia.examples/
