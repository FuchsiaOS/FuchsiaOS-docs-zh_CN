<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0120" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

This RFC formalizes the requirements to use (i.e. encode and decode) the FIDL
wire format absent of a transport. It also specifies a rubric on how bindings
should expose this functionality. We introduce the concept of **wire format
metadata** describing the revision and the features of the wire format, and
require its usage in encoding and decoding APIs, such that:

- Bindings officially support using the FIDL wire format without a transport.
- Users must transmit the wire format metadata along with the encoded message.
- Bindings may support a persistence convention where the message is prefixed by
  the metadata.

## Motivation

A core principle of Fuchsia is to be updatable. We have invested heavily in ABI
compatibility when FIDL is used in an IPC context, such as two peers speaking a
FIDL protocol over a Zircon channel. The standalone use cases of the FIDL wire
format on the other hand, by virtue of being relatively rare, have seen less
attention to compatibility. For example, it was sometimes incorrectly assumed
that passing the encoded bytes of a FIDL message alone would result in an
evolvable ABI.

Both the [Driver metadata RFC][driver-metadata-rfc] and [RFC-0109: Fast datagram
sockets][fast-datagram-rfc] call for sending FIDL over byte-oriented interfaces.
Now is a good time to formalize the standalone uses of the FIDL wire format to
provide them with evolution and interoperability guarantees.

## Design

Bindings MUST support encoding and decoding the FIDL wire format without a
transport, an API whose requirements are detailed below. Note that many bindings
already have some form of public encoding/decoding API (e.g. `fidl::Encode` in
the high-level C++ bindings). They should be adjusted in accordance with this
RFC. This part of the RFC can thus be seen as a formalization of a core
functionality, clarifying the layering of FIDL.

### FIDL wire format

The focus of the FIDL wire format is binary compatibility: a set of guarantees
around schema evolution to support reading data written using a different
version of that schema. For example, a type with layout `struct{uint8;uint8;}`
may evolve into layout `struct{uint16;}`. While FIDL provides extensible data
structures, those do not support the evolution of the wire format itself, such
as switching FIDL tables to a more efficient representation. Two pieces of
information in the transactional header aid the binary compatibility of the FIDL
wire format when used over a protocol and a transport:

- Magic number: identifies the revision of the wire format. If a receiver does
  not support this revision, it can deterministically refuse to decode, as
  opposed to assigning erroneous interpretations to a mismatched wire format.
- Flags: indicates any soft-transitions that are enabled in this message. For
  example, during the union-to-xunion migration, one of the bits in the flags
  was used to indicate that unions were encoded using the extensible
  representation.

When the FIDL wire format is used standalone, this information is missing from
the encoded results. We propose to incorporate a subset of that information into
encoding and decoding. Specifically:

- **Encoding** transforms binding/language-specific domain objects to the
  **[FIDL encoded form][fidl-encoded-form]** and an opaque blob of **wire format
  metadata** that describes the revision and features of the wire format being
  used.
- **Decoding** consumes a FIDL message in encoded form and the corresponding
  **wire format metadata**, producing binding/language-specific domain objects.

In pseudocode, they would have the following function signatures:

```
function Encode<T>(object: T) -> (EncodedMessage, WireFormatMetadata);
function Decode<T>(message: EncodedMessage, metadata: WireFormatMetadata) -> T;
```

`EncodedMessage` contains the encoded bytes as well as any handles in the
message. Most bindings do define a type with similar or equivalent purpose.

The wire format metadata itself will have an ABI compatible with a 64-bit
integer. Its layout is as follows, in pseudo-C-notation:

```c
struct fidl_wire_format_metadata_t {
    uint8_t disambiguator;
    uint8_t magic_number;
    uint8_t at_rest_flags[2];
    uint8_t reserved[4];
};
```

