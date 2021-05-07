# FIDL language specification

This document is a specification of the Fuchsia Interface Definition Language
(**FIDL**) syntax.

For more information about FIDL's overall purpose, goals, and requirements,
see [Overview][fidl-overview].

Also, see a modified [EBNF description of the FIDL grammar][fidl-grammar].

[TOC]

## Syntax

FIDL provides a syntax for declaring named bits, constants, enums, structs,
tables, unions, and protocols. These declarations are collected into libraries
for distribution.

FIDL declarations are stored in plain text UTF-8 files. Each file consists of a
sequence of semicolon-delimited declarations. The order of declarations within a
FIDL file, or among FIDL files within a library, is irrelevant. FIDL does not
require (or support) forward declarations of any kind.

### Comments

FIDL comments start with two (`//`) or three (`///`) forward slashes, continue
to the end of the line, and can contain UTF-8 content (which is, of course, ignored).
The three-forward-slash variant is a "documentation comment", and causes the comment
text to be emitted into the generated code (as a comment, escaped correctly
for the target language).

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="comments" %}
```

Note that documentation comments can also be provided via the
[`[Doc]` attribute][doc-attribute].

### Keywords

The following are keywords in FIDL.

```
as, bits, compose, const, enum, library, protocol,
resource, struct, table, union, using, xunion.
```

### Identifiers

FIDL _identifiers_ label declarations and their members. FIDL identifiers must
match the regex `[a-zA-Z]([a-zA-Z0-9_]*[a-zA-Z0-9])?`. In words: identifiers
must start with a letter, can contain letters, numbers, and underscores, but
cannot end with an underscore.

```fidl
// a struct named "Foo"
struct Foo { };

// an enum named "enum", containing a single member
enum enum { WITH_A_MEMBER = 1; };
```

Note: While using keywords as identifiers is supported, it can lead to
confusion, and should therefore be considered on a case-by-case basis. See the
`Names` section of the [Style Rubric][naming-style].

FIDL _library names_ label [FIDL libraries](#libraries). FIDL library names
consist of one or more elements each matching the regex `[a-z][a-z0-9]*`. In
words: library name elements must start with a lowercase letter, can contain
lowercase letters, and numbers (they cannot contain uppercase letters, nor
underscores). Library names are used in [Qualified
Identifiers](#qualified-identifiers).

```fidl
// a library named "foo"
library foo;
```

Identifiers and library names are case-sensitive.

### Qualified Identifiers {#qualified-identifiers}

FIDL always looks for unqualified symbols within the scope of the current
library. To reference symbols in other libraries, they must be qualified by
prefixing the identifier with the library name or alias thereof.

**objects.fidl:**

```fidl
library objects;
using textures as tex;

protocol Frob {
    // "Thing" refers to "Thing" in the "objects" library
    // "tex.Color" refers to "Color" in the "textures" library
    Paint(Thing thing, tex.Color color);
};

struct Thing {
    string name;
};
```

**textures.fidl:**

```fidl
library textures;

struct Color {
    uint32 rgba;
};
```

### Literals

FIDL supports integer, floating point, boolean, string, and enumeration literals, using
a simplified syntax familiar to C programmers (see below for examples).

### Constants {#constants}

FIDL supports the following constant types: bits, booleans, signed and unsigned
integers, floating point values, strings, and enumerations.
The syntax is similar to C:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="consts" %}
```

These declarations introduce a name within their scope.
The constant's type must be either a primitive or an enum.

Constant expressions are either literals or the names of other
constant expressions.

> For greater clarity, there is no expression processing in FIDL; that is,
> you *cannot* declare a constant as having the value `6 + 5`, for
> example.

### Default Initialization

