# HLCPP bindings

<<../../../_common/_hlcpp_notice.md>>

## Libraries {#libraries}

Given the library declaration:

```fidl
library fuchsia.examples;
```

All code for this library is generated in the `fuchsia::examples` namespace, and
 [test scaffolding](#test-scaffolding) is generated in
 `fuchsia::examples::testing`.

## Constants {#constants}

All [constants][lang-constants] are generated as a `constexpr`. For example, the
 following constants:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="consts" %}
```

Are generated in the header file as:

```c++
constexpr uint8_t BOARD_SIZE = 9u;
extern const char[] NAME;
```

The correspondence between FIDL primitive types and C++ types is outlined in
[built-in types](#builtins). Instead of `constexpr`, strings are declared as an
`extern const char[]` in the header file, and defined in a `.cc` file.

## Fields {#fields}

This section describes how the FIDL toolchain converts FIDL types to native
types in HLCPP. These types can appear as members in an aggregate type or as
parameters to a protocol method.

### Built-in types {#builtins}

The FIDL types are converted to C++ types based on the following table:

|FIDL Type|HLCPP Type|
|--- |--- |
|`bool`|`bool`|
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
|`array<T, N>`|`std::array`|
|`vector<T>:N`|`std::vector`|
|`vector<T>:<N, optional>`|`fidl::VectorPtr`|
|`string`|`std::string`|
|`string:optional`|`fidl::StringPtr`|
|`server_end:P`, `server_end:<P, optional>`|`fidl::InterfaceRequest`|
|`client_end:P`, `client_end:<P, optional>`|`fidl::InterfaceHandle`|
|`zx.handle`, `zx.handle:optional`|`zx::handle`|
|`zx.handle:S`, `zx.handle:<S, optional>`|The corresponding zx type is used. For example, `zx::vmo` or `zx::channel`.|

### User defined types {#user-defined-types}

In HLCPP, a user defined type (bits, enum, constant, struct, union, or table) is
referred to in the bindings using the generated class or variable (see [Type
Definitions](#type-definitions)). For a nullable user-defined type `T`,
`unique_ptr` of the equivalent generated type is used.

### Request, response, and event parameters {#request-response-event-parameters}

Whenever FIDL needs to generate a single type representing parameters for a
request, response, or event (e.g. when generating [`fpromise::result` compatible result types](#protocols-results)),
it uses the following rules:

* Multiple arguments are generated as an `std::tuple` of the parameter types.
* A single parameter is just referred to using the parameter type itself.
* An empty set of parameters is represented using `void`.

## Type definitions {#type-definitions}

### Bits {#bits}

Given the [bits][lang-bits] definition:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="bits" %}
```

The FIDL toolchain generates a C++ `enum class` using the specified underlying
type, or `uint32_t` if none is specified:

```c++
enum class FileMode : uint16_t {
    READ = 1u;
    WRITE = 2u;
    EXECUTE = 4u;
};
```

In addition, FIDL generates the following methods for `FileMode`:

* Bitwise operators: implementations for the `|`, `|=`, `&`, `&=`, `^`, `^=`,
  and `~` operators are generated, allowing bitwise operations on the bits like
  `mode |= FileMode::EXECUTE`.

FIDL also generates a `const static FileMode FileModeMask` variable. This is a
bitmask containing all of the bits in the enum class, which can be used to get
rid of any unused bit values from a raw underlying `uint16_t` (or whichever type
the `bits` are based on). In the above example, `FileModeMask` has a value of
`0b111`.

Example usage:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/main.cc" region_tag="bits" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

#### Flexible bits {#flexible-bits}

[Flexible][lang-flexible] bits are implemented as a `class` instead of an `enum
class`, with the following additional methods:

* `constexpr FileMode()`: Default constructor that initializes a value with no
  bits set.
* `constexpr FileMode(uint16_t)`: Constructs a value from an underlying
   primitive value, preserving any unknown bit members.