[RFC-0138: Handling unknown interactions][unknown-interactions-rfc] proposes
subdividing the flags in the transactional header into `dynamic_flags` - ones
that concern the request/response interaction model of a protocol, and
`at_rest_flags` - ones that concern the wire format. This RFC assumes that
design, but could be easily adapted accordingly without losing its key
properties.

The wire format metadata should have an alignment of 8 bytes, to facilitate
in-place decoding of messages. Bindings MUST represent the metadata externally
as an opaque structure that is 8 bytes long and has an alignment of 8 bytes
(e.g. a struct with a single `uint64` field). This allows the metadata itself to
evolve by preventing users from depending on specific fields inside the
metadata.

Bindings MUST check that the `reserved` bytes are zero. Bindings MUST NOT depend
on the `at_rest_flags` bytes to have any particular value. Bindings MUST
validate that the `magic_number` represents a wire format revision that is
[supported][supported-magic-number].

Bindings MUST check that the `disambiguator` byte is zero. Having a zero byte in
the front of the metadata prevents programs from mistaking a FIDL message as
text when the message is persisted as a file (see
[Convention for data persistence](#convention_for_data_persistence)).

Note that the information contained in a FIDL transactional header is a superset
of that in the wire format metadata. The semantics of the `at_rest_flags` field
and `magic_number` field is identical between the transactional header and the
wire format metadata.

Each message MUST only be used with its corresponding piece of metadata. In
other words, it is not allowed to share metadata (e.g. use metadata `A` to
decode both message `A` and message `B`) or swap metadata (e.g. use metadata `A`
to decode message `B` and use metadata `B` to decode message `A`). This allows
the message wire format revision to be altered at run-time, such as during wire
format soft migrations.

Bindings MUST support standalone usage with the following top-level types:

- Struct
- Table
- Union

Encoding and decoding functions MUST fail given any other data type. The failure
SHOULD occur at compile-time where possible.

The FIDL language does not mandate how the wire format metadata is transmitted
or associated with the encoded message. For example, the metadata could be
derived from the transactional message header when FIDL is used in a production
IPC context.

### Convention for data persistence

To better support the motivating use cases, we would like to specify a
convention for attaching the metadata to the encoded message, where the byte
content of the message is prefixed by the metadata. Bindings SHOULD support this
prefixed flavor of the standalone wire format usage, referred to as
**persistence**.

The following persistence use cases are in scope:

- Writing a single FIDL object to network, disk, or other byte/packet oriented
  interfaces that do not support transferring Zircon handles, without opting
  into a request/response paradigm. In other words, the data is "at rest".
- Support messages larger than 64 KiB. The 64 KiB message size limit is a
  property of the Zircon channel transport. When persisting messages to a byte
  vector, no such limitations apply. The existing Rust persistence API support
  large messages and has been used to workaround the channel message size limit
  pending built-in FIDL support for large messages, by manually persisting large
  values into a VMO.

The following use cases are out of scope:

- Built-in support for encoding a _sequence_ of messages of the same type.
  Applications may define custom streaming approaches that work better with
  their specific use cases.

Using this prefixed API flavor improves ergonomics and safety in a number of
ways:

- The user does not have to manually keep track of the association between data
  and metadata. The data simply follows the metadata, and can be sent as one
  unit. By comparison, passing the metadata out-of-band increases risk of
  mismatched versions. There is extra complexity when a receiver needs to handle
  multiple wire format versions multiplexed into the same persistence medium:
  - When the sender in a streaming API changes identity, the new sender may be
    speaking a different wire format revision than the original sender.
  - Consider a proxy that receives persistent messages from multiple components
    using different wire format revisions, and stores them into a database. The
    proxy would have to convert out-of-band flavors back into prefixed flavors
    in order to preserve the different wire format revisions.
- Buffer management is simplified and performance may improve, which is
  beneficial if the standalone wire format is used in a hot path. For example,
  the bindings could allocate one buffer to hold both the metadata and the
  payload, or describe a single vectorized write with the first element pointing
  to the metadata.
- Users do not have to re-implement the same logic for passing the metadata in
  multiple languages and client libraries, since FIDL already provides an
  implementation.

The persistence API MUST support the following top-level types:

- Non-resource struct
- Non-resource table
- Non-resource union

Persistence MUST fail given any other data type. The failure SHOULD occur at
compile-time where possible.

In pseudocode, the persistence API would have the following function signatures:

```
function Persist<T>(object: T) -> vector<uint8>;
function Unpersist<T>(bytes: vector<uint8>) -> T;
```

Bindings MAY use alternate naming/method signatures that are the most
appropriate in the target language, as long as they follow the shape of the API
from a data flow perspective.

Bindings MAY support a vectorized `Persist` variant that supports vectorized
output, such as producing a `zx_channel_iovec_t` or `zx_iovec_t` that links to a
number of buffers, or integrating with the idiomatic writer interfaces of target
languages. Bindings SHOULD provide the vectorizing variant if they already use
that in IPC code paths.

Bindings MAY support a vectorized `Unpersist` variant the takes vectorized
input, such as consuming `zx_iovec_t` that links to a number of buffers, or
integrating with the idiomatic reader interfaces of target languages.

Note that persistence results in bytes, as opposed to standalone
encoding/decoding which may result in handles.

Bindings MUST support persisting large values that cause the encoded message
size to exceed 64 KiB.

The FIDL style guide and API rubric should be updated to include persistence
considerations:

- Clearly indicate if a binary blob uses the persistence convention or a
  custom/out-of-band mechanism to pass the metadata.

### FIDL source language

This RFC does not change the FIDL source language.

## Implementation

Bindings should adjust their standalone encode/decode API to align with the
proposed design involving metadata. They do not have to exactly follow the
function signatures, as long as the functions are consistent with the proposal
from a data dependency perspective. For example, the behavior of the decoder
must be configurable via some means by the metadata.

The same standalone encode/decode API should be used to implement messaging,
such as transactional message dispatching over Zircon channels.

Binding support for the persistence API flavors can be added independently.

There is already an [implementation][rust-persistence-impl] of persistence APIs
in the Rust bindings, but the data format and API do not match the design in
this RFC. The Rust implementation will be adjusted to align with the accepted
design.

### Rust changes

Currently, the Rust bindings provide the following functions:

```rust
fn create_persistent_header() -> PersistentHeader;
fn encode_persistent_header(header: &mut PersistentHeader) -> Result<Vec<u8>>;
fn encode_persistent<T: Persistable>(body: &mut T) -> Result<Vec<u8>>;
fn encode_persistent_body<T: Persistable>(body: &mut T, header: &PersistentHeader) -> Result<Vec<u8>>;
fn decode_persistent<T: Persistable>(bytes: &[u8]) -> Result<T>;
fn decode_persistent_header(bytes: &[u8]) -> Result<PersistentHeader>;
fn decode_persistent_body<T: Persistable>(bytes: &[u8], header: &PersistentHeader) -> Result<T>;
```

These should be replaced with the following (exact signature might differ due to
borrowing and lifetime subtleties):

```rust
fn persist<T: Persistable, W: std::io::Write>(body: &mut T, writer: W) -> Result<()>;
fn unpersist<T: Persistable, R: std::io::Read>(reader: R) -> Result<T>;

fn standalone_encode<T: TopLevel, W: std::io::Write, H: core::iter::Extend<HandleDisposition>>(body: &mut T, writer: W, out_handles: &mut H) -> Result<WireMetadata>;
fn standalone_decode<T: TopLevel, R: std::io::Read>(reader: R, handles: &mut [HandleInfo], metadata: &WireMetadata) -> Result<T>;

struct WireMetadata { /* private fields */ }
```

The `TopLevel` trait is implemented for structs, unions, and tables.

In particular, the user can no longer create a persistent header out of nothing
and reuse the same header for encoding multiple messages.

Additionally, the bindings SHOULD provide a way to serialize/deserialize
`WireMetadata` to/from bytes, to support passing the metadata out-of-band.

## Performance

Standalone encoding and decoding is part of the transactional usages of FIDL,
while persistence APIs should share a majority of the code paths. Therefore, we
can reuse the same standards and performance benchmarks.

## Ergonomics

Binding ergonomics should be designed to encourage the **persistence**
convention. For example, a binding could use a shorter and more idiomatic
function name to represent the persistent flavor (e.g. `fidl::persist`), and use
a longer and more explicit function name to represent the public standalone
encoding/decoding API (e.g. `fidl::standalone::encode`).

## Backwards compatibility

This change itself is backwards compatible since it is purely additive, with the
exception of the Rust FIDL persistence implementation. To our knowledge, all
current readers, writers, and stored data of Rust FIDL persistence always evolve
in lockstep.

Adding wire format metadata improves future backwards compatibility, in
anticipation of upcoming FIDL wire format migrations.

The wire format metadata contains 5 reserved bytes. Those bytes may be
repurposed to take on additional meaning in the future. For example, we might
use one byte to describe persistence-specific concerns.

## Security considerations

The [validation requirements][validation] of the FIDL wire format apply here,
and hold the same security properties.

Of note, FIDL is not a self-describing format. Successfully deserializing a
persisted message using one message type does not guarantee the data was
originally serialized using that same message type:

- A program may be confused over whether a FIDL message contains a prefixed
  metadata header, or if the metadata is passed out-of-band, leading to
  incorrect input parsing. We believe such errors tend to be caught early at the
  testing stage. Coupled with clear documentation, the security risk of this
  confusion should be small.

- A malicious actor may trick a program into overwriting a persisted FIDL
  message with type `Foo` with another message of a different type `Bar`, which
  the attacker controls, by exploiting vulnerabilities in path processing. This
  allows the malicious actor to indirectly influence the contents of the `Foo`
  message.

The alternatives section presents a more involved format that mitigates this
risk, by extending the metadata header with information about the message type.

## Privacy considerations

[Padding bytes][padding] in the FIDL wire format are required to be zero, which
helps avoid leaking sensitive information.

Persistent data tends to carry larger privacy concerns compared to ephemeral
data in IPC that is swiftly consumed, but we also have IPC data being sent to a
component that will persist it or send it over a network. As a result, the
privacy concerns are similar between IPC and persistent APIs.

It is worth noting that developers can always manually persist FIDL data via
other means, such as JSON or XML, even if we do not provide an API. The usual
privacy reviews should apply when a future design mentions that it involves
persisting user or other sensitive data, regardless of if the methodology is via
FIDL persistence.

Privacy annotations on FIDL API elements will simplify privacy reviews and
enable better downstream tooling (e.g. automated redaction); they are out of
scope of this RFC which focuses on a particular method of transmitting FIDL
messages.

## Testing

We will extend GIDL, the FIDL conformance test suite, to test encoding and
decoding of the persistent format.

## Documentation

- Augment the [bindings spec][fidl-bindings-spec] to include the added
  requirements from this RFC (e.g. [LLCPP][llcpp-bindings-reference]).

- Create a reference page about FIDL for standalone encoding/decoding and
  persistence, and the relationship between the two APIs.
  - [Rust][rust-encoding-decoding] and [LLCPP][llcpp-encoding-decoding] already
    have related documentation. The existing documentation will be updated.

- Add to `//examples/fidl/` in all languages demonstrating standalone encoding
  decoding and persistence, and add corresponding tutorials.

## Drawbacks, alternatives, and unknowns

### Alternative 1: Only support the persistence API

We could go one step further on the convention and prescribe it as the standard:
all metadata must precede the message payload. While sufficient for the use
cases we have observed today, this direction runs the risk of becoming too rigid
in the future. By providing both an un-opinionated standalone encoding/decoding
API and an opinionated persistence API on top, users will be able to pick one
most fitting to their design.

### Alternative 2: Allow sharing of the wire format metadata

We could allow the same metadata to be shared for all messages in a session.
This allows the metadata to be sent once at the beginning and then omitted for
the rest of the communication between two peers. This could be used to stream
multiple FIDL objects over one instance of a persistent medium. For example,
[Fast datagram sockets RFC][fast-datagram-rfc] could avoid adding 8 bytes to
each UDP datagram, by first sending the metadata over the socket, followed by
multiple objects in encoded form.

In doing so, we take on a constraint that the metadata must be independent of
specific messages being encoded, and only dependent on the version of the FIDL
runtime compiled into the producer of the message. This also means that a FIDL
encoder must not arbitrarily switch wire format representations at runtime,
should it support multiple wire format representations.

The 8 bytes additional tax imposed by the metadata does not seem prohibitively
expensive. Always including it with the message relieves a lot of extra metadata
tracking complexity on the users' end.

For use cases that really desire the raw performance of not including the
metadata, we can look towards adding streaming features natively in FIDL. For
example, one could imagine defining a transport over a socket that streams
values of a single type. Bindings could be implemented to skip sending the
metadata where possible and disallow changing the sender/receiver identity (for
example, the transport must be re-established every time a new peer is
introduced).

```fidl
@stream
protocol UdpSocketPayload over zx.socket {
    Send(SendMsgPayload);
    -> Recv(RecvMsgPayload);
};
```

Having the metadata always specific to a message also enables using flags on a
per-message basis. For example, we might use one flag to indicate that the body
is compressed. It's also conceivable that we could design a more packed wire
representation that is less rotated towards the constraints of efficient
in-memory IPC (e.g. no need to reserve space for pointers or alignment). The
alternative format could be indicated by another bit in the reserved region of
the metadata.

### Alternative 3: Use the transactional message header

The wire format metadata during persistence could be made compatible with the
[transactional message header][transactional-message-header-spec].

We would do so by framing persistence as a transport, with a single method that
writes the object:

```fidl
@persistence
type Metadata = table {
    1: foo int32;
    2: bar vector<int64>;
};

// desugars to
protocol MetadataSink over persistence {
    // Ordinal is the hash of `mylib/MetadataSink.Metadata`.
    Metadata(Metadata);
};
```

This approach has some advantages:

- Reuse existing FIDL features. Persistence is the same as messaging over
  channels except that you write the bytes to some other kind of sink (vmo,
  file, socket). We could also add streaming or multiple message support later
  on, by adding control packets (control ordinals) that inform the message size
  among other things.
- Reduce cross-talk and improve security by reusing the ordinal hash check: an
  attacker cannot fake a message as another type by twiddling some bytes in the
  data plane (the payload). This strategy is consistent with the security
  property of FIDL methods over channels.

This seems an elegant case of transport generalization, but the result will be a
very strange protocol that is only one-way and one-shot: the client may only
send one kind of value exactly once. There is no opportunity for the receiver to
make a response. This wouldn't mesh well with evolution features we are adding
at a protocol level, such as open and closed interactions.

### Alternative 4: Extend the wire format metadata with message type information

As a less adventurous step than alternative 3, we could hash the fully-qualified
type name of the persisted message and add that to the wire format metadata to
identify the type of the message being persisted, without introducing the idea
of a full-fledged transport.

This reduces cross-talk but still has other subtle complexities:

- How to handle the renaming of a type: the name of a persisted type now becomes
  part of the ABI since it affects the hash in the metadata.
- How to harmonize this with transactional IPC uses of FIDL: the main proposal
  formulates the standalone encoding/decoding API as a lower-level core feature
  of FIDL from which the transactional IPC functionalities could be built on
  top. This alternative results in two separate functionalities. Specifically,
  the wire format metadata cannot be derived from the transactional message
  header, since the latter uses a method ordinal hash that is the same for both
  the request and response type.

Overall, we believe the security risks prevented by this alternative is not
worth the extra complexities required.

### Alternative 5: Restrict standalone APIs to non-resource types

The main proposal suggests two kinds of public APIs dealing with the FIDL wire
format:

- Standalone encoding/decoding: may encode resource types and produce handles.
  Shared by FIDL transactional messaging implementations (client and server
  bindings).
- Persistence: does not allow resource types; results are pure data. Wire format
  metadata always precedes the payload as one unit.

This stems from the observation that the persistence convention is sufficient
for all standalone use cases of FIDL today.

A future use case may be better suited to separately transmitting or storing the
wire format metadata and the payload. They could reach for the standalone
encoding/decoding API, but that has the drawback of allowing handles in the API,
which could be unwarranted.

An alternative is to provide three kinds of APIs:

- Binding-internal standalone encoding/decoding: may encode resource types. Used
  by transactional messaging implementations.
- Public standalone encoding/decoding: does not allow resource types.
- Persistence: does not allow resource types. Wire format metadata always
  precedes the payload as one unit.

This improves the non-resource guarantee when a use case desires separately
transmitting or storing the wire format metadata, but makes for a confusing API,
since we end up with two kinds of encoding/decoding API with the only difference
being support for resource types. This is compounded by target language
limitations where sometimes it can be difficult to hide an API from the public
surface.

The main proposal takes the simplification path and merges the first two kinds
of APIs in this alternative together.

## Prior art and references

- [I2I: Persistent FIDL messages][go-i2i-persistent-fidl]
- [Transactional Message Header V3][transactional-message-header-rfc]
- [FIDL Wire Format for Wires][go-fidl-wire-format-for-wires]
- [Phickle][go-phickle]
- [Cutting the FIDL Encoding Pickle][go-cutting-the-fidl-encoding-pickle]
- [Protocol buffers][protobuf]

[driver-metadata-rfc]: https://fuchsia-review.googlesource.com/c/fuchsia/+/543802
[fast-datagram-rfc]: contribute/governance/rfcs/0109_socket_datagram_socket.md
[fidl-bindings-spec]: reference/fidl/language/bindings-spec.md
[fidl-encoded-form]: reference/fidl/language/wire-format/README.md#dual-forms
[go-cutting-the-fidl-encoding-pickle]: http://go/cutting-the-fidl-encoding-pickle
[go-fidl-wire-format-for-wires]: http://go/fidl-wire-format-for-wires
[go-i2i-persistent-fidl]: http://go/i2i-persistent-fidl-messages
[go-phickle]: http://go/phickle
[llcpp-bindings-reference]: reference/fidl/bindings/llcpp-bindings.md#encoding-decoding
[llcpp-encoding-decoding]: reference/fidl/bindings/llcpp-bindings.md#encoding-decoding
[unknown-interactions-rfc]: contribute/governance/rfcs/0138_handling_unknown_interactions.md
[rust-encoding-decoding]: reference/fidl/bindings/rust-bindings.md#encoding-decoding
[rust-persistence-impl]: https://cs.opensource.google/fuchsia/fuchsia/+/b9b8d7aae9cff4398182a0e125055950556178e1:src/lib/fidl/rust/fidl/src/encoding.rs;l=3662
[supported-magic-number]: contribute/governance/rfcs/0037_transactional_message_header_v3.md#when_should_a_new_magic_number_be_assigned
[transactional-message-header-rfc]: contribute/governance/rfcs/0037_transactional_message_header_v3.md
[transactional-message-header-spec]: reference/fidl/language/wire-format/README.md#transactional-messages
[padding]: reference/fidl/language/wire-format/README.md#padding
[protobuf]: https://developers.google.com/protocol-buffers
[validation]: reference/fidl/language/wire-format/README.md#validation
