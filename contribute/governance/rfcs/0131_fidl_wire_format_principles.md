<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0131" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

We describe the current (as of Sept 2021) design principles underpinning the
[FIDL wire format][wire-format].

## Motivation

The FIDL wire format specifies how messages are to be encoded (and decoded), as
well as the format for transport level metadata such as the the transactional
message header. Implicit in the wire format specification are theoretical
boundaries which optimal implementation of it may attain. Just like data
structures imply certain big-O bounds on operations, so does the wire format.

In Fuchsia, interprocess communication (at least control plane) is ubiquitously
over FIDL or intended to be. As a result, the wire format has a significant
impact on the overall target performance of the operating system. Similarly, the
wire format has an important role as part of the many layered defense of privacy
and security.

In March 2017, the design for "FIDL 2.0" was being completed. FIDL 2.0 is a more
static version of FIDL, compared with later developments. See also [RFC-0027:
You only pay for what you use][RFC-0027] for additional historical context.

The goals for the wire format specification were as follows[^1]:

[^1]: Authored by Jeff Brown <jeffbrown@google.com>.

* Efficiently transfer messages between processes.
* General purpose, for use with device drivers, high-level services, and
  applications.
* Optimized for Zircon IPC only; portability is not a goal. (This goal was since
  relaxed.)[^2]
* Optimized for direct memory access; inter-machine transport is not a goal.
* Optimized for 64-bit only; no accommodation for 32-bit environments.
* Uses uncompressed native datatypes _with host-endianness_, first-fit packing of
  elements, and correct alignment to support in-place access of message
  contents.
* Compatible with C structure in-memory layout (with suitable field ordering and
  packing annotations).
* Structures are fixed size and inlined; variable-sized data is stored
  out-of-line.
* Structures are not self-described; FIDL files describe their contents.
* No versioning of structures, but interfaces can be extended with new methods
  for protocol evolution. (This goal was since relaxed.)[^2]
* No offset calculations required, very little arithmetic which may overflow.
* Support fast single-pass encoding and validation (as a combined operation).
* Support fast single-pass decoding and validation (as a combined operation).

[^2]: Some goals have since been relaxed (portability, no versioning of
    structures), or tightened ([endianness][RFC-0030]).

While the the ongoing evolution of the wire format has followed very specific
design principles, some outlined above, these were not necessarily written down
along with rationale. This RFC is an attempt at clearly writing these design
principles down.

## Design

We describe the various design principles underpinning the FIDL wire format.

### Low level first {#low-level-first}

> When faced with making a design tradeoff to support low level programming at
> the expense of high level programming (or the reverse), we typically opt for
> enabling low level programming.

