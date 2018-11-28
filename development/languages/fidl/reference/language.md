# 语言规范

本文档是 Fuchsia 接口定义语言(**FIDL**)的语法规范。

有关 FIDL 的总体目的、目标和要求，以及相关文档的链接的更多信息，请参阅[FIDL概述](index.md)。

<!--
You can find a modified [EBNF description of the FIDL grammar here](grammar.md).
-->
你可以在这里找到修改后的[FIDL语法的EBNF描述](grammar.md)。


[TOC]

## 语法

Fuchsia 接口定义语言提供了声明命名常量、枚举、结构体、联合体和接口的语法。这些声明被集中到库中以便分发。

FIDL 声明存储在 UTF-8 纯文本文件中，每个文件都由一系列以分号分隔的声明组成。 库中的 FIDL 文件顺序或 FIDL 文件中的声明顺序是无关紧要的。 FIDL 不要求（且不支持）任何类型的前向声明。

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

以下是 FIDL 中保留的关键字。

```
array, as, bool, const, enum, float32, float64, handle, int8, int16,
int32, int64, interface, library, request, string, struct, uint8, uint16,
uint32, uint64, union, using, vector
```

#### 标识符

FIDL 标识符必须匹配正则表达式 `[a-zA-Z]([a-zA-Z0-9_]*[a-zA-Z0-9])?`。

简而言之，标识符必须以字母开始，可以包含字母、数字和下划线，但不能以下划线结束。

标识符区分大小写。

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

FIDL 始终在当前库的作用域内查找未限定的符号。要引用其他库中的符号，必须在标识符前添加库名或别名来限定它们。

**objects.fidl:**

<!--
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
-->
```fidl
library objects;
using textures as tex;

interface Frob {
    // "Thing"指的是"objects"库中的"Thing"
    // "tex.Color"指的是"textures"库中的"Color"
    1: Paint(Thing thing, tex.Color color);
};

struct Thing {
    string name;
};


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
FIDL 支持使用类 C 语法的字面量类型:整型，浮点型，布尔型，字符串，枚举类型（请参见以下示例）。

#### 常量

<!--
FIDL supports the following constant types: booleans, signed and unsigned integers,
floating point values, strings, and enumerations.
The syntax is similar to C:
-->
FIDL 支持使用类 C 语法的以下常量类型：布尔型，有符号整型，无符号整型，浮点型，字符串和枚举类型。

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
<!--
These declarations introduce a name within their scope.
The constant's type must be either a primitive or an enum.
-->
这些声明在其范围内引入了一个名称。
常量的类型必须是基本数据类型或枚举。

<!--
Constant expressions are either literals or the names of other
constant expressions.
-->
常量表达式可以是字面量, 也可以是其他常量表达式的名称。


> 为了更加清晰，FIDL 中没有表达式处理；也就是说，例如，你**不能**将常量声明为`6+5`。

#### 声明分隔符
<!--
FIDL uses the semi-colon **';'** to separate adjacent declarations within the
file, much like C.
-->

如同 C 语言一样，FIDL 使用分号**`;`**                                                       来分隔文件中的相邻声明。

<!-- ### Libraries -->
### 库

<!-- 
Libraries are named containers of FIDL declarations. 
-->
库被称为 FIDL 声明的容器。
<!-- 
Each library has a name consisting of a dot-delimited identifier. Library names
appear in [Qualified Identifiers](#qualified-identifiers). 
-->
每个库都有一个由单个标识符(例如："objects")或者多个由点分隔的标识符组成的名称(例如： "mozart.composition").
关于库名称的说明在[限定标识符](#qualified-identifiers)一节中出现。

<!--
```fidl
// library identifier separated by dots
library mozart.composition;

// "using" to import library "mozart.buffers"
using mozart.buffers;

// "using" to import library "mozart.geometry" and create a shortform called "geo"
using mozart.geometry as geo;

```
-->
```fidl
// 使用点标识符分隔库
library mozart.composition;

//使用 "using" 来导入 "mozart.buffers" 库
using mozart.buffers;