Primitive structure members may have initialization values specified
in the declaration.
For example:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="defaults" %}
```

If the programmer does not supply a background color, the default
value of `0xFF77FF` will be used.

However, if the program does not supply a foreground color, there is no
default.
The foreground color must be supplied; otherwise it's a logic error on
the programmer's part.

There is a subtlety about the semantics and what defaults mean:

* If the target language can support defaults (Dart, C++)
    * then it MUST support defaults
* If the target language cannot support defaults (C, Rust, Go)
    * then it MAY provide support that programmers can optionally
      invoke (e.g., a macro in C).

### Declaration Separator

FIDL uses the semi-colon **';'** to separate adjacent declarations within the
file, much like C.

## Libraries {#libraries}

Libraries are named containers of FIDL declarations.

```fidl
// library identifier separated by dots
library fuchsia.composition;

// "using" to import library "fuchsia.buffers"
using fuchsia.buffers;

// "using" to import library "fuchsia.geometry" and create a shortform called "geo"
using fuchsia.geometry as geo;
```

Libraries may declare that they use other libraries with a "using" declaration.
This allows the library to refer to symbols defined in other libraries upon which
they depend. Symbols imported this way may be accessed by:

*   qualifying them with the fully qualified library name (as in _"fuchsia.geometry.Rect"_),
*   specifying just the library name (as in _"geometry.Rect"_), or,
*   using a library alias (as in _"geo.Rect"_).

In the source tree, each library consists of a directory with some number of
**.fidl** files. The name of the directory is irrelevant to the FIDL compiler
but by convention it should resemble the library name itself. A directory should
not contain FIDL files for more than one library.

The scope of "library" and "using" declarations is limited to a single file.
Each individual file within a FIDL library must restate the "library"
declaration together with any "using" declarations needed by that file.

The library's name may be used by certain language bindings to provide scoping
for symbols emitted by the code generator.

For example, the C++ bindings generator places declarations for the
FIDL library "fuchsia.ui" within the C++ namespace
"fuchsia::ui". Similarly, for languages such as Dart and Rust, which
have their own module system, each FIDL library is compiled as a
module for that language.

## Types and Type Declarations

FIDL supports a number of builtin types as well as declarations of new types
(e.g. structs, unions, type aliases) and protocols.

### Primitives

*   Simple value types.
*   Not nullable.

The following primitive types are supported:

*    Boolean                 **`bool`**
*    Signed integer          **`int8 int16 int32 int64`**
*    Unsigned integer        **`uint8 uint16 uint32 uint64`**
*    IEEE 754 Floating-point **`float32 float64`**

Numbers are suffixed with their size in bits, **`bool`** is 1
byte.

We also alias **`byte`** to mean **`uint8`** as a [built-in alias](#built-in-aliases).

#### Use

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="primitives" %}
```

### Bits {#bits}

* Named bit types.
* Discrete subset of bit values chosen from an underlying integer primitive
  type.
* Not nullable.
* Bits must have at least one member.
* Bits can either be [`strict` or `flexible`](#strict-vs-flexible).
  <!-- TODO(fxbug.dev/64463): update default -->
  * Bits default to `strict`.

#### Use

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="bits" %}
```

### Enums {#enums}

* Proper enumerated types.
* Discrete subset of named values chosen from an underlying integer primitive
  type.
* Not nullable.
* Enums must have at least one member.
* Enums can be [`strict` or `flexible`](#strict-vs-flexible).
  <!-- TODO(fxbug.dev/64463): update default -->
  * Enums default to `strict`.

#### Declaration

The ordinal index is **required** for each enum element. The underlying type of
an enum must be one of: **int8, uint8, int16, uint16, int32, uint32, int64,
uint64**. If omitted, the underlying type is assumed to be **uint32**.

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="enums" %}
```

#### Use

Enum types are denoted by their identifier, which may be qualified if needed.

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="enum-use" %}
```

### Arrays

*   Fixed-length sequences of homogeneous elements.
*   Elements can be of any type including: primitives, enums, arrays, strings,
    vectors, handles, structs, tables, unions.
*   Not nullable themselves; may contain nullable types.

#### Use

Arrays are denoted **`array<T>:n`** where _T_ can
be any FIDL type (including an array) and _n_ is a positive
integer constant expression that specifies the number of elements in
the array.

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="arrays" %}
```

### Strings

