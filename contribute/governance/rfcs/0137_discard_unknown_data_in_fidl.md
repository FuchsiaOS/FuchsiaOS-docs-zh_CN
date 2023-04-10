<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0137" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

Most FIDL bindings preserve unknown table fields and union variants, allowing
user code to inspect and re-encode the raw bytes and handles. This behavior
poses security and privacy risks, adds significant complexity to FIDL, makes
wire format migrations difficult, and cannot be implemented in all bindings. We
propose to have bindings discard unknown data instead, resulting in the behavior
shown in [Table 1](#table-1).

**Table 1: Changes for [flexible][lang-flexible] types with unknown data**
{:#table-1}

| Type  | Can access unknowns?   | Can re-encode? | Proxies unknowns? |
|-------|------------------------|----------------|-------------------|
| bits  | Yes                    | Yes            | Yes               |
| enum  | Yes                    | Yes            | Yes               |
| table | Yes → **No**           | Yes            | Yes → **No**      |
| union | Yes → **Ordinal only** | Yes → **No**   | Yes → **No**      |

## Background

Flexible types are an important feature in FIDL for writing evolvable APIs.
Introduced in [RFC-0033: Handling of unknown fields and strictness][rfc-0033],
they have been available in all bindings since late 2020. With flexible types,
decoding succeeds even if there are unknown members. FIDL tables are always
flexible, whereas bits, enums, and unions can be marked strict or flexible.
With flexible bits and enums, an unknown value is simply an integer. However,
for an unknown table field or union variant, the value consists of raw bytes
and handles, which we refer to as **unknown data**.

Most bindings today **preserve** unknown data in [domain objects]. The exception
is LLCPP, whose design constraints make this difficult to support. For the
others, preserving unknown data enables the following behavior. Suppose
processes A, B, and C communicate over FIDL. If A and C know about a new table
field but B does not, and that field is sent from A to B to C, then C will
receive and understand it despite B's ignorance of it. In other words, B
**proxies** the unknown data. An application could also interpret unknown data
based on assumptions about the schema, for example that the first four bytes are
always an identifier. However, such cases are contrived and better modeled
directly with FIDL types. Proxying is the only realistic use case for preserving
unknown data.

## Motivation {#motivation}

In designing FIDL, we strive to [solve real problems] using the [fewest
features] necessary. The feature of preserving unknown data has not lived up to
these principles. Since its implementation, it has seen little or no use in
Fuchsia, and it has repeatedly come up as a [complicating
factor][rfc-0114-alternative-8-byte] in other FIDL efforts.

This leads us to question the merits of proxying unknown data. Is it a good idea
in the first place? We contend that it is not, at least not as a default
behavior. It might be useful as an opt-in feature for FIDL servers that are
truly meant to be proxies. However, these cases would be better served by
dedicated proxying support in FIDL, which would address all aspects of the
problem, not just unknown data.

Even if we suppose that proxying by default is desirable, it only works when
directly re-encoding a FIDL domain object. However, it is common (and
recommended) to convert these objects to richer, application-specific types
before further processing. This practice is counter to how one would approach
proxying, where it is desirable to either pass encoded messages unaltered, or
directly re-encode decoded messages with minimal processing. For example, the
Rust crate [`fidl_table_validation`] provides validation while converting a
FIDL domain object into an application domain object. Thus a peer sending a
table across multiple hops in a complex system cannot rely on all fields
reaching the final destination if any participants use this pattern.

Whether or not proxying is desirable, preserving unknown data has several
downsides. It makes wire format migrations more difficult. During a migration,
there comes a point when all peers can read both the old and new format, meaning
it is safe to start writing the new format. Since this change cannot occur
everywhere simultaneously, there will inevitably be a period where a peer is
receiving both old-format and new-format messages. Suppose it receives one of
each, both with unknown table fields, and then tries to encode both tables in a
single message. The only way to preserve unknown data in this case is to include
[wire format metadata] in every envelope, but this would add unacceptable
complexity and overhead.

Another downside concerns feature parity among FIDL bindings. In bindings that
support in-place decoding (e.g. LLCPP), it is difficult to choose a domain
object representation that can both own handles and represent unknown handles.
For known data, the decoder inserts handles in the domain object by overwriting
their presence indicators. For unknown data, the decoder only knows the number
of handles to skip in the handle table, not the locations of their presence
indicators. It is thus not possible to return an owning domain object short of
returning both the domain object and a re-packed handle table. Instead, these
bindings simply do not support preserving unknown data. This is likely to
surprise users who rely on it in other bindings, and it increases our testing
burden, requiring two GIDL tests for all cases involving unknown data.

In general, the need to preserve unknown data adds significant complexity to
FIDL. This complexity is not limited to the implementation, but affects users as
well, due to interactions with other features. For example, the distinction
between value and resource types was designed to only affect API compatibility,
not ABI. However, it was [later discovered][rfc-0057-bwd] to have unavoidable
ABI impact in the case when unknown handles are received for a flexible value
type. This corner case only exists because of the requirement to preserve
unknown data in domain objects.

## Stakeholders

Who has a stake in whether this RFC is accepted? (This section is optional but
encouraged.)

_Facilitator:_ pascallouis@google.com

_Reviewers:_ abarth@google.com, yifeit@google.com, ianloic@google.com

_Consulted:_ bryanhenry@google.com

_Socialization:_ A draft of this RFC was sent to the FIDL team for comments.

## Design {#design}

The handling of unknown values for flexible bits and enums remains unchanged.

When decoding tables and flexible unions:

* Bindings MUST NOT store unknown bytes and handles in domain objects, unless
  the bindings are specifically designed for proxying.

* Bindings MUST close all unknown handles.

When re-encoding tables and flexible unions that were previously decoded:

* Bindings MUST successfully re-encode the known fields of a table, and MUST NOT
  include unknown fields (which would imply storing them).

* Bindings MUST fail and return an error when encoding a flexible union with an
  unknown variant.

Concerning domain objects for tables and flexible unions:

* Bindings SHOULD NOT provide any mechanism to distinguish a table that had no
  unknown fields from a table that had unknown fields discarded. They should be
  considered equal if bindings provide deep equality functions.

* Bindings MUST provide a mechanism to determine if a flexible union had an
  unknown variant, and SHOULD provide access to the unknown ordinal (i.e. the
  domain object's unknown variant should only store the ordinal). Unknown
  variants should behave like [NaN] if bindings provide deep equality functions,
  comparing unequal even if ordinals are the same.

In Rust, the latter point implies removing the `Eq` trait from flexible unions
and types that transitively contain one, as is already done for floats.

## Implementation

The implementation is mostly a matter of deleting the code responsible for
preserving unknowns in all the bindings. We do not believe there are any
production uses of the unknown data accessors. If there are, we will have to
understand the use case and try to find a way forward.

Currently, LLCPP fails to re-encode a table that had unknown fields. This will
need to change per the design, to successfully encode just the known fields.

## Security considerations

This proposal improves security, as it results in less information and
capabilities being passed around implicitly. When unknown data is preserved, it
is easy to pass arbitrary bytes and handles through unsuspecting components.
When it is discarded, the data boundary becomes accurately encoded by the FIDL
schema, making the system easier to audit.

## Privacy considerations

This proposal improves privacy because it restricts the transmission of unknown
data, which could include sensitive information.

## Testing

Testing mostly occurs in GIDL. The `success` tests involving unknown data will
be split into two parts: `decode_success` and either `encode_success` (encodes
only the known table fields) or `encode_failure` (unions fail to encode). The
representation of values with unknown data will also change. GIDL should no
longer parse unknown bytes and handles, and instead use the syntax `123:
unknown` to indicate an unknown envelope at ordinal 123.

The allowlists and denylists that split LLCPP and non-LLCPP can be removed. All
bindings will have the same encoding/decoding behavior with respect to unknown
data. In addition, the LLCPP-specific unit tests added in fxrev.dev/428410 can
be removed in favor of GIDL tests.

Tests exercising all combinations of strict/flexible and value/resource should
remain, although decoding unknown data with handles for a flexible value type
will no longer fail.

## Documentation

The following documentation needs to be updated:

* [FIDL language specification]
* [FIDL bindings specification]
* Each section in the [Bindings reference]

## Drawbacks, alternatives, and unknowns

### Alternative: Optionally preserve unknowns

Rather than completely remove support for preserving unknown data, we could
continue to support it, just not by default. For example, it could be opt-in
with an attribute on the type, perhaps restricted to value types to alleviate
concerns about proxying unknown handles. However, this approach adds even more
complexity to a little-used feature, and it does not solve the wire format
migration problem.

### Drawback: Flexible types are inconsistent

A drawback of this proposal is that it makes the behavior of flexible types less
consistent, and perhaps less intuitive. To explain this, it helps to classify
bits, enums, tables, and unions along two axes as shown in [Table 2](#table-2):
algebraic type (product or sum) and payload (with or without a payload).

**Table 2: Classification of flexible types**
{:#table-2}

|                 | Product type | Sum type |
|-----------------|--------------|----------|
| Without payload | bits         | enum     |
| With Payload    | table        | union    |

Currently, all flexible types proxy unknown information. This proposal breaks
that symmetry along both axes. For example, consider the following FIDL types:

```fidl
// Product types (multiple fields set)
type Bits  = bits  {    A = 1;         B = 2; };
type Table = table { 1: a struct{}; 2: b struct{}; };

// Sum types (one variant selected)
type Enum  = enum  {    A = 1;         B = 2; };
type Union = union { 1: a struct{}; 2: b struct{}; };
```

First, we lose consistency across the payload axis. Currently, going from `Bits`
to `Table` or from `Enum` to `Union` increases functionality, permitting each
member to carry a payload. With this proposal, that functionality comes with the
cost of no longer preserving unknowns.

Second, we lose consistency across the algebraic type axis. Currently, both
`Table` and `Union` allow re-encoding after decoding objects with unknown data.
With this proposal, `Table` can re-encode but `Union` cannot.

We believe this trade-off of pragamatism over consistency is worthwhile to avoid
the complexities described in [Motivation](#motivation). However, there are
alternatives designs described below that retain more consistency.

### Alternative: Discard all unknown information {#alternative-discard}

To improve consistency, we could discard all unknown information, even unknown
integers that are easily stored. This means having a single unknown state for
bits and enums, and discarding the unknown ordinal in addition to the payload
for unions. [Table 3](#table-3) shows the resulting behavior.

**Table 3: Adjustment of [Table 1](#table-1): Discard all unknown information**
{:#table-3}

| Type  | Can access unknowns? | Can re-encode? | Proxies unknowns? |
|-------|----------------------|----------------|-------------------|
| bits  | Yes → **No**         | Yes            | Yes → **No**      |
| enum  | Yes → **No**         | Yes → **No**   | Yes → **No**      |
| table | Yes → **No**         | Yes            | Yes → **No**      |
| union | Yes → **No**         | Yes → **No**   | Yes → **No**      |

### Alternative: Optional flexible unions {#alternative-optional}

To improve consistency, we could require that flexible unions are always
optional, and then decode unknown variants as absent unions. This would make it
possible to re-encode unions, making them consistent with tables. [Table
4](#table-4) shows the resulting behavior.

**Table 4: Adjustment of [Table 1](#table-1): Optional flexible unions**
{:#table-4}

| Type  | Can access unknowns? | Can re-encode? | Proxies unknowns? |
|-------|----------------------|----------------|-------------------|
| bits  | Yes                  | Yes            | Yes               |
| enum  | Yes                  | Yes            | Yes               |
| table | Yes → **No**         | Yes            | Yes → **No**      |
| union | Yes → **No**         | Yes            | Yes → **No**      |

### Alternative: Remember if unknown fields were discarded {#alternative-remember}

In the proposed design, it is impossible for bindings users to tell if unknown
fields were discarded while decoding a table. An alternative would be to store a
boolean, or a set of unknown ordinals, in the table domain object. Users could
then query this via a function such as `has_unknown_fields()`. For example, a
storage service might want to fail in this case to avoid data loss.

A downside of this alternative is that it adds extra hidden state to table
domain objects. They are no longer simple value types, the sum of their fields.
For example, it raises the question of whether the `==` operator should take
such a boolean flag into account.

### Alternative: Important fields

The only realistic use case for checking `had_unknown_fields()`, as [described
earlier](#alternative-remember), is to fail if it returns true. Rather than
providing that accessor in bindings, we could accept an attribute on table and
flexible union members to opt into that behavior:

```fidl
type Data = table {
  1: foo string;
  @important
  2: bar string;
};
```

The effect of this attribute would be to set a newly reserved bit in the
envelope header for that field. When decoders encounter an unknown field with
the important bit set, they must fail. In other words, the `@important`
attribute opts out of forward compatibility, functioning like a dynamic version
of the static `strict` modifier we allow on bits, enums, and unions.

This alternative would likely require its own RFC on top of this one.

_Acknowledgement_: This idea originates with yifeit@google.com.

### Alternative: Keep ABI impact of value/resource

This proposal eliminates the [ABI impact of RFC-0057][rfc-0057-bwd], and
considers this to be an improvement made possible by discarding unknown data.
However, it can be argued that the ABI impact is desirable and should be kept.

Advantages of dropping ABI impact (this proposal):

* It makes strict/flexible and value/resource more independent features. There
  is no longer a special case in their intersection. Given that we did not
  notice this case until long after writing their respective RFCs, it is likely
  to be surprising to users as well.
* It makes it easier to transition a type from value to resource, since it only
  breaks API, not ABI. In cases where no code breaks (plausible for request and
  response types, which cannot be referenced directly in some bindings), this
  transition no longer silently changes behavior.

Advantages of keeping ABI impact (this alternative):

* It more accurately models the intent of the interface. If you indicate that
  you do not expect handles (by using a value type), and you receive handles at
  runtime, this points to a gap and failing is appropriate.
* If we change our minds, we can drop ABI impact later. Switching in the other
  direction is more likely to cause disruption.

## Prior art and references

### Protobuf

The design of protocol buffers has [gone back and forth on this
point][proto-unknowns]. In proto2, unknown fields are preserved and proxied
(like FIDL today). In proto3 the behavior was changed to discard unknown fields
during decoding (like this proposal). However, the decision was later reverted,
so in versions 3.5 and later proto3 once again preserves unknown fields.

This raises the question: will FIDL follow the same path if we accept this
proposal? We believe the answer is no, because FIDL and Protobuf occupy
different design spaces. Protobuf had to revert to the old preservation behavior
because of two use cases: intermediary servers and read-modify-write patterns.
Neither of these is prevalent in Fuchsia. Instead of having intermediary proxy
servers, Fuchsia's security and privacy principles encourage direct
communication. Instead of the read-modify-write pattern, the FIDL API rubric
recommends the [partial update pattern].

### Thrift

Apache Thrift [discards unknown fields][thrift-unknowns].

<!-- link labels -->
[`fidl_table_validation`]: https://fuchsia-docs.firebaseapp.com/rust/fidl_table_validation/index.html
[Bindings reference]: /reference/fidl/bindings/overview.md
[domain objects]: /contribute/governance/rfcs/0097_fidl_toolchain.md#terminology
[fewest features]: /contribute/contributing-to-fidl/design-principles.md#fewest-features
[FIDL bindings specification]: /reference/fidl/language/bindings-spec.md
[FIDL language specification]: /reference/fidl/language/language.md
[lang-flexible]: /reference/fidl/language/language.md#strict-vs-flexible
[NaN]: https://en.wikipedia.org/wiki/NaN
[partial update pattern]: /development/api/fidl.md#controlling-settings
[proto-unknowns]: https://developers.google.com/protocol-buffers/docs/proto3#unknowns
[rfc-0033]: /contribute/governance/rfcs/0033_handling_unknown_fields_strictness.md
[rfc-0057-bwd]: /contribute/governance/rfcs/0057_default_no_handles.md#backwards-compatibility
[rfc-0114-alternative-8-byte]: /contribute/governance/rfcs/0114_fidl_envelope_inlining.md#alternative-8-byte
[solve real problems]: /contribute/contributing-to-fidl/design-principles.md#solve-real-problems
[thrift-unknowns]: http://mail-archives.apache.org/mod_mbox/thrift-user/201204.mbox/%3CCACK7Cy6GgDharG=HBm5dyt75xpCmBVkVZnG3f73bdd-JmqK_vQ@mail.gmail.com%3E
[wire format metadata]: /contribute/governance/rfcs/0120_standalone_use_of_fidl_wire_format.md
