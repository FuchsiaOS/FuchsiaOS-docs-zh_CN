# Fuchsia Device Interface Rubric

The Fuchsia device interfaces are expressed as FIDL protocols.  These FIDL
definitions should conform to the [FIDL Style Rubric][fidl-style] and
[FIDL API Rubric][fidl-api].

## Identifiers

Prefer descriptive identifiers.  If you are using domain-specific abbreviations,
document the expansion or provide a reference for further information.

Every identifier that is defined as part of a protocol must be documented with
a comment explaining its interpretation (in the case of fields, types, and
parameters) or behavior (in the case of methods).

## Protocols

All device interface protocols must use the `[Layout = "Simple"]` attribute.  This
restriction exists to allow ease of implementing protocols in any of our
supported languages for driver development.

## Method Statuses

Use a `zx.status` return to represent success and failure.  If a method should not be
able to fail, do not provide a `zx.status` return.  If the method returns multiple
values, the `zx.status` should come first.

## Arrays, Strings, and Vectors

All arrays, strings, and vectors must be of bounded length.  For arbitrarily
selected bounds, prefer to use a `const` identifier as the length so that
protocol consumers can programmatically inspect the length.

## Enums

Prefer enums with explicit sizes (e.g. `enum Foo : uint32 { ... }`) to plain
integer types when a field has a constrained set of non-arithmetic values.

## Bitfields

If your protocol has a bitfield, represent its values using `bits` values.
For details, see the ["bits"][bits] topic in the readability rubric.

## Non-channel based protocols

Some interface protocols may negotiate a non-channel protocol as a performance
optimization (e.g. the zircon.ethernet.Device's GetFifos/SetIOBuffer methods).
FIDL does not currently support expressing these protocols.  For now, represent
any shared data structures with `struct` definitions and provide detailed
documentation about participation in the protocol.  Packed structures are not
currently supported.

[fidl-style]: /docs/development/languages/fidl/guides/style.md
[fidl-api]: /docs/concepts/api/fidl.md
[bits]: /docs/concepts/api/fidl.md#bits