*   Variable-length sequence of UTF-8 encoded characters representing text.
*   Nullable; null strings and empty strings are distinct.
*   Can specify a maximum size, eg. **`string:40`** for a
    maximum 40 byte string.
*   May contain embedded `NUL` bytes, unlike traditional C strings.

#### Use

Strings are denoted as follows:

*   **`string`** : non-nullable string ([validation error][lexicon-validate]
    occurs if null is encountered)
*   **`string?`** : nullable string
*   **`string:N, string:N?`** : string, and nullable string, respectively,
    with maximum length of _N_ bytes

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="strings" %}
```

> Strings should not be used to pass arbitrary binary data since bindings enforce
> valid UTF-8. Instead, consider `bytes` for small data or
> [`fuchsia.mem.Buffer`](/docs/concepts/api/fidl.md#consider-using-fuchsia_mem_buffer)
> for blobs. See
> [Should I use string or vector?](/docs/concepts/api/fidl.md#should-i-use-string-or-vector)
> for details.

### Vectors

*   Variable-length sequence of homogeneous elements.
*   Nullable; null vectors and empty vectors are distinct.
*   Can specify a maximum size, eg. **`vector<T>:40`** for a
    maximum 40 element vector.
*   There is no special case for vectors of bools. Each bool element takes one
    byte as usual.
*   We have a [built-in alias](#built-in-aliases) for **`bytes`** to mean
    `vector<uint8>`, and it can be size bound in a similar fashion e.g.
    `bytes:1024`.

#### Use

Vectors are denoted as follows:

*   **`vector<T>`** : non-nullable vector of element type
    _T_ ([validation error][lexicon-validate] occurs if null is encountered)
*   **`vector<T>?`** : nullable vector of element type
    _T_
*   **`vector<T>:N, vector<T>:N?`** : vector, and nullable vector, respectively,
    with maximum length of _N_ elements

_T_ can be any FIDL type.

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="vectors" %}
```

### Handles {#handles}

*   Transfers a Zircon capability by handle value.
*   Stored as a 32-bit unsigned integer.
*   Nullable by encoding as a zero-valued handle.

#### Use

Handles are denoted:

*   **`handle`** : non-nullable Zircon handle of
    unspecified type
*   **`handle?`** : nullable Zircon handle of
    unspecified type
*   **`zx.handle:H`** : non-nullable Zircon handle
    of type _H_
*   **`zx.handle:H?`** : nullable Zircon handle of
    type _H_

_H_ can be any [object](/docs/reference/kernel_objects/objects.md) supported by
Zircon, e.g. `channel`, `thread`, `vmo`. Please refer to the
[grammar](grammar.md) for a full list.

Structs, tables, and unions containing handles must be marked with the
[`resource` modifier](#value-vs-resource).

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="handles" %}
```

### Structs {#structs}

*   Record type consisting of a sequence of typed fields.
*   Declaration is not intended to be modified once deployed; use protocol
    extension instead.
*   Declaration can have the [`resource` modifier](#value-vs-resource).
*   Reference may be nullable.
*   Structs contain zero or more members.

#### Declaration

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="structs" %}
```

#### Use

Structs are denoted by their declared name (eg. **Circle**) and nullability:

*   **`Circle`** : non-nullable Circle
*   **`Circle?`** : nullable Circle

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="structs-use" %}
```

### Tables {#tables}

*   Record type consisting of a sequence of typed fields with ordinals.
*   Declaration is intended for forward and backward compatibility in the face of schema changes.
*   Declaration can have the [`resource` modifier](#value-vs-resource).
*   Tables cannot be nullable. The semantics of "missing value" is expressed by an empty table
    i.e. where all members are absent, to avoid dealing with double nullability.
*   Tables contain zero or more members.

#### Declaration

```fidl
table Profile {
    1: vector<string> locales;
    2: vector<string> calendars;
    3: vector<string> time_zones;
};
```

#### Use

Tables are denoted by their declared name (eg. **Profile**):

*   **`Profile`** : non-nullable Profile

Here, we show how `Profile` evolves to also carry temperature units.
A client aware of the previous definition of `Profile` (without temperature units)
can still send its profile to a server that has been updated to handle the larger
set of fields.

```fidl
enum TemperatureUnit {
    CELSIUS = 1;
    FAHRENHEIT = 2;
};

