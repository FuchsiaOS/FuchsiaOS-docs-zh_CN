# Dart bindings

## Libraries {#libraries}

Given the library declaration:

```fidl
library fuchsia.examples;
```

The bindings code for this library is generated into a
`fidl_fuchsia_examples_async` dart library. The `fidl_` prefix and `_async`
suffix are hardcoded by the FIDL toolchain.

This code can then be imported using:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="import" %}
```

## Constants {#constants}

All [constants][lang-constants] are generated as a `const`. For example, the
following constants:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="consts" %}
```

Are generated as:

```dart
const int BOARD_SIZE = 9;
const String NAME = "Tic-Tac-Toe";
```

The correspondence between FIDL primitive types and Dart types is outlined in
[built-in types](#builtins).

## Fields {#fields}

This section describes how the FIDL toolchain converts FIDL types to native
types in Dart. These types can appear as members in an aggregate type, as
parameters to a protocol method, or as the type contained in an event or method
response `Future`.

Nullable types do not have different generated types than their non-nullable
counterparts in Dart.

### Built-in types {#builtins}

The FIDL types are converted to Dart types based on the following table:

|FIDL Type|Dart Type|
|--- |--- |
|`bool`|`bool`|
|`int8`, `int16`, `int32`, `int64`, `uint8`, `uint16`, `uint32`, `uint64`|`int`|
|`float32`, `float64`|`double`|
|`array<int8, N>`, `vector<int8>:N`|`Int8List`|
|`array<int16, N>`, `vector<int16>:N`|`Int16List`|
|`array<int32, N>`, `vector<int32>:N`|`Int32List`|
|`array<int64, N>`, `vector<int64>:N`|`Int64List`|
|`array<uint8, N>`, `vector<uint8>:N`|`Uint8List`|
|`array<uint16, N>`, `vector<uint16>:N`|`Uint16List`|
|`array<uint32, N>`, `vector<uint32>:N`|`Uint32List`|
|`array<uint64, N>`, `vector<uint64>:N`|`Uint64List`|
|`array<float32, N>`, `vector<float32>:N`|`Float32List`|
|`array<float64, N>`, `vector<float64>:N`|`Float64List`|
|`array<T, N>`, `vector<T>:N`|`List<T>`|
|`string`|`String`|
|`server_end:P`|`fidl.InterfaceRequest<P>`|
|`client_end:P`|`fidl.InterfaceHandle<P>`|
|`zx.handle:CHANNEL`|`zircon.Channel`|
|`zx.handle:EVENTPAIR`|`zircon.EventPair`|
|`zx.handle:SOCKET`|`zircon.Socket`|
|`zx.handle:VMO`|`zircon.Vmo`|
|`zx.handle:S`, `zx.handle`|`zircon.Handle`|

### Response and event parameters {#response-event-parameters}

Method response and event types (see [Protocols](#protocols)) are represented
using `Future<T>`, where `T` is a type containing all of the response/event
parameters. This section describes how the FIDL toolchain generates this inner
type `T`.

* Empty responses and events use `void`.
* Responses and events with a single parameter `T` just use `T` as the response
  or event type.
* Responses and events with multiple parameters use a generated wrapper class
  which follows the naming scheme `[Protocol]$[Method]$Response`. For example,
  an event `OnOpponentMove` for protocol `TicTacToe` that has multiple
  parameters would use generated class `TicTacToe$OnOpponentMove$Response`. This
  class provides a single method: the constructor, which has positional
  arguments corresponding to the response or event parameters.

Note that methods that do not have a response will have a response type of
`Future<void>`, which is the same type used by methods with an empty response.
In the former case, the `Future` can be expected to resolve immediately after
sending the request, whereas in the latter case, the `Future` is only resolved
after receiving the empty response from the server.

## Type definitions {#type-definitions}

### Bits {#bits}

Given the [bits][lang-bits] definition:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="bits" %}
```

The FIDL toolchain generates a `FileMode` class with `static const` variables
for each bits member, as well as for a `FileMode` with no flag set (`$none`)
or every flag set (`$mask`):

* `static const FileMode read`
* `static const FileMode write`
* `static const FileMode execute`
* `static const FileMode $none`
* `static const FileMode $mask`

`FileMode` provides the following methods:

* `int get $value`: Getter for the underlying int value.
* `String toString()`: Returns a readable representation of the `FileMode`.
* `FileMode operator |(FileMode other)`: Bitwise or operator.
* `FileMode operator &(FileMode other)`: Bitwise and operator.
* `bool operator(dynamic other)`: Equality operator.
* `int getUnknownBits()`: Returns only the set bits that are unknown. Always
  returns 0 for [strict][lang-flexible] bits.
* `bool hasUnknownBits()`: Returns whether this value contains any unknown bits.
  Always returns `false` for [strict][lang-flexible] bits.

Example usage:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="bits" adjust_indentation="auto" %}
```

### Enums {#enums}

Given the [enum][lang-enums] definition:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="enums" %}
```

The FIDL toolchain generates a `LocationType` class with `static const`
variables for each enum member:

* `static const LocationType museum`
* `static const LocationType airport`
* `static const LocationType restaurant`

As well as the following variables:

* `static const Map<String, LocationType> $valuesMap`: A mapping of the string
  representation of the member (`'museum'`, `'airport'`, or `'restaurant'`) to
  its corresponding enum value (`LocationType.museum`, `LocationType.airport`,
  or `LocationType.restaurant`)
* `static const List<LocationType> $values`: A list of all of the enum values.

If `LocationType` is [flexible][lang-flexible], it will have an unknown
placeholder member as well:

* `static const LocationType $unknown`

If the enum has a member tagged with the [`[Unknown]`][unknown-attr] attribute,
the placeholder variable will have the same value as the tagged unknown
member.

`LocationType` provides the following methods:

* `static LocationType $valueOf(String name)`: Look up a string name in the
  `$valuesMap`.
* `String toString()`: Returns a readable representation of the `LocationType`.
* `bool isUnknown()`: Returns whether this enum is unknown. Always returns
  `false` for [strict][lang-flexible] enums.

Example usage:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="enums" adjust_indentation="auto" %}
```

### Structs {#structs}

Given the [struct][lang-structs] declaration:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="structs" %}
```

The FIDL toolchain generates a `Color` class with the following methods:

* `const Color({@required id, name})`: The constructor for `Color` takes named
  arguments corresponding to the `struct`'s fields. Fields that are not nullable
  and do not have a default value specified are marked as `@required`.
* `int get id`: Getter for the `id` field.
* `String get name`: Getter for the `name` field.
* `Color.clone(Color, {int id, String name})`: Clone constructor that will clone
  an existing `Color`, possibly overriding specific field values based on the
  provided named arguments.
* `List<Object> get $fields`: Returns a list of fields in declaration order.
* `String toString()`: Returns a readable string of the `Color`
* `bool operator==(dynamic other)`: Equality operator that performs a deep
  comparison when compared to another instance of a `Color`.

Example usage:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="structs" adjust_indentation="auto" %}
```

