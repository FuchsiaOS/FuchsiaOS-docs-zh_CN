<!-- 
# C Library Readability Rubric
 -->
# C 库可读性评分准则

<!-- 
This document describes heuristics and rules for writing C libraries
that are published in the Fuchsia SDK.
 -->
本文档描述了针对编写发布于 Fuchsia SDK 中的 C 库的启发与规则。

<!-- 
A different document will be written for C++ libraries. While C++ is
almost an extension of C, and has some influence in this document, the
patterns for writing C++ libraries will be quite different than for C.
 -->
关于 C++ 库的内容，将会编写另外的文档。虽然 C++ 几乎是 C 的扩展，并且在本文档中有一些影响，但编写 C++ 库的模式将与 C 有很大不同。

<!-- 
Most of this document is concerned with the description of an
interface in a C header. This is not a full C style guide, and has
little to say about the contents of C source files. Nor is this a
documentation rubric (though public interfaces should be well
documented).
 -->
本文档的大部分内容都与 C 头文件中的接口描述有关。这并非完整的 C 风格指南，对 C 源文件的内容几乎没有提及。这亦非一个文档评分标准（尽管公共接口应当记录完善）。

<!-- 
Some C libraries have external constraints that contradict these
rules. For instance, the C standard library itself does not follow
these rules. This document should still be followed where applicable.
 -->
一些 C 库具有与此相悖的外部约束规则。例如，C 标准库本身不遵循这些规则。在适用的情况下，仍应当遵循本文档。

[TOC]

<!-- 
## Goals
 -->
## 目标

<!-- 
### ABI Stability
 -->
### ABI 稳定性

<!-- 
Some Fuchsia interfaces with a stable ABI will be published as C
libraries. One goal of this document is to make it easy for Fuchsia
developers to write and to maintain a stable ABI. Accordingly, we
suggest not using certain features of the C language that have
potentially surprising or complicated implications on the ABI of an
interface. We also disallow nonstandard compiler extensions, since we
cannot assume that third parties are using any particular compiler,
with a handful of exceptions for the DDK described below.
 -->
一些具有稳定 ABI 的 Fuchsia 接口将作为 C 库发布。本文档的一个目标是让 Fuchsia 开发人员可以轻松编写和维护稳定的 ABI。因此，我们建议不要使用 C 语言的某些功能，这些功能可能会对接口的 ABI 产生惊人或复杂的影响。我们也不允许非标准的编译器扩展，因为我们不能假设第三方使用任何特定的编译器，除了下述针对 DDK 的少数例外。

<!-- 
### Resource Management
 -->
### 资源管理

<!-- 
Parts of this document describe best practices for resource management
in C. This includes resources, Zircon handles, and any other type of
resource.
 -->
本文档的部分内容描述了 C 中资源管理的最佳做法。这包括资源、Zircon 句柄及其他任何类型的资源。

<!-- 
### Standardization
 -->
### 标准化

<!-- 
We would also like to adopt reasonably uniform standards for Fuchsia C
libraries. This is especially true of naming schemes. Out parameter
ordering is another example of standardization.
 -->
我们还希望为 Fuchsia C 库采用合理统一的标准。命名方案尤其如此。另外还有 out 参数排序等。

<!-- 
### FFI Friendliness
 -->
### FFI 友好性

<!-- 
Some amount of attention is paid to Foreign Function Interface (FFI)
friendliness. Many non-C languages support C interfaces. The
sophistication of these FFI systems varies wildly, from essentially
sed to sophisticated libclang-based tooling. Some amount of
consideration of FFI friendliness went into these decisions.
 -->
外部功能接口 (FFI) 的友好性受到了一定程度的关注。许多非 C 语言都支持 C 接口。这些 FFI 系统的复杂程度千差万别，从基本的 sed 到基于 libclang 的复杂工具。它们的决定体现了对 FFI 友好性的一些考虑。

<!-- 
## Language versions
 -->
## 语言版本

### C

<!-- 
Fuchsia C libraries are written against the C11 standard (with a small
set of exceptions, such as unix signal support, that are not
particularly relevant to our C library ABI). C99 compliance is not a
goal.
 -->