table Profile {
    1: vector<string> locales;
    2: vector<string> calendars;
    3: vector<string> time_zones;
    4: TemperatureUnit temperature_unit;
};
```

### Unions {#unions}

* Record type consisting of an ordinal and an envelope.
* Ordinal indicates member selection, envelope holds contents.
* Declaration can be modified after deployment, while maintaining ABI
  compatibility. See the [Compatibility Guide][union-compat] for
  source-compatibility considerations.
* Declaration can have the [`resource` modifier](#value-vs-resource).
* Reference may be nullable.
* Unions contain one or more members. A union with no members would have no
  inhabitants and thus would make little sense in a wire format.
* Unions can either be [`strict` or `flexible`](#strict-vs-flexible).
  <!-- TODO(fxbug.dev/64463): update default -->
  * Unions default to `strict`.

#### Declaration

```fidl
{%includecode gerrit_repo="fuchsia/samples" gerrit_path="src/calculator/fidl/calculator.fidl" region_tag="union" %}
```

#### Use {#unions-use}

Unions are denoted by their declared name (e.g. **Result**) and nullability:

*   **`Either`** : non-nullable Result
*   **`Either?`** : nullable Result

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="unions-use" %}
```

### Strict vs. Flexible {#strict-vs-flexible}

FIDL declarations can either have **strict** or **flexible** behavior:

<!-- TODO(fxbug.dev/64463): Update this when defaults change. -->
*   Bits, enums, and unions are strict unless declared with the `flexible`
    modifier.
*   Structs always have strict behavior.
*   Tables always have flexible behavior.

For strict types only, serializing or deserializing a value that contains data
not described in the declaration is a [validation error][lexicon-validate].

In this example:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="strict-vs-flexible" %}
```

By virtue of being flexible, it is simpler for `FlexibleEither` to evolve to
carry a third variant. A client aware of the previous definition of
`FlexibleEither` without the third variant can still receive a union from a
server that has been updated to contain the larger set of variants. If the
union is of the unknown variant, bindings may expose it as unknown data (i.e. as
raw bytes and handles) to the user and allow re-encoding the unknown union (e.g.
to support proxy-like use cases). The methods provided for interacting with
unknown data for flexible types are described in detail in the [bindings
reference][bindings-reference].

More details are discussed in
[RFC-0033: Handling of Unknown Fields and Strictness][rfc-033].

Note: A type that is both flexible and a [value type](#value-vs-resource) will
not allow deserializing unknown data that contains handles.

### Value vs. Resource {#value-vs-resource}

Every FIDL type is either a **value type** or a **resource type**. Resource
types include:

*   [handles](#handles)
*   [protocol endpoints](#protocols-use)
*   [aliases](#aliasing) of resource types
*   arrays and vectors of resource types
*   structs, tables, and unions marked with the `resource` modifier
*   nullable references to any of the above types

All other types are value types.

Value types must not contain resource types. For example, this is incorrect:

```fidl
struct Foo { // ERROR: must be "resource struct Foo"
    handle h;
};
```

Types can be marked with the `resource` modifier even if they do not contain
handles. You should do this if you intend to add handles to the type in the
future, since adding or removing the `resource` modifier requires
[source-compatibility considerations][resource-compat]. For example:

```fidl
// No handles now, but we will add some in the future.
resource table Record {
    1: string str;
};

