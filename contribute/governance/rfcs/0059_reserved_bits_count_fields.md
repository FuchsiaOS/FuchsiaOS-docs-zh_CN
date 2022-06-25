{% set rfcid = "RFC-0059" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-059.

## Summary

FIDL vectors (and strings) use a 64-bit count field to represent the number of
encoded bytes. This FTP proposes a few changes:

* Vector, string and array counts max out at 32-bits (max value 2<sup>32</sup>-1).

* The upper 8-bits of vector/string count fields are reserved for use by
  bindings for in-memory use. They must be filled with zero on the wire.

* The next 24-bits of vector/string count fields are reserved but are unused and
  must be filled with zero on the wire. They cannot be used by bindings and may
  be reallocated in a future FTP.

A visualization of the breakdown of the existing 64-bit count field.

![Visualization: 32-bits for count, 24 bits unused, 8 bits reserved for
bindings](resources/0059_reserved_bits_count_fields/bits.png)

This would decrease the maximum vector, string and array encoding size from
18.45 exabytes to 4.29 gigabytes.

## Motivation

The LLCPP bindings enable in-place encoding of objects. As part of the LLCPP
Builder effort, the bindings now keep track of memory ownership to simplify
object creation for users. In particular, for vectors, the most significant bit
(MSB) of the count field is used to store ownership information (see
[vector_view.h]).
This bit is zeroed during linearization and doesn't affect the FIDL wire format,
but it does prevent the LLCPP bindings from using the MSB for count values.

The goal of this is to formalize the reservation of the MSB of the vector's
count field for use by bindings and extend the reservation to the upper 8-bits.

Separately, there is some inconsistency in maximum counts between bindings.
Various parts of the C++ language bindings and compiler assume a maximum size of
32-bits, but this size was never formalized and isn't followed by other
bindings. This FTP formalizes a maximum size of 32-bits for vector/string/array
counts.

## Design

The upper 8-bits of the 64-bit vector (and string) count field are reserved for
in-memory use by bindings. The upper 8-bits MUST be zero when sent on the wire.

In addition, the next uppermost 24 bits (bits 32-55) are reserved and must be
unused by the bindings. They also MUST be zero on the wire.

The ABI is unchanged.

## Implementation Strategy

Encode and decode logic in each of the bindings will be updated to validate that
the count is at most 2^32-1. This includes validating that the upper bits are
zero. The channel will be closed if this constraint is violated during decoding.

## Ergonomics

N/A

## Documentation and Examples

The wire format documentation will be updated to show the reserved bits.

## Backwards Compatibility

Channels max out at a byte size of 65536 (16-bits) per message so parts of the
system that FIDL-encode into channel messages won't have any compatibility
issues. Additionally, a vector/string/array max count of 2<sup>32</sup>-1 is already
assumed by some parts of the code.

## Performance

There is negligible performance impact. The only implementation change is
additional validation checks.

## Security

This should not introduce any security risks. The count field will have the same
value over the wire and there are now additional validation checks.

## Testing

Each binding implementation is responsible for testing its implementation of the
feature, including testing validation checks.

## Drawbacks

After this FTP, bindings are able to use the upper 8 bits of the count field for
any purpose. Therefore, it is difficult or impossible to reclaim those bits for
a new purpose - it would require exhaustively validating that each binding
leaves the bits unused and potentially migrating them away from using these
bits.

## Alternatives

Instead of storing vector ownership information in the count field, it could
instead be stored inside of the pointer field of the vector.

Two possible approaches:

* Assume >= 2 byte alignment and store an ownership boolean in the
  least-significant-bit. The problem with this approach is that the assumption
  of 2-byte alignment is hard to enforce and often not true. There are places in
  the codebase where bytes are read from arbitrary offsets in a buffer.

* Use properties of the address space to assign unused bits to hold memory
  ownership information. The problem with this is that there are no guarantees
  that bits are unused in the memory space. Even if this is true today, tools
  like address sanitizer tend to store information in available bits in pointers
  and there could be a collision between those tools and LLCPP.

Separately, there was discussion around the number of bits to reserve for
bindings and how large the vector count should be. An argument for a larger
vector count is that it is hard to predict future needs and other systems have
run into issues with arbitrary size limitations. That said, there are few
realistic use cases for writing a FIDL object greater than 4.29 gb. Encoding or
decoding an object with this many elements would take over a second under the
outrageous assumption of 1/cycle per element on current CPUs. Large primitive
arrays, which can be encoded more efficiently should generally be sent with
alternative mechanisms such as VMOs rather than FIDL. Still, reserving too many
bits for bindings might limit future use cases. As a compromise, 8 bits are
reserved for bindings, 24 bits unused and 32 bits for the count. This allows for
future changes as needs arise.

## Prior Art and References

N/A

<!-- xrefs -->
[vector_view.h]: https://fuchsia.googlesource.com/fuchsia/+/729dc895768a8064dc04d42171c09402a9816f09/zircon/system/ulib/fidl/include/lib/fidl/llcpp/vector_view.h