Fuchsia C 库依据 C11 标准编写（有一小部分例外，如 unix 信号支持，其与我们的 C 库 ABI 不是特别相关）。C99 合规性不是目标。

<!-- 
In particular, Fuchsia C code can use the `<threads.h>` and
`<stdatomic.h>` headers from the C11 standard library, as well as the
`_Thread_local` and alignment language features.
 -->
特别地，Fuchsia C 代码可以使用 C11 标准库中的`<threads.h>`和`<stdatomic.h>` 头文件，以及 `_Thread_local` 和对齐语言功能。

<!-- 
The thread locals should use the `thread_local` spelling from
`<threads.h>`, rather than the built in `_Thread_local`. Similarly,
prefer `alignas` and `alignof` from `<stdalign.h>`, rather than
`_Alignas` and `_Alignof`.
 -->
线程局部变量应当使用 `<threads.h>` 中的 `thread_local` 写法，而非内置的 `_Thread_local`。类似地，应当使用 `<stdalign.h>` 中的 `alignas` 和 `alignof`，而非 `_Alignas` 和 `_Alignof`。

<!-- 
Note that compilers support flags that may alter the ABI of the
code. For instance, GCC has a `-m96bit-long-double` flag that alters
the size of a long double. We assume that such flags are not used.
 -->
请注意，编译器支持可能会改变代码 ABI 的标志。例如，GCC 中的 `-m96bit-long-double` 标志可以改变 long double 的大小。我们假设不使用此类标志。