* `constexpr cpp17::optional<FileMode> TryFrom(uint16_t value)`: Constructs an
  instance of the bits from an underlying primitive value if the value does not
  contain any unknown members, and returns `cpp17::nullopt` otherwise.
* `constexpr FileMode TruncatingUnknown(uint16_t value)`: Constructs an instance
  of the bits from an underlying primitive value, clearing any unknown members.
* `constexpr FileMode unknown_bits() const`: Returns a bits value that contains
  only the unknown members from this bits value.
* `constexpr bool has_unknown_bits() const`: Returns whether this value contains
  any unknown bits.
* `explicit constexpr operator uint16_t() const`: Converts the bits value back
  to its underlying primitive value.
* `explicit constexpr operator bool() const`: Returns whether any bits are set.

<!-- TODO(fxbug.dev/64760): mask value should be consistent -->
The generated class contains a static number for each bits member as well as
for the bits mask. These correspond exactly with the members of the `enum class`
value, with the addition a `kMask` member that replaces `FileModeMask`.

* `const static FileMode READ`
* `const static FileMode WRITE`
* `const static FileMode EXECUTE`
* `const static FileMode kMask`

Note: When applying bitwise negation to bits values that contain unknown
members, the resulting bits value is only defined for the known bits.

### Enums {#enums}

Given the [enum][lang-enums] definition:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="enums" %}
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
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/main.cc" region_tag="enums" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

#### Flexible enums {#flexible-enums}

[Flexible][lang-flexible] enums are implemented as a `class` instead of an `enum
class`, with the following methods:

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
  underlying value

The generated class contains a static member for each enum member, which are
guaranteed to match the members of the `enum class` in the equivalent
[strict][lang-flexible] enum:

* `const static LocationType MUSEUM`
* `const static LocationType AIRPORT`
* `const static LocationType RESTAURANT`

### Structs {#structs}

Given a [struct][lang-structs] declaration:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="structs" %}
```

The FIDL toolchain generates a `Color` type with public members and methods.

* `public` members:
  * `uint32_t id{}`: This field is zero-initialized since no default value is
    provided.
  * `std::string name = "red"`: The corresponding field for `name`.

* Methods:
  * `static inline std::unique_ptr<Color> New()`: returns a `unique_ptr` to a
    new `Color`.

The 6 special members of `Color` (default, copy and move constructor,
destructor, copy and move assignment) are implicitly defined.

`Color` also has the following associated generated values:

* `ColorPtr`: an alias to `unique_ptr<Color>`.

Structs may have additional members if they represent the response variant of a
[result](#protocols-results).

Example usage:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/main.cc" region_tag="structs" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Unions {#unions}

Given the union definition:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="unions" %}
```

FIDL generates a `JsonValue` class. `JsonValue` contains a public tag enum
representing the possible [variants][union-lexicon]:

```c++
enum Tag : fidl_xunion_tag_t {
  kIntValue = 2,
  kStringValue = 3,
  Invalid = std::numeric_limits<fidl_xunion_tag_t>::max(),
};
```

Each member of `Tag` has a value matching its [ordinal][union-lexicon] specified
in the `union` definition. Reserved fields do not have any generated code. In
addition, there is an `Invalid` field, which is the initial value used for a
`JsonValue` that has no variant set yet.

`JsonValue` provides the following methods:

* `JsonValue()`: Default constructor. The tag is initially `Tag::Invalid` until
  the `JsonValue` is set to a specific variant. Using the `WithFoo` constructors
  should be preferred whenever possible.
* `~JsonValue()`: Default destructor
* `static JsonValue WithIntValue(int32&&)` and `static JsonValue
  WithStringValue(std::string&&)`: Static constructors that directly construct a
  specific variant of the union.
* `static inline std::unique_ptr<JsonValue> New()`: Returns a `unique_ptr` to a
  new `JsonValue`
* `bool has_invalid_tag()`: Returns `true` if the instance of `JsonValue` does
   not yet have a variant set. Users should not access a union until a variant
   is set - doing so should be considered undefined behavior.
