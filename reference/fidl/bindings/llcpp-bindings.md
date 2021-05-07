# LLCPP bindings

## Libraries {#libraries}

Given the library declaration:

```fidl
library fuchsia.examples;
```

Bindings code for this library is generated in the `fuchsia_examples`
namespace, and [test scaffolding](#test-scaffolding) is generated in the
`fuchsia::examples::testing` namespace.

## Constants {#constants}

[Constants][lang-constants] are generated as a `constexpr`. For example, the
following constants:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="consts" %}
```

Are generated in the header file as:

```c++
constexpr uint8_t BOARD_SIZE = 9u;
extern const char[] NAME;
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
|`array<T>:N`|`fidl::Array<T, N>`|
|`vector<T>:N`|`fidl::VectorView<T>`|
|`string`|`fidl::StringView`|
|`P`, where `P` is a protocol |`fidl::ClientEnd<P>`|
|`request<P>` |`fidl::ServerEnd<P>`|
|`handle`|`zx::handle`|
|`handle:S`|The corresponding zx type is used whenever possible. For example, `zx::vmo` or `zx::channel`.|

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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="bits" %}
```

The FIDL toolchain generates a `FileMode` class with a static member for each
flag, as well as a `kMask` member that contains a mask of all bits members (in
this example `0b111`):

* `const static FileMode READ`
* `const static FileMode WRITE`
* `const static FileMode EXECUTE`
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
  `mode |= FileMode::EXECUTE`.
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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="bits" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Enums {#enums}

Given the [enum][lang-enums] definition:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="enums" %}
```

The FIDL toolchain generates a C++ `enum class` using the specified underlying
type, or `uint32_t` if none is specified:

```c++
enum class LocationType : uint32_t {
    MUSEUM = 1u;
    AIRPORT = 2u;
    RESTAURANT = 3u;
};
```

Example usage:

```c++
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="enums" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
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

* `const static LocationType MUSEUM`
* `const static LocationType AIRPORT`
* `const static LocationType RESTAURANT`

### Structs {#structs}

Given the [struct][lang-structs] declaration:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="structs" %}
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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="structs" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Unions {#unions}

Given the union definition:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="unions" %}
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
  "invalid" state until a variant is set. The `WithFoo` constructors should be
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
* `void set_int_value(fidl::ObjectView<int32_t> value)` and `void
  set_string_value(fidl::ObjectView<fidl::StringView>&& value)`: Setter
  methods for each variant. These setters will overwrite the previously selected
  member, if any.
* `Tag which() const`: returns the current [tag][union-lexicon] of the
  `JsonValue`. Calling this method without first setting the variant leads to an
  assertion error.

Example usage:

```c++
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="unions" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
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
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="tables" %}
```

The FIDL toolchain `User` class with the following methods:

* `User()`: Default constructor, initializes with all fields unset.
* `User(::fidl::AnyAllocator& allocator)`: Constructor which allocates the frame but with all
fields unset.
* `User(User&&)`: Default move constructor.
* `~User()`: Default destructor.
* `User& operator=(User&&)`: Default move assignment.
* `bool IsEmpty() const`: Returns true if no fields are set.
* `bool has_age() const` and `bool has_name() const`: Returns whether a field is
  set.
* `const uint8_t& age() const` and `const fidl::StringView& name() const`:
  Read-only field accessor methods. Calling these methods without first setting
  the field leads to an assertion error.
* `uint8_t& mutable_age()` and `fidl::StringView& mutable_name()`: Mutable field accessor
  methods. Calling these methods without first setting the variant leads to an
  assertion error.
* `void set_age(::fidl::ObjectView<uint8>)`: set age an already allocated value.
* `void set_age(::fidl::AnyAllocator&, uint8_t)`: set age with the given value. The allocator is
  used to allocate the storage.
* `void set_age(std::nullptr_t)`: unset age.
* `void set_name(::fidl::ObjectView<::fidl::StringView>)`: set name with an already allocated value.
* `void set_name(::fidl::AnyAllocator&, ::fidl::AnyAllocator&, std::string_view)`: set name with the
  given value. The storage for the storage of the value (StringView) and the storage of the string
  are allocated using the two allocators. The same allocator should be given to the two allocator
  arguments.
* `void set_name(std::nullptr_t)`: unset name.

In order to build a table, one additional class is generated: `User::Frame`.

`User::Frame` is a container for the table's internal storage, and is allocated
separately from the builder because LLCPP maintains the object layout of the
underlying wire format. It is only use internally by `User(::fidl::AnyAllocator&)`.

`User::Frame` has the following methods:

* `Frame()`: Default constructor.

Example usage:

```c++
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="tables" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