// "Foo" must be a resource because it contains "Record", which is a resource.
resource struct Foo {
    Record record;
};
```

More details are discussed in [RFC-0057: Default No Handles][rfc-0057].

### Protocols {#protocols}

*   Describe methods that can be invoked by sending messages over a channel.
*   Methods are identified by their ordinal index. The compiler calculates the ordinal by
    * Taking the SHA-256 hash of the string generated by concatenating:
        * The UTF-8 encoded library name, with no trailing \0 character
        * '.' (ASCII 0x2e)
        * The UTF-8 encoded protocol name, with no trailing \0 character
        * '/' (ASCII 0x2f)
        * The UTF-8 encoded method name, with no trailing \0 character
    * Extracting the upper 32 bits of the hash value, and
    * Setting the upper bit of that value to 0.
    * To coerce the compiler into generating a different value, methods can have
      a `Selector` attribute.  The value of the `Selector` attribute will be
      used in the place of the method name above.
*   Each method declaration states its arguments and results.
    *   If no results are declared, then the method is one-way: no response will
        be generated by the server.
    *   If results are declared (even if empty), then the method is two-way:
        each invocation of the method generates a response from the server.
    *   If only results are declared, the method is referred to as an
        *event*. It then defines an unsolicited message from the server.
    *   Two-way methods may declare an error type that a server can send
        instead of the response. This type must be an `int32`, `uint32`, or an
        `enum` thereof.

*   When a server of a protocol is about to close its side of the channel, it
    may elect to send an **epitaph** message to the client to indicate the
    disposition of the connection. The epitaph must be the last message
    delivered through the channel. An epitaph message includes a 32-bit int
    value of type **zx_status_t**.  Negative values are reserved for system
    error codes. The value `ZX_OK` (0) indicates an operation was successful.
    Application-defined error codes (previously defined as all positive
    `zx_status_t` values) are deprecated. For more details about epitaphs, see
    rejection of  [RFC-0031: Typed Epitaphs][rfc-0031]. For more details about
    `zx_status_t` see [RFC-0085: Reducing the zx_status_t space][rfc-0085].

#### Declaration

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="calculator" %}
```

#### Use {#protocols-use}

Protocols are denoted by their name, directionality of the channel, and
optionality:

*   **`Protocol`** : non-nullable FIDL protocol (client endpoint of channel)
*   **`Protocol?`** : nullable FIDL protocol (client endpoint of channel)
*   **`request<Protocol>`** : non-nullable FIDL protocol
    request (server endpoint of channel)
*   **`request<Protocol>?`** : nullable FIDL protocol request
    (server endpoint of channel)

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="endpoints" %}
```

### Protocol Composition {#protocol-composition}

A protocol can include methods from other protocols.
This is called composition: you compose one protocol from other protocols.

Composition is used in the following cases:

1. you have multiple protocols that all share some common behavior(s)
2. you have varying levels of functionality you want to expose to different audiences

#### Common behavior

In the first case, there might be behavior that's shared across multiple protocols.
For example, in a graphics system, several different protocols might all share a
common need to set a background and foreground color.
Rather than have each protocol define their own color setting methods, a common
protocol can be defined:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="composition-base" %}
```

It can then be shared by other protocols:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="composition-inherit" %}
```

In the above, there are three protocols, `SceneryController`, `Drawer`, and `Writer`.
`Drawer` is used to draw graphical objects, like circles and squares at given locations
with given sizes.
It composes the methods **SetBackground()** and **SetForeground()** from
the `SceneryController` protocol because it includes the `SceneryController` protocol
(by way of the `compose` keyword).

The `Writer` protocol, used to write text on the display, includes the `SceneryController`
protocol in the same way.

Now both `Drawer` and `Writer` include **SetBackground()** and **SetForeground()**.

This offers several advantages over having `Drawer` and `Writer` specify their own color
setting methods:

*   the way to set background and foreground colors is the same, whether it's used
    to draw a circle, square, or put text on the display.
*   new methods can be added to `Drawer` and `Writer` without having to change their
    definitions, simply by adding them to the `SceneryController` protocol.

The last point is particularly important, because it allows us to add functionality
to existing protocols.
For example, we might introduce an alpha-blending (or "transparency") feature to
our graphics system.
By extending the `SceneryController` protocol to deal with it, perhaps like so:

```fidl
protocol SceneryController {
    SetBackground(Color color);
    SetForeground(Color color);
    SetAlphaChannel(int a);
};
```

we've now extended both `Drawer` and `Writer` to be able to support alpha blending.

#### Multiple compositions

Composition is not a one-to-one relationship &mdash; we can include multiple compositions
into a given protocol, and not all protocols need be composed of the same mix of
included protocols.

For example, we might have the ability to set font characteristics.
Fonts don't make sense for our `Drawer` protocol, but they do make sense for our `Writer`
protocol, and perhaps other protocols.

So, we define our `FontController` protocol:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="composition-multiple-1" %}
```

