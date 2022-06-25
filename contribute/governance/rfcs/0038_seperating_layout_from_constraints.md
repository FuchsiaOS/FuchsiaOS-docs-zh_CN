{% set rfcid = "RFC-0038" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-038.

## Rejection rationale

When this proposal was drafted, and socialized, there was a strong consensus to
consider syntax changes all at once, rather than one at a time (see also
[RFC-0039](contribute/governance/rfcs/0039_types_come_second.md)). We also
wanted one person to be the syntax arbiter, rather than risk designing by
committee.

Eventually, this proposal was obsoleted by
[RFC-0050](contribute/governance/rfcs/0050_syntax_revamp.md) which met
both conditions sought.

## Summary

We propose a syntax change to convey differences between layout from
constraints.

## Motivation

#### Layout vs constraint

Quickly:

* If two types have a different layout, it is not possible to soft transition
  from one to the other, and vice versa (not easily at least).
* The layout describes how the bytes are laid out, vs how they are interpreted.
* The constraint of a type is the validation step done during encoding/decoding.
* Constraints can evolve, and as long as writers are more constrained than
  readers, things are compatible.

#### Same syntax for different things

Types which have different layout, and types which have the same layout (but
different constraints) look alike.

Same layout:

* `vector<T>:6`, or `vector<T>:10`
* `T?` where `T` is a `union`, `vector`, or `string`
* `handle`, `handle<vmo>`, or `handle<channel>`

Different layout:

* `array<T>:6` vs `array<T>:10`
* `S?` where `S` is a `struct`

## Design

Align on the syntax

    layout:constraint

For types, i.e. anything that controls layout is before the colon, anything that
controls constraint is after the colon.

Suggested changes:

```
    array<T>:N    becomes            array<T, N>

    S?            becomes            box<S>:nullable       (S is a struct)

    T?            becomes            T:nullable            (T is a vector, or union)

    string        is an alias for    vector<uint8>:utf8

    handle<K>     becomes            handle:K
```

Notes:

* Not all constraints are meaningful for all types, for instance it is not
  possible to mark a `struct` nullable.
* Not everything can be boxed, initially only structs can (the goal is to change
  the syntax, not introduce more ways to have optionality).

## Ergonomics

This proposal improves ergonomics by conveying ABI implications to developers
through syntax.

## Documentation and Examples

At least:

* [Language Specification](reference/fidl/language/language.md)
* [Grammar](reference/fidl/language/grammar.md)
* Examples using structs

## Backwards Compatibility

This is not source level backwards compatible.

--------------------------------------------------------------------------------

Note: This RFC was rejected early during its socialization phase, which explains
the multiple missing sections (e.g. "Implementation strategy", or "Drawbacks,
alternatives, and unknowns").
