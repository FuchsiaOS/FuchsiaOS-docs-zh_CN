# Handling multiple clients

## Prerequisites

This tutorial builds on the [HLCPP getting started tutorials][overview].

## Overview

This tutorial updates the [Echo client][client-tut] from the getting started
tutorials to make multiple connections to the server, and update the
[Echo server][server-tut] to handle multiple client connections. For running
multiple instances of a server (or multiple FIDL protocols), see the
tutorial on [services][services-tut].

The full example code for this tutorial is located at
[`//examples/fidl/hlcpp/multiple_clients`][src].

## Implement the server

In the [previous implementation][server-tut-impl], the `main()` function initialized
a single `fidl::Binding`, and bound any incoming requests to it:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/main.cc" region_tag="main" highlight="5,7,8,9,10,12" %}
```

This means that if a second client tries to connect to the server at the same
time, the second call to `binding.Bind` will overwrite the channel from the
first client. To support multiple clients, keep track of multiple
`fidl::Binding`s (one for each client) using a `fidl::BindingSet`:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/multiple_clients/server/main.cc" region_tag="main" highlight="5,7" %}
```

A binding set also simplifies the code since it no longer
needs to create a custom handler. The binding set has a `GetHandler` method,
which returns a handler that creates a new `Binding` and stores it in a vector.

To use `fidl::BindingSet`, include `lib/fidl/cpp/binding_set.h`.

## Implement the client

In order to manage multiple clients connected to a protocol, the FIDL HLCPP
runtime library provides an anolog to `fidl::BindingSet`: the
`fidl::InterfacePtrSet`. Use the class to write code that makes multiple
connections to the same protocol:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/multiple_clients/client/main.cc" region_tag="main" %}
```

The code for setting up a proxy and making requests is the same as in the
[client tutorial][client-tut-main] except it uses
an interface pointer set to simplify the process of broadcasting a message
to a set of clients. An added benefit of using `fidl::InterfacePtrSet` and
`fidl::BindingSet` is that any binding or interface pointer that experiences an
error on its channel is automatically removed from the set.

To use `fidl::InterfacePtrSet`, include `lib/fidl/cpp/interface_ptr_set.h`.


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
    fx set core.qemu-x64 --with //examples/fidl/hlcpp:echo-hlcpp-multi-client
    ```

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

1. Run the `echo_realm` component. This creates the client and server component
   instances and routes the capabilities:

    ```posix-terminal
    ffx component run /core/ffx-laboratory:echo_realm fuchsia-pkg://fuchsia.com/echo-hlcpp-multi-client#meta/echo_realm.cm
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
[echo_client][][I] Got response Hello echoer 0
```

Terminate the realm component to stop execution and clean up the component
instances:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo_realm
```

<!-- xrefs -->
[glossary.realm]: /docs/glossary/README.md#realm
[client-tut]: /docs/development/languages/fidl/tutorials/hlcpp/basics/client.md
[client-main]: /docs/development/languages/fidl/tutorials/hlcpp/basics/client.md#main
[server-tut]: /docs/development/languages/fidl/tutorials/hlcpp/basics/server.md
[server-tut-impl]: /docs/development/languages/fidl/tutorials/hlcpp/basics/server.md#impl
[services-tut]: /docs/development/languages/fidl/tutorials/hlcpp/topics/services.md
[src]: /examples/fidl/hlcpp/multiple_clients
[overview]: /docs/development/languages/fidl/tutorials/hlcpp/README.md