In addition to assigning fields with `fidl::ObjectView`, any of the allocation
strategies described in the [tutorial][llcpp-allocation] can also be used.

Note: Tables with unknown fields will decode successfully but will fail to
encode.

## Protocols {#protocols}

Given the [protocol][lang-protocols]:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="protocols" %}
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
  the channel would consume this type. For example, a `fidl::Client<TicTacToe>`
  may be constructed from a `fidl::ClientEnd<TicTacToe>`, also known as "binding
  the channel to the message dispatcher".
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

* `TicTacToe::StartGameRequest`
* `TicTacToe::MakeMoveRequest`
* `TicTacToe::MakeMoveResponse`
* `TicTacToe::OnOpponentMoveResponse`

<!-- TODO: zeroargmessage -->

The naming scheme for requests is `[Method]Request`, and the naming scheme for
both responses and events is `[Method]Response`.

Any empty request, response, or event is aliased to `fidl::AnyZeroArgMessage`,
which is a type representing an empty message, instead of having a new type
generated.

### Client {#client}

The LLCPP bindings provides multiple ways to interact with a FIDL protocol as a
client:

* `fidl::Client<TicTacToe>`: This class exposes thread-safe APIs for outgoing
  asynchronous and synchronous calls as well as asynchronous event handling. It
  owns the client end of the channel. An `async_dispatcher_t*` is required to
  support the asynchronous APIs as well as event and error handling. This is the
  recommended variant for most use-cases, except for those where an
  `async_dispatcher_t` cannot be used.
* `TicTacToe::SyncClient`: This class exposes purely synchronous APIs for
  outgoing calls as well as for event handling. It owns the client end of the
  channel.
* `TicTacToe::Call`: This class is identical to `SyncClient` except that it does
  not have ownership of the client end of the channel. `Call` may be preferable
  to `SyncClient` when migrating code from the C bindings to the LLCPP bindings,
  or when implementing C APIs that take raw `zx_handle_t`s.

#### fidl::Client {#async-client}

<!-- TODO(fxbug.dev/58672) fidl::Client should be covered by generated docs -->

`fidl::Client` is thread-safe and supports both synchronous and asynchronous
calls as well as asynchronous event handling. It also supports use with a
multi-threaded dispatcher.

##### Creation