and then invite `Writer` to include it, by using the `compose` keyword:

```fidl
protocol Writer {
    compose SceneryController;
    compose FontController;
    Text(int x, int y, string message);
};
```

Here, we've extended the `Writer` protocol with the `FontController` protocol's methods,
without disturbing the `Drawer` protocol (which doesn't need to know anything about fonts).

Protocol composition is similar to [mixin].
More details are discussed in [RFC-0023: Compositional Model][rfc-0023].

#### Layering

At the beginning of this section, we mentioned a second use for composition, namely
exposing various levels of functionality to different audiences.

In this example, we have two protocols that are independently useful, a `Clock` protocol
to get the current time and timezone:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="layering-clock" %}
```

And an `Horologist` protocol that sets the time and timezone:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="layering-horologist" %}
```

We may not necessarily wish to expose the more privileged `Horologist` protocol to just
any client, but we do want to expose it to the system clock component.
So, we create a protocol (`SystemClock`) that composes both:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="layering-systemclock" %}
```

### Aliasing {#aliasing}

Type aliasing is supported.
For example:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="aliasing" %}
```

In the above, the identifier `StoryID` is an alias for the declaration of a
`string` with a maximum size of `MAX_SIZE`. The identifier `Chapters` is an
alias for a vector declaration of five `StoryId` elements.

The identifiers `StoryID` and `Chapters` can be used wherever their aliased
definitions can be used.
Consider:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="aliasing-usage" %}
```

Here, the `Message` struct contains a string of `MAX_SIZE` bytes called `baseline`,
and a vector of up to `5` strings of `MAX_SIZE` called `chapters`.

Note that **`byte`** and **`bytes`** are built in aliases, [see below](#built-in-aliases).

### Built-ins

FIDL provides several built-ins:

* convenience types (**`byte`** and **`bytes`**)
* `zx library` [see below](#zx-library)

#### Built-in aliases {#built-in-aliases}

The types **`byte`** and **`bytes`** are built-in, and are conceptually
equivalent to:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference_builtin.test.fidl" region_tag="builtin" %}
```

When you refer to a name without specific scope, e.g.:

```fidl
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/language_reference.test.fidl" region_tag="builtin-aliases" %}
```

we treat this as `builtin.byte` automatically (so long as there isn't a
more-specific name in scope).

#### ZX Library {#zx-library}

The `fidlc` compiler automatically generates an internal [ZX library](library-zx.md)
for you that contains commonly used Zircon definitions.

<!-- xref -->
[mixin]: https://en.wikipedia.org/wiki/Mixin
[rfc-0023]: /docs/contribute/governance/rfcs/0023_compositional_model_protocols.md
[rfc-0031]: /docs/contribute/governance/rfcs/0031_typed_epitaphs.md
[rfc-0033]: /docs/contribute/governance/rfcs/0033_handling_unknown_fields_strictness.md
[rfc-0057]: /docs/contribute/governance/rfcs/0057_default_no_handles.md
[rfc-0085]: /docs/contribute/governance/rfcs/0085_reducing_zx_status_t_space.md
[fidl-overview]: /docs/concepts/fidl/overview.md
[fidl-grammar]: /docs/reference/fidl/language/grammar.md
[doc-attribute]: /docs/reference/fidl/language/attributes.md#Doc
[naming-style]: /docs/development/languages/fidl/guides/style.md#Names
[union-compat]: /docs/development/languages/fidl/guides/compatibility/README.md#union
[resource-compat]: /docs/development/languages/fidl/guides/compatibility/README.md#modifiers
[bindings-reference]: /docs/reference/fidl/bindings/overview.md
[lexicon-validate]: /docs/reference/fidl/language/lexicon.md#validate
