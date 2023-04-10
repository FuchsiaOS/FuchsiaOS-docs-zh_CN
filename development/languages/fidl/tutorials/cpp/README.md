# New C++ bindings tutorials

This section helps you learn how to use the new C++ FIDL bindings. See
[Getting started](#getting_started) for a step-by-step guide to setting up the
build and writing a simple client or server from scratch. See [Topics](#topics)
for more involved guides and recommendations to using the bindings effectively.
See [Terminologies](#terminologies) for names and concepts that come up
frequently in code, and a quick explainer on how to make the right choices.

At a high level, the C++ bindings is made up of:

* **Data**: the [domain objects](/docs/glossary/README.md#domain-object) (the
  generated FIDL structures e.g. struct, table, ...)
* **Behavior**: the client/server APIs to send those domain objects over a
  protocol, receive events, ...

## Natural and wire domain objects

The bindings support two types of domain objects: **natural** types and
**wire** types:

* **natural** types are high level domain objects that optimize for ergonomics.
    * These types own their children with smart pointers.
    * They use idiomatic C++ types such as `std::vector`, `std::optional`, and
      `std::string`.
    * Given a FIDL library named `fuchsia.my.lib`, the types are generated
      in the `fuchsia_my_lib` namespace.
* **wire** types are optimized for performance and in-place decoding.
    * They are specialized C++ Standard Layout types whose memory layout
      coincides with the FIDL wire format.
    * Out-of-line children are unowned pointers into a separate buffer.
      See [Memory ownership of wire domain objects][memory-ownership].
    * Given a FIDL library named `fuchsia.my.lib`, the types are generated
      in the `fuchsia_my_lib::wire` namespace.

When starting a project, choose natural types by default, since they are easier
to use and reasonably performant. Only turn to the wire types when optimizing
logic in the critical path, or when needing to precisely control memory
allocation. Because the wire types consists of unsafe views, improper use of
those may lead to use-after-free and other memory safety bugs.

## Getting started

1. [Using natural and wire domain objects][domain-objects]
2. [Write a server][server]
3. Write a client ([async][async] or [synchronous][sync])

## Topics

* [Memory ownership of wire domain objects][memory-ownership]
* [Responding asynchronously][async-completers]
* [Thread safety and memory safety][threading]
* [Communicating over the driver transport][driver-transport]
* [Request pipelining][pipelining]
* [Unified services][services]

## Terminologies

### `Wire` vs no `Wire` in clients and servers

The presence of `Wire` prefix in a client/server API name indicates that the API
only accepts wire types. Otherwise, the API typically can accept both wire and
natural types, or defaults to natural types. For example, `fidl::Client`
supports making calls with both natural and wire types, whereas
`fidl::WireClient` exposes a more restrictive interface that only accepts wire
types.

`fidl::Server` receives requests into natural types, and may send replies with
either natural or wire types. On the other hand, `fidl::WireServer` receives
requests into wire types and sends replies exclusively in wire types.

To support both wire and natural types on the send side without function
overload ambiguities, the wire interfaces are housed under a `.wire()` accessor.
For example, given a `fidl::Client<MyProtocol> client;`, one would write
`client->SomeMethod(natural_type);` to make request using natural types, and
`client.wire()->SomeMethod(wire_type);` to make request using wire types.

#### Recommendation

Use client/server APIs without the `Wire` prefix. Only when there is a need to
ensure at compile time that only wire types are used, one may define function
signatures that use the `Wire` counterparts e.g. `fidl::WireClient`. One may
also depend on only the wire parts of the bindings by depending on the
`fuchsia.my.lib_cpp_wire` target instead of the `fuchsia.my.lib_cpp` GN target.

<!-- TODO(fxbug.dev/98989): Talk about Bazel targets -->

Note: the wire types portions of the bindings was historically named "Low-Level
C++ bindings", or LLCPP.

### `Sync` vs no `sync` in clients

Synchronous, or "sync" for short, applies to FIDL calls with a response (two-way
calls), and means the call is blocking: a thread making such a call will not
return from the call until the response comes back. For example,
`fidl::WireSyncClient` is a client where all two-way calls are synchronous.
Similarly, `fidl::WireClient` has a `.sync()` accessor which returns an
interface for making synchronous calls.

One-way calls do not have a response, hence the concept of synchronousness do
not apply to them.

#### Recommendation

If your code is a standalone program that only consumes capabilities from other
components, determine the level of concurrency required by its business needs:

* If it does not manage lots of concurrent operations, you may use a synchronous
  client which leads to easy to read straight-line logic. For example, a
  short-running command line tool may use `fidl::SyncClient`.

* If your code manages lots of concurrent operations, it typically has access to
  an asynchronous dispatcher (`async_dispatcher_t*`). When choosing between
  synchronous and asynchronous clients and calls in that case, prefer the
  asynchronous counterpart. For example, prefer `fidl::WireClient` without going
  through `.sync()` over `fidl::WireSyncClient` or `.sync()`. In particular, do
  not make synchronous calls on a dispatcher thread if the dispatcher is single
  threaded, to avoid deadlocks.

If your code is a service, i.e. a component that provides capabilities to other
components, it should use asynchronous dispatchers and asynchronous clients to
support the level of concurrency needed by multiple consumers.

If your code is a library that's used by other applications, it will require
more careful thought regarding whether it should expose a synchronous or
asynchronous interface, depending on the needs of its users. For example, a
library using synchronous clients and exposing a synchronous interface will be
more difficult to use by highly concurrent applications that schedules their
work on asynchronous dispatchers.

The above is general advice, and different asynchronous runtimes may have their
own more specific recommendations.

### `Shared` vs no `shared` in clients

When a client type has "shared" in its name, it may be bound and destroyed on
arbitrary threads. See [`SharedClient`][shared-client] in the threading
guide. It will have a counterpart without "shared", such as `Client`, that
must be bound and destroyed on the dispatcher thread. A similar relationship
exists between `WireClient` and `WireSharedClient`.

#### Recommendation

When choosing between [`Client`][client] and `SharedClient`, prefer
`Client` unless the threading model or performance requirements of your
application necessitates multi-threaded usage of clients. Refer to the
[threading guide][threading] for the many areas of caution when using
`SharedClient`. The extra restrictions in `Client` are designed to
reduce memory races and use-after-frees. For example, you may use `Client`
if your objects all live on the same single-threaded async dispatcher.

### `Then` vs `ThenExactlyOnce` in two-way calls

When an asynchronous call has a response, there are two ways to specify a
callback to receive the result of that call:

* When you use `.ThenExactlyOnce(...)`, the callback is always called exactly
  once, delivering the result.
* When you use `.Then(...)`, the callback is silently discarded when the client
  object is destroyed, which is suitable for object-oriented code.

#### Motivation for `Then`

When making an asynchronous two-way call, the result of that call is delivered
back to the application at a later time, after the execution had already left
the original scope of making the call. The asynchronous dispatcher would later
invoke the follow-up logic you specified when making the call, called a
_continuation_. This means it's easy to use objects after they are destroyed,
leading to memory corruptions:

```c++
// The following snippet shows an example of use-after-free
// occurring in asynchronous two-way calls.
void Foo(fidl::WireClient<MyProtocol>& client) {
  bool call_ok;
  client->SomeMethod().Then(
      // The following lambda function represents the continuation.
      [&call_ok] (fidl::WireUnownedResult<SomeMethod>& result) {
        // `call_ok` has already gone out of scope.
        // This would lead to memory corruption.
        call_ok = result.ok();
      });
}
```

A more insidious form of this corruption occurs when the continuation captures
the `this` pointer, and said referenced object also owns the client. Destroying
the outer object (in turn, destroying the client) causes all pending two-way
calls to fail. As their continuation runs, the `this` pointer it captured is no
longer valid.

Both `Then` and `ThenExactlyOnce` registers a continuation for a two-way call.
However, `Then` is designed to mitigate corruption cases like the above.
Specifically:

* `Then` ensures the provided continuation will be called at most once, until
  the client is destroyed. You should choose `Then` if your continuation only
  captures objects with the same lifetime as the client (e.g. your user object
  owns the client). Destroying the user object passivates any outstanding
  callbacks. No concerns of use-after-free.

* `ThenExactlyOnce` on the other hand guarantees to call the continuation
  exactly once. If the client object is destroyed, the continuation receives a
  cancellation error. You need to ensure any referenced objects are still alive
  by the time the continuation runs, which may be an unspecified time after the
  client object is destroyed. You should choose `ThenExactlyOnce` if your
  continuation must be called exactly once, such as when interfacing with
  `fpromise` completers or FIDL server completers, or during unit tests.

#### Recommendation

As a rule of thumb:

* If your callback looks like `client_->Foo([this]`, use `Then` (note that
  `client_` is a member variable).
* If your callback looks like
    * `client->Foo([completer]`, or
    * `client->Foo([]`, or
    * `client->Foo([&]` (common in unit tests),
    * callback captures a weak pointer or a strong pointer,
    * use `ThenExactlyOnce`.

Do not capture objects of differing lifetimes such that only a subset of the
objects are alive when the continuation runs.

### Zircon channel transport vs driver transport

A FIDL protocol is associated with a corresponding transport, specified in the
FIDL definition, which determines the kinds of resources that may flow through
the protocol, and may affect the generated API for sending and receiving
messages. The C++ bindings support two transports:

The Zircon channel transport is represented by endpoint types
`fidl::ClientEnd<SomeProtocol>` and `fidl::ServerEnd<SomeProtocol>`.

The driver transport uses endpoint types `fdf::ClientEnd<SomeProtocol>` and
`fdf::ServerEnd<SomeProtocol>`.

### Arenas

Arenas objects manage a pool of memory buffers and provide efficient allocation.
They are used pervasively in wire domain objects and wire clients and servers to
avoid expensive copies.

Arenas are not used with natural domain objects and associated clients and
servers, which encapsulate details about memory allocation.

You may use `fidl::Arena` to create wire domain objects which live on that
arena. See [memory management][memory-ownership].

When using protocols over the [driver transport][driver-transport] with wire
domain objects, `fdf::Arena` objects should be used to allocate the buffers
needed to encode messages.

<!-- xrefs -->
[domain-objects]: basics/domain-objects.md
[server]: basics/server.md
[async]: basics/client.md
[sync]: basics/sync-client.md
[pipelining]: topics/request-pipelining.md
[services]: topics/services.md
[async-completers]: topics/async-completer.md
[memory-ownership]: topics/wire-memory-ownership.md
[threading]: topics/threading.md
[driver-transport]: topics/driver-transport.md
[client]: topics/threading.md#client
[shared-client]: topics/threading.md#sharedclient
