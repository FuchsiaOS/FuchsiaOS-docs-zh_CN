# C Library Readability Rubric

This document describes heuristics and rules for writing C libraries
that are published in the Fuchsia SDK.

A different document will be written for C++ libraries. While C++ is
almost an extension of C, and has some influence in this document, the
patterns for writing C++ libraries will be quite different than for C.

Most of this document is concerned with the description of an
interface in a C header. This is not a full C style guide, and has
little to say about the contents of C source files. Nor is this a
documentation rubric (though public interfaces should be well
documented).

Some C libraries have external constraints that contradict these
rules. For instance, the C standard library itself does not follow
these rules. This document should still be followed where applicable.

[TOC]

## Goals

### ABI Stability

Some Fuchsia interfaces with a stable ABI will be published as C
libraries. One goal of this document is to make it easy for Fuchsia
developers to write and to maintain a stable ABI. Accordingly, we
suggest not using certain features of the C language that have
potentially surprising or complicated implications on the ABI of an
interface. We also disallow nonstandard compiler extensions, since we
cannot assume that third parties are using any particular compiler,
with a handful of exceptions for the DDK described below.

### Resource Management

Parts of this document describe best practices for resource management
in C. This includes resources, Zircon handles, and any other type of
resource.

### Standardization

We would also like to adopt reasonably uniform standards for Fuchsia C
libraries. This is especially true of naming schemes. Out parameter
ordering is another example of standardization.

### FFI Friendliness

Some amount of attention is paid to Foreign Function Interface (FFI)
friendliness. Many non-C languages support C interfaces. The
sophistication of these FFI systems varies wildly, from essentially
sed to sophisticated libclang-based tooling. Some amount of
consideration of FFI friendliness went into these decisions.

## Language versions

### C

Fuchsia C libraries are written against the C11 standard (with a small
set of exceptions, such as unix signal support, that are not
particularly relevant to our C library ABI). C99 compliance is not a
goal.

In particular, Fuchsia C code can use the `<threads.h>` and
`<stdatomic.h>` headers from the C11 standard library, as well as the
`_Thread_local` and alignment language features.

The thread locals should use the `thread_local` spelling from
`<threads.h>`, rather than the built in `_Thread_local`. Similarly,
prefer `alignas` and `alignof` from `<stdalign.h>`, rather than
`_Alignas` and `_Alignof`.

Note that compilers support flags that may alter the ABI of the
code. For instance, GCC has a `-m96bit-long-double` flag that alters
the size of a long double. We assume that such flags are not used.

Finally, some libraries (such as Fuchsia's C standard library) in our
IDK are a mix of externally defined interfaces and Fuchsia specific
extensions. In these cases, we allow some pragmatism. For instance,
libc defines functions like `thrd_get_zx_handle` and
`dlopen_vmo`. These names are not strictly in accordance with the
rules below: the name of the library is not a prefix. Doing so would
make the names fit less well next to other functions like
`thrd_current` and `dlopen`, and so we allow the exceptions.

### C++

While C++ is not an exact superset of C, we still design C libraries
to be usable from C++. Fuchsia C headers should be compatible with the
C++11, C++14, and C++17 standards. In particular, function
declarations must be `extern "C"`, as described below.

C and C++ interfaces should not be mixed in one header. Instead,
create a separate `cpp` subdirectory and place C++ interfaces in their
own headers there.

## Library Layout and Naming

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
naming document], should be `lib/tag/foo.h`.

## Header Layout

A single header in a C library contains a few kinds of things.

- A copyright banner
- A header guard
- A list of file inclusions
- Extern C guards
- Constant declarations
- Extern symbol declarations
  - Including extern function declarations
- Static inline functions
- Macro definitions

### Header Guards

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
should have a header guard `LIB_TAG_OBJECT_BITS_H_`.

### Inclusions

Headers should include what they use. In particular, any public header
in a library should be safe to include first in a source file.

Libraries can depend on the C standard library headers.

Some libraries may also depend on a subset of POSIX headers. Exactly
which are appropriate is pending a forthcoming libc API review.

### Constant Definitions

Most constants in a library will be compile-time constants, created
via a `#define`. There are also read-only variables, declared via
`extern const TYPE NAME;`, as it sometimes is useful to have storage
for a constant (particularly for some forms of FFI). This section
describes how to provide compile time constants in a header.

There are several types of compile time constants.

- Single integer constants
- Enumerated integer constants
- Floating point constants

#### Single integer constants

A single integer constants has some `NAME` in a library `TAG`, and its
definition looks like the following.

```C
#define TAG_NAME EXPR
```

where `EXPR` has one of the following forms (for a `uint32_t`)

- `((uint32_t)23)`
- `((uint32_t)0x23)`
- `((uint32_t)(EXPR | EXPR | ...))`

#### Enumerated integer constants

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
the set of constants grows.

#### Floating point constants

Floating point constants are similar to single integer constants,
except that a different mechanism is used to describe the type. Float
constants must end in `f` or `F`; double constants have no suffix;
long double constants must end in `l` or `L`. Hexadecimal versions of
floating point constants are allowed.

```C
// A float constant
#define TAG_FREQUENCY_LOW 1.0f

// A double constant
#define TAG_FREQUENCY_MEDIUM 2.0

// A long double constant
#define TAG_FREQUENCY_HIGH 4.0L
```

### Function Declarations

Function declarations should all have names beginning with `tag_...`.

Function declarations should be placed in `extern "C"` guards. These
are canonically provided by using the `__BEGIN_CDECLS` and
`__END_CDECLS` macros from [compiler.h].

#### Function parameters

Function parameters must be named. For example,

```C
// Disallowed: missing parameter name
zx_status_t tag_frob_vmo(zx_handle_t, size_t num_bytes);

// Allowed: all parameters named
zx_status_t tag_frob_vmo(zx_handle_t vmo, size_t num_bytes);
```

It should be clear which parameters are consumed and which are
borrowed. Avoid interfaces in which clients may or may not own a
resource after a function call. If this is infeasible, consider noting
the ownership hazard in the name of the function, or one of its
parameters. For example:

```C
zx_status_t tag_frobinate_subtle(zx_handle_t foo);
zx_status_t tag_frobinate_if_frobable(zx_handle_t foo);
zx_status_t tag_try_frobinate(zx_handle_t foo);
zx_status_t tag_frobinate(zx_handle_t maybe_consumed_foo);
```

By convention, out parameters go last in a function's signature, and
should be named `out_*`.

#### Variadic functions

Variadic functions should be avoided for everything except printf-like
functions. Those functions should document their format string
contract with the `__PRINTFLIKE` attribute from [compiler.h].

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
either an allowlist of exported symbols, or explicit visibility
annotations on symbols to exported.

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

Note that this applies to the C language feature, not to an API that
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

This is the C99 feature that allows declaring an incomplete array as
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

These are part of a Clang extension to C-like languages that attempt to solve
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

[compiler.h]: /zircon/system/public/zircon/compiler.h
[library naming document]: development/languages/c-cpp/naming.md
[zircon/errors.h]: /zircon/system/public/zircon/errors.h
