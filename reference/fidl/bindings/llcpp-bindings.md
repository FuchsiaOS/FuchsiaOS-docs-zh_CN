# LLCPP bindings

## Libraries {#libraries}

Given the library declaration:

```fidl
library fuchsia.examples;
```

[Protocol types](#protocols) are generated in the `fuchsia_examples` namespace.
[Domain objects](#type-definitions) for this library are generated in the
`fuchsia_examples::wire` namespace, and [test scaffolding](#test-scaffolding)
is generated in the `fidl::testing` namespace.

Generated type names are transformed to follow the
[Google C++ style guide][cpp-style].

## Constants {#constants}

[Constants][lang-constants] are generated as a `constexpr`. For example, the
following constants:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="consts" %}
```

Are generated in the header file as:

```c++
constexpr uint8_t kBoardSize = 9u;
extern const char[] kName;
```

The correspondence between FIDL primitive types and C++ types is outlined in
[built-in types](#builtins). Instead of `constexpr`, strings are declared as an
`extern const char[]` in the header file, and defined in a `.cc` file.

## Fields

This section describes how the FIDL toolchain converts FIDL types to native
types in LLCPP. These types can appear as members in an aggregate type or as
parameters to a protocol method.

### Built-in types {#builtins}

The FIDL types are converted to C++ types based on the following table:

|FIDL Type|LLCPP Type|
|--- |--- |
|`bool`|`bool`, *(requires sizeof(bool) == 1)*|
|`int8`|`int8_t`|
|`int16`|`int16_t`|
|`int32`|`int32_t`|
|`int64`|`int64_t`|
|`uint8`|`uint8_t`|
|`uint16`|`uint16_t`|
|`uint32`|`uint32_t`|
|`uint64`|`uint64_t`|
|`float32`|`float`|
|`float64`|`double`|
|`array<T, N>`|`fidl::Array<T, N>`|
|`vector<T>:N`|`fidl::VectorView<T>`|
|`string`|`fidl::StringView`|
|`client_end:P` |`fidl::ClientEnd<P>`|
|`server_end:P` |`fidl::ServerEnd<P>`|
|`zx.handle`|`zx::handle`|
|`zx.handle:S`|The corresponding zx type is used whenever possible. For example, `zx::vmo` or `zx::channel`.|

Nullable built-in types do not have different generated types than their
non-nullable counterparts in LLCPP, and are omitted from the table above.

### User defined types {#user-defined-types}

In LLCPP, a user defined type (bits, enum, constant, struct, union, or table) is
referred to using the generated class or variable (see [Type
Definitions](#type-definitions)). The nullable version of a user defined type
`T` is referred to using a `fidl::ObjectView` of the generated type *except*
for unions, which simply use the generated type itself. Refer to the [LLCPP
memory guide][llcpp-allocation] for information about `ObjectView`.

## Type definitions {#type-definitions}

### Bits {#bits}

Given the [bits][lang-bits] definition:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="bits" %}
```

The FIDL toolchain generates a `FileMode` class with a static member for each
flag, as well as a `kMask` member that contains a mask of all bits members (in
this example `0b111`):

* `const static FileMode kRead`
* `const static FileMode kWrite`
* `const static FileMode kExecute`
* `const static FileMode kMask`

`FileMode` provides the following methods:

* `explicit constexpr FileMode(uint16_t)`: Constructs a value from an underlying
  primitive value, preserving any unknown bit members.
* `constexpr static cpp17::optional<FileMode> TryFrom(uint16_t value)`: Constructs
  an instance of the bits from an underlying primitive value if the value does
  not contain any unknown members, and returns `cpp17::nullopt` otherwise.
* `constexpr static FileMode TruncatingUnknown(uint16_t value)`: Constructs an
  instance of the bits from an underlying primitive value, clearing any unknown
  members.
* Bitwise operators: Implementations for the `|`, `|=`, `&`, `&=`, `^`, `^=`,
  and `~` operators are provided, allowing bitwise operations on the bits like
  `mode |= FileMode::kExecute`.
* Comparison operators `==` and `!=`.
* Explicit conversion functions for `uint16_t` and `bool`.

If `FileMode` is [flexible][lang-flexible], it will have the following
additional methods:

* `constexpr FileMode unknown_bits() const`: Returns a bits value that contains
  only the unknown members from this bits value.
* `constexpr bool has_unknown_bits() const`: Returns whether this value contains
  any unknown bits.

Note: When applying bitwise negation to bits values that contain unknown
members, the resulting bits value is only defined for the known bits.

Example usage:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="bits" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Enums {#enums}

Given the [enum][lang-enums] definition:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="enums" %}
```

The FIDL toolchain generates a C++ `enum class` using the specified underlying
type, or `uint32_t` if none is specified:

```c++
enum class LocationType : uint32_t {
    kMuseum = 1u;
    kAirport = 2u;
    kRestaurant = 3u;
};
```

Example usage:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="enums" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

#### Flexible enums {#flexible-enums}

Flexible enums are implemented as a `class` instead of an `enum class`, with the
following methods:

* `constexpr LocationType()`: Default constructor, which initializes the enum to
  an unspecified unknown value.
* `constexpr LocationType(uint32_t value)`: Explicit constructor that takes in a
  value of the underlying type of the enum.
* `constexpr bool IsUnknown()`: Returns whether the enum value is unknown.
* `constexpr static LocationType Unknown()`: Returns an enum value that is
  guaranteed to be treated as unknown. If the enum has a member annotated with
  [`[Unknown]`][unknown-attr], then the value of that member is returned. If
  there is no such member, then the underlying value of the returned enum member
  is unspecified.
* `explicit constexpr operator int32_t() const`: Converts the enum back to its
  underlying value.

The generated class contains a static member for each enum member, which are
guaranteed to match the members of the `enum class` in the equivalent
[strict][lang-flexible] enum:

* `const static LocationType kMuseum`
* `const static LocationType kAirport`
* `const static LocationType kRestaurant`

### Structs {#structs}

Given the [struct][lang-structs] declaration:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="structs" %}
```

The FIDL toolchain generates an equivalent `struct`:

```c++
struct Color {
    uint32_t id = {};
    fidl::StringView name = {};
}
```

LLCPP does not currently support default values, and instead zero-initializes
all fields of the struct.

Example usage:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="structs" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Unions {#unions}

Given the union definition:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="unions" %}
```

FIDL will generate a `JsonValue` class. `JsonValue` contains a public tag enum
class representing the possible [variants][union-lexicon]:

```c++
enum class Tag : fidl_xunion_tag_t {
  kIntValue = 2,
  kStringValue = 3,
};
```

Each member of `Tag` has a value matching its ordinal specified in the `union`
definition. Reserved fields do not have any generated code.

`JsonValue` provides the following methods:

* `JsonValue()`: Default constructor. The constructed union is initially in an
  "absent" state until a variant is set. The `WithFoo` constructors should be
  preferred whenever possible.
* `~JsonValue()`: Destructor that clears the underlying union data.
* `JsonValue(JsonValue&&)`: Default move constructor.
* `JsonValue& operator=(JsonValue&&)`: Default move assignment
* `static JsonValue WithIntValue(fidl::ObjectView<int32>)` and `static
  JsonValue WithStringValue(fidl::ObjectView<fidl::StringView>)`: Static
  constructors that directly construct a specific variant of the union.
* `bool has_invalid_tag()`: Returns `true` if the instance of `JsonValue` does
   not yet have a variant set. Calling this method without first setting the
   variant leads to an assertion error.
* `bool is_int_value() const` and `bool is_string_value() const`: Each variant
  has an associated method to check whether an instance of `JsonValue` is of
  that variant
* `const int32_t& int_value() const` and `const fidl::StringView& string_value()
  const`: Read-only accessor methods for each variant. Calling these methods
  without first setting the variant leads to an assertion error.
* `int32_t& int_value()` and `fidl::StringView& string_value()`: Mutable
  accessor methods for each variant. These methods will fail if `JsonValue` does
  not have the specified variant set
* `Tag Which() const`: returns the current [tag][union-lexicon] of the
  `JsonValue`. Calling this method without first setting the variant leads to an
  assertion error.

Example usage:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="unions" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

#### Flexible unions and unknown variants

[Flexible][lang-flexible] unions have an extra variant in the generated `Tag`
class:

```c++
  enum class Tag : fidl_xunion_tag_t {
    ... // other fields omitted
    kUnknown = ::std::numeric_limits<::fidl_union_tag_t>::max(),
  };
```

When a FIDL message containing a union with an unknown variant is decoded into
`JsonValue`, `JsonValue::Which()` will return `JsonValue::Tag::kUnknown`.

The LLCPP bindings do not store the raw bytes and handles of unknown variants.

Encoding a union with an unknown variant is not supported and will cause
an encoding failure.

### Tables {#tables}

Given the [table][lang-tables] definition:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="tables" %}
```

The FIDL toolchain generates a `User` class with the following methods:

* `User()`: Default constructor, initializes an empty table with no fields set.
* `User::Builder(fidl::AnyArena& arena)`: Builder factory.
  Returns a `fidl::WireTableBuilder<User>` that allocates the frame and members
  from the supplied arena.
* `User::ExternalBuilder(fidl::ObjectView<fidl::WireTableFrame<User>> frame)`:
  External builder factory. Returns a `fidl::WireTableExternalBuilder<User>`
  with the supplied frame. This builder requires careful, memory management but
  might occasionally be useful. _Caveat Emptor_.
* `User(User&&)`: Default move constructor.
* `~User()`: Default destructor.
* `User& operator=(User&&)`: Default move assignment.
* `bool IsEmpty() const`: Returns true if no fields are set.
* `bool has_age() const` and `bool has_name() const`: Returns whether a field is
  set.
* `const uint8_t& age() const` and `const fidl::StringView& name() const`:
  Read-only field accessor methods. Calling these methods without first setting
  the field leads to an assertion error.

In order to build a table, three additional class is generated:
`fidl::WireTableBuilder<User>`, `fidl::WireTableExternalBuilder<User>` and
`fidl::WireTableFrame<User>`.

`fidl::WireTableFrame<User>` is a container for the table's internal storage,
and is allocated separately from the builder because LLCPP maintains the object
layout of the underlying wire format. It is only use internally by builders.

`fidl::WireTableFrame<User>` has the following methods:

* `WireTableFrame()`: Default constructor.

`fidl::WireTableExternalBuilder<User>` has the following methods:

* `fidl::WireTableExternalBuilder<User> age(uint8_t)`:
  set age by inlining it into the table frame.
* `fidl::WireTableExternalBuilder<User> name(fidl::ObjectView<fidl::StringView>)`:
  set name with an already allocated value.
* `User Build()`: build and return the table object. After `Build()` is called
  the builder must be discarded.

`fidl::WireTableBuilder<User>` has all of the methods of
`fidl::WireTableExternalBuilder<User>` (but with the right return type from the
setters) and adds:

* `fidl::WireTableBuilder<User> name(std::string_view)`: set name by allocating
  a new `fidl::StringView` from the builder's arena and copying the supplied
  string into it.


Example usage:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="tables" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

In addition to assigning fields with `fidl::ObjectView`, any of the allocation
strategies described in the [tutorial][llcpp-allocation] can also be used.

Note: Tables with unknown fields will decode successfully but will fail to
encode.

### Inline layouts

The generated C++ code uses the [the name reserved by `fidlc`][anon-names] for
inline layouts.

LLCPP also generates scoped names to refer to any inline layouts that were
defined directly within a parent layout in FIDL. For example, for the FIDL:

```fidl
type Outer = struct {
  inner struct {};
};
```

The inner struct can be referred to using its globally unique name `Inner` as
well as the scoped name `Outer::Inner`. This can be useful when the top level
name is overridden using the [`@generated_name`][generated-name-attr] attribute,
for example in:

```fidl
type Outer = struct {
  inner
  @generated_name("SomeCustomName") struct {};
};
```

the inner struct can be referred to as `SomeCustomName` or `Outer::Inner`.

Another example of this is the [protocol result](#protocol-results) types: the
success and error variants of a type such as `TicTacToe_MakeMove_Result` can be
referenced as `TicTacToe_MakeMove_Result::Response` and
`TicTacToe_MakeMove_Result::Err`, respectively.

## Protocols {#protocols}

Given the [protocol][lang-protocols]:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="protocols" %}
```

Note: The `MakeMove` method above returns a bool representing success, and a
nullable response value. This is considered un-idiomatic, you should use an
[error type](#protocols-results) instead.

FIDL will generate a `TicTacToe` class, which acts as an entry point for types
and classes that both clients and servers will use to interact with this
service. The members of this class are described in individual subsections in
the rest of this section.

### Typed channel endpoints {#typed-channels}

LLCPP sends and receives FIDL protocol messages over the
[Zircon channel][zircon-channel] transport, which carry arbitrary blobs of bytes
and handles. Rather than exposing raw endpoints, for instance `zx::channel`, the
API exposes three templated endpoint classes:

* `fidl::ClientEnd<TicTacToe>`: the client endpoint of the `TicTacToe` protocol;
  it owns a `zx::channel`. Client bindings that require exclusive ownership of
  the channel would consume this type. For example, a
  `fidl::WireClient<TicTacToe>` may be constructed from a
  `fidl::ClientEnd<TicTacToe>`, also known as "binding the channel to the
  message dispatcher".
* `fidl::UnownedClientEnd<TicTacToe>`: an unowned value borrowing some client
  endpoint of the `TicTacToe` protocol. Client APIs that do not require
  exclusive ownership of the channel would take this type. An `UnownedClientEnd`
  may be derived from a `ClientEnd` of the same protocol type by calling
  `borrow()`. The borrowed-from endpoint may be `std::move`-ed within the same
  process, but cannot be dropped or transferred out-of-process, while there are
  unowned borrows to it.
* `fidl::ServerEnd<TicTacToe>`: the server endpoint of the `TicTacToe` protocol;
  it owns a `zx::channel`. Server bindings that require exclusive ownership of
  the channel would consume this type. For example, a
  `fidl::ServerEnd<TicTacToe>` may be provided to `fidl::BindServer<TicTacToe>`
  among other parameters to create a server binding.

There is no `UnownedServerEnd` as it is not yet needed to safely implement the
current set of features.

A pair of client and server endpoint may be created using the
`::fidl::CreateEndpoints<TicTacToe>` library call. In a protocol request
pipelining scenario, one can immediately start performing operations on the
client endpoint after `std::move()`-ing the server endpoint to the remote
server.

See the class documentation on these types for more details.

### Request and response structs {#request-response-structs}

FIDL generates a type for each request, response, and event in the protocol by
treating the parameters as struct fields. For example, the `MakeMoveRequest` is
generated as if it were a struct with two fields: `uint8 row`, and `uint8 col`,
providing the same generated code API as [regular structs](#structs):

```c++
struct MakeMoveRequest final {
    uint8_t row;
    uint8_t col;
}
```

For this example, the following types are generated:

* `fidl::WireRequest<TicTacToe::StartGame>`
* `fidl::WireRequest<TicTacToe::MakeMove>`
* `fidl::WireResponse<TicTacToe::MakeMove>`
* `fidl::WireEvent<TicTacToe::OnOpponentMove>`

The naming scheme for requests is `[Method]Request`. The naming scheme for
responses is `[Method]Response`. The naming scheme for events is `[Method]Event`.

Any empty request, response, or event is represented by a `nullptr`.

### Client {#client}

The LLCPP bindings provides multiple ways to interact with a FIDL protocol as a
client:

* `fidl::WireClient<TicTacToe>`: This class exposes thread-safe APIs for
  outgoing asynchronous and synchronous calls as well as asynchronous event
  handling. It owns the client end of the channel. An `async_dispatcher_t*` is
  required to support the asynchronous APIs as well as event and error handling.
  It must be used with a single-threaded dispatcher. Objects of this class must
  be bound to the client endpoint and destroyed on the same thread that is
  running the dispatcher. This is the recommended variant for most use cases,
  except for those where an `async_dispatcher_t` cannot be used or when the
  client needs to be moved between threads.
* `fidl::WireSharedClient<TicTacToe>`: This class has less opinions on threading
  models compared to `WireClient`, but requires a two-phase shutdown pattern to
  prevent use-after-frees. Objects of this class may be destroyed on an
  arbitrary thread. It also supports use with a multi-threaded dispatcher. For
  more details, see [LLCPP threading guide][llcpp-threading-guide].
* `fidl::WireSyncClient<TicTacToe>`: This class exposes purely synchronous APIs
  for outgoing calls as well as for event handling. It owns the client end of
  the channel.
* `fidl::WireCall<TicTacToe>`: This class is identical to `WireSyncClient`
  except that it does not have ownership of the client end of the channel.
  `WireCall` may be preferable to `WireSyncClient` when migrating code from the
  C bindings to the LLCPP bindings, or when implementing C APIs that take raw
  `zx_handle_t`s.

#### WireClient {#async-client}

<!-- TODO(fxbug.dev/58672) fidl::WireClient should be covered by generated docs -->

`fidl::WireClient` is thread-safe and supports both synchronous and asynchronous
calls as well as asynchronous event handling.

##### Creation

A client is created with a client-end `fidl::ClientEnd<P>` to the protocol `P`,
an `async_dispatcher_t*`, and an optional pointer to an
[`WireAsyncEventHandler`](#async-event-handlers) that defines the methods to be
called when a FIDL event is received or when the client is unbound. If the
virtual method for a particular event is not overridden, the event is ignored.

```cpp
class EventHandler : public fidl::WireAsyncEventHandler<TicTacToe> {
 public:
  EventHandler() = default;

  void OnOpponentMove(fidl::WireEvent<OnOpponentMove>* event) override {
    /* ... */
  }

  void on_fidl_error(fidl::UnbindInfo unbind_info) override { /* ... */ }
};

fidl::ClientEnd<TicTacToe> client_end = /* logic to connect to the protocol */;
EventHandler event_handler;
fidl::WireClient<TicTacToe> client;
client.Bind(std::move(client_end), dispatcher, &event_handler);
```

The binding may be torn down automatically in case of the server-end being
closed or due to an invalid message being received from the server. You may also
actively tear down the bindings by destroying the client object.

##### Outgoing FIDL methods

You can invoke outgoing FIDL APIs through the `fidl::WireClient` instance.
Dereferencing a `fidl::WireClient` provides access to the following methods:

* For `StartGame` (fire and forget):

    * `fidl::Status StartGame(bool start_first)`: Managed variant of a fire and
      forget method.

* For `MakeMove` (two way):

    * `[...] MakeMove(uint8_t row, uint8_t col)`: Managed variant of an
      asynchronous two way method. It returns an internal type that must be used
      to register the asynchronous continuation for receiving the result, such
      as a callback. See [specifying asynchronous
      continuation][specifying-asynchronous-continuation]. The continuation will
      be executed on a dispatcher thread unless the dispatcher is shutting down.

`fidl::WireClient::buffer` provides access to the following methods:

* `fidl::Status StartGame(bool start_first)`: Caller-allocated variant of a fire
  and forget method.
* `[...] MakeMove(uint8_t row, uint8_t col)`: Asynchronous, caller-allocated
  variant of a two way method. It returns the same internal type as that from
  the managed variant.

`fidl::WireClient::sync` provides access to the following methods:

* `fidl::WireResult<MakeMove> MakeMove(uint8_t row, uint8_t col)`: Synchronous,
  managed variant of a two way method. The same method exists on
  `WireSyncClient`.

##### Specifying asynchronous continuation {#specifying-asynchronous-continuation}

See the corresponding C++ [documentation comments][wire-thenable-impl].

The continuation is called with a result object either representing a
successfully decoded response or an error. This is useful when the user needs to
propagate errors for each FIDL call to their originators. For example, a server
may need to make another FIDL call while handling an existing FIDL call, and
need to fail the original call in case of errors.

The are a few methods on the returned object from a two way call:

* `Then`: takes a callback, and invokes the callback at most once until the
  client goes away.

* `ThenExactlyOnce`: when passed a callback, the callback is executed exactly
  once, either when the call succeeds or fails. However, because the callbacks
  are invoked asynchronously, be ware of [use-after-free bugs when destroying a
  client][result-callback-use-after-free]: the objects captured by the callback
  may not be valid.

* `ThenExactlyOnce` may also take a response context when control over
  allocation is desired. `TicTacToe` has only one response context,
  `fidl::WireResponseContext<TicTacToe::MakeMove>`, which has pure virtual
  methods that should be overridden to handle the result of the call:

```c++
virtual void OnResult(fidl::WireUnownedResult<MakeMove>& result) = 0;
```

`OnResult` is called with a result object either representing a successfully
decoded response or an error. You are responsible for ensuring that the response
context object outlives the duration of the entire async call, since the
`fidl::WireClient` borrows the context object by address to avoid implicit
allocation.

##### Centralized error handler {#central-error-handler}

When the binding is torn down due to an error,
`fidl::WireAsyncEventHandler<TicTacToe>::on_fidl_error` will be invoked from the
dispatcher thread with the detailed reason. When the error is dispatcher
shutdown, `on_fidl_error` will be invoked from the thread that is calling
dispatcher shutdown. It is recommended to put any central logic for logging or
releasing resources in that handler.

#### WireSyncClient {#sync-client}

`fidl::WireSyncClient<TicTacToe>` is a synchronous client which provides the
following methods:

* `explicit WireSyncClient(fidl::ClientEnd<TicTacToe>)`: Constructor.
* `~WireSyncClient()`: Default destructor.
* `WireSyncClient(&&)`: Default move constructor.
* `WireSyncClient& operator=(WireSyncClient&&)`: Default move assignment.
* `const fidl::ClientEnd<TicTacToe>& client_end() const`: Returns the underlying
  [client endpoint](#typed-channels).
* `fidl::Status StartGame(bool start_first)`: Managed variant of a fire and
  forget method call. Buffer allocation for requests are entirely handled within
  this function.
* `fidl::WireResult<TicTacToe::MakeMove> MakeMove(uint8_t row, uint8_t col)`:
  Managed variant of a two way method call, which takes the parameters as
  arguments and returns a `WireResult` object. Buffer allocation for requests
  and responses are entirely handled within this function. The bindings
  calculate a safe buffer size specific to this call at compile time based on
  FIDL wire-format and maximum length constraints. The buffers are allocated on
  the stack if they fit under 512 bytes, or else on the heap. See
  [WireResult](#result) for details on buffer management.
* `fidl::Status HandleOneEvent(SyncEventHandler& event_handler)`: Blocks to
  consume exactly one event from the channel. See [Events](#events).

`fidl::WireSyncClient<TicTacToe>::buffer` provides the following methods:

* `fidl::WireUnownedResult<TicTacToe::StartGame> StartGame(bool start_first)`:
  Caller-allocated variant of a fire and forget call, which takes in backing
  storage for the request buffer passed as the argument to `buffer`, as well as
  request parameters, and returns an `fidl::WireUnownedResult`.

* `fidl::WireUnownedResult<TicTacToe::MakeMove> MakeMove(uint8_t row, uint8_t
  col)`: Caller-allocated variant of a two way method, which requests both the
  space for encoding the request and the space for receiving the response from
  the same memory resource that is passed to the `buffer` method.

Note that each method has both an owned and caller-allocated variant. In brief,
the owned variant of each method handles memory allocation for requests and
responses, whereas the caller-allocated variant allows the user to provide the
buffer themselves. The owned variant is easier to use, but may result in extra
allocation.

#### WireCall {#client-call}

`fidl::WireCall<TicTacToe>` provides similar methods to those found in
`WireSyncClient`, with the only difference being that `WireCall` can be
constructed with a `fidl::UnownedClientEnd<TicTacToe>` i.e. it borrows the
client endpoint:

* `fidl::WireResult<StartGame> StartGame(bool start_first)`: Owned variant of
  `StartGame`.
* `fidl::WireResult<MakeMove> MakeMove(uint8_t row, uint8_t col)`: Owned variant
  of `MakeMove`.

`fidl::WireCall<TicTacToe>(client_end).buffer` provides the following methods:

* `fidl::WireUnownedResult<StartGame> StartGame(bool start_first)`:
  Caller-allocated variant of `StartGame`.
* `fidl::WireUnownedResult<MakeMove> MakeMove(uint8_t row, uint8_t col);`:
  Caller-allocated variant of `MakeMove`.

#### Result, WireResult, and WireUnownedResult {#result}

The managed variants of each method of `WireSyncClient` and `WireCall` all
return a `fidl::WireResult<Method>` type, whereas the caller-allocating variants
all return an `fidl::WireUnownedResult<Method>`. Fire and forget methods on
`fidl::WireClient` return a `fidl::Status`. These types define the same set of
methods:

*   `zx_status status() const` returns the transport status. it returns the
    first error encountered during (if applicable) linearizing, encoding, making
    a call on the underlying channel, and decoding the result. If the status is
    `ZX_OK`, the call has succeeded, and vice versa.
*   `fidl::Reason reason() const` returns details about which operation failed,
    when `status()` is not `ZX_OK`. For example, if encoding failed, `reason()`
    will return `fidl::Reason::kEncodeError`. `reason()` should not be called
    when status is `ZX_OK`.
*   `const char* error_message() const` contains a brief error message when
    status is not `ZX_OK`. Otherwise, returns `nullptr`.
*   **(only for WireResult and WireUnownedResult for two-way calls)** `T* Unwrap()`
    returns a pointer to the [response struct](#request-response-structs). For
    `WireResult`, the pointer points to memory owned by the result object. For
    `WireUnownedResult`, the pointer points to the caller-provided buffer.
    `Unwrap()` should only be called when the status is `ZX_OK`.

Additionally, `WireResult` and `WireUnownedResult` for two-way calls will
implement dereference operators that return the response struct itself.
This allows code such as:

```cpp
fidl::WireResult result = client.sync()->MakeMove(0, 0);
auto* response = result.Unwrap();
bool success = response->success;
```

To be simplified to:

```cpp
fidl::WireResult result = client.sync()->MakeMove(0, 0);
bool success = result->success;
```

> `WireResult<Method>` manages ownership of all buffer and handles, while
> `::Unwrap()` returns a view over it. Therefore, this object must outlive any
> references to the unwrapped response.

##### Allocation strategy And move semantics

`WireResult` stores the response buffer inline if the message is guaranteed to
fit under 512 bytes. Since the result object is usually instantiated on the
caller's stack, this effectively means the response is stack-allocated when it
is reasonably small. If the maximal response size exceeds 512 bytes,
`WireResult` instead contains a heap-allocated buffer.

Therefore, `std::move()` on `WireResult` is not supported. The content has to be
copied if the buffer is inline, and pointers to out-of-line objects have to be
updated to locations within the destination object, these are surprising
overheads for a move operation that is commonly understood to be low cost.

If the result object need to be passed around multiple function calls, consider
pre-allocating a buffer in the outer-most function and use the caller-allocating
flavor.

### Server

Implementing a server for a FIDL protocol involves providing a concrete
implementation of `TicTacToe`.

The generated `fidl::WireServer<TicTacToe>` class has pure virtual methods
corresponding to the method calls defined in the FIDL protocol. Users implement
a `TicTacToe` server by providing a concrete implementation of
`fidl::WireServer<TicTacToe>`, which has the following pure virtual methods:

* `virtual void StartGame(StartGameRequestView request, StartGameCompleter::Sync&
  completer)`
* `virtual void MakeMove(MakeMoveRequestView request, MakeMoveCompleter::Sync&
  completer)`

Refer to the [example LLCPP server][llcpp-server-example] for how to bind and
set up a server implementation.

The LLCPP bindings also provide functions for manually dispatching a message
given an implementation, `fidl::WireDispatch<TicTacToe>`:

* `void fidl::WireDispatch<TicTacToe>(fidl::WireServer<TicTacToe>* impl,
  fidl::IncomingMessage&& msg, ::fidl::Transaction* txn)`: Dispatches the
  incoming message. If there is no matching handler, it closes all handles in
  the message and notifies `txn` of an error.

#### Requests {#server-requests}

The request is provided as the first argument of each generated FIDL method
handler. This a view of the request (a pointer). All the request arguments are
accessed using the arrow operator and the argument name.

For example:

* `request->start_first`
* `request->row`

See [LLCPP memory guide][llcpp-allocation] for notes on request lifetime.

#### Completers {#server-completers}

A completer is provided as the last argument of each generated FIDL method
handler, after all the FIDL request parameters for that method. The completer
classes capture the various ways one can complete a FIDL transaction, e.g. by
sending a reply, closing the channel with an epitaph, etc, and come in both
synchronous and asynchronous versions (though the `::Sync` class is provided as
an argument by default). In this example, this completers are:

* `fidl::WireServer<TicTacToe>::StartGameCompleter::Sync`
* `fidl::WireServer<TicTacToe>::StartGameCompleter::Async`
* `fidl::WireServer<TicTacToe>::MakeMoveCompleter::Sync`
* `fidl::WireServer<TicTacToe>::MakeMoveCompleter::Async`

All completer classes provide the following methods:

* `void Close(zx_status_t status)`: Close the channel and send `status` as the
  epitaph.

In addition, two way methods will provide two versions of a `Reply` method for
replying to a response: a managed variant and a caller-allocating variant. These
correspond to the variants present in the [client API](#client). For example,
both `MakeMoveCompleter::Sync` and `MakeMoveCompleter::Async` provide the
following `Reply` methods:

* `::fidl::Status Reply(bool success, fidl::ObjectView<GameState> new_state)`
* `::fidl::Status Reply(fidl::BufferSpan _buffer, bool success,
  fidl::ObjectView<GameState> new_state)`

Because the status returned by Reply is identical to the unbinding status, it
can be safely ignored.

Finally, sync completers for two way methods can be converted to an async
completer using the `ToAsync()` method. Async completers can out-live the scope
of the handler by e.g. moving it into a lambda capture (see [LLCPP
tutorial][llcpp-async-example] for example usage), allowing the server to
respond to requests asynchronously. The async completer has the same methods for
responding to the client as the sync completer.

Note: Each `Completer` object must only be accessed by one thread at a time.
Simultaneous access from multiple threads will result in a crash.

##### Parallel message handling

By default, messages from a single binding are handled sequentially, i.e. a
single thread attached to the dispatcher (run loop) is woken up if necessary,
reads the message, executes the handler, and returns back to the dispatcher. The
`::Sync` completer provides an additional API, `EnableNextDispatch()`, which may
be used to selectively break this restriction. Specifically, a call to this API
will enable another thread waiting on the dispatcher to handle the next message
on the binding while the first thread is still in the handler. Note that
repeated calls to `EnableNextDispatch()` on the same `Completer` are idempotent.

Note: This use-case is currently possible only using the
[lib/fidl](/sdk/lib/fidl/llcpp) bindings.

```cpp
void DirectedScan(int16_t heading, ScanForPlanetsCompleter::Sync& completer) override {
  // Suppose directed scans can be done in parallel. It would be suboptimal to block one scan until
  // another has completed.
  completer.EnableNextDispatch();
  fidl::VectorView<Planet> discovered_planets = /* perform a directed planet scan */;
  completer.Reply(std::move(discovered_planets));
}
```

### Caller-allocated methods

A number of the APIs above provide owned and caller-allocated variants of
generated methods.

The caller-allocated variant defers all memory allocation responsibilities to
the caller. The type `fidl::BufferSpan` references a buffer address and size. It
will be used by the bindings library to construct the FIDL request, hence it
must be sufficiently large. The method parameters (e.g. `heading`) are
*linearized* to appropriate locations within the buffer. There are a number of
ways to create the buffer:

```cpp
// 1. On the stack
using StartGame = TicTacToe::StartGame;
fidl::SyncClientBuffer<StartGame> buffer;
auto result = client.buffer(buffer.view())->StartGame(true);

// 2. On the heap
auto buffer = std::make_unique<fidl::SyncClientBuffer<StartGame>>();
auto result = client.buffer(buffer->view())->StartGame(true);

// 3. Some other means, e.g. thread-local storage
constexpr uint32_t buffer_size = fidl::SyncClientMethodBufferSizeInChannel<StartGame>();
uint8_t* buffer = allocate_buffer_of_size(buffer_size);
fidl::BufferSpan buffer_span(/* data = */buffer, /* capacity = */request_size);
auto result = client.buffer(buffer_span)->StartGame(true);

// Check the transport status (encoding error, channel writing error, etc.)
if (result.status() != ZX_OK) {
  // Handle error...
}

// Don't forget to free the buffer at the end if approach #3 was used...
```

> When the caller-allocating flavor is used, the `result` object borrows the
> request and response buffers (hence its type is under `WireUnownedResult`).
> Make sure the buffers outlive the `result` object.
> See [WireUnownedResult](#result).

Note: buffers passed to the bindings must be aligned to 8 bytes. The
`fidl::SyncClientBuffer` helper class does this automatically. For an
asynchronous client, use `fidl::AsyncClientBuffer`. Failure to align would
result in a run-time error.

### Events {#events}

In LLCPP, events can be handled asynchronously or synchronously, depending
on the type of [client](#client) being used.

#### Async client {#async-event-handlers}

When using a `fidl::WireClient`, events can be handled asynchronously by passing
the class a `fidl::WireAsyncEventHandler<TicTacToe>*`. The
`WireAsyncEventHandler` class has the following members:

* `virtual void OnOpponentMove(fidl::WireEvent<OnOpponentMove>* event) {}`:
  handler for the OnOpponentMove event (one method per event).

* `virtual on_fidl_error(::fidl::UnbindInfo info) {}`: method called when the
  client encounters a terminal error.

To be able to handle events and errors, a class that inherits from
`fidl::WireAsyncEventHandler<TicTacToe>` must be defined.

#### Sync client {#sync-event-handlers}

In `WireSyncClient`, events are handled synchronously by calling
a `HandleOneEvent` function and passing it a
`fidl::WireSyncEventHandler<TicTacToe>`.

`WireSyncEventHandler` is a class that contains a pure virtual method for each
event. In this example, it consists of the following member:

* `virtual void OnOpponentMove(fidl::WireEvent<TicTacToe::OnOpponentMove>*
  event) = 0`: The handle for the OnOpponentMove event.

To be able to handle events, a class that inherits from `WireSyncEventHandler`
must be defined. This class must define the virtual methods for all the events
in the protocol. Then an instance of this class must be created.

There are two ways to handle one event. Each one use an instance of the user
defined event handler class:

* `::fidl::Status fidl::WireSyncClient<TicTacToe>::HandleOneEvent(
       SyncEventHandler& event_handler)`: A bound version for sync clients.
* `::fidl::Status fidl::WireSyncEventHandler<TicTacToe>::HandleOneEvent(
       fidl::UnownedClientEnd<TicTacToe> client_end)`: An unbound version that
  uses an `fidl::UnownedClientEnd<TicTacToe>` to handle one event for a specific
  handler.

For each call to `HandleOneEvent`, the method waits on the channel for exactly
one incoming message. Then the message is decoded. If the result is
`fidl::Status::Ok()` then exactly one virtual method has been called. Otherwise,
no virtual method has been called and the status indicates the error.

If the handlers are always the same (from one call to `HandleOneEvent` to the
other), the `WireSyncEventHandler` object should be constructed once and used
each time you need to call `HandleOneEvent`.

If an event is marked as transitional, then a default implementation is
provided, which causes `HandleOneEvent` to return an error upon receiving a
transitional event that is not handled by the user.

#### Server

`fidl::WireSendEvent` is used to send events from the server side. There are two
overloads:

* `fidl::WireSendEvent(const fidl::ServerBindingRef<Protocol>& binding_ref)`
  to send events over a server binding reference.
* `fidl::WireSendEvent(const fidl::ServerEnd<Protocol>& endpoint)`
  to send events over an endpoint.

##### Sending events using a server binding object {#bound-event-sending}

When binding a server implementation to a channel, `fidl::BindServer` returns a
`fidl::ServerBindingRef<Protocol>`, which is the means by which you may interact
safely with a server binding.

Calling `fidl::WireSendEvent` with a binding reference returns an interface to
send events.

The event sender interface contains methods for sending each event. As a
concrete example, the event sender interface for `TicTacToe` provides the
following methods:

* `fidl::Status OnOpponentMove(GameState new_state)`: Managed flavor.

Calling `.buffer(...)` returns a similar interface for the caller-allocating
flavor, allocating encoding buffers from the memory resource passed to
`.buffer`, analogous to the [client API](#client) as well as the [server
completers](#server-completers).

##### Sending events with a ServerEnd object

A server endpoint by itself is represented by `fidl::ServerEnd<Protocol>`.

[Sending events using a server binding object](#bound-event-sending) is the
standard approach to sending events while the server endpoint is bound to an
implementation. However, there may be occasions which call for sending events
on a `fidl::ServerEnd<TicTacToe>` object directly, without setting up a server
binding.

`fidl::WireSendEvent` takes a constant reference to `fidl::ServerEnd<Protocol>`.
It does not support `zx::unowned_channel`, to reduce the chances of using an
endpoint after the handle has been closed elsewhere.

### Results {#protocols-results}

Given a method:

```fidl
protocol TicTacToe {
    MakeMove(struct {
      row uint8;
      col uint8;
    }) -> (struct {
      new_state GameState;
    }) error MoveError;
};
```

FIDL will generate convenience methods on the [completers](#server-completers)
corresponding to methods with an error type. Depending on the Reply "variant",
the completer will have `ReplySuccess`, `ReplyError`, or both methods to respond
directly with the success or error data, without having to construct a union.

For the managed flavor, both methods are available:

* `void ReplySuccess(GameState new_state)`
* `void ReplyError(MoveError error)`

Since `ReplyError` doesn't require heap allocation, only `ReplySuccess` exists
for the caller-allocated flavor:

* `void ReplySuccess(fidl::BufferSpan _buffer, GameState new_state)`

Note that the passed in buffer is used to hold the *entire* response, not just
the data corresponding to the success variant.

The regularly generated `Reply` methods are available as well:

* `void Reply(TicTacToe_MakeMove_Result result)`: Owned variant.
* `void Reply(fidl::BufferSpan _buffer, TicTacToe_MakeMove_Result result)`:
  Caller-allocated variant.

The owned and caller-allocated variant use a parameter of
`TicTacToe_MakeMove_Result`, which is generated as a [union](#unions) with two
variants: `Response`, which is a `TicTacToe_MakeMove_Response`, and `Err`, which
is a `MoveError`. `TicTacToe_MakeMove_Response` is generated as a
[struct](#structs) with the response parameters as its fields. In this case, it
has a single field `new_state`, which is a `GameState`.

### Protocol composition {#protocol-composition}

FIDL does not have a concept of inheritance, and generates full code as
described above for all [composed protocols][lang-protocol-composition]. In
other words, the code generated for

```fidl
protocol A {
    Foo();
};

protocol B {
    compose A;
    Bar();
};
```

Provides the same API as the code generated for:

```fidl
protocol A {
    Foo();
};

protocol B {
    Foo();
    Bar();
};
```

The generated code is identical except for the method ordinals.

### Protocol and method attributes {#protocol-and-method-attributes}

#### Transitional

For protocol methods annotated with the
[`@transitional`](/docs/reference/fidl/language/attributes.md#transitional)
attribute, the `virtual` methods on the protocol class come with a default
`Close(ZX_NOT_SUPPORTED)` implementation. This allows implementations of the
protocol class with missing method overrides to compile successfully.

#### Discoverable

A protocol annotated with the
[`@discoverable`](/docs/reference/fidl/language/attributes.md#discoverable)
attribute causes the FIDL toolchain to generate an additional `static const char
Name[]` field on the protocol class, containing the full protocol name.

## Persistence, and standalone use of the FIDL wire format

Standalone use of the FIDL wire format, such as encoding and decoding individual
FIDL domain objects, are not yet supported (fxbug.dev/82681).

## Test scaffolding {#test-scaffolding}

The FIDL toolchain also generates a file suffixed with `_test_base.h` that
contains convenience code for testing FIDL client and server implementations. To
use these headers, depend on the generated test scaffolding library with a
`_testing` suffix (`my_library_llcpp_testing` instead of `my_library_llcpp`).

### Server test base

The test base header contains a class for each protocol that provides stub
implementations for each of the class’ methods, making it possible to implement
only the methods that are used during testing. These classes are template
specializations of `fidl::testing::WireTestBase<Protocol>` where `Protocol` is
the FIDL protocol that is stubbed (e.g. for protocol
`games.tictactoe/TicTacToe`, the test base is
`fidl::testing::WireTestBase<games_tictactoe::TicTacToe>`).

For the same `TicTacToe` protocol listed above, generated test base subclasses
`fidl::WireServer<TicTacToe>` (see [Protocols](#protocols)), offering the
following methods:

* `virtual ~WireTestBase() = default`: Destructor.
* `virtual void NotImplemented_(const std::string& name, ::fidl::CompleterBase&
  completer) = 0`: Pure virtual method that is overridden to define behavior for
  unimplemented methods.

The test base provides an implementation for the virtual protocol methods
`StartGame` and `MakeMove`, which are implemented to just call
`NotImplemented_("StartGame", completer)` and `NotImplemented_("MakeMove",
completer)`, respectively.

### Synchronous event handler test base

The test base header contains a class for each protocol that provides stub
implementations for each of the class’ events, making it possible to implement
only the events that are used during testing. Similar to the server test base,
these classes are template specializations of
`fidl::testing::WireSyncEventHandlerTestBase<Protocol>` where `Protocol` is the
FIDL protocol that is stubbed.

For the same `TicTacToe` protocol listed above, generated test base subclasses
`fidl::WireSyncEventHandler<TicTacToe>` (see [Protocols](#protocols)), offering
the following events:

* `virtual ~WireSyncEventHandlerTestBase() = default`: Destructor.
* `virtual void NotImplemented_(const std::string& name) = 0`: Pure virtual
  method that is overridden to define behavior for unimplemented events.

The test base provides an implementation for the virtual protocol events
`OnOpponentMove`, which is implemented to just call
`NotImplemented_("OnOpponentMove")`.


<!-- xrefs -->
[anon-names]: /docs/reference/fidl/language/language.md#inline-layouts
[cpp-style]: https://google.github.io/styleguide/cppguide.html#Naming
[generated-name-attr]: /docs/reference/fidl/language/attributes.md#generated-name
[llcpp-allocation]: /docs/development/languages/fidl/tutorials/llcpp/topics/memory-ownership.md
[llcpp-async-example]: /docs/development/languages/fidl/tutorials/llcpp/topics/async-completer.md
[llcpp-threading-guide]: /docs/development/languages/fidl/tutorials/llcpp/topics/threading.md
[llcpp-tutorial]: /docs/development/languages/fidl/tutorials/llcpp
[llcpp-server-example]: /examples/fidl/llcpp/server
[lang-constants]: /docs/reference/fidl/language/language.md#constants
[lang-bits]: /docs/reference/fidl/language/language.md#bits
[lang-enums]: /docs/reference/fidl/language/language.md#enums
[lang-flexible]: /docs/reference/fidl/language/language.md#strict-vs-flexible
[lang-structs]: /docs/reference/fidl/language/language.md#structs
[lang-tables]: /docs/reference/fidl/language/language.md#tables
[lang-unions]: /docs/reference/fidl/language/language.md#unions
[lang-resource]: /docs/reference/fidl/language/language.md#value-vs-resource
[lang-protocols]: /docs/reference/fidl/language/language.md#protocols
[lang-protocol-composition]: /docs/reference/fidl/language/language.md#protocol-composition
[result-callback-use-after-free]: /docs/development/languages/fidl/tutorials/llcpp/topics/threading.md#additional_use-after-free_risks_with_thenexactlyonce
[specifying-asynchronous-continuation]: #specifying-asynchronous-continuation
[union-lexicon]: /docs/reference/fidl/language/lexicon.md#union-terms
[unknown-attr]: /docs/reference/fidl/language/attributes.md#unknown
[wire-thenable-impl]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/fidl/llcpp/include/lib/fidl/llcpp/internal/thenable.h;l=34?q=wirethenable&ss=fuchsia%2Ffuchsia
[zircon-channel]: /docs/reference/kernel_objects/channel.md
