<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0116" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Rejection Rationale

This RFC proposing a sparse FIDL wire format for tables is rejected with
the following pro / con analysis.

See also:

* [RFC-0047: Tables](0047_tables.md)
* [RFC-0132: FIDL table size limit](0132_fidl_table_size_limit.md)

### Benefits of sparse tables

The main benefits of sparse tables are:

- Performance that scales `O(n)`, where `n` is the number of fields set.
Rather than `m`, the maximum ordinal. Consequently, sparse tables should
perform well with small numbers of fields at high ordinals, in comparison
with the existing tables. It is important to note that users generally expect
table performance to scale as `O(n)`, so sparse tables more closely align with
user expectation.
- More compact wire format. This means fewer bytes to be copied on the wire
and fewer bytes persisted. The practical benefits of a more compact wire format
are unclear - it likely improves performance and reduces memory usage but the
extent to which this happens is unknown.

### Downsides of sparse tables

The main downsides of sparse tables are:

- Generally more complex - especially in comparison to the existing dense
table implementation which is quite simple. Some implementations take
significant work to understand and implement.
- Poor performance with denser inputs
- May use extra storage with dense inputs as compared to dense table
implementations
- In practice, the more efficient table builder implementations don't scale as
O(n), where n is the number of fields set. This negates some of the key
benefits.

### Unknowns of sparse tables

#### Proposed design + builder

The proposed design in this RFC (and several other similar designs) require a
builder to layout field values in the wire format layout. The performance of
this depends on the specific builder algorithm that is chosen, which in turn
is chosen based on performance on the expected inputs.

What builder implementation should be chosen?

##### Scaling of builders

Builders that scale as `O(n)`, where `n` is the number of fields set tend to be
slow on all but the sparsest inputs. This includes builders that perform
insertion sort or use an intermediary data structure such as a min heap and
convert to a sorted list.

Builders that scale as `O(m)`, tend to be faster for existing dense inputs.
An example of this is a builder that builds a table in a dense layout and
then converts to a sparse layout on the wire.

##### Distribution of table entries

Depending on the distribution of expected inputs, builder and sparse table
wire format designs can have drastically different performance.

Some factors:

- Are the entries dense?
- What is the highest ordinal?
- Are there just a few entries scattered?
- Are the entries just in a single region?

There isn't much evidence that current table usage would benefit from sparse
tables. The sparse table effort is based on the hypothesis that in the future
it will be more common for table usage to be sparse, or perhaps the existence
of the sparse wire format will trigger more sparse use.

##### Specializations: compile-time, count etc

The alternative of designing a table builder that performs well against a
distribution of inputs is to specialize builders for specific inputs. Examples:

- Specializing to a known set of fields at compile time and directly populating
the wire format structure without runtime building cost.
- Changing the building algorithm based on field count.
- Letting the user explicitly decide what builder to use.

The downside of this is that the API is made more complex and in some cases the
details of how to actually create the builder are unclear (in the case of
compile time builders).

#### If performance is not a factor

The builder designs are complicated due to performance requirements and due to
canonicalization, which often leads to poor performance. If performance only
needs to be "reasonably fast", a key-value (or ordinal-value) approach can be
taken instead. This is much simpler and is the approach taken by protobuf.

### Cost vs benefits of deferring migration

#### Cost of deferring migration

##### Difficulty of migrating at a later time

There are a number of factors that could increase the cost of a later
migration:

- It may be necessary to maintain both versions indefinitely if there is
sufficient unmigratable usage of the existing version.
- For migrations in the shorter term where all usages can still be switched
over, there is some overhead in not combining these changes in with other
changes being migrated at the same time (e.g. more GIDL tests need to be
created and version support needs to be set up in each binding).

##### ABI Stability

There is an effort to reach ABI stability soon, particularly for the Drivers
SDK which will be dependent on FIDL. Deferred changes to the table wire format
would have a negative impact on this effort.

#### Benefits of deferring migration

##### Know more about use cases and bottlenecks

