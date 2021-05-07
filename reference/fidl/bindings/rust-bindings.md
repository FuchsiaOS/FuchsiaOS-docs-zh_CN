# Rust bindings

## Generating FIDL Rust crates

A FIDL Rust crate can be generated from a FIDL library in two ways:

1. Manually, using the
   [standard FIDL toolchain](/docs/development/languages/fidl/guides/cli.md).
2. Automatically,
   [using the Fuchsia build system](/docs/development/languages/rust/fidl_crates.md)
   (which under the hood uses the standard FIDL toolchain). This option is only
   available within the Fuchsia source tree.

## Libraries {#libraries}

A FIDL `library` maps to a Rust library crate named `fidl_`, followed by the
full library path with underscores instead of dots.

For example, given the `library` declaration:

```fidl
library fuchsia.examples;
```

The corresponding FIDL crate is named `fidl_fuchsia_examples`:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="import" %}
```

## Constants {#constants}

Given the [constants][lang-constants]:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="consts" %}
```

The FIDL toolchain generates the following constants:

* `pub const BOARD_SIZE: u8`
* `pub const NAME: &str`

The correspondence between FIDL primitive types and Rust types is outlined in
[built-in types](#builtins).

## Fields

This section describes how the FIDL toolchain converts FIDL types to native
types in Rust. These types can appear as members in an aggregate type or as
parameters to a protocol method.

### Built-in types {#builtins}

Note: In Rust, the equivalent type for a nullable FIDL type `T?` is an `Option`
of the Rust type for `T`. These are not explicitly listed in the table below.

In following table, when both an "owned" and "borrowed" variant are specified,
the "owned" type refers to the type that would appear in an aggregate type (e.g.
as the type of a struct field or vector element), and the "borrowed" type refers
to the type that would appear if it were used as a protocol method parameter
(from the client's perspective) or response tuple value (from the server's
perspective). The distinction between owned and borrowed exists in order to take
advantage of Rust’s ownership model. When making a request with a parameter of
type `T`, the [proxied function call](#protocols-client) does not need to take
ownership of `T` so the FIDL toolchain needs to generate a borrowed version of
`T`. Borrowed versions often use `&mut` since the type `T` may contain handles,
in which case the FIDL bindings zero out the handles when encoding, which
modifies the input. Using `&mut` instead of taking ownership allows callers to
reuse the input value if it does not contain handles.

|FIDL Type|Rust Type|
|--- |--- |
|`bool`|`bool`|
|`int8`|`i8`|
|`int16`|`i16`|
|`int32`|`i32`|
|`int64`|`i64`|
|`uint8`|`u8`|
|`uint16`|`u16`|
|`uint32`|`u32`|
|`uint64`|`u64`|
|`float32`|`f32`|
|`float64`|`f64`|
|`array:N`|`&mut [T; N]` *(borrowed)*<br> `[T, N]` *(owned)*|
|`vector:N`|`&[T]` *(borrowed, when T is a numeric primitive)*<br> `&mut dyn ExactSizeIterator` *(borrowed)*<br>`Vec` *(owned)*|
|`string`|`&str` *(borrowed)*<br>`String` *(owned)*|
|`request`|`fidl::endpoints::ServerEnd<PMarker>`, *where `PMarker` is the [marker type](#protocols) for this protocol.*|
|`P`|`fidl::endpoints::ClientEnd<PMarker>` *where `PMarker` is the [marker type](#protocols) for this protocol.*|
|`handle`|`fidl::Handle`|
|`handle`|The corresponding handle type is used. For example,`fidl::Channel` or `fidl::Vmo`|


#### User defined types {#user-defined-types}

Bits, enums, and tables are always referred to using their generated type `T`.
structs and unions  can be either non-nullable or nullable, and used in an owned
context or borrowed context, which means that there are four possible equivalent
Rust types. For a given `struct T` or `union T`, the types are as follows:

||owned|borrowed|
|--- |--- |--- |
|non-nullable|`T`|`&mut T`|
|nullable|`Option<T>`|`Option<&mut T>`|

### Request, response, and event parameters {#request-response-event-parameters}

When FIDL needs to generate a single Rust type representing the parameters to a
request, response, or event, such as for [result types](#protocols-client), it
uses the following rules:

* Multiple parameters are represented as a tuple of the parameter types.
* A single parameter is represented just using the parameter's type.
* An empty set of parameters is represented using the unit type `()`.

## Types {#types}

### Bits {#types-bits}

Given the [bits][lang-bits] definition:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="bits" %}
```

The FIDL toolchain generates a set of
[`bitflags`](https://fuchsia-docs.firebaseapp.com/rust/bitflags/) called
`FileMode` with flags `FileMode::Read`, `FileMode::Write`, and
`FileMode::Execute`. Bits members are emitted in camel case in the generated
Rust code.
<!-- TODO(fxbug.dev/47034): Should be UPPER_SNAKE_CASE, not CamelCase. -->

The `bitflags` struct also provides the following methods:

* `get_unknown_bits(&self) -> u16`: Returns a primitive value containing only
  the unknown members from this bits value. For [strict][lang-flexible] bits, it
  is marked `#[deprecated]` and always returns 0.
* `has_unknown_bits(&self) -> bool`: Returns whether this value contains any
  unknown bits. For [strict][lang-flexible] bits, it is marked `#[deprecated]`
  and always returns `false`.

The generated `FileMode` struct always has the complete set of [`#[derive]`
rules](#derives).

Example usage:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="bits" adjust_indentation="auto" %}
```

### Enums {#types-enums}

Given the [enum][lang-enums] definition:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="enums" %}
```

The FIDL toolchain generates a Rust `enum` using the specified underlying type,
or `u32` if none is specified:

```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u32)]
pub enum LocationType {
    Museum = 1,
    Airport = 2,
    Restaurant = 3,
}
```

With the following methods:

* `from_primitive(prim: u32) -> Option<Self>`: Returns `Some` of the enum
  variant corresponding to the discriminant value if any, and `None` otherwise.
* `into_primitive(&self) -> u32`: Returns the underlying discriminant value.
* `validate(self) -> Result<Self, u32>`: Returns `Ok` of the value if it
  corresponds to a known member, or an `Err` of the underlying primitive value
  otherwise. For [strict][lang-flexible] types, it is marked `#[deprecated]` and
  always returns `Ok`.
* `is_unknown(&self) -> bool`: Returns whether this enum is unknown. For
  [strict][lang-flexible] types, it is marked `#[deprecated]` and always returns
  `false`.

If `LocationType` is [flexible][lang-flexible], it will have the following
additional methods:

* `from_primitive_allow_unknown(prim: u32) -> Self`: Create an instance of the
  enum from a primitive value.
* `unknown() -> Self`: Return a placeholder unknown enum value. If the enum
  contains a member marked with [`[Unknown]`][unknown-attr], then the value
  returned by this method contains the value of specified unknown member.

The generated `LocationType` `enum` always has the complete set of [`#[derive]`
rules](#derives).

Example usage:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="enums_init" adjust_indentation="auto" %}
```

To provide source-compatibility, [flexible][lang-flexible] enums have an unknown
macro that should be used to match against unknown members instead of the `_`
pattern. For example, see the use of the `LocationTypeUnknown!()` macro:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="enums_flexible_match" adjust_indentation="auto" %}
```

The unknown macro acts the same as a `_` pattern, but it can be configured to
expand to an exhaustive match. This is useful for discovering missing cases.

### Structs {#types-structs}

Given the [struct][lang-structs] declaration:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="structs" %}
```

The FIDL toolchain generates a Rust `struct`:

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Color {
    pub id: u32,
    pub name: String,
}
```

Note: The Rust bindings do not support default values for FIDL struct members,
so the default value for `name` does not affect the generated Rust `struct`.

The generated `Color` `struct` follows the [`#[derive]` rules](#derives).

Example usage:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="structs" adjust_indentation="auto" %}
```

### Unions {#types-unions}

Given the [union][lang-unions] definition:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="unions" %}
```

The FIDL toolchain generates a Rust `enum`:

```rust
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum JsonValue {
    IntValue(i32),
    StringValue(String),
}
```

With the following methods:

* `validate(self) -> Result<Self, (u64, Vec<u8>)>`: Returns `Ok` of the value if
  it corresponds to a known variant, or an `Err` containing the ordinal and raw
  bytes otherwise. For [resource][lang-resource] types, the `Vec<u8>` changes to
  `fidl::UnknownData`. For [strict][lang-flexible] types, it is marked
  `#[deprecated]` and always returns `Ok`.
* `is_unknown(&self) -> bool`: Returns whether this union is unknown. Always
  returns `false` for non-flexible union types. For [strict][lang-flexible]
  types, it is marked `#[deprecated]` and always returns `false`.

If `JsonValue` is [flexible][lang-flexible], it will have the following
additional methods:

* `unknown(ordinal: u64, data: Vec<u8>) -> Self`: Create an unknown union value.
  This should only be used in tests. For [`resource`][lang-resource] types, the
  `Vec<u8>` changes to `fidl::UnknownData`.)

The generated `JsonValue` `enum` follows the [`#[derive]` rules](#derives).

Example usage:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="unions_init" adjust_indentation="auto" %}
```

#### Flexible unions and unknown variants

[Flexible][lang-flexible] unions have an extra variant generated to represent
the unknown case. This variant is considered private and should not be
referenced directly.

To provide source-compatibility, [flexible][lang-flexible] unions have an
unknown macro that should be used to match against unknown members instead of
the `_` pattern. For example, see the use of the `JsonValueUnknown!()` macro:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="unions_flexible_match" adjust_indentation="auto" %}
```

The unknown macro acts the same as a `_` pattern, but it can be configured to
expand to an exhaustive match. This is useful for discovering missing cases.

When a FIDL message containing a union with an unknown variant is decoded into
`JsonValue`, `JsonValue::validate` returns `Err(ordinal, data)` where `ordinal`
is the unknown ordinal and `data` contains the raws bytes and handles.

Encoding a union with an unknown variant writes the unknown data and the
original ordinal back onto the wire.

[Strict][lang-flexible] unions fail when decoding an unknown variant.
[Flexible][lang-flexible] unions that are [value][lang-resource] types fail when
decoding an unknown variant with handles.

### Tables {#types-tables}

Given the [table][lang-tables] definition:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="tables" %}
```

The FIDL toolchain generates a `struct` `User` with optional members:

```rust
#[derive(Debug, PartialEq)]
pub struct User {
  pub age: Option<u8>,
  pub name: Option<String>,
  pub unknown_data: Option<BTreeMap<u64, Vec<u8>>>,
  #[deprecated = "Use `..Foo::empty()` to construct and `..` to match."]
  #[doc(hidden)]
  pub __non_exhaustive: (),
}
```

And the following associated constants:

* `const EMPTY: User`: A `User` with each member initialized to `None`.

The `unknown_data` member stores a mapping from ordinal to the raw bytes of any
unknown field that was encountered during decoding. If the table is declared
as a `resource`, the map will also contain the raw handles in addition to the
bytes (i.e. the `unknown_data` member will have type
`Option<BTreeMap<u64, fidl::UnknownData>>`). If no unknown members were
encountered during decoding, the `unknown_data` field is guaranteed to be
`None` rather than `Some` of an empty map.

The `__non_exhaustive` member prevents intializing the table exhaustively, which
causes API breakage when new fields are added. Instead, you should use the
struct update syntax to fill in unspecified fields with `empty()`. For example:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="tables_init" adjust_indentation="auto" %}
```

Similarly, tables do not permit exhaustive matching. Instead, you must use the
`..` syntax to ignore unspecified fields. For example:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="tables_match" adjust_indentation="auto" %}
```

The generated `User` `struct` follows the [`#[derive]` rules](#derives).

