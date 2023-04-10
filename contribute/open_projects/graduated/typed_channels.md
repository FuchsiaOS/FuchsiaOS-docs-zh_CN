# Low-level C++ typed channel migration

## Goal & motivation

FIDL [protocols and protocol requests][fidl-protocol] are backed by Zircon
channels under the hood. Given the following FIDL definition:

```fidl
library foo;

protocol Calculator {};

resource struct Record {
    // Client endpoint of a channel speaking the Calculator protocol
    Calculator c;
    // Server endpoint of a channel speaking the Calculator protocol
    request<Calculator> s;
};
```

We used to generate a struct with two Zircon channels in LLCPP:

```c++
struct Record {
    zx::channel c;
    zx::channel s;
};
```

Any FIDL protocol became just a channel, opening the door to accidentally mixing
up protocol types or directions (here are some instances that were
[identified][fix-protocol-1] [and][fix-protocol-2] [fixed][fix-protocol-3]). To
increase type safety and self-documentation, we have changed the generated code
to the following:

```c++
struct Record {
    // Now it's clear that |c| is a client channel endpoint speaking the |Calculator| protocol.
    fidl::ClientEnd<foo::Calculator> c;
    // Similarly, |s| is a server channel endpoint for that protocol.
    fidl::ServerEnd<foo::Calculator> s;
};
```

Similarly, all functions in the LLCPP runtime that previously dealt with
`zx::channel` were updated to speak a more precise type that encodes the
direction and kind of the protocol (for example:
[`fidl::BindServer`][bind-server]).

However, the majority of user code still uses `zx::channel`. They continue to
compile because we have added temporary implicit conversions support to
`fidl::ClientEnd` / `fidl::ServerEnd`, at the cost of type safety. To reap the
benefits of this change across the code base, user code should propagate the
`fidl::ClientEnd` / `fidl::ServerEnd` type through their public interface, as
opposed to locally casting from a raw channel.

## Technical background

[LLCPP typed channel reference][typed-channel]

## How to help

### Picking a task

Search for a BUILD.gn file that contains the string `TODO(fxbug.dev/69585)`. It
would look similar to this:

```gn
# TODO(fxbug.dev/69585): This target uses raw zx::channel with LLCPP which is deprecated.
# Please migrate to typed channel APIs (fidl::ClientEnd<T>, fidl::ServerEnd<T>).
# See linked bug for details.
configs += [ "//build/cpp:fidl-llcpp-deprecated-raw-channels" ]
```