There is currently not an immediate need for sparse tables. There is also not
much known about what the distribution of table entry layouts will look like
further down the line. Because of this, there is some argument for waiting
things out and seeing what the bottlenecks are.

##### Avoid complexity in the system

Many of the sparse table implementations are quite complex. Once this
complexity is in the system, we will be dealing with it forever. Because of
this, we might want to approach adding this complexity to the system
hesitantly.

### Conclusion

Based on the factors presented above, I would recommend not implementing sparse
tables at this time. A performant implementation is complex and the benefits
are currently unclear. This does not rule out developing sparse tables at a
later time, however, if factors change.

## Summary

This RFC introduces changes to the FIDL wire format that optimize tables
for a sparser distribution of present ordinals.

## Motivation

Today, there is a significant cost to having an unset table field, whether the
field is unset due to being reserved or simply not present.
This cost comes from the current wire format layout of tables - tables are
represented as a vector with size equal to the greatest ordinal set.

Consider the case where a single ordinal is present. The vector will be as
large as the value of this single ordinal, so if the ordinal is large so will
be the cost of allocating and processing the vector.

This is not aligned with user expectations.  In similar wire formats like
protobuf, the cost of setting a field is independent of the ordinal
value.

With FIDL, it might not be necessary to strictly have equal cost for all
ordinals. There are some properties that bias the wire format towards lower
ordinals. For instance, FIDL requires reserving every unexpected field and for
security reasons something analogous to proto extensions would be discouraged.
Because of this, denser inputs are expected than what exist for other wire
formats. Still, the existing wire format is oriented towards denser inputs than
users expect.

## Design

This design introduces a new wire format for tables that targets a particular
kind of sparsity - performance is ordinal dependent, but with a less steep
drop off as ordinals increase than in the existing wire format.

This design assumes that
[RFC-0032: Efficient Envelopes](0032_efficient_envelopes.md)
is approved and uses that as a base.

### Wire format

The new wire format layout will be defined as follows. Pseudo-C-structs are used
here for illustration purposes. The decoded form of the wire format is closely
tied to the wire format and also included in this illustration.

```c
// Top-level inline table component.
// This is identical to the existing representation of table headers.
struct TableHeader {
    // The maximum ordinal used in the table frame, or 0 if the table is empty.
    uint64_t max_ordinal;

    // Two possibilities, depending if this is the decoded form or wire form.
    union {
        // Wire form: marker value 0xffffffffffffffff indicating the presence
        // of a frame. A frame is always present, but may be zero-sized.
        // This field acts as a placeholder for the pointer to the table frame
        // in the decoded form of the table.
        uint64_t frame_present_marker;

        // Decoded form: pointer to the table's frame.
        TableFrame* frame;
    };
};

// Body or 'frame' of the table.
struct TableFrame {
    // Array of *m* 64-bit bitmasks, with each bit in a bitmask indicating the
    // presence of each ordinal in the table, up to the maximum present ordinal.
    // Within a bitmask, presence bits are ordered in little endian order.
    uint64_t presence_bitmasks[m];

    // Array of *k* envelopes, where *k* is the number of bits set in
    // `presence_bitmasks`. There is one entry in the array for each of the
    // table envelopes that is present. Envelopes that are not present do not
    // appear in this array. Envelopes are ordered by ordinal.
    FieldEnvelope envelopes[k];
};

// An envelope represents a present table field.
// This is identical to the representation in RFC-0032: Efficient envelopes.
struct FieldEnvelope {
    // Two possibilities, depending if this is the decoded form or wire form.
    union {
        // Wire form: counts of bytes and handles.
        FieldCounts counts;

        // Decoded form: pointer to the data in the field.
        void* data;
    };
}

// Counts of bytes and handles, used for supporting unknown fields.
// This is identical to the representation in RFC-0032: Efficient envelopes.
struct FieldCounts {
    // Number of bytes, recursively, used by this field.
    uint32_t num_bytes;

    // Number of handles, recursively, used by this field.
    uint32_t num_handles
};
```

