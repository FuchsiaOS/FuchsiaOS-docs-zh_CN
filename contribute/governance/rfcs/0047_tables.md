{% set rfcid = "RFC-0047" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-007.

## Summary

Add a mechanism for forward and backwards compatible compound data types to the FIDL language.

### Relation to other RFCs

This RFC was later amended by:

* [RFC-0116: Wire format support for sparser FIDL tables](0116_fidl_sparser_tables.md)
* [RFC-0132: FIDL table size limit](0132_fidl_table_size_limit.md)

## Motivation {#motivation}

FIDL structs provide no mechanism to mutate the schema over time.
Tables are similar to structs, but add ordinals to each field to allow structure evolution:

- New fields can be added and ignored by existing code
- Old (deprecated) fields can be skipped over by newer code

Tables are necessarily more complicated than structs, and so processing them will
be slower and serializing them will use more space.
As such, it's preferred to keep structs as is and introduce something new.

Additionally, having an evolvable schema opens the way to having a
variant of FIDL that can be sensibly serialized to disk or over a network.

An example table might look like:

```fidl
table Station {
    1: string name;
    3: bool encrypted;
    2: uint32 channel;
};
```

## Design

### Source language

Add the **table_declaration** to the FIDL grammar:

```
declaration = const-declaration | enum-declaration | interface-declaration |
              struct-declaration | union-declaration | table-declaration ;

table-declaration = ( attribute-list ) , "table" , IDENTIFIER , "{" , ( table-field , ";" )* , "}" ;

table-field = table-field-ordinal , table-field-declaration ;

table-field-ordinal = ordinal , ":" ;

table-field-declaration = struct-field | "reserved" ;
```

Notes:

- Ordinals must start at 1 and no gaps are allowed in the ordinal space (if the largest
  ordinal is 7, then all of 1,2,3,4,5,6,7 must be present).
- No two fields can claim the same ordinal.
- A "reserved" field is dropped by the compiler after checks for ordinal clashes have occurred.
  It allows annotation that a field was used in some previous version of the table but was
  dropped, so that future revisions do not accidentally reuse that ordinal.
- Nullable fields are **not allowed** on tables.

A table can be used anywhere a struct can currently be used in the language.
Particularly:

- structs and unions can contain tables
- tables can contain structs and unions
- interface arguments can be tables
- tables can be made optional

### Wire format {#wire-format}

Tables are stored as a packed `vector<envelope>`,
each element of the vector is one ordinal element (so index 0 is ordinal 1,
index 1 is ordinal 2, etc.).
We describe envelopes below.

A table must only store envelopes up to the last present one, i.e. the maximal set ordinal.
This ensures a canonical representation. For instance, if no field is set, the correct
encoding is an empty vector.
For a table with field at ordinal 5, but with fields set only up to ordinal 3, the correct
encoding is a vector of 3 envelopes.

#### Envelopes {#envelopes}

An `envelope` stores a variable size, uninterpreted payload out-of-line.
The payload may contain an arbitrary number of bytes and handles.
This organization allows for encapsulation of one FIDL message inside of another.

Envelopes are stored as a record consisting of:

* `num_bytes` : 32-bit unsigned number of bytes in envelope, always a multiple of 8,
  must be zero if envelope is null
* `num_handles` : 32-bit unsigned number of handles in envelope,
  must be zero if envelope is null
* `data` : 64-bit presence indication or pointer to out-of-line data

The `data` field has two different behaviors.

When encoded for transfer, `data` indicates presence of content:

* `FIDL_ALLOC_ABSENT` (all 0 bits): envelope is null
* `FIDL_ALLOC_PRESENT` (all 1 bits): envelope is non-null,
  data is the next out-of-line object

When decoded for consumption, `data` is a pointer to content.

* `0` : envelope is null
* `<valid pointer>` : envelope is non-null, data is at indicated memory address

For handles, the envelope reserves storage for the handles immediately following the content.
When decoded, assuming `data` is not null, `data` points to the first byte of data.

The envelope is padded to the next 8 byte object alignment (which, in practice means that
there's no additional padding).

### Language bindings

Instead of generating data fields like structs, tables generate a set of methods per field.
For instance, in C++ we would have:

```cpp
class SampleTable {
 public:
  // For "1: int32 foo;"
  const int32* foo();         // getter, returns nullptr if foo not present
  bool has_foo();             // presence check
  int32* mutable_foo();       // mutable getter, forces a default value if not set
  void set_foo(int32 x);      // set value
  void clear_foo();           // remove from structure
  optional<int32> take_foo(); // get foo if present, remove from structure
};
```

### Style Guide

#### Should I use a struct or a table?

Structs and tables provide semantically similar notions,
and it can seem complicated deciding which to prefer.

For very high level IPCs, or for persistent storage, where
serialization performance tends not to be a concern:

- Tables provide some forwards and backwards compatibility, and so offer an
  element of future proofing: prefer them for most concepts.
- Take the performance benefits of structs only for concepts that are very
  unlikely to change in the future (say `struct Vec3 { float x; float y; float z }`,
  or `Ipv4Address`).

Once serialization performance becomes an overriding concern (this is common on the
data path for device drivers for example), we can begin to prefer structs only and
rely on adding new methods to interfaces to account for future changes.

## Backwards compatibility
While this change introduces two keywords, `table` and `reserved`.
there are no backwards compatibility concerns.

## Performance
Use of this feature is opt-in, and should have no impact on IPC performance if it's not used.
We expect build performance differences to be within measurable noise.

## Security
No impact on security.

## Testing
Additional tests for each language binding will be needed, and tests for `fidlc`.

An extended version of the echo suite to use tables would be appropriate.

Adding a fuzzer for table encode/decode would be beneficial &mdash;
there are always tricky cases in parsing.

## Drawbacks, alternatives, and unknowns

There are two big questions to be answered in this space:

- Ordinals vs strings for field identification (ordinals force schemas to be shipped)
- If ordinals: sparse vs dense ordinal spaces per message

### Tables as Vector of Unions

It was proposed that we consider `table` as a `vector<union>`.
This brings two problems:

- The most efficient implementation of a reader of this format must be less efficient
  than the most efficient implementation of a reader for the proposed table format
  &mdash; so we permanently limit our peak performance.
- It doesn't bring any wire compatibility guarantees!
  A vector necessarily needs to carry a length as well as a body, and so a union is
  never convertible on the wire into a table with this proposal (and the number of
  times we'd want to make that transformation seems low).

Instead, by introducing the envelope primitive, we can write down and reason
about the compatibility guarantees in the same fashion... *and* we get to share
some tricky implementation details between tables and extensible unions (in development),
*and* we get to expose a useful primitive up in the language for almost-free.

### Ordinals vs Strings

Use of ordinals requires having a schema present at compilation time,
but allows for more efficient implementations (string handling will always
be slower than integer handling).
Since FIDL already requires schema presence during compilation, it's hoped that
ordinals over strings here is non-controversial.

### Dense vs Sparse Packing {#dense-vs-sparse}

The question of a dense vs sparse ordinal space is likely to be more controversial.
There are two camps in existing practice:

- Thrift and Protobuf use a sparse ordinal space &mdash; fields can be given any ordinal value.
- FlatBuffers and Cap'n'Proto use a dense ordinal space &mdash;
  fields must be given consecutive ordinals.

The Protobuf wire format, when paired with a typical Protobuf implementation that parses
into a fixed size struct, has a bug whereby the amount of memory used by a decoded memory
is uncorrelated with the number of bytes that are transmitted on the wire.
To see this, imagine a message with 10000 (optional) `int64`s fields.
A sender could choose to send just one, resulting in a message that's just a few bytes on
the wire, but almost 100kB in memory.
By sending many of these as RPCs, it tends to be easy to thwart flow control
implementations and cause OOMs.

An alternative implementation strategy for sparse ordinals (as suggested in earlier conversations),
would be to send an ordered array of `(ordinal, value)` tuples.
Implementations choosing in-place decoding would have to rely on binary searches through data
to find an ordinal.
It avoids the flow control bug noted previously, but introduces what could be some large
inefficiencies at runtime as we perform a potentially incredible number of binary searches.

Cap'n'Proto implements a very complex algorithm for dealing with ordinals,
and since we'd like to avoid that complexity it's not discussed further here.

FlatBuffers has a very similar wire format to what is proposed in this document:
utilizing its dense ordinal space to provide a single array lookup to find the
data for a field (or that it's null).

## Prior art and references

- FlatBuffers algorithm is analogous to this one, but has been adapted here
  to better fit with FIDL conventions.
- Protobuf (we believe) originally popularized the ordinal/value representation,
  and its use at scale within Google has demonstrated the scheme's robustness over the years.
- Cap'n'Proto and Thrift each provide small twists on the above.

