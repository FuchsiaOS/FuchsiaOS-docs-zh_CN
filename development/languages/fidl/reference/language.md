# 语言规范

本文档是Fuchsia接口定义语言(**FIDL**)的语法规范。

有关FIDL的总体目的、目标和要求，以及相关文档的链接的更多信息，请参阅[FIDL概述](index.md)。

<!--
You can find a modified [EBNF description of the FIDL grammar here](grammar.md).
-->
你可以在这里找到修改后的[FIDL语法的EBNF描述](grammar.md)。
[TOC]

## 语法

Fuchsia接口定义语言提供了声明命名常量、枚举、结构体、联合体和接口的语法。这些声明被集中到库中以便分发。

FIDL声明存储在UTF-8纯文本文件中，每个文件都由一系列以分号分隔的声明组成。 库中的FIDL文件顺序或FIDL文件中的声明顺序是无关紧要的。 FIDL不要求（且不支持）任何类型的前向声明。

### 令牌(Tokens)

#### 注释

<!--
FIDL comments start with two (`//`) or three (`///`) forward slashes, continue
to the end of the line, and can contain UTF-8 content (which is, of course, ignored).
The three-forward-slash variant is a "documentation comment", and causes the comment
text to be emitted into the generated code (as a comment, escaped correctly
for the target language).
-->

FIDL注释是从`//`或者`///`开始到行尾结束，并且可以包含UTF-8内容（它们当然会被FIDL编译器作为注释忽略）。`///`是“文档注释”，并且通过其注释的内容将会发送到生成的代码中（作为注释，正确地为目标语言进行了转义）。

```fidl
// 这里是注释
/// 这里也是注释，但是他会在最终生成的代码中
struct Foo { // 这里也是
    int32 f; // 以及这里
}; // 最后一行注释!
```