* `bool is_int_value() const` and `bool is_string_value() const`: Each variant
  has an associated method to check whether an instance of `JsonValue` is of
  that variant
* `const int32_t& int_value() const` and `const std::string& string_value()
  const`: Read-only accessor methods for each variant. These methods fail if
  `JsonValue` does not have the specified variant set
* `int32_t& int_value()` and `std::string& string_value()`: Mutable accessor
  methods for each variant. If the `JsonValue` has a different variant than the
  called accessor method, it will destroy its current data and re-initialize it
  as the specified variant.
* `JsonValue& set_int_value(int32_t)` and `JsonValue&
  set_string_value(std::string)`: Setter methods for each variant.
* `Tag Which() const`: returns the current [tag][union-lexicon] of the
  `JsonValue`.
* `fidl_xunion_tag_t Ordinal() const`: returns the raw `fidl_xunion_tag_t` tag.
  Prefer to use `Which()` unless the raw ordinal is required

`JsonValue` also has the following associated generated values:

* `JsonValuePtr`: an alias to `unique_ptr<Foo>`.

Unions may have additional methods if they represent the response variant of a
[result](#protocols-results).

Example usage:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/main.cc" region_tag="unions" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

#### Flexible unions and unknown variants

[Flexible][lang-flexible] unions have an extra variant in the generated `Tag`
class:

```c++
enum Tag : fidl_xunion_tag_t {
    kUnknown = 0,
    ... // other fields omitted
};
```

When a FIDL message containing a union with an unknown variant is decoded into
`JsonValue`, `JsonValue::Which()` returns `JsonValue::Tag::kUnknown`, and
`JsonValue::Ordinal()` returns the unknown ordinal.

A flexible `JsonValue` type will have extra methods for interacting with unknown
data that will depend on whether the type is a
[value or resource type][lang-resource]. Value types will not have
unknown data methods that reference `zx::handle`.

A flexible `JsonValue` that is a [resource][lang-resource] type has the
following extra methods:

* `const vector<uint8_t>* UnknownBytes() const`: Returns the raw bytes of the
  union variant if it is unknown, or `nullptr` otherwise.
* `const vector<zx::handle>* UnknownHandles() const`: Returns the handles of the
  union variant in [traversal order][traversal] if it is unknown, or
  `nullptr` otherwise.
* `JsonValue& SetUnknownData(fidl_xunion_tag_t ordinal, vector<uint8_t> bytes,
  vector<zx::handle> handles)`: Similar to the setter methods for the known
  members, this sets the union to an unknown variant with the specified ordinal,
  bytes, and handles. This method should only be used for testing, e.g. to
  ensure that code can handle unknown data correctly.

A flexible `JsonValue` that is a [value][lang-resource] type has the following
extra methods:

* `const vector<uint8_t>* UnknownBytes() const`: Returns the raw bytes of the
  union variant if it is unknown, or `nullptr` otherwise.
* `JsonValue& SetUnknownData(fidl_xunion_tag_t ordinal, vector<uint8_t> bytes)`:
  Similar to the setter methods for the known members, this sets the union to an
  unknown variant with the specified ordinal and bytes. This method should only
  be used for testing, e.g. to ensure that code can handle unknown data
  correctly.

Encoding a union with an unknown variant writes the unknown data and the
original ordinal back onto the wire.

[Strict][lang-flexible] unions fail when decoding an unknown variant.
[Flexible][lang-flexible] unions that are [value][lang-resource] types fail when
decoding an unknown variant with handles.

### Tables {#tables}

Given the [table][lang-tables] definition:

```table
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="tables" %}
```

The FIDL toolchain generates a `User` class with the following methods:

* `User()`: Default constructor, initializes with all fields unset.
* `User(User&&)`: Move constructor.
* `~User()`: Destructor.
* `User& User::operator=(User&&)`: Move assignment.
* `bool IsEmpty() const`: Returns true if no fields are set.
* `bool has_age() const` and `bool has_name() const`: Returns whether a field is
  set.
* `const uint8_t& age() const` and `const std::string& name() const`: Read-only
  field accessor methods. These fail if the field is not set.
* `uint8_t* mutable_age()` and `std::string* mutable_name()`: Mutable field
  accessor methods. If the field is not set, a default one will be constructed,
  set, and returned.
* `User& set_age(uint8_t)` and `User& set_name(std::string)`: Field setters.
* `void clear_age()` and `void clear_name()`: Clear the value of a field by
  calling its destructor

The `User` class will also provide methods for interacting with unknown fields
which will depend on whether the type is a [value or resource type][lang-resource].
Tables that are a value type will not have unknown
data methods that reference `zx::handle`, and will fail to decode data with
unknown fields that contain handles.

If `User` is a [resource][lang-resource] type, it will have the following
methods:

* `const std::map<uint64_t, fidl::UnknownData>>& UnknownData() const`: Returns a
  map from ordinal to bytes and handles. The handles are guaranteed to be in
  [traversal order][traversal].
* `void SetUnknownDataEntry(uint32_t ordinal, fidl::UnknownData&& data)`: Set
  the bytes and handles of an unknown field if it doesn't already exist. This
  method should only be used for testing, e.g. to check that tables with unknown
  fields are handled correctly.

If `User` is a [value][lang-resource] type, it will have the following methods:

* `const std::map<uint64_t, vector<uint8_t>& UnknownData() const`: Returns a
  map from ordinal to bytes.
* `void SetUnknownDataEntry(uint32_t ordinal, vector<uint8_t>&& data)`: Set
  the bytes of an unknown field if it doesn't already exist. This method should
  only be used for testing, e.g. to check that tables with unknown fields are
  handled correctly.

`User` also has the following associated generated values:

* `UserPtr`: an alias to `unique_ptr<User>`.

Example usage:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/main.cc" region_tag="tables" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Inline layouts

The generated C++ code uses the [the name reserved by `fidlc`][anon-names] for
inline layouts.

## Protocols {#protocols}

Given the [protocol][lang-protocols]:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="protocols" %}
```

Note: The `MakeMove` method above returns a bool representing success, and a
nullable response value. This is considered un-idiomatic, you should use an [error type](#protocols-results)
instead.

FIDL generates a `TicTacToe` class, which acts as an entry point for interacting
with the protocol and defines the interface of the service used by
clients to proxy calls to the server, and for the server for implementing the
protocol. Synchronous clients use a different virtual interface, `TicTacToe_Sync`.

`TicTacToe` contains the following member types:

* `MakeMoveCallback` and `OnOpponentMoveCallback`: Each response and event has a
  member type generated that represents the type of the callback for handling
  that response or event. In the above example, `MakeMoveCallback` aliases
  `fit::function<void(bool, std::unique_ptr<GameState>)>` and
  `OnOpponentMoveCallback` aliases `fit::function<void(GameState)>`.

`TicTacToe` additionally has the following pure virtual methods, corresponding
to the methods in the protocol definition:

* `virtual void StartGame(bool start_first)`: Pure virtual method for a fire and
  forget protocol method. It takes as arguments the request parameters.
* `virtual void MakeMove(uint8_t row, uint8_t col, MakeMoveCallback callback)`:
  Pure virtual method for a two way protocol method. It takes as arguments the
  request parameters followed by the response handler callback.

`TicTacToe_Sync` has the following pure virtual methods, corresponding to the
methods in the protocol definition:

* `virtual zx_status_t StartGame(bool start_first)`: Pure virtual method for a
  fire and forget protocol method. It takes as arguments the request parameters,
  and returns a `zx_status_t` representing whether the request was sent
  successfully.
* `virtual zx_status_t MakeMove(uint8_t row, uint8_t col, bool* out_success, std::unique_ptr<GameState>* out_new_state)`: Pure virtual method for a two way
  method protocol. It takes as arguments the request parameters, followed by
  output pointers for each of the response parameters. It returns a `zx_status_t`
  representing whether the method call was made successfully.

Other code may be generated depending on the
[attributes](#protocol-and-method-attributes) applied to the protocol or its
methods.

### Client {#protocols-client}

The FIDL toolchain generates two aliases for the classes used to make calls to a
`TicTacToe` server : `TicTacToePtr`, which aliases
`fidl::InterfacePtr<TicTacToe>` representing an async client, and
`TicTacToeSyncPtr`, which aliases `fidl::SynchronousInterfacePtr<TicTacToe>`
representing a synchronous client.

When dereferenced, `TicTacToePtr` and `TicTacToeSyncPtr` return a proxy class
that implements `TicTacToe` and `TicTacToe_Sync`, respectively, which proxies
requests to the server. In this example, given a `TicTacToePtr` called
`async_tictactoe`, requests could be made by calling
`async_tictactoe->StartGame(start_first)` or `async_tictactoe->MakeMove(row,
col, callback)`.

Examples on how to set up and bind an `InterfacePtr` or a
`SynchronousInterfacePtr` to a channel are covered in the
[HLCPP tutorial][client-tut].

The `fidl::InterfacePtr` type is thread-hostile.  All calls to an instance of
this type must be made from the same thread. The `fidl::SynchronousInterfacePtr`
type is thread-compatible. Once an instance of this type is bound it can be used
from multiple threads simultaneously. The `fidl::InterfaceHandle` type can be
used to safely transfer a channel handle between threads. See the class
documentation on these types for more details.

### Server

Implementing a server for a FIDL protocol involves providing a concrete
implementation of `TicTacToe`.

Examples on how to set up and bind a server implementation are covered in the
HLCPP tutorial.

### Events {#events}

#### Client

For a `TicTacToePtr` `tictactoe`, `tictactoe.events()` returns a proxy class
that contains the following public members:

* `OnOpponentMoveCallback OnOpponentMove`: The callback handler for the
  `OnOpponentMove` event.

Clients can handle events by setting the members of this class to the desired
event handlers.

Refer to the [top-level generated protocol code](#protocols) for details on the
callback types.

#### Server

For a `Binding<TicTacToe>` `tictactoe`, `tictactoe.events()` returns a stub
class that contains the following public members:

* `void OnOpponentMove(GameState new_state)`: Send an `OnOpponentMove`.

The [tutorial][server-tut] has an example for obtaining a `Binding`.

### Results {#protocols-results}

Given a method with an error type, a flexible method, or a flexible method with
an error type:

```fidl
open protocol TicTacToe {
    strict MakeMove(struct {
      row uint8;
      col uint8;
    }) -> (struct {
      new_state GameState;
    }) error MoveError;

