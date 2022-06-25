{% set rfcid = "RFC-0064" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-017.

## Rejection rationale

The problem of transmitting large messages between peers over FIDL is important,
a very current concern faced by developers, and a current deficiency of FIDL.
The combination of [channel limits][channel_limits], the ease of defining
messages that can surpass those limits depending on runtime behavior, along
with the inherent difficulty to size messages (e.g. [max out
pagination][max_out_pagination]) make this problem untractable in application
code (at least, at scale). A solution **must be provided by FIDL**.

In short, FIDL needs to provide a way for library authors to eliminate the
possibility of runtime surprises due to protocol limits (e.g. hard limits of
number of bytes and handles in the case of Zircon channels), and FIDL developers
need to be able to rely on bindings to implement protocols that may exchange
large messages.

This RFC has many desirable ingredients, and will serve as a reference in the
future. In particular:

* The **problem statement** and motivation.
* Favoring **static verification** to provide guarantees that either messages
  satisfy the protocol limits, or opting into a runtime mechanism offered by the
  bindings to 'make it fit' (possibly incurring some extra cost)
* Relying on a distinction between **value types vs resource types** to offer
  different points in the convenience/performance tradeoff continuum.

Despite the above, this RFC is rejected for the following reasons:

* How a messages' bytes and handles are transmitted from one peer to another is
  not a wire format concern, and there should be no modification to the wire
  format needed to satisfy the requirements.
* The introduction of the `box<T>` notation helps identify a place in a message
  where boxing can occur, i.e. it is a fine grained mechanism to indicate boxing
  behavior at an (almost) arbitrary place in a message. Instead, the current
  thinking is that providing an annotation at the method level is preferable,
  i.e. a coarser description mechanism, at use site rather than declaration
  site.
* With the gradual generalization of FIDL to support different transports beyond
  Zircon channels, we must take into consideration that each transport will come
  with its own limitations. For instance, as we look to expand FIDL to Zircon
  fifos, the size limitation will be different, and no handles can be allowed.
  This furthers the view that a method level annotation (and not a type centric
  annotation) is preferable.

Today, an approach taken to address this in applications uses the
[`fuchsia.mem/Data`][fuchsia_mem_data] type, a union representing data either
inline or in a VMO. Generalizing this approach to any request and/or response
using syntactic sugar as is done for [errors][rfc0060], along with bindings
support, is the current top contender to move forward on this topic.

## Summary

Introduce a mechanism for transmitting large structured data objects through an
auxiliary VMO associated with a FIDL message.

## Motivation

There is an upper bound on the maximum size of FIDL message that can be
transmitted between processes through a Zircon channel, as defined by
[ZX_CHANNEL_MAX_MSG_BYTES][channel_limits].  At the time of this writing, the
limit is 64 KB including FIDL headers.

Although the limit is adequate for many applications, it is occasionally
necessary to transmit larger objects.  This poses a challenge for FIDL API
designers since they must design alternative means for transmitting those
objects, such as:

* **Pagination**: When transmitting collections of objects (typically vectors),
  deliver objects incrementally in batches rather than all at once.
 * Works well as long as each individual object is smaller than the limit
   (taking headers and other fields into account).
 * Somewhat difficult for developers to determine how to pack objects
   efficiently into messages without exceeding the limit due since there are
   currently no APIs for estimating the size of a FIDL message or for
   incrementally constructing one.
* **VMO Encapsulation**: Instead of transmitting large objects within the FIDL
  message itself, copy them into a VMO.  Typically this is represented as a
  [`fuchsia.mem/Buffer`][fuchsia_mem_buffer].
 * Works well for byte vectors (blobs).
 * Cumbersome for structured data objects since the developer is responsible for
   invoking serialization/deserialization.
 * Requires discipline to mitigate security threats due to shared memory.

Failure to anticipate this issue is a major cause of runtime instability.

We believe that FIDL should offer a **built-in**, **safe**, **statically
verifiable**, and **efficient** mechanism for transmitting large data objects.

## Design

### Boxes in Brief

A **box** is a container for a possibly large data object that may need to be
transmitted out of band when the total message size (including headers) exceeds
the limit for a Zircon **channel**[[1]](#footnote1).

A **box** only holds data objects; it cannot hold objects that have
handles[[2]](#footnote2).

At **design time**, a FIDL protocol author..

* encapsulates data objects into boxes when they anticipate that messages
  containing those objects could exceed channel limits

At **compile time**, the FIDL compiler...

* parses declarations
* statically verifies that each box only contains data objects (no handles)
* statically verifies that the maximum possible size of each FIDL message does
  not exceed channel limits ([in static message size enforcement
  mode](#backwards-compatibility))
  * assumes vectors and strings are as large as their bounds will allow
  * assumes that extensible unions and structs (tables) are as large as they
    could possibly become
  * rejects recursive structures unless the recursion is broken by a box

At **compile time**, each FIDL code generator...

* produces code sufficient for serializing and deserializing boxed data

At **runtime**, the FIDL encoder..

* packs as many boxed objects as will fit into the message body, following all
  other out-of-line objects
* packs all remaining boxed objects that didn't fit into a VMO

At **runtime**, the FIDL decoder..

* prior to accessing the VMO containing boxed objects (if any), ensures that it
  has the only handle to the VMO (the VMO is not shared)
* extracts boxed objects from the message body and from the VMO

### Language Details

We introduce a new built-in FIDL type, denoted `box<T>`.  `T` designates the
type of object to be put in a box.

* `T` must be a reference type that does not directly or indirectly contain
  handles.
* `T` must not be a primitive type.
* `T` may be an optional type.

The new type `box<T>` can be used wherever reference types such as **structs**,
**unions**, **vectors**, and **strings** would be accepted.

### Example Box Types

* `box<string>`
  * a box containing an unbounded string
* `box<int>`
  * ERROR: a box cannot contain an object of primitive type
* `box<vector<T>:100>`
  * a box containing a bounded vector of T objects
* `box<vector<T>>`
  * a box containing an unbounded vector of T objects
* `vector<box<T>>:100`
  * a bounded vector of boxed T objects.
* `box<string:100>`
  * a box containing a bounded string
* `box<string>`
  * a box containing an unbounded string
* `box<MyStruct>`
  * a box containing a structure
* `box<MyStruct?>`
  * a box containing an optional structure
* `box<MyStruct>?`
  * ERROR: a box cannot be optional

### Example Declarations

```fidl
interface Database {
    // OK
    1: SelectTop(string:1000 query) -> (box<Record> record);

    // ERROR: reply may exceed message size limit
    // consider wrapping large objects in a box<>,
    // "Record" size is unbounded
    2: BadSelectTop(string:1000 query) -> (Record record);

    // OK
    3: SelectAll(string:1000 query) -> (box<vector<Record>> records);

    // ERROR: reply may exceed message size limit
    // consider wrapping large objects in a box<>,
    // "vector<Record>" size is unbounded
    4: BadSelectAll(string:1000 query) -> (vector<Record> records);
};

struct Record {
    string name;
    string address;
};
```

### Wire Format

_(Section is incomplete.)_

Idea 1: during depth-first traversal for serialization add all boxes encountered
to a queue, once finished first pass pack boxed items in order following
out-of-line objects until no more space remains then calculate size of remaining
boxed objects, allocate a single VMO, and continue packing box contents from
there

Idea 2: like Idea 1 but put each box into its own VMO, slightly simpler to
implement but may be more limiting

Idea 3: maybe we should drop the idea of boxes altogether and do something at
the level of a method instead, like an annotation, e.g. `[Huge]`

### Bindings

_(Section is incomplete.)_

### Enhanced VMO Syscalls

_(Section is incomplete.)_

Idea 1: define "ensure not shared" flag, verify VMO has exactly one handle,
shares no pages with other VMOs, and is not mapped, can pass this flag to
zx_vmo_read/write/map, etc.

Idea 2: define a new syscall to check if VMO is unshared

Idea 3: make it really check to reverse-COW snapshot a VMO if it is already
unshared (should be a no-op)

Idea 4: punt on VMOs and use Views instead

## Implementation Strategy

_(Section is incomplete.)_

## Ergonomics

_(Section is incomplete.)_

## Documentation and Examples

_(Section is incomplete.)_

## Backwards Compatibility {#backwards-compatibility}

In addition to providing a mechanism for transferring large data objects, boxes
are also intended to address **static safety concerns**.

Currently, programs that attempt to transmit FIDL messages that exceed channel
limits will **fail at runtime** leading to system instability.  Once boxes are
available, it should be feasible to introduce **static message size enforcement
in the FIDL compiler** such that we can guarantee **at compile time** that no
message will ever exceed channel limits; the excedent content can simply be
moved into a box by the FIDL protocol author.

However, enabling static message size enforcement all at once may break existing
code and hinder migration efforts.

We propose to resolve this problem as follows:

* Initially, enable static message size checks in **permissive mode**.
 * The FIDL compiler should check message sizes and emit warnings when channel
   limits could possibly be exceeded.  Recommend that the FIDL protocol author
   start using boxes instead.
* Proceed with migration.
* When complete, enable static message size checks in **enforcement mode**.
  * The FIDL compiler should check message sizes and emit errors when channel
    limits could possibly be exceeded.  Halt compilation.

## Performance

_(Section is incomplete.)_

## Security

By replacing existing ad-hoc mechanisms with an official solution supported by
FIDL language bindings, we have an opportunity to improve overall security
discipline.

For example, the FIDL language bindings can ensure that a VMO containing boxed
data only has a single owner before attempting to access its content.  This
resolves common shared memory threats such as:

* The provider of a VMO modifies data while it is being accessed by the client.
* The provider of a VMO changes the size of the VMO while it is being accessed
  by the client or otherwise induces the client to page fault.

Conversely, introducing this feature may contribute to increased use of large
messages and therefore increase the likelihood of other threats such as:

* The provider of a VMO sends an enormous reply to a client, causing the client
  to allocate a lot of heap while deserializing it.

## Testing

_(Section is incomplete.)_

## Drawbacks, Alternatives, and Unknowns

_(Section is incomplete.)_

## Prior Art and References

_(Section is incomplete.)_

_Editor's note:_ Rejected [RFC-0062: Method Impossible][rfc0062], which proposed
banning all methods that could go beyond protocol limits. This is too
constraining due to the limitation of static analysis, which prevents capturing
runtime behavior to properly paginate messages, or manually 'box' them.

--------------------------------------------------------------------------------

##### Footnote1
Hypothetically, FIDL could be transmitted over other channels for which boxing
may take on a different nature.  How that would be implemented is out of scope
for this proposal.

##### Footnote2
_Editor's note:_ Since this RFC was authored in 2018, the distinction between
value types and resources is formally part of the FIDL language, see
[RFC-0057][rfc0057].

<!-- xrefs -->
[rfc0062]: contribute/governance/rfcs/0062_method_impossible.md
[rfc0060]: contribute/governance/rfcs/0060_error_handling.md
[rfc0057]: contribute/governance/rfcs/0057_default_no_handles.md
[channel_limits]:
https://fuchsia.googlesource.com/fuchsia/+/b04f4996b17a78c6d18a9fc77432a8b4e62f57ce/zircon/system/public/zircon/types.h#303
[fuchsia_mem_buffer]:
https://fuchsia.googlesource.com/fuchsia/+/b04f4996b17a78c6d18a9fc77432a8b4e62f57ce/sdk/fidl/fuchsia.mem/buffer.fidl#9
[fuchsia_mem_data]:
https://fuchsia.googlesource.com/fuchsia/+/b04f4996b17a78c6d18a9fc77432a8b4e62f57ce/sdk/fidl/fuchsia.mem/buffer.fidl#30
[max_out_pagination]: development/languages/fidl/guides/max-out-pagination.md