Remove these lines and `fx build`. If the build succeeds without any warning or
error, skip to the [last step](#finalize). Otherwise, the warning and errors
point to the deprecated usages. From there, three typical scenarios follow:

### Scenario 1: implementing a server

Migrating servers is quite straightforward - look for places where the server
implementation is inheriting from a class named
[`RawChannelInterface`][raw-channel-interface]. That class is a shim that
translates server methods taking `fidl::ClientEnd<P>` / `fidl::ServerEnd<P>`
arguments into ones taking `zx::channel`. Change that to the usual `Interface`
and update method arguments to match:

#### FIDL

```fidl
protocol Foo {
    TakeBar(Bar bar);
    HandleBar(request<Bar> bar);
};
```

* {Before}

  ```c++
  class MyServer : public fidl::WireRawChannelInterface<Foo> {
    void TakeBar(zx::channel bar, TakeBarCompleter::Sync& completer) override;
    void HandleBar(zx::channel bar, HandleBarCompleter::Sync& completer) override;
  };
  ```

* {After}

  ```c++
  class MyServer : public Foo::Interface {
    void TakeBar(fidl::ClientEnd<Bar> bar, TakeBarCompleter::Sync& completer) override;
    void HandleBar(fidl::ServerEnd<Bar> bar, HandleBarCompleter::Sync& completer) override;
  };
  ```

### Scenario 2: protocol request pipelining

It's common to create a pair of channel endpoints, and pass the server-end to
the protocol implementation. We can avoid creating raw Zircon channels with the
[`fidl::CreateEndpoints<Protocol>`][create-endpoints] method:

* {Before}

  ```c++
  zx::channel client_end, server_end;
  zx_status_t status = zx::channel::create(0, &client_end, &server_end);
  if (status != ZX_OK)
    return status;
  foo.HandleBar(std::move(server_end));
  fidl::WireClient<Bar> bar(std::move(client_end), &dispatcher);
  ```

* {After}

  ```c++
  auto bar_ends = fidl::CreateEndpoints<Bar>();
  if (!bar_ends.is_ok())
    return bar_ends.status_value();
  foo.HandleBar(std::move(bar_ends->server));
  fidl::WireClient bar(std::move(bar_ends->client), &dispatcher);

  // Alternatively, |CreateEndpoints| supports returning the client-end by address,
  // which would be useful when the client-end is an instance variable, for example
  // in a test fixture.
  fidl::ClientEnd<Foo> bar_client_end;
  auto bar_server_end = fidl::CreateEndpoints(&bar_client_end);
  if (!bar_server_end.is_ok())
    return bar_server_end.status_value();
  foo.HandleBar(std::move(*bar_server_end));
  ```

Note that the protocol template parameter to `fidl::WireClient` may be omitted
when typed channels are used, leading to more succinct code.

#### Sync clients

You may use `fidl::WireSyncClient` to convert a `fidl::ClientEnd` into the
corresponding synchronous client for the protocol. This has the advantage of
avoiding having to spell out the protocol type twice (one in `ClientEnd` and
then in the synchronous client class).

```
fidl::WireSyncClient bar{std::move(bar_ends->client)};
```

### Scenario 3: connecting to a protocol

[`fdio_service_connect`][fdio-service-connect] is commonly used to connect to
FIDL services in a component's namespace. Because its signature is C, it becomes
quite verbose to use, especially in the presence of typed channels. We have
created ergonomic wrappers: [`component::Connect<Protocol>`][service-connect],
[`component::ConnectAt<Protocol>`][service-connect-at], and
[`component::OpenServiceRoot`][open-service-root]. They are located in the
[sdk/lib/sys/component/cpp][lib-component] library.

#### Connecting to an individual protocol

* {Before}

  ```c++
  zx::channel client_end, server_end;
  zx_status_t status = zx::channel::create(0, &client_end, &server_end);
  if (status != ZX_OK)
    return status;
  status = fdio_service_connect("/svc/fuchsia.Foo", server_end.release());
  if (status != ZX_OK)
    return status;
  fidl::WireClient<Foo> foo(std::move(client_end), &dispatcher);
  ```

* {After}

  ```c++
  // The channel creation and service connection is done in one function.
  // By default it opens the protocol name.
  // Returns |zx::result<fidl::ClientEnd<Foo>>|.
  auto client_end = component::Connect<Foo>();
  if (!client_end.is_ok())
    return client_end.status_value();
  // Note: can omit template argument
  fidl::WireClient foo(std::move(*client_end), &dispatcher);
  ```

#### Opening service directory

* {Before}

  ```c++
  zx::channel client_end, server_end;
  zx_status_t status = zx::channel::create(0, &client_end, &server_end);
  if (status != ZX_OK)
    return status;
  status = fdio_service_connect("/svc", server_end.release());
  if (status != ZX_OK)
    return status;
  fidl::WireClient<::fuchsia_io::Directory> dir(std::move(client_end));
  ```

* {After}

  ```c++
  // The channel creation and service connection is done in one function.
  // Opens "/svc" and returns the client endpoint, as a
  // |zx::result<fidl::ClientEnd<::fuchsia_io::Directory>>|.
  auto client_end = component::OpenServiceRoot<Foo>();
  if (!client_end.is_ok())
    return client_end.status_value();
  // Note: can omit template argument
  fidl::WireClient dir(std::move(*client_end), &dispatcher);
  ```

### Note: propagating protocol types

Whenever feasible, prefer to propagate the protocol types across related
functions and variables. **Any time you find yourself creating a `ClientEnd` /
`ServerEnd` / `UnownedClientEnd` from a channel, consider if the source channel
could also be changed to a typed channel**. They serve as self-checking
documentation and could reveal incorrect assumptions about the kind of protocols
flowing through a channel. Different from LLCPP generated structures, using
typed channels on the public API does not unfavorably predispose the interface
towards a particular ownership model or set of types, because typed channels are
simply lightweight wrappers around Zircon channels. Here we show an example
migrating a `zx::unowned_channel`:

* {Before}

  ```c++
  // |client| should speak the |fuchsia.foobar/Baz| protocol.
  zx_status_t DoThing(zx::unowned_channel client, int64_t args) {
    return fidl::WireCall<fuchsia_foobar::Baz>(std::move(client))->Method(args).status();
  }
  ```

* {After}

  ```c++
  // The intended protocol is encoded in the type system. No need for comment.
  zx_status_t DoThing(fidl::UnownedClientEnd<fuchsia_foobar::Baz> client, int64_t args) {
    return fidl::WireCall(client)->Method(args).status();
  }
  ```

### Note: resolving type mismatch due to protocol composition

There is [no "is-a" (inheritance, subsumption) relationship][rfc-0023] between
FIDL protocols when one composes another. This implies that when protocol `More`
composes protocol `Less`, one may want to call a function `void
foo(fidl::ClientEnd<Less>)` with a `fidl::ClientEnd<More>`, but we would not
provide implicit conversions between those types.

Upon determining that the usage is safe, one could manually convert one
client-end into another via
`fidl::ClientEnd<Less>(more_client_end.TakeChannel())`. Prefer commenting on the
conversion as to why it would be safe (e.g. `More` will not add new events on
top of `Less`).

## Last step: making the CL {#finalize}

Before uploading the changes, make sure to double-check these three places:

* The `"//build/cpp:fidl-llcpp-deprecated-raw-channels"` config was removed from
  your target-specific `BUILD.gn` file.
* In `//build/cpp/BUILD.gn`, delete the lines in the [visibility
  section][target-allowlist] corresponding to your GN target, such that it won't
  regress back into raw channels. It'll also easily visualize the migration
  progress.
* If you're sure that the target being migrated is the last user of the
  `RawChannelInterface` of a particular FIDL protocol, you may delete that
  [protocol from the `fidlgen_cpp` compiler][protocol-allowlist]. Don't worry,
  the code won't compile if you made a premature removal.

Then you can upload the CL and tag it with `Bug: 69585` 🎉

You may add one of ianloic@, yifeit@ if need specific review from the FIDL team.

## Example CLs

* [Example 1: network-device][example-1]
* [Example 2: ddk-lifecycle][example-2]
* [Example 3: ldsvc][example-3]

## Known pain-points identified during migration:

* When converting `fdio_get_service_handle`, the function takes an out-param of
  `zx_handle_t`, without any protocol types. We would like a
  `fidl::ClientEnd<T>`.
* When converting `fdio_open(path, flags, server.release())`, there is no
  type-safe alternative of `fdio_open`.
* Converting between HLCPP and LLCPP endpoint types is tricky. We would like
  `fidl::ClientEnd<::my_thing::Protocol>` and
  `fidl::InterfaceHandle<my::thing::Protocol>` to easily convert into one
  another, and same for servers.
* HLCPP and legacy component framework APIs (`sys::ServiceDirectory`,
  `sys::OutgoingDirectory`) use HLCPP `InterfaceHandle` and `InterfaceRequest`
  types, hence need additional conversion into LLCPP typed channels.

## Sponsors

Reach out for questions or for status updates:

* yifeit@google.com
* ianloic@google.com

<!-- xrefs -->
[fidl-protocol]: /docs/reference/fidl/language/language.md#protocols
[fix-protocol-1]: https://fuchsia-review.googlesource.com/c/fuchsia/+/463034
[fix-protocol-2]: https://fuchsia-review.googlesource.com/c/fuchsia/+/466169
[fix-protocol-3]:
https://fuchsia-review.googlesource.com/c/fuchsia/+/478491/28/src/storage/lib/paver/partition-client.h#b46
[bind-server]:
https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/server.h?q=fidl::BindServer&ss=fuchsia%2Ffuchsia
[typed-channel]: /docs/reference/fidl/bindings/cpp-bindings.md#typed-channels
[raw-channel-interface]:
https://cs.opensource.google/search?q=RawChannelInterface&sq=&ss=fuchsia%2Ffuchsia
[create-endpoints]:
https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/connect_service.h;l=36?q=fidl::CreateEndpoints&ss=fuchsia%2Ffuchsia
[fdio-service-connect]:
https://cs.opensource.google/search?q=fdio_service_connect&ss=fuchsia%2Ffuchsia&start=11
[service-connect]:
https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/component/incoming/cpp/protocol.h;l=41
[service-connect-at]:
https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/component/incoming/cpp/protocol.h;l=69
[open-service-root]:
https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/component/incoming/cpp/protocol.h;l=28
[lib-component]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/sys/component/cpp/
[rfc-0023]: /docs/contribute/governance/rfcs/0023_compositional_model_protocols.md#is_a_relationship_considered_harmful
[target-allowlist]:
https://cs.opensource.google/fuchsia/fuchsia/+/main:build/cpp/BUILD.gn?q=%22could%20be%20migrated%20to%20use%20typed%20channels%22&ss=fuchsia%2Ffuchsia:build%2Fcpp%2F
[protocol-allowlist]:
https://cs.opensource.google/fuchsia/fuchsia/+/main:tools/fidl/lib/fidlgen_cpp/typed_channel_migration.go?q=%22rawChannelInterfaceAllowed%20%3D%20map%5Bstring%5Dbool%22&ss=fuchsia%2Ffuchsia
[example-1]: https://fxrev.dev/I195ebf4d38c4e949de2b913b8c96788275889b5a
[example-2]: https://fxrev.dev/I36f77e139eae177890d8037ec4f0e9396496d70b
[example-3]: https://fxrev.dev/I89ec43ffb61eb3b5a5b21f2da4835862895ee842