<!-- 
Finally, some libraries (such as Fuchsia's C standard library) in our
IDK are a mix of externally defined interfaces and Fuchsia specific
extensions. In these cases, we allow some pragmatism. For instance,
libc defines functions like `thrd_get_zx_handle` and
`dlopen_vmo`. These names are not strictly in accordance with the
rules below: the name of the library is not a prefix. Doing so would
make the names fit less well next to other functions like
`thrd_current` and `dlopen`, and so we allow the exceptions.
 -->
最后，我们 IDK 中的一些库（例如 Fuchsia 的 C 标准库）是外部定义接口和 Fuchsia 特定扩展的混合体。在这些情况下，我们允许一定的实用主义。例如，libc 定义了 `thrd_get_zx_handle` 和 `dlopen_vmo` 等函数。这些名称不严格遵守以下规则：库的名称不是前缀。这样做会使其名称与其他函数（如 `thrd_current` 和 `dlopen` ）不太吻合，因此我们允许这些例外。

### C++

<!-- 
While C++ is not an exact superset of C, we still design C libraries
to be usable from C++. Fuchsia C headers should be compatible with the
C++11, C++14, and C++17 standards. In particular, function
declarations must be `extern "C"`, as described below.
 -->
虽然 C++ 并非 C 的精确超集，但是我们仍将 C 库设计得可从 C++ 使用。Fuchsia C 头文件应与 C++11、C++14 和 C++17 标准兼容。特别地，函数声明必须是 `extern "C"`，如下所述。

<!-- 
C and C++ interfaces should not be mixed in one header. Instead,
create a separate `cpp` subdirectory and place C++ interfaces in their
own headers there.
 -->
C 和 C++ 接口不应混合在同一头文件中。而是应当创建一个单独的 `cpp` 子目录，并将 C++ 接口置于其自己的头文件中。

<!-- 
## Library Layout and Naming
 -->
## 库的布局与命名

<!-- 
A Fuchsia C library has a name. This name determines its include path
(as described in the [library naming document]) as well as identifiers
within the library.
 -->
Fuchsia C 库具有一个名称。该名称决定其包含路径（如[库命名文档][library naming document]中所述）以及库中的标识符。

<!-- 
In this document, the library is always named `tag`, and is variously
referred to as `tag` or `TAG` or `Tag` or `kTag` to reflect a
particular lexical convention. The `tag` should be a single identifier
without underscores. The all-lowercase form of a tag is given by the
regular expression `[a-z][a-z0-9]*`.  A tag can be replaced by a shorter
version of the library name, for example `zx` instead of `zircon`.
 -->
本文档中，库始终命名为 `tag`，并以不同的方式称为 `tag` 、`TAG`、`Tag` 或 `kTag` 以反映特定的用词习惯。`tag` 应当是不带下划线的单个标识符。tag 的全小写形式由正则表达式“[a-z][a-z0-9]*”给出。tag 可以替换为库名称的较短版本，例如 `zx` 而非 `zircon`。

<!-- 
The include path for a header `foo.h`, as described by the [library
naming document], should be `lib/tag/foo.h`.
 -->
如[库命名文档][library naming document]所述，一个头文件 `foo.h` 的包含路径应为 `lib/tag/foo.h`。

<!-- 
## Header Layout
 -->
## 头文件布局

<!-- 
A single header in a C library contains a few kinds of things.
 -->
C 库中的单个头文件包含几样内容。

<!-- 
- A copyright banner
- A header guard
- A list of file inclusions
- Extern C guards
- Constant declarations
- Extern symbol declarations
  - Including extern function declarations
- Static inline functions
- Macro definitions
 -->
- 版权片段
- 头文件保护
- 文件包含列表
- extern C 保护
- 常量声明
- 外部符号声明
  - 包含外部函数声明
- 静态内联函数
- 宏定义

<!-- 
### Header Guards
 -->
### 头文件保护

<!-- 
Use #ifndef guards in headers. These look like:
 -->
在头文件中使用 #ifdef 保护。形如：

```C
#ifndef SOMETHING_MUMBLE_H_
#define SOMETHING_MUMBLE_H_

// code
// code
// code

#endif // SOMETHING_MUMBLE_H_
```

<!-- 
The exact form of the define is as follows:
 -->
define 的具体形式如下：

<!-- 
- Take the canonical include path to the header
- Replace all ., /, and - with _
- Convert all letters to UPPERCASE
- Add a trailing _
 -->
- 采用标题的规范包含路径
- 将所有“.”、“/”和“-”替换为“_”
- 将所有字母转换为大写
- 结尾添加“_”

<!-- 
For example, the header located in the SDK at `lib/tag/object_bits.h`
should have a header guard `LIB_TAG_OBJECT_BITS_H_`.
 -->
例如，位于 `lib/tag/object_bits.h` 的 SDK 中的头文件应当有一个头文件保护 `LIB_TAG_OBJECT_BITS_H_`。

<!-- 
### Inclusions
 -->
### 包含项

<!-- 
Headers should include what they use. In particular, any public header
in a library should be safe to include first in a source file.
 -->
头文件应当包含他们使用的内容。特别地，库中的任何公共头文件都应当首先是可以安全地包含在源文件中的。

<!-- 
Libraries can depend on the C standard library headers.
 -->
库可以依赖于 C 标准库头文件。

<!-- 
Some libraries may also depend on a subset of POSIX headers. Exactly
which are appropriate is pending a forthcoming libc API review.
 -->
一些库可能还依赖于 POSIX 头文件的子集。确切的恰当列表正待即将进行的 libc API 审查。

<!-- 
### Constant Definitions
 -->
### 常量定义

<!-- 
Most constants in a library will be compile-time constants, created
via a `#define`. There are also read-only variables, declared via
`extern const TYPE NAME;`, as it sometimes is useful to have storage
for a constant (particularly for some forms of FFI). This section
describes how to provide compile time constants in a header.
 -->
库中的大多数常量都是编译时常量，通过 `#define` 创建。还有只读变量，通过 `extern const TYPE NAME;` 声明，因为有时存储常量很有用（特别是对于某些形式的 FFI）。本节介绍在头文件中提供编译时常量的方法。

<!-- 
There are several types of compile time constants.
 -->
编译时常量有几种类型。

<!-- 
- Single integer constants
- Enumerated integer constants
- Floating point constants
 -->
- 单整型常量
- 枚举整型常量
- 浮点常量

<!-- 
#### Single integer constants
 -->
#### 单整型常量

<!-- 
A single integer constants has some `NAME` in a library `TAG`, and its
definition looks like the following.
 -->
单整型常量在库 `TAG` 中有某个 `NAME`，其定义如下所示。


```C
#define TAG_NAME EXPR
```

<!-- 
where `EXPR` has one of the following forms (for a `uint32_t`)
 -->
其中 `EXPR` 具有以下形式之一（对于 `uint32_t` 而言）：

- `((uint32_t)23)`
- `((uint32_t)0x23)`
- `((uint32_t)(EXPR | EXPR | ...))`

<!-- 
#### Enumerated integer constants
 -->
#### 枚举整型常量

<!-- 
Given an enumerated set of integer constants named `NAME` in a library
`TAG`, a related set of compile-time constants has the following parts.
 -->
给定库 `TAG` 中名为 `NAME` 的一组枚举整型常量，一组相关的编译时常量具有以下部分。

<!-- 
First, a typedef to give the type a name, a size, and a
signedness. The typedef should be of an explicitly sized integer
type. For example, if `uint32_t` is used:
 -->
首先，一个为类型提供名称、大小和有无符号性的 typedef。typedef 应当是显式确定大小的整数类型。例如，如果使用 `uint32_t`：

```C
typedef uint32_t tag_name_t;
```

<!-- 
Each constant then has the form
 -->
每个常量具有以下形式

```C
#define TAG_NAME_... EXPR
```

<!-- 
where `EXPR` is one of a handful of types of compile-time integer
constants (always wrapped in parentheses):
 -->
其中 `EXPR` 是少数几种编译时整型常量类型之一（总是括在括号中）：

- `((tag_name_t)23)`
- `((tag_name_t)0x23)`
- `((tag_name_t)(TAG_NAME_FOO | TAG_NAME_BAR | ...))`

<!-- 
Do not include a count of values, which is difficult to maintain as
the set of constants grows.
 -->
不要包含值的计数，随着常量集合的增长，这很难维护。

<!-- 
#### Floating point constants
 -->
#### 浮点常量

<!-- 
Floating point constants are similar to single integer constants,
except that a different mechanism is used to describe the type. Float
constants must end in `f` or `F`; double constants have no suffix;
long double constants must end in `l` or `L`. Hexadecimal versions of
floating point constants are allowed.
 -->
浮点常量类似于单整型常量，只是使用了不同的机制来描述该类型。浮点常量必须以 `f` 或 `F` 结尾；双精度常量没有后缀；long double 常量必须以 `l` 或 `L` 结尾。允许使用十六进制版本的浮点常量。

```C
// A float constant
#define TAG_FREQUENCY_LOW 1.0f

// A double constant
#define TAG_FREQUENCY_MEDIUM 2.0

// A long double constant
#define TAG_FREQUENCY_HIGH 4.0L
```

<!-- 
### Function Declarations
 -->
### 函数声明

<!-- 
Function declarations should all have names beginning with `tag_...`.
 -->
函数声明应当具有以 `tag_` 开头的名称。

<!-- 
Function declarations should be placed in `extern "C"` guards. These
are canonically provided by using the `__BEGIN_CDECLS` and
`__END_CDECLS` macros from [compiler.h].
 -->
函数声明应当置于 `extern "C"` 保护内。这些是通过使用 [compiler.h] 中的 `__BEGIN_CDECLS` 和 `__END_CDECLS` 宏规范地提供的。

<!-- 
#### Function parameters
 -->
#### 函数参数

<!-- 
Function parameters must be named. For example,
 -->
函数参数必须命名。例如，

```C
// Disallowed: missing parameter name
zx_status_t tag_frob_vmo(zx_handle_t, size_t num_bytes);

// Allowed: all parameters named
zx_status_t tag_frob_vmo(zx_handle_t vmo, size_t num_bytes);
```

<!-- 
It should be clear which parameters are consumed and which are
borrowed. Avoid interfaces in which clients may or may not own a
resource after a function call. If this is infeasible, consider noting
the ownership hazard in the name of the function, or one of its
parameters. For example:
 -->
应当清楚哪些参数是被使用的（consumed），哪些是被借用的（borrowed）。请避免在函数调用后客户端不一定拥有资源的接口。如果不可行，请考虑在函数名称或其参数之一中注明所有权风险。例如：

```C
zx_status_t tag_frobinate_subtle(zx_handle_t foo);
zx_status_t tag_frobinate_if_frobable(zx_handle_t foo);
zx_status_t tag_try_frobinate(zx_handle_t foo);
zx_status_t tag_frobinate(zx_handle_t maybe_consumed_foo);
```

<!-- 
By convention, out parameters go last in a function's signature, and
should be named `out_*`.
 -->
习惯上，out 参数在函数签名中排在最后，并且应当命名为 `out_*` 。

<!-- 
#### Variadic functions
 -->
#### 可变参数函数

<!-- 
Variadic functions should be avoided for everything except printf-like
functions. Those functions should document their format string
contract with the `__PRINTFLIKE` attribute from [compiler.h].
 -->
除了类 printf 函数外，应在所有情况下避免使用可变参数函数。这些函数应当使用 [compiler.h] 中的 `__PRINTFLIKE` 属性记录它们的格式化字符串约定。

<!-- 
#### Static inline functions
 -->
#### 静态内联函数

<!-- 
Static inline functions are allowed, and are preferable to
function-like macros. Inline-only (that is, not also `static`) C
functions have complicated linkage rules and few use cases.
 -->
静态内联函数是允许的，并且优于类函数宏。仅内联的（即亦非“静态”）C 函数具有复杂的链接规则和很少的用例。

<!-- 
### Types
 -->
### 类型

<!-- 
Prefer explicitly sized integer types (e.g. `int32_t`) to
non-explicitly sized types (e.g. `int` or `unsigned long int`). An
exemption is made for `int` when referring to POSIX file descriptors,
and for typedefs like `size_t` from the C or POSIX headers.
 -->
优先使用显式确定大小的整数类型（例如 `int32_t`），而不是非显式确定大小的类型（例如 `int` 或 `unsigned long int`）。对于 POSIX 文件描述符引用中的 `int`，以及 C 或 POSIX 头文件中的 `size_t` 之类的类型定义，本规则可以免除。

<!-- 
When possible, pointer types mentioned in interfaces should refer to
specific types. This includes pointers to opaque structs. `void*` is
acceptable for referring to raw memory, and to interfaces that pass
around opaque user cookies or contexts.
 -->
如果可能，接口中提到的指针类型应当指向特定类型。这包括指向不透明结构体的指针。`void*` 可以用于引用原始内存，以及传递不透明用户 cookie 或上下文的接口。

<!-- 
#### Opaque/Explicit types
 -->
#### 不透明/显式类型

<!-- 
Defining an opaque struct is preferable to using `void*`. Opaque
structs should be declared like:
 -->
定义一个不透明（opaque）结构体比使用 `void*` 更可取。不透明结构体应当如此声明：

```C
typedef struct tag_thing tag_thing_t;
```

<!-- 
Exposed structs should be declared like:
 -->
公开的结构体应当如此声明：

```C
typedef struct tag_thing {
} tag_thing_t;
```

<!-- 
#### Reserved fields
 -->
#### 保留字段

<!-- 
Any reserved fields in a struct should be documented as to the purpose
of the reservation.
 -->
结构体中的任何保留字段都应当记录保留的目的。

<!-- 
A future version of this document will give guidance as to how to
describe string parameters in C interfaces.
 -->
本文档的未来版本将提供有关在 C 接口中描述字符串参数方法的指导。

<!-- 
#### Anonymous types
 -->
#### 匿名类型

<!-- 
Top-level anonymous types are not allowed. Anonymous structures and
unions are allowed inside other structures, and inside function
bodies, as they are then not part of the top level namespace. For
instance, the following contains an allowed anonymous union.
 -->
顶级匿名类型是不允许的。匿名结构体和共用体允许在其他结构体和函数体内使用，因为它们不是顶级命名空间的一部分。例如，以下包含允许的匿名共用体。

```C
typedef struct tag_message {
    tag_message_type_t type;
    union {
        message_foo_t foo;
        message_bar_t bar;
    };
} tag_message_t;
```

<!-- 
#### Function typedefs
 -->
#### 函数类型定义

<!-- 
Typedefs for function types are permitted.
 -->
针对函数类型的类型定义是允许的。

<!-- 
Functions should not overload return values with a `zx_status_t` on
failure and a positive success value. Functions should not overload
return values with a `zx_status_t` that contains additional values not
described in [zircon/errors.h].
 -->
函数不应在失败时使用 `zx_status_t` 来重载返回值，也不应使用正的成功值重载。函数不应使用包含了 [zircon/errors.h] 中未描述附加值的 `zx_status_t` 重载返回值。

<!-- 
#### Status return
 -->
#### 状态返回

<!-- 
Prefer `zx_status_t` as a return value to describe errors relating to
Zircon primitives and to I/O.
 -->
优先使用 `zx_status_t` 作为返回值来描述与 Zircon 原语和 I/O 相关的错误。

<!-- 
## Resource Management
 -->
## 资源管理

<!-- 
Libraries can traffic in several kinds of resources. Memory and Zircon
handles are examples of resources common across many
libraries. Libraries may also define their own resources with
lifetimes to manage.
 -->
库可以使用几种类型的资源。内存和 Zircon 句柄是许多库中常见资源的例子。库也可以定义自己的资源，并对其进行寿命管理。

<!--
Ownership of all resources should be unambiguous. Transfer of
resources should be explicit in the name of a function. For example,
`create` and `take` connote a function transferring ownership.
 -->
所有资源的所有权应当是明确的。资源应以函数的名义显式转移。例如，`create` 和 `take` 表示转移所有权的函数。

<!--
Libraries should be memory tight. Memory allocated by a function like
`tag_thing_create` should released via `tag_thing_destroy` or some
such, not via `free`.
 -->
库应当是内存紧密的（memory tight）。由例如 `tag_thing_create` 函数分配的内存应当通过 `tag_thing_destroy` 之类函数释放，而非通过 `free`。

<!--
Libraries should not expose global variables. Instead, provide
functions to manipulate that state. Libraries with process-global
state must be dynamically linked, not statically. A common pattern is
to split a library into a stateless static part, containing almost all
of the code, and a small dynamic library holding global state.
 -->
库不应公开全局变量。相反，应当提供用以操控该状态的函数。具有全局于过程的状态的库必须动态链接，而不可静态链接。一个常见的模式是将库拆分为一个包含几乎所有代码的无状态的静态部分，以及一个含有全局状态的小型动态库。

<!--
In particular, the `errno` interface (which is a global thread-local
global) should be avoided in new code.
 -->
特别地，应当避免在新代码中避免 `errno` 接口（一个局部于全局线程的全局变量（a global thread-local global））。

<!--
## Linkage
 -->
## 链接

<!--
The default symbol visibility in a library should be hidden. Use
either an allowlist of exported symbols, or explicit visibility
annotations on symbols to exported.
 -->
库中的默认符号可见性应为隐藏。请使用导出符号的允许列表，或使用针对要导出的符号的显式可见性注释。

<!--
C libraries must not export C++ symbols.
 -->
C 库不得导出 C++ 符号。

<!--
## Evolution
 -->
## 演化

<!--
### Deprecation
 -->
### 弃用

<!--
Deprecated functions should be marked with the __DEPRECATED attribute
from [compiler.h]. They should also be commented with a description
about what to do instead, and a bug tracking the deprecation.
 -->
已弃用函数应当使用来自 [Compiler.H] 的 __DEPRECATED 属性进行标记。还应对其进行注释，描述代替做法，并跟踪弃用的错误。

<!--
## Disallowed or Discouraged Language Features
 -->
## 不允许或不鼓励的语言特性

<!--
This section describes language features that cannot or should not be
used in the interfaces to Fuchsia's C libraries, and the rationales
behind the decisions to disallow them.
 -->
本节介绍了无法或不应在 Fuchsia 的 C 库中使用的语言特性及个中缘由。

<!--
### Enums
 -->
### 枚举

<!--
C enums are banned. They are brittle from an ABI standpoint.
 -->
C 枚举（enum）是禁止的。从 ABI 的角度来看，该特性是脆弱的。

<!--
- The size of integer used to represent a constant of enum type is
  compiler (and compiler flag) dependent.
- The signedness of an enum is brittle, as adding a negative value to
  an enumeration can change the underlying type.
 -->
- 用于表示枚举类型常数的整数大小是依赖于编译器（和编译器标志）的。
- 枚举的有无符号性是脆弱的，因为将负值加至枚举变量可以改变其内部类型。

<!--
### Bitfields
 -->
### 位域

<!--
C's bitfields are banned. They are brittle from an ABI standpoint, and
have a lot of nonintuitive sharp edges.
 -->
C 位域（bitfield）是禁止的。从 ABI 的角度来看，该特性是脆弱的，并且具有许多非直观的不良影响。

<!--
Note that this applies to the C language feature, not to an API that
exposes bit flags. The C bitfield feature looks like:
 -->
请注意，这适用于 C 语言功能，而非公开位标志的 API。C 位域特性形如：

<!--
```C
typedef struct tag_some_flags {
    // Four bits for the frob state.
    uint8_t frob : 4;
    // Two bits for the grob state.
    uint8_t grob : 2;
} tag_some_flags_t;
```
 -->
```C
typedef struct tag_some_flags {
    // 为 frob 状态分配 4 比特。
    uint8_t frob : 4;
    // 为 grob 状态分配 2 比特。
    uint8_t grob : 2;
} tag_some_flags_t;
```

<!--
We instead prefer exposing bit flags as compile-time integer
constants.
 -->
相反，我们倾向于将位标志作为编译时的整数常数。

<!--
### Empty Parameter Lists
 -->
### 空参数列表

<!--
C allows for function `with_empty_parameter_lists()`, which are
distinct from `functions_that_take(void)`. The first means "take any
number and type of parameters", while the second means "take zero
parameters". We ban the empty parameter list for being too dangerous.
 -->
C 允许使用函数 `with_empty_parameter_lists()`，其与 `functions_that_take（void）` 不同。前者表示“接受任何数量和类型的参数”，而后者表示“接受零参数”。我们因前者的危险性而将其禁止。

<!--
### Flexible Array Members
 -->
### 灵活数组成员

<!--
This is the C99 feature that allows declaring an incomplete array as
the last member of a struct with more than one parameter. For example:
 -->
这是 C99 特性，它允许将不完整的数组声明为具有多个参数的结构体的最后一个成员。例如：

```C
typedef struct foo_buffer {
    size_t length;
    void* elements[];
} foo_buffer_t;
```

<!--
As an exception, DDK structures are allowed to use this pattern when
referring to an external layout that fits this header-plus-payload
pattern.
 -->
例外情况是，当 DDK 结构体在引用适合该头文件加载荷模式（header-plus-payload
pattern）的外部布局时，允许其使用此模式。

<!--
The similar GCC extension of declaring a 0-sized array member is
similarly disallowed.
 -->
类似地，声明零大小数组成员的 GCC 扩展特性同样是不允许的。

<!--
### Module Maps
 -->
### 模块映射（module map）

<!--
These are part of a Clang extension to C-like languages that attempt to solve
many of the issues with header-driven compilation. While the Fuchsia
toolchain team is very likely to invest in these in the future, we
currently do not support them.
 -->
这是 Clang 针对类 C 语言的扩展特性之一，试图通过由头文件驱动的编译来解决许多问题。尽管 Fuchsia 工具链团队未来很可能会投入该类特性的开发，但目前是不支持的。

<!--
### Compiler Extensions
 -->
### 编译器扩展

<!--
These are, by definition, not portable across toolchains.
 -->
定义上，编译器扩展不是跨工具链可移植的。

<!--
This in particular includes packed attributes or pragmas, with one
exception for the DDK.
 -->
具体地，这包含打包的属性或 pragma，除了 DDK 这一例外。

<!--
DDK structures often reflect an external layout that does not match
the system ABI. For instance, it may refer to an integer field that is
less aligned than required by the language. This can be expressed via
compiler extensions such as pragma pack.
 -->
DDK 结构体通常反映出与系统 ABI 不符的外部布局。例如，它可以引用一个不满足语言对齐要求的整数字段。这可以通过例如 pragma pack 的编译器扩展表明。

[compiler.h]: /zircon/system/public/zircon/compiler.h
[library naming document]: /development/languages/c-cpp/naming.md
[zircon/errors.h]: /zircon/system/public/zircon/errors.h
