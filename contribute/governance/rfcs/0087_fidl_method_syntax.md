{% set rfcid = "RFC-0087" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

After [RFC-0050], the parameters of a FIDL method request or response are
specified inline in the form of `(name1 Type1, name2 Type2)`, which implicitly
defines a request/response type of a struct with the parameters as its members.
This RFC proposes changing the syntax to specify the top level type explicitly,
e.g. `(struct { name1 Type1; name2 Type2; })`. Users will also be allowed to
specify request/response types of unions or tables in addition to structs. This
RFC has no wire format impact.

## Terminology

In this RFC, "wrapping a type in a struct" refers to the process of taking an
existing type and defining a new struct that consists of a single member of that
type. For example, wrapping a type `T` in a struct refers to defining a new type
`type Wrapped = struct { my_t T; }`. This technique can be used to work around
certain constraints of the FIDL language. For example, a `uint8` cannot be
nullable but a struct can, so it is possible to effectively have a nullable
`uint8` by first wrapping it into a struct. It is also worth noting that `T` and
`Wrapped` have the exact same wire format.

## Motivation

Changing the syntax to specify the top level type explicitly, e.g. `(struct {
name1 Type1; name2 Type2; })` provides two main benefits, by:

* putting the ABI implications at the forefront of the syntax, following FIDL's
  [design principles][abi-first]. As an example, writing `(struct { name1 Type1;
  })` instead of `(name1 Type1)` makes explicit the fact that the top level
  request or response type is a struct, and that therefore adding or removing a
  new parameter is not ABI or API compatible.
* making it possible for users to specify a different top level type without
  requiring an additional level of indirection of wrapping the type in a struct.
  Besides improving readability by having the definition inline, this allows the
  FIDL compiler to pick an appropriate name rather than putting this burden on
  the developer. One example scenario where this may be desirable is a method
  where the extensibility of the request parameters is a priority - in this
  case, the user can [use a table][extensible-method-args] instead of a struct.

The timing of the introduction of this RFC is linked to [RFC-0050] in two ways:

* The introduction of anonymous layouts in the RFC makes it possible to reuse
  this syntax for specifying the request/response type without needing to give
  it a separate name.
* The syntax change proposed in this RFC can be grouped into the existing
  implementation and migration required for [RFC-0050], obviating the need for a
  separate migration.

## Design

### Syntax {#design-syntax}

Before:

```fidl
protocol Oven {
  StartBake(temp Temperature);
  // message with no payload
  -> OnReady();
};
```

After:

```fidl
protocol Oven {
  StartBake(struct { temp Temperature; });
  // message with no payload
  -> OnReady();
};
```

The full set of possible method variations would be:

```
MyMethod(struct { ... }) -> (struct { ... });   // Two-way
MyMethod(struct { ... }) -> ();                 // Two-way, but response is empty
MyMethod() -> (struct { ... });                 // Two-way, but request is empty
MyMethod() -> ();                               // Two-way, but both request and response are empty
MyMethod() -> (struct { ... }) error zx.status; // Two-way; response leverages error syntax
MyMethod() -> () error zx.status;               // Error: must specify a type for success case.
MyMethod(struct { ... });                       // One-way
MyMethod();                                     // One-way, but request is empty
-> MyMethod(struct { ... })                     // Event
-> MyMethod();                                  // Event, but response is empty
```

Note: the syntax for a transactional message with no body does not change.
This is considered to be a readability improvement, since in the old syntax,
this case can be misconstrued to mean a transactional message with a body of
an empty struct: if `(foo Foo)` translates to a message with body
`struct { foo Foo; }`, a valid interpolation would be that `()` translates to
a message with body `struct {}`.  In the new syntax, `(T)` translates to a
message with body of type `T`, and `()` simply means a message with no body.

More formally, the grammar of

```
protocol-method = ( attribute-list ) , IDENTIFIER , parameter-list,
                  ( "->" , parameter-list , ( "error" type-constructor ) ) ;
protocol-event = ( attribute-list ) , "->" , IDENTIFIER , parameter-list ;
parameter-list = "(" , ( parameter ( "," , parameter )+ ) , ")" ;
parameter = ( attribute-list ) , type-constructor , IDENTIFIER ;
```

becomes:

```
protocol-method = ( attribute-list ) , IDENTIFIER , method-params
                  ( "->" , method-params ( "error" type-constructor ) ) ;
protocol-event = ( attribute-list ) , "->" , IDENTIFIER , method-params;
method-params = "(" , type , ")"
```

A `type` is [as defined in RFC-0050][grammar], i.e. it is either a
reference to an existing type like `MyType<args>:constraints`, or an anonymous
layout e.g. `struct { name Type; }:constraints`.

Though the grammar will allow arbitrary types to be used as requests and
responses, the FIDL compiler will validate that the top level types are either
structs, unions, or tables.

As [specified][flattened-name] in [RFC-0050], the compiler reserves a name for
any inlined top level request or response type which makes it possible to shift
away from an inlined style when this is desired (for example to improve
readability when the number of parameters increases). As an example, it is
possible to change from:

```fidl
protocol MyProtocol {
    Foo(struct {
      // input param
      input uint32;
    }) -> (struct {
      // output param
      output uint32;
    });
};
```

to:

```fidl
type FooRequest = struct {
  // input param
  input uint32;
};

type FooResponse = struct {
  // output param
  output uint32;
};

protocol MyProtocol {
    Foo(FooRequest) -> (FooResponse);
}
```

with no API or ABI impact (assuming that `FooRequest` and `FooResponse` are
the names that were reserved by the compiler).

### Bindings {#design-bindings}

The main impact in the bindings is that there may be cases where the API
corresponding to a set of request/response parameters is either flattened or not
depending on the top level type of the request or response. Currently there
exists instances of both flattened and non-flattened generated APIs.

Here, a "flattened" API refers to any API in the bindings that uses request and
response parameters directly, abstracting away the fact that they are wrapped in
a struct. For example, the function signature for the client call corresponding
to a FIDL method `GetName(struct { id uint32; }) -> (struct { name string; })`
in HLCPP is: `void GetName(uint32_t id, GetNameCallback callback)`. The
parameters specified in FIDL correspond directly to function parameters in C++.

A "non-flattened" API refers to the case where the top-level type itself is
exposed to the user. In the previous example, this would be something like:
`void GetName(GetNameRequest req, GetNameCallback callback)`. `GetNameRequest`
corresponds to the top level struct type, and would have a single `uint32` `id`
field.

Note: The question of how an appropriate name for the top level type  is chosen
is discussed in [RFC-0050][flattened-name].

In the current syntax where all top level request or response types are
implicitly structs, flattening the parameters so that they correspond directly
to the arguments of a function signature is OK because adding or removing a
struct member is both ABI and API incompatible anyways (i.e. this inlined API in
the generated bindings does not add additional restrictions to the guarantees
provided by FIDL). However, this is not the case for e.g. tables and unions,
which support adding and removing members. For this reason, there may be cases
where flattening cannot happen if the compatibility guarantees of the language
construct being used to represent the method (in this example, positional
function arguments in C++), are more restrictive than those provided by the top
level type (e.g. a table or a union). Going again with the example above, this
would mean that `GetName(table { 1: id uint32; }) -> (table { 1: name string;})`
would need to generate a non-flattened signature of the form `void
GetName(GetNameRequest req, GetNameCallback callback)` to maintain the
compatibility guarantees provided by the top level type of a table.

For generated functions or methods, some programming languages like Dart could
get around this by using [named arguments][dart-named-params] on the sending
side, but this would still be source incompatible on the receiving side due to
having to correspondingly add a new parameter to the receiving method.

In summary, bindings code that uses a flattened API for structs may need to
provide a different, non-flattened API if the top level type is a table or a
union. In cases where bindings currently already generate a non-flattened API -
for example, `MyProtocol::MyRequest` or `MyProtocol::MyResponse` in LLCPP, there
will be no such distinction between the API for a top level struct
request/response or a top level union or table request/response.

### JSON IR

The JSON entry for `maybe_request` and `maybe_response` will be changed. The old
schema of:

```
"maybe_request": {
    "description": "Optional list of interface method request parameters",
    "type": "array",
    "items": {
        "$ref": "#/definitions/interface-method-parameter"
    }
},
```

becomes:

```
"maybe_request_payload": {
    "description": "Optional type of the request",
    "$ref": "#/definitions/compound-identifier"
},
```

(and the same change for `maybe_response`)

The `"maybe_request_payload"` field already exists that matches this shape but
is not yet specified in the JSON IR as part of the work for ["Changing our
representation of messages"][fxbug.dev/7704]. In practice, the JSON IR change
for this RFC will involve completing the migration from `"maybe_request"` to the
`"maybe_request_payload"` (see [Implementation](#ir-implementation)).

## Implementation

There are two parts to the implementation of this RFC: the first is the purely
cosmetic change of modifying all existing files to conform to the new syntax
proposed here, and the second part is changing the FIDL compiler and bindings to
allow tables and unions as top level types. The syntax change will be
implemented as part of the broader [RFC-0050] FIDL syntax conversion but support
for union and table top-level types can be deferred so as to avoid being a
blocker for the FIDL syntax improvements project.  All FIDL files written in the
"new" syntax will be expected to conform to the changes laid out in this RFC,
and the formal FIDL grammar will be updated to reflect its design at the same
time as the rest of [RFC-0050].

There are some cases in the existing bindings where enabling top level types of
tables and unions for requests and responses will not require significant
changes besides handling the new JSON IR format. When this is not the case, i.e.
the encoding and decoding code in a binding relies on the assumption that the
top level type is a struct, there are two possible approaches:

* The first approach is to wrap any tables and unions into a struct first before
  encoding and decoding. This can be unappealing since it requires generating an
  additional type and adds an extra step to encoding and decoding.
* An alternative approach would be to modify the encoding/decoding code to
  support inputs that are not structs. Currently there is at least some code
  that assumes that the input is always a struct (for example, the correct
  [traits] in LLCPP are only generated for structs, and request and response
  encoding in Rust happens through tuples rather than structs), but the number
  of places where this assumption is currently unknown - this will need to be
  determined to understand the tradeoffs and ultimately decide between the two
  approaches. This latter approach may have benefits beyond method calls, for
  example it eliminates the need for wrapping types in a struct in a persistent
  data use case.

### JSON IR {#ir-implementation}

As part of [fxbug.dev/7704], a
migration was already in progress to move the `"maybe_request"` and
`"maybe_response"` fields out of the JSON IR so that any special treatment of
request and response types occur only in FIDL backends. This work was paused
before being completed, but will be resumed in order to implement this RFC.
Currently, the C++ backend is the only remaining fidlgen backend that uses
`"maybe_request"` and `"maybe_response"` (though other libraries using the JSON
IR, such as FIDL codec, will also need to be updated).

## Security and Privacy

This RFC does not modify the FIDL wire format and thus has no impact on security
and privacy.

## Testing

This RFC will be tested using existing infrastructure: unit tests, golden tests,
and integration tests (e.g. FIDL compatibility tests).

## Documentation

As this feature is enabled, documentation (including examples) should be added
to describe the new functionality.

## Drawbacks, Alternatives & Unknowns

### Syntax

The syntax suggested in this RFC makes the common path of using a struct as the
top level type more verbose, since it needs to be specified explicitly.
Alternatives could include introducing syntactic sugar for the common case (e.g.
keeping the current syntax for structs, and using the new explicit syntax for
tables and unions), but the readability of being explicit in all cases is
considered more important than reducing the verbosity.

Another part of the syntax that may be considered unappealing is the redundancy
in brackets: `(struct { ... })`, which was an issue that was also discussed in
FTP-058. Here there is a preference for consistency: keeping the curly braces
ensures that the syntax for a type inside of a request is the same as the syntax
for a type anywhere else in a FIDL file. The approach taken in FTP-058 to avoid
redundant braces by replacing them with spaces (e.g. `MyMethod struct { ... } ->
union { ... };`) could be valid here as well. In FIDL text, this more functional
style was consistent with the rest of the proposal and aligned with the syntax
used in the Fuchsia shell, whereas here it is inconsistent with the rest of
FIDL's more C-family/Go based syntax.

Finally, another alternative that was suggested was to instead change the syntax
used to specify types to align with method parameter syntax: structs would be
specified using a tuple/record like syntax: `type MyStruct = (foo Foo, bar
Bar);`. This would then allow us to keep the same parameter syntax when the top
level type is a struct by omitting the extra set of parentheses `MyMethod(foo
Foo, bar Bar);`. As a full example, this suggestion would look like:

```fidl
// Declare a struct with two fields foo, bar.
type SomeStruct = (foo Foo, bar Bar);

protocol MyProtocol {
  // Declare a method with two request parameters.
  // The two parameters are stored in a struct.
  MyStructMethod(foo Foo, bar Bar);

  // Declare a method with two optional parameters.
  // The two parameters are stored in a table.
  MyTableMethod table { 1: foo Foo, 2: bar Bar };
};
```

### Bindings

As mentioned in the [design](#design), in many cases bindings cannot flatten or
inline the top level type's members in generated APIs for tables and unions in
the same way they can for structs so as not to introduce additional
compatibility constraints. The rules on when a method's top level type members
get inlined or not may not be straightforward for users to memorize - this means
that they will need to rely on documentation or generated code inspection to
determine what the resulting API for each FIDL protocol method is. This
introduces some complexity over the current situation, where bindings APIs
consistently inline/flatten the top level type members.

In theory it is possible to provide a consistent API by never flattening request
or response parameters, but this is considered infeasible in practice as it
requires migrating all instances of user code that depend on this API (which
is most of user code interacting with FIDL methods).

# Prior Art & References

The syntax suggested in this RFC is closer to that used in gRPC, where method
request and responses are specified using a single protobuf message.

A similar idea to this RFC was previously suggested by ctiller@google.com,
allowing ordinal syntax (e.g. `MyMethod(1: foo Foo; 2: bar Bar)`) to imply that
the top level type is a table instead of a struct. The main difference is that
this RFC supports top level unions in addition to tables and structs, which
makes the originally suggested semantics ambiguous given that unions also use
ordinals.

<!-- xrefs -->

[RFC-0050]: /docs/contribute/governance/rfcs/0050_syntax_revamp.md
[traits]: /zircon/system/ulib/fidl/include/lib/fidl/llcpp/traits.h
[fxbug.dev/7704]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=7704
[extensible-method-args]: /docs/contribute/governance/rfcs/0050_syntax_revamp.md#extensible-method-args
[flattened-name]: /docs/contribute/governance/rfcs/0050_syntax_revamp.md#flattened-name
[grammar]: /docs/contribute/governance/rfcs/0050_syntax_revamp.md#grammar
[abi-first]: /docs/contribute/governance/rfcs/0050_syntax_revamp.md#abi-first
[dart-named-params]: https://dart.dev/guides/language/language-tour#named-parameters