FIDL must satisfy the requirements of low level protocols in Fuchsia, sometimes
used during the boot process when a `malloc` is not yet available for instance.
The alternative, should FIDL not satisfy these requirements, is to manually
design protocols. However, in high level programming, if FIDL is not able to
satisfy the requirements, there are a lot of other options to choose from
(Protobuf, Cap'n Proto, JSON, Yaml, and the like).

### Single pass, and no heap allocation {#single-pass-no-heap}

> It must be possible to encode and decode in a single pass, without allocation
> beyond stack space (i.e. no dynamic heap allocation).

This principle somewhat follows being over specialized towards low level use
cases, and ensuring that any software on the system can fully participate in the
FIDL ecosystem.

Because FIDL provides "decode + validate", the single pass requirement should be
compared to similar systems offering both deserialization and validation, which
is most often done in two passes (with validation occurring on the decoded
form).

A corollary of the no allocation requirement is that encoding and decoding is
done in-place, i.e. with in-place modifications.

### As efficient as hand-rolled data structures

> It must be possible to write an implementation of the wire format which is as
> efficient as hand-rolled data structures.

This is a specialization of the "you pay for what you use" principle, whereby
the convenience and ergonomics that FIDL aims to provide must not be offered at
the expense of performance. In practice, many implementations choose to be less
efficient to provide additional ergonomics, but the wire format does not dictate
this choice.

### Canonical representation

> There must be a single unambiguous representation of a FIDL value, i.e. there
> is one and only one encoded representation of a FIDL value, and one and only
> one decoded representation of a FIDL value.

By forcing a single representation, the wire format is naturally more strict,
which means that implementations have to expect less variance in inputs and
follow a more straight-line path. This helps ensure correctness, through
reduction of surprises coming from data divergences. A canonical form makes it
possible to check for equality of two values without the need to understand the
schema, i.e. a `memcmp` suffices for value types (things are a little [more
complicated for resource
types](/docs/reference/fidl/language/bindings-spec.md#equality-comparison)).

See also the [drawbacks of a canonical form](#drawback-canonical-representation).

### Specify every byte

> When encoding or decoding, it must be possible to traverse every single byte
> of a message in a [single pass and without any heap
> allocation](#single-pass-no-heap).

To ensure that no data leaks from one process to another unbeknownst to the
sender, we both ensure that all bytes can be efficiently traversed, and that all
bytes have a specified value (e.g. padding must be 0). As an example, this can
help to ensure that no personally identifiable information (PII) is
inadvertently shared across process boundaries, or help avoid leaking
uninitialized memory that could contain pointer values, which could be used to
defeat address space layout randomization (ASLR). Another example is considering
"trailing junk" invalid since all data and handles must be accounted for.

### Validation everywhere

> As part of our [defense in depth](/docs/concepts/principles/secure.md), we
> want the FIDL wire format to enforce strict validation (e.g. bound checks,
> strings are well-formed UTF-8 code unit sequences, handles are of the correct
> type and rights) everywhere it is used.

Strict validation is considered worthwhile
in ensuring the security of the platform, and helps API authors state
assumptions and invariants of a design onto the API schema. It is also our
experience that absent of validation in lower layers, applications tend to
validate invariants themselves, leading to code that is less clear, tends to
be less efficient, and more prone to bugs.

Since strict validation can be the source of high performance costs, and that
FIDL is geared towards being used in [low-level](#low-level-first) layers, a
corollary is that such validation must be done efficiently, and designed to fit
in a [single pass](#single-pass-no-heap).

### No reflective functionality out of the box {#avoid-reflection}

> Without explicit opt-in, a peer must not be allowed to perform reflection on a
> protocol, be it exposed methods, or exposed types.

For instance, if a peer calls the wrong FIDL method, the connection is closed,
preventing any information to be extracted about the peer. It might seem
convenient to build such functionality, but that may compromise privacy and be
difficult to undo (users would start building load-bearing functionality off
of this feature).

Similarly, structures lacking a self-descriptive format are in line with this
principle, and meant to avoid disclosing more than necessary in an ecosystem
where interacting peers ought to distrust each other. (There are also
significant performance gains with avoiding a self-descriptive format, which
aligns with the low level first approach.)

As we have changed the FIDL wire format to allow evolution, e.g.
[tables][RFC-0047], we have had to navigate carefully the balance between
forbidding reflection, and adding just enough to allow handling without a
schema.

## Implementation

Keep calm, and follow the principles. As seen in [RFC-0017].

## Performance

Most guiding principles of the FIDL wire format are aimed at performance, and
over specialize towards low level use cases. Performance is a central concern.

## Ergonomics

No change to ergonomics.

## Backwards Compatibility

Some of the principles stated here are in conflict with the primary goal of FIDL
which is providing a foundation for stable ABI, e.g. implementing backwards
compatible protocols is challenging in the absence of reflexive features. Among
other things, the design of the FIDL wire format strikes a balance between
performance (often a result of rigidity) and evolvability concerns (often a
result of flexibility). Balancing these is where the fun lies.

## Security considerations

The role of FIDL in the multi layered approach to security on Fuchsia is
explained in this RFC.

## Privacy considerations

The role of FIDL in the multi layered approach to privacy on Fuchsia is
explained in this RFC.

## Testing

No change to testing.

## Documentation

Amend as needed:

* [FIDL Overview](/docs/concepts/fidl/overview.md)
* [FIDL design principles](/docs/contribute/contributing-to-fidl/design-principles.md)
* [FIDL wire format][wire-format]

## Drawbacks, alternatives, and unknowns

As described in the text.

### Drawbacks of a canonical form {#drawback-canonical-representation}

Requiring a canonicalized form can constrain the problem of finding a good
representation for data, to the point of discarding otherwise interesting or
pursuable forms.

When working on [sparser tables][RFC-0116], canonicalization was one of the
toughest constraints to satisfy, and directly conflicted with the need for the
format to be performant. For instance, we could have explored writing members in
the order provided by the users, without needing a second pass which reorders
those members to satisfy canonicalization requirements.

## Prior art and references

As described in the text.

<!-- link labels -->
[RFC-0017]: 0017_folding_ftp_into_rfc.md
[RFC-0027]: 0027_you_only_pay_what_you_use.md
[RFC-0030]: 0030_fidl_is_little_endian.md
[RFC-0047]: 0047_tables.md
[RFC-0116]: 0116_fidl_sparser_tables.md
[wire-format]: /docs/reference/fidl/language/wire-format/README.md