    flexible GetState() -> (struct {
      current_state GameState;
    });

    flexible DryRunMove(struct {
      row uint8;
      col uint8;
    }) -> (struct {
      new_state GameState;
    }) error MoveError;
};
```

FIDL generates code so that clients and servers can use `fpromise::result` in
place of the generated response type. This is done by generating a result class
to represent the response that is interchangeable with an `fpromise::result`.

* `MakeMove`:
  * Generates class `TicTacToe_MakeMove_Result`
  * Interchangeable with `fpromise::result<GameState, MoveError>`
* `GetState`:
  * Generates class `TicTacToe_GetState_Result`
  * Interchangeable with `fpromise::result<GameState, fidl::TransportErr>`
* `DryRunMove`:
  * Generates class `TicTacToe_DryRunMove_Result`
  * Interchangeable with
    `fpromise::result<GameState, std::variant<MoveError, fidl::TransportErr>>`

 Using this feature, example implementations of these methods on the server side
 could look like:

```c++
void MakeMove(uint8_t row, uint8_t col, MakeMoveCallback callback) override {
  std::optional<MoveError> error = ApplyMove(row, col);
  if (!error.has_value()) {
    callback(fpromise::ok(game_state_.state()));
  }
  callback(fpromise::error(error.value()));
}

void GetState(MakeMoveCallback callback) override {
  callback(fpromise::ok(game_state_.state()));
  // The server application code *must not* attempt to send a
  // fidl::TransportErr. If it does, the server binding will panic.
  }

