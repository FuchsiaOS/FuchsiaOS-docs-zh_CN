{% set rfcid = "RFC-0023" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-023.

## Summary

We propose the following changes:

* The keyword **interface** is replaced by the keyword **protocol**.
  (We will use the term "protocol" in the rest of this document.)
* Extending protocols is clarified to denote a **compositional model**, where one protocol can
  be defined as a set of messages, augmented by one or many other protocols.
* The syntax used for protocol extension is changed from one that resembles **inheritance**
  to one that resembles **mixins**.
* Binding authors **must avoid subsumption** (e.g. "is-a" hierarchy, inheritance, subtyping)
  when representing composed protocols in target languages.

## Motivation

The contextual baggage that comes with the term **interface** are things such as method
overloading, constructors and destructors, an object model as recipient of messages, and so on.

However, the goals of FIDL are more modest, and meant to describe a **protocol** between
two peers &mdash; that is, a set of messages that can be exchanged.

We start the [FIDL API][fidl-api] by making this clear, noting for instance that
"Although the syntax resembles a definition of an object-oriented interface, the design
considerations are more akin to network protocols than to object systems."
When faced with the option to introduce more "object-oriented like" capabilities,
we've shied away from that (e.g. recently in comments about overloading on
[RFC-0020: Ordinal Hashing][rfc0020]).

We want this distinction to be clearer in the language, and recommend changing
the syntax by replacing the keyword `interface` by the keyword `protocol`.