//使用 "using" 来导入 "mozart.buffers" 库，并且将其简写为 geo
using mozart.geometry as geo;

```

<!--
Libraries may declare that they use other libraries with a "using" declaration.
This allows the library to refer to symbols defined in other libraries upon which
they depend. Symbols which are imported this way may be accessed by:
-->
库使用 `using` 来声明它们使用的其他库，这允许库引用它们所依赖的其他库中定义的符号。
以这种方式导入的符号可以通过以下方式访问： 

<!--
*   qualifying them with the fully qualified library name (as in _"mozart.geometry.Rect"_),
*   specifying just the library name (as in _"geometry.Rect"_), or,
*   using a library alias (as in _"geo.Rect"_).
-->

* 使用完整的库名称来限定他们（例如：_"mozart.geometry.Rect"_）
* 仅指定库的名称（例如：_"geometry.Rect"_）
* 使用库的别名（例如：_"geo.Rect"_）

<!--
In the source tree, each library consists of a directory with some number of
**.fidl** files. The name of the directory is irrelevant to the FIDL compiler
but by convention it should resemble the library name itself. A directory should
not contain FIDL files for more than one library.
-->
在源码树中，每个库都有一个包含 **.fidl**文件的目录。 
目录的名称与 FIDL 编译器无关，但按照约定俗成，它应该类似于库本身的名字。 
一个目录不应包含多个库的 FIDL 文件。

<!--
The scope of "library" and "using" declarations is limited to a single file.
Each individual file within a FIDL library must restate the "library"
declaration together with any "using" declarations needed by that file.
-->
`library` 和 `using` 声明的作用域仅限于单个文件。
FIDL一个库中的每个单独文件必须重新声明 `library` 及该文件所需的任何 `using` 声明。

<!--
The library's name may be used by certain language bindings to provide scoping
for symbols emitted by the code generator.
-->
某些语言的绑定可以使用库的名称来为代码生成器生成的符号提供相应的作用域范围。

<!--
For example, the C++ bindings generator places declarations for the
FIDL library "fuchsia.ui" within the C++ namespace
"fuchsia::ui". Similarly, for languages such as Dart and Rust which
have their own module system, each FIDL library is compiled as a
module for that language.
-->
例如，C++ 绑定的生成器将 `fuchsia.ui` FIDL库的声明放置于 `fuchsia::ui` 命名空间中。
同样，对于具有自己的模块系统的 Dart 和 Rust 等语言，每个 FIDL 库都被编译为该语言的一个模块。

<!--
### Types and Type Declarations
-->
### 类型和类型声明

<!--
#### Primitives
-->
#### 基本数据类型

<!--
*   Simple value types.
*   Not nullable.
-->
* 简单的值类型
* 不能为 `null`

<!--
The following primitive types are supported:
--> 
以下是 FIDL 支持的基本数据类型：

<!--
*    Boolean                 **`bool`**
*    Signed integer          **`int8 int16 int32 int64`**
*    Unsigned integer        **`uint8 uint16 uint32 uint64`**
*    IEEE 754 Floating-point **`float32 float64`**
-->
* 布尔型 **`bool`**
* 有符号整型 **`uint8 uint16 uint32 uint64`**
* 无符号整型 **`uint8 uint16 uint32 uint64`**
* IEEE 754浮点型 **`float32 float64`**

<!--
Numbers are suffixed with their size in bits, **`bool`** is 1
byte.
-->
数字的大小是以位为单位的，**`bool`** 占1个字节。

<!--
##### Use
-->
##### 使用

<!--
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
-->
```fidl
// 包含一些基本类型字段的结构体
struct Sprite {
    float32 x;
    float32 y;
    uint32 index;
    uint32 color;
    bool visible;
};
```

<!--
#### Enums
-->
#### 枚举（Enum）

<!--
*   Proper enumerated types.
*   Discrete subset of named values chosen from an underlying integer primitive
    type.
*   Not nullable.
*   Enums must have at least one member.
-->
* 适当的可枚举类型
* 从基本整型中，选出命名值的离散子集
* 不能为 `null`
* 枚举必须至少有一个元素

<!-- ##### Declaration -->
##### 声明

<!--
The ordinal index is **required** for each enum element. The underlying type of
an enum must be one of: **int8, uint8, int16, uint16, int32, uint32, int64,
uint64**. If omitted, the underlying type is assumed to be **uint32**.
-->

每个枚举元素的序号索引是**必须的**。枚举的基本类型必须是以下之一：**int8，uint8，int16，uint16，int32，uint32，int64，uint64**。如果省略，则假定基本类型为**uint32**。

<!--
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
-->
```
// 在库作用域中声明枚举。
enum Beverage : uint8 {
    WATER = 0;
    COFFEE = 1;
    TEA = 2;
    WHISKEY = 3;
};

