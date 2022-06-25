<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0132" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC limits FIDL tables to at most 64 members.

See also:

* [RFC-0047: Tables](0047_tables.md)
* [RFC-0116: Wire format support for sparser FIDL tables](0116_fidl_sparser_tables.md)

## Motivation

The user-mode performance cost of sending a FIDL table (building, encode and
decode) is a function of the maximum set ordinal in the table body. This
behavior isn't obvious or intuitive to users and can have negative performance
consequences if misused. This RFC proposes a limit to prevent egregious
behavior in the near term, act as a stop-gap solution to push FIDL authors
to rethink their design and encourage the FIDL team to provide a better
longer-term answer if FIDL authors start running into the limit.

## Stakeholders

Who has a stake in whether this RFC is accepted?

_Facilitator:_ pascallouis@google.com

_Reviewers:_ pascallouis@google.com, yifeit@google.com, mkember@google.com

_Consulted:_ ianloic@google.com, azaslavsky@google.com, abarth@google.com

_Socialization:_ N/A

## Design

Tables will be limited to at most 64 ordinals. Specifically:

- The FIDL compiler MUST emit an error if an ordinal value is above 64.
- Bindings MUST NOT error when an ordinal higher than 64 is received.
They MAY ignore ordinals higher than 64.

Additionally, if the 64th ordinal is non-reserved it must contain another
table, as a mechanism to ensure that the message continues to be extensible.
Specifically, the FIDL compiler MUST emit an error if the 64-th ordinal is
non-reserved and assigned a non-table type.

## Implementation

This will require changes to the FIDL compiler as well as each of the FIDL
bindings.

## Performance

All measurements in this section are in nanoseconds and were recorded on a NUC
with a Intel Core i5-7300U CPU @ 2.60GHz.

The following shows the total (build + encode + decode) user-mode time needed
for 1-way communication, measured for LLCPP:

![Total user-mode time for 1-way communication
](resources/0132_fidl_table_size_limit/build_encode_decode_time.png)

Both the cases where only the last field is set and all fields are set are
very close to linear, with slopes of 14.7 ns/field and 63.7 ns/field
respectively. This means an unset or reserved field costs approximately 14.7
ns while a set field costs approximately 63.7 ns.

When the maximum ordinal is 64, the last field set time is 3234.1 ns and the
all field set time is 6294.2 ns.

Note that these times may change as the binding implementation changes.

The following shows the current distribution of max non-reserved ordinal in
FIDL table definitions in the Fuchsia system:

![Distribution of max non-reserved ordinals in table definitions
](resources/0132_fidl_table_size_limit/table_definition_distribution.png)

The highest ordinal in a table definition is 56 with the next highest
being 26. The table with 56 fields has many fields that are unnecessarily
marked reserved and can be restructured to be a more compact table. This
implies that current tables will generally not reach the 64 ordinal limit
for some time.

### Smaller allocations

Some bindings, like LLCPP, calculate the maximum buffer size needed for a
message and use this information for sizing allocations. If tables are no
longer unbounded in size, it will be possible to decrease the size of the
allocations in some cases which may improve performance.

## Ergonomics

This RFC makes it more difficult to use FIDL because it is now necessary to
take a limit into consideration.

## Backwards Compatibility

This RFC breaks backwards compatibility with tables with over 64 entries.

## Security considerations

This has no security implications.

## Privacy considerations

This has no privacy implications.

## Testing

GIDL tests will validate that the wire format is unchanged and decode of tables
larger than 64-entries continues to work.
Unit tests will check compiler-level changes.

## Documentation

The FIDL language documentation will need to be updated to indicate this limit.

## Drawbacks, alternatives, and unknowns

### Why is 64 the limit?

The limit of 64 is arbitrary but it was chosen for several reasons:

- It is larger than what should be needed for the vast majority of use cases.
There will be outliers, but they can use alternative structures to represent
the data. If the limit were 32 instead, it would be much closer to the table
sizes commonly used today.

- If the limit were larger than 64, then there may be situations in which users
are surprised by degraded performance. If it is discovered that higher limits
are more commonly needed, there is still a possibility of raising the limit
again through another RFC.

- 64 is the number of bits in a 64-bit integer. This makes it possible for
bindings to potentially use bit flags to mark presence or absence. Similarly,
bit masks can be used to locate ordinals as in the rejected RFC
[RFC-0016](0116_fidl_sparser_tables.md).

### Alternatives for large tables

A number of alternative structures can be used in place of large tables. A
vector of unions is one such option - it is similarly extensible as a table
but has no limit on the number of union fields. It also has a representation
that may be more suitable for cases with sparse usage of fields.

### Sparse table layout

A previous rejected RFC explored an alternative layout for tables that enables
a sparser representation:
[RFC-0116: Wire format support for sparser FIDL tables
](0116_fidl_sparser_tables.md)

If tables used a sparser representation, then there may be less of a need for
a size limit. 