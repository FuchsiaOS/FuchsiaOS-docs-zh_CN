{% set rfcid = "RFC-0088" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- *** DO NOT EDIT ABOVE THIS LINE -->

## Rejection rationale

This proposal was rejected due to neither of the proposed changes being a clear
and obvious improvement over the existing state of affairs.

**See also:**

* [RFC-0050: FIDL Syntax Revamp](0050_syntax_revamp.md)
* [RFC-0086: Updates to RFC-0050: FIDL Attributes Syntax](0086_rfc_0050_attributes.md)
* [RFC-0087: Updates to RFC-0050: FIDL Method Parameter Syntax](0087_fidl_method_syntax.md)

### New separator for `bits` and `enum` layouts

This RFC was originally motivated by the observation that [RFC-0050: Syntax
Revamp][rfc-0050] states that colons are meant to separate layouts and
constraints, but that their use for demarcating wrapped types in `bits` and
`enum` declarations breaks this rule. Ultimately, this inconsistency was deemed
to be insufficiently ambiguous to justify inventing a new syntax solely to
remove the colon for just those two use cases.

None of the proposed alternatives was immediately better than the colon. Some
options that were considered:

*   Simply removing the colon was judged to be confusing - the second type
    modifies, or "templates," the `bits` or `enum` declaration. There should be
    syntax indicating that this is the case.
*   Placing the wrapped type in brackets, like `bits<uint32> {...`, was also
    rejected, as it was not consistent with the other use of angled brackets in
    the FIDL language. Angled brackets as they stand today are meant to contain
    layout parameters, as in the cases of the builtin `vector` layout being
    parameterized by `vector<TYPE>` and the builtin `array` layout being
    parameterized by `array<TYPE, SIZE>`. But putting wrapped types in brackets
    would be more akin to a template that produces a layout definition, like the
    (currently unsupported) `struct<K,V>{...}`. Mixing the meaning of angled
    brackets in this manner was judged to be too subtle and confusing.
*   Finally, simply replacing the colon with a new keyword `of` was considered.
    This too was deemed unsatisfactory, as introducing the cognitive overhead
    `of` a new keyword for FIDL authors to learn in order to accommodate two
    very minor edge cases is a bad trade. Additionally, this deviates from the
    syntax style used to date, which leans toward more of a C-like flavor, as
    opposed to an [ALGOL][algol-wiki] influenced one.

In the end, it's likely that most users will figure out fairly quickly that
these two cases are a special exception to the `layout : constraints` rule
described in [RFC-0050: Syntax Revamp][rfc-0050].

### Wrapping constraint lists in square brackets

There were two reasons for rejecting this change:

*   The language sets an expectation that parameters appear inside of angled
    brackets, regardless of what kind of parameter (layout or constraint) they
    happen to be. Using square brackets would have broken this expectation.
*   With [RFC-0086: Updates to FTP-050: FIDL Attributes Syntax][rfc-0086],
    square brackets are no longer used in FIDL syntax. It would have been
    wasteful to "burn" this bracket type (one of only four available to us:
    rounded, curly, angled, and square) to describe a syntax that is already
    sufficiently clear with angled brackets. This was simply a very small change
    whose existing state is unlikely to cause much confusion in practice.

This was simply a very small change whose existing state is unlikely to cause
much confusion in practice.

## Summary

This document proposes two minor updates to the syntax described in
[RFC-0050: Syntax Revamp][rfc-0050]. Specifically:

*   Wrapped type definitions for `bits` and `enum` layouts no longer require
    a colon to separate the layout specifier and the wrapped type. Instead, the
    newly introduced `of` keyword is used as the separator.
*   Constraint lists are now wrapped in square, instead of angled, brackets.

## Motivation

Implementation of the new syntax defined by [RFC-0050][rfc-0050] is ongoing,
which presents a good opportunity to make final updates to that standard. It is
preferable to do large syntax migrations as a single "one fell swoop" effort,
rather than sprinkling the changes across many projects ([RFC-0038: Separating
Layout from Constraints][rfc-0038] and [RFC-0039: Types Come Second][rfc-0039]
were rejected on that basis, which led to a larger overhaul culminating in
RFC-0050 for exactly this reason). It is thus desirable to roll any final syntax
tweaks into the broader RFC-0050 migration effort.

### New separator for `bits` and `enum` layouts

[RFC-0050][rfc-0050] mandates the form `layout:constraints`; that is, that FIDL
file readers should expect that everything to the left of the colon in a type
declaration affects how bytes are organized on the wire, while everything to the
right is a runtime-enforced constraint on the values that type may contain.
Currently, we have one exception to this convention: bits/enums declarations,
which do place layout affecting information to the right of the colon. This
creates inconsistency in the FIDL syntax's underlying logic.

### Wrapping constraint lists in square brackets

In the case of the second change, consider the following type declaration:

```none {:.devsite-disable-click-to-copy}
vector<vector<zx.handle:<VMO,zx.READ,optional>>>
```

This is currently difficult to parse visually. Generic type specifications and
constraint lists are fundamentally different in a number of ways (one affects
layout, the other does not; one is a required fixed size list, the other is
optional and of variable length, and so on), yet they are currently rendered
using the same syntax. This overloads the `<...>` syntax, and is difficult to
read for deeply nested definitions.

## Design

This RFC specifies two changes to the FIDL syntax.

### New separator for `bits` and `enum` layouts

Wrapped type definitions for `bits` and `enum` declarations no longer require
preceding colons. Instead, the new `of` keyword is introduced as a separator,
valid only when placed between a `bits` or `enum` keyword and its wrapped type.
Thus, a declaration previously written as:

```none {:.devsite-disable-click-to-copy}
type Foo = bits : uint32 {...
```

is now written as:

```none {:.devsite-disable-click-to-copy}
type Foo = bits of uint32 {...
```

### Wrapping constraint lists in square brackets

Constraints lists are now wrapped in square, instead of angled, brackets. Thus,
a type declaration previously written as:

```none {:.devsite-disable-click-to-copy}
vector<vector<zx.handle:<VMO,zx.READ,optional>>>
```

is now written as

```none {:.devsite-disable-click-to-copy}
vector<vector<zx.handle:[VMO,zx.READ,optional]>>
```

## Implementation

This proposal will be implemented as part of the broader [RFC-0050][rfc-0050]
FIDL syntax conversion. All FIDL files written in the "new" syntax will be
expected to conform to the changes laid out in this RFC, and the formal FIDL
grammar will be updated to reflect its design at the same time as the rest of
RFC-0050.

## Performance

These syntax changes are unlikely to have a performance impact.

## Security considerations

These syntax changes are unlikely to have a security impact.

## Privacy considerations

These syntax changes are unlikely to have a privacy impact.

## Testing

These syntax changes will be tested as part of the broader suite of tests
covering [RFC-0050][rfc-0050].

## Documentation

All relevant documentation and examples will be updated to feature the new
syntax as part of the broader [RFC-0050][rfc-0050] documentation update.

## Drawbacks, Alternatives, and Unknowns

### New separator for `bits` and `enum` layouts

An alternative formulation of the new `bits`/`enum` syntax, where the wrapped
type is encased in angled brackets, was considered: `type E = enum<int32> {...`.
It was decided that this use of the parameterized type syntax wouldn't quite
conform to its existing meaning for `vector` and `array`: `enum<int32>` is an
actual `int32` on the wire (ie, the wrapped type and the wire type are
identical), whereas the inner type for `vector<int32>` describes the payload of
some encompassing wire container (ie, the wrapped type is only a portion of the
full wire type). Further, where `vector<int32>` a valid type constructor,
neither `bits` nor `bits<int32>` are. Using the angled bracket syntax for this
case would overload its meaning in a subtle but confusing way.

### Wrapping constraint lists in square brackets

There are two alternatives to the proposed constraints syntax: leave the current
angled brackets notation, or remove brackets altogether. Both of these are
judged to be difficult for readers to parse visually:
`vector<vector<zx.handle:<VMO,zx.READ,optional>>>` contains many nested angle
brackets with different types of argument lists, while
`vector<vector<zx.handle:VMO,zx.READ,optional>>` makes it difficult to spot
where layouts stop and constraints begin.

## Prior art and references

This RFC is an evolution of the syntax defined in [RFC-0050: Syntax
Revamp][rfc-0050].

[algol-wiki]: https://en.wikipedia.org/wiki/ALGOL
[fidl-versioning]: /docs/contribute/governance/rfcs/0083_fidl_versioning.md
[rfc-0038]: /docs/contribute/governance/rfcs/0038_seperating_layout_from_constraints.md
[rfc-0039]: /docs/contribute/governance/rfcs/0039_types_come_second.md
[rfc-0050]: /docs/contribute/governance/rfcs/0050_syntax_revamp.md
[rfc-0086]: /docs/contribute/governance/rfcs/0086_rfc_0050_attributes.md