// 在库作用域中声明枚举。
// 默认底层类型为uint32。
enum Vessel {
    CUP = 0;
    BOWL = 1;
    TUREEN = 2;
    JUG = 3;
};
```

<!-- ##### Use -->
##### 使用

<!--
Enum types are denoted by their identifier, which may be qualified if needed.
-->
枚举类型由其标识符表示，如果需要，可以对其进行限定。

<!-- 
```fidl
// A record which contains two enum fields.
struct Order {
    Beverage beverage;
    Vessel vessel;
};
```
-->
```fidl
// 包含两个枚举字段的结构体。
struct Order {
    Beverage beverage;
    Vessel vessel;
};
```

<!-- #### Arrays -->
#### 数组（Array）

<!--
*   Fixed-length sequences of homogeneous elements.
*   Elements can be of any type including: primitives, enums, arrays, strings,
    vectors, handles, structs, unions.
*   Not nullable themselves; may contain nullable types.
-->
*   同一类型元素的定长序列。
*   元素可以是任何类型，包括：基本类型，枚举(enum)，数组(array)，字符串(string)，向量(vector)，句柄(handle)，结构体(struct)，联合体(union)。
*   自己本身不能为 `null`，但可以包含 `null` 类型。

<!-- ##### Use -->
##### 使用

<!--
Arrays are denoted **`array<T>:n`** where _T_ can
be any FIDL type (including an array) and _n_ is a positive
integer constant expression which specifies the number of elements in
the array.
-->
数组表示为 **`array<T>:n`**，其中 _T_ 可以是任何 FIDL 类型（包括数组），_n_ 是正整数常量表达式，它指定了数组中的元素个数。

<!--
```fidl
// A record which contains some arrays.
struct Record {
    // array of exactly 16 floating point numbers
    array<float32>:16 matrix;

    // array of exactly 10 arrays of 4 strings each
    array<array<string>:4>:10 form;
};
```
-->
```fidl
// 包含数组的结构体。
struct Record {
    // 包含16个浮点数的数组
    array<float32>:16 matrix;

    // 包含10个子数组的数组，每个子数组中包含4个字符串
    array<array<string>:4>:10 form;
};
```

<!--
#### Strings
-->
#### 字符串（String）

<!--
*   Variable-length sequence of UTF-8 encoded characters representing text.
*   Nullable; null strings and empty strings are distinct.
*   Can specify a maximum size, eg. **`string:40`** for a
    maximum 40 byte string.
-->

* UTF-8 编码文本的可变序列
* 可以为 `null`，`null`字符串和空字符串是不同的
* 可以指定最大长度。例如：**`string:40`** 表示最多包含40个字节的字符串

<!--
##### Use
-->
##### 使用

<!--
Strings are denoted as follows:
-->
字符串表示如下：

<!--
*   **`string`** : non-nullable string (validation error
    occurs if null is encountered)
*   **`string?`** : nullable string
*   **`string:N, string:N?`** : string, and nullable string, respectively,
    with maximum length of _N_ bytes
-->
* 不为 `null` 的字符串（如果为 `null`，则产生验证错误）
* **`string?`**：可为 `null` 的字符串
* **`string:N, string:N?`**：分别表示最大长度为_N_ 个字节的不为 `null` 字符串和可为 `null` 的字符串

<!--
```fidl
// A record which contains some strings.
struct Record {
    // title string, maximum of 40 bytes long
    string:40 title;