<!--
Note that documentation comments can also be provided via the
[`[Doc]` attribute](attributes.md#Doc).
-->

备注：文档注释也可参阅[`[Doc]` attribute](attributes.md#Doc)。

#### 关键字

以下FIDL中保留的关键字.

```
array, as, bool, const, enum, float32, float64, handle, int8, int16,
int32, int64, interface, library, request, string, struct, uint8, uint16,
uint32, uint64, union, using, vector
```

#### 标识符

FIDL标识符必须匹配正则表达式 `[a-zA-Z]([a-zA-Z0-9_]*[a-zA-Z0-9])?`.

简而言之，标识符必须以字母开始，可以包含字母、数字和下划线，但不能以下划线结束。

标识符区分大小写.

```fidl
// 名为"foo"的库
library foo;

// 名为"Foo"的结构体
struct Foo { };

// 名为"struct"的结构体
struct struct { };
```

<!--
> Note that while using keywords as identifiers is supported, it can lead to confusion,
> and should the be considered on a case-by-case basis. See the `Names` section of the
> [Readability Rubric](https://fuchsia.googlesource.com/docs/+/master/development/api/fidl.md#Names)
-->

> 请注意，虽然可以使用关键字作为标识符，但可能会导致混淆，
> 应该根据具体情况加以考虑，请参阅[可读性规范(Readability Rubric)](https://fuchsia.googlesource.com/docs/+/master/development/api/fidl.md#Names)的`名称(Names)`部分。

#### 限定标识符

FIDL始终在当前库的作用域内查找未限定的符号。 要引用其他库中的符号，必须在标识符前添加库名或别名来限定它们。

**objects.fidl:**

```fidl
library objects;
using textures as tex;

interface Frob {
    // "Thing" refers to "Thing" in the "objects" library
    // "tex.Color" refers to "Color" in the "textures" library
    1: Paint(Thing thing, tex.Color color);
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

#### 字面量

<!--
FIDL supports integer, floating point, boolean, string, and enumeration literals, using
a simplified syntax familiar to C programmers (see below for examples).
-->
FIDL支持使用类C语法的字面量类型:整型，浮点型，布尔型，字符串，枚举类型（请参见以下示例）。

#### Constants

FIDL supports the following constant types: booleans, signed and unsigned integers,
floating point values, strings, and enumerations.
The syntax is similar to C:

```fidl
const bool enabled_flag = true;
const int8 offset = -33;
const uint16 answer = 42;
const uint32 population_2018 = 7700000000;
const uint64 diamond = 0x183c7effff7e3c18;
const uint64 fuchsia = 4054509061583223046;
const string username = "squeenze";
const float32 min_temp = -273.15;
const float64 conversion_factor = 1.41421358;
const Beverage my_drink = WATER;
```

These declarations introduce a name within their scope.
The constant's type must be either a primitive or an enum.

Constant expressions are either literals or the names of other
constant expressions.

> For greater clarity, there is no expression processing in FIDL; that is,
> you *cannot* declare a constant as having the value `6 + 5`, for
> example.

#### Declaration Separator

FIDL uses the semi-colon **';'** to separate adjacent declarations within the
file, much like C.

### Libraries

Libraries are named containers of FIDL declarations.

Each library has a name consisting of a single identifier (e.g., "objects"),
or multiple identifiers separated by dots (e.g., "mozart.composition").
Library names are used in [Qualified Identifiers](#qualified-identifiers).

```fidl
// library identifier separated by dots
library mozart.composition;

// "using" to import library "mozart.buffers"
using mozart.buffers;

// "using" to import library "mozart.geometry" and create a shortform called "geo"
using mozart.geometry as geo;

```

Libraries may declare that they use other libraries with a "using" declaration.
This allows the library to refer to symbols defined in other libraries upon which
they depend. Symbols which are imported this way may be accessed by:

*   qualifying them with the fully qualified library name (as in _"mozart.geometry.Rect"_),
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
"fuchsia::ui". Similarly, for languages such as Dart and Rust which
have their own module system, each FIDL library is compiled as a
module for that language.

### Types and Type Declarations

#### Primitives

*   Simple value types.
*   Not nullable.

The following primitive types are supported:

*    Boolean                 **`bool`**
*    Signed integer          **`int8 int16 int32 int64`**
*    Unsigned integer        **`uint8 uint16 uint32 uint64`**
*    IEEE 754 Floating-point **`float32 float64`**

Numbers are suffixed with their size in bits, **`bool`** is 1
byte.

##### Use

```fidl
// A record which contains fields of a few primitive types.
struct Sprite {
    float32 x;
    float32 y;
    uint32 index;
    uint32 color;
    bool visible;
};
```

#### Enums

*   Proper enumerated types.
*   Discrete subset of named values chosen from an underlying integer primitive
    type.
*   Not nullable.
*   Enums must have at least one member.

##### Declaration

The ordinal index is **required** for each enum element. The underlying type of
an enum must be one of: **int8, uint8, int16, uint16, int32, uint32, int64,
uint64**. If omitted, the underlying type is assumed to be **uint32**.

```fidl
// An enum declared at library scope.
enum Beverage : uint8 {
    WATER = 0;
    COFFEE = 1;
    TEA = 2;
    WHISKEY = 3;
};

// An enum declared at library scope.
// Underlying type is assumed to be uint32.
enum Vessel {
    CUP = 0;
    BOWL = 1;
    TUREEN = 2;
    JUG = 3;
};
```

##### Use

Enum types are denoted by their identifier, which may be qualified if needed.

```fidl
// A record which contains two enum fields.
struct Order {
    Beverage beverage;
    Vessel vessel;
};
```

#### Arrays

*   Fixed-length sequences of homogeneous elements.
*   Elements can be of any type including: primitives, enums, arrays, strings,
    vectors, handles, structs, unions.
*   Not nullable themselves; may contain nullable types.

##### Use

Arrays are denoted **`array<T>:n`** where _T_ can
be any FIDL type (including an array) and _n_ is a positive
integer constant expression which specifies the number of elements in
the array.

```fidl
// A record which contains some arrays.
struct Record {
    // array of exactly 16 floating point numbers
    array<float32>:16 matrix;

    // array of exactly 10 arrays of 4 strings each
    array<array<string>:4>:10 form;
};
```

#### Strings

*   Variable-length sequence of UTF-8 encoded characters representing text.
*   Nullable; null strings and empty strings are distinct.
*   Can specify a maximum size, eg. **`string:40`** for a
    maximum 40 byte string.

##### Use

Strings are denoted as follows:

*   **`string`** : non-nullable string (validation error
    occurs if null is encountered)
*   **`string?`** : nullable string
*   **`string:N, string:N?`** : string, and nullable string, respectively,
    with maximum length of _N_ bytes

```fidl
// A record which contains some strings.
struct Record {
    // title string, maximum of 40 bytes long
    string:40 title;

    // description string, may be null, no upper bound on size
    string? description;
};
```

#### Vectors

*   Variable-length sequence of homogeneous elements.
*   Nullable; null vectors and empty vectors are distinct.
*   Can specify a maximum size, eg. **`vector<T>:40`** for a
    maximum 40 element vector.
*   There is no special case for vectors of bools. Each bool element takes one
    byte as usual.

##### Use

Vectors are denoted as follows:

*   **`vector<T>`** : non-nullable vector of element type
    _T_ (validation error occurs if null is encountered)
*   **`vector<T>?`** : nullable vector of element type
    _T_
*   **`vector<T>:N, vector<T>:N?`** : vector, and nullable vector, respectively,
    with maximum length of _N_ elements

_T_ can be any FIDL type.

```fidl
// A record which contains some vectors.
struct Record {
    // a vector of up to 10 integers
    vector<int32>:10 params;

    // a vector of bytes, no upper bound on size
    vector<uint8> blob;

    // a nullable vector of up to 24 strings
    vector<string>:24? nullable_vector_of_strings;

    // a vector of nullable strings, no upper bound on size
    vector<string?> vector_of_nullable_strings;

    // a vector of vectors of 16-element arrays of floating point numbers
    vector<vector<array<float32>:16>> complex;
};
```

#### Handles

*   Transfers a Zircon capability by handle value.
*   Stored as a 32-bit unsigned integer.
*   Nullable by encoding as a zero-valued handle.

##### Use

Handles are denoted:

*   **`handle`** : non-nullable Zircon handle of
    unspecified type
*   **`handle?`** : nullable Zircon handle of
    unspecified type
*   **`handle<H>`** : non-nullable Zircon handle
    of type _H_
*   **`handle<H>?`** : nullable Zircon handle of
    type _H_

_H_ can be one of: `channel, event, eventpair, fifo, job,
process, port, resource, socket, thread, vmo`. New types will
be added to the FIDL language as they are added to Zircon.

```fidl
// A record which contains some handles.
struct Record {
    // a handle of unspecified type
    handle h;

    // an optional channel
    handle<channel>? c;
};
```

#### Structs

*   Record type consisting of a sequence of typed fields.
*   Declaration is not intended to be modified once deployed; use interface
    extension instead.
*   Reference may be nullable.
*   Structs contain one or more members. A struct with no members is
    difficult to represent in C and C++ as a zero-sized type. Fidl
    therefore chooses to require all structs to have nonzero size.

##### Declaration

```fidl
struct Point {
    float32 x;
    float32 y;
};
struct Color {
    float32 r;
    float32 g;
    float32 b;
};
```

#### Use

Structs are denoted by their declared name (eg. **Circle**) and nullability:

*   **`Circle`** : non-nullable Circle
*   **`Circle?`** : nullable Circle

```fidl
struct Circle {
    bool filled;
    Point center;    // Point will be stored in-line
    float32 radius;
    Color? color;    // Color will be stored out-of-line
    bool dashed;
};
```

#### Unions

*   Tagged option type consisting of tag field and variadic contents.
*   Declaration is not intended to be modified once deployed; use interface
    extension instead.
*   Reference may be nullable.
*   Unions contain one or more members. A union with no members would have
    no inhabitants and thus would make little sense in a wire format.

##### Declaration

```fidl
union Pattern {
    Color color;        // the Pattern contains either a Color
    Texture texture;    // or a Texture, but not both at the same time
};
struct Color {
    float32 r;
    float32 g;
    float32 b;
};
struct Texture { string name; };
```

##### Use

Union are denoted by their declared name (eg. **Pattern**) and nullability:

*   **`Pattern`** : non-nullable Shape
*   **`Pattern?`** : nullable Shape

```fidl
struct Paint {
    Pattern fg;
    Pattern? bg;
};
```

#### Interfaces

*   Describe methods which can be invoked by sending messages over a channel.
*   Methods are identified by their ordinal index. Ordinals must be stated
    explicitly to reduce the chance that developers might break interfaces by
    reordering methods and to help with interface extension and derivation.
    *   Method ordinals are unsigned values in the range **0x00000001** to
        **0x7fffffff**.
    *   The FIDL wire format internally represents ordinals as 32-bit values but
        reserves the range **0x80000000** to **0xffffffff** for protocol control
        messages, so these values cannot be associated with methods.
*   Each method declaration states its arguments and results.
    *   If no results are declared, then the method is one-way: no response will
        be generated by the server.
    *   If results are declared (even if empty), then the method is two-way:
        each invocation of the method generates a response from the server.
    *   If only results are declared, the method is referred to as an
        *event*. It then defines an unsolicited message from the server.

*   When a server of an interface is about to close its side of the channel, it
    may elect to send an **epitaph** message to the client to indicate the
    disposition of the connection. The epitaph must be the last message
    delivered through the channel. An epitaph message includes a 32-bit int
    value of type **zx_status_t**.  Negative values are reserved for system
    error codes.  Positive values are reserved for application errors.  A status
    of ZX_OK indicates successful operation.

*   **Interface extension:** New methods can be added to existing interfaces as
    long as they do not collide with existing methods.
*   **Interface derivation:** New interfaces can be derived from any number of
    existing interfaces as long as none of their methods use the same ordinals.
    (This is purely a FIDL language feature, does not affect the wire format.)

##### Declaration

```fidl
interface Calculator {
    1: Add(int32 a, int32 b) -> (int32 sum);
    2: Divide(int32 dividend, int32 divisor)
    -> (int32 quotient, int32 remainder);
    3: Clear();
    4: -> OnClear();
};

interface RealCalculator : Calculator {
    1001: AddFloats(float32 a, float32 b) -> (float32 sum);
};

interface Science {
    2001: Hypothesize();
    2002: Investigate();
    2003: Explode();
    2004: Reproduce();
};

interface ScientificCalculator : RealCalculator, Science {
    3001: Sin(float32 x) -> (float32 result);
};
```

##### Use

Interfaces are denoted by their name, directionality of the channel, and
optionality:

*   **`Interface`** : non-nullable FIDL interface (client
    endpoint of channel)
*   **`Interface?`** : nullable FIDL interface (client
    endpoint of channel)
*   **`request<Interface>`** : non-nullable FIDL interface
    request (server endpoint of channel)
*   **`request<Interface>?`** : nullable FIDL interface request
    (server endpoint of channel)

```fidl
// A record which contains interface-bound channels.
struct Record {
    // client endpoint of a channel bound to the Calculator interface
    Calculator c;

    // server endpoint of a channel bound to the Science interface
    request<Science> s;

    // optional client endpoint of a channel bound to the
    // RealCalculator interface
    RealCalculator? r;
};
```
