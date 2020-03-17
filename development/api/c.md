<!--# C Library Readability Rubric

This document describes heuristics and rules for writing C libraries
that are published in the Fuchsia SDK.

A different document will be written for C++ libraries. While C++ is
almost an extension of C, and has some influence in this document, the
patterns for writing C++ libraries will be quite different than for C.-->

# C 库可读性评估准则

本文档描述了要发布在 Fuchsia SDK 中 C 库的编写方法和规则。

将为 C++ 库单独编写文档。尽管 C++ 几乎是 C 的扩展，但在本文中也有一定的影响，编写 C++ 库的模式将与 C 完全不同。

<!--Most of this document is concerned with the description of an
interface in a C header. This is not a full C style guide, and has
little to say about the contents of C source files. Nor is this a
documentation rubric (though public interfaces should be well
documented).

Some C libraries have external constraints that contradict these
rules. For instance, the C standard library itself does not follow
these rules. This document should still be followed where applicable.

[TOC]-->

本文档的大部分内容与 C 头中接口的描述有关。这不是一个完整的 C 风格指南，对于 C 源文件的内容几乎没什么可说的。这也不是文档准则（尽管公开的接口应该有很好的文档记录）。

一些 C 库具有与这些规则相矛盾的外部约束。例如，C 标准库本身不遵循这些规则。在适用的情况下，仍应遵循本文档。

[TOC]

<!--## Goals

### ABI Stability

Some Fuchsia interfaces with a stable ABI will be published as C
libraries. One goal of this document is to make it easy for Fuchsia
developers to write and to maintain a stable ABI. Accordingly, we
suggest not using certain features of the C language that have
potentially surprising or complicated implications on the ABI of an
interface. We also disallow nonstandard compiler extensions, since we
cannot assume that third parties are using any particular compiler,
with a handful of exceptions for the DDK described below.-->

## 目标

### ABI 稳定

一些带有稳定 ABI 的 Fuchsia 接口将作为 C 库发布。本文档的一个目标是使 Fuchsia 开发人员易于编写和维护一个稳定的 ABI。因此，我们建议不要使用 C 语言的某些特性，这些特性可能会使接口的 ABI 产生潜在问题或复杂的影响。我们也不允许非标准编译器扩展，因为我们不能假定第三方正在使用任何特定的编译器，下面描述的 DDK 有几个例外。

<!--### Resource Management

Parts of this document describe best practices for resource management
in C. This includes resources, Zircon handles, and any other type of
resource.-->

### 资源管理

本文档的部分内容描述了 C 语言中资源管理的最佳实践。这包括 resources、Zircon handles 和任何其他类型的资源。

<!--### Standardization

We would also like to adopt reasonably uniform standards for Fuchsia C
libraries. This is especially true of naming schemes. Out parameter
ordering is another example of standardization.-->

### 标准化

我们也希望对 Fuchsia 的 C 库采用合理统一的标准。这尤其适用于命名方案。另一个标准化的例子是对输出参数排序。

<!--### FFI Friendliness

Some amount of attention is paid to Foreign Function Interface (FFI)
friendliness. Many non-C languages support C interfaces. The
sophistication of these FFI systems varies wildly, from essentially
sed to sophisticated libclang-based tooling. Some amount of
consideration of FFI friendliness went into these decisions.-->

### FFI 友好性

对外部函数接口 `Foreign Function Interface`（FFI）友好性给予了一定的关注。许多非 C 语言支持 C 接口。从基本的 sed 到复杂的基于 libclang 的工具，这些 FFI 系统的复杂程度差别很大。在做出这些决定时，一定程度上考虑了 FFI 友好性。

<!--## Language versions

### C

Fuchsia C libraries are written against the C11 standard (with a small
set of exceptions, such as unix signal support, that are not
particularly relevant to our C library ABI). C99 compliance is not a
goal.

In particular, Fuchsia C code can use the `<threads.h>` and
`<stdatomic.h>` headers from the C11 standard library, as well as the
`_Thread_local` and alignment language features.-->

## 语言版本

### C

Fuchsia C 库是根据 C11 标准编写的（除了一小部分异常，比如 unix 信号支持，它们与我们的 C 库 ABI 没有特别的关系）。不需要符合 C99 标准。

特别是，Fuchsia C 代码可以使用 C11 标准库中的 `<threads.h>` 和 `<stdatomic.h>` 头，以及 `_Thread_local` 和语言对齐特性。

