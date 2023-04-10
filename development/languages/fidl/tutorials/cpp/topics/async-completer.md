# Responding to requests asynchronously

## Prerequisites

This tutorial builds on the [getting started tutorials][overview].

## Overview

The full example code for this tutorial is located at
[//examples/fidl/cpp/server_async_completer][src].

In the `Echo` implementation from the initial [server tutorial][server-tut], the
server code responded to `EchoString` requests using the completer:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server/main.cc" region_tag="impl-echo-string" adjust_indentation="auto" %}
```

Notice that the type of the completer ends with `::Sync`. Sync completers must
be used before the handler returns. Enforcing this allows optimizations since
the bookkeeping metadata for making a reply can be stack allocated.

## Respond asynchronously

In many cases responding synchronously is infeasible. For example, the reply may
need results from other asynchronous calls. To respond asynchronously, we must
obtain an async completer from the sync completer, using `ToAsync`:

```cpp
EchoStringCompleter::Async async_completer = completer.ToAsync();
```

The resulting `async_completer` exposes the same API as `completer`, but can be
moved away to be used at a later time.

In the example code, the server makes the reply inside a delayed task, emulating
a scenario where the server must execute a long running task before sending a
response:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/server_async_completer/main.cc" region_tag="impl-echo-string" adjust_indentation="auto" %}
```

The server won't start handling any new requests until returning from the
current handler method. After returning from `EchoString`, the server will
monitor the endpoint for new FIDL messages, while the reply is scheduled one
second into the future. This means if the client sent multiple `EchoString`
requests in quick succession, we may have just as many concurrent async delayed
tasks in flight.

Note: if the async completer is captured by a lambda function, the lambda must
be marked **mutable**, because making a reply using the completer mutates it
such that duplicate replies will panic.

### Respond asynchronously in servers speaking wire domain objects

You would use the same `ToAsync` operation when the server speaks
[wire domain objects][wire-types], but pay extra attention to object lifetimes.
In particular, the request views provided to method handlers do not own the
request message. If an asynchronous task needs to use the request parameters
after the `EchoString` method returns, we need to copy relevant fields to an
owned type:

```cpp
class EchoImpl : public fidl::WireServer<fuchsia_examples::Echo> {
 public:
  void EchoString(EchoStringRequestView request, EchoStringCompleter::Sync& completer) override {
    // Copy the contents of |request->value| (a fidl::StringView) to a string.
    std::string value_owned{request->value.get()};
    async::PostDelayedTask(
        dispatcher_,
        [value = value_owned, completer = completer.ToAsync()]() mutable {
          completer.Reply(fidl::StringView::FromExternal(value));
        },
        zx::duration(ZX_SEC(1)));
  }

  // ...
};
```

For further information on the memory ownership, refer to
[Memory ownership of wire domain objects][memory-ownership].

## Run the example

In order for the client and server to communicate using the `Echo` protocol,
component framework must route the `fuchsia.examples.Echo` capability from the
server to the client. For this tutorial, a [realm][glossary.realm] component is
provided to declare the appropriate capabilities and routes.

Note: You can explore the full source for the realm component at
[`//examples/fidl/echo-realm`](/examples/fidl/echo-realm)

1. Configure your build to include the provided package that includes the
   echo realm, server, and client:

    ```posix-terminal
    fx set core.qemu-x64 --with //examples/fidl/cpp/server_async_completer:echo-cpp-async
    ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

    ```posix-terminal
    ffx component run /core/ffx-laboratory:echo_realm fuchsia-pkg://fuchsia.com/echo-cpp-async#meta/echo_realm.cm
    ```

1. Start the `echo_client` instance:

    ```posix-terminal
    ffx component start /core/ffx-laboratory:echo_realm/echo_client
    ```

The server component starts when the client attempts to connect to the `Echo`
protocol. You should see output similar to the following in the device logs
(`ffx log`). The leftmost column is the timestamp:

```none {:.devsite-disable-click-to-copy}
[21611.962][echo_server][I] Running C++ echo server with natural types
[21611.965][echo_server][I] Incoming connection for fuchsia.examples.Echo
[21612.998][echo_client][I] (Natural types) got response: hello
[21613.999][echo_client][I] (Natural types) got response: hello
[21614.000][echo_client][I] (Natural types) got event: hello
[21615.002][echo_client][I] (Wire types) got response: hello
[21615.003][echo_client][I] (Natural types) got event: hello
[21615.003][echo_server][I] Client disconnected
```

Note that it takes about one second between
`Incoming connection for fuchsia.examples.Echo` and
`(Natural types) got response: hello`, because the server is programmed to
asynchronously delay the response by one second.

Terminate the realm component to stop execution and clean up the component
instances:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo_realm
```

<!-- xrefs -->
[src]: /examples/fidl/cpp/server_async_completer
[server-tut]: /development/languages/fidl/tutorials/cpp/basics/server.md
[overview]: /development/languages/fidl/tutorials/cpp/README.md
[memory-ownership]: /development/languages/fidl/tutorials/cpp/topics/wire-memory-ownership.md
[wire-types]: /development/languages/fidl/tutorials/cpp/basics/domain-objects.md#using-wire