### Unions {#unions}

Given the union definition:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="unions" %}
```

FIDL generates an `enum` representing the [tags][union-lexicon] of the union:

```dart
enum JsonValueTag {
  intValue,
  stringValue,
}
```

As well as a `JsonValue` class with the following methods:

* `const JsonValue.withIntValue(int)` and `const
  JsonValue.withStringValue(String)`: Constructors for each variant.
* `JsonValueTag get $tag`: Getter for the tag corresponding to this the
  [variant][union-lexicon] of this union.
* `int? get intValue` and `String? get stringValue`: Getter for the underlying
  value. If the instance's variant does not match the getter method, `null` is
  returned.
* `String toString()`: Returns a readable string of the `JsonValue`.
* `int get $ordinal`: Getter for the underlying [ordinal][union-lexicon] value.
* `Object get $data`: Getter for the underlying union data.
* `bool operator ==(dynamic other)`: Equality operator that performs deep
   comparison when compared to another `JsonValue` of the same variant.
* `fidl.UnknownRawData? get $unknownData`: Returns the bytes and handles of the
  unknown data if this union contains an unknown variant, or `null` otherwise.
  Always returns `null` for [strict][lang-flexible] unions.

If `JsonValue` is [flexible][lang-flexible], it will have the following
additional methods:

* `const JsonValue.with$UnknownData(int ordinal, fidl.UnknownRawData data)`:
  Constructor for a value with an unknown variant set. This should only be used
  for testing, e.g. to check that code handles unknown unions correctly.

Example usage:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="unions" adjust_indentation="auto" %}
```

