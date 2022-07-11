{% set rfcid = "RFC-0113" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: This is a resubmission of the previously rejected
[RFC-0032](0032_efficient_envelopes.md).

_"Turning Envelopes into Postcards"_

## Summary

This RFC proposes a more compact encoding for FIDL[^1].

## Motivation

Envelopes are the foundation for extensible, evolvable data structures
(tables and extensible unions).
A more compact and efficient wire format for envelopes enables those
extensible structures to be used in more contexts where performance
and wire size matter.

## Design

The proposed envelope format can be described as the following C-struct:

```c++
struct Envelope {
    uint32_t byte_size;
    uint32_t handle_count;
};
```

Compared with the [existing envelope format][envelopes]:

*   **The byte size field remains the same (32 bits).**
    *   The size includes the size of any sub-objects that may be recursively
        encoded.
    *   For example, the size of a `vector<string>` includes the size of the
        outer vector's inner string sub-objects.
    *   This matches the existing behavior for the current envelope
        implementation's size field.
*   **The handle count field remains the same (32 bits).**
    *   The handle_count includes the handle count for all recursive
        sub-objects.
*   **The presence/absence field is dropped.**
    *   Presence is represented by a non-zero value in either the size or
        handle_count field.
    *   Absence is represented by the size & handle count fields both being zero.
        *   We call this a _zero envelope_.
        *   A zero envelope is equivalent to
            [`FIDL_ALLOC_ABSENT`][FIDL_ALLOC_ABSENT].
*   **Validation of byte size field**
    * The byte size field MUST be validated to be a multiple of 8.

Decoders MAY overwrite the envelope with a pointer to the envelope data,
assuming they know the static type (schema) of the envelope's contents.
See the [Unknown Data](#unknown-data) section for recommendations on how
to process an envelope if the content's type is unknown.

### C/C++ Struct for Encoded/Decoded Form

The encoded or decoded form of an envelope can be described as a C-union:

```c
typedef union {
  struct {
    uint32_t byte_size;
    uint32_t handle_count;
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
*   If the envelope's handle count is non-zero, a validator MUST either store
    or close each of the handles.
*   A decoder MAY overwrite the unknown envelope with a pointer to the
    envelope's contents, if it wishes to decode in-place.
    *   If a decoder does overwrite the envelope with a pointer, it will lose
        the size & handle count information in the envelope.
        Bindings MAY offer a mechanism for a decoder to save the size &
        handle count information before overwriting the envelope; this
        RFC does not express an opinion on how such a mechanism could work.

## Implementation strategy

**This RFC is a breaking wire format change.**

A complex wire format migration will be undertaken to switch to efficient
envelopes. This wire format change will be combined with other migrations
to reduce the per-feature migration cost.

## Backwards compatibility

The proposed wire format change is API (source) compatible.
Any hand-rolled FIDL code would need to be updated to handle the new wire
format.

**The wire format change is ABI-incompatible**.

## Performance

A performance evaluation was run in a
[CL](https://fuchsia-review.googlesource.com/c/fuchsia/+/534543/5) that
prototypes an efficient envelope implementation. For this test, the input
was a table with all fields set. Other inputs produced similar results.

The following times are in nanoseconds. The time without efficient envelopes is
before the arrow and the time with efficient envelopes is after the arrow.

| # Fields | Encode       | Decode       |
|----------|--------------|--------------|
| 16       | 64   -> 40   | 176  -> 146  |
| 64       | 165  -> 121  | 321  -> 221  |
| 256      | 567  -> 368  | 923  -> 527  |
| 1024     | 2139 -> 1429 | 3284 -> 1636 |

Depending on the input, using efficient envelopes appears to be 1.1-2x faster

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

There should not be any security implications from this RFC.

One minor security advantage is that this RFC removes information that is
otherwise duplicated in the size and pointer in the old format.
Previously, an envelope may be received with non-zero size/handles and
`FIDL_ALLOC_ABSENT`, or zero size/handles and `FIDL_ALLOC_PRESENT`.
This required extra validation checks, which will no longer be needed.

It is not possible to determine if an envelope is in wire form or
decoded form off of the data alone. This is not a problem because in
practice there is always separate bookkeeping in bindings that keeps
track of whether the message is in wire form or decoded form.

## Privacy

There should not be any privacy implications from this RFC.

## Testing

*   Since this RFC is changing the wire format for envelopes, we feel that
    the existing FIDL test suite &mdash; particularly compatibility tests
    &mdash; will adequately test all scenarios where envelopes are used.
*   If we agree to land the wire format change as a soft transition (see the
    [Implementation Strategy](#implementation-strategy) section), we will add
    tests for peers to negotiate and possibly switch to the new wire format.

## Drawbacks, alternatives, and unknowns

We can keep the existing wire format if we believe the efficiency gains in
this proposal are not worth the implementation cost.

## Previous RFC rejection and argument for approving now

*This RFC was previously rejected with the following rationale (copied
here verbatim) before being resubmitted for review:*

> In February 21, 2019, this RFC was initially accepted. The FIDL team worked to
stabilize the wire format for most of 2019, culminating in an all-hands-on-deck
effort which spanned Q3 and Q4. The migration completed on
Dec 1<sup>st</sup>, 2019.
>
> The stabilization effort spanned multiple changes:
>
> * **Major** [RFC-0061: Extensible Unions](0061_extensible_unions.md)
> * [RFC-0032: Efficient Envelopes]
  (0032_efficient_envelopes.md), i.e. this RFC
> * [RFC-0037: Transactional Message Header v3]
  (0037_transactional_message_header_v3.md)
> * [RFC-0048: Explicit Union Ordinals](0048_explicit_union_ordinals.md)
>
> However, as the work unfolded, and the Dec 1<sup>st</sup> deadline loomed, the
FIDL team decided to punt on implementing the efficient envelopes change,
preferring to push this work to 2020. Unlike the other changes which were part
of the stabilization effort, efficient envelopes was simply an in-memory size
saving, which was very small, especially when compared to other aspects of the
FIDL wire format (e.g. [tables' dense format](0047_tables.md#dense-vs-sparse)).
Deferring was a project risk reduction calculation, by reducing the scope, the
odds of completing all the work on time were improved. So was the FIDL team's
work schedule.
>
> We're now close to 18 months after the deferral, and efficient envelopes are
long forgotten. Significant performance work in 2020 demonstrated that this
change would have no material impact.
>
> It's time to face the truth, this ain't going to happen. Rejected.

### Why re-approve now?

The FIDL team is currently planning to batch together several wire format
changes and undergo a migration with all of them at once. This means there is
opportunity to add support for efficient envelopes in a lower cost way (in that
the cost is shared with other migrations).

Additionally, there are now concrete numbers for the performance gains due to
efficient envelopes and the gains are significant.

Because of these factors, this is an opportune time to resurrect this RFC and
implement it.

## Prior art and references

This RFC is a slimmed-down version of [rfc-0026], which was rejected since
there wasn't enough consensus around the whole RFC.

[^1]: This RFC is based on [rfc-0026], but with _only_ the out-of-line envelope
    proposal. Inlining, envelopes everywhere, and moving the string/vector count
    out-of-line, have all been removed.

<!-- xrefs -->
[envelopes]: /contribute/governance/rfcs/0047_tables.md#envelopes
[rfc-0061]: /contribute/governance/rfcs/0061_extensible_unions.md
[rfc-0026]: /contribute/governance/rfcs/0026_envelopes_everywhere.md
[rfc-0045]: /contribute/governance/rfcs/0045_zero_size_empty_structs.md
[FIDL_ALLOC_ABSENT]: /zircon/system/public/zircon/fidl.h
[messageheader]: /reference/fidl/language/wire-format/README.md#transactional-messages
[cognitivechunking]: https://en.wikipedia.org/wiki/Chunking_(psychology)