    // description string, may be null, no upper bound on size
    string? description;
};
```
-->
```fidl
// 包含一些字符串的结构体
struct Record {
    // title 字符串，最长40个字节
    string:40 title;

    // description 字符串, 可以为 null，并且没有长度限制
    string? description;
};
```

<!--
#### Vectors
-->
#### 向量（Vector）

<!--
*   Variable-length sequence of homogeneous elements.
*   Nullable; null vectors and empty vectors are distinct.
*   Can specify a maximum size, eg. **`vector<T>:40`** for a
    maximum 40 element vector.
*   There is no special case for vectors of bools. Each bool element takes one
    byte as usual.
-->
* 同一类型元素的可变长度序列。
* 可为`null`，`null`向量和空向量是不同的。
* 可以指定最大容量，例如。 **`vector<T>:40`**表示最多40个元素的向量。
* `bool`类型的向量也不例外，每个`bool`元素像往常一样占用一个字节。

<!--
##### Use
-->
##### 使用

<!--
Vectors are denoted as follows:
-->
向量的表示如下：

<!--
*   **`vector<T>`** : non-nullable vector of element type
    _T_ (validation error occurs if null is encountered)
*   **`vector<T>?`** : nullable vector of element type
    _T_
*   **`vector<T>:N, vector<T>:N?`** : vector, and nullable vector, respectively,
    with maximum length of _N_ elements
-->
* 元素类型 _T_ 的不为的`null`向量（如果为`null`，则产生验证错误）

<!--
_T_ can be any FIDL type.
-->
_T_可以表示任意 FIDL 类型。

<!--
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
-->

```fidl
//包含一些向量的结构体
struct Record {
    // 最多包含10个整型的向量
    vector<int32>:10 params;

    // 包含字节的向量，没有大小限制
    vector<uint8> blob;

    // 一个可为null的向量，最多包含24个字符串
    vector<string>:24? nullable_vector_of_strings;

    // 一个包含可为null字符串的向量，没有大小限制
    vector<string?> vector_of_nullable_strings;

	//一个包含最长16个元素的浮点数数组的向量的向量
    vector<vector<array<float32>:16>> complex;
};
```

<!--
#### Handles
-->
#### 句柄（Handle）

<!--
*   Transfers a Zircon capability by handle value.
*   Stored as a 32-bit unsigned integer.
*   Nullable by encoding as a zero-valued handle.
-->
* 通过句柄（handle）值传递 Zircon 的功能
* 作为32位无符号整型存储
* 通过编码为零值的句柄（handle）表示为 null

<!--
##### Use
-->
##### 使用

<!--
Handles are denoted:
-->
句柄（handle）可表示为如下：

<!--
*   **`handle`** : non-nullable Zircon handle of
    unspecified type
*   **`handle?`** : nullable Zircon handle of
    unspecified type
*   **`handle<H>`** : non-nullable Zircon handle
    of type _H_
*   **`handle<H>?`** : nullable Zircon handle of
    type _H_
-->
* **`handle`**：未指定类型的不可为 `null` 的 Zircon句柄（handle）
* **`handle?`**：未指定类型的可为 `null` 的 Zircon句柄（handle）
* **`handle<H>`**：_H_ 类型不可为 `null` 的 Zircon句柄（handle）
* **`handle<H>?`**：_H_ 类型可为 `null` 的 Zircon 句柄（handle）

<!--
_H_ can be one of: `channel, event, eventpair, fifo, job,
process, port, resource, socket, thread, vmo`. New types will
be added to the FIDL language as they are added to Zircon.
-->
_H_ 可以是以下类型之一：`channel, event, eventpair, fifo, job, process, port, resource, socket, thread, vmo`。 
如果有新句柄类型被添加到 Zircon 中，那么它们也会被添加到 FIDL 语言中。

<!--
```fidl
// A record which contains some handles.
struct Record {
    // a handle of unspecified type
    handle h;

