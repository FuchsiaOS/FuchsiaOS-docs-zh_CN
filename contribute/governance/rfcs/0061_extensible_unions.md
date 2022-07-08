{% set rfcid = "RFC-0061" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-015.

_"Catering to Hawaii and Alaska"_

## Summary

To provide more ways to express payloads whose shape may need to evolve
over time, we propose to **replace unions as they exist today with
extensible unions**.

# Motivation

Today, unions provide no way to evolve over time, and we even warn that
"in general, changing the definition of a union will break binary
compatibility."

There are a number of unions defined today where extensibility is
necessary, e.g.,
[fuchsia.modular/TriggerCondition][triggercondition],
where fields are deprecated without being removed, or
[fuchsia.modular/Interaction][interaction].

As described [later](#drawbacks_alternatives_and-unknowns),
there also many unions whose current representation is
appropriate as they are unlikely to evolve in the near-future. However,
keeping both `static unions` and `extensible unions` introduces
unneeded complexity, see the
[pros and cons](#pros-and-cons-of-keeping-static-unions).

# Design

To introduce extensible unions, we need to modify multiple parts of FIDL:
the language and `fidlc`, the JSON IR, the wire format and all language
bindings.
We'll also need to document this new feature in various places.
We discuss each change one by one.

## Language

Syntactically, extensible unions look exactly the same as static unions:

```fidl
union MyExtensibleUnion {
    Type1 field1;
    Type2 field2;
     ...
    TypeN fieldN;
}
```

Behind the scenes, each field is assigned an ordinal: this is comparable
to how [tables](/contribute/governance/rfcs/0047_tables.md) have
ordinals for each field, and how
[methods' ordinals](/contribute/governance/rfcs/0020_interface_ordinal_hashing.md)
get automatically assigned.

Specifically:

- Ordinals are **calculated using the same algorithm as method ordinals**
  ([details](/contribute/governance/rfcs/0020_interface_ordinal_hashing.md#hash)),
  we concatenate the library name,
  "`.`", the extensible union name, "`/`", and finally the member name,
  then take the SHA256, and mask with `0x7fffffff`.
- Ordinals are `uint32`, **no two fields can claim the same ordinal**,
  and we **disallow `0`**.
  In the case of ordinal conflict, the `[Selector]` attribute
  should be used to provide an alternate name (or the member renamed).
- Ordinals **can be sparse**, i.e., unlike how tables work, which require
  dense ordinals.
- **Nullable fields are not allowed** on extensible unions.
- Extensible unions **MUST have at least one member**.

An extensible union can be used anywhere a union can currently be used in
the language.
Particularly:

- Structs, tables and extensible unions can contain extensible unions;
- Extensible unions can contain structs, tables and extensible unions;
- Interface arguments or returns can be extensible unions;
- Extensible unions can be nullable.

## JSON IR

Following tables, we will add one key in each union field declaration
"ordinal."

## Wire format

On the wire, an extensible union is represented by the ordinal to
discriminate amongst the choices (padded to 8 bytes), followed by an
envelope of the various members known to the producer.
Specifically, that is:

* A `uint32` **tag** which contains the ordinal of the member being encoded;
* A `uint32` **padding** to align to 8 bytes;
* A `uint32` **num_bytes** storing the number of bytes in the envelope,
  always a multiple of 8, and must be 0 if the envelope is null;
* A `uint32` **num_handles** storing the number of handles in the envelope,
  and must be 0 if the envelope is null;
* A `uint64` **data** pointer to indicate presence (or absence) of
  out-of-line data:
    * `0` when envelope is null;
    * **FIDL_ALLOC_PRESENT** (or **UINTPTR_MAX**) when envelope is present,
      and next out-of-line object;
* When decoded for consumption, this **data** pointer is either **nullptr**
  if envelope is null, or a **valid pointer** to the envelope otherwise.
* The envelope reserves storage for the handles immediately following the
  content.

A **nullable extensible union** has a **tag of 0**, **num_bytes is set to 0**,
**num_handles is set to 0**, and the **data pointer is FIDL_ALLOC_ABSENT**,
i.e., **0**.
Essentially, a null extensible union is 24 bytes of 0s.

## Language Bindings

Extensible unions are similar to unions, except that one needs to also
handle an "unknown" case when union is read.
Ideally, most language bindings would treat

```fidl
union Name { Type1 field1; ...; TypeN fieldN; };
```

as they would an extensible union, such that code can easily be switched
from one to the other, modulo support of the unknown case, which is
meaningful only in the extensible union case.

To start, we suggest no language bindings expose reserved members: while
these are present in the JSON IR for completeness, we do not expect that
exposing them in language bindings be useful.

# Implementation strategy

Implementation will be done in two steps.

First, we will build support for extensible unions:

1. Introduce the feature in the language (`fidlc`), by using a different
   keyword (`xunion`) to distinguish between static unions and extensible
   unions.
2. Implement the various core language bindings (C, C++, Rust, Go, Dart).
   Extend the compatibility test, and other tests accordingly.

Second, we will migrate all static unions to extensible unions:

1. Generate ordinals for static unions, and place them in the JSON IR.
   Backends should initially ignore those.

2. On read paths, have both modes of reading unions, as if they were
   static unions, and as if they were extensible unions (ordinals are needed
   for that to be possible).
   Choose between one and the other based on a flag in the transaction
   message header.

3. Update write paths to encode unions as extensible unions, and indicate
   as much by setting the flag in the transaction message header.

4. When all writers have been updated, deployed, and propagated, remove
   static union handling, and scaffolding code for the soft transition.

# Documentation and examples

This would require documentation in at least these places:

* [Language Specification][fidl-language]: add a section under "Types and Type
  Declarations;"
* [Wire Format Specification][fidl-wire-format]: add a section under "Data
  Types;"

# Backwards compatibility

An extensible union is explicitly **not** backwards compatible with a "static"
union.

# Performance

No impact on performance when not used.
Negligible performance impact during build time.

# Security

No impact on security.

# Testing

Unit tests in the compiler, unit tests for encoding/decoding in various
language bindings, and compatibility test to check various language
bindings together.

# Drawbacks, alternatives, and unknowns

Extensible unions are less efficient than non-extensible unions.
Furthermore, non-extensible unions are not expressible through other means
in the language.
As such, we propose both features living side by side.

However, we could decide that only extensible unions should exist, and do
away with unions as currently defined.
This would go against various places in Fuchsia where unions represent
performance critical messages, and where there is little extension
expectation, e.g. `fuchsia.io/NodeInfo`, `fuchsia.net/IpAddress`.

## Pros and Cons of Keeping Static Unions {#pros-and-cons}

### Pros

* Compared to a union, an extensible union incurs an 8 byte cost (for the
  size of the envelope, and number of handles).
  Additionally, extensible unions' data is always stored out-of-line
  (i.e., an additional 8 bytes for the data pointer), whereas only
  nullable unions' data are stored out-of-line.
* Because of the encoding of unions, it is not possible to express them with
  other primitives in FIDL.
  As such, should they be removed from the language, some classes of messages
  could not be expressed anymore as compactly and efficiently.
* In some cases, and depending on their use, unions can be represented as
  efficiently but differently; however, that is the exception not the norm.
  One example that could be rewritten without using union is the
  [fuchsia.net.stack/InterfaceAddressChangeEvent][interface1]
  used only in the
  [fuchsia.net.stack/InterfaceAddressChange][interface2]
  where the **InterfaceAddress** could directly be written, with an `enum`
  to indicate whether it is added or removed.

### Cons

* Keeping both static unions and extensible unions forces complexity in the
  compiler, the JSON IR, all backends, as well as encoding/decoding.
  The gains are minimal: the size difference is marginal, in a world where FIDL
  encoding is not particularly size efficient in the first place.
  Furthermore, decoding of extensible unions can be done in place if needed.
* As an example of how minimal the gains are, here is the analysis for
  fuchsia.io/NodeInfo:
    * Today **NodeInfo** has 6 options: service (size 1), file (size 4),
      directory (size 1), pipe (size 4), vmofile (size 24), device (size 4).
    * As such, the total size of a **NodeInfo** is always 32 bytes, i.e.,
      tag + max(size of options) = 8 + 24 = 32.
    * With extensible unions, **NodeInfo** size would depend on the option
      being encoded.
      There is always a 16 byte 'tax' (vs. 8), so the respective sizes would
      be: service = 24, file = 24, directory = 24, pipe = 24, vmofile = 40,
      device = 24.
    * So, in all cases, we're shaving off 8 bytes, except in the case of a
      vmofile where we are adding an additional 8 bytes.
* The complexity in the language of having both static unions and extensible
  unions is also a worry.
  We expect library authors to waver between using one vs the other, when
  choosing extensible unions is a safer long term choice, for very little cost.

All in all, we decided to replace static unions with extensible unions.

## Tag vs Ordinal

We use **ordinal** to denote the internal numeric value assigned to fields,
i.e., the value calculated through hashing.
We use **tag** to denote the representation of the variants in bindings:
in Go this may be constants of a type `alias`, in Dart this may be an `enum`.

The `fidlc` compiler deals with ordinals only.
Developers would most likely deal with tags only.
And bindings provide translation from the high-level tag, to the low-level
internal ordinal.

## No Empty Extensible Unions

During the design phase, we considered having extensible unions be empty.
However, we chose to disallow that in the end: choosing a nullable
extensible union with a single variant (e.g., an empty struct) clearly
models the intent.
This also avoids having two "unit" values for extensible unions i.e., a
null value and an empty value.

# Prior art and references

* Protocol buffers has [oneof].
* FlatBuffers's [unions][flatbufferunion] aren't extensible except under special circumstances.

<!-- xrefs -->
[triggercondition]: https://fuchsia.googlesource.com/fuchsia/+/0d83b07e19d055d609715b77750a8a4009b593f7/public/fidl/fuchsia.modular/agent/agent_context.fidl#54
[interaction]: https://fuchsia.googlesource.com/fuchsia/+/4d82bce3c69970f305a0583ebe96a2a821cba8a8/public/fidl/fuchsia.modular/suggestion/suggestion_provider.fidl#123
[fidl-language]: /reference/fidl/language/language.md
[fidl-wire-format]: /reference/fidl/language/wire-format/README.md
[interface1]: https://fuchsia.googlesource.com/fuchsia/+/17bec424c1c24d1a9a41410108c4e018be8ac5e7/system/fidl/fuchsia-net-stack/stack.fidl#114
[interface2]: https://fuchsia.googlesource.com/fuchsia/+/17bec424c1c24d1a9a41410108c4e018be8ac5e7/system/fidl/fuchsia-net-stack/stack.fidl#119
[oneof]: https://developers.google.com/protocol-buffers/docs/proto#oneof
[flatbufferunion]: https://google.github.io/flatbuffers/flatbuffers_guide_writing_schema.html
