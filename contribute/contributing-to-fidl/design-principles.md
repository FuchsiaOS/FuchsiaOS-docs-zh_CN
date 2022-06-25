# FIDL design principles

This page summarizes the key design principles that FIDL has adopted over time.

## Priority of constituencies

FIDL aims to respect the following priority of constituencies:

1. Users (using a Fuchsia product)
2. End-developers (using FIDL bindings)
3. Fuchsia contributors (using FIDL bindings)
4. API designers (authoring FIDL libraries)
5. [Fuchsia FIDL Team] members

This list was adapted from that of the [API Council Charter].

## ABI first, API second

From [RFC-0050: Syntax revamp][rfc-0050-principles]:

> FIDL is primarily concerned with defining Application Binary Interface (ABI)
> concerns, and second with Application Programming Interface (API) concerns.

## Binary wire format first {#binary-wire-format-first}

From [RFC-0050: Syntax revamp][rfc-0050-binary-wire-format-first]:

> While many formats can represent FIDL messages, the [FIDL Wire
> Format][wire-format] (or "FIDL Binary Wire Format") is the one which has
> preferential treatment, and is catered to first ... we choose to over rotate
> towards the binary ABI format when making syntax choices.

## Low level first

From [RFC-0131: Design principles of the FIDL wire format][rfc-0131]:

> When faced with making a design tradeoff to support low level programming at
> the expense of high level programming (or the reverse), we typically opt for
> enabling low level programming.

## Fewest features {#fewest-features}

From [RFC-0050: Syntax revamp][rfc-0050-fewest-features]:

> We strive to have the fewest features and rules, and aim to combine features
> to achieve use cases. In practice, when considering new features, we should
> first try to adapt or generalize other existing features rather than introduce
> new features.

## You only pay for what you use {#you-only-pay}

From [RFC-0027: You only pay for what you use][rfc-0027]:

> When adding functionality to FIDL, we should evaluate the costs that adding
> that functionality imposes on people who use FIDL but do not use the new
> functionality. We should then have a very high bar for accepting functionality
> that imposes costs on people who do not use the functionality.

For example, [RFC-0047: Tables][rfc-0047-motivation] followed this principle by
adding tables to the language rather than replacing structs:

> Tables are necessarily more complicated than structs, and so processing them
> will be slower and serializing them will use more space. As such, it's
> preferred to keep structs as is and introduce something new.

In contrast, [RFC-0061: Extensible unions][rfc-0061-pros-and-cons] reached the
opposite decision of replacing static unions with extensible unions, but only
after a careful analysis of the tradeoffs. Unlike with tables, the extra cost
imposed by extensible unions is marginal or nonexistent in most cases.

## Solve real problems {#solve-real-problems}

We design FIDL to solve real problems and address real needs, not imagined ones.
We avoid designing a "solution in search of a problem".

For example, FIDL initially did not support empty structs because it was unclear
how to represent them in C/C++. In [RFC-0056: Empty structs][rfc-0056], we saw
users were employing workarounds and recognized the need for an official
solution. Only then did we add empty structs to the language.

## Optimize based on data

Optimizing without data is useless at best and dangerous at worst. When
designing optimizations (e.g. performance, binary size), we follow the data.

For example, [RFC-0032: Efficient envelopes][rfc-0032] was initially accepted,
but later rejected. In hindsight, at the time it should not have been accepted
because there was no data to back it up. Later, it was re-proposed and accepted
as [RFC-0113: Efficient envelopes][rfc-0113] after there was data demonstrating
a significant performance improvement.

Similarly, there was significant momentum behind using a sparser representation
of table data. However, after investigation, there was an unfavorable tradeoff
between design complexity and performance that led to a decision to not move
forward (see [RFC-0116: Sparser Tables][rfc-0116]). Without a significant
prototyping and data-gathering phase, sparser tables would likely have been
adopted and negatively impacted Fuchsia.

## No breakage at a distance

We strive to avoid _breakage at a distance_. Changes in one place should not
cause surprising breakage in a faraway place. For example, it would be
surprising if adding a struct named `Foo` to a FIDL file broke compilation
because another FIDL file in a completely different part of the codebase already
had a type named `Foo`. This is why FIDL, like most programming languages, uses
namespaces to limit the scope of name collisions.

[RFC-0029: Increasing method ordinals][rfc-0029-breakage-at-a-distance]
discusses this problem as it relates to protocol composition. [RFC-0048:
Explicit union ordinals][rfc-0048-hashing-only-for-protocols] revisits the
topic, explaining why FIDL only uses hashing for protocols.

[RFC-0057: Default no handles][rfc-0057] introduced a distinction between [value
and resource types][lang-resource]. One motivation for this was providing the
`Clone` trait in Rust for types without handles without breakage at a distance:

> Although FIDL bindings _can_ conditionally enable code based on the presence
> of handles, doing so is undesirable because it breaks evolvability guarantees.
> For example, adding a field to a table is normally safe, but adding a handle
> field would become source-breaking &mdash; not only for that table, but for
> all types transitively containing it.

[RFC-0149: FIDL encode validation not mandatory][rfc-0149] also touches upon
this topic. It breaks down the classes of breakages that can occur and moves
the decision of whether to perform encode-side validation to bindings. This
allows there to be a more fine-grained discussion of the pros and cons of
overhead from encode-side validation vs the risk of breakage at a distance.

## Liberal syntax, idiomatic style

We do not rigidly adhere to a "one way to do it" philosophy. When we are
concerned that users will waste time deciding between trivial alternatives, we
introduce restrictions in `fidl-lint` or `fidl-format` rather than in `fidlc`.