#### Flexible unions and unknown variants

[Flexible][lang-flexible] unions have an extra variant in the generated tag
class:

```dart
enum JsonValueTag {
  $unknown,
  intValue,
  stringValue,
}
```

When a FIDL message containing a union with an unknown variant is decoded into
`JsonValue`, `JsonValue.$tag` returns `JsonValueTag.$unknown`, and
`JsonValue.$ordinal` returns the unknown ordinal.

Encoding a union with an unknown variant writes the unknown data and the
original ordinal back onto the wire.

[Strict][lang-flexible] unions fail when decoding an unknown variant.
[Flexible][lang-flexible] unions that are [value][lang-resource] types fail when
decoding an unknown variant with handles.

### Tables {#tables}

Given the [table][lang-tables] definition:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="tables" %}
```

The FIDL toolchain generates a `User` class that defines the following methods:

* `const User({$unknownData, age, name})`: Constructor for `User`. Contains an
  optional parameter for each field as well as a map containing any unknown
  fields, as a `Map<int, fidl.UnknownRawData>`. Specifying a value for the
  unknown fields should only be done for testing, e.g. to test that a table with
  unknown fields is handled correctly.
* `int get age`: Getter for the `age` field.
* `String get name`: Getter for the `name` field.
* `Map<int, dynamic> get $fields`: Returns a map of ordinals to field values.
* `Map<int, fidl.UnknownRawData>? get $unknownData`: Returns a map of ordinals
  to unknown field values (i.e. bytes and handles). The list of handles is
  returned in [traversal order][traversal], and is guaranteed to be empty if the
  table is a [value][lang-resource] type.
* `bool operator ==(dynamic other)`: Equality operator that performs deep
  comparison when compared to another `User`.

Example usage:

```dart
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="tables" adjust_indentation="auto" %}
```

### Inline layouts

The generated Dart code uses the [the name reserved by `fidlc`][anon-names] for
inline layouts.

## Protocols {#protocols}

Given the [protocol][lang-protocols]:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/types.test.fidl" region_tag="protocols" %}
```

