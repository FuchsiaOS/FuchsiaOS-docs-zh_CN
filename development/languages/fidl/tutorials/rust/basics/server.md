# Implement a FIDL server in Rust

## Prerequisites

This tutorial assumes that you are familiar with listing the FIDL Rust
bindings for a library as a dependency in GN and importing the bindings into
Rust code, which is covered in the [Rust FIDL crates][fidl-crates] tutorial.

## Overview

<!-- TODO(fxbug.dev/58758) <<../../common/server/overview.md>> -->

This tutorial shows you how to implement a FIDL protocol
(`fuchsia.examples.Echo`) and run it on Fuchsia. This protocol has one method
of each kind:

* `EchoString` is a method with a response.
* `SendString` is a method without a response.
* `OnString` is an event.

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="echo" %}
```

For more on FIDL methods and messaging models, refer to the [FIDL concepts][concepts] page.

This document covers how to complete the following tasks:

* Implement a FIDL protocol.
* Build and run a package on Fuchsia.
* Serve a FIDL protocol.

The tutorial starts by creating a component that is served to a Fuchsia device
and run. Then, it gradually adds functionality to get the server up and running.

If you want to write the code yourself, delete the following directories:

```posix-terminal
rm -r examples/fidl/rust/server/*
```

## Create the component {#component}

To create a component:

1. Add a `main()` function to `examples/fidl/rust/server/src/main.rs`:

   ```rust
   fn main() {
     println!("Hello, world!");
   }
   ```

1. Declare a target for the server in `examples/fidl/rust/server/BUILD.gn`:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/BUILD.gn" region_tag="imports" %}

   # Declare an executable for the server. This produces a binary with the
   # specified output name that can run on Fuchsia.
   rustc_binary("bin") {
     output_name = "fidl_echo_rust_server"
     edition = "2018"

     sources = [ "src/main.rs" ]
   }

   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/BUILD.gn" region_tag="rest" %}
   ```

   <!-- TODO(fxbug.dev/58758) <<../../common/server/packages.md>> -->

   To get the server component up and running, there are three targets that are
   defined:

   * The raw executable file for the server that is built to run on Fuchsia.
   * A component that is set up to simply run the server executable,
     which is described using the component's manifest file.
   * The component is then put into a package, which is the unit of software
     distribution on Fuchsia. In this case, the package just contains a
     single component.

   For more details on packages, components, and how to build them, refer to
   the [Building components][building-components] page.

1. Add a component manifest in `examples/fidl/rust/server/meta/server.cml`:

   Note: The binary name in the manifest must match the output name of the `executable`
   defined in the previous step.

   ```json5
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/meta/server.cml" region_tag="example_snippet" %}
   ```

   <!-- TODO(fxbug.dev/58758) <<../../common/server/qemu.md>> -->

1. Add the server to your build configuration:

   ```posix-terminal
   fx set core.qemu-x64 --with //examples/fidl/rust/server:echo-rust-server
   ```

   Note: This build configuration assumes your device target is the emulator.
   To run the example on a physical device, select the appropriate
   [product configuration][products] for your hardware.

1. Build the Fuchsia image:

   ```posix-terminal
   fx build
   ```

## Implement the server

First you'll implement the behavior of the Echo protocol. In Rust, this is expressed
as code that can handle the protocol's associated request stream type, which in this case is an
`EchoRequestStream`. This type is a stream of Echo requests, i.e. it implements
`futures::Stream<Item = Result<EchoRequest, fidl::Error>>`.

You'll implement `run_echo_server()` to handle the request stream,
which is an async function that handles incoming service requests.
It returns a future that completes once the client channel is closed.

### Add dependencies

1. Import the required dependencies:

   ```rust
   // we'll use anyhow to propagate errors that occur when handling the request stream
   use anyhow::{Context as _, Error};
   // the server will need to handle an EchoRequestStream
   use fidl_fuchsia_examples::{EchoRequest, EchoRequestStream};
   // import the futures prelude, which includes things like the Future and Stream traits
   use futures::prelude::*;
   ```
1. Add them as build dependencies to the `rustc_binary` target. The deps field should look like:

   ```gn
   deps = [
     "//examples/fidl/fuchsia.examples:fuchsia.examples-rustc",
     "//third_party/rust_crates:anyhow",
     "//third_party/rust_crates:futures",
   ]
   ```

Note: The `rustdoc` for all Fuchsia crates can be found at
[https://fuchsia-docs.firebaseapp.com/rust/](https://fuchsia-docs.firebaseapp.com/rust/). You can
refer to this as necessary for documentation on the crates mentioned in this and following
tutorials.

### Define `run_echo_server`:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/src/main.rs" region_tag="impl" %}
```

The implementation consists of the following elements:

* The code converts the `fidl:Error`s from the request stream into `anyhow::Error`s, by attaching
  context using the `.context()` method on each result:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/src/main.rs" region_tag="impl" highlight="3,4" %}
  ```

  At this stage, the stream of `Result<EchoRequest, fidl::Error>` becomes a stream of
  `Result<EchoRequest, anyhow::Error>`.
* Then, the function calls [try_for_each][try-for-each] on the resulting stream, which returns a
  future. This method unwraps the `Result`s in the stream - any failures cause the future
  to return immediately with that error, and the contents of any successes are passed to the
  closure. Similarly, if the return value of the closure resolves to a failure, the resulting future
  will return immediately with that error:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/src/main.rs" region_tag="impl" highlight="5,23" %}
  ```
* The contents of the closure handle incoming `EchoRequest`s by matching on them to
  determine what kind of request they are:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/src/main.rs" region_tag="impl" highlight="6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22" %}
  ```

  This implementation handles `EchoString` requests by echoing the input back, and it handles
  `SendString` requests by sending an `OnString` event. Since `SendString` is a fire and forget
  method, the request enum variant comes with a [control handle][control-handle], which can be used
  to communicate back to the server.

  In both cases, errors from sending messages back to the client are propagated by adding context
  and using the `?` operator. If the end of the closure is reached successfully, then it returns
  `Ok(())`.
* Finally, the server function `await`s the future returned from `try_for_each` to completion, which
  will call the closure on every incoming request, and return when either all requests have been
  handled or any error is encountered.

You can verify that the implementation is correct by running:

```posix-terminal
fx build
```

## Serve the protocol {#main}

Now that you've defined code to handle incoming requests, you'll need listen for incoming
connections to the Echo server. This is done by asking the
[component manager][component-manager] to expose the Echo protocol to other components. The
comopnent manager then routes any requests for the echo protocol to our server.

To fulfill these requests, the component manager requires the name of the protocol
as well as a handler that it should call when it has any incoming requests to
connect to a protocol matching the specified name.

### Add dependencies

1. Import the required dependencies:

   ```rust
   // Import the Fuchsia async runtime in order to run the async main function
   use fuchsia_async as fasync;
   // ServiceFs is a filesystem used to connect clients to the Echo service
   use fuchsia_component::server::ServiceFs;
   ```
1. Add them as build dependencies to the `rustc_binary` target. The full target looks like:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/BUILD.gn" region_tag="server" %}
   ```

### Define the `main` function

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/src/main.rs" region_tag="main" highlight="1,2,20" %}
```

The main function is async since it consists of listening for incoming connections to the
Echo server. The `run_singlethreaded` attribute tells the fuchsia async runtime to run
the `main` future to completion on a single thread.

The `run_singlethreaded`, `run`, and `run_until_stalled` macros from
the `fuchsia_async` crate can be used to run asynchronous `main` or test
functions to completion using the `fuchsia_async::Executor`.

`main` also returns `Result<(), Error>`. If an `Error` is returned from `main`
as a result of one of the `?` lines, the error will be `Debug` printed and
the program will return with a status code indicating failure.

### Initialize `ServiceFs`

Obtain an instance of `ServiceFs`, which represents a filesystem containing various services.
Since the server will be run singlethreaded, use
`ServiceFs::new_local()` instead of `ServiceFs::new()` (the latter is multithreaded capable).

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/src/main.rs" region_tag="main" highlight="4" %}
```

### Add the Echo FIDL service

Ask the component manager to expose the Echo FIDL service. There are two parts to this
function call:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/src/main.rs" region_tag="main" highlight="5" %}
```

* The component manager must know what to do with incoming connection requests. This is specified
  by passing in a closure that accepts a `fidl::endpoints::RequestStream`, and returns some new
  value with it. For example, passing in a closure of `|stream: EchoRequestStream| stream` would
  be completely valid. A [common pattern](https://fuchsia-docs.firebaseapp.com/rust/fuchsia_component/server/struct.ServiceFsDir.html#method.add_fidl_service)
  is to define an enum of the possible services offered by the server, in this example:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/src/main.rs" region_tag="enum" %}
  ```

  and then passing the enum variant "constructor" as the closure. When there are multiple services
  being offered, this results in a common return type (the `IncomingService` enum). The return
  values of all `add_fidl_service` closures will become the elements in the `ServiceFs` stream when
  [listening for incoming connections](#incoming).

* The component manager must also know *where* this service is going to be available.
  Since this is an outgoing service (i.e. a service that is offered to other components),
  the service must add a path inside `/svc` directory. `add_fidl_service` obtains this
  path implicitly by taking the [`SERVICE_NAME`](https://fuchsia-docs.firebaseapp.com/rust/fidl/endpoints/trait.DiscoverableService.html)
  associated with the closure input argument.
  In this case, the closure argument (`IncomingService::Echo`) has an input argument of type
  `EchoRequestStream`, which has an associated `SERVICE_NAME` of `"fuchsia.examples.Echo"`. So this
  call is adding an entry at `/svc/fuchsia.examples.Echo`, and clients will need to search for a
  service called `"fuchsia.examples.Echo"` to connect to this server.

### Serve the outgoing directory

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/src/main.rs" region_tag="main" highlight="7,8" %}
```

This call will bind the `ServiceFs` to the `DirectoryRequest` startup handle for the component, and
listen for incoming connection requests.
Note that since this removes the handle from the process's handle table,
this function can only be called once per process. If you wish to provide
a `ServiceFs` to a different channel, you can use the `serve_connection`
function.

This process is described further in
[Life of a protocol open][protocol-open].

### Listen for incoming connections {#incoming}

Run the `ServiceFs` to completion in order to listen for incoming connections:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/server/src/main.rs" region_tag="main" highlight="10,11,12,13,14,15,16,17" %}
```

This runs the `ServiceFs` future, handling up to 10,000 incoming
requests concurrently. The closure passed to this call is the handler used for incoming requests -
`ServiceFs` will first matching incoming connections to the closure provided to `add_fidl_service`,
then call the handler on the result (which is an `IncomingService`). The handler takes
the `IncomingService`, and calls `run_echo_server` on the inner request stream to handle incoming
Echo requests.

There are two types of requests being handled here. The stream of requests handled by the
`ServiceFs` consists of requests to connect to an Echo server (i.e. each client will make this type
of request once when connecting to the server), whereas the stream of requests handled by
`run_echo_server` are requests on the Echo protocol (i.e. each client may make any number of
`EchoString` or `SendString` requests to the server). Many clients can request to connect to the
Echo server at the same time, so this stream of requests is handled concurrently. However, all
requests for a single client happen in sequence so there is no benefit to processing requests
concurrently.

## Test the server

Rebuild:

```posix-terminal
fx build
```

Then run the server component:

```posix-terminal
ffx component run fuchsia-pkg://fuchsia.com/echo-rust-server#meta/echo_server.cm
```

Note: Components are resolved using their [component URL][glossary.component-url],
which is determined with the [`fuchsia-pkg://`][glossary.fuchsia-pkg-url] scheme.

You should see output similar to the following in the device logs (`ffx log`):

```none {:.devsite-disable-click-to-copy}
[ffx-laboratory:echo_server][][I] Listening for incoming connections...
```

The server is now running and waiting for incoming requests.
The next step will be to write a client that sends `Echo` protocol requests.
For now, you can simply terminate the server component:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo_server
```

Note: Component instances are referenced by their
[component moniker][glossary.moniker], which is determined by their location in
the [component instance tree][glossary.component-instance-tree]

<!-- xrefs -->
[glossary.component-instance-tree]: glossary/README.md#component-instance-tree
[glossary.component-url]: glossary/README.md#component-url
[glossary.fuchsia-pkg-url]: glossary/README.md#fuchsia-pkg-url
[glossary.moniker]: glossary/README.md#moniker
[concepts]: concepts/fidl/overview.md
[fidl-crates]: development/languages/fidl/tutorials/rust/basics/using-fidl.md
[building-components]: development/components/build.md
[products]: development/build/build_system/boards_and_products.md
[control-handle]: reference/fidl/bindings/rust-bindings.md#protocol-control-handle
[declaring-fidl]: development/languages/fidl/tutorials/fidl.md
[component-manager]: concepts/components/v2/component_manager.md
[protocol-open]: concepts/components/v2/capabilities/life_of_a_protocol_open.md#binding_to_a_component_and_sending_a_protocol_channel
[compiling-fidl]: development/languages/fidl/tutorials/fidl.md
[async-loop]: /zircon/system/ulib/async-loop/include/lib/async-loop/cpp/loop.h
[overview]: development/languages/fidl/tutorials/overview.md
[try-for-each]: https://docs.rs/futures/0.3.5/futures/stream/trait.TryStreamExt.html#method.try_for_each
