{% set rfcid = "RFC-0031" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}

<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-031.

## Rejection rationale

This proposal was rejected due to its poor interaction with service discovery,
and to a lesser extent, to the estimated implementation complexity.

**Interaction with service discovery**

A very common pattern on Fuchsia is to request a specific protocol by its name,
through a `fuchsia.io/Directory.Open` call. We call this service discovery.
During service discovery, a client interacts with a server implementing the
`fuchsia.io/Directory` protocol. When a service discovery request is received by
that server, it finds the appropriate service requested, and transfers its
server end of the channel to the requested service. This means that the client
is interacting with one server (backing the `fuchsia.io/Directory`), then
another server (backing the requested service).

Unfortunately, service discovery imposes strong restrictions on epitaphs. In the
case of a failure leading to an epitaph being sent, the client cannot tell which
peer issued the epitaph -- was it the server backing `fuchsia.io/Directory`, or
the requested service?

In practice, service discovery can include many more than just two servers as
described above. As a result, epitaphs must be very generic, and cannot carry
domain specific details. Essentially, epitaphs suffer from having to satisfy the
least common denominator of all discoverable protocols.

As part of this proposal, and to remedy this restriction, it was discussed
having all protocols which take part in service discovery to compose a
`fuchsia/IsDiscoverable` protocol. This protocol would define a typed epitaph:

<pre language="fidl"><code>
protocol IsDiscoverable {
    <strong>epitaph</strong> zx.status;
};
</code></pre>

None of the children of `fuchsia/IsDiscoverable` could define a custom epitaph,
thus properly capturing the restriction in the type system. Specifically, this
part of the proposal:

> There can be no more than one epitaph type declaration for a protocol
> (including any and all composed protocols, recursively). We specifically
> prevent two semantically equivalent epitaph type declarations with the same
> type.

However, it was deemed infeasible to make all discoverable protocols compose
this new `fuchsia/IsDiscoverable`. For instance, no static enforcement could be
provided since by design service discovery is dynamic.

**Interaction with request pipelining**

