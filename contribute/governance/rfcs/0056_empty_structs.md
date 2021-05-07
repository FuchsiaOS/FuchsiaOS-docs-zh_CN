{% set rfcid = "RFC-0056" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-012.

## Summary

Allow empty structs in FIDL library declarations and define the encoding
of that to be a struct with a single zero-valued `uint8`.

## Motivation

Today, we see the need for empty structs come up in various areas.
For instance, when using the
[command pattern](/docs/concepts/api/fidl.md#Command-union)
where one of the commands does not require any arguments.
Say you have a "ping" command, where simply selecting this union option is enough to convey the intent.
Generalizing, empty structs are useful to represent 'unit types' when simulating
[Algebraic Data Types](https://en.wikipedia.org/wiki/Algebraic_data_type).

However, empty structs are disallowed in some target languages: you can't have empty C/C++
structs, and we want FIDL wire encoding to be mappable to C data types.

As a result of the need, and the current constraint, we've seen a proliferation
of structs that are semantically empty, but in practice have a single small
placeholder field so that they compile.

Here are some examples:

- [fuchsia.modular.GlobalScope](https://fuchsia.googlesource.com/fuchsia/+/ce931e090d0c54030a80397bd24f217132983794/peridot/public/fidl/fuchsia.modular/user_intelligence/scope.fidl#22)
- [fuchsia.modular.FocusStory](https://fuchsia.googlesource.com/fuchsia/+/5ed52652adfeb70c011a5d86acfd3bbfb768b13e/public/fidl/fuchsia.modular/suggestion/proposal.fidl#74)

This proposal would decouple the constraints of a particular set of languages from
the FIDL language while maintaining wire format compatibility with C structs.

## Design

The FIDL frontend compiler should accept empty struct definitions.
The TypeShape of an empty struct should be the same as a struct with a single `uint8` field.

Bindings generators for languages that can handle empty structs should generate
actual empty structs but care must be taken to ensure that empty structs are
correctly encoded as a struct that takes a byte.
For C and C++ a single `uint8_t __reserved` field should be generated.

## Implementation strategy

1. Allow `fidlc` to accept empty structs and synthesize a `uint8` field in the IR.
2. Update the `fidl_compatibility_test` to include an empty struct.
3. Update the documentation.
4. Update `fidlc` to not emit a bonus bogus field for empty structs, update
   the bindings generators and validate that this works using `fidl_compatibility_test`.
5. Remove the placeholder fields from the structs identified above.

## Documentation and examples

The FIDL documentation already includes (invalid)
[examples][identifiers] of empty structs.
The language documentation could simply remove the
[constraint on zero-length structs][structs].
The [wire format][wire-format]
definition would gain a description of how to encode empty structs.

## Backwards compatibility

This change is backwards compatible as it loosens rather than constrains the FIDL language.
If interface authors replace a semantically empty struct that contains just
a byte sized placeholder field (`boolean`, `uint8`, `int8`) with an empty struct then
that will be even be represented the same on the wire.

## Performance

This should have no impact on build or IPC performance but for languages that
allow empty structs they will save on the work of moving around meaningless values.

## Security

No impact.

## Testing

The compilation of empty structs will be added as a `fidlc` test.
The wire format of empty structs will be tested as part of the `fidl_compatibility_test`.

## Drawbacks, alternatives, and unknowns

We could leave things the way they are but this seems like a simple improvement.

## Prior art and references

Most programming languages and IDLs allow empty structs.

[identifiers]: /docs/reference/fidl/language/language.md#identifiers
[structs]: /docs/reference/fidl/language/language.md#structs
[wire-format]: /docs/reference/fidl/language/wire-format/README.md#Structs