Additionally, the "is-a" relationship implied by borrowing inheritance syntax is unsound,
and leads to incorrect expectations.
(For clarity, FIDL does not provide such inheritance semantics, but the syntax suggests as much.)
See the ["Is A" Relationship Considered Harmful](#is-a-relationship-considered-harmful)
section for more details.

## Design

This proposal introduces formal semantics to describe process interaction, and protocols.

This proposal changes the FIDL source language to clarify the semantics of protocol extension,
and provides new guidance to bindings authors.

Today, inheritance relationships are not represented in the JSON IR, and therefore cannot
be leveraged by bindings authors.
Thus, we expect there to be minimal change to how this new guidance modifies generated
bindings code, aside from improved documentation.

This proposal does not change the wire format.

This proposal does not change the JSON IR, though we do expect to include a key rename as part
of larger changes down the road.

### A Model for Protocols {#a-model-for-protocols}

Zircon channels do not require a specific schema for payloads they carry.
FIDL builds upon this primitive and restricts channels to carry specific protocols.
In so doing, FIDL gives meaning and names to both ends of a channel.
We call one the **client**, and the other the **server**.

Our model describes a **protocol** as a **set of directed interactions**,
with an optional **epitaph**.
We call a **session** a particular instance of a communication between a client
and a server using a protocol.

The direction can be **from client to server**, or **from server to client**.

An **interaction** starts with a **request**, and may **optionally require a response**.
We often use the term "fire and forget" or "one way" for responseless interactions,
and the term "call" for requests expecting responses.

Both requests and responses are **messages**, which are represented as a header,
followed by the payload of a struct, the **arguments** of the
request or response.

Today, we restrict server-to-client messages from having responses.
Put simply "events are only fire and forget."

An **epitaph** is a server-to-client interaction that concludes a session.
More details in [RFC-0053: Epitaphs][rfc0053].

Absent from this model are more complex interactions, such as three-way handshakes a la
**SYN**/**SYN-ACK**/**ACK** of TCP.
We consider this to be out of scope, and unlikely to be covered by future refinements to the model.

### Compositional Model

Today, a protocol can both define interactions, as well as extend one or more protocols.
The resulting protocol (the "composed protocol") carries all interactions defined directly,
as well as inheriting all interactions defined by its antecedents (direct or indirect).

For instance, the `Child` protocol defined as:

```
protocol Parent1 { Method1(); };
protocol Parent2 { Method2(); };
protocol Child { compose Parent1; compose Parent 2; Method3(); };
```

Would have all three interactions `Method1`, `Method2`, and `Method3`.

However, whether `Method1` and `Method2` were defined in `Child` as a result of a composition,
or directly, is not carried to language-specific backends, i.e. this is not represented
in the JSON IR.

### "Is A" Relationship Considered Harmful

Since protocols can carry requests in both directions, having a subtyping relationship
requires more care.
For this reason, we do not allow protocols to have "is a" relationships with the
protocols they extend.

For instance, suppose we have the two protocols:

```
protocol Parent { Method(); };
protocol Child { ->Event(...); };
```

Were we to allow a channel that carries protocol `Child` to be viewed as a `Parent`
(i.e. "`Child` is a `Parent`" relationship), we would expose clients to receiving `Event`,
which they would be unable to handle.
See the [next section](#reliance-on-is-a-relationship-today) for a concrete example.

Instead, we will be looking to support specific protocol annotations such as "only client
to server interactions" to support and allow "is a" relationships.
When this occurs, such relationships would carry to the JSON IR for the use of backends.

### Reliance on "Is A" Relationship Today

Looking at a concrete example, the [fuchsia.media] library composes various protocols together.
In particular:

* [AudioCapturer] composes [StreamBufferSet] and [StreamSource]
* [AudioRenderer] composes [StreamBufferSet] and [StreamSink]

Neither the [AudioCapturer], nor the [AudioRenderer] define events, i.e. these are purely
"client-to-server protocols" &mdash; they are unidirectional.
([StreamSource] defines two events, but here we are specifically discussing each
protocol's own definitions.)

As a result, if a client knows how to interact with a [StreamBufferSet] or [StreamSource][StreamSource]
([StreamBufferSet] or [StreamSink] respectively), then it can also interact with an
[AudioCapturer][AudioCapturer] (and [AudioRenderer] respectively) &mdash; i.e. the client will simply
ignore the extra methods exposed.
Here, we could define the "is a" relationship as one would expect.

However, if an event were to be added to either interface, this "is a"
relationship would cease to exist.
Let's say that a client is interacting with a [StreamBufferSet], which really is
an [AudioRenderer] at the server end.
What would happen if the [AudioRenderer] triggers an event?
How would that client handle it?

Since we do not (yet) have the ability to provide this distinction in `fidlc`,
we are affirming that no "is a" relationship is supported.
This proposal essentially clarifies the status quo.

Like in the [fuchsia.media] case, authors who know certain relationships to be true
can bend bindings to their needs (using casting, etc.).

In a subsequent proposal, we expect to introduce attributes, or new keywords, to capture
this directionality constraint, and based on this, provide "is a" relationships in bindings.
Before such a proposal, we cannot provide better support as part of the FIDL toolchain.

### Syntactic Changes

> During the design phase, several different alternatives were proposed,
> see [Drawbacks, Alternatives, and Unknowns, below](#drawbacks_alternatives_and-unknowns)

An extended protocol, using the accepted syntax, looks like:

```
protocol Parent1 {
  Method1OfParent1();
  Method2OfParent1();
};

protocol Parent2 {
  Method1OfParent2();
  Method2OfParent2();
};

protocol Child {
  compose Parent1;
  compose Parent2;
  Method1OfChild();
  Method2OfChild();
};
```

Formally, the grammar is changed as follows:

```
declaration = const-declaration | enum-declaration | protocol-declaration |
              struct-declaration | union-declaration | table-declaration ;

protocol-declaration = ( attribute-list ) , "protocol" , IDENTIFIER ,
                       "{" , ( method-or-compose-declaration , ";" )*  , "}";

method-or-compose = method-declaration | compose-declaration ;

method-declaration = ( ordinal , ":" ) , method-parameters ;

method-parameters = IDENTIFIER , parameter-list , ( "->" , parameter-list )
                     | "->" , IDENTIFIER , parameter-list ;

compose-declaration = "compose", compound-identifier ;
```

A composed protocol may only be mentioned once.

#### Possible Extension

We expect a subsequent proposal to additionally allow server to client
interactions from requiring a response, thus enabling multiplexing protocols on
a channel, possibly in reverse order. For instance [coordinator.fidl] defines
two command-response protocols, one from devmgr -> devhost, and one from devhost
-> devmgr. Currently, these are muxed manually, with reliance on ordinal
dispatch to sort out which is which.

We may use the "->" syntax in the compose block to later introduce muxing in reverse direction.
An alternative would be to only require explicit direction when extension includes a
reversed protocol, which would have the benefit to not introduce any direction syntax today,
since we're postponing extensions with reversed protocols.

We allow the compose block to be placed anywhere in the definition of a protocol,
and we also allow multiple compose blocks.
We could alternatively have only one block, and could also require this to be at the top.
Here, we're choosing to be open, and instead rely on automated formatting and/or style guides
for recommendations, rather than have enforcement baked into the language itself.

### JSON IR

We will not change the JSON IR as part of this change.

Instead, we will rename the "interface_declarations" key to be "protocol_declarations"
as part of a larger set of changes.
This larger set of changes will require a multi-step approach, bumping the schema version from
0.0.1 to 0.0.2, and have a transitional period for backends to adapt.

### Breakage at a Distance, and the Use of `[FragileBase]`

The status of the possibility of breakage at a distance is unchanged by this proposal,
and we therefore reaffirm the use of `[FragileBase]` for any protocol being extended[[1](#Footnote-1)].

### Guidance to Bindings Authors

* Binding **must avoid subsumption** (e.g. "is-a" hierarchy, inheritance, subtyping)
  when representing composed protocols in target languages.
* It should be **an error to receive an unknown ordinal**.
  Bindings should bubble this as "unknown ordinal error", and close the channel.

## Implementation strategy

Three steps:

1. Add support for the new syntax;
2. Convert all FIDL files to use the new syntax;
3. Drop support for the old syntax.

## Ergonomics

This change makes FIDL clearer to understand, see [motivation](#motivation) section.
This change may not make FIDL simpler to understand upfront, but avoids
misunderstandings down the road, and misaligned expectations.

## Documentation and examples

We will need to update the language, grammar, rubric, and other such documentation.

## Backwards compatibility

This change breaks source compatibility with FIDL files currently using inheritance.
As described in the [implementation](#syntactic-changes), we will use a phased approach to
introduce the new syntax, migrate all FIDL files, and then remove support for the old syntax.

This change does not change the FIDL wire format, so it is a backward-compatible ABI change.

## Performance

No performance impact.

## Security

We may be able to leverage tighter typing semantics for securing channels, or observing channels.
This is not a goal of this proposal, does not regress the status quo, and arguably improves it.

## Testing

Testing of this change can be done entirely with unit tests, at the `fidlc` level.

## Drawbacks, alternatives, and unknowns

The following sections record alternate syntax proposed during the design phase.

### Alternative Syntax (pascallouis@)

**Example**:

```
protocol Parent1 { Method1(); };
protocol Parent2 { Method2(); };
protocol Child {
  compose {
    -> Parent1();
    -> Parent2();
  };
  Method1OfChild();
}
```

**Notes**: This was the original proposed syntax.
Having a `compose` block seemed unnatural, and strayed too much from the language as it exists.
It made composing multiple protocols the preferred approach, whereas composing a single one
felt verbose.
It was also unclear whether multiple `compose` blocks would be allowed, and how that would
look like.
Finally, we chose to back away from having a directional "`->`" indicator on protocols preferring
to introduce this down the road along with multidirectional muxing (if such a feature
is ever considered).

### Alternative Syntax (jeremymanson@)

**Why**: To clarify the difference between a list of methods we expect to implement and
a list of methods that defines a communications protocol:

**Example**:

```
protocol Parent1 {
  Method1OfParent1();
  Method2OfParent1();
};

protocol Parent2 {
  Method1OfParent2();
  Method2OfParent2();
};

interface Child {
  compose {
    -> Parent1();
    -> Parent2();
  };
  Method1OfChild();
};
```

**Notes**: The "interface" keyword indicates that each method must have an implementation,
and the "protocol" keyword indicates requirements for conforming protocols and interfaces
that incorporate it.
We wouldn't necessarily expect, say, a `StreamSource` to have its own implementation.
This gets us further away from implementation inheritance by clarifying that none will take place.
You would not be able to compose an interface into another interface.

### Alternative Syntax: Go-like interface composition (proppy@)

**Why**: Doesn't look like inheritance, familiarity with Golang syntax for interface
[embedding](https://golang.org/doc/effective_go.html#embedding)

**Example**:

```
protocol Parent1 { Method1(); };
protocol Parent2 { Method2(); };
protocol Child {
    Parent1;
    Parent2;
    Method3();
};
```

**Notes**: Go language [spec](https://golang.org/ref/spec#Interface_types)
on interface and embedding.

### Alternative Syntax: Using Declaration (jeffbrown@)

**Why**: Doesn't look like inheritance, reuses existing keyword to indicate names
being brought into scope.
Less likely to be confused with a method declaration or a "property[[2](#Footnote-2)]."

**Example**:

```
protocol Parent1 { Method1(); };
protocol Parent2 { Method2(); };
protocol Child {
    using Parent1;
    using Parent2;
    Method3();
};
```

**Notes**: Precedents in FIDL, C++, Rust, and other languages.

### Alternative Keywords

Alternatives to "`compose`" keyword:

* `extends` (pascallouis@)
* `contains` (smklein@)

## Prior art and references

Nothing specific.

[Cap'n Proto](https://capnproto.org/language.html#interfaces) has interfaces that support
inheritance, including multiple inheritance (in the style of mixins).


##### Footnote 1
The introduction of [Ordinal Hashing](/docs/contribute/governance/rfcs/0020_interface_ordinal_hashing.md) for methods, combined with an
intended change to up method ordinals from 32 bits to 64 bits in a future proposal,
will likely make this breakage at a distance inexistant (in practical terms), and
will revisit the use of [FragileBase] then.

##### Footnote 2
Property: A hypothetical FIDL extension to facilitate observation / data binding.
Loosely speaking, the bindings would produce methods for accessing, modifying, and/or observing a value exposed by the interface.

<!-- xref table -->
[fidl-api]: /docs/development/api/fidl.md
[rfc0053]: /docs/contribute/governance/rfcs/0053_epitaphs.md
[rfc0020]: /docs/contribute/governance/rfcs/0020_interface_ordinal_hashing.md
[fuchsia.media]: /sdk/fidl/fuchsia.media/
[AudioCapturer]: https://fuchsia.googlesource.com/fuchsia/+/81597afce01451c2c9d1af6f03453f036b63adff/sdk/fidl/fuchsia.media/audio_capturer.fidl#255
[StreamBufferSet]: https://fuchsia.googlesource.com/fuchsia/+/985ff2f0c4374dddafb8ecf2f0e9a83c772de623/public/fidl/fuchsia.media/stream.fidl#9
[StreamSource]: https://fuchsia.googlesource.com/fuchsia/+/985ff2f0c4374dddafb8ecf2f0e9a83c772de623/public/fidl/fuchsia.media/stream.fidl#40
[AudioRenderer]: https://fuchsia.googlesource.com/fuchsia/+/caa3f20aa7b64240f4265ede5e6deddf0f2d0cf7/garnet/public/fidl/fuchsia.media/audio_renderer.fidl#21
[StreamSink]: https://fuchsia.googlesource.com/fuchsia/+/985ff2f0c4374dddafb8ecf2f0e9a83c772de623/public/fidl/fuchsia.media/stream.fidl#20
[coordinator.fidl]: https://fuchsia.googlesource.com/fuchsia/+/4abe84d253c32746b0324c427cdc2d31a3af438c/system/fidl/fuchsia-device-manager/coordinator.fidl
