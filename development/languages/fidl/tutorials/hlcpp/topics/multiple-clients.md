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
[//examples/fidl/hlcpp/multiple_clients][src].

## Implement the server

In the [previous implementation][server-tut-impl], the `main()` function initialized
a single `fidl::Binding`, and bound any incoming requests to it:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/server/main.cc" region_tag="main" highlight="5,7,8,9,10,12" %}
```

This means that if a second client tries to connect to the server at the same
time, the second call to `binding.Bind` will overwrite the channel from the
first client. To support multiple clients, keep track of multiple
`fidl::Binding`s (one for each client) using a `fidl::BindingSet`:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/multiple_clients/server/main.cc" region_tag="main" highlight="5,7" %}
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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/multiple_clients/client/main.cc" region_tag="main" %}
```

The code for setting up a proxy and making requests is the same as in the
[client tutorial][client-tut-main] except it uses
an interface pointer set to simplify the process of broadcasting a message
to a set of clients. An added benefit of using `fidl::InterfacePtrSet` and
`fidl::BindingSet` is that any binding or interface pointer that experiences an
error on its channel is automatically removed from the set.

To use `fidl::InterfacePtrSet`, include `lib/fidl/cpp/interface_ptr_set.h`.


## Run the example

To run the example:

1. Configure the GN build as follows:

   ```
   fx set core.x64 --with //examples/fidl/hlcpp/multiple_clients/client --with //examples/fidl/hlcpp/multiple_clients/server --with //examples/fidl/test:echo-launcher
   ```

1. Build the Fuchsia image:

   ```
   fx build
   ```

1. Run the launcher by passing it the client URL, the server URL, and the protocol that
   the server provides to the client:

   ```
   fx shell run fuchsia-pkg://fuchsia.com/echo-launcher#meta/launcher.cmx fuchsia-pkg://fuchsia.com/echo-hlcpp-multi-client#meta/echo-client.cmx fuchsia-pkg://fuchsia.com/echo-hlcpp-multi-server#meta/echo-server.cmx fuchsia.examples.Echo
   ```

<!-- xrefs -->
[client-tut]: /docs/development/languages/fidl/tutorials/hlcpp/basics/client.md
[client-main]: /docs/development/languages/fidl/tutorials/hlcpp/basics/client.md#main
[server-tut]: /docs/development/languages/fidl/tutorials/hlcpp/basics/server.md
[server-tut-impl]: /docs/development/languages/fidl/tutorials/hlcpp/basics/server.md#impl
[services-tut]: /docs/development/languages/fidl/tutorials/hlcpp/topics/services.md
[src]: /examples/fidl/hlcpp/multiple_clients
[overview]: /docs/development/languages/fidl/tutorials/hlcpp/README.md