### Derives {#derives}

When the FIDL toolchain generates a new `struct` or `enum` for a FIDL type, it
attempts to `derive` as many traits from a predefined list of useful traits as
it can, including `Debug`, `Copy`, `Clone`, etc. The complete list of traits can
be found in [Appendix A](#derived-traits).

For aggregate types, such as structs, unions, and tables, the set of derives is
determined by starting with the list of all possible derives and then removing
some based on the fields that are transitively present in the type. For example,
aggregate types that transitively contain a `vector` do not derive `Copy`, and
types that my contain a `handle` (i.e. types that are not marked as
[`resource`][lang-resource]) do not derive `Copy` and `Clone`. When in doubt,
refer to the generated code to check which traits are derived by a specific
type. See [Appendix B](#fill-derives) for implementation details.

## Protocols {#protocols}

Given a [protocol][lang-protocols]:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="protocols" %}
```

Note: The `MakeMove` method above returns a bool representing success, and a
nullable response value. This is considered un-idiomatic, you should use an [error type](#protocols-results)
instead.

The main entrypoint for interacting with `TicTacToe` is the `TicTacToeMarker`
struct, which contains two associated types:

* `Proxy`: The associated proxy type for use with async clients. In this
  example, this is a generated `TicTacToeProxy` type. Synchronous clients should
  use `TicTacToeSynchronousProxy` directly (see
  [Synchronous](#protocols-client-synchronous)), which is not stored in an
  associated type on the `TicTacToeMarker`.
* `RequestStream`: The associated [request stream](#protocol-request-stream) that servers
  implementing this protocol will need to handle. In this example, this is
  `TicTacToeRequestStream`, which is generated by FIDL.

Additionally, `TicTacToeMarker` has the following associated constants:

* `DEBUG_NAME: &’static str`: The name of the service suitable for debug
  purposes

Other code may be generated depending on the [Protocol and method
attributes](#protocol-method-attributes) applied to the protocol or its methods.

### Client {#protocols-client}

#### Asynchronous {#protocols-client-asynchronous}

For asynchronous clients, the FIDL toolchain generates a `TicTacToeProxy` struct
with the following:

Associated types:

* `TicTacToeProxy::MakeMoveResponseFut`: The `Future` type for the response of a
  two way method. This type implements `std::future::Future<Output =
  Result<(bool, Option<Box<GameState>>), fidl::Error>> + Send`.
* `TicTacToeProxy::OnOpponentMoveResponseFut`: The `Future` type for an incoming
  event. This type implements `std::future::Future<Output = Result<GameState,
  fidl::Error>> + Send`

Methods:

* `new(channel: fidl::AsyncChannel) -> TicTacToeProxy`: Create a new proxy for
  `TicTacToe`.
* `take_event_stream(&self) -> TicTacToeEventStream`: Get a `Stream` of events
  from the server end (see [Events](#protocols-events-client)).

Methods from implementing `fidl::endpoints::Proxy`:

* `from_channel(channel: fidl::AsyncChannel) -> TicTacToeProxy`: Same as
  `TicTacToeProxy::new`.
* `into_channel(self) -> Result<fidl::AsyncChannel>`: Attempt to convert the
  proxy back into a channel.
* `as_channel(&self) -> &fidl::AsyncChannel`: Get a reference to the proxy's
  underlying channel
* `is_closed(&self) -> bool`: Check if the proxy has received the `PEER_CLOSED`
  signal.
* `on_closed<'a>(&'a self) -> fuchsia_async::OnSignals<'a>`: Get a future that
  completes when the proxy receives the `PEER_CLOSED` signal.

