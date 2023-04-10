<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0138" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

We expand the FIDL semantics to allow peers to handle unknown interactions, i.e.
receiving an unknown event, or receiving an unknown method call. To that end:

* We introduce **flexible interactions** and **strict interactions** to the FIDL
  language. A flexible interaction, even when unknown, can be gracefully be
  handled by a peer. A strict interaction leads to abrupt termination.

* We introduce three modes of operation for protocols. A **closed protocol** is
  one which never allows unknown interactions. Conversely, an **open protocol**
  is one which allows any kind of unknown interaction. Lastly, an
  **ajar protocol** is one which supports only one way unknown interactions.

## A big picture view at FIDL's support for evolution

Before diving into the specifics of this proposal, it is useful to understand
how FIDL aims to answer evolutionary concerns.

The problem has two facets: source-compatibility (API), and binary-compatibility
(ABI).

API compatibility aims to provide guarantees that user code written against
generated code before a change can still compile against generated code after a
change. As an example, one can reasonably expect that adding a new declaration
to a FIDL library (say defining a new `type MyNewTable = table {};`) will not
cause existing code using this library to fail to compile.

There is a three pronged approach to solving source-compatibility problems:

1. Make as many changes source compatible as possible (e.g. [RFC-0057: Default
   no handles][RFC-0057]);
2. Provide clear guarantees (e.g. [RFC-0024: Mandatory source
   compatibility][RFC-0024]);
3. Provide versioning (e.g. [RFC-0083: FIDL versioning][RFC-0083]).

Separately, ABI compatibility aims to provide interoperability of programs built
against different versions of a library. As an example, two programs can have a
different understanding of a table's schema and yet be able to successfully
communicate.

Achieving ABI compatibility can be broken down into three parts:

1. At rest compatibility is concerned with achieving interoperability at a data
   level, i.e. when can two peers with different schema of the same table
   interoperate?
2. Dynamic compatibility assumes that all data types are compatible, and focuses
   on achieving interoperability when peers have different versions of a
   protocol (e.g. different methods);
3. Lastly, there are some cases where having divergent protocols is not an
   option, and where the solution is instead to learn about the capabilities of
   each peer (negotiation), and then adapt the communication (which protocol is
   spoken) based on that.

Dynamic compatibility is particularly appropriate when "local flexibility" is
sought, such as small additions to an otherwise mostly unchanged model of
operation. In other cases, say fuchsia.io1 relative to fuchsia.io2, a domain
model shift is required. There "global flexibility" is needed, and solutions
sought fall in the protocol negotiation category.

The mechanism we specifically discuss in this RFC (strict and flexible
interactions) improves the status quo of dynamic compatibility (2).

## Terminology