Note: The `MakeMove` method above returns a bool representing success, and a
nullable response value. This is considered un-idiomatic, you should use an
[error type](#protocols-results) instead.

FIDL generates an abstract `TicTacToe` class, which defines the interface of the
service used by clients to proxy calls to the server, and for the server for
implementing the protocol.

`TicTacToe` contains a `static const String $serviceName`, which is defined
depending on the presence of the [@transitional attribute](#transitional).

`TicTacToe` has the following abstract methods, representing the protocol
methods:

* `async.Future<void> startGame(bool start_first)`: Abstract method for a fire
  and forget protocol method. It takes as arguments the request parameters and
  returns a future of `void`.
* `async.Future<TicTacToe$MakeMove$Response> makeMove(int row, int col)`:
  Abstract method for a two way protocol method. It takes as arguments the
  request parameters and returns a [future of the response
  type](#response-event-parameters).
* `async.Stream<GameState> get onOpponentMove`: Getter for a `Stream` of
  `onOpponentMove` events.

### Client {#proxy}

The FIDL toolchain generates a `TicTacToeProxy` class that extends
`fidl.AsyncProxy<TicTacToe>`, and provides an implementation for the abstract
`TicTacToe` class that encodes and sends the request to the server end of the
channel.

Example client code could thus look like the following:

```dart
final tictactoe = fidl_tictactoe.TicTacToeProxy();
// ...bind the proxy, omitted from this example
tictactoe.startGame(true);
final state = await tictactoe.makeMove(0, 0);
```

Examples on how to set up and bind a proxy class to a channel are covered in the
[Dart tutorial][dart-tutorial].

### Server {#server}

Implementing a server for a FIDL protocol involves providing a concrete
implementation of `TicTacToe` abstract class.

The bindings provide a `TicTacToeBinding` class that can bind to a `TicTacToe`
instance and a channel, and listens to incoming messages on the channel,
dispatches them to the server implementation, and sends messages back through
the channel. This class implements
<!-- TODO(fxbug.dev/58672) add link to API docs when those are available -->
`fidl.AsyncBinding<TicTacToe>`.

Examples on how to set up and bind a server implementation are covered in the
[Dart tutorial][dart-tutorial].

### Events {#events}

#### Client

The `TicTacToeProxy` class automatically implements the `onOpponentMove` getter.
Clients obtain an `async.Stream` of `onOpponentMove` events sent from the server
using this getter.

#### Server
<!-- TODO(fxbug.dev/58672) add link to API docs when those are available -->
Servers send events by implementing the `onOpponentMove` getter on the abstract
`TicTacToe` class. A `TicTacToeBinding` (see [tutorial][dart-tutorial]) that is
bound to an instance of `TicTacToe` that has implemented the `onOpponentMove`
getter will listen for events on the returned `async.Stream`, forwarding them to
the client.

### Results {#protocols-results}

Given the method with an error type:

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

The method signature for `MakeMove` on the generated abstract `TicTacToe` class
is:

```dart
async.Future<GameState> makeMove(int row, int col)
```

The encapsulated `Future` corresponds to the generated [response
type](#response-event-parameters) for the success case, and the error case is
represented by having the server implementation or the proxy class throw a
`fidl.MethodException`.

Note: Unlike the [previous example](#protocols), the response type is just a
`Future<GameState>` instead of a `TicTacToe$MakeMove$Response` class. This is
because the method went from having two parameters to one parameter,
following the [response and event type rules](#response-event-parameters).

Using this feature, an example implementation of `MakeMove` on the server side
could look like:

```dart
@override
async.Future<GameState> makeMove(int row, int col) {
  if (row > 9 || col > 9) {
    return async.Future.error(fidl.MethodException(MoveError.OutOfBounds));
  }
  return async.Future.value(doSomething(row, col));
}
```

The `TicTacToeBinding` class will `catch` `fidl.MethodException`s and encode it
as an error.

An example of using this on the client side would be:

```dart
myproxy.makeMove(1, 2).then((gameState) { ... })
                      .catchError((moveError) { ... });

```

### Protocol composition {#protocol-composition}

FIDL does not have a concept of inheritance, and generates full code as
described above for all [composed protocols][lang-protocol-composition]. In
other words, the code generated for:

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

#### Transitional {#transitional}

For protocol methods annotated with the
[`@transitional`](reference/fidl/language/attributes.md#transitional)
attribute, the FIDL toolchain generates a default implementation on the abstract
class so that server implementations will continue to compile without having to
override the new method.

#### Discoverable {#discoverable}

The generated class for a protocol annotated with the
[`@discoverable`](reference/fidl/language/attributes.md#discoverable)
attribute has a non-null `$serviceName` field.

### Test scaffolding {#test-scaffolding}

The FIDL toolchain generates a `fidl_test.dart` file that contains convenience
code for testing FIDL server implementations. This file contains a class for
each protocol that provides stub implementations for each of the class’s
methods, making it possible to implement only the methods that are used during
testing.

Given the example protocol above, The FIDL toolchain generates a
`TicTacToe$TestBase` class that extends the `TicTacToe` abstract class. All
methods are implemented by returning `async.Future.error(UnimplementedError())`,
and all events are implemented by returning a Stream with a single
`UnimplementedError` event.

<!-- xrefs -->
[anon-names]: reference/fidl/language/language.md#inline-layouts
[dart-tutorial]: development/languages/fidl/tutorials/dart
[lang-constants]: reference/fidl/language/language.md#constants
[lang-bits]: reference/fidl/language/language.md#bits
[lang-enums]: reference/fidl/language/language.md#enums
[lang-flexible]: reference/fidl/language/language.md#strict-vs-flexible
[lang-structs]: reference/fidl/language/language.md#structs
[lang-tables]: reference/fidl/language/language.md#tables
[lang-unions]: reference/fidl/language/language.md#unions
[lang-resource]: reference/fidl/language/language.md#value-vs-resource
[lang-protocols]: reference/fidl/language/language.md#protocols
[lang-protocol-composition]: reference/fidl/language/language.md#protocol-composition
[union-lexicon]: reference/fidl/language/lexicon.md#union-terms
[unknown-attr]: reference/fidl/language/attributes.md#unknown
[traversal]: reference/fidl/language/wire-format/README.md#traversal-order
