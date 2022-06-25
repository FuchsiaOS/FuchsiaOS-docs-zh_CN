<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0114" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes a FIDL wire format change that inlines values <= 4-bytes in
size into the body of envelopes.

## Motivation

The motivation for this change is to improve performance of FIDL tables and
unions (i.e. layouts which use envelopes today).

FIDL unions and tables use a shared representation for out-of-line objects
called an envelope. Out-of-line pointers are a known source of overhead
for encode and decode. Small objects can fit inline within the envelope
itself, avoiding the need for the out-of-line overhead.

Additionally, in some cases it may be possible to reduce allocations. Instead
of allocating an out-of-line location for an object and pointing to it from the
envelope, the object can be directly stored in the envelope.

## Design

This RFC design assumes the approval of
[RFC-0113](0113_efficient_envelopes.md), which introduces efficient envelopes.

A new inlined value format will be used for the following types:

- bool
- float32
- uint8, uint16, uint32
- int8, int16, int32
- enums with layout uint8, uint16, uint32, int8, int16, int32
- bits with layout uint8, uint16, uint32, int8, int16, int32
- handle, client_end, server_end
- structs <= 4-bytes in size
- arrays <= 4-bytes in size

If new types of values are added in the future that are <= 4 bytes in size,
they will also use the inlined value format unless otherwise stated.

The new format can be described through a C-struct representation:

```c++
// An envelope corresponds to a union value or an entry in a table.
struct Envelope {
    union {
        // Inlined values have the same envelope structure for both wire and
        // decoded formats.
        InlinedValueEnvelope inlined_value_envelope;

        // Out-of-line values have a different structure on the wire and in
        // decoded format.
        union OutOfLineEnvelope {
            // Wire representation.
            OutOfLineWireEnvelope out_of_line_wire_envelope;
            // Decoded representation.
            void* decoded_data;
        };
    };
};
struct InlinedValueEnvelope {
    // A <= 4-byte value stored little-endian and 0-padded up to 4-bytes.
    uint8_t value[4];
    // Number of handles within the envelope.
    uint16_t num_handles;
    // Bit 0 of flags is 1 to indicate the inline representation is used and
    // the envelope is present.
    uint16_t flags;
};
struct OutOfLineWireEnvelope {
    // Number of bytes recursively within the envelope.
    uint32_t num_bytes;
    // Number of handles recursively within the envelope.
    uint16_t num_handles;
    // Bit 0 of flags is 0 to indicate the out-of-line representation is used.
    uint16_t flags;
}
```

Both wire representations `InlinedValueEnvelope` and `OutOfLineWireEnvelope`
have overlapping `flags` fields. The LSB in the `flags` indicates if the
inline form is used or not: `1` for inlined and `0` for out-of-line. All
unused flag bits MUST be `0`.

There is only a single canonical representation of data in FIDL.
Present values that are up to 4 bytes in size MUST be inlined and values over 4
bytes MUST use the out-of-line representation. Receipt of a value of incorrect
representation MUST trigger a decoding error.
Absent envelopes continue to use the zero envelope representation, meaning they
are always represented by the out-of-line representation.

## Implementation

This change will require a complex migration. However, this migration can be
combined with other wire format migrations, making it much cheaper in
practice.

## Performance

There is a significant decrease in encode time in LLCPP when fields are inlined
([CL](https://fuchsia-review.googlesource.com/c/fuchsia/+/542901)):

Encode time (in nanoseconds) w/ all fields set:

| # Fields | Before  | After               |
|----------|---------|---------------------|
| 1        | 178  ns | 147  ns             |
| 16       | 720  ns | 325  ns             |
| 256      | 9396 ns | 2909 ns             |

This chart shows encode time as a function of the number of fields a table has.
All fields in the table were set.

Decode time was not measured, but is also expected to have a significant
improvement as the decode algorithm follows a similar series of steps as
encode.

Additionally, bindings may in some cases be able to avoid making allocations
for small values which will further improve performance.

## Ergonomics

This RFC allows bindings to avoid allocations for small values, but does not
prescribe that they must do so. If bindings do change, the API for working
with these types could end up being different than the API for working with
other types that require allocations. This inconsistency could lead to poorer
ergonomics and care must be taken to avoid this.

## Backwards Compatibility

The migration needed for this change breaks ABI-compatibility.

However, once the change is in effect there is no effect on ABI-compatibility
for type changes. All <= 4-byte types provide no guarantees of ABI
compatibility for type changes both before and after this change.

This change MAY break source compatibility. No source compatibility breaking
changes are required by the RFC, but bindings MAY choose to make source
compatibility breaking changes if they improve the performance of the binding
or other reasons.

## Security considerations

This has no impact on security.

## Privacy considerations

This has no impact on privacy.

## Testing

Several strategies will be used to test the change:

- Custom unit tests in each binding.
- GIDL conformance suite.
- FIDL compatibility testing.

## Documentation

The wire format documentation needs to be updated.

The performance tradeoff needs to be documented in the API rubric to inform
field size decisions.

## Drawbacks, alternatives, and unknowns

### Drawbacks

The main drawback to this proposal is the increased complexity. Now there are
two representations of values - inline and out-of-line, depending on the type,
and it might be surprising that there is a 4-byte threshold for switching
behavior.

### Alternative: 8-byte inline values {#alternative-8-byte}

This RFC proposes inlining values that are 4-bytes or less.
The reason for this is that it does not appear to be possible to inline
8-byte values - at least when implemented in conjunction with efficient
envelopes.

The reason for this is that bindings must support unknown envelopes. When
an unknown envelope arrives, no type information is known. It is therefore
unknown whether it is pointing to an out-of-line object or not, which would
change the behavior of the decoder. Because of this, there needs to be some
information in the value itself that indicates if it is structured in the
inline or out-of-line format.

If the envelope size is 8-bytes and the value being inlined is 8-bytes, there
is no spare bit to store if the value is in the inline or out-of-line format.

Because of this, 8-byte inline values are incompatible with efficient
envelopes. A choice needs to be made to either not use efficient envelopes
or reduce the size of the value that can be inlined. This RFC makes the latter
choice, since this direction seems most likely to have the most significant
performance improvement.

## Prior art and references

[RFC-0113](0113_efficient_envelopes.md)
introduced efficient envelopes, which form the basis for the envelope structure
used in this RFC.