_A reminder about the [compositional model of
protocols](0023_compositional_model_protocols.md#a-model-for-protocols)._

Communication between two peers is an **interaction**. An interaction starts
with a **request**, and may **optionally require a response**.

Both requests and responses are **transactional messages**, which are
represented as a header ("the transactional header"), optionally followed by a
**payload**.[^transactional-message]

[^transactional-message]: Confusingly, a message (as opposed to a transactional
    message) refers to the [encoded form of a FIDL
    value](/docs/reference/fidl/language/wire-format/README.md#message).

An interaction is directed, and we name the two peers **client** and **server**
respectively. A **client to server interaction** starts by a request from the
client to the server, with the response if there is one in the reverse
direction. Similarly, we speak about a **server to client interaction**.

We often use the term **fire and forget** or **one way** for responseless
interactions initiated by the client, and the term **call** or **two way** for
interactions requiring responses (always client initiated in the current model).
When the server is the initiating peer of a responseless interaction, it is
often called an **event**.[^fidlc-response-request]

[^fidlc-response-request]: For `fidlc` and JSON IR aficionados, note that the
    internals of the compiler represent an event as a `maybe_request_payload`
    equal `nullptr` and `maybe_response_payload` is `present`. From a model
    standpoint however, we call this payload a request but with a
    server-to-client direction. We should align to the compositional model,
    change `fidlc` and the JSON IR. This is out of scope of this RFC, but noted
    for completeness.

A **protocol** is a **set of interactions**. We define a **session** as a
particular instance of a communication between a client and a server using a
protocol, i.e. a sequence of interactions between a client and a server.

An **application error** is one which follows the [error syntax][RFC-0060]. A
**transport error** is either an error occurring due to a kernel error (e.g.
writing to a channel that was closed), or an error occurring in FIDL.

## Motivation

A core principle of Fuchsia is to be
[updatable](/docs/concepts/principles/updatable.md): packages are designed to be
updated independently of each other. Even drivers are meant to be binary-stable,
so that devices can update to a newer version of Fuchsia seamlessly while
keeping their existing drivers. FIDL [plays a central
place](0050_syntax_revamp.md#principles) in achieving this updatability, and is
primordially designed to define [Application Binary
Interface](https://en.wikipedia.org/wiki/Application_binary_interface) (ABI),
thus providing a strong foundation for forward and backward compatibility.

Specifically, we want to allow two peers with a slightly different understanding
of the communication protocol between them to safely interoperate. Better yet,
we want the assurance of a strong static guarantee that two peers are
'compatible'.

A lot of work has gone into providing flexibility and guarantees for encoding
and decoding FIDL types, which we call **at rest compatibility**. We introduced
the [`table` layout](0047_tables.md), the [`union`
layout](0061_extensible_unions.md), chose [explicit `union`
ordinals](0048_explicit_union_ordinals.md), introduced the [`strict` and
`flexible` layout modifiers][RFC-0033],
introduced [`protocol` ordinal hashing](0020_interface_ordinal_hashing.md),
[reduced collision probability of `protocol` ordinal
hashing](0029_increasing_method_ordinals.md), and evolved the [transactional
message header format][RFC-0037] to future proof it.

We now turn to dynamic flexibility and guarantees, which we call **dynamic
compatibility**. Assuming two peers are at rest compatible, i.e. all the types
they use to interact are at rest compatible, dynamic compatibility is the
ability for these two peers to interoperate successfully, with neither one or
the other peer aborting the communication due to an unexpected interaction.

## Stakeholders

* **Facilitator:** jamesr@google.com.
* **Reviewers:**
  * abarth@google.com (FEC)
  * bprosnitz@google.com (FIDL)
  * ianloic@google.com (FIDL)
  * yifeit@google.com (FIDL)
* **Consulted:**
  * jamesr@google.com
  * jeremymanson@google.com
  * jsankey@google.com
  * tombergan@google.com
* **Socialization:** RFC draft was shared with the FIDL team, and discussed with
  various members of the Fuchsia team. It was shared broadly on the Eng Council
  Discuss mailing list (<eng-council-discuss@fuchsia.dev>).

## Design

We introduce the concept of **flexible interactions** and **strict
interactions**. Succinctly, even if unknown, a flexible interaction can be
gracefully handled by a peer. Conversely, if unknown to the receiving peer, a
strict interaction is one which causes that peer to abruptly terminate the
session. We refer to the **strictness** of an interaction to refer to whether it
is a flexible or strict interaction. See [semantics of flexible and strict
interactions](#semantics-interactions).

Without guardrails, flexible interactions could be inadvertently used in ways
that jeopardize privacy:

* Consider for instance a rendering engine which is designed to evolve. A new
  version adds a `flexible SetAlphaBlending(...);` one way interaction with the
  intent that newer clients targeting older renderers will simply have their
  setting ignored (but most of the rendering will still work). Now, if instead
  that new method was about a special PII rendering mode `StartPIIRendering();`
  it would be crucial for an older renderer to stop processing, rather than
  ignore this, and hence the use of a `strict` interaction would be appropriate.
* Another example would be a malicious peer trying to reflectively discover the
  exposed surface by sending various messages to see which one(s) are
  understood. Typically, reflective functionality comes with extra performance
  cost, and opens the door to privacy issues (you may expose more than you
  realize). By [principle][RFC-0131-avoid-reflection], FIDL chooses to forbid
  reflection, or require an explicit opt-in.

As a result, we additionally introduce three modes in which protocols can
operate:

* A **closed protocol** is one where no flexible interaction is allowed or
  expected, receipt of a flexible interaction is abnormal.
* An **open protocol** is one where any flexible interaction is allowed (be it
  one way or two way). Such protocols offer the most flexibility.
* An **ajar protocol** is one where flexible one way interactions are allowed
  (fire-and-forget calls and events), but flexible two way interactions are not
  allowed (cannot make a method call if the peer does not know about this
  method).

For further details, see [semantics of protocols](#semantics-protocols).

### Semantics of strict and flexible interactions {#semantics-interactions}

The semantics of a strict interaction are quite simple: when receiving an
unknown request, i.e. one whose ordinal is not known to the recipient, the peer
abruptly terminates the session (by closing the channel).

The goal of flexible interaction is to allow recipients to gracefully handle
unknown interactions. This has a few implications which guide the design.

The sender of a flexible interaction must know that its request may be ignored
(because it is not understood) by the recipient.

The recipient must be able to tell that this request is flexible (as opposed to
strict), and act accordingly.

Since a two way interaction requires the recipient to respond to the sender, it
is imperative for the recipient of an unknown request to be able to construct a
response absent any additional details. The recipient must convey to the sender
that the request was not understood. To satisfy this requirement, the response
of a flexible two way interaction is a result union (see
[details](#result-union)).

It follows from the semantics that in the case of a one way interaction, the
sender cannot tell whether its request was known or unknown by the recipient.
When using flexible one way interactions, FIDL authors should be careful about
the semantics of their overall protocols.

It is worth noting that one-way interactions are somewhat of "best effort", in
the sense that the sender cannot tell whether the peer received the interaction.
However, channels provide ordering guarantees such that the sequencing of
interactions is deterministic and known. Strict one-way interactions make it
possible to ensure that some interactions occur if and only if a preceding
interaction was understood. As an example, a logging protocol might have a
`StartPii()` and `StopPii()` strict interactions to ensure that no peer ever
ignore these.

For further discussion of the tradeoffs to consider when choosing between a
strict and flexible interaction, see also:

* [Performance considerations](#performance-considerations)
* [Security considerations](#security-considerations)

### Semantics of open, closed, and ajar protocols {#semantics-protocols}

The semantics of a `closed` protocol are restrictive, only strict interactions,
no flexible interactions. It is a compile-time error for a `closed` protocol to
have any `flexible` interactions.

The semantics of an `ajar` protocol allow strict interactions, and one way
flexible interactions. It is a compile-time error for an `ajar` protocol to
have any `flexible` two way interactions.

An `open` protocol has no restriction, both strict and flexible, one way and
two way interactions are allowed.

For further discussion of the tradeoffs to consider when choosing between a
closed, ajar, or open protocol, see also:

* [Performance considerations](#performance-considerations)
* [Security considerations](#security-considerations)

### Changes to the language

We introduce the modifiers `strict` and `flexible` to mark interactions as
strict or flexible:

<pre language="fidl"><code>
protocol Example {
    <strong>strict</strong> Shutdown();
    <strong>flexible</strong> Update(value int32) -> () error UpdateError;
    <strong>flexible</strong> -> OnShutdown(...);
};
</code></pre>

By default, interactions are flexible.

Style guide wise, it is recommended to always indicate explicitly the strictness
of an interaction, i.e. it should be set for every interaction.[^default-debate]

We introduce the modifiers `closed`, `ajar`, and `open` to mark protocols
as closed, ajar (partially open), or open:

<pre language="fidl"><code>
<strong>closed</strong> protocol OnlyStrictInteractions { ...
<strong>ajar</strong> protocol StrictAndOneWayFlexibleInteractions { ...
<strong>open</strong> protocol AnyInteractions { ...
</code></pre>

In a closed protocol, there can be no flexible interaction defined. A closed
protocol may only compose other closed protocols.

In an ajar protocol, there can be no two way flexible interaction defined. An
ajar protocol may only compose closed or ajar protocols.

(There are no restrictions on open protocols.)

By default, protocols are open.

A previous version of this proposal specified ajar as the default. However, this
lead to a conflict where the default value of the openness modifier, ajar,
conflicted with the default value of the strictness modifier, flexible, in the
case of a two-way method declared without explicit modifiers. This meant that a
protocol containing a two way method could not be compiled without a modifier on
at least either the protocol or the method.  See below: the default value of
openness is shown in bold and the default value of strictness is shown in
italics.

![Visualization: grid showing which combinations of open/ajar/closed compile
with
strict/flexible.](resources/0138_handling_unknown_interactions/compileable_interactions.png)

To resolve this, we changed the default of openness from ajar to open, which
allows protocols to compile two way methods without modifiers on either the
protocol or the method.

Style guide wise, it is recommended to always indicate explicitly the mode
of a protocol, i.e. it should be set for every protocol.[^default-debate]

[^default-debate]: We prefer having a liberal grammar, along with a style guide
    enforced by linting. This is design choice is motivated by wanting to both
    have a more approachable language to newcomers, while at the same time
    having very explicit (and in turn verbose) standards for the Fuchsia
    platform.

### Changes to the wire format: transactional message header flags {#transactional-message-header-v4}

We modify the [transactional message
header][RFC-0037-transactional-message-header-v3] to be:

* Transaction ID (`uint32`)
* At rest flags (`array<uint8>:2`, i.e. 2 bytes)
* Dynamic flags (`uint8`)
* Magic Number (`uint8`)
* Ordinal (`uint64`)

i.e. flags bytes are split into two portions, at rest flags two bytes, and
dynamics flags one byte.

The dynamic flags byte is structured as follows:

* Bit 7, first MSB "strictness bit": strict method 0, flexible method 1.
* Bit 6 through 0, unused, set to 0.

Some further details about the use of "dynamic flags":

1. We added flags in [the third version of the transactional message
   header][RFC-0037]. These flags were intended to "be temporarily used for soft
   migrations". As an example, one bit was used during the [strict to extensible
   union migration](0061_extensible_unions.md). However, there are no plans that
   would require using that many flags at once, and we can therefore change the
   intent of these flags from solely being used on a temporary basis to being
   used for as part of the wire format.

1. The strictness bit is required for the sender to indicate to the receiver a
   `strict` interaction in the case where the receiver is unaware of that
   interaction. The semantics expected in this case is for the communication to
   abruptly terminate. Without this strictness bit, such skew between the sender
   and receiver could go unnoticed. Consider for instance an ajar (or
   open) protocol with a newly added `strict StopSomethingImportant();` one
   way interaction. Without a strictness bit, the receiver would have to guess
   whether the unknown interaction is strict or flexible, opting for flexible
   given the intended evolvability improvements sought in this RFC. As a result,
   FIDL authors would be forced to rely on two way strict interactions when
   expanding protocols.

See also [placing strictness bit in transactional
identifier](#alternative-using-transactional-identifiers) for a discussion of an
alternative representation, and [interaction mode
bit](#alternative-interaction-mode-bit) for an alternative representation future
needs may call for.

### Changes to the wire format: result union {#result-union}

The result union, which today has two variants (ordinal `1` for success
response, ordinal `2` for error response) is expanded to have a third variant,
ordinal `3`, which will carry a new enum `fidl.TransportError` indicating
"transport level" errors.

As an example, the interaction:

```
open protocol AreYouHere {
    flexible Ping() -> (struct { pong Pong; }) error uint32;
};
```

Has a response payload:

```fidl
type result = union {
    1: response struct { pong Pong; };
    2: err uint32;
    3: transport_err fidl.TransportError;
};
```

Specifically, if a flexible method uses the `error` syntax the success type and
error type are set accordingly (ordinal 1 and 2 respectively). Otherwise, if a
flexible method does not use the `error` syntax, the error variant of the result
union (ordinal 2) is marked `reserved`.[^abi-implication-of-result-union]

[^abi-implication-of-result-union]: It is worth noting that adding an `error` to
    a `flexible` interaction can be made as a soft ABI compatible change.

Some precisions:

* We are choosing the name `transport_err` since from an application standpoint,
  where that error came from should be indistinguishable. There are application
  errors, and then "transport errors" which is a mix bag of errors due to FIDL
  encoding/decoding, FIDL protocol errors, kernel errors, etc. Essentially,
  "transport errors" is the set of all the kinds of errors which can occur in
  the framework (which includes many layers of software).

* We define the type `fidl.TransportErr` to be a strict `int32` enum with a
  single variant, `UNKNOWN_METHOD`. The value for this variant is the same as
  `ZX_ERR_NOT_SUPPORTED`; that is -2:

  ```fidl
  type TransportErr = strict enum : int32 {
    UNKNOWN_METHOD = -2;
  };
  ```

  When presenting transport errors to the client, if the binding provides a way
  to get a `zx.status` for an unknown interaction `transport_err`, the binding
  is required to use `ZX_ERR_NOT_SUPPORTED`. However, bindings are not required
  to map unknown interaction `transport_err` to `zx.status` if that does not fit
  how they surface errors to the client.

  An alternative approach would be to just use `zx.status`, and always use
  `ZX_ERR_NOT_SUPPORTED` as the value to indicate an unknown method, but that
  has two significant downsides:

  * It requires a dependency on library `zx`, which may not be directly used by
    many libraries. This makes it difficult to define the result union in the
    IR, as we either need to auto-insert a dependency on `zx` or downgrade the
    type to `int32` in the IR but have generated bindings treat it as
    `zx.status`.

  * It does not define how bindings should handle `transport_err` values which
    are not `ZX_ERR_NOT_SUPPORTED`. By specifying that the type is a strict
    enum, we clearly define the semantics for bindings which receive a
    `transport_err` value which is not recognized; it is then treated as a
    decode error.

* We refer to "the result union" singular for simplicity when in fact we
  describe a class of union types which share a common structure, i.e. three
  ordinals, first variant is unconstrained (the success type can be anything),
  second variant must be `int32`, `uint32`, or an enum thereof, and the third
  variant must be a `fidl.transport_err`.

### Changes to the JSON IR

We expose the strictness for interactions in the JSON IR. In practice, we update
the `#/definitions/interface-method` type, and add a `strict` boolean as a
sibling of `ordinal`, `name`, `is_composed`, etc.

We expose the mode of a protocol in the JSON IR. In practice, we update the
`#/definitions/interface` type, and add a `mode` enum with members `closed`,
`ajar` and `open` as a sibling of `composed_protocols`, `methods`, etc.

### Changes to the bindings {#changes-to-bindings}

We want to have bindings visible manifestations of automatic handling of
requests. For instance, while the bindings may be able to automatically
construct a request indicating that the request was unknown, it is important to
both raise that an unknown request was received (possibly with some metadata
about the request), and the choice to respond with "request unknown" or abruptly
terminate the communication.

**At rest concerns.**

* In the case of flexible interactions, the bindings should present the
  `transport_err` variant of the result union to the client through the same
  mechanism that they use to present other transport-level errors such as errors
  from [`zx_channel_write`] or errors during decoding. The `err` and `response`
  variants of the result union should be presented to the client the same way
  that the bindings would present those types if the method was declared as
  strict.

  * For example, in the Rust bindings, `Result<T, fidl::Error>` is used to
    present other transport-level errors from calls, so `transport_err` should
    be folded into `fidl::Error`. Similarly, in the low-level C++ bindings,
    `fit::result<fidl::Error>` is used to convey transport-level errors, so
    `transport_err` should be merged into `fidl::Error`.  The `response` and
    `err` variants would be conveyed the same way as for a strict method. In
    Rust that would mean `Result<Result<T, ApplicationError>, fidl::Error>` for
    a method with error syntax, or `Result<T, fidl::Error>` for a method without
    error syntax, with the `response` value being `T` and the `err` value being
    `ApplicationError`.

  * For bindings which fold errors into a `zx.status`, the `transport_err` value
    `UNKNOWN_METHOD` must be converted to `ZX_ERR_NOT_SUPPORTED`.

**Dynamic concerns.**

* When sending a request using [`zx_channel_write`], [`zx_channel_call`] or
  their siblings, the dynamic flags must be set as follows:
  * Strictness bit (bit 7) must be set to 0 for strict interactions, and must be
    set to 1 for flexible interactions.
  * The next six bits must be set to 0.
* When receiving a known interaction:
  * No change from how bindings work today.
  * Specifically, bindings should not verify the strictness to ease the
    migration from strict to flexible interactions (or vice versa).
* When receiving an unknown interaction (i.e. unknown ordinal):
  * If interaction is strict (as indicated by the received strictness flag):
    * Bindings must close the communication (i.e. close the channel).
  * If interaction is flexible (as indicated by the received strictness flag):
    * For closed protocols, bindings must close the channel.
    * If the interaction is one way (transaction id is zero):
      * Bindings must raise this unknown interaction to the application (details
        below).
    * If the interaction is two way (transaction id is non-zero):
      * For ajar protocols, bindings must close the channel.
      * For open protocols, bindings must raise this unknown interaction to the
        application (details below).
    * Details about raising an unknown interaction:
      * If the interaction is two way, bindings must respond to the request by
        sending a result union with the third variant selected, and a
        `fidl.TransportErr` of `UNKNOWN_METHOD`. This must happen before the
        unknown interaction is raised to user code.
      * Bindings should raise the unknown interaction to the application,
        possibly by invoking a previously registered handler (or similar).
      * It is recommended for bindings to require the registration of an unknown
        interaction handler to avoid building in "default behavior" that could
        be misunderstood. Bindings can offer a "no-op handler" or similar, but
        it is recommended for its use to be explicit.
      * Bindings MAY choose to offer the option to the application to close the
        channel when handling unknown interactions.

When an unknown message contains handles, the server must close the handles in
the incoming message. The server must close all handles in the incoming message
before:

* closing the channel, in the case of a strict method, a flexible method on a
  closed protocol, or a flexible two-way method on an ajar protocol
* replying to the message, in the case of a flexible two-way method on an open
  protocol
* notifying user code of the unknown method call, in the case of a flexible
  one-way method on an open or ajar protocol.

Likewise, when a client receives an unknown event which contains handles, the
client must close the handles in the incoming message. The client must close all
handles in the incoming message before:

* closing the channel, in the case of a strict event or a flexible event on a
  closed protocol.
* notifying user code of the unknown event, in the case of a flexible event on
  an open or ajar protocol.

In general, when an unknown interaction is handled, the order of operations is
as follows.

1.  Close handles in the incoming message.
2.  If applicable, close the channel or send the `UNKNOWN_METHOD` reply.
3.  Raise the unknown interaction to the unknown interaction handler or report
    an error.

In asynchronous environments where multiple threads may be simultaneously
attempting to send/receive messages on the channel, it may not be possible or
practical to guarantee the channel is closed before reporting the unknown method
error. Therefore it is not required to close the channel before reporting an
error for an unknown method or event when that interaction is fatal. However,
for recoverable unknown interactions as specified in this RFC, it is required to
close handles and reply (if applicable) before dispatching the unknown
interaction handler.

Previous versions of this RFC did not specify ordering between closing handles
in incoming messages, responding to unknown two-way methods, and raising unknown
interactions to the user.

### Compatibility implications

#### ABI compatibility

Changing an interaction from `strict` to `flexible`, or `flexible` to `strict`
is not ABI compatible.

Changing a protocol mode (e.g. from `closed` to `ajar`) is not ABI
compatible. While it might seem like changing from a more restrictive mode to a
less restrictive mode could be ABI compatible, it actually is not due to
protocols defining both the sender and receiver side, at once (fire-and-forget
and events).

All changes can be soft transitioned. Modifiers can
[versionned][RFC-0083-versioning-properties] if need be.

#### Source compatibility

Changing an interaction from `strict` to `flexible`, or `flexible` to `strict`
may be source compatible. Bindings are encouraged to offer the same API
regardless of the strictness of interactions, by folding existing transport
error apis.

Changing a protocol mode (e.g. from `closed` to `ajar`) is not
source compatible. Bindings are encouraged to specialize the API they offer
depending on the protocol mode. As an example, a closed protocol does not need
to offer an "unknown method" handler, and is encouraged not to provide such
a handler which will go unused.

### Relation to platform versioning

As detailed in the [evolution section of
RFC-0002](0002_platform_versioning.md#evolution), we "change the ABI revision
whenever the platform makes a _backwards-incompatible_ change to the semantics
of the [Fuchsia System Interface](/docs/concepts/packages/system.md)".

One metric of how well we achieve our
[updatable](/docs/concepts/principles/updatable.md) goal is the pace at which we
mint new ABI revisions. Since adding or removing flexible interactions can be
made in a backwards compatible way, this feature will help with improving
Fuchsia's updatability.

## Implementation

* We can imagine a world where bindings only implement the strict part of the
  spec, this would be safe in that communication would stop early, as if the
  peer had encountered some other error or bug.
* Given importance of evolvability to FIDL, the #1 goal, this is not a desirable
  future, and we therefore require bindings to adhere to this specification.
* In order to comply with the bindings specification, bindings MUST implement
  strict and flexible interaction semantics, as well as the three modes for
  protocols.
* With that in mind, we detail changes to the bindings specification. This is
  ABI breaking, and is a major evolution of the wire format (which
  covers both "at rest" and "dynamic" concerns).

A previous version of this RFC called for gating the rollout of unknown
interactions behind a new magic number. However, as specified, unknown
interactions is backwards compatible with existing protocols, since the header
bit used to indicate strictness was previously unused/reserved and the wire
format only changes for flexible two way methods, which can only exist in open
protocols. Instead of changing the magic number, we will use a two stage rollout
where we enable unknown interactions support but have the default modifiers set
to `closed` and `strict`, then add those modifiers explicitly to existing FIDL
files, then change the defaults to `open` and `flexible`.

## Performance considerations {#performance-considerations}

No impact to `closed` protocols. It is not necessary for closed protocols to
check the strictness bit, as noted in the [changes to the
bindings](#changes-to-bindings) section.

Small impact for `ajar` and `open` protocols:

* Processing unknown interaction is similar to handling a known interaction, a
  pre-registered handler is invoked, and application code is run.
* Furthermore, in the case of a two way unknown interaction (only `open`
  protocols), a response will be constructed and sent by the bindings.

It is our expectation that performance considerations rarely matter, and that
the choice between protocol mode be mostly guided by [security
considerations](#security-considerations).

## Ergonomics

This makes FIDL more complex to understand, but addresses a very important need
around evolvability which has been a sharp edge until now.

## Backwards Compatibility

This features is not backwards compatible, and will require a soft migration
of all FIDL clients and servers.

## Security considerations {#security-considerations}

Adding the ability to send unknown requests to peers (i.e. in the case of
flexible interactions) opens the door to security concerns.

For particularly sensitive protocols, evolution concerns may need to be
preempted by the need for very rigid interactions, and therefore favor the
use of `closed` protocol. It is expected that most of the inner bowels of
Fuchsia rely on `closed` protocols (e.g. `fuchsia.ldsvc`).

When considering `ajar` or `open` protocols, there are two concerns
that FIDL authors need to consider:

* Malicious peer sending unknown requests with large payloads. (This is similar
  to the concern with exists when using `flexible` types which can carry large
  unknown payloads as well.) As noted in [size is
  ABI-impacting](#size-is-abi-impacting) further features are required to
  provide control to FIDL authors, and will be addressed in future work.
* Opening the door to protocol sniffing, where a peer attempts to discover which
  methods are implemented without a priori knowledge, then work to craft a
  message to exploit discovered methods. This can be problematic if an
  implementation exposes more methods than intended. For instance, intending to
  expose a parent protocol but instead binding a child protocol composing the
  parent. Note that the attack vector is not changed by flexible interactions,
  but it may be more easily exploitable due to the ability for a peer to attempt
  multiple ordinals one after the other, without having to reconnect (which
  could be prohibitively expensive in some cases).
* When balancing between opting for an `ajar` versus an `open` protocol,
  consider that a peer is unable to tell whether a one way interaction was
  processed or ignored, whereas in the case of a two way unknown interaction (as
  an `open` protocol allows), the processing peer discloses its inability to
  understand an interaction, and in so doing, may reveal valuable information
  to a malicious peer.

## Privacy considerations

Opening the door to protocol sniffing could lead to privacy concerns. As noted
in the [security considerations](#security-considerations) section, this threat
model is not changed by this RFC but it could be exploited more easily.

## Testing

The key to developing the new set of functionality described in this RFC is
ensuring that all bindings follow the same specification, and all behave
similarly. To that end, one needs to be able to express the specification in
tests, e.g. "send this request, respond with correct transaction id, but wrong
ordinal, expect sender channel to close". It is our experience that additional
focus on fluently expressing the specification results in increased testing, and
as a result, increased compliance by all bindings to the spec, along with
increased regression protection.

We will follow the same approach taken with encoding and decoding, which
culminated in the development of [GIDL](/tools/fidl/gidl/): start by writing
tests by hand, exercise as many bindings as possible, and little by little
generalize the parts that can with an eye towards a declarative based testing
approach. While it is our hope that we can build a similar tool than GIDL for
dynamic concerns, and what we will strive towards, we are not anchoring this as
a end-result and may instead prefer fluently expressed tests written by hand.

## Documentation

There will be extensive documentation for this feature. On the specification
side:

* [FIDL Language Specification](/docs/reference/fidl/language/language.md)
* [FIDL Wire Format Specification](/docs/reference/fidl/language/wire-format/README.md)
* [FIDL Bindings Specification](/docs/reference/fidl/language/bindings-spec.md)

Additional entries in the [FIDL API Rubric](/docs/development/api/fidl.md) will be
added covering protocol evolution.

On the concrete use of this feature in a given target language, we expect every
single binding to update its documentation, and provide working examples.

## Drawbacks, alternatives, and unknowns

### Drawback: maximum size of message is ABI-impacting {#size-is-abi-impacting}

An issue with dealing with unknowns, be it unknown payloads as can be experienced
with `flexible` types or unknown interactions as introduced here, is that the
maximum size of a message expected to be read by a peer is ABI-impacting,
without this limit ever being explicitly described, not statically verified.

Currently, there is no vectorized read of a channel, nor is there the ability to
do a partial read. As a result, a message can be sent to a peer which satisfies
all requirements (e.g. flexible interaction, when peer is expecting) and yet,
result in failed communication thus breaking ABI. If the message in question is
too big for the peer to read because that peer expects messages say of less than
1KiB, then a new message that is over that limit will never be read, and instead
the channel will be closed, and the communication between the two peers aborted.

The introduction of flexible interactions increases the likely occurrences of
such a problem, already present due to `flexible` types.

Some ideas for future direction might be:

* A vectorized channel read, making it possible for a recipient to for instance
  only read the header of a message, then decide whether to read the rest of the
  payload or discard that message (that would also require a new syscall).
* Making the maximum size of a message an explicit property of a protocol,
  possibly with pre-defined size categories such as `small`, `medium`, `large`,
  or `unbounded`.

### Alternative: comparison to the command pattern

The [command pattern](/docs/development/api/fidl.md#command-union) is useful to
allow clients to batch many requests to be processed by a server. It is also
possible to use the command pattern to achieve the kind of evolvability
described in this RFC.

Consider for instance:

```fidl
open protocol AnOpenProtocol {
    flexible FirstMethod(FirstMethodRequest) -> (FirstMethodResponse);
    flexible SecondMethod(SecondMethodRequest) -> (SecondMethodResponse);
};
```

This can be approximated with the closed protocol which follows, i.e. this is
what one would have to resort to with the FIDL feature set today to achieve the
same level of evolvability:

```fidl
closed protocol SimulateAnOpenProtocol {
    strict Call(Request) -> (Response);
};

type Request = flexible union {
    1: first FirstMethodRequest;
    2: second SecondMethodRequest;
    ...
};

type Response = flexible union {
    1: first FirstMethodResponse;
    2: second SecondMethodResponse;
    ...
    n: transport_err zx.status;
};
```

Unsurprisingly, the command pattern approach is unsatisfactory.

Since we have to match each request to a response in the union, we lose
syntactic enforcement of "matching pairs" which in turn also causes a loss of
syntactic locality.

Since an unruly server could respond with `SecondMethodResponse` to a
`FirstMethodRequest`, we also lose type safety. One could argue that smart
bindings could notice this pattern, maybe with the help of an `@command`
attribute`, and provide the same ergonomics we do today for methods.

At a wire level, the command pattern forces "two method discriminators" of
sorts. We have the ordinal in the transactional message header (identifying
`Call` is the interaction), and we have the union ordinal (identifying which
variant of the union is selected, i.e. 1 for `FirstMethodRequest`, 2 for
`SecondMethodRequest`).

Here again, one could argue that if all methods followed the command pattern,
i.e. all methods' requests and responses were unions, we would not need the
ordinal in the transactional message header. Essentially, the flexible protocol
described above would "compile down to" the closed protocol using the command
pattern. The wire format of a union requires counting the bytes and handles of
the variant, and requires these counts to be validated by a compliant decoder.
This is problematic on two fronts:

* The rigidity which the transactional message header allows (no description of
  the payload, decode if you can) is one that is unmatched by the union wire
  format (by design, actually). This rigidity and simplicity is particularly
  well suited for low level uses, which FIDL over rotates towards.

* The compositional model does not have any sense of "a protocol grouping". This
  is very powerful since we can (and do) multiplex multiple protocols over the
  same channel. We use structured composition when possible (i.e. `compose`
  stanza), and also resort to dynamic composition (e.g. service discovery). If
  we took the view that "all compiles down to a union" we would impose a rigid
  grouping.

Lastly, there has been a desire from certain FIDL authors to have "automatic
batching of requests". For instance, the
[`fuchsia.ui.scenic`](/sdk/fidl/fuchsia.ui.scenic/) library is famous for its
use of the command pattern in the `fuchsia.ui.scenic/Session.Enqueue` method.
However, providing "automatic batching of requests" is a dangerous feature to
consider since the semantics of how to process multiple commands in one unit
tend to differ widely from one application to another. How should we deal with
unknown commands? How should we deal with commands that fail? Should commands be
ignored, stop execution, cause an abort and rollback? Even RDBMs systems which
are designed around the notion of 'a batched unit of work' (a transaction) tend
to offer many batching modes ([isolation
levels)(https://en.wikipedia.org/wiki/Isolation_(database_systems))). Suffice it
to say that FIDL has no plans to support "automatic batching of requests".

All in all, while on the surface it might look like the semantics of strict and
flexible interactions are the same as the command pattern, they are sufficiently
different that special semantics are warranted.

### Alternative: protocol negotiation

#### What is protocol negotiation

Protocol negotiation is a broad term describing the set of techniques for peers
interacting with each other to progressively build up context about each other,
thus allowing them to have correct, faster, more efficient communication.

For instance, imagine calling a phone number at random. Maybe the peer will
start with "So and so, yes?". You went from no context about the peer to some
identification. We can continue with "Oh, so and so. Did I get this right?".
Given the prevalence of marketing calls, it's likely that you now be faced with
a "What is this call about? Who are you?". And so on, so forth. Both peers
little by little discovering who the other is, and what capabilities they have.

- Which data elements are understood? Like indicating to the peer the fields of
  a table which are desired, being cautious to avoid the peer generating lots of
  complicated data only to be ignored upon receipt.
- What methods does the peer support? In a rendering engine, you can imagine
  asking whether alpha blending is available as a feature, and if not, adapting
  the interactions with the renderer (possibly by sending different content).
- What performance characteristics should be used? It is common to negotiate the
  size of buffers, or the frequency of calls one is allowed to make (think
  quota).

Each kind tends to require slightly different solutions, though all are
essentially turning an abstract description of an interaction model (e.g. "the
set of methods a peer understands") into data which can be exchanged.

To solve protocol negotiation well, the first step is to provide a way to
describe these concepts ("a protocol", "the response type of method foo"). And
because the peers are starting with a low context world, i.e. they do not know
about each other, and must assume that they have a different definition of the
world, the description of the concepts tend to rely on structural properties.
For instance, saying "response type is `MyCoolType`" is meaningless and up to
interpretation, but saying "response type is `struct { bool; }`" stands on its
own and can be interpreted context-free.

#### How protocol negotiation relates to strict and flexible interactions

What is proposed in this RFC, strict and flexible interactions, provides some
wiggle room when it comes to evolving protocols. Now, it is possible to add or
remove methods. Maybe even a few more. But, abuse evolution powers, and you end
up with a protocol that becomes amorphous, and whose domain is hard to
understand from its shape. This is similar to tables which overtime will have a
myriad of fields because they now represent a sort of "aggregate struct"
combining multiple set of requirements which changed over time.

In contract protocol negotiation makes it possible -- when used well -- to
isolate the versioning burden, and after some dynamic choice (the negotiation),
land on a much cleaner and rigid protocol (possibly a `closed` protocol).

Both techniques to evolution have their place, and they are both needed in the
tool box of evolution.

### Alternative: placing strictness bit in transactional identifier {#alternative-using-transactional-identifiers}

Using transactional identifiers to convey the bits required for strict and
flexible interactions has one important drawback. Some transactional identifiers
are generated by the kernel, i.e. [`zx_channel_call`] treats the first four
bytes of a message as a transaction identifier of type `zx_txid_t`. Packing more
information into the transactional identifiers forces a stronger coupling
between the kernel and FIDL, which is not desirable. By using transactional
header flags instead, FIDL code using `zx_channel_call` can continue to
structure everything in the header except for the identifier.

### Alternative: interaction mode bit {#alternative-interaction-mode-bit}

An earlier versions of this RFC called for adding an "interaction mode" bit to
delineate one way interactions from two way interactions, and expected to expand
to more complex interactions such as [terminal
interaction](0031_typed_epitaphs.md#terminal-interaction)).

The main drawback if that the interaction mode bit is redundant with the
information provided in a transaction identifier: one way interactions have a
zero transaction identifier, two way interactions have a non-zero transaction
identifier. Due to information redundancy, this opens the door to different
implementations (e.g. bindings) using different subsets of the redundant bits to
decide how to process the message. This in turns opens the door to maliciously
crafting a message which is interpreted differently by different parts of the
system.

While we have the ambition to both assign transaction identifiers to all
interactions, and expand interaction modes, both changes that would necessitate
extra bits as discussed in the interaction mode, we prefer to table this design
discussion to when those features will be designed.

### Alternative: on naming

As this RFC iterated, there was a lot of discussion about how to properly name
the new concepts introduced. We summarize here some of that discussion.

To delineate interactions which can be "unknown" versus those which need to be
"known":

* `open` and `closed` original names chosen.
* `(none)` and `required` in the sense that your peer must implement the method,
  else the protocol is terminated.
* **Finalist:** `flexible` and `strict` borrowing from [RFC-0033: Handling of
  unknown fields and strictness][RFC-0033].

To delineate protocols which can never receive unknown interactions, from
protocols which can receive one way unknown interactions, from protocols which
can receive both one way and two way interactions:

* `static`, `standard`, `dynamic` original names chose. A slight drawback of
  "static" and "dynamic" is that we have been using the terms "at-rest" and
  "dynamic" to refer to the wire format and messaging aspects of FIDL. For
  example, part of this RFC refer to "dynamic concerns" which has a different
  meaning ascribed to "dynamic" as compared to "dynamic protocols".
* `strict`, `(none)`, `flexible` again borrowing from [RFC-0033].
* In lieu of `static`, using `sealed` to highlight that the protocol cannot
  expand easily.
* In lieu of `standard`, using `hybrid` or `mixed`.
* **Finalist:** `closed`, `ajar`, and `open`. Since open and closed are not used
  for interactions, we can put them to use for protocol modifiers. The
  definition of ajar is literally "partially opened" which is exactly the
  concept we mean to describe. Yes, all concerned felt it had a bit of a spooky
  twist to it.

## Prior art and references

(As mentioned in the text.)

<!-- link labels -->

[`zx_channel_call`]: /docs/reference/syscalls/channel_call.md
[`zx_channel_write`]: /docs/reference/syscalls/channel_write.md
[RFC-0024]: 0024_mandatory_source_compatibility.md
[RFC-0033]: 0033_handling_unknown_fields_strictness.md
[RFC-0037-transactional-message-header-v3]: 0037_transactional_message_header_v3.md#transactional-message-header-v3
[RFC-0037]: 0037_transactional_message_header_v3.md
[RFC-0057]: 0057_default_no_handles.md
[RFC-0060]: 0060_error_handling.md
[RFC-0083-versioning-properties]: 0083_fidl_versioning.md#versioning-properties
[RFC-0083]: 0083_fidl_versioning.md
[RFC-0131-avoid-reflection]: 0131_fidl_wire_format_principles.md#avoid-reflection
