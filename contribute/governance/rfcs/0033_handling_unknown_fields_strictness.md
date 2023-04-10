{% set rfcid = "RFC-0033" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-033.

## Summary

This FTP amends and clarifies the behavior of FIDL decoders when
encountering tables, extensible unions, enums, and bits &mdash;
_extensible messages_[[^1]](#Footnote1) &mdash; contain fields where the
type is unknown.

Specifically, we propose:

*   Defining a **strict and flexible behavior** for extensible messages,
    specifying how decoders encountering unknown fields (including handles)
    ought to behave;
*   A **`strict` keyword that can prefix an extensible message
    declaration**.
    This guarantees that messages will be received with no unknown fields,
    by rejecting them during validation.
*   Default **extensible messages to be flexible,** i.e.,
    where unknown values are allowed and exposed via bindings;
*   Define and **recommend APIs that bindings provide for clients to
    inspect messages with unknown fields**.

### Relation to other RFCs

This RFC was amended by:

* [RFC-0137: Discard unknown data in FIDL](0137_discard_unknown_data_in_fidl.md)

## Motivation

Extensible messages are a valuable mechanism to enable a data interchange
format to evolve without breaking wire format (binary) compatibility.
However, changing the schema poses design decisions for FIDL decoders,
since questions arise in how to validate, parse and expose those fields to
an end-user.

While each language has different mechanisms and norms for data structure
access, specifying behavior for decoders and their APIs increases security
by enforcing validation behavior, and improves overall ergonomics by
increasing consistency across types and languages.

We also wish to enable bindings for constrained environments, where
parsing unknown fields may not be necessary for correct operation and add
undue performance burdens.
This is also relevant for messages that have achieved maturity, and are
not expected to evolve further.

## Design

An _unknown field_ is one whose ordinal (table), tag (extensible union),
value (enum), or specific bit (bits) is unknown to the reader.
From here, we will use "tag" to refer to the unknown
ordinal/tag/value/specific bit, for brevity.

*   Messages containing unknown tags MUST be validated and parsed
    successfully.
    *   However, see below for an exception for
        [strict](#strict-handling-of-messages) messages.
*   Decoders MUST process unknown handles in messages.
    *   The default processing behavior MUST be to close all handles.
    *   Bindings MAY offer a mechanism for clients to process unknown
        handles specially.
*   Bindings MUST provide a mechanism to detect whether an unknown tag was
    received as part of a message.
*   Bindings SHOULD provide a mechanism to detect whether a field with a
    given tag exists as part of a received message.
*   Bindings MAY provide a mechanism to read the tag, raw data, and
    (untyped) handles in an unknown field.
*   If the target language offers a mechanism to exhaustively check tags
    at compile time (e.g., a `switch()` in C/C++, `match` in Rust):
    *   That language binding SHOULD offer a special "unknown" tag that
        can be included as part of the exhaustive check, so that a catch-all case
        (e.g., `default` in C/C++, `_` in Rust) can be omitted.
    *   The intention of this recommendation is to prevent the catch-all
        case from being required for proper compilation, because if it is, tags
        added in the future will not elicit compiler warnings.
    *   This FTP does not define a mechanism for how this should be
        fulfilled, since implementation strategies can differ between languages.
    *   Example:

        ```c {:.devsite-disable-click-to-copy}
        // Bindings SHOULD NOT offer this API:
        switch(union.Which()) {
          case Tag1: ...
          case Tag2: ...
          case Tag3: ...
          default: ...
          // no unknown tag in bindings forces handling using default case
        }

        // Bindings SHOULD offer this API:
        switch(union.Which()) {
          case Tag1: ...
          case Tag2: ...
          case Tag3: ...
          case Tag_Unknown: ...
          // no default case: new tags cause a non-exhaustiveness warning
        }
        ```

### Strict Handling of Messages

*   We introduce a `strict` keyword that can prefix extensible message
    declarations, e.g., `strict table T { ... }` or `strict enum T { ... }`.
*   Strict messages containing unknown fields MUST be considered invalid.
*   Bindings MUST NOT offer a special "unknown" tag for exhaustive tag
    checking of strict messages, if they support such a mechanism for flexible
    messages.
*   Transitions from strict messages to flexible messages, and vice versa,
    MUST be supported as a non-breaking source-level (API) change, possibly
    using the [`[Transitional]` attribute][transitional] to soft transition.
    *   Such a transition MUST NOT change the wire format (ABI).
*   Strict messages are _not_ transitive.
    If a message is marked as strict, only that message is strict.
    Sub-messages contained within that message are not strict.

*   Example syntax:

    ```fidl {:.devsite-disable-click-to-copy}
    // One simply doesn't walk into Mordor and add a new file mode, so this is
    // reasonable to be strict.
    strict bits UnixFilePermission : uint16 {
        ...
    };

    // It's too dangerous for clients to ignore data in this table if we
    // extend it later, but we wish to keep the wire format compatible if we
    // do change it, so it's not a struct.
    strict table SecurityPolicy {
        ...
    };
    ```

## Implementation strategy

1. Update the FIDL compatibility test to validate that existing language
    bindings conform to this specification.
    1. Add test cases for messages with (1) only known fields, (2) only
       unknown fields, and (3) at least one known and one unknown field.
2. Ensure the FIDL compatibility test has test cases for empty messages of
   all appropriate types.
3. Add support for strict messages in `fidlc`.
4. Update language bindings to support strict messages.
5. Add test cases for strict messages to the FIDL compatibility test.

#### Looking Ahead: Use Site Modifiers

During the design phase, we also considered allowing the strict keyword to
be placed in use sites of declarations, in addition to the proposed
declaration site placement.

Example syntax could be:

```fidl {:.devsite-disable-click-to-copy}
protocol Important {
    SomeMethod(...) -> (strict other.library.Message response);
}
```

Here, the `other.library.Message` may not have been defined `strict`, but
we want to use it all the while requiring strict validation.

This adds some design complexity for binding authors, since
`other.library.Message` may be needed both in strict mode and flexible
mode.

On the encoding/validation/decoding, exposing both strict and flexible
mode for the same message depending on context is not dissimilar to how
strings or vectors are handled.
They have the same layout, but can have different bounds depending on
where they are used.
It is also similar to how extensible unions can be used in nullable or
non-nullable contexts.
Generally, bindings have chosen a type schema, with some way to indicate
bounds, nullability, or as is being explored here, strictness mode.

The second issue with exposing both strict and flexible mode for the same
message, is that of dealing with assembly of messages, and querying of
messages in user code.

Consider for instance an enum with three members, `A`, `B`, and `C`.
In order to expose the flexible mode, we need a special enum member
"unknown".
As a result, it is now possible to assemble an enum that does not pass
strict validation, such that in the other context where this enum is
needed, in the strict context, things will fail during encoding.
Here again, the parallel with strings and vectors is important: without a
highly specialized API, bindings allow creating strings and vectors that
are too long, and then fail to be encoded.

The strategy to follow when faced with supporting both strict and flexible
mode is to generate all the extra pieces for flexible mode, and ensure
that where needed, strict validation is applied during encoding, decoding,
and validation.

## Ergonomics

This FTP improves ergonomics in a few ways:

*   We better set users' expectations for FIDL behavior across languages.
*   Strict messages enable users to avoid writing unnecessary code to
    handle unknown fields.

## Documentation and examples

*   The grammar & language specifications need to be updated for strict
    fields.
*   The FIDL style guide should be updated to give guidance on when to
    declare a message as strict.

## Backwards compatibility

*   This change does not affect ABI compatibility.
*   If changes are needed to decoders or bindings to conform to this FTP,
    those changes may cause source-level (API) breakage, which should be
    addressed on a case-by-case basis.

## Performance

*   Forcing decoders and bindings to conform to this FTP may impose a
    (likely insignificant) performance penalty, by forcing them to process all
    unknown fields and closing all handles.
*   Bindings may need an additional level of indirection (and thus use
    additional memory/binary size) to offer the "unknown" tag for exhaustive
    tag checks.

## Security

This FTP increases security.

*   We specify validation behavior for messages with unknown content.
*   Strict messages enable a decoder to validate and discard unknown
    content before clients inspect them, decreasing the possibility of bugs.

## Testing

See the [Implementation Strategy](#implementation-strategy) section (we
plan to use the FIDL compatibility test).
Additionally, each language binding should have its own tests to assert
correct behavior.

## Drawbacks, alternatives, and unknowns

This FTP largely clarifies behavior, and has an associated implementation
cost to ensure that language bindings conform to its recommendations.

#### Alternative: default to strict, or mixed mode

Strictness ought to be viewed in a similar light as size bounds on vectors
or strings; it is a constraint that is independent from a message's layout,
and can be changed without ABI breakage.

We want FIDL authors to make an explicit choice to restrict (constrain)
their messages.

Further, we do not want a mixed mode, where some messages (e.g., enums)
are strict by default, and others (e.g., tables) are not.

#### Alternative: [Strict] attribute instead of a new keyword

It's an important-enough idea to deserve its own keyword.
There's enough precedent for similar features in other languages that it
translates well to FIDL.

#### Alternative: other keywords

During the design phase, several different alternatives were proposed.
The likeliest contender was `final`: it denotes "final word on the
subject," has precedence in C++, Java, C# (among others).

However, because we may want to use the keyword "final" on protocols to
indicate that one cannot use it in composition (i.e., the traditional use
of "final"), we opted for another keyword to indicate strict validation.

This leaves the door open to introduce syntax such as:

```fidl
final strict protocol Important {
    MyMethod(SomeTable arg);
};
```

Which would indicate that protocol `Important` cannot be composed AND that
all validation must be strict.

Other explored keywords were: `sealed`, `rigid`, `fixed`, `closed`,
`known`, and `standardized`.

#### Alternative: only strict

We could define all extensible messages to always be strict.
Currently, enums and bits are only strict, so this alternative would
extend that to tables and extensible unions.

Under such a scenario, changes to extensible structures (e.g., adding a
new field) would require readers to be updated _prior_ to writers being
updated.
This severely limits the use of these extensible data structures, and is
too constraining for higher level use cases.

Furthermore, if that were the design choice, we would not need to use
envelopes for tables and extensible unions (i.e., no need for number of
bytes nor the number of handles).
Indeed, under a strict only interpretation, unknown fields would be
rejected, and otherwise the schema would determine the number of bytes and
handles to be consumed in a fashion similar to the rest of the messages
FIDL processes.

#### Alternative: only flexible

We could define all extensible messages to always be flexible.

This would be very surprising for enums (and bits), and counter to
expectations.
This leads us to two bad sub-alternatives:

*   Have an exception for enums (and bits) to make them strict &mdash; as
    noted above, this is confusing and makes the language rules harder to
    understand.
*   Keep these messages flexible &mdash; which would be counter to
    expectations, open the door to bugs (e.g., reading an invalid value), and
    certainly cause lots of plain-vanilla validation code to be written by
    hand vs being provided by bindings.

Continuing the exploration to other extensible messages (tables and
extensible unions), there is room and a need for strictness.

Consider, for instance, a secure logging protocol `LogEntry` defined
as a table.
Implementations of this protocol would likely want to guarantee that
clients do not send fields the server does not understand, for fear that
these clients may have expectations about how these new fields may control
the handling of the log entry.
As an example, a newer version may add a field "`pii ranges`" providing
ranges of the log entry that contain PII and must be logged specifically
(e.g., replaced by a unique ID, with the original data vaulted under that
unique ID).
To protect old servers from accepting such payload, and likely mishandling
those log entries, authors would choose the strict mode for their
`LogEntry`, thus protecting themselves from potential misuse down the line.

## Prior art and references

Some of this rationale was guided by
[go/proto3-unknown-fields](http://go/proto3-unknown-fields), which
describes why proto3 dropped support for preserving unknown fields, then
later reversed the decision.

*   FTP-037: Transactional Message Header v3 (not yet posted)

-------------------------------

##### Footnote1
Enums & bits are included in extensible messages, since new members can be
added or removed after the message is defined.

<!-- xrefs -->

[transitional]: /docs/reference/fidl/language/attributes.md#transitional
