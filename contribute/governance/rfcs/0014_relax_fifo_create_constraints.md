{% set rfcid = "RFC-0014" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

`zx_fifo_create` currently requires the caller to pass an element count that is a power of two.
This allows the kernel to make a small optimization on the read and write paths by using a
bit-mask instead of the modulus operator to determine the offset of a wrapped record in a buffer.

This constraint means that users cannot effectively use the FIFO to its full capacity if their
element sizes do not happen to be `4096/(2**n)` bytes; in these cases the FIFO capacity
(`element_size * element_count`) will be less than possible full capacity of the FIFO (which
internally uses a `4096`-byte buffer).

The proposal is to allow FIFOs to be created with arbitrary element counts (up to the FIFO size
limit) so the full FIFO capacity can be used for FIFO records of arbitrary sizes.

## Motivation

This proposal is motivated by the desire to add a field to an existing FIFO record used in the
block stack; the field in question is a trace identifier, which will be used to support
cross-process tracing of block IO. Adding this field changes the size of the FIFO records from
`32` bytes to `40` bytes.

If FIFO records are `40` bytes long, then the maximum value that `element_count` could currently
be set to is `64`, which results in `40 * 64 = 2560` bytes used in the `4096`-byte buffer.

This RFC would permit an `element_count` value of `102`, which results in `40 * 102 = 4080` bytes
used (almost doubling the usable capacity of the FIFO).

## Design

This is a trivial change that is implemented in
<https://fuchsia-review.googlesource.com/c/fuchsia/+/409498>.

## Implementation

This is a trivial change that is implemented in
<https://fuchsia-review.googlesource.com/c/fuchsia/+/409498>.

## Performance

The kernel can currently make a small performance optimization since it knows the element count
is a power of two; particularly, the kernel can use a bitwise-AND to determine the position of an
element in the internal buffer, instead of a modulo.

That said, this performance optimization washes out given the expense of jumping back and forth
between the kernel and userspace.

Concretely: performance testing in <https://fuchsia-review.googlesource.com/c/fuchsia/+/409498>
indicates that there are no measurable performance costs for eliminating this optimization.

## Security considerations

None.

## Privacy considerations

None.

## Testing

This is a trivial change for which tests are added in
<https://fuchsia-review.googlesource.com/c/fuchsia/+/409498>.

## Documentation

The `zx_fifo_create` documentation is adjusted in
https://fuchsia-review.googlesource.com/c/fuchsia/+/409498 based on the new relaxed constraints.

## Drawbacks, alternatives, and unknowns

None.

## Prior art and references

None.