    // an optional channel
    handle<channel>? c;
};
```
-->

```fidl
// 包含句柄（handle）的结构体
struct Record {
    // 未指定类型的句柄（handle）
    handle h;

    // 可为 null 的句柄
    handle<channel>? c;
};
```
<!--
#### Structs
-->
#### 结构体（Struct）

<!--
*   Record type consisting of a sequence of typed fields.
*   Declaration is not intended to be modified once deployed; use interface
    extension instead.
*   Reference may be nullable.
*   Structs contain one or more members. A struct with no members is
    difficult to represent in C and C++ as a zero-sized type. Fidl
    therefore chooses to require all structs to have nonzero size.
-->
*   结构体类型由一系列类型字段组成。
*   结构体一旦被部署到系统中，声明应不再被修改；请使用接口进行扩展。
*   引用可以为 `null`。
*   结构体包含一个或多个成员。 
    没有元素的结构体很难在 C 和 C++ 中表示为长度为零的类型。 
    因此，FIDL 要求所有结构体都具有非零长度。

<!--
##### Declaration
-->
##### 声明

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

<!--
#### Use
-->
#### 使用

<!--
Structs are denoted by their declared name (eg. **Circle**) and nullability:
-->
结构体由其声明的名称（例如**Circle**）和是否为 `null` 表示：

<!--
*   **`Circle`** : non-nullable Circle
*   **`Circle?`** : nullable Circle
-->
* **`Circle`**：不可为`null`的Circle类型结构体
* **`Circle?`**：可为`null`的Circle类型结构体

<!--
```fidl
struct Circle {
    bool filled;
    Point center;    // Point will be stored in-line
    float32 radius;
    Color? color;    // Color will be stored out-of-line
    bool dashed;
};
```
-->
```
struct Circle {
    bool filled;
    Point center;    // Point将存储在结构体之内
    float32 radius;
    Color? color;    // Color存储在结构体之外
    bool dashed;
};
```

<!--
#### Unions
-->
#### 联合体（Union）

<!--
*   Tagged option type consisting of tag field and variadic contents.
*   Declaration is not intended to be modified once deployed; use interface
    extension instead.
*   Reference may be nullable.
*   Unions contain one or more members. A union with no members would have
    no inhabitants and thus would make little sense in a wire format.
-->
*   联合体是标签化的可选项类型，包含标签字段和可变参数内容。
*   联合体一旦被部署，声明不应再被修改；如果需要修改，请使用接口进行扩展。
*   引用可以为 `null`。
*   联合体包含一个或多个成员。没有成员的联合体没有任何内容，因此在有线格式中没有任何意义。

<!--
##### Declaration
-->
##### 声明

<!--
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
-->
```fidl
union Pattern {
    Color color;        // Pattern要么包含color
    Texture texture;    // 要么包含texture，但是两者不会同时包含
};
struct Color {
    float32 r;
    float32 g;
    float32 b;
};
struct Texture { string name; };
```

<!--
##### Use
-->
##### 使用

<!--
Union are denoted by their declared name (eg. **Pattern**) and nullability:
-->
联合体由其声明的名称（例如**Pattern**）和是否为`null`表示：

<!--
*   **`Pattern`** : non-nullable Shape
*   **`Pattern?`** : nullable Shape
-->
* **`Pattern`**：不可为 `null` 的Pattern
* **`Pattern?`**：可为 `null` 的Pattern

```fidl
struct Paint {
    Pattern fg;
    Pattern? bg;
};
```

<!--
#### Interfaces
-->
#### 接口（Interface）

<!--
*   Describe methods which can be invoked by sending messages over a channel.
-->
*   描述可以通过向通道发送消息来调用的方法。
<!--
*   Methods are identified by their ordinal index. Ordinals must be stated
    explicitly to reduce the chance that developers might break interfaces by
    reordering methods and to help with interface extension and derivation.
    *   Method ordinals are unsigned values in the range **0x00000001** to
        **0x7fffffff**.
 	*   The FIDL wire format internally represents ordinals as 32-bit values but
        reserves the range **0x80000000** to **0xffffffff** for protocol control
        messages, so these values cannot be associated with methods.
-->
* 方法由它们的序号索引确定。必须明确说明序号，以减少开发人员通过重新排序方法破坏接口的机会，有助于进行接口扩展和派生。
	*   方法序号是 **0x00000001** 到 **0x7fffffff** 区间内的无符号数值。
	*   FIDL有线格式在内部将序号表示为32位值，但为协议控制消息保留了从 **0x80000000** 到  **0xffffffff** 区间内的数值，因此这些值不能与方法相关联。
<!--
*   Each method declaration states its arguments and results.
    *   If no results are declared, then the method is one-way: no response will
        be generated by the server.
    *   If results are declared (even if empty), then the method is two-way:
        each invocation of the method generates a response from the server.
    *   If only results are declared, the method is referred to as an
        *event*. It then defines an unsolicited message from the server.
-->
*   每个方法的声明都表示了其参数和返回值。
    *   如果未声明返回值，则该方法是单向的：服务器不会生成任何响应。
    *   如果声明了返回值（即使为空），则该方法是双向的：每次调用该方法都会从服务端生成响应。
    *   如果仅声明结果，则该方法称为 *event*。 然后，它定义了来自服务器的未请求消息。
<!--
*   When a server of an interface is about to close its side of the channel, it
    may elect to send an **epitaph** message to the client to indicate the
    disposition of the connection. The epitaph must be the last message
    delivered through the channel. An epitaph message includes a 32-bit int
    value of type **zx_status_t**.  Negative values are reserved for system
    error codes.  Positive values are reserved for application errors.  A status
    of ZX_OK indicates successful operation.
-->
*	当接口的客户端或服务端即将关闭其通道的一侧时，它可以选择向其对等端发送**墓志铭（epitaph）**消息来配置连接。墓志铭（epitaph）消息必须是通过该通道传递的最后一条信息。一条墓志消息包含 **zx_status_t** 类型的32位 int 值。保留负值用于系统错误代码。保留正值用于应用程序错误。状态为 ZX_OK 表示操作成功。
<!--
*   **Interface extension:** New methods can be added to existing interfaces as
    long as they do not collide with existing methods.
-->
*   **接口扩展**：只要新方法不与现有方法冲突，就可以将新方法添加到现有接口中。
<!--
*   **Interface derivation:** New interfaces can be derived from any number of
    existing interfaces as long as none of their methods use the same ordinals.
    (This is purely a FIDL language feature, does not affect the wire format.)
-->
*   **接口派生**：新接口可以从任意数量的现有接口的基础上派生，只要它们的方法都不使用相同的序数。（这纯粹是 FIDL 语言的特性，不影响其有线格式。）

<!--
##### Declaration
-->
##### 声明

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

<!--
##### Use
-->
##### 使用

<!--
Interfaces are denoted by their name, directionality of the channel, and
optionality:
-->
接口由它们的名称，通道的方向性和可选性（是否为 null）表示：

<!--
*   **`Interface`** : non-nullable FIDL interface (client
    endpoint of channel)
*   **`Interface?`** : nullable FIDL interface (client
    endpoint of channel)
*   **`request<Interface>`** : non-nullable FIDL interface
    request (server endpoint of channel)
*   **`request<Interface>?`** : nullable FIDL interface request
    (server endpoint of channel)
-->
* **`Interface`**：不可为 `null` 的FIDL接口（通道的客户端点）
* **`Interface?`**：可为 `null` 的FIDL接口（通道的客户端点）
* **`request<Interface>`**：可为 `null` 的FIDL接口请求（通道的服务端点）
* **`request<Interface>s?`**：可为 `null` 的FIDL接口请求（通道的服务端点


<!--
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
-->
```fidl
// 包含接口绑定通道的结构体。
struct Record {
    // 绑定到 Calculator 接口通道的客户端点
    Calculator c;

    // 绑定到Science接口通道的服务端点
    request<Science> s;

    // 绑定到RealCalculator接口通道的可为 null 的客户端点
    // RealCalculator interface
    RealCalculator? r;
};
```