A client is created with a client-end `fidl::ClientEnd<P>` to the protocol `P`,
an `async_dispatcher_t*`, and an optional shared pointer on an
[`AsyncEventHandler`](#async-event-handlers) that defines the methods to be
called when a FIDL event is received or when the client is unbound. If the
virtual for a particular event is not overridden, the event is ignored.

```cpp
class EventHandler : public fidl::WireAsyncEventHandler<TicTacToe> {
 public:
  EventHandler() = default;

  void OnOpponentMove(OnOpponentMoveResponse* event) override { /* ... */ }

  void Unbound(fidl::UnbindInfo unbind_info) override { /* ... */ }
};

fidl::ClientEnd<TicTacToe> client_end = /* logic to connect to the protocol */;
Client<TicTacToe> client;
zx_status_t status = client.Bind(
    std::move(client_end), dispatcher, std::make_shared<EventHandler>());
```
The channel may be unbound automatically in case of the server-end being closed
or due to an invalid message being received from the server. You may also
actively unbind the channel through `client.Unbind()`.

#### Unbinding

Unbinding is thread-safe. In any of these cases, ongoing and future operations
will not cause a fatal failure, only returning `ZX_ERR_CANCELED` where
appropriate.

If you provided an unbound hook, it is executed as task on the dispatcher,
providing a reason and error status for the unbinding. You may also recover
ownership of the client end of the channel through the hook. The unbound hook is
guaranteed to be run.

##### Interaction with dispatcher

All asynchronous responses, event handling, and error handling are done through
the `async_dispatcher_t*` provided on creation of a client. With the exception
of the dispatcher being shutdown, you can expect that all hooks provided to the
client APIs will be executed on a dispatcher thread (and not nested within other
user code).

Note: If you shutdown the dispatcher while there are any active bindings, the
unbound hook may be executed on the thread executing shutdown. As such, you must
not take any locks that could be taken by hooks provided to `fidl::Client` APIs
while executing `async::Loop::Shutdown()/async_loop_shutdown()`. (You should
probably ensure that no locks are held around shutdown anyway since it joins all
dispatcher threads, which may take locks in user code).

##### Outgoing FIDL methods

You can invoke outgoing FIDL APIs through the `fidl::Client` instance.
Dereferencing a `fidl::Client` provides access to the following methods:

* `fidl::Result StartGame(bool start_first)`: Managed variant of a fire
  and forget method.
* `fidl::Result StartGame(::fidl::BufferSpan _request_buffer, bool
  start_first)`: Caller-allocated variant of a fire and forget method.
* `fidl::Result MakeMove(uint8_t row, uint8_t col,
  fit::callback<void(TicTacToeResponse* response)>
  _cb)`: Managed variant of an asynchronous two way method. It takes a
  callback to handle responses as the last argument. The callback is executed
  on response in a dispatcher thread. The returned `fidl::StatusAndError` refers
  just to the status of the outgoing call.
* `fidl::Result MakeMove(fidl::BufferSpan _request_buffer, uint8_t row,
  uint8_t col, MakeMoveResponseContext* _context)`: Asynchronous,
  caller-allocated variant of a two way method. The final argument is a response
  context, which is explained below.
* `ResultOf::MakeMove MakeMove_Sync(uint8_t row, uint8_t col)`: Synchronous,
  managed variant of a two way method. The same method exists on `SyncClient`.
* `UnownedResultOf::MakeMove_sync(fidl::BufferSpan _request_bufffer, uint8_t row,
  uint8_t col, fidl::BufferSpan _response_buffer)`: Synchronous, caller-allocated
  variant of a two way method. The same method exists on `SyncClient`.

Note: One-way and synchronous two-way FIDL methods have a similar API to the
[`SyncClient`](#sync-client) versions. Aside from one-way methods directly
returning `fidl::StatusAndError` and the added `_Sync` on the synchronous
methods, the behavior is identical.

Each two way method has a response context that is used in the caller-allocated,
asynchronous case. `TicTacToe` has only one response context,
`TicTacToe::MakeMoveResponseContext`, which has pure virtual methods that
should be overridden to handle responses:

```c++
virtual void OnReply(MakeMoveResponse* msg) = 0;
virtual void OnError() = 0;
```

Only one of the two methods is called for a single response: `OnReply()` is
called with a successfully decoded response, whereas `OnError()` is called on
any error that would cause the response context to be discarded without
`OnReply()` being called. You are responsible for ensuring that the response
context object outlives the duration of the entire async call, since the
`fidl::Client` borrows the context object by address to avoid implicit
allocation.

Note: If the client is destroyed with outstanding asynchronous transactions,
`OnError()` will be invoked for all of the associated `ResponseContext`s

#### SyncClient {#sync-client}

`TicTacToe::SyncClient` provides the following methods:

* `explicit SyncClient(fidl::ClientEnd<TicTacToe>)`: Constructor.
* `~SyncClient()`: Default destructor.
* `SyncClient(&&)`: Default move constructor.
* `SyncClient& operator=(SyncClient&&)`: Default move assignment.
* `fidl::ClientEnd<TicTacToe>& client_end()`: Returns a mutable reference to
  the underlying [client endpoint](#typed-channels).
* `const fidl::ClientEnd<TicTacToe>& client_end() const`: Returns the underlying
  client endpoint as a const.
* `const zx::channel& channel() const`: Returns the underlying channel as a
  const. Prefer using the `client_end()` accessors for improved type-safety.
* `zx::channel* mutable_channel()`: Returns the underlying channel as mutable.
* `TicTacToe::ResultOf::StartGame StartGame(bool start_first)`: Owned variant of
  a fire and forget method call, which takes the parameters as arguments and
  returns the `ResultOf` class. Buffer allocation for requests and responses are
  entirely handled within this function, as is the case in simple C bindings.
  The bindings calculate a safe buffer size specific to this call at compile
  time based on FIDL wire-format and maximum length constraints. The buffers are
  allocated on the stack if they fit under 512 bytes, or else on the heap.
  In general, the managed flavor is easier to use, but may result in extra
  allocation. See [ResultOf](#resultof) for details on buffer
  management.
* `TicTacToe::UnownedResultOf::StartGame StartGame(fidl::BufferSpan, bool
  start_first)`: Caller-allocated variant of a fire and forget call, which takes
  in backing storage for the request buffer, as well as request parameters, and
  returns an `UnownedResultOf`.
* `ResultOf::MakeMove MakeMove(uint8_t row, uint8_t col)`: Owned variant of a
  two way method call, which takes the parameters as arguments and returns the
  `ResultOf` class.
* `UnownedResultOf::MakeMove(fidl::BufferSpan _request_buffer, uint8_t row,
  uint8_t col, fidl::BufferSpan _response_buffer)`: Caller-allocated variant of
  a two way method, which takes in backing storage for the request buffer,
  followed by the request parameters, and finally backing storage for the
  response buffer, and returns an `UnownedResultOf`.
* `fidl::Result HandleOneEvent(SyncEventHandler& event_handler)`: Blocks to
  consume exactly one event from the channel. See [Events](#events)

Note that each method has both an owned and caller-allocated variant. In brief,
the owned variant of each method handles memory allocation for requests and
responses, whereas the caller-allocated variant allows the user to pass in the
buffers themselves. The owned variant is easier to use, but may result in extra
allocation.

#### Call {#client-call}

`TicTacToe::Call` provides similar methods to those found in `SyncClient`, with
the only difference being that they are all `static` and take a
`fidl::UnownedClientEnd<TicTacToe>` as the first parameter:

* `static ResultOf::StartGame StartGame(
      fidl::UnownedClientEnd<TicTacToe> _client_end, bool start_first)`:
  Owned variant of `StartGame`.
* `static UnownedResultOf::StartGame StartGame(
      fidl::UnownedClientEnd<TicTacToe> _client_end,
      fidl::BufferSpan _request_buffer, bool start_first)`:
  Caller-allocated variant of `StartGame`.
* `static ResultOf::MakeMove MakeMove(
      fidl::UnownedClientEnd<TicTacToe> _client_end, uint8_t row, uint8_t col)`:
  Owned variant of `MakeMove`.
* `static UnownedResultOf::MakeMove MakeMove(
      fidl::UnownedClientEnd<TicTacToe> _client_end,
      fidl::BufferSpan _request_buffer, uint8_t row, uint8_t col,
      fidl::BufferSpan _response_buffer);`:
  Caller-allocated variant of `MakeMove`.

#### Result, ResultOf and UnownedResultOf {#resultof}

The managed variants of each method of `SyncClient` and `Call` all return a
`ResultOf::` type, whereas the caller-allocating variants all return an
`UnownedResultOf::`. Fire and forget methods on `fidl::Client` return a
`Result`. These types define the same set of methods:

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
*   **(only for ResultOf and UnownedResultOf for two-way calls)** `T* Unwrap()`
    returns a pointer to the [response struct](#request-response-structs). For
    `ResultOf::`, the pointer points to memory owned by the result object. For
    `UnownedResultOf::`, the pointer points to the caller-provided buffer.
    `Unwrap()` should only be called when the status is `ZX_OK`.

Additionally, `ResultOf` and `UnownedResultOf` for two-way calls will
implement dereference operators that return the response struct itself.
This allows code such as:

```cpp
auto result = client->MakeMove_Sync(0, 0);
auto response = result->Unwrap();
bool success = response->success;
```

To be simplified to:

```cpp
auto result = client->MakeMove_Sync(0, 0);
bool success = result->success;
```

> `ResultOf` manages ownership of all buffer and handles, while `::Unwrap()`
> returns a view over it. Therefore, this object must outlive any references
> to the unwrapped response.

##### Allocation strategy And move semantics

`ResultOf::` stores the response buffer inline if the message is guaranteed
to fit under 512 bytes. Since the result object is usually instantiated on the
caller's stack, this effectively means the response is stack-allocated when it
is reasonably small. If the maximal response size exceeds 512 bytes,
`ResultOf::` instead contains a `std::unique_ptr` to a heap-allocated buffer.

Therefore, a `std::move()` on `ResultOf::Foo` may be costly if the response
buffer is inline: the content has to be copied, and pointers to out-of-line
objects have to be updated to locations within the destination object.
Consider the following snippet:

```cpp
int CountPlanets(ResultOf::ScanForPlanets result) { /* ... */ }

auto result = client->ScanForPlanets();
SpaceShip::ScanForPlanetsResponse* response = result.Unwrap();
Planet* planet = &response->planets[0];
int count = CountPlanets(std::move(result));    // Costly
// In addition, |response| and |planet| are invalidated due to the move
```

It may be written more efficiently as:

```cpp
int CountPlanets(fidl::VectorView<SpaceShip::Planet> planets) { /* ... */ }

auto result = client.ScanForPlanets();
int count = CountPlanets(result.Unwrap()->planets);
```

> If the result object need to be passed around multiple function calls,
> consider pre-allocating a buffer in the outer-most function and use the
> caller-allocating flavor.

### Server

Implementing a server for a FIDL protocol involves providing a concrete
implementation of `TicTacToe`.

The generated `fidl::WireServer<TicTacToe>` class has pure virtual methods
corresponding to the method calls defined in the FIDL protocol. Users implement
a `TicTacToe` server by providing a concrete implementation of
`fidl::WireServer<TicTacToe>`, which has the following pure virtual methods:

* `virtual void StartGame(StartGameRequestView request, StartGameCompleter::Sync
  _completer)`
* `virtual void MakeMove(MakeMoveRequestView request, MakeMoveCompleter::Sync
  _completer)`

Refer to the [example LLCPP server][llcpp-server-example] for how to bind and
set up a server implementation.

The LLCPP bindings also provide functions for manually dispatching a message
given an implementation, `fidl::WireTryDispatch<TicTacToe>` and
`fidl::WireDispatch<TicTacToe>`:

* `fidl::DispatchResult fidl::WireTryDispatch<TicTacToe>(
  fidl::WireServer<TicTacToe>* impl, fidl::IncomingMessage& msg,
  ::fidl::Transaction* txn)`: Attempts to dispatch the incoming message. If
  there is no matching handler, it returns `fidl::DispatchResult::kNotFound`,
  leaving the message and transaction intact. In all other cases, it consumes
  the message and returns `fidl::DispatchResult::kFound`.
* `fidl::DispatchResult fidl::WireDispatch<TicTacToe>(
  fidl::WireServer<TicTacToe>* impl, fidl::IncomingMessage&& msg,
  ::fidl::Transaction* txn)`: Dispatches the incoming message. If there is no
  matching handler, it closes all handles in the message and closes the channel
  with a `ZX_ERR_NOT_SUPPORTED` epitaph, and returns
  `fidl::DispatchResult::kNotFound`.

#### Requests {#server-requests}

The request is provided as the first argument of each generated FIDL method
handler. This a view of the request (a pointer). All the request arguments are
accessed using the arrow operator and the argument name.

For example:
* `request->start_first`
* `request->row`

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

* `::fidl::Result Reply(bool success, fidl::ObjectView<GameState> new_state)`
* `::fidl::Result Reply(fidl::BufferSpan _buffer, bool success,
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
[lib/fidl](/zircon/system/ulib/fidl) bindings.

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
fidl::Buffer<StartGameRequest> request_buffer;
auto result = client.StartGame(request_buffer.view(), true);

// 2. On the heap
auto request_buffer = std::make_unique<fidl::Buffer<StartGameRequest>>();
auto result = client.StartGame(request_buffer->view(), true);

// 3. Some other means, e.g. thread-local storage
constexpr uint32_t request_size = fidl::MaxSizeInChannel<StartGameRequest>();
uint8_t* buffer = allocate_buffer_of_size(request_size);
fidl::BufferSpan request_buffer(/* data = */buffer, /* capacity = */request_size);
auto result = client.StartGame(request_buffer, true);

// Check the transport status (encoding error, channel writing error, etc.)
if (result.status() != ZX_OK) {
  // Handle error...
}

// Don't forget to free the buffer at the end if approach #3 was used...
```

> When the caller-allocating flavor is used, the `result` object borrows the
> request and response buffers (hence its type is under `UnownedResultOf`).
> Make sure the buffers outlive the `result` object.
> See [UnownedResultOf](#resultof-and-unownedresultof).

Note: Buffers passed to the bindings must be aligned to 8 bytes. The
`fidl::Buffer` helper class does this automatically. Failure to align would
result in a run-time error.

### Events {#events}

In LLCPP, events can be handled asynchronously or synchronously, depending
on the type of [client](#client) being used.

#### Async client {#async-event-handlers}

When using a `fidl::Client`, events can be handled asynchronously by passing the
class a `std::shared_ptr<TicTacToe::AsyncEventHandler>`. The `AsyncEventHandler` class has the
following members:

* `virtual void OnOpponentMove(OnOpponentMoveResponse* message) {}`: handler for the
  OnOpponentMove event (one method per event).

* `virtual Unbound(::fidl::UnbindInfo info) {}`: method called when the client has been unbound.

To be able to handle events and unbound, a class that inherits from `AsyncEventHandler` must be
defined.

#### Sync client {#sync-event-handlers}

For `SyncClient` and `Call` clients, events are handled synchronously by calling
a `HandleOneEvent` function and passing it a `TicTacToe::SyncEventHandler`.

`SyncEventHandler` is a class that contains a pure virtual method for each event. In
this example, it consists of the following member:

* `virtual void OnOpponentMove(TicTacToe::OnOpponentMoveResponse* event) = 0`:
  The handle for the OnOpponentMove event.
* `virtual zx_status_t Unknown() { return ZX_ERR_NOT_SUPPORTED; }`:
  The status to be returned by `HandleOneEvent` if an unknown event is found.
  This method should be overriden only if a specific status is needed.

To be able to handle events, a class that inherits from `SyncEventHandler` must
be defined. This class must define the virtual methods for the events it wants
to handle. All the other events are ignored. Then an instance of this class must
be allocated.

There are two ways to handle one event. Each one use an instance of the user
defined event handler class:

* `::fidl::Result TicTacToe::SyncClient::HandleOneEvent(
       SyncEventHandler& event_handler)`:
  A bound version for sync clients.
* `::fidl::Result TicTacToe::SyncEventHandler::HandleOneEvent(
       fidl::UnownedClientEnd<TicTacToe> client_end)`:
  An unbound version that
  uses an `fidl::UnownedClientEnd<TicTacToe>` to handle one event for a
  specific handler.

For each call to `HandleOneEvent`, the method waits on the channel for exactly
one incoming message. Then the message is decoded. If the result is ZX_OK then
exactly one virtual method has been called. If not no virtual method has been
called and the status indicates the error.

If the handlers are always the same (from one call to `HandleOneEvent` to the
other), the `SyncEventHandler` object should be constructed once and used each time
you need to call `HandleOneEvent`.

If an event is marked as transitional, then a default implementation is
provided (instead of the pure virtual).

#### Server

##### Sending events using a server binding object {#bound-event-sending}

When binding a server implementation to a channel, calling `fidl::BindServer`
will return a `fidl::ServerBindingRef<Protocol>`, which is the means by which you
may interact safely with a server binding. This class allows access to an event
sender interface through the following operators:

```c++
typename Protocol::EventSender* get() const;
typename Protocol::EventSender* operator->() const;
typename Protocol::EventSender& operator*() const;
```

where `Protocol` is a template parameter.

The `EventSender` class contains managed and caller-allocated methods for
sending each event. As a concrete example, `TicTacToe::EventSender` provides the
following methods:

* `zx_status_t OnOpponentMove(GameState new_state)`: Managed flavor.
* `zx_status_t OnOpponentMove(fidl::BufferSpan _buffer, GameState new_state)`:
  Caller allocated flavor.

##### Sending events with a ServerEnd object

[Sending events using a server binding object](#bound-event-sending) is the
standard approach to sending events while the server endpoint is bound to an
implementation. However, there may be occasions which call for sending events
on a `fidl::ServerEnd<TicTacToe>` object directly, without setting up a server
binding.

The `TicTacToe` class contains an `EventSender` which provides methods for
sending events on a channel. The `EventSender` may be constructed from a
`fidl::ServerEnd<TicTacToe>`. Each event sending method has managed and
caller-allocating variants, analogous to the [client API](#client) as well as
the [server completers](#server-completers).

The event sender methods are:

```c++
class TicTacToe::EventSender {
 public:
  EventSender(fidl::ServerEnd<TicTacToe> server_end);

  zx_status_t SendOnOpponentMoveEvent(
      GameState new_state);
  zx_status_t SendOnOpponentMoveEvent(
      fidl::BufferSpan _buffer, GameState new_state);

  const fidl::ServerEnd<TicTacToe>& server_end() const;
  fidl::ServerEnd<TicTacToe>& server_end();
};
```

`EventSender` consumes the server endpoint upon construction, to ensure that the
channel stays alive while events are being written. After sending events, the
user may deconstruct the `EventSender` by extracting the server endpoint via
`server_end()`.

### Results {#protocols-results}

Given a method:

```fidl
protocol TicTacToe {
    MakeMove(uint8 row, uint8 col) -> (GameState new_state) error MoveError;
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
[`[Transitional]`](/docs/reference/fidl/language/attributes.md#transitional)
attribute, the `virtual` methods on the protocol class come with a default
`Close(ZX_NOT_SUPPORTED)` implementation. This allows implementations of the
protocol class with missing method overrides to compile successfully.

#### Discoverable

A protocol annotated with the
[`[Discoverable]`](/docs/reference/fidl/language/attributes.md#discoverable)
attribute causes the FIDL toolchain to generate an additional `static const char
Name[]` field on the protocol class, containing the full protocol name.

## Explicit encoding and decoding {#encoding-decoding}

FIDL messages are automatically encoded when they are sent and decoded when they
are received.

However, some use cases like persistence need to explicitly encode or decode a
table or struct.

This section describes how to explicitly use the encoding and the decoding.

### Encoding

When an object is allocated and initialized,
`fidl::OwnedEncodedMessage<FidlType>` can be used to encode it. For example:

```c++
void Encode(::fuchsia_examples::User& user) {
  ::fidl::OwnedEncodedMessage<::fuchsia_examples::User> encoded(&user);
  if (!encoded.ok()) {
    // Do something about the error.
    return;
  }
  fidl_outgoing_msg_t* message = encoded.GetOutgoingMessage().message();
  // Do something with the data referenced by message.
}
```

At this point, the table `user` is encoded within `encoded`. The following
methods are available on an encoded FIDL type:

* `bool encoded.ok()`
* `zx_status_t encoded.status()`
* `const char* encoded.error_message()`
* `::fidl::OutgoingMessage& encoded.GetOutgoingMessage()`

`::fidl::OutgoingMessage` is defined in
[/zircon/system/ulib/fidl/include/lib/fidl/llcpp/message.h](/zircon/system/ulib/fidl/include/lib/fidl/llcpp/message.h).

### Decoding

Once an object has been encoded (and eventually stored somewhere),
`fidl::DecodedMessage<FidlType>` can be used to decode it. For example:

```c++
void UseEncodedUser(std::vector<uint8_t> buffer) {
  fidl::DecodedMessage<::fuchsia_examples::User> decoded(
      buffer.data(), static_cast<uint32_t>(buffer.size()));
  if (!decoded.ok()) {
    // Display an error.
    return;
  }
  ::fuchsia_examples::User* user = decoded.PrimaryObject();
  // Do something with the table (user).
}
```

When an object is decoded, the following methods are available on a decoded FIDL
type:

* `bool decoded.ok()`
* `zx_status_t decoded.status()`
* `const char* decoded.error_message()`
* `FidlType* decoded.PrimaryObject()`
* `void decoded.ReleasePrimaryObject()`

The FIDL type is the type used by the templated class (in the example above:
`::fuchsia_examples::User`).

The primary object is decoded in place within the provided buffer. This is also
the case of all the secondary objects. That means that the provided buffer must
be kept alive while the decoded value is used.

For FIDL types that allow handles, the handles can be specified during
construction after the bytes (the same way bytes are specified).

### Persistence

Persistence is not officially supported by LLCPP. However, explicit encoding and
decoding can be used to store FIDL values by encoding a value and then writing
it and by reading a value and then decoding it. In that case, the values can't
use any handle.

### Test scaffolding {#test-scaffolding}

The FIDL toolchain also generates a file suffixed with  `_test_base.h` that
contains convenience code for testing FIDL server implementations. This file
contains a class for each protocol that provides stub implementations for each
of the class’s methods, making it possible to implement only the methods that
are used during testing. These classes are generated into a `testing` namespace
that is inside of the generated library’s namespace (e.g. for library
`games.tictactoe`, these classes are generated into
`games::tictactoe::testing`).

For the same `TicTacToe` protocol listed above, the FIDL toolchain generates a
`TicTacToe_TestBase` class that subclasses `TicTacToe` (see
[Protocols](#protocols)), offering the following methods:

* `virtual ~TicTacToe_TestBase() {}`: Destructor.
* `virtual void NotImplemented_(const std::string& name, ::fidl::CompleterBase& completer) = 0`:
  Pure virtual method that is overridden to define behavior for unimplemented methods.

`TicTacToe_TestBase` provides an implementation for the virtual protocol
methods `StartGame` and `MakeMove`, which are implemented to just call
`NotImplemented_("StartGame", completer)` and `NotImplemented_("MakeMove",
completer)`, respectively.

<!-- xrefs -->
[llcpp-allocation]: /docs/development/languages/fidl/guides/llcpp-memory-ownership.md
[llcpp-async-example]:
/docs/development/languages/fidl/tutorials/llcpp/topics/async-completer.md
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
[union-lexicon]: /docs/reference/fidl/language/lexicon.md#union-terms
[unknown-attr]: /docs/reference/fidl/language/attributes.md#unknown
[zircon-channel]: /docs/reference/kernel_objects/channel.md