There are three main changes:

- Only envelopes that are present are included in the table frame. This
  contrasts with the existing wire format that includes every element in the
  table frame whether present or absent.
- There is a new presence bitmask section of the table frame. This metadata
  is used to improve the speed of indexing into the table - for a given
  ordinal position in the bitmask, the number of bits that are marked
  present up to that point corresponds to the index of the envelope in the
  envelopes list.
- There is a new limit on the maximum table ordinal, which is 256. Bindings
  MUST close the channel if a message contains an ordinal greater than 256.
  This limit is somewhat arbitrarily chosen to balance performance with the
  range of expected ordinals. The purpose of setting a limit is to ensure
  performance is within the expected range and simplify allocation of buffers
  in bindings. The limit can be raised if needed in a future RFC if there are
  sufficient motivating use cases. The existence of a limit will also force
  a conversation with users over use cases around high ordinals and help reach
  a better understanding around needs.

Some specifics:

- The envelopes array MUST only include the *k* envelopes corresponding to
  present fields.
- If the maximum ordinal, the ordinal of the kth envelope, is *n* then the
  length of the bitmask array *m* MUST be `floor((n+63)/64)`. Recall that *n*
  here uses 1-based indexes. That is, for for ordinals 1-64 *m* is 1 and for
  65-128 *m* is 2, etc. When *n* is zero, then `frame_present_marker` is set to
  absent and there is no table body.

## Implementation

### Operations on tables

Given that several operations change as a result of the new wire format,
it is useful to walk through how they will work in the new design.

The particular approaches described here are not mandated by the RFC, but serve
as illustration to the reader on how the operations might be accomplished.

#### Iterating through present fields

Both encode and decode need to iterate through, and process, fields that are
present. A `1` bit in a particular position in a bitmask indicates that a
field is present at the corresponding ordinal. In order to iterate through
present fields, one simply needs to iterate through bits that are `1`.

This can be accomplished with the following procedure:

```c++
template<CallbackFuncType>
void iterate_through_present(
  uint64_t* bitmasks, int num_bitmasks, CallbackFuncType callback) {
  int index = 0;
  for (int i = 0; i < num_bitmasks; i++) {
    uint64_t bitmask = bitmasks[i];
    uint64_t last_offset = 0;
    while (true) {
      if (last_offset >= 64)
        break;
      if ((bitmask >> last_offset) == 0)
        break;
      last_offset += __builtin_ctzll(bitmask >> last_offset) + 1;
      if (last_offset > 64)
        break;

      uint64_t ordinal = 64*i + last_offset;
      callback(index, ordinal);
      index++;
    }
  }
}
```

#### Encode

Iterate through each present field as in the previous section. Descend into the
field value and encode it, computing `num_bytes` and `num_handles`. Fill in
these values on the wire format envelope.

#### Decode

Iterate through each present field. Descend into the field value and decode,
setting the `data` pointer on the envelope to the appropriate value.

#### Finding a field index by ordinal

Bit operations can be used to find the index of a field in the `envelopes` list
of the table frame, given the ordinal.

This can be done with the following procedure:

```c++
int field_index(uint64_t ordinal, uint64_t* bitmasks) {
  uint64_t bitmask_index = (ordinal - 1)/64;
  if ((bitmasks[bitmask_index] & (1ull << (ordinal - 1) % 64)) == 0) {
      // Not found.
      return -1;
  }
  uint64_t field_index = 0;
  for (uint64_t i = 0; i < bitmask_index; i++)
    field_index += __builtin_popcountll(bitmasks[i]);
  uint64_t mask = (1ull << ((ordinal - 1) % 64)) - 1;
  field_index += __builtin_popcountll(bitmasks[bitmask_index] & mask);
  return field_index;
}
```

Note that this takes linear time, but in steps such that for every ordinal
there is a 64-element region in which it locally takes constant time. The
benefit of this is that for the first 64-elements indexing takes constant
time.