Methods from implementing `TicTacToeProxyInterface`:

* `start_game(&self, mut start_first: bool) -> Result<(), fidl::Error>`: Proxy
  method for a fire and forget protocol method. It takes as arguments the
  request parameters and returns an empty result.
* `make_move(&self, mut row: u8, mut col: u8) -> Self::MakeMoveResponseFut`:
  Proxy method for a two way method. It takes as arguments the request
  parameters and returns a `Future` of the response.

An example of setting up an asynchronous proxy is available in the
[Rust tutorial][tutorial].

The `TicTacToeProxyInterface` trait can be useful for testing client code. For
example, if you write a function that takes `&T` as a parameter where `T:
TicTacToeProxyInterface`, you can unit test it with a fake proxy type:

```rust
use futures::future::{ready, Ready};

struct FakeTicTacToeProxy {
    move_response: (bool, Option<Box<GameState>>),
}

impl TicTacToeProxyInterface for FakeTicTacToeProxy {
    fn start_game(&self, mut start_first: bool) -> Result<(), fidl::Error> {}

    type MakeMoveResponseFut = Ready<fidl::Result<(bool, Option<Box<GameState>>)>>;
    fn make_move(&self, mut row: u8, mut col: u8) -> Self::MakeMoveResponseFut {
        ready(self.move_response.clone())
    }
}
```

