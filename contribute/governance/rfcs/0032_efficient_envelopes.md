{% set rfcid = "RFC-0032" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: A version of this RFC was resubmitted and accepted as
[RFC-0113](0113_efficient_envelopes.md)

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-032.

_"Turning Envelopes into Postcards"_

## Rejection rationale

On February 21, 2019, this RFC was initially accepted. The FIDL team worked to
stabilize the wire format for most of 2019, culminating in an all-hands-on-deck
effort which spanned Q3 and Q4. The migration completed on Dec 1<sup>st</sup>,
2019.

The stabilization effort spanned multiple changes:

* **Major** [RFC-0061: Extensible Unions](/docs/contribute/governance/rfcs/0061_extensible_unions.md)
* [RFC-0032: Efficient Envelopes](/docs/contribute/governance/rfcs/0032_efficient_envelopes.md), i.e. this FTP
* [RFC-0037: Transactional Message Header v3](/docs/contribute/governance/rfcs/0037_transactional_message_header_v3.md)
* [RFC-0048: Explicit Union Ordinals](/docs/contribute/governance/rfcs/0048_explicit_union_ordinals.md)

However, as the work unfolded, and the Dec 1<sup>st</sup> deadline loomed, the
FIDL team decided to punt on implementing the efficient envelopes change,
preferring to push this work to 2020. Unlike the other changes which were part
of the stabilization effort, efficient envelopes was simply an in-memory size
saving, which was very small, especially when compared to other aspects of the
FIDL wire format (e.g. [tables' dense format](/docs/contribute/governance/rfcs/0047_tables.md#dense-vs-sparse)).
Deferring was a project risk reduction calculation, by reducing the scope, the
odds of completing all the work on time were improved. So was the FIDL team's
work schedule.

We're now close to 18 months after the deferral, and efficient envelopes are
long forgotten. Significant performance work in 2020 demonstrated that this
change would have no material impact.

It's time to face the truth, this ain't going to happen. Rejected.

### Relation to other RFCs

In June 2021, this topic was revisited and performance was measured with
targeted benchmarks. This was conclusive and
[RFC-0113](0113_efficient_envelopes.md) proposed reintroducing the change, which
was accepted.

## Summary

This FTP proposes a more compact encoding for envelopes[^1].

## Motivation

Envelopes are the foundation for extensible, evolvable data structures
(tables and extensible unions).
A more compact and efficient wire format for envelopes enables those
extensible structures to be used in more contexts where performance
and wire size matter.

## Design

The proposed envelope format is:

![Figure: 64 bit little endian word, MSB 32 bits size, 16 bits handle_count,
16 bits reserved](resources/0032_efficient_envelopes/figure1.png)

Compared with the [existing envelope format][envelopes]:

*   **The size field remains the same (32 bits).**
    *   The size includes the size of any sub-objects that may be recursively
        encoded.
    *   For example, the size of a `vector<string>` includes the size of the
        outer vector's inner string sub-objects.
    *   This matches the existing behavior for the current envelope
        implementation's size field.
*   **16 bits are reserved.**
    *   Decoders MUST validate that the reserved bits are zero.
    *   If we wish to use a reserved bit in the future, we should revise the
        wire format instead.
        *   Reserved bits should be thought about for FIDL more holistically,
            so that behavior is consistent across specifications.
        *   In particular, there is no precedent in FIDL for decoders to
            ignore any bits: all bits on the wire are defined and specified.
        *   This decision is the simplest one &mdash; require a wire format
            change instead of enabling forward compatibility &mdash; to keep
            things simple until a policy about reserved bits is decided on.
*   **The handle_count is 16 bits, instead of 32 bits.**
    *   It's not currently possible to send > 64 handles over a Zircon
        channel; we feel that 16 bits provides enough headroom for future needs.
    *   The handle_count includes the handle count for all recursive sub-objects.
*   **The presence/absence field is dropped.**
    *   Presence is represented by a non-zero value in either the size or
        handle_count field.
    *   Absence is represented by the size & handle count fields both being zero.
        *   We call this a _zero envelope_.
        *   A zero envelope is equivalent to [`FIDL_ALLOC_ABSENT`][FIDL_ALLOC_ABSENT].
*   **A size of `UINT32_MAX` and handle count of `0` is special: it
    represents envelope content that is present, but has zero size.**
    *   This is reserved for future use if [zero-size empty structs][rfc-0045]
        become a reality[^2], and does not impose any performance
        or complexity penalty on decoders today.
        We wish to mention this now so that a possible future implementation
        does not break the wire format.
    *   We could steal one of the reserved bits instead.
        We don't have a strong opinion about this; as long as there's some
        way to distinguish a "present but zero-size" envelope from
        `FIDL_ALLOC_ABSENT`, that's OK.
        Happy to go with consensus.

Decoders MAY overwrite the envelope with a pointer to the envelope data,
assuming they know the static type (schema) of the envelope's contents.
See the [Unknown Data](#unknown-data) section for recommendations on how
to process an envelope if the content's type is unknown.

### C/C++ Struct for Encoded/Decoded Form

The encoded form of an envelope can be represented by a union of the encoded
or decoded form.

```c
typedef union {
  struct {
    uint32_t size;
    uint16_t handle_count;
    uint16_t reserved;
  } encoded;
  void* data;
} fidl_envelope_t;

static_assert(sizeof(fidl_envelope_t) == sizeof(void*));
```

### Unknown data

Receivers &mdash; validators & decoders &mdash; may not know the type of an
envelope when they're used in an evolvable data structure.
If a receiver doesn't know the type, an envelope can be minimally parsed
and skipped.

*   The envelope's size determines the amount of out-of-line data to skip.
*   If the envelope's handle count is non-zero, a validator MUST process
    the specified number of handles.
    *   The default processing behavior MUST be to close all handles.
*   A decoder MAY overwrite the unknown envelope with a pointer to the
    envelope's contents, if it wishes to decode in-place.
    *   If a decoder does overwrite the envelope with a pointer, it will lose
        the size & handle count information in the envelope.
        Bindings MAY offer a mechanism for a decoder to save the size &
        handle count information before overwriting the envelope; this
        FTP does not express an opinion on how such a mechanism could work.

## Implementation strategy

**This FTP is a breaking wire format change.**

Both FIDL peers need to understand the new envelope format &mdash; and
communicate that understanding to its peer &mdash; for both parties to use
the new format.
As such, this would typically be considered as a hard transition.
**Since this FTP adds no new functionality, if we decide to land this as a
hard transition, the authors recommended that this change is grouped with
other wire format changes (e.g. a proposed ordinal size change).**

That said, a soft transition is possible.
Two approaches are:

1. There is a `uint32` reserved/flags field in the [transactional message
   header][messageheader].
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
   "`[WireFormat=EnvelopeV2]`" attribute (or similar) that indicates that the
   message/interface should use the new wire format.
    1. While decorating an interface with a WireFormat attribute seems to
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
Given that the code to handle both wire formats will exist, **we recommend
prototyping whether a soft transition is feasible using one of the above soft
transition approaches.
Such prototyping work may also lead to general strategies for landing
future breaking wire format changes, which may be valuable.**
If not, *c'est la vie*; hard transition it is.

For either a soft or hard transition, any instances in Fuchsia where FIDL
messages are hand-rolled would need to also be upgraded to the new wire
format.

## Backwards compatibility

The proposed wire format change should be API (source) compatible.
Any hand-rolled FIDL code would need to be updated to handle the new wire
format.

**The wire format change is ABI-incompatible**.
It may be possible to achieve ABI compatibility via the strategies outlined
in the [Implementation Strategy](#implementation-strategy) section.

## Performance

This FTP significantly shrinks the size required for envelopes, which seems
like it would be an overall significant net benefit.
However, if extensible data structures become more pervasive due to their
better efficiency, this may be outweighed by their increased usage, which may
result in less compact messages overall and more dynamic allocation, vs. using
non-extensible data structures.

## Ergonomics

*   More efficient extensible data structures enable them to be used
    in more contexts where efficiency matters, so users need to worry
    less about their performance, and can gain the benefits of extensibility
    where they would previously need to use non-extensible structures.
*   We may even wish to recommend that tables should be used by default for
    FIDL data structures, and structs should be reserved for high-performance
    contexts.
    *   Extensible unions ([RFC-0061][rfc-0061]) are already attempting to
        remove static unions.

## Documentation

*   The wire format documentation needs to be updated.
*   When updating the documentation, envelopes should be explained as a
    first-class concept: this enables better [cognitive
    chunking][cognitivechunking] once readers encounter the wire format for
    optionality and extensible data structures.
*   We should update the FIDL style guide to make recommendations for when
    extensible types should be used.

## Security

There should no significant security implications from this FTP.

One minor security advantage is that this FTP removes information that is
otherwise duplicated in the size and pointer in the old format.
Previously, an envelope may be received with non-zero size/handles and
`FIDL_ALLOC_ABSENT`, or zero size/handles and `FIDL_ALLOC_PRESENT`.
This required extra validation checks, which will no longer be needed.

## Testing

*   Since this FTP is changing the wire format for envelopes, we feel that
    the existing FIDL test suite &mdash; particularly compatibility tests
    &mdash; will adequately test all scenarios where envelopes are used.
*   If we agree to land the wire format change as a soft transition (see the
    [Implementation Strategy](#implementation-strategy) section), we will add
    tests for peers to negotiate and possibly switch to the new wire format.

## Drawbacks, alternatives, and unknowns

We can keep the existing wire format if we believe the efficiency gains in
this proposal are not worth the implementation cost.

## Design Decisions

While this FTP makes recommendations, we are actively seeking input and
consensus on the following decisions:

*   Do we want to consider a soft transition or a hard transition? See the
    [Implementation Strategy](#implementation-strategy) section for pros & cons.
*   We propose using 32 bits for size, 16 bits for handles, and reserving
    16 bits.
    *   Is 32 bits for size reasonable?
    *   Is 16 bits for handles reasonable?
*   [rfc-0026], which this proposal is derived from, proposed inlining data
    directly into the envelope for types that are <= 32 bits.
    *   We decided to withdraw inlining from this proposal since it adds
        significant implementation complexity, and provide marginal benefit
        unless there are a large number of fields that could be inlined.
    *   There is work-in-progress to think about optionality more
        holistically, e.g. by grouping optional fields into a single
        optional struct.
        Such work may obsolete any benefits that inlining may bring.

## Prior art and references

This FTP is a slimmed-down version of [rfc-0026], which was rejected since
there wasn't enough consensus around the whole FTP.

[^1]: This FTP is based on [rfc-0026], but with _only_ the out-of-line envelope
proposal.
Inlining, envelopes everywhere, and moving the string/vector count
out-of-line, have all been removed.

[^2]: Note that today, empty (zero-field) structs occupy one byte on-the-wire.

<!-- xrefs -->
[envelopes]: /docs/contribute/governance/rfcs/0047_tables.md#envelopes
[rfc-0061]: /docs/contribute/governance/rfcs/0061_extensible_unions.md
[rfc-0026]: /docs/contribute/governance/rfcs/0026_envelopes_everywhere.md
[rfc-0045]: /docs/contribute/governance/rfcs/0045_zero_size_empty_structs.md
[FIDL_ALLOC_ABSENT]: /zircon/system/public/zircon/fidl.h
[messageheader]: /docs/reference/fidl/language/wire-format/README.md#transactional-messages
[cognitivechunking]: https://en.wikipedia.org/wiki/Chunking_(psychology)
