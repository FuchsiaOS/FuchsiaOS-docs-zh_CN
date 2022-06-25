{% set rfcid = "RFC-0052" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-052.

_"Call me by another name"_

## Summary

Portions of
[RFC-0019](contribute/governance/rfcs/0019_using_evolution_uint64.md)
introduced the notion of type aliasing into FIDL.
This proposal aims to formally evolve FIDL's type aliasing and new type
declaration mechanisms and their effects on bindings. More specifically, this
proposes refinements to the type alias grammar, exposition of type aliases to
bindings, and a new feature known as new types.

## Motivation

Type aliasing is already a feature in FIDL today, and this RFC aims to expand
its scope and usefulness. With type aliasing, declarations will have an impact
in bindings and thus will be helpful for API name transitions.

In addition to type aliasing, this RFC also introduces a sister concept -- new
type declarations. New types, much like aliases, declare a new name in the local
scope that wraps another type, but are considered distinct to the type checker
(named type semantics). This would allow FIDL authors to better leverage the
type systems of their binding languages.

## Design

### Type alias

For full context on type aliasing,
[RFC-0019](contribute/governance/rfcs/0019_using_evolution_uint64.md)
(which proposes the current using style aliases) highlights the original design,
motivation, and precedence. This RFC aims to pave a path forward for exposing
type aliases in language bindings and refine some of the language grammar
decisions in RFC-0019. Much like using aliases, the new type aliases are purely
nominal and will not affect the ABI. This section will primarily outline the
changes and improvements to the original feature.

#### Grammar

```ebnf
alias-declaration = ( attribute-list ) , "alias" , IDENTIFIER , "=" , type-constructor;
```

`type-constructor` should resolve to a fully-formed type reference that exists
in the current scope. In other words, for a type like `vector`, it must contain
both the inner type and size constraint.

For example, this would be valid:

```fidl
alias SmallBytes = vector<uint8>:SMALL_NUM;
```

But this would not (partial type reference on the right hand side of the equals):

```fidl
alias SmallVec = vector:SMALL_NUM;
```

While FIDL today supports partial type references (illustrated by the `SmallVec`
example) with a `using` alias, this feature will be removed. The reasoning of
this deprecation is threefold:

1. Type generics have not been fully defined or reviewed for FIDL. In a future
where type generics exist, the language would have a better and more formal way
to describe types and type aliases that can be parameterized.
2. The FIDL language has not undergone a formal decision on how generics would
translate to languages that don't support generics (i.e. Go [[1]](#Footnote1), C).
3. The current syntax creates scenarios where type parameters may be
*implicitly* required to create a fully-formed type. Furthermore, the specifics
of how to parse and parameterize a nested partial type alias (i.e.
`using SmallVecOfVecs = vector<vector:SMALL_NUM>:SMALL_NUM`)  are unclear.

Aliases of `protocol`s will not be supported, as a `protocol` cannot be fully
classified as a type, either in FIDL or binding languages. The goal is to
revisit this decision in the future, if or when protocols become more type-like
(i.e. `client_end:ProtocolName` and `server_end:ProtocolName`).

#### Examples

| Language               | Code |
|------------------------|------|
| FIDL                   | `alias SmallBytes = vector<uint8>:SMALL_NUM;` |
| C                      | (vectors do not exist in simple C bindings)<br>`typedef small_bytes uint8_t*;` |
| C++                    | `using SmallBytes = std::vector<uint8_t>;` |
| Rust                   | `type SmallBytes = Vec<u8>;` |
| Dart [[2]](#Footnote2) | `typedef SmallBytes = List<int>;` |
| Go                     | `type SmallBytes = []uint8;` |

### New Type

A new type is a type that wraps an underlying type but is considered distinct
from the underlying type in a language's type system. This may be useful if two
values possess the same type size and shape (and possibly characteristics) but
have different *semantic* meaning. For example, a `zx_vaddr_t` and a
`zx_paddr_t` are represented by the same underlying `uintptr_t` type but have
different (yet easily confusable) meanings.

A new type allows for these semantic differences to be expressed in the type
system. In strongly-typed languages, this means that logic bugs can be caught
via type-checking and/or static analysis at compile-time.

This comes at no change or cost in the wire format.

#### Grammar

```ebnf
newtype-declaration = ( attribute-list ), "type", IDENTIFIER, "=", type-constructor;
```

This syntax was chosen to align with the introduction of top-level types as
specified in RFC-0050: Syntax Revamp.

Because a new type should translate to a concrete type in the bindings,
`type-constructor` should resolve to a fully-formed type reference. This may
look like:

```fidl
type MyBytes = vector<uint8>:MAX;
```

New types of `protocol`s will not be supported as a `protocol` cannot be fully
classified as a type, either in FIDL or binding languages. Furthermore, there is
currently no compelling use case for named type semantics on `protocol`-derived
entities in generated bindings.

#### Type Traits and Operators

Without new types, the effects of a new type can roughly be accomplished with a
new single-field `struct` type in FIDL. For example:

```fidl
struct MyBytes {
    vector<uint8>:MAX value;
};
```

However, new types indicate to backends and bindings a narrower semantic space
and allow them to generate language-specific features that may map to a native
language feature or make the new type ergonomic to use. For example:

| Language | Explanation |
|----------|-------------|
| C        | Unfortunately, C's type system has no good representation. Fallback to `typedef`.|
| C++      | The new type can be translated to a `class` that may expose the functionality of the following from the underlying type: (1) explicit conversion constructor, (2) explicit assignment operator, (3) arithmetic & bitwise operations, and (4) misc operators (`[]`, `*`, `->`, etc.) |
| Rust     | The new type can be translated to a singleton struct that derives traits such as `From<UnderlyingType>`, `Into<UnderlyingType>`, `Hash`, `PartialEq`, `Copy`, `Clone`, `std::ops::*`, etc. that the underlying type implements. |
| Dart     | From conversations with the Dart team, there is currently no way to map new type semantics to the language, though there have been discussions of supporting such a use case [[3]](#Footnote3).<br>A new type can be downgraded to a type alias, pending #65 [[2]](#Footnote2). |
| Go       | A new type maps directly to a type definition.<br>`type <NewType> <UnderlyingType>` |

The set of innate functionality of the underlying type are henceforth referred
to as *type traits* (a concept that is not yet formally defined in the
language).

Unless/until type traits are defined in FIDL, the new type shall inherit the
underlying type's traits by default. By doing so, new types will still be useful
in the same ways the underlying type would be, without needing to unwrap the
underlying type and potentially undoing the type safety benefits of a new type.
It may be useful in the future to be able to define custom trait behavior of a
new type and the design of this does not prevent an evolution towards such a
path in the future. However, trait inheritance by default does make the
migration path towards opt-in inheritance of traits a large, breaking change.

##### Examples

C++

```cpp
// This example uses C++20 concepts for readability but this can be translated to a
// template approach in C++14.
template<typename T>
concept Addable = requires(T a) {
  { a + a } -> T;
};

template<typename T>
concept Multipliable = requires(T a) {
  { a * a } -> T;
};

template <typename T>
concept Copyable = std::is_copy_constructible_v<T> || std::is_copy_assignable_v<T>;

template <typename T>
concept Movable = std::is_move_constructible_v<T> || std::is_move_assignable_v<T>;

class MyNumber {
 public:
  using UnderlyingType = uint32_t;

  explicit MyNumber() = default;

  explicit MyNumber(const UnderlyingType& value)
      requires Copyable<UnderlyingType>
    : value_(value) {}

  explicit MyNumber(UnderlyingType&& value) requires Movable<UnderlyingType>
    : value_(std::move(value)) {}

  MyNumber& operator=(const MyNumber&) = default;
  MyNumber& operator=(MyNumber&&) = default;

  [[nodiscard]] MyNumber operator+(const MyNumber& other) const
      requires Addable<UnderlyingType> && Copyable<UnderlyingType> {
    return MyNumber(value_ + other.value_);
  }

  [[nodiscard]] MyNumber operator+(MyNumber&& other)
      requires Addable<UnderlyingType> && Movable<UnderlyingType> {
    return MyNumber(value_ + other.value_);
  }

  [[nodiscard]] MyNumber operator*(const MyNumber& other) const
      requires Multipliable<UnderlyingType> && Copyable<UnderlyingType> {
    return MyNumber(value_ * other.value_);
  }

  [[nodiscard]] MyNumber operator*(MyNumber&& other)
      requires Multipliable<UnderlyingType> {
    return MyNumber(value_ + other.value_);
  }

  // ...other operators defined here...

  [[nodiscard]] UnderlyingType value() const
      requires Copyable<UnderlyingType> {
    return value_;
  }

  UnderlyingType take_value() requires Movable<UnderlyingType> {
    return std::move(value_);
  }

 private:
  UnderlyingType value_;
};
```
Rust

```rust
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct MyNumber(u32);

impl From<u32> for MyNumber {
    fn from(value: u32) -> Self {
        MyNumber(value)
    }
}

impl Into<u32> for MyNumber {
    fn into(self) -> u32 {
        self.0
    }
}

impl Add for MyNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Mul for MyNumber {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

// ...implement other traits here...
```

------------------------------------------------------------------------------------------

##### Footnote1

Go [will have generics](https://blog.golang.org/generics-next-step) in the near future.

##### Footnote2

Pending Dart's introduction of
[typedefs for non-function types (#65)](https://github.com/dart-lang/language/issues/65).

##### Footnote3

See comment at Dart's issue
[#42](https://github.com/dart-lang/language/issues/42#issuecomment-502966868).