![Indexing access time]
(resources/0116_fidl_sparser_tables/indexing_access_time.png)

#### Builder

In many bindings, there is a build step to convert between domain objects and
the decoded wire format before encoding can take place.

This step is more complex after tables switch to the sparse representation
because the envelope list must be in sorted order and only include present
envelopes. In addition to table fields being set, they can be edited or unset.

Some approaches:

- Always maintain objects in the FIDL decoded layout. This means performing
  insertion sort as fields are set.
- Keep key-value pairs (where key = ordinal, value = pointer) in some data
  structure like a min-heap. Produce FIDL decoded layout once `build()` is
  called.
- Use an array of present and absent fields, where index in the array is
  `ordinal - 1`. When finished an `build()` is called, construct the final
  decoded format.
- Specialize a builder for a particular set of table fields so that the decoded
  format is directly populated by assignments. This could mean using templates
  in C++ or macros in Rust to build a clean interface.

For a comparison and more details, see the alternatives section.

The specific technique for building a decoded format representation of
an object is not specified by this RFC.

### Migration

A complex migration will be needed from the existing wire format to the new
format. This RFC does not prescribe the method of performing the migration.
Multiple wire format changes are planned to be batched together, lessening the
cost of the migration per feature.

## Performance

A [CL](https://fuchsia-review.googlesource.com/c/fuchsia/+/536930) was
written that implements a prototype of the proposed table layout.
Benchmarks were run for three different table inputs to understand the effect
of input sparsity: a table with all fields set, a table with every other field
set and a table with only a single field set.

All measurements are from a machine with a Intel Core i5-7300U CPU @ 2.60GHz.

Encode times before and after using sparse tables:

| # Fields | All Set     | Every Other Set | Last Set  |
|----------|-------------|-----------------|-----------|
| 16       | 38   -> 25  | 32   -> 20      | 28  -> 20 |
| 64       | 120  -> 74  | 103  -> 25      | 80  -> 19 |
| 256      | 366  -> 205 | 307  -> 106     | 199 -> 20 |
| 1024     | 1401 -> 656 | 1081 -> 335     | 600 -> 22 |

Encode was faster in all cases.
Decode uses a nearly identical algorithm to encode and is expected to have
nearly identical results.

Average time to lookup an index for an ordinal in ns/op before and after
using sparse tables:

| # Fields | All Set    |
|----------|------------|
| 16       | 1.4 -> 2.6 |
| 64       | 0.4 -> 2.0 |
| 256      | 0.3 -> 3.5 |
| 1024     | 0.2 -> 5.7 |

Indexing time is much slower. But for 64 entries, 103ns total time is added to
index all entries, which is similar to the savings in encode time. Note that
this is measuring time time to access all fields. If only a subset of fields
are accessed the time will be less.

Since the average time to lookup the index with 256 entries is 1.5ns more than
the time to look up the index with 64 entries, it can be inferred that the gain
from memoizing is approximately 1.5ns per op or about 380ns overall. This time
however is for the unlikely case where all 256 fields are set.

The time to build an object in FIDL decoded format were also recorded, but the
specific numbers are elided here because build time is highly
implementation-dependent and will vary widely based on implementation.

The numbers measured assumed the fields being set were known in advance and
were largely similar to the existing implementation. However, if the fields
being set are not known in advance the build time would be expected to be
larger. Still, the times are expected to be within a factor of 2 of the time
for efficient envelopes alone.

### Performance results in context

The encoder and decoder being measured were written by hand and lack the
overhead of existing encoders. They are intended to indicate the ideal
performance of highly optimized code. If we account for the overhead of
real implementations, the before/after performance numbers would likely be
closer.

## Ergonomics

The primary user-visible change is changes to the specific builder
implementations used in language bindings. These changes will vary on a binding
by binding basis.

## Backwards Compatibility

This change is ABI-breaking. It is potentially also source-breaking, depending
on the builder implementation in a given binding.

## Security considerations

There are no new security concerns. The same information is conveyed in tables,
just in a different form.

## Privacy considerations

There is no impact on privacy. The same amount of data is kept in the
table wire format representation as was previously done.

## Testing

The GIDL conformance suite will be used for unit testing and integration
testing will be performed using the FIDL compatibility test suite.

## Documentation

This change is primarily under the hood, so most documentation will not need to
change. What will need to change is:

- The wire format spec will need to be updated.
- Any changes to the binding APIs that fall out from this change will need to be
  updated.

## Drawbacks, alternatives, and unknowns

### Drawbacks

#### Complexity

The use of bitmasks for translating between ordinals and wire format indexes
increases the complexity of all stages: building, encode, decode and indexing
by ordinal. Additionally, the builder becomes much more complex in that for
optimal performance it may need to compute the bitmasks, set the appropriate
fields at appropriate positions -- especially when done at compile time.

#### Builder performance

One general implementation of a builder would layout the fields in an array,
one array slot per ordinal up to the maximum ordinal. When building is
complete, it will compact the entries into a sparse table. This is a lot of
work to do and may end up in some cases being significantly slower than dense
tables.

There may be faster ways to implement builders, particularly in languages with
the ability to perform significant compile time logic but this also increases
the complexity of the implementation.

#### Migration

A full wire format migration is needed. This can be quite costly. The most
recent wire-migration involved a complex schedule of steps and took many
person-months to complete. It also tends to leave a trail of cleanup tasks in
all of the places touched by the migration.

### Approaches to builder implementation

The implementation section described a number of approaches for implementing
builders, which output objects in decoded form. This section compares the
options.

It is important to note, that a choice was made to represent built object in
decoded form, rather than another intermediate form because it is believed
that this will make migration easier. Currently, encode and decode are inverses
of another. If it is necessary to convert to a different intermediate form
to encode, then all bindings will need to be updated to reflect this.

#### Always maintain objects in FIDL decoded format

A simple way to build objects into FIDL decoded format is to always have them
in that format. Unfortunately, it is expensive to do this because newly set
fields will need to be insertion sorted into the sorted list that exists in
the decoded format. Doing this can be quite expensive.

#### Keep key-value pairs (key = ordinal, value = pointer) in a min-heap

A min-heap is easy and relatively efficient to modify. It also has a relation
to a sorted list, in that a sorted list is a min-heap which makes it easier to
do things like modify a decoded value, though this is rarely done in practice.
The idea is to edit with a min heap and then when building is complete, sort
all entries in the heap into the decoded format.

One advantage of this approach is that the amount of work scales with the
number of set fields `k` (actually `O(k log k)`). This differs from other
approaches that might scale with the maximum ordinal. The downside of this
approach is that it isn't necessarily the fastest for many cases but it is
still viable for consideration.

#### Use an array of present or absent fields where the index is `ordinal-1`

This is identical to the array used with the existing decoded representation,
minus bitmasks. When a field is set, it can directly be assigned in an array
by directly indexing into the appropriate position. In order to convert to
the FIDL decoded form, the array must be "compacted" to only include present
entries. This can be done in `O(m)` time, where `m` is the maximum ordinal.

The downside of this approach is that the build time scales with `m` rather
than the number of set fields `k`. However, in practice it appears to often
be faster than the min-heap based approach.

#### Append to log and then compact

Another possibility is that when a field is set, the builder can append a
key-value entry (with ordinal as key) to a log. The log would contain
entries for initial field assignments along with edits. The builder can then
sort the list and apply edits when they are encountered, producing the
desired output.

#### Specialize a builder for a particular set of table fields

At compile time, use templating to generate a builder for the specific fields
being set. This is suitable for cases that use tables for evolution but do not
dynamically change which fields are being set.

When the fields are known, they can directly be assigned into the appropriate
possitions in the sorted list in the FIDL decoded format, making this more
efficient than the other techniques.

A downside is that it is difficult to implement and may not be possible to
implement in many cases.

### Alternatives

There is a close interrelation between the performance of the FIDL operations
(build, encode, decode, lookup) and the representation chosen for the wire
format.

#### No bitmask

Another alternative implementation would be to remove the bitmasks from the
wire format. Each sparse table entry would be a key-value pair with key being
the ordinal and value being the envelope. Binary search would be used to find
entries. While this would work, the cost of binary search seems prohibitive.
Additionally, the extra storage for the key is expected to slow down
performance.

#### Hierarchies of blocks

There is a family of structures that roughly amount to a tree or list of blocks
with the leafs being envelopes. One example of this would be a page table-style
design where the intermediate layers point to lower layers which eventually
have envelopes on the leafs.

There are a few issues with these types of structures:

- Each layer either needs to have a key-value list that needs to be searched
for the appropriate entry (B-tree like) or a list that can be directly indexed
(page-table like). Both of these have performance tradeoffs. With a
page-table like structure, there would be many empty entries that are
encountered through encoding and decoding, though the count of these entries
might still be much less than what would be encountered with a dense table.
However, with a key-value list, the number of bytes per entry would increase,
which is known to cause slowdown, and it would be slower to index into the
table.

- The unused entries of the blocks tend to take up a lot of space. Consider
a 3 layer tree with 8 pointers on each layer (for 512 max elements). The
first entry will require 3*8*8=192 bytes. A fully populated tree would take
up 2624 bytes. A 192 byte dense table would fit 24 elements and a 4096 byte
dense table would fit the full 512 elements, so there isn't much size savings
at the high end and a lot of extra size is used at the low end. As seen in
other areas, size can have a significant effect on performance.

- They are expensive to canonicalize. One of the FIDL principles is that wire
messages must have a single canonical representation given an input. But with
structures based around pointers, the pointed objects need to be put in
specific, canonical locations that is difficult to do efficiently.
Any key-value pairs would need to be sorted and while FIDL has standardized
around depth-first order for pointer traversal, it is also costly to process
each pointer in terms of encode and decode time. Note that like the proposed
bitmask-based approach, the canonicalization cost can be hidden at compile time
if the fields being set are known in advance.

- They may require size limits. There may be reasons to impose size limits on
ordinals anyways, but for some block hierarchies they are baked into the
representation. For instance, a two layer tree might have a size limit of
256-entries based on the sizes of the different layers. Changes to the size
limit are ABI-breaking changes because they mean changing the sizes and
configuration of layers of blocks.

#### Protocol buffers message representation

Messages in protocol buffers are a series of key-value pairs with the key being
equivalent to the FIDL ordinal. There is no guaranteed order that its fields
will be written in.

Because of FIDL's canonicalization requirements, the FIDL equivalent of this
would need to have fields sent in sorted order. A linear or binary search would
be used to find a field. This format is similar to the proposed bitmask format
except using keys instead of bitmasks to identify ordinals.

For small tables, the bitmask format would be expected to be faster than binary
searching through a list of ordinal keys. Ordinals are expected to be lower in
FIDL than with protobuf so there isn't the same need for high ordinal support.
Additionally, the object building step - the most complicated of the common
operations in the proposed format - is identical for both formats.

#### Two record types in FIDL

One option that could be considered is to have two record types in FIDL: sparse
tables and dense tables. The main motivation for not doing this, is it puts the
burden on the user for choosing between the two types which is not necessarily
an easy task. Because of this, there is preference for keeping the FIDL
language simple and only having one type.

It is worth noting, however, that if a new type were introduced it would avoid
the need to migrate from the existing type.

### Other related potential wire format changes

#### Inline fields in envelopes

An upcoming RFC is expected to propose changing envelopes holding pointers to
primitives to directly hold the primitive inside of the envelope, instead of a
pointer.
If approved, that change may be implemented at the same time as the change
proposed in this RFC. It is left out of this RFC because it is a mostly
orthogonal change with its own set of complexities.

## Prior art and references

The representation used by Protocol Buffers was used for comparison.
See the "Protocol Buffers Message Representation" section.
