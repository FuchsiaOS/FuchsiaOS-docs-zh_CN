{% set rfcid = "RFC-0025" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-025.

_"Just a Little Bit"_

## Summary

Extend the FIDL language with bit flag declarations.

## Motivation

There are several use cases for describing a set of flags over an integer in
FIDL.
Currently, users of FIDL are advised to make a set of constants of the same
underlying type.
Because these are all independent, creation of invalid values cannot be
detected by the bindings at runtime.

## Design

### Source language Changes

This proposal adds the `bits` keyword to FIDL.

`bits` introduce a top-level declaration, similar to `enum`s.
Formally, the productions in the grammar are as follows:

```
bits-declaration = ( attribute-list ) , "bits" , IDENTIFIER , ( ":" , type-constructor ) ,
                   "{" , ( bits-or-enum-member , ";" )+ , "}" ; [NOTE 1]

bits-or-enum-member = ( attribute-list ) , IDENTIFIER , ( "=" , bits-or-enum-member-value ) ;

bits-or-enum-member-value = IDENTIFIER | literal ; [NOTE 2]
```

Notes:

1. The `bits-declaration` allows the more liberal `type-constructor` in the grammar, but
   the compiler limits this to unsigned integer types, see [primitives].

2. The `bits-or-enum-member-value` allows the more liberal `literal` in the grammar, but the compiler limits this to:
    * A `NUMERIC-LITERAL` in the context of an `enum`;
    * A `NUMERIC-LITERAL`, which must be a power of two, in the context of a `bits`.

Each member in a `bits` declaration is a power of two.
This proposal suggests not allowing more complicated expressions in the `bits`
declaration itself, nor allowing them to be ORed together in `bits` constant
expressions, for the sake of simplicity.
They could be added to `bits` declarations in the future.

An example of a `bits` declaration, taken from constants currently in the
fuchsia.io library:

```fidl
bits OpenRights : uint32 {
    READABLE = 0x00000001;
    WRITABLE = 0x00000002;
    ADMIN = 0x00000004;
};
```

Furthermore, this proposal adds a binary literal syntax like so:

```fidl
bits OpenRights : uint32 {
    READABLE = 0b0001;
    WRITABLE = 0b0010;
    ADMIN = 0b0100;
};
```

### Semantics

Overflowing the underlying integer type is a compilation error.

Each `bits` member value must be distinct.

Serializing or deserializing a `bits` value with a bit set that is not a
member of the `bits` declaration is a validation error.

The semantics of `bits` are distinct from `enum`.
An `enum` value must be exactly one of the values declared in FIDL, while
`bits` may not.
For instance, if `OpenRights` were an `enum`, the only valid values to send
would be `1`, `2`, and `4`.
As a `bits` type, though, `0`, `3`, `5`, `6`, and `7` are all also valid.

### Bindings

Each language binding will be extended to handle these values idiomatically.
At worst, this simply generates a constant for each `bits` member as though it
were an integral constant of the underlying type.

The wire format for a `bits` value is the same as the underlying integral value.

Serializing and deserializing code should ensure that the value is a subset of
the described bits.
For instance, attempting to use 8 as an `OpenRights` value should fail validation.

### Signal and Rights Constants

I also propose adding signal and handle right values to the `zx` library.
This includes a `bits` declaration of all the signal value and rights, and
possibly a set of constants with default rights for each handle type.

## Implementation strategy

### Phase 1

Add all of the source changes to the FIDL compiler, including parser tests.

### Phase 2

Add support to all language bindings, and to the compatibility test suite.

### Phase 3

Migrate existing pile-of-int-constants to `bits`.

## Ergonomics

This change makes FIDL more ergonomic, by letting users explicitly express
their intention, and readers see the explicit grouping.

## Documentation and examples

This proposal describes changes to the FIDL grammar above.

I would tweak the FIDL tutorial to include an example of this pattern.

## Backwards compatibility

This is a backwards compatible change to FIDL source.

The wire format is backwards compatible, in the sense that the value of `(2 |
4)`, or `6`, is the same on the wire whether sent as a `uint32` or a `bits Bits :
uint32`.

## Performance

Changing a type in FIDL from a `uint32` to a `bits` value adds some
minor overhead in the bindings of checking that the serialized or deserialized
value is valid.

This is a bitmask and a branch, and unlikely to be noticable.

## Security

I do not see a security downside.
The better type safety is perhaps a minor upside.

## Testing

`fidlc` host unit tests will exercise the FIDL parser.

The compatibility test suite will be extended with `bits` types of various sizes
and values to exercise the sending and receiving paths of all supported
bindings.

## Drawbacks, alternatives, and unknowns

This proposal suggests only allowing bits for unsigned integer types.
I believe it would be possible to allow it for signed underlying types, but
with more care than desirable necessary in all of the language bindings.
I'd rather not have us accidentally shifting bits too far in C/C++ in
particular.

More general bitfield patterns seem more complicated than worthwhile for this
proposal.
By this I mean carving an integer type up into ranges of several bits, and
giving each range of bits a name as a field.

`&` and `~` expressions feel unnecessary, at least at first.
Target languages could optionally support such arithmetic expressions on bit
flag values, but I do not yet see a need for them in FIDL constants directly.

## Prior art and references

The hesitancy to do anything too complicated with member values, and to avoid
signed types, is based on eternal confusion with those concepts in C and C++,
whose bindings must support this concept.

<!-- xrefs -->
[primitives]: /reference/fidl/language/language.md#primitives