#### Synchronous {#protocols-client-synchronous}

For synchronous clients of the `TicTacToe` protocols, the FIDL toolchain
generates a `TicTacToeSynchronousProxy` struct with the following methods:

* `new(channel: fidl::Channel) -> TicTacToeSynchronousProxy`: Returns a new
  synchronous proxy over the client end of a channel. The server end is assumed
  to implement the `TicTacToe` protocol.
* `into_channel(self) -> fidl::Channel`: Convert the proxy back into a channel.
* `start_game(&mut self, mut a: i64) -> Result<(), fidl::Error>`: Proxy method
  for a fire and forget method: it takes the request parameters as arguments and
  returns an empty result.
* `make_move(&mut self, mut row: u8, mut col: u8, __deadline: zx::Time) ->
  Result<(bool, Option<Box<GameState>>), fidl::Error>`: Proxy method for a two
  way method. It takes the request parameters as arguments followed by a
  deadline parameter, which dictates how long the method call will wait for a
  response (or `zx::Time::INFINITE` to block indefinitely). It returns a
  `Result` of the [response parameters](#request-response-event-parameters).

An example of setting up a synchronous proxy is available in the
[Rust tutorial][tutorial].

### Server {#protocols-server}

#### Protocol request stream {#protocol-request-stream}

To represent the stream of incoming requests to a server, the FIDL toolchain
generates a `TicTacToeRequestStream` type that implements `futures::Stream<Item
= Result<TicTacToeRequest, fidl::Error>>` as well as
`fidl::endpoints::RequestStream`. Each protocol has a corresponding request
stream type.

#### Request enum {#request-enum}

`TicTacToeRequest` is an enum representing the possible requests of the
`TicTacToe` protocol. It has the following variants:

* `StartGame { start_first: bool, control_handle: TicTacToeControlHandle }`: A
  fire and forget request, which contains the request parameters and a [control handle](#protocol-control-handle).
* `MakeMove { row: u8, col: u8, responder: TicTacToeMakeMoveResponder }`: A two
  way method request, which contains the request parameters and a
  [responder](#request-responder).

One such enum is generated for each protocol.

#### Request responder {#request-responder}

Each two way method has a corresponding generated responder type, which the
server uses to respond to a request. In this example, which only has one two way
method, the FIDL toolchain generates `TicTacToeMakeMoveResponder`, which
provides the following methods:

* `send(self, mut success: bool, mut new_state: Option<&mut GameState>) ->
  Result<(), fidl::Error>`: Sends a response.
* `send_no_shutdown_on_err(self, mut success: bool, mut new_state: Option<&mut
  GameState>) -> Result<(), fidl::Error>`: Similar to `send` but does not shut
  down the channel if an error occurs.
* `control_handle(&self) -> &TicTacToeControlHandle`: Get the underlying
  [control handle](#protocol-control-handle).
* `drop_without_shutdown(mut self)`: Drop the Responder without shutting down
  the channel.

#### Protocol control handle {#protocol-control-handle}

The FIDL toolchain generates `TicTacToeControlHandle` to encapsulate the client
endpoint of the `TicTacToe` protocol on the server side. It contains the
following methods:

* `shutdown(&self)`: Shut down the channel.
* `shutdown_with_epitaph(&self, status: zx_status::Status)`: Send an epitaph and
  then shut down the channel.
* `send_on_opponent_move(&self, mut new_state: &mut GameState) -> Result<(),
  fidl::Error>`: Proxy method for an event, which takes as arguments the event’s
  parameters and returns an empty result (see
  [Events](#protocols-events-server)).

### Events {#protocols-events}

#### Client {#protocols-events-client}

For receiving events on the client, the FIDL toolchain generates a
`TicTacToeEventStream`, which can be obtained using the `take_event_stream()`
method on the [`TicTacToeProxy`](#protocols-client-asynchronous).
`TicTacToeEventStream` implements `futures::Stream<Item = Result<TicTacToeEvent,
fidl::Error>>`.

`TicTacToeEvent` is an enum representing the possible events. It has the
following variants:

* `OnOpponentMove { new_state: GameState }`: Discriminant for the
  `TicTacToeEvent` event.

And provides the following methods:

* `into_on_opponent_move(self) -> Option<GameState>`: Return `Some` of the
  [parameters](#request-response-event-parameters) of the event, or `None` if
  the variant does not match the method call.

#### Server {#protocols-events-server}

Servers can send events by using the [control handle](#protocol-control-handle)
corresponding to the protocol. The control handle can be obtained through a
`TicTacToeRequest` received from the client. For fire and forget methods, the
control handle is available through the `control_handle` field, and for two way
methods, it is available through the `control_handle()` method on the responder.
A control handle for a protocol can also be obtained through the corresponding
request stream (in this example, `TicTacToeRequestStream`), since it implements
`fidl::endpoints::RequestStream`.

### Results {#protocols-results}

For a method with an error type:

```fidl
protocol TicTacToe {
    MakeMove(uint8 row, uint8 col) -> (GameState new_state) error MoveError;
};
```

The FIDL toolchain generates a public `TicTacToeMakeMoveResult` type alias for
`std::result::Result<GameState, MoveError>`. The rest of the bindings code for
this method is generated as if it has a single response parameter `result` of
type `TicTacToeMakeMoveResult`. The type used for a successful result follows
the [parameter type conversion rules](#request-response-event-parameters).

### Protocol composition {#protocol-composition}

FIDL does not have a concept of inheritance, and generates full code as
described above for all [composed protocols][lang-protocol-composition]. In
other words, the code generated for the following:

```fidl
protocol A {
    Foo();
};

protocol B {
    compose A;
    Bar();
};
```

Is the same as the following code:

```fidl
protocol A {
    Foo();
};

protocol B {
    Foo();
    Bar();
};
```

### Protocol and method attributes {#protocol-method-attributes}

#### Transitional

The `[Transitional]` attribute only affects the `ProxyInterface` trait, which is
sometimes used in test code. For non-test code, protocols can be transitioned on
the server side by having request handlers temporarily use a catch-all match arm
in the `Request` handler. Client code does not need to be soft transitioned
since the generated proxy will always implement all methods.

For methods annotated with the `[Transitional]` attribute,  the `ProxyInterface`
trait for [asynchronous clients](#protocols-client-asynchronous}) provides
default implementations that call `unimplemented!()`. As noted earlier, this has
no effect on the `Proxy` type, which always implements all the trait's methods.
However, it can help for soft transitions when the `ProxyInterface` trait is
used for fake proxies in client-side unit tests.

#### Discoverable

For protocols annotated with the `[Discoverable]` attribute, the Marker type
additionally implements the `fidl::endpoints::DiscoverableService` trait.

## Appendix A: Derived traits {#derived-traits}

```go
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="tools/fidl/fidlgen_rust/codegen/ir.go" region_tag="derived_traits" adjust_indentation="auto" %}
```

## Appendix B: Fill derives {#fill-derives}

The calculation of traits derivation rules is visible in
[fidlgen_rust](/tools/fidl/fidlgen_rust/codegen/ir.go):

```go
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="tools/fidl/fidlgen_rust/codegen/ir.go" region_tag="fill_derives" adjust_indentation="auto" %}
```

<!-- xrefs -->
[lang-bits]: /docs/reference/fidl/language/language.md#bits
[lang-constants]: /docs/reference/fidl/language/language.md#constants
[lang-enums]: /docs/reference/fidl/language/language.md#enums
[lang-flexible]: /docs/reference/fidl/language/language.md#strict-vs-flexible
[lang-protocol-composition]: /docs/reference/fidl/language/language.md#protocol-composition
[lang-protocols]: /docs/reference/fidl/language/language.md#protocols
[lang-resource]: /docs/reference/fidl/language/language.md#value-vs-resource
[lang-structs]: /docs/reference/fidl/language/language.md#structs
[lang-tables]: /docs/reference/fidl/language/language.md#tables
[lang-unions]: /docs/reference/fidl/language/language.md#unions
[tutorial]: /docs/development/languages/fidl/tutorials/rust
[unknown-attr]: /docs/reference/fidl/language/attributes.md#unknown
