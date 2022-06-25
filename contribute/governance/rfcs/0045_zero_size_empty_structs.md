{% set rfcid = "RFC-0045" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-045.

## Rejection rationale

This RFC has been rejected by its author due to higher-priority work, and
low impact.
We can revisit this at a later point if the ideas in this turn out to have
higher impact in the future.

As a consequence of this, some sections are incomplete.

## Summary

[RFC-0056](contribute/governance/rfcs/0056_empty_structs.md) ("Empty Structs") improved language ergonomics by
enabling empty structs to be defined.
Empty structs carry no content, but they currently occupy one byte in the
wire format to be compatible across all FIDL language implementations.
This uses unnecessary space, which is typically made worse due to FIDL's
eight-byte alignment in many contexts.

This RFC builds on RFC-0056 by enhancing empty structs to occupy zero bytes
on the wire.

## Motivation

[RFC-0056](contribute/governance/rfcs/0056_empty_structs.md) identifies use cases for empty structs:

*   planned future use,
*   as an option in a union,
*   as part of a [Command
    pattern](https://en.wikipedia.org/wiki/Command_pattern),

Besides the general inelegance of an object that carries zero information
taking a non-zero amount of space on the wire, an efficient implementation
of empty structs can be important for potential future FIDL work, such as
[Algebraic Data Types](https://en.wikipedia.org/wiki/Algebraic_data_type)
or [Generics](https://en.wikipedia.org/wiki/Generic_programming).

A size-one empty struct design also makes every other FIDL target language
pay a cost that's unique to C++ (see the [Design](#design) below for more
details).
Other languages can typically represent an empty struct with zero bytes.

## Design

Two designs: one choice. We gotta pick.

I prefer design #1, where we use zero-length arrays to represent empty
structs.
It's less surprising for users, and it's also uniform and consistent
across all use cases.
The downside is that we need C extensions, which may not be acceptable.

Design #2 can be done, but there's arguably surprising ergonomics when
empty structs are used as a parameter to a FIDL method.
I'd love some ideas and feedback about that.

We can also assume design #1 if the compiler supports zero-length arrays,
and design #2 if not. I kinda like that.

## Design #1: Zero-Length Arrays

This RFC proposes using zero-length arrays to represent an empty struct.
This is a commonly supported extension that is supported by FIDL's target
C & C++ compilers.

A simplified example looks similar for both C & C++:

```c
// FIDL: struct Empty {};

// define a zero-length array type
typedef int FIDLEmptyStructInternal[0];

typedef struct {
  FIDLEmptyStructInternal reserved;
} Empty;
```

The above code snippet asserts to `sizeof(Empty) == 0` for both C & C++.
In practice, generated code for bindings should turn off various warnings
for C and C++[[1]](#Footnote1); a [Github Gist] shows a more complete
example of generated C & C++ bindings.

## Design #2: Omit Emitting Empty Structs Altogether

FIDL structs are required to be cast to equivalent structs in C and C++.
Empty structs are a special case, since C and C++ differ in how they treat
empty structs:

*   C leaves the size of an empty struct undefined[[2]](#Footnote2).
    Many compilers (e.g., `gcc` & `clang`) therefore define an empty
    struct to be
    [zero-sized](https://gcc.gnu.org/onlinedocs/gcc/Empty-Structures.html).
*   C++ defines empty structs to have a [size of
    1](http://www.stroustrup.com/bs_faq2.html#sizeof-empty).
    An "[empty base
    class](https://en.cppreference.com/w/cpp/language/ebo)" optimization is
    employed by most compilers to optimize them to 0 under certain
    circumstances.

As a workaround, RFC-0056 proposed generating a struct with a single
`uint8` member to represent an empty struct, which is consistent in both C
and C++.

There are three different contexts where empty structs appear:

*   inside a containing struct,
*   inside a containing union or table, or
*   as a "top-level" struct by being a parameter to a FIDL interface
    method.

These three contexts can be handled separately.

### Inside a Containing Struct

An empty struct inside a containing struct can have the member for the
empty struct omitted.
For example, this FIDL struct:

```fidl
// FIDL
struct FooOptions {
    // There is currently no information here but
    // we have preserved the plumbing for now.
};

struct Foo {
    uint32 before;
    FooOptions options;
    uint32 after;
};
```

can simply omit generating the "FooOptions" empty struct member in the
C/C++ bindings:


```c
// generated C or C++ code
struct Foo {
    uint32_t before;
    // "FooOptions options" is missing here
    uint32_t after;
};
```

The serialized FIDL wire format is then compatible with the C/C++ memory
layout, and can be cast directly to/from either format.

Since the empty struct contains no information, not having access to the
`.options` member carries little consequence.
If the struct later changes to become non-empty, the containing struct can
emit the formerly empty struct member `options` in a source
compatible way [[3]](#Footnote3).

One reasonable operation that people may wish to to do is to take the
address of an empty struct, i.e. `&(foo.options)`, which will no longer be
possible with this change.
We think this is an acceptable trade-off for consistent, cross-language
zero-size empty structs.

> TODO(apang): Go, Rust, Dart.

### Inside a Containing Table or Union

Tables or (static or extensible) unions have ordinals ("tags") that
indicate what information the table/union carries.
In this case, an empty struct "carries information" in the sense that the
presence of it represents information, even though the empty struct itself
carries no information.

As such, tables or unions will still emit the ordinal so that client code
can inspect it to determine what information is in the table/union.
However, the empty struct itself will not be accessible.
For example, a union of empty structs:

```fidl
// FIDL
struct Option1 {};
struct Option2 {};
union {
    // an "empty" union!
    Option1 o1;
    Option2 o2;
};
```

would still:

1. have a well-defined memory layout, which will contain the single
   `uint32` enum tag.
2. emit enums representing the ordinals and appropriate accessor methods,
   so that client code can create and inspect such unions.

> TODO(apang): Include example C/C++ binding.

Tables are similar: the presence of an empty struct as a table field
represents information.
The same approach for unions &mdash; emit enumerations for ordinals and
client code, but omit access to the empty struct &mdash; can be used for
tables.

> TODO(apang): Go, Rust, Dart.

### As a FIDL Method Parameter

There are existing use cases where an empty struct is used as a FIDL
method parameter, e.g., in [fuchsia.ui.viewsv1]:

```fidl
interface ViewContainerListener {
    // |ViewInfo| is an empty struct
    OnChildAttached(uint32 child_key, ViewInfo child_view_info) -> ();
};

struct ViewInfo {};
```

Any method parameters that are empty structs can either be:

1. omitted from the method signature (recommended),
2. canonicalized to single empty struct singleton type in C or C++ (e.g.,
   a `fidl::EmptyStruct`) that doesn't directly map to the zero-byte wire
   format, or
3. emitted as-is, with a language representation that doesn't directly
   encode/decode to the wire format.

### Changes

*   The FIDL source language does not need to be changed.
*   The FIDL wire format and documentation will change, so that empty
    structs take 0 bytes on the wire, instead of 1.
*   `fidlc` needs no changes.
*   Every language backend (C, C++, Rust, Go, Dart) needs to be updated to
    reflect the bindings changes discussed in this section.
    This should be done as a hard transition so that cross-layer ABI
    compatibility is preserved.

## Implementation strategy

The implementation will be similar to the [RFC-0056](contribute/governance/rfcs/0056_empty_structs.md),
and needs to be split across multiple CLs:

*   CLs to update generated bindings for all languages, without updating
    cross-language compatibility tests.
*   A hard-transition integration CL to ensure rollers succeed.
*   Update cross-language.
*   Update.

The FIDL source language does not need to be changed for this RFC.

## Drawbacks, alternatives, and unknowns

Note that

- that's consistent across both C and C++
- the two languages have different size implementations
- C++ notionally requires that
- zero-length array
- C++ [[no_unique_address]]

## Prior art and references

[https://herbsutter.com/2009/09/02/when-is-a-zero-length-array-okay/](https://herbsutter.com/2009/09/02/when-is-a-zero-length-array-okay/)

--------------------------------------

Footnotes
---------
##### Footnote1
`-Wzero-length-array`, with `-Wc++-compat` for C and `-Wextern-c-compat`
for C++.
The warnings can be scoped with the commonly supported `#pragma`
diagnostic push/ignored/pop compiler directive, so that warnings apply
only to the empty struct code.

##### Footnote2
C99, 6.7.2.1: [...] If the `struct-declaration-list` contains no named
members, the behavior is undefined.

##### Footnote3
Note that most changes to structs are ABI-breaking changes.

<!-- xrefs -->
[Github Gist]: https://gist.github.com/andrep/f94d432ec9b207deac13a22d9b710071
[fuchsia.ui.viewsv1]: https://fuchsia.googlesource.com/fuchsia/+/06afe7e00503cf608d5df1e8be60fc0db9d3f602/public/fidl/fuchsia.ui.viewsv1/view_containers.fidl