The [request pipelining](development/api/fidl.md#request-pipelining) pattern
can be thought of as a generalization of the service discovery pattern, and
imposes similarly strong restrictions on epitaphs.

**Implementation complexity**

While FIDL features carry complexity (language rules, extension to the JSON IR,
bindings code, generated code, ergonomics), typed epitaphs are quite high on the
complexity spectrum, and their usefulness is quite low. That tradeoff didn't
exactly made us jump for joy.

### Where do we go from here? {#terminal-interaction}

_This section represents the author's opinion (pascallouis@google.com)._

When epitaphs were introduced, the [stated goal][RFC-0053] was to 'provide an
indication of why a client-to-server connection is being closed'.

Epitaphs have fallen short of this goal, and their usefulness is marginal.
Protocols that have come to rely on epitaphs could have as well used custom
events, which have none of the shortcomings described above, nor any of the
[payload restrictions][RFC-0037-performance] imposed due to the low-level first
principle behind FIDL.

One benefit of epitaphs is the 'clean termination' of a protocol, where the
server is mostly assured that client peer will unbind from the channel and
refrain from issuing subsequent requests.

It has been floated to introduce a `terminal` modifier applicable to events, to
shift away from the use of epitaphs and onto custom events without losing this
'clean termination' property. With terminal events, library authors would be
free to define the payload of the event as they so desire. To support this, we
would extend the wire format to allocate one flag of the [transactional
header][wire-format-transactional-messages] to indicate termination of the
protocol. Upon receiving of messages marked as terminal, the client would
terminate the connection in addition to the normal handling. Should the client
be unaware of the event (very likely in complex request pipelining cases), the
transactional header could be understood by the client, while the payload
discarded.

## Summary

We propose:

1. New **syntax to indicate the type of an epitaph** on a protocol (we do not
   change the default type `zx.status` of an epitaph);
2. A way to **expose the type of an epitaph** to bindings, so that this
   information can be leveraged appropriately during code generation;
3. Supplement the compositional model to specify that the **type of an epitaph
   is unique per protocol**, and it is **carried over by composition**.

We expect epitaph typing to be used: either on 'leaf' protocols, i.e. protocols
defined by themselves or composing others; or once in a 'composition tree', with
the epitaph type placed on the 'top' or 'root' protocol.

## Motivation

_**tl;dr** we like types. types are good. let's have more types._

### Syntax, and Error Types

In [RFC-0053: Epitaphs][RFC-0053] we introduced the concept of epitaphs "a
mechanism to allow a server to send a message prior to closing a connection that
provides an indication of why the connection is being closed". The epitaph
contains an error status, which is currently fixed to being an int32. When
reviewing epitaphs, we chose to fix the type to int32 to overlay with zx.status,
with the idea to fold 'user errors' with 'protocol errors' down the road.

In [RFC-0060: Error Handling][RFC-0060] we introduced syntax to specifically
type errors, in particular noting that "error types must be int32, uint32, or an
enum of one of those types." We want to allow epitaphs' errors to be typed, and
are choosing a representation matching that of error handling.

In [RFC-023: Compositional Model for Protocols][RFC-0023] we introduced new
syntax to declare protocols, and to compose protocols. The syntax we are
proposing here for epitaphs follows that style.

We consider all these prior decisions to be directionally aligned with
propositions (1) and (2).

### Compositional Model with Epitaphs

Proposition (3) has two aspects, uniqueness of an epitaph type per protocol, and
behavior under composition.

The semantics of an epitaph are similar to that of a special event, and it
follows that the response type would be unique per protocol.

Behavior under composition follows a similar line of thinking, affirming that
epitaphs' types compose. We discuss an alternative definition below, and its
downside.

By allowing epitaphs types to compose, we are introducing a potential breakage
at a distance scenario. Consider a protocol `ChildWithEpitaph` composing a
protocol `FarawayParent`, and defining it's epitaph type to be
`SomeSpecificErrorCode` enum. Should the `FarawayParent` decide post-facto to
specify an epitaph type, the composition would then be disallowed, and
compilation of `ChildWithEpitaph` would fail.

### Alternative: Epitaphs do not Compose

Another approach would be to consider that only defined messages (methods and
events) can be composed into another protocol. In this alternative model, should
a parent protocol define an epitaph type, this type would be independent from
the possibly separate epitaph type of the child protocol. For instance, we would
allow the following definitions:

```fidl
protocol Parent {
    epitaph ParentErrorCode;
};

protocol Child {
    compose Parent;
    epitaph ChildErrorCode;
};
```

Since we do not provide any relationship between protocols (e.g. no subsumption,
no evolvability rule), that alternative model has some merits. It gives much
more freedom for situations where both epitaph typing and composition are used.

However, we have all intents and purposes to define relationships between
protocols in the near future, and should therefore weigh this choice against
this design goal. For instance, if and when we introduce a formal subsumption
relationship ("is a"), should a protocol compose another, where both define an
incompatible epitaph type, these protocols would immediately fail the submission
test: a client expecting one type of epitaph would be ill-equipped to handle
another epitaph type.

We therefore consider it a better choice to be restrictive _today_ in how
epitaph types may be used, so as to leave the door open to extensions
_tomorrow_.

## Design

### Syntax

We extend the grammar to allow an **epitaph** stanza within protocol
declaration:

<pre language="fidl"><code>
protocol SomeProtocol {
    ExampleMethod(...) -> (...);

    <strong>epitaph</strong> SomeErrorCode;
};
</code></pre>

The epitaph stanza is syntactically similar to the **compose** stanza, and also
follows the syntax chosen for error specification.

Formally, the grammar is modified as follows:

```
protocol-declaration = ( attribute-list ) , "protocol" , IDENTIFIER ,
                        "{" , ( protocol-member , ";" )*  , "}" ;

protocol-member = ...
                | "epitaph" type-constructor ; [NOTE]

NOTE: The epitaph stanza allows the more liberal type-constructor in the
grammar, but the compiler limits this to int32, uint32, or enum thereof. There
may be only one epitaph stanza per protocol definition.
```

### ABI and Source Compatibility

When we [introduced epitaphs][RFC-0053] we fixed the type to be int32 with the
expectation that we would constraint this to 32 bits. Fixing error codes to 32
bits was later affirmed when [introducing the errors syntax][RFC-0060].

Here, we are maintaining this choice, and as noted will constraint the epitaph
type to int32, uint32, or an enum thereof.

As a result, changing an epitaph type (possibly by going from the default to a
specified one), does not modify the ABI compatibility.

However, changing an epitaph type will be most likely a source level breaking
change. Binding authors MAY break source compatibility. We do not foresee this
being a problem as epitaphs are not widely used today.

### JSON IR

We add to the  `definition/interface` object a member epitaph of type
`definitions/type`.

For instance, we may have:

```json
    {
      "name": "example/SomeProtocol",
      "epitaph": {
          "kind": "primitive",
          "subtype": "uint32"
      },
      "methods": [
        {
          "ordinal": 296942602,
```

The epitaph type is always present in an interface declaration, and set to the
default `zx.status` if not specified otherwise.

### Composing Epitaphs

When composing a protocol into another, the type of the epitaph carries over.
For instance, with the protocols:

```fidl
protocol Parent {
    epitaph SomeErrorCode;
};

protocol Child {
    compose Parent;
};
```

The resulting epitaph type of both `Parent` and `Child` is `SomeErrorCode`.

There can be no more than one epitaph type declaration for a protocol (including
any and all composed protocols, recursively). We specifically prevent two
semantically equivalent epitaph type declarations with the same type.

This example is invalid and should fail to compile:

```fidl
protocol Parent1 {
    epitaph SomeErrorCode1;
};

protocol Parent2 {
    epitaph SomeErrorCode2;
};

protocol Child {
    compose Parent1;
    compose Parent2;
};
```

This example is also invalid, and should fail to compile:

```fidl
protocol Parent {
    epitaph SomeErrorCode;
};

protocol Child {
    compose Parent;
    epitaph SomeErrorCode;
};
```

## Implementation Strategy

(To be determined by the FIDL team post review. This change is not dissimilar
from many prior changes.)

## Ergonomics

N/A

## Documentation and Examples

At least:

* [Wire format specification][wire-format] should indicate that epitaphs are
  `int32`/`uint32`, which need to be interpreted with their appropriate type.
* The [guidance for developers][RFC-0053-guidance] should be clarified with
  statements such as "`ZX_OK` (or relevant success code)" to widen the advice to
  developer specified types.

## Backwards Compatibility

**FIDL source**: Strictly backwards compatible since we're expanding the
language grammar. No FIDL file could have a "epitaph type-constructor;" stanza
in a protocol declaration prior to this change.

**JSON IR**: Backwards compatible for non-strict parsers which allow extra keys,
since we're adding the "epitaph" key to all interface declarations.

## Performance

No impact on performance.

## Security

No impact, or slightly positive given additional type safety.

## Testing

Unit testing in fidlc, and binding generation tests.

## Drawbacks, Alternatives, and Unknowns

N/A

## Prior Art and References

(As noted in text.)

<!-- xrefs -->

[RFC-0023]: contribute/governance/rfcs/0023_compositional_model_protocols.md
[RFC-0037-performance]: contribute/governance/rfcs/0037_transactional_message_header_v3.md#performance
[RFC-0053-guidance]: contribute/governance/rfcs/0053_epitaphs.md#guidance
[RFC-0053]: contribute/governance/rfcs/0053_epitaphs.md
[RFC-0060]: contribute/governance/rfcs/0060_error_handling.md
[wire-format]: reference/fidl/language/wire-format/README.md
[wire-format-transactional-messages]: reference/fidl/language/wire-format/README.md#transactional-messages