<!-- TODO(fxbug.dev/74753): Say "the linter enforces an ordering" when it is true. -->
For example, FIDL accepts modifier keywords in any order, but we intend to
enforce a consistent ordering in the linter.

As another example, [RFC-0040: Identifier uniqueness][rfc-0040] fixed the
problem of identifiers colliding after case transformation by having `fidlc`
report an error if any two identifiers have the same canonical form. A simpler
alternative would have been to enforce FIDL naming conventions in the compiler.
However, this goes a step too far. There are valid reasons for using different
naming styles, for example in describing the Kernel API, where `snake_case`
methods are strongly preferred.

## Canonical representation

From [RFC-0131: Design principles of the FIDL wire format][rfc-0131]:

> There must be a single unambiguous representation of a FIDL value, i.e. there
> is one and only one encoded representation of a FIDL value, and one and only
> one decoded representation of a FIDL value.

The FIDL wire format [is canonical][wire-format-dual-forms]: there is exactly
one encoding for a given message. As a corollary, every byte is accounted for:
there is no byte that can be changed without altering the message's meaning.

For example, the [specification][wire-format-padding] requires that all padding
bytes are zero. Similarly, [RFC-0047: Tables][rfc-0047-wire-format] disallows
storing extraneous empty envelopes to ensure a canonical representation.

A canonical representation makes FIDL simpler and more secure. For example,
allowing nonzero padding could result in FIDL messages leaking sensitive
information that happened to occupy those bytes in memory. Allowing multiple
representations for a given message also leads to rarely executed code paths
that can hide bugs, e.g. the "extra empty envelopes" code path. A canonical
representation also makes it easy to compare messages for equality without
knowing the schema: for [value types][lang-resource], it is a simple `memcmp`.

## No allocations required

From [RFC-0131: Design principles of the FIDL wire format][rfc-0131]:

> It must be possible to encode and decode in a single pass, without allocation
> beyond stack space (i.e. no dynamic heap allocation).

This requirement significantly influences the design of the wire format: you
must be able to decode in place using only the stack. It is the reason the wire
format uses presence indicators and a depth-first traversal order rather than,
for example, and offset-based format that requires auxiliary data structures to
keep track of information while decoding.

This principle is related to ["You only pay for what you use"](#you-only-pay),
in that it caters to very low-level uses of FIDL where `malloc` may not yet
exist, or is prohibitively expensive.

## No reflective functionality out of the box

From [RFC-0131: Design principles of the FIDL wire format][rfc-0131]:

> Without explicit opt-in, a peer must not be allowed to perform reflection on a
> protocol, be it exposed methods, or exposed types.

## Transport generality

While [the binary wire format comes first](#binary-wire-format-first), this does
not mean FIDL should be tightly coupled to the Zircon channel transport. There
are other important use cases to consider, such as describing the Kernel API,
in-process messaging, and persistence.

[RFC-0050: Syntax revamp][rfc-0050-transport-generalization] describes the
future direction for transport generalization.

[RFC-0062: Method impossible][rfc-0062] was rejected in part because it coupled
FIDL too closely to the Zircon channel transport.

<!-- link labels -->
[API Council Charter]: contribute/governance/api_council.md#values
[Fuchsia FIDL Team]: /src/fidl/OWNERS
[lang-resource]: reference/fidl/language/language.md#value-vs-resource
[wire-format]: reference/fidl/language/wire-format
[wire-format-dual-forms]: reference/fidl/language/wire-format#dual-forms
[wire-format-padding]: reference/fidl/language/wire-format#padding
[rfc-0027]: contribute/governance/rfcs/0027_you_only_pay_what_you_use.md
[rfc-0029-breakage-at-a-distance]: contribute/governance/rfcs/0029_increasing_method_ordinals.md#breakage-at-a-distance
[rfc-0029]: contribute/governance/rfcs/0029_increasing_method_ordinals.md
[rfc-0032]: contribute/governance/rfcs/0032_efficient_envelopes.md
[rfc-0040]: contribute/governance/rfcs/0040_identifier_uniqueness.md
[rfc-0047-motivation]: contribute/governance/rfcs/0047_tables.md#motivation
[rfc-0047-wire-format]: contribute/governance/rfcs/0047_tables.md#wire-format
[rfc-0048-hashing-only-for-protocols]: contribute/governance/rfcs/0048_explicit_union_ordinals.md#hashing-only-for-protocols
[rfc-0050-binary-wire-format-first]: contribute/governance/rfcs/0050_syntax_revamp.md#binary-wire-format-first
[rfc-0050-fewest-features]: contribute/governance/rfcs/0050_syntax_revamp.md#fewest-features
[rfc-0050-principles]: contribute/governance/rfcs/0050_syntax_revamp.md#principles
[rfc-0050-transport-generalization]: contribute/governance/rfcs/0050_syntax_revamp.md#transport-generalization
[rfc-0056]: contribute/governance/rfcs/0056_empty_structs.md
[rfc-0057]: contribute/governance/rfcs/0057_default_no_handles.md
[rfc-0061-pros-and-cons]: contribute/governance/rfcs/0061_extensible_unions.md#pros-and-cons
[rfc-0062]: contribute/governance/rfcs/0062_method_impossible.md
[rfc-0113]: contribute/governance/rfcs/0113_efficient_envelopes.md
[rfc-0116]: contribute/governance/rfcs/0116_fidl_sparser_tables.md
[rfc-0131]: contribute/governance/rfcs/0131_fidl_wire_format_principles.md
[rfc-0149]: contribute/governance/rfcs/0149_fidl_encode_validation_not_mandatory.md
