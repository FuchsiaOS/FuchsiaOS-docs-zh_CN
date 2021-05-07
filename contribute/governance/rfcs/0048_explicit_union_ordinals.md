{% set rfcid = "RFC-0048" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-048.

## Summary

## Summary

To better align ABI implications around extensible unions (or simply unions), we
propose to:

1. **Change the syntax of variant members to require an explicit ordinal**
   (similar to how table ordinals are required).
2. Use this **explicit ordinal** rather than the previously implemented hashed
   ordinal.
3. Lastly, we change the wire format such that **union ordinals are 64 bits**
   (rather than 32 bits).

These changes **make flexible unions syntactically closer to tables**, and
**correct odd[[2]](#footnote2) and unduly strict ABI restrictions** today around
renaming unions or union members.

## Motivation and design

Aside from unions, names have no ABI impact when it comes to types: enums, bits,
struct, and tables can all be renamed, or have their members renamed without any
worries to binary compatibility. Unions are different since we chose to use a
hash based technique to assign variant ordinals (see [RFC-0061: Extensible
Unions](/docs/contribute/governance/rfcs/0061_extensible_unions.md)).

We have recognized this shortcoming, and have proposed ways to address it: see
[Intent to implement: xunion ordinals change](#footnote3), which suggests
changing the hashing scheme to only include the member name.

Instead, this proposal goes a step further and simply uses explicit ordinals,
thus avoiding any correlation between names and ABI compatibility. We consider
the extra effort to write ordinals in union declarations to be minimal: to date
explicit ordinals in tables have not been an issue, and there is a vast
precedent to numbering members or variants from other popular IDLs.

Additionally, and to further align with tables, we require that ordinals be
sequentially assigned from 1, and allow the keyword `reserved` to be used to
explicitly skip a union variant.

### Hashing Only for Protocols {#hashing-only-for-protocols}

Unlike types, protocols use a hash based approach to assign ordinals. This is
motivated by two key use cases:

* **Protocols can be composed**, and we therefore need a global ordinal
  assignment scheme to avoid breakage at a distance issues. See [RFC-0063:
  [OrdinalRange]](/docs/contribute/governance/rfcs/0063_OrdinalRange.md), [RFC-0020: Interface Ordinal Hashing](/docs/contribute/governance/rfcs/0020_interface_ordinal_hashing.md),
  and [RFC-0029: Increasing Method Ordinals](/docs/contribute/governance/rfcs/0029_increasing_method_ordinals.md).
* Practically **globally unique identifiers** greatly simplify and bolster
  monitoring and tracing like needs, e.g. fidlcat hinges on the ability to
  uniquely identify method invocations.

These use cases do not translate to types.

The effect of this proposal is that hashing is used only for protocols. There is
only one hashing scheme, i.e. the one described in [RFC-0029](/docs/contribute/governance/rfcs/0029_increasing_method_ordinals.md).

### 64 bit ordinals is the standard

Right now, there are 4 bytes of padding in a union inline content:

* Ordinal (`uint32`)
* Padding (`uint32`)
* [Envelope][wire-format-envelopes] (16 bytes)

Instead, we want all bytes to be called for explicitly in the format and therefore change the ordinal to 64b. Generally, we prefer padding-less structures since they are more efficient (e.g. do not require explicit memsets or extra coding tables).

See the [Implementation Strategy](#implementation-strategy) section for how to soft-transition to 64-bit ordinals.

### JSON Wire Format

It's been discussed in the past that when we create a JSON wire format, renaming
of types and members will pose ABI implications in that format.

It's useful to separate ABI breakage on a per 'wire format' basis. We can get
different properties from different ones. The rare messages that need to be ABI
compatible on all possible wire formats supported will be very limited in how
they can evolve. Others should benefit from more flexible rules where possible.

### Looking ahead: sparse tables

There's been discussion of supporting sparse tables, i.e. a third layout to
record-like data in addition to [structs][wire-format-structs] and
[tables][wire-format-tables]. Should we decide to introduce this third option,
the strawman syntax would follow this proposal, and the current tables' syntax:

```fidl
sparse_table Example {
    1: T1 field1;
    2: reserved; // deprecated
    3: T3 field3;
};
```

## Implementation strategy

To enable a soft transition, we need to disambiguate between the classic (32-bit
hashed) ordinal syntax and the proposed (64-bit explicit) ordinal syntax. This
can be done by:

1. Adding a check in fidlc to ensure that the value of a 32-bit hashed ordinal
   is never lower than N. For example, if N is 512, a hashed ordinal's hex value
   MUST be at least 0x200.
  * A hashed ordinal that is < 0x200 will result in a compile error, and the
    field name must be manually renamed with the [Selector=] attribute. We would
    add [Selector] to the appropriate fields before landing this change in
    fidlc.
  * Given the randomness of the existing hashing scheme, we expect a hashing
    error to occur near-zero times, so manual resolution will likely not be
    necessary.
  * Adding this check effectively allocates the [0..N] ordinal space for
    explicit ordinals, and ensures it will not clash with hashed ordinals.
2. When language bindings interpret the ordinal value:
  * if the ordinal is between [0, N), the ordinal is 64-bit & explicit.
  * if the ordinal is between [N, UINT32_MAX], the ordinal is 32-bit and hashed.
  * if the ordinal is between [UINT32_MAX, UINT64_MAX), bindings MUST evoke an
    error and close the channel with epitaph.

## Ergonomics

Makes ABI ergonomics much simpler, at the very minimal syntactic cost of having explicit ordinals.

## Documentation and examples

At least:

* [Language Specification][language-spec]
* [Grammar][grammar]
* Examples using unions
* ABI Compatibility Guide

## Backwards compatibility

Unions defined without the explicit ordinal syntax will continue to use the
existing 32-bit, hashed-ordinal scheme. So, a union that exists today will
continue to be API- and ABI-compatible.

Unions defined with explicit ordinal syntax will use the 64-bit ordinal scheme
described in this RFC. See the [Implementation
Strategy](#implementation-strategy) section for how to support both the 32-bit
and 64-bit ordinal schemes.

## Performance

Extremely minor improvement: this scheme is more efficient than hashed ordinals due to better codegen for switch() statements.

## Security

No impact.

### Testing

Trivial, as usual.

## Drawbacks, alternatives, and unknowns

### Alternative: Ordinal Hashing on Member Names Only

See [Intent to implement: xunion ordinals change](#footnote3).

After further thought, we do not consider the above to enough since the
syntactical advantage (no ordinals in source) does not compensate:

* Two hashing schemes, which make understanding ABI implications harder;
* Keeps unions distinct from its siblings enums, bits, structs, and tables when
  it comes to renaming declarations or members.

## Prior art and references

None that is particularly relevant.

--------------------------------------------------------------------------------

##### Footnote1

"Union" in this document refers to [extensible unions](/docs/contribute/governance/rfcs/0061_extensible_unions.md), not "static"
unions nearing their end-of-life.

##### Footnote2

Odd in the sense that names have no impact on the binary wire format of messages
(i.e. bits, enum, struct, table), except for unions. It's therefore a case that
stands out as being different than others.

##### Footnote3

_From: apang@google.com<br />
To: fidl users list<br />
Date: 5/23/2019_

Hi FIDLers! When writing a test yesterday, the FIDL team saw a surprising
behavior with the current xunion spec & impl. If one declares this:

```fidl
xunion MyXUnion {
    int32 i;  // ordinal might be 0x11111111
}
````

and renames the name of the xunion (not the field), the ordinal of the field
changes:

```fidl
// rename from MyXUnion to MyXUnion2
xunion MyXUnion2 {
    int32 i;  // ordinal now changes to 0x22222222 since the xunion was renamed. d'oh!
}
```

This is arguably unexpected behavior: changing the name of the xunion shouldn't
change the ABI.

Making this better means two things:

1. We'd like to amend the xunion RFC (RFC-0061) so that the ordinal is derived
   from the field name only, removing the xunion name & library name from the
   ordinal hash calculation.
2. We need to change the code, which unfortunately means the xunion ABI changes
   and could lead to bad builds. Thankfully, we can make this a soft transition
   through the technique that Jeremy Manson pioneered for implementing ordinal
   hashing for methods: have clients check for both the old & new hashes until
   the changes fully roll through the tree.

(One lesson learnt: in the future, we should look carefully at what's included
in an ordinal hash, and whether changing those things should change the ABI.)

We believe this is a low-risk plan given that it's possible to do a
soft-transition, and Jeremy's successfully done it for method ordinals. Please
chime in with comments, otherwise we'll get started on this work soon.

<!-- xrefs -->
[wire-format-structs]: /docs/reference/fidl/language/wire-format#structures
[wire-format-tables]: /docs/reference/fidl/language/wire-format#tables
[wire-format-envelopes]: /docs/reference/fidl/language/wire-format#envelopes
[language-spec]: /docs/reference/fidl/language/language.md
[grammar]: /docs/reference/fidl/language/grammar.md
