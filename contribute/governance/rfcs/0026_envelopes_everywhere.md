{% set rfcid = "RFC-0026" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-026.

## Rejection rationale

Given the amount of feedback and comments on this RFC, we've decided to
withdraw (i.e. self-reject) the proposal.
That said, it still has some great ideas: we'll be taking those ideas and
publishing them as separate RFCs with smaller scope, to enable clearer
discussion and separate independent features into their own RFCs.

[RFC-0032][rfc-0032] was spun out of this RFC.

## Summary

This RFC has two goals:

1. Improve the efficiency of the existing [envelope] format, by making
   envelopes more than twice as compact.
2. Use envelopes as the sole means of referencing _all_ out-of-line objects.
   This increases the consistency of the wire format, and the uniformity of
   protocol design and implementation.

A side-effect of both (1) and (2) is that optionality (nullability) can be
efficiently implemented for all types, not just structs, handles, vectors,
strings, tables and (extensible) unions[[1]](#footnote1)

## Motivation

Envelopes are the foundation for extensible, evolvable data structures
(tables and extensible unions).
Making envelopes more efficient enables those extensible structures to be
used in more contexts where performance and wire size matter.

FIDL also has several pervasive types that are used for dynamically-sized
data: vectors & strings.
These types are required to be out-of-line since the size of the FIDL primary
object is expected to be statically known.
If envelopes can be used to represent all out-of-line data, we can simplify
both the protocol and implementation, reducing implementation cost and room
for error.

Additionally, FIDL would benefit from a holistic, consistent approach to
optionality.
This leads to better ergonomics, optionality for more types than the current
mechanisms allow for, and a simplified user mental model.
Envelopes fulfill these goals by enabling optionality for all types in a
uniform manner.

## Design

Envelopes can refer to data that is either:

*   _out-of-line_, similar to the existing envelope format, or
*   _inline_, where data is stored in the envelope itself.
     This can be used for "small-sized" types that are fixed-size and less
     than 64 bits.

### Out-Of-Line Envelopes {#out_of_line-envelopes}

An out-of-line envelope is:

![Figure: out of line envelope, 64 bit little endian, lower 48 bits size with
least significant bit zero, 16 bits
handle_count](resources/0026_envelopes_everywhere/figure1.png)

As a C struct:

```c
typedef struct {
  uint64_t size:48;  // Low bit will be 0
  uint16_t handle_count;
} fidl_out_of_line_envelope_t;
```

The out-of-line envelope has the following changes vs the [existing envelope
format][wformat-envelopes]:

*   **Size (num_bytes) is 48 bits instead of 32 bits**, enabling larger
    payloads.
    *   The size includes the size of any sub-objects that may be recursively
        encoded.
        *   For example, the size of a `vector<string>` includes the size of
            the outer vector's inner string sub-objects.
        *   This matches the existing behavior for the current envelope
            implementation's size field.
    *   A legitimate out-of-line object's size will always be a multiple of
        8, due to out-of-line objects being eight-byte aligned.
        This implies that `size % 8 == 0`, which means that
        *   the lowest three bits of the size field &mdash; and thus the LSB
            of the size field &mdash; will be zero, thus
        *   the envelope's LSB &mdash; since the size field is in the LSB of
            the envelope &mdash; will always also be zero.
        *   This is important, as discussed in the [Tag Bit](#tag-bit) below.
    *   See [Encoding Sizes for Out-of-Line
        Envelopes](#encoding-sizes-for-out_of_line-envelopes) below for
        performance implications of calculating the recursive size.
*   **The `handle_count` is 16 bits, instead of 32 bits.**
    *   It's not currently possible to send > 64 handles over a Zircon channel;
        we feel that 16 bits provides enough headroom for future needs.
    *   The `handle_count` includes the handle count for all recursive
        sub-objects.
*   **The presence/absence field is dropped.**
    *   Presence is represented by a non-zero value in either the `size` or
        `handle_count` field.
    *   Absence is represented by the `size` & `handle_count` fields both
        being zero.
        *   We call this a _zero envelope_.

Decoders MAY overwrite the envelope with a pointer to the envelope data,
assuming they know the static type (schema) of the envelope's contents.
See the [Decoder Callback](#decoder-callback) section for recommendations on
how to process an envelope if the content's type is unknown.

### Tag Bit {#tag-bit}

An out-of-line envelope explicitly has the size occupying the least
significant bits, and the handle count occupying the most significant bits.
As discussed in the [Envelope](#out_of_line-envelopes) section,

*   since the lowest bit of the size field will always be zero (due to the
    size being a multiple of 8),
*   the lowest bit of the envelope will _also_ always be zero.

We call the lowest bit of the envelope the _tag bit_.

*   If the tag bit is zero, the envelope's data is _out-of-line_.
*   If the tag bit is one, the envelope's data is _inline_.

Since the tag bit is one for inline data, an inline envelope also cannot be
an actual pointer on architectures that require 64-bit alignment, since
pointers will be a multiple of 8 and also require the lowest three bits to be
zero.
This is useful for a decoder to be able to distinguish inline envelopes from
an actual pointer, since decoders typically overwrite out-of-line envelopes
&mdash; but not inline envelopes &mdash; with a pointer to the envelope's
content.

### Inline Envelopes

Inline envelopes are encoded as:

![Figure: in line envelope, 64 bit little endian, least significant bit is the
value 1 indicating tag, 31 bits reserved, then 8, 16, or 32 bits of inline
data](resources/0026_envelopes_everywhere/figure2.png)

As a C struct:

```c
typedef struct {
  uint8_t tag:1;  // == 1
  uint32_t reserved:31;
  union {
    _Bool bool;
    uint32_t uint32;
    int32_t int32;
    uint16_t uint16;
    int16_t int16;
    uint8_t uint8;
    int8_t int8;
    float float32;
    zx_handle_t handle;  // Only when decoded (see Handles for more details)
  };
} fidl_inline_envelope_t;
```

*   Inline envelopes have their LSB set to 1, which differentiates them from
    out-of-line envelopes and actual pointers.
*   The upper 32 bits of the envelope are used to represent the inline value,
    which can be an `int8`, `uint8`, `int16`, `uint16`, `int32`, `uint32`,
    `float32`, `bool`, or a handle.
    *   The lowest bits of the upper 32 bits are used to represent the value
        if the value is less than 32 bits wide, which is standard
        little-endian representation.
*   Encoders MUST encode reserved bits as zero unless a future RFC specifies
    how those bits are to be interpreted.
*   Decoders and validators MUST ignore reserved bits unless a future RFC
    specifies how those bits are to be interpreted.
*   Decoders SHOULD leave inline envelopes as-is during decoding.
    *   Since inline data has the data inline instead of needing to be
        referenced out-of-line, decoders do not need to replace them with a
        pointer when decoding in-place (unlike out-of-line envelopes).

### Should Encoders Encode as Out-of-Line or Inline?

An encoder MUST:

*   encode data inline iff the type is a `bool`, (`u`)`int8`, (`u`)`int16`,
    (`u`)`int32`, `float32` or handle.
    (Informally: if the type is fixed-size and <= 32 bits.)
*   encode data out-of-line for all other types.
    (Informally: if the type is >= 64 bits or variable-sized.)

### Handles

There are three contexts for handle declaration:

1. a non-optional handle in a non-extensible container, e.g. `struct S
   { handle h; };`
2. an optional handle in a non-extensible container, e.g. `struct S {
   handle? h; };`
3. a handle in an extensible container, e.g. `table T { handle h; }`

For (1), a non-optional handle in a non-extensible container, we propose
keeping the [existing wire format][wformat-handles], which is a `uint32`.
There is no need for a non-optional handle in a non-extensible container to
be an envelope, since envelopes are designed to carry optional or
dynamically-sized data.

For (3), a handle in an extensible container: since envelopes are the
foundation for extensible containers, an envelope must be used to encode the
handle.
To encode a handle, an encoder MUST encode it as an out-of-line envelope,
with `size` set to 0, and `handle_count` set to 1:

![Figure: little-endian 64 bit data field with bottom 48 bits of size set to
zero and next 16 bits indicating handle_count set to
1](resources/0026_envelopes_everywhere/figure3.png)

This encoding instructs a decoder to look up the handle value in the
out-of-line handle table.
If a decoder wishes to decode in-place, the decoder SHOULD:

*   look up the handle in the out-of-line handle table, to determine the
    actual handle value.
*   set the tag bit to 1, which changes the envelope from out-of-line to inline.
*   set the handle field of the fidl_inline_envelope_t struct to the actual
    handle value.

![Figure: little-endian 64 bit data field with least significant bit tag set to
1, next 31 bits reserved, next 32 bits
handle_value](resources/0026_envelopes_everywhere/figure4.png)

See the [Examples](#examples) section for an example encoded/decoded handle.

We choose this dual encoded/decoded form since it is compatible with both the
out-of-line and inline envelope encodings.
While this does result in specialized code for handles in envelopes, we
believe that having more uniform, i.e. fewer, data encodings is a better
trade-off than simpler code that requires more encodings.

For (2), an optional handle in a non-extensible container: we also propose
using the same envelope representation as context (3) for the wire format,
i.e. the dual out-of-line-encoded/inline-decoded form.
Unfortunately, this representation of an optional handle is less compact than
the [existing optional handle wire format][wformat-handles], which is a
`uint32`.
However, we still advocate using the envelope-based representation, since

*   using an envelope for an optional handle is consistent with using
    envelopes for any optional type,
*   optional handles are relatively rare in a FIDL message vs other message
    types[[2]](#footnote2), so the extra 4 bytes of envelope overhead should
    not significantly impact message size,
*   keeping the existing `uint32` wire format for optional handles would
    result in three encodings and three separate code paths for handles:
    non-optional, optional, and handle-in-envelope.
    Using the envelope representation for optionals eliminates one encoding
    and one code path, which increases uniformity and decreases specialized
    code.

The encoding for (2) &mdash; optional handles in a non-extensible container
&mdash; is explicitly listed in the [Design Decisions](#design-decisions)
section below, since the more compact `uint32` representation for an optional
handle could be worth considering.

### Strings & Vectors {#strings-vectors}

The current wire format for non-nullable [Strings][wformat-strings] and
[Vectors][wformat-vectors] are stored as 16 bytes:

*   a `uint64` for the number of elements (vector) or number of bytes (string),
*   a `uint64` for the presence/absence/pointer.

We propose using an envelope to represent both strings and vectors, either
nullable or non-nullable:

*   **The number of elements (vector) or bytes (string) is moved out-of-line**.
    *   This enables a vector/string to be represented by an envelope (only),
        so envelopes become the sole means of referencing _any_ out-of-line
        data, for all FIDL types, enabling a consistent representation for
        all out-of-line data.
    *   The vector/string contents are in a separate out-of-line object, and
        immediately follow the element/byte count.
*   Presence/absence is determined by the envelope either being zero or
    non-zero.

For vectors, note that the vector element count is not the same as the envelope's size:

*   The envelope's size is the vector element count multiplied by the element
    size.
*   If the vector contains sub-objects (e.g. `vector<Table>`,
    `vector<vector<string>>`), the envelope's size includes the size of all
    recursive sub-objects.

Nullable strings/vectors, and strings/vectors inside extensible containers,
are represented the same way as non-nullable strings and vectors: the zero
envelope is used to indicate an absent string/vector.

Conversely, if a string/vector is non-nullable, a validator MUST error if
it encounters a zero envelope.

This may be a source-breaking change for code that uses the C bindings,
which expect the memory layout for a `fidl_vector_t` and `fidl_string_t`
to exactly match the wire format.
We can, however, implement a transitional plan before a wire format change
(e.g. change the C API to use functions or macros) that enable this to be a
soft transition.

Note that it's still possible to represent this new string/vector layout as a
C struct via [flexible array members][flexible-array-members] (e.g. `struct {
uint64 element_count; element_type data[]; };`).

### Optional (Nullable) Types

Currently, structs, strings, vectors, handles, unions, tables and extensible
unions can be optional (nullable).

**Using envelopes everywhere enables _all_ types to be optional**:

*   Present optional data is stored with an envelope, either out-of-line
    or inline.
*   Absent optional data is stored as a zero envelope.

Note that for small-sized types, inline data can store optional types as
compactly as non-optional types, depending on the container's alignment
requirements.

### C/C++ Structs for Encoded/Decoded Forms

The encoded form of an envelope can be represented by a union of either an
inline or out-of-line envelope.
Similarly, a decoded envelope can either be inline, a pointer to the envelope
data, or a callback-determined value (see the [Decoder
Callback](#decoder-callback) section for details).

```c
typedef union {
  fidl_inline_envelope_t inline;            // Low bit is 1
  fidl_out_of_line_envelope_t out_of_line;  // Low bit is 0
} fidl_encoded_envelope_t;

typedef union {
  fidl_inline_envelope_t inline;  // Low bit is 1
  void* data;                     // Low bit is 0
  uintptr_t callback_data;  // Value determined by callback (see Decoder Callback)
} fidl_decoded_envelope_t;

static_assert(sizeof(fidl_encoded_envelope_t) == sizeof(void*));
static_assert(sizeof(fidl_decoded_envelope_t) == sizeof(void*));
```

### Unknown data {#unknown-data}

Receivers &mdash; validators & decoders &mdash; may not know the type of an
envelope when they're used in an evolvable data structure, such as a table
or extensible union.
If a receiver doesn't know the type of an envelope:

*   Inline envelopes can be safely ignored.
    *   Handles MUST be encoded with out-of-line envelopes, not inline
        envelopes, which makes all inline envelopes safe to ignore.
*   Out-of-line envelopes can be minimally parsed and skipped.
    *   The envelope's size determines the amount of out-of-line data to skip.
    *   If the envelope's handle count is non-zero, a validator MUST process
        the specified number of handles.
        *   The default processing behavior MUST be to close all handles.
    *   A decoder MAY overwrite the unknown envelope with a pointer to the
        envelope's contents, if it wishes to decode in-place.
        *   If a decoder does overwrite the envelope with a pointer, it will
            lose the size & handle count information in the envelope.
            If this is problematic, see the [Decoder Callback](#decoder-callback)
            section for an alternative.

Note that embedding the size in the out-of-line envelope enables rapid linear
seeking through a FIDL message if many unknown types need to be skipped.

### Decoder Callback {#decoder-callback}

As mentioned in the [Unknown Data](#unknown-data) section, an unknown
envelope may be overwritten by a decoder: if this happens, the decoder will
lose the size and handle count information.
As an alternative, a decoder MAY have a callback attached to it that can
process the envelope and override the default behavior.
The callback API can look similar to the following function prototype:

```c
void set_unknown_envelope_callback(
    unknown_envelope_callback_t callback,  // a callback
    void* context                          // client-specific data storage
);

typedef uintptr_t (*unknown_envelope_callback_t)(
    const void* message,  // pointer to the envelope's containing message
    size_t offset,        // offset in the message where the unknown envelope is
    size_t size,          // the envelope's size
    size_t handle_count,  // the envelope's handle count
    const char* bytes,    // pointer to the envelope's data
    void* context         // a context pointer set via set_unknown_envelope_callback()
);
```

The callback returns a `uintptr_t`, which the decoder can use to overwrite
the unknown envelope with.
This enables the decoder to copy the size and handle count from the unknown
envelope, and overwrite the envelope with a pointer to the decoder's own
custom data structure.

### Encoding Sizes for Out-of-Line Envelopes {#encoding-sizes-for-out_of_line-envelopes}

This RFC requires that out-of-line envelopes have the correct (recursive)
size for present out-of-line data.
This requirement can impose additional burden on an encoder, since if the
envelope's type is expected to be known by the receiver, the size field is
unnecessary since the decoder can compute the size[[3]](#footnote3).
Thus, the encoder is arguably performing additional work for no apparent
benefit.
This argument also applies to the handle count.

However, we still recommend that the size and handle count MUST be present,
for several reasons:

1. Consistency: requiring the size means that the envelope encoding is
   consistent for all use cases, whether it's inside an extensible container
   or not.
   The increased uniformity leads to less code, and a simpler cognitive model.
2. _We can change this later_.
   A future RFC has the option of using a sentinel value for the size (e.g.
   `UINT48_MAX`) or reserving one of the three LSBs in the size field to
   indicate that the size is unknown, in which case the decoder must traverse
   the out-of-line payload and calculate the size itself.
   This change would not affect the wire format, since the structure of the
   fields remain the same.
   It can also be landed as a soft transition since decoders can implement
   the logic first, before encoders are updated.

Overall, the RFC authors believe that requiring an encoding for an unknown
size is possible premature optimization, and advocate starting with a simple,
more consistent, uniform design.
If we feel that this decision should be re-visited in the future &mdash; e.g.
a zero-copy [vectored I/O][vectored-io] encoder becomes available so encoders
don't have to patch up envelopes to write the correct size &mdash; there is a
clear path to implementing it in as a soft transition.

## Examples {#examples}

An optional `uint` stored inline:

```fidl
uint32? u = 0xdeadbeef;  // an optional uint: stored inline.
```

C++ representation:

```cpp
    vector<uint8_t> object{
        0x01, 0x00, 0x00, 0x00,                          // inline tag
                                0xEF, 0xBE, 0xAD, 0xDE,  // inline data
    };
```

An optional `vector<uint16>` stored out-of-line:

```fidl
vector<uint16>? v = { 10, 11, 12, 13, 14 };  // an optional vector<uint16>; stored out-of-line.
```

The out-of-line size is 24:

*   8 bytes for element count stored out of line as its own secondary object,
*   + 10 for vector contents (5 elements * `sizeof(uint16_t)`),
*   = 18, rounded up to 24 for alignment.

C++ representation:

```cpp
    vector<uint8_t> object{
      0x18, 0x00, 0x00, 0x00, 0x00, 0x00,              // envelope size (24)
                                          0x00, 0x00,  // handle count
    };

    vector<uint8_t> sub_objects{
      // element count
      0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      // vector data
      0x0A, 0x00, 0x0B, 0x00, 0x0C, 0x00, 0x0D, 0x00,
      0x0E, 0x00,
      // padding
                  0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    };
```

A `table` with three fields:

```fidl
table T { 1: int8 i; 2: reserved; 3: int64 j; } = { .i: 241, .j: 71279031231 };
```

C++ representation:

```cpp
    // a table is a vector<envelope>, which is represented with an
    // out-of-line envelope
    vector<uint8_t> object{
      0x28, 0x00, 0x00, 0x00, 0x00, 0x00,              // envelope size (40)
                                          0x00, 0x00,  // handle count
    };

    vector<uint8_t> sub_objects{
      // vector element count (max table ordinal)
      0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      // vector[0], 1: int8, stored inline
      0x01, 0x00, 0x00, 0x00,                          // inline tag
                              0xF1, 0x00, 0x00, 0x00   // 241
      // vector[1], 2: reserved
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // zero envelope
      // vector[2], 3: int64, stored out-of-line
      0x08, 0x00, 0x00, 0x00, 0x00, 0x00,              // envelope size
                                          0x00, 0x00,  // handle count
      // vector[2] content
      0xBF, 0xB3, 0x8F, 0x98, 0x10, 0x00, 0x00, 0x00   // 71279031231
    };
```

A handle:

```fidl
handle h;  // decoded to 0xCAFEF00D
```

C++ representation:

```cpp
    vector<uint8_t> encoded_form{
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00,              // envelope size
                                          0x01, 0x00,  // handle count
    };

    vector<uint8_t> decoded_form{
      0x01, 0x00, 0x00, 0x00,                          // inline tag
                              0x0D, 0xF0, 0xFE, 0xCA,  // inline data
    };
```

## Implementation strategy {#implementation-strategy}

This RFC is a breaking wire format change.
Both FIDL peers need to understand the new wire format &mdash; and
communicate that understanding to its peer &mdash; for both parties to use
the new format.

A soft transition is possible.
Two approaches are:

1. There is a `uint32` reserved/flags field in the [transactional message
    header][wformat-transactional].
    We can reserve 1 bit for the initiating peer to indicate that it
     understands the new wire format, and soft transition in stages:
    1. Ensure all clients & servers can understand the old & new wire format.
       We keep using the old wire format.
    2. Enable the new wire format by having a peer set the bit in the
       transactional message header.
       If both parties have the bit set, both parties can switch to the new
       wire format.
    3. Once the soft transition has rolled through all the layers, all of
       Fuchsia can use the new wire format.
       We can remove setting the bit in the transactional message header.
    4. Delete the code for the old wire format, and unreserve the
       transactional message header bit.
2. We could decorate specific FIDL message types, interfaces, or both, with a
   `[WireFormat=EnvelopeV2]` attribute (or similar) that indicates that the
   message/interface should use the new wire format.
    1. While decorating an interface with a `[WireFormat]` attribute seems to
       align better with a wire format change, it should be easier to
       implement a WireFormat change on a struct, since the struct could be
       used in different interfaces, and bindings would need extra logic to
       determine the context for which the struct is used.
    2. We recommend that an interface `[WireFormat]` attribute affect the
       wire format of the interface's method arguments only, without
       recursively affecting the argument's structs.
    3. This enables partial migration and opt-in to the new wire format, and
       lets teams move at their own pace.
    4. Once all structs and interfaces have the `[WireFormat]` attribute, we
       can drop the old wire format, assume all structs & interfaces use the
       new wire format, and ignore the attribute.

Both these soft transition approaches involve a lot of development time,
testing time, and room for error.
Implementing the code to do either approach correctly, executing on the plan,
and following up successfully to remove old code is a large effort.

It is likely that we will have code to handle both the old & new wire format
at the same time; otherwise, it would not be possible to progressively land
CLs as we implement support for the new wire format.
Given that the code to handle both wire formats will exist, we recommend
prototyping whether a soft transition is feasible using either approach.
If not, *c'est la vie*; hard transition it is.

For either a soft or hard transition, any instances in Fuchsia where FIDL
messages are hand-rolled would need to also be upgraded to the new wire format.

We should also use this wire format change to fold in other changes that
need to happen (e.g. a proposed ordinal size change).

Note that this is an easier transition than FIDL1 to FIDL2, which changed
language bindings significantly.
We do not propose calling this FIDL3 since there are no user-visible
changes[[4]](#footnote4)

## Backwards compatibility

The proposed wire format change is API (source) compatible, with one
exception: C bindings will be a breaking API change if we move the
vector/string element count to be out-of-line.
We can mitigate this by planning ahead and abstracting the current C bindings
with macros or functions, before the new wire format lands.

The wire format change is ABI-incompatible, but it is possible to achieve ABI
compatibility with existing code via the strategies outlined in the [Implementation
Strategy](#implementation-strategy) section.

## Performance

This RFC significantly shrinks the size required for envelopes, which seems
like it would be an overall significant net benefit.
However, the overall performance implications are less clear.
In favor of better performance:

*   FIDL messages that use extensible data structures (tables & extensible
    unions) will become significantly more compact.
*   Having a uniform representation for envelopes and optionality may reduce
    code size and improve cache locality, since envelope code can be shared.

However:

*   If extensible data structures become more pervasive due to their better
    efficiency, this may be outweighed by their increased usage, which may
    result in less compact messages and more dynamic allocation, vs using
    non-extensible data structures.
*   Introducing optionality for all types may make FIDL messages slightly
    larger, since users may use this feature to make some previously
    non-optional types optional.
*   Optional handles become less efficient, if we decide to use the envelope
    encoding for optional handles.
*   As discussed in [Encoding Sizes for Out-of-Line
    Envelopes](#encoding-sizes-for-out_of_line-envelopes), encoding the size
    and handle count in an envelope for a type that a receiver will know is
    a performance regression from the current behavior.

## Ergonomics

*   Optionality can be enabled for all FIDL types.
    This is an ergonomic improvement, since optionality becomes consistent,
    instead of only for specific types.
*   More efficient extensible data structures enable them to be used in more
    contexts where efficiency matters, so users need to worry less about their
    performance, and can gain the benefits of extensibility where they would
    previously need to use non-extensible structures.
    *   We may even wish to recommend that tables should be used by default
        for FIDL data structures, and structs should be reserved for
        high-performance contexts.
    *   Extensible unions ([RFC-0061][rfc-0061]) are already attempting to
        remove static unions.

## Documentation

*   The wire format documentation needs to be updated.
*   When updating the documentation, envelopes should be explained as a
    first-class concept: this enables better [cognitive
    chunking][cognitive-chunking] once readers encounter the wire format for
    optionality and extensible data structures.
*   We should update the FIDL style guide to make recommendations for when
    optional types should be used (vs non-optional types with sentinel values).

## Security

*   There should no significant security implications from this RFC.
*   However, the bit-twiddling needed to manipulate the out-of-line and
    inline envelope formats should be significantly well tested and
    conservative to ensure that code properly handles edge cases.
    We do feel that the use of standard C/C++ struct/unions to represent
    envelopes &mdash; as opposed to manual bit shifts & masking &mdash;
    greatly increases our confidence in code correctness.

## Testing

*   Since this RFC is changing the wire format for envelopes, we feel that
    the existing FIDL test suite &mdash; particularly compatibility tests
    &mdash; will adequately test all scenarios where envelopes are used.
*   We will add unit tests for envelope parsing, encoding and decoding for
    out-of-line and inline forms, since that is a potentially error-prone area.
*   If we agree to land the wire format change as a soft transition (see the
    [Implementation Strategy](#implementation-strategy) section), we will add
    tests for peers to negotiate and possibly switch to the new wire format.
*   If we agree to expose optionality for all types as part of this change,
    we will need to add tests for any types that can become optional.

## Drawbacks, alternatives, and unknowns

*   We can keep the existing wire format if we believe the efficiency gains
    in this proposal are not worth the implementation cost.
    If so, we will want to find an alternative strategy to implement
    optionality for all types.
*   Using specialized representations for extensible containers and optional
    types will likely be more efficient than using an envelope for all cases.
    However, since this RFC exists, we obviously feel that the increased
    genericity and uniformity that envelopes provide outweigh the efficiency
    gains for specialized representations.

## Design Decisions {#design-decisions}

While this RFC makes recommendations, we are actively seeking input and consensus on the following decisions:

*   See the [Strings & Vectors](#strings-vectors) section for discussion
    about moving the element count (vector) and byte count (string)
    out-of-line, which affects the C bindings.
    We can opt not to do this, at the cost of less uniformity: strings &
    vectors become exceptions to envelopes being used for all out-of-line
    references.
    (An envelope can still be used to refer to the out-of-line vector/string
     data.)
*   Do we want to consider a soft transition or a hard transition? See the
    [Implementation Strategy](#implementation-strategy) section for pros & cons.
*   We propose using 48 bits for size and 16 bits for handles in the
    out-of-line envelope.
    For comparison, the current envelope format uses 32/32 bits.
    Is 48 bits for size reasonable?
    *   For size, we can encode a size up to 50 bits by right-shifting 2 bits
        in the encoded form, since the envelope size is always a multiple of 8.
        (We cannot right-shift three bits, since that cannot guarantee that the
        [tag bit](#tag-bit) is 0.)
        The decoder would left-shift two bits to determine the size.
        We then lose two extra bits that may be used for flags or more tags.
    *   While current 64-bit architectures typically don't allow the entire
        64-bit memory space to be addressed, and typically allow up to 48 bits,
        some architectures are already enabling address spaces up to 57 bits large.
        See [References](#prior-art-and-references) for more details.
    *   Is 16 bits for handles reasonable?
*   We propose using an envelope to encode an optional handle in a
    non-extensible container, which is less compact than the current optional
    handle encoding (8 bytes vs 4 bytes).
    *   There is a trade-off between compactness & more specialized code vs
        consistency here.
        We believe consistency and uniformity is more important than a
        specialized, more compact representation since optional handles are
        likely to be relatively rare use case.
        (37 optional uses in code vs 187 non-optional.)
*   Do we enable optionality immediately?
    *   We propose exposing optionality for all types in a separate
        transition from upgrading the wire format, since the change can be
        done incrementally.
    *   Implementing such optionality would require changes to the parser,
        encoders, validators and decoders, which feels large enough to
        warrant being its own transition.
*   We propose inlining types that are <= 32 bits; we could inline more
    aggressively.
    *   We can inline any data <= 63 bits, since the tag bit only uses one
        bit in the 64-bit envelope.
    *   We can inline small strings & vectors by using a specialized
        representation for them, e.g. one byte for the element/byte count,
        followed by the string/vector data. (See [Prior
        Art](#prior-art-and-references) for inspiration).
    *   We discarded these approaches even though they're more efficient,
        since inlining based on the content instead of the type means that
        (1) decoders cannot know in advance whether to expect an inline or
        out-of-line envelope or not based on the type, and (2) changing a
        field's contents means that it can be encoded differently, which
        seems contrary to FIDL's goals and static focus.

## Prior art and references {#prior-art-and-references}

The authors took a lot of inspiration from existing uses of [tagged
pointers](https://en.wikipedia.org/wiki/Tagged_pointer), which have a long
history in dynamic and functional languages.
In particular, the Objective-C 64-bit runtime makes [heavy use][objective-c-pointers]
of them for better performance (even going so far as using specialized
[5/6-bit encodings for inline strings][objective-c-strings]).

Since current 64-bit platforms tend to use 48 bits (or less) to encode a
pointer, we considered stealing more
bits from the decoded pointer with bit-shifting to attempt to encode an
out-of-line object's size along in the pointer.
However, some architectures are already expanding their physical address
space past 48 bits ([ARM64][arm-physical], [x64-64 5-level
paging][x86-physical]), so stealing more pointer bits may not be very
future-proof.

--------------------------------------------------------------------------------

##### Footnote1
Envelopes _enable_ optionality for all types; however, exposing this
optionality to end-users can (and perhaps should) be done separately.

##### Footnote2
As of 1/28/19, there appears to be 37 uses of optional handles in the Fuchsia
code base.
This is a conservative number, as it does not count optional protocol
handles, nor protocol request handles.

##### Footnote3
This only applies to envelopes in non-extensible containers, i.e. structs and
static unions.
Extensible containers must encode the recursive size since decoders may not
know the type, and need to know how much data to ignore.

##### Footnote4
Except allowing optionality on more types, if we wish to do that simultaneously.

<!-- xrefs -->
[RFC-0032]: /contribute/governance/rfcs/0032_efficient_envelopes.md
[RFC-0061]: /contribute/governance/rfcs/0061_extensible_unions.md
[wformat-handles]: /reference/fidl/language/wire-format/README.md#handles
[wformat-strings]: /reference/fidl/language/wire-format/README.md#strings
[wformat-vectors]: /reference/fidl/language/wire-format/README.md#vectors
[wformat-envelopes]: /reference/fidl/language/wire-format/README.md#envelopes
[wformat-transactional]: /reference/fidl/language/wire-format/README.md#transactional-messages
[flexible-array-members]: https://en.wikipedia.org/wiki/Flexible_array_member
[vectored-io]: https://en.wikipedia.org/wiki/Vectored_I/O
[cognitive-chunking]: https://en.wikipedia.org/wiki/Chunking_(psychology)
[tagged-pointers]: https://en.wikipedia.org/wiki/Tagged_pointer
[objective-c-pointers]: https://www.mikeash.com/pyblog/friday-qa-2012-07-27-lets-build-tagged-pointers.html
[objective-c-strings]: https://www.mikeash.com/pyblog/friday-qa-2015-07-31-tagged-pointer-strings.html
[arm-physical]: https://lwn.net/Articles/741776/
[x86-physical]: https://en.wikipedia.org/wiki/Intel_5-level_paging