void DryRynMove(uint8_t row, uint8_t col, MakeMoveCallback callback) override {
  std::optional<MoveError> error = TestMove(row, col);
  if (!error.has_value()) {
    callback(fpromise::ok(game_state_.state()));
  }
  // The server application code *must not* attempt to send a
  // fidl::TransportErr. If it does, the server binding will panic.
  callback(fpromise::error(error.value()));
}
```

An example of using this on the client side, in the async case would be:

```c++
async_game->MakeMove([&](fpromise::result<GameState, MoveError> response) { ... });
async_game->GetState(
    [&](fpromise::result<GameState, fidl::TransportErr> response) { ... });
async_game->DryRunMove(
    [&](fpromise::result<GameState, std::variant<MoveError, fidl::TransportErr>> response) { ... });
```

On the client side, `fidl::TransportErr` means that the flexible two-way
interaction was not known to the server.

When generating code, the FIDL toolchain treats `TicTacToe_*_Result` as a
`union` with up to three variants:

* `response` is a generated type which follows the
  [parameter type conversion rules](#protocol-and-method-attributes):
  * if `MakeMove` returns a single parameter in its struct return type, or if
    the return type is a tuple or union, the return type would be
    `fpromise::result<T, ...>` where `T` is either the single parameter of the
    struct or the tuple or union return type.
  * if `MakeMove` returned multiple values on success, the result type would be
    a tuple of the response parameters `fpromise::result<std::tuple<...>, ...>`
  * if `MakeMove` returned an empty response, the result type would be
    `fpromise::result<void, ...>`
* `err` is the error type, which is `MoveError` in the examples of both
  `MakeMove` and `DryRunMove`.
  * This variant only exists if the method uses error syntax.
* `transport_err` always has the type `fidl::TransportErr`.
  * This variant only exists if the method is `flexible`.

The `TicTacToe_*_Result` types provide all the methods available to a [regular
union](#unions). In addition, `TicTacToe_*_Result` types provide methods that
allow interop with `fpromise::result`, for example, for
`TicTacToe_MakeMove_Result`:

* `TicTacToe_MakeMove_Result(fpromise::result<GameState, MoveError>&& result)`: Move
  constructor from a `fpromise::result`.
* `TicTacToe_MakeMove_Result(fpromise::ok_result<GameState>&& result)`: Move
  constructor from a `fpromise::ok_result`.
* `TicTacToe_MakeMove_Result(fpromise::error_result<MoveError>&& result)`: Move
  constructor from a `fpromise::error_result`.
* `operator fpromise::result<GameState, MoveError>() &&`: Conversion to a
  `fpromise::result`.

The other `TicTacToe_*_Result` types will have similar conversions for their
corresponding `fpromise::result` types.

The FIDL toolchain also generates a `TicTacToe_MakeMove_Response` class, which
is the type of the `response` variant of `TicTacToe_MakeMove_Result`. This class
is treated as a FIDL struct with fields corresponding to each parameter of the
successful response. In addition to the methods and members available to a
[regular struct](#structs), `TicTacToe_MakeMove_Response` provides additional
methods that allow interop with `std::tuple`:

* `explicit TicTacToe_MakeMove_Response(std::tuple<GameState> _value_tuple)`:
  Constructor from a tuple.
* `operator std::tuple<GameState>() &&`: Conversion operator for a tuple.

### Unknown interaction handling {#unknown-interaction-handling}

#### Server-side

When a protocol is declared as `open` or `ajar`, the generated interface class,
e.g. `TicTacToe`, will contain an extra virtual method, called
`handle_unknown_method`, with this signature:

```c++
// If the protocol is open:
virtual void handle_unknown_method(uint64_t ordinal, bool method_has_response) = 0;
// If the protocol is ajar:
virtual void handle_unknown_method(uint64_t ordinal) = 0;
```

When implementing an `open` or `ajar` server, you must also implement this
method. The `ordinal` parameter is the FIDL method ordinal of the method that
was called. If the protocol was `open`, the `method_has_response` parameter
indicates whether the method was one-way or two-way; for a one-way method,
`method_has_response` is false, for a two-way method, it is true. In an `ajar`
protocol, only unknown one-way methods can be handled.

The `handle_unknown_method` method will be called any time the server receives
an unknown flexible method that it can handle.

#### Client-side

There is no way for the client to tell if a `flexible` one way-method was known
to the server or not. For `flexible` two-way methods, the [result
union](#protocols-results) can be used to tell whether the method was known to
the server. If the `transport_err` variant of the `TicTacToe_<Method>_Result`
class is set or the converted `fpromise::result` has the `fidl::TransportErr`
error set, that means that the server did not recognize the method.

The API for sending one-way methods and receiving events is the same for `strict`
and `flexible` one-way methods and events.

For `open` and `ajar` protocols, the generated `TicTacToe_Proxy` class will have an extra public field, called `handle_unknown_event` with this type:

```c++
fit::function<void(uint64_t)> handle_unknown_event;
```

Just like a an event handler for an event declared in the protocol, e.g.
`OnOpponentMove`, you can assign a function callback here to be called whenever
an unknown `flexible` event is received. The single `uint64_t` argument to the
callback is the FIDL method ordinal of the event.

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
attribute, the `virtual` methods on the protocol class are not pure. This allows
implementations of the protocol class with missing method overrides to compile
successfully.

#### Discoverable {#discoverable}

A protocol annotated with the
[`@discoverable`](/docs/reference/fidl/language/attributes.md#discoverable)
attribute causes the FIDL toolchain to generate an additional `static const char
Name_[]` field on the protocol class, containing the full protocol name. For a
protocol `Baz` in the library `foo.bar`, the generated name is `"foo.bar.Baz"`.

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
* `virtual void NotImplemented_(const std::string& name) = 0`: Pure virtual
  method that is overridden to define behavior for unimplemented methods.

`TicTacToe_TestBase` provides an implementation for the virtual protocol methods
`StartGame` and `MakeMove`, which are implemented to just call
`NotImplemented_("StartGame")` and `NotImplemented_("MakeMove")`, respectively.


## Extensions

[`fostr`][fostr] is a separate library that provides utilities for formatting
(pretty printing) FIDL types in HLCPP. Usage information can be found in the
[tutorial][fostr-tutorial].


<!-- xrefs -->
[anon-names]: /docs/reference/fidl/language/language.md#inline-layouts
[client-tut]: /docs/development/languages/fidl/tutorials/hlcpp/basics/client.md
[server-tut]: /docs/development/languages/fidl/tutorials/hlcpp/basics/server.md
[lang-constants]: /docs/reference/fidl/language/language.md#constants
[lang-bits]: /docs/reference/fidl/language/language.md#bits
[lang-enums]: /docs/reference/fidl/language/language.md#enums
[lang-flexible]: /docs/reference/fidl/language/language.md#strict-vs-flexible
[lang-structs]: /docs/reference/fidl/language/language.md#structs
[lang-tables]: /docs/reference/fidl/language/language.md#tables
[lang-unions]: /docs/reference/fidl/language/language.md#unions
[lang-protocols]: /docs/reference/fidl/language/language.md#protocols
[lang-protocol-composition]: /docs/reference/fidl/language/language.md#protocol-composition
[lang-resource]: /docs/reference/fidl/language/language.md#value-vs-resource
[union-lexicon]: /docs/reference/fidl/language/lexicon.md#union-terms
[unknown-attr]: /docs/reference/fidl/language/attributes.md#unknown
[traversal]: /docs/reference/fidl/language/wire-format/README.md#traversal-order
[fostr]: /src/lib/fostr
[fostr-tutorial]: /docs/development/languages/fidl/tutorials/hlcpp/topics/fostr.md