<!--The thread locals should use the `thread_local` spelling from
`<threads.h>`, rather than the built in `_Thread_local`. Similarly,
prefer `alignas` and `alignof` from `<stdalign.h>`, rather than
`_Alignas` and `_Alignof`.

Note that compilers support flags which may alter the ABI of the
code. For instance, GCC has a `-m96bit-long-double` flag which alters
the size of a long double. We assume that such flags are not used.-->

线程局部变量应使用 `<threads.h>` 中的 `thread_local`，而不是内置的 `_Thread_local`。同样的，使用 `<stdalign.h>` 中的 `alignas` 和 `alignof`，而不是 `_Alignas` 和 `_Alignof`。

注意编译器支持可能更改代码 ABI 的标志。例如，GCC 有一个 `-m96bit-long-double` 标志，它可以改变 long double 的大小。我们假定不使用这种标志。

<!--Finally, some libraries (such as Fuchsia's C standard library) in our
SDK are a mix of externally defined interfaces and Fuchsia specific
extensions. In these cases, we allow some pragmatism. For instance,
libc defines functions like `thrd_get_zx_handle` and
`dlopen_vmo`. These names are not strictly in accordance with the
rules below: the name of the library is not a prefix. Doing so would
make the names fit less well next to other functions like
`thrd_current` and `dlopen`, and so we allow the exceptions.-->

最后，我们 SDK 中的一些库（比如 Fuchsia 的 C 标准库）混合了外部定义接口和 Fuchsia 的特定扩展。在这些情况下，我们允许一些例外发生。例如，libc 定义了诸如 `thrd_get_zx_handle` 和 `dlopen_vmo` 之类的函数。这些名字不是严格按照下面的规则命名的：没有使用库的名称做前缀。这样做会使名称与其他诸如 `thrd_current` 和 `dlopen` 之类的函数不太匹配，但我们允许这种例外发生。

<!--### C++

While C++ is not an exact superset of C, we still design C libraries
to be usable from C++. Fuchsia C headers should be compatible with the
C++11, C++14, and C++17 standards. In particular, function
declarations must be `extern "C"`, as described below.

C and C++ interfaces should not be mixed in one header. Instead,
create a separate `cpp` subdirectory and place C++ interfaces in their
own headers there.-->

### C++

虽然 C++ 不是 C 的精确超集，但我们仍然设计了可用于 C++ 的 C 库。Fuchsia C 头应该兼容 C++11、C++14 和 C++17 标准。特别是，函数声明必须是 `extern "C"`，如下所述。

C 和 C++ 接口不应该混写在一个头中。相反，创建一个单独的 `CPP` 子目录，并在它们自己的头中写 C++ 接口。

<!--## Library Layout and Naming

A Fuchsia C library has a name. This name determines its include path
(as described in the [library naming document]) as well as identifiers
within the library.

In this document, the library is always named `tag`, and is variously
referred to as `tag` or `TAG` or `Tag` or `kTag` to reflect a
particular lexical convention. The `tag` should be a single identifier
without underscores. The all-lowercase form of a tag is given by the
regular expression `[a-z][a-z0-9]*`.  A tag can be replaced by a shorter
version of the library name, for example `zx` instead of `zircon`.

The include path for a header `foo.h`, as described by the [library
naming document], should be `lib/tag/foo.h`.-->

## 库的设计和命名

一个 Fuchsia C 库有一个名字。此名称决定了其引入路径（如[库命名文档]中所述）以及库中的标识符。

在本文档中，库始终用 `tag` 命名，并以各种方式被称为 `tag` 或 `TAG` 或 `Tag` 或 `kTag` 以反映特定的词汇习惯。`tag` 需要是一个没有下划线的标识符。标签的全小写形式需满足正则表达式 `[a-z][a-z0-9]*`。标签可以被库名称的较短版本替换，例如用 `zx` 替换 `zircon`。

对于头文件 `foo.h` 的引入路径，如[库命名文档]中所述，应该为 `lib/tag/foo.h`。

<!--## Header Layout

A single header in a C library contains a few kinds of things.

- A copyright banner
- A header guard
- A list of file inclusions
- Extern C guards
- Constant declarations
- Extern symbol declarations
- Including extern function declarations
- Static inline functions
- Macro definitions-->

## 头的设计

C 库中的单个头包含几种类型的内容。

- 版权说明
- 头部防护
- 文件包含列表
- 外部 C 防护
- 常量声明
- 外部符号声明
- 包括外部函数声明
- 静态内联函数
- 宏定义

<!--### Header Guards

Use #ifndef guards in headers. These look like:

```C
#ifndef SOMETHING_MUMBLE_H_
#define SOMETHING_MUMBLE_H_

// code
// code
// code

#endif // SOMETHING_MUMBLE_H_
```

The exact form of the define is as follows:
- Take the canonical include path to the header
- Replace all ., /, and - with _
- Convert all letters to UPPERCASE
- Add a trailing _

For example, the header located in the SDK at `lib/tag/object_bits.h`
should have a header guard `LIB_TAG_OBJECT_BITS_H_`.-->

### 头部防护

在头部使用 #ifndef 防护。这些看起来像：

```C
#ifndef SOMETHING_MUMBLE_H_
#define SOMETHING_MUMBLE_H_

// code
// code
// code

#endif // SOMETHING_MUMBLE_H_
```

定义的确切形式如下：
- 在头中使用规范的引入路径
- 用 `_` 替换所有的 `.` `/` 和 `-`
- 将所有字母转换为大写
- 末尾添加 `_`

例如，SDK 中位于 `lib/tag/object_bits.h` 的头应该有一个头部防护 `LIB_TAG_OBJECT_BITS_H_`

<!--### Inclusions

Headers should include what they use. In particular, any public header
in a library should be safe to include first in a source file.

Libraries can depend on the C standard library headers.

Some libraries may also depend on a subset of POSIX headers. Exactly
which are appropriate is pending a forthcoming libc API review.-->

### 包含物

头应该包括它们使用的内容。特别是，库中的任何公共头都应该安全地优先包含在源文件中。

库可以依赖于 C 标准库的头。

有些库还可能依赖于 POSIX 头的子集。究竟哪一个是合适的取决于未来 libc API 审查。

<!--### Constant Definitions

Most constants in a library will be compile-time constants, created
via a `#define`. There are also read-only variables, declared via
`extern const TYPE NAME;`, as it sometimes is useful to have storage
for a constant (particularly for some forms of FFI). This section
describes how to provide compile time constants in a header.

There are several types of compile time constants.

- Single integer constants
- Enumerated integer constants
- Floating point constants-->

### 常量声明

库中的大多数常量都是编译时常量，通过 `#define` 创建。也有只读变量，通过 `extern const TYPE NAME;` 来声明，因为有时需要存储一个常数（特别是对一些类型的 FFI）。本节描述了如何在头中提供编译时常量。

编译时常量有几种类型。

- 单整数常量
- 枚举整数常量
- 浮点常量

<!--#### Single integer constants

A single integer constants has some `NAME` in a library `TAG`, and its
definition looks like the following.

```C
#define TAG_NAME EXPR
```

where `EXPR` has one of the following forms (for a `uint32_t`)

- `((uint32_t)23)`
- `((uint32_t)0x23)`
- `((uint32_t)(EXPR | EXPR | ...))`-->

#### 单整数常量

单个整数常量在库 `TAG` 中有一些 `NAME`，其定义如下。

```C
#define TAG_NAME EXPR
```

其中 `EXPR` 具有以下形式之一（对于 `uint32_t`）

- `((uint32_t)23)`
- `((uint32_t)0x23)`
- `((uint32_t)(EXPR | EXPR | ...))`

<!--#### Enumerated integer constants

Given an enumerated set of integer constants named `NAME` in a library
`TAG`, a related set of compile-time constants has the following parts.

First, a typedef to give the type a name, a size, and a
signedness. The typedef should be of an explicitly sized integer
type. For example, if `uint32_t` is used:

```C
typedef uint32_t tag_name_t;
```

Each constant then has the form

```C
#define TAG_NAME_... EXPR
```

where `EXPR` is one of a handful of types of compile-time integer
constants (always wrapped in parentheses):

- `((tag_name_t)23)`
- `((tag_name_t)0x23)`
- `((tag_name_t)(TAG_NAME_FOO | TAG_NAME_BAR | ...))`

Do not include a count of values, which is difficult to maintain as
the set of constants grows.-->

#### 枚举整数常量

给一个库 `TAG` 中定义名为 `NAME` 的整数常量枚举集，一组相关的编译时常量包含以下部分。

首先，用 typedef 给该类型一个名称、一个大小和一个符号。typedef 应为显式大小的整数类型。例如，如果使用 `uint32_t`：

```C
typedef uint32_t tag_name_t;
```

每个常数都有

```C
#define TAG_NAME_... EXPR
```

其中 `EXPR` 是少数几种编译时整型常量之一（总是用括号括起来）：

- `((tag_name_t)23)`
- `((tag_name_t)0x23)`
- `((tag_name_t)(TAG_NAME_FOO | TAG_NAME_BAR | ...))`

不要包含太多值，因为随着常数集的增长会很难维护。

<!--#### Floating point constants

Floating point constants are similar to single integer constants,
except that a different mechanism is used to describe the type. Float
constants must end in `f` or `F`; double constants have no suffix;
long double constants must end in `l` or `L`. Hexadecimal versions of
floating point constants are allowed.-->

#### 浮点常量

浮点常量类似于单整数常量，除了用不同的机制来描述以。浮点常量必须以 `f` 或 `F` 结尾；double 常量不需要后缀；long double 常量必须以 `l` 或 `L` 结尾。允许十六进制的浮点常量。

<!--```C
// A float constant
#define TAG_FREQUENCY_LOW 1.0f

// A double constant
#define TAG_FREQUENCY_MEDIUM 2.0

// A long double constant
#define TAG_FREQUENCY_HIGH 4.0L
```-->

```C
// 浮点常量
#define TAG_FREQUENCY_LOW 1.0f

// double 常量
#define TAG_FREQUENCY_MEDIUM 2.0

// long double 常量
#define TAG_FREQUENCY_HIGH 4.0L
```

<!--### Function Declarations

Function declarations should all have names beginning with `tag_...`.

Function declarations should be placed in `extern "C"` guards. These
are canonically provided by using the `__BEGIN_CDECLS` and
`__END_CDECLS` macros from [compiler.h].-->

### 函数声明

函数声明的名称都应该以 `tag_...` 开头。

函数声明应该放在 `extern "C"` 防护区中。这些是通过使用 [compiler.h] 中的 `__BEGIN_CDECLS` 和 `__END_CDECLS` 宏来规范地提供的。

<!--#### Function parameters

Function parameters must be named. For example,

```C
// Disallowed: missing parameter name
zx_status_t tag_frob_vmo(zx_handle_t, size_t num_bytes);

// Allowed: all parameters named
zx_status_t tag_frob_vmo(zx_handle_t vmo, size_t num_bytes);
```-->

#### 函数参数

函数参数必须命名。例如，

```C
// 不允许：缺少参数名
zx_status_t tag_frob_vmo(zx_handle_t, size_t num_bytes);

// 允许：所有参数都有命名
zx_status_t tag_frob_vmo(zx_handle_t vmo, size_t num_bytes);
```

<!--It should be clear which parameters are consumed and which are
borrowed. Avoid interfaces in which clients may or may not own a
resource after a function call. If this is infeasible, consider noting
the ownership hazard in the name of the function, or one of its
parameters. For example:-->

应该明确哪些参数是消费的，哪些是借用的。避免在函数调用后客户端可能拥有或可能不拥有资源的接口。如果这是不可行的，考虑在函数或参数命名中体现该所有权风险。例如：

```C
zx_status_t tag_frobinate_subtle(zx_handle_t foo);
zx_status_t tag_frobinate_if_frobable(zx_handle_t foo);
zx_status_t tag_try_frobinate(zx_handle_t foo);
zx_status_t tag_frobinate(zx_handle_t maybe_consumed_foo);
```

<!--By convention, out parameters go last in a function's signature, and
should be named `out_*`.-->

按照惯例，输出参数在函数签名中位于最后，应该命名为 `out_*`。

<!--#### Variadic functions

Variadic functions should be avoided for everything except printf-like
functions. Those functions should document their format string
contract with the `__PRINTFLIKE` attribute from [compiler.h].-->

#### 变量函数

除了 printf 类函数外，其他所有函数都应避免使用变量函数。这些函数应该用 [compiler.h] 中的 `__PRINTFLIKE` 属性记录它们的格式字符串协定。

#### Static inline functions

Static inline functions are allowed, and are preferable to
function-like macros. Inline-only (that is, not also `static`) C
functions have complicated linkage rules and few use cases.

### Types

Prefer explicitly sized integer types (e.g. `int32_t`) to
non-explicitly sized types (e.g. `int` or `unsigned long int`). An
exemption is made for `int` when referring to POSIX file descriptors,
and for typedefs like `size_t` from the C or POSIX headers.

When possible, pointer types mentioned in interfaces should refer to
specific types. This includes pointers to opaque structs. `void*` is
acceptable for referring to raw memory, and to interfaces that pass
around opaque user cookies or contexts.

#### Opaque/Explicit types

Defining an opaque struct is preferable to using `void*`. Opaque
structs should be declared like:

```C
typedef struct tag_thing tag_thing_t;
```

Exposed structs should be declared like:

```C
typedef struct tag_thing {
} tag_thing_t;
```

#### Reserved fields

Any reserved fields in a struct should be documented as to the purpose
of the reservation.

A future version of this document will give guidance as to how to
describe string parameters in C interfaces.

#### Anonymous types

Top-level anonymous types are not allowed. Anonymous structures and
unions are allowed inside other structures, and inside function
bodies, as they are then not part of the top level namespace. For
instance, the following contains an allowed anonymous union.

```C
typedef struct tag_message {
    tag_message_type_t type;
    union {
        message_foo_t foo;
        message_bar_t bar;
    };
} tag_message_t;
```

#### Function typedefs

Typedefs for function types are permitted.

Functions should not overload return values with a `zx_status_t` on
failure and a positive success value. Functions should not overload
return values with a `zx_status_t` that contains additional values not
described in [zircon/errors.h].

#### Status return

Prefer `zx_status_t` as a return value to describe errors relating to
Zircon primitives and to I/O.

## Resource Management

Libraries can traffic in several kinds of resources. Memory and Zircon
handles are examples of resources common across many
libraries. Libraries may also define their own resources with
lifetimes to manage.

Ownership of all resources should be unambiguous. Transfer of
resources should be explicit in the name of a function. For example,
`create` and `take` connote a function transferring ownership.

Libraries should be memory tight. Memory allocated by a function like
`tag_thing_create` should released via `tag_thing_destroy` or some
such, not via `free`.

Libraries should not expose global variables. Instead, provide
functions to manipulate that state. Libraries with process-global
state must be dynamically linked, not statically. A common pattern is
to split a library into a stateless static part, containing almost all
of the code, and a small dynamic library holding global state.

In particular, the `errno` interface (which is a global thread-local
global) should be avoided in new code.

## Linkage

The default symbol visibility in a library should be hidden. Use
either a whitelist of exported symbols, or explicit visibility
annotations, on symbols to exported.

C libraries must not export C++ symbols.

## Evolution

### Deprecation

Deprecated functions should be marked with the __DEPRECATED attribute
from [compiler.h]. They should also be commented with a description
about what to do instead, and a bug tracking the deprecation.

## Disallowed or Discouraged Language Features

This section describes language features that cannot or should not be
used in the interfaces to Fuchsia's C libraries, and the rationales
behind the decisions to disallow them.

### Enums

C enums are banned. They are brittle from an ABI standpoint.

- The size of integer used to represent a constant of enum type is
  compiler (and compiler flag) dependent.
- The signedness of an enum is brittle, as adding a negative value to
  an enumeration can change the underlying type.

### Bitfields

C's bitfields are banned. They are brittle from an ABI standpoint, and
have a lot of nonintuitive sharp edges.

Note that this applies to the C language feature, not to an API which
exposes bit flags. The C bitfield feature looks like:

```C
typedef struct tag_some_flags {
    // Four bits for the frob state.
    uint8_t frob : 4;
    // Two bits for the grob state.
    uint8_t grob : 2;
} tag_some_flags_t;
```

We instead prefer exposing bit flags as compile-time integer
constants.

### Empty Parameter Lists

C allows for function `with_empty_parameter_lists()`, which are
distinct from `functions_that_take(void)`. The first means "take any
number and type of parameters", while the second means "take zero
parameters". We ban the empty parameter list for being too dangerous.

### Flexible Array Members

This is the C99 feature which allows declaring an incomplete array as
the last member of a struct with more than one parameter. For example:

```C
typedef struct foo_buffer {
    size_t length;
    void* elements[];
} foo_buffer_t;
```

As an exception, DDK structures are allowed to use this pattern when
referring to an external layout that fits this header-plus-payload
pattern.

The similar GCC extension of declaring a 0-sized array member is
similarly disallowed.

### Module Maps

These are a Clang extension to C-like languages which attempt to solve
many of the issues with header-driven compilation. While the Fuchsia
toolchain team is very likely to invest in these in the future, we
currently do not support them.

### Compiler Extensions

These are, by definition, not portable across toolchains.

This in particular includes packed attributes or pragmas, with one
exception for the DDK.

DDK structures often reflect an external layout that does not match
the system ABI. For instance, it may refer to an integer field that is
less aligned than required by the language. This can be expressed via
compiler extensions such as pragma pack.

[compiler.h]: https://fuchsia.googlesource.com/zircon/+/master/system/public/zircon/compiler.h
[library naming document]: ../languages/c-cpp/naming.md
