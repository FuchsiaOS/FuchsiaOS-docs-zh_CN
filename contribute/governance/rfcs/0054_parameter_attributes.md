{% set rfcid = "RFC-0054" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-054.

_"A Chance to write self documenting APIs"_

## Summary

We can already apply attributes to various elements of FIDL but not to parameters. This is a proposal to extend the grammar to accept parameter attributes.

### Relation to other RFCs

This RFC was superseded by:

* [RFC-0050: Syntax revamp](0050_syntax_revamp.md)

## Motivation

Having attributes on more language elements increases the language consistency
and also an enabler for further features in the future. Any fact about the API
that cannot be encoded in the type system can be encoded in attributes instead.
Any fact that cannot be encoded in the type system can be captured with more
structure in attributes, simplifying late validation in various backends. These
attributes can be useful for linting and generators can also leverage this
information to generate better code or propagate this to the attributes of the
target language. This can help static and/or dynamic analysis of the target
language. Moreover, attributes can be a great way to prototype possible
extensions or refinements to the type system.

The driving use case for this feature is to check for handle leak, double free
and use after free errors.

The Kernel ABI (i.e. syscalls) are also expressed in FIDL format and often the
ownership of handles are not clear. Should the user call handle_close after a
task_kill syscall or not? This is not clear from the documentation. Other
syscalls have very clear documentation but it can only be checked by manually:

```fidl
[Transport = "Syscall"]
protocol handle {
    /// Close a handle.
    /// Rights: None.
    handle_close(handle handle) -> (status status);

    /// Close a number of handles.
    /// Rights: None.
    handle_close_many(vector<handle> handles) -> (status status);

    /// Duplicate a handle.
    /// Rights: handle must have ZX_RIGHT_DUPLICATE.
    handle_duplicate(handle handle, rights rights) -> (status status, handle out);

    /// Replace a handle.
    /// Rights: None.
    handle_replace(handle handle, rights rights) -> (status status, handle out);
};
```

After we enable attributes on parameters we can replace the comments with
attributes. Those attributes are as readable as comments and they can help
generators to generate code that can be checked using static and dynamic
analysis tools, like [this](https://reviews.llvm.org/D70470). The kazoo tool can
generate the corresponding annotations for C and C++ bindings making the [Clang
Static Analyzer](https://clang.llvm.org/docs/ClangStaticAnalyzer.html) capable
of catching handle misuse errors.

The prototype already found a number of bugs in Fuchsia.

## Design

The proposal is to add parameter attributes to the FIDL source language. These
attributes should be exposed in the JSON IR, in a similar fashion than other
attributes. We might need to update the formatter. It does not affect the wire
format. The language bindings are only affected by introducing specific
annotations that can change the generated code but they are not affected by this
proposal directly.

Our proposal only changes one production rule of the [FIDL
grammar][grammar]:

```
- parameter = type-constructor , IDENTIFIER ;
+ parameter = ( attribute-list ) , type-constructor , IDENTIFIER ;
```

We also need to diagnose triple slash documentation or doc attributes on
parameters.

## Implementation strategy

This change is backward compatible, no migration required. There are plenty of
documentation that needs to be updated along with the feature. This proposal
would change the parser and add new information to the IR. A potential change to
the generators can be made separately, when new parameter attributes are
introduced.

## Ergonomics

The has the potential of making FIDL APIs easier to understand and bindings more
friendly to static and dynamic analysis. The additional complexity cost should
be low. See next section for examples with potential parameter attributes.
Editor support might need a small update as well.

## Documentation and examples

The grammar documentation and the language reference need a small update. We
might want to add some style guidelines how to break lines in certain cases for
parameter attributes. This might not be very important as attributes are not
mentioned in the current style guide at all.

See the [motivation section](#motivation) for examples.

## Backwards compatibility

This proposal maintains both FIDL source and wire ABI compatibility. The
specific attributes that are introduced later might cause wire ABI
incompatibility. Those attributes should be required to pass the RFC process
separately.

## Performance

Future attributes might have a good impact on performance. More information
about the APIs could potentially help the optimizer of the target language.

## Security

The specific attributes might improve security as they can aid static and
dynamic analysis tools in the target languages.

## Testing

Tests will be written for fidlc to ensure correct parsing, sensible diagnostics
for compilation failures. The generated JSON IR will also be tested. In order to
be able to test we need to introduce at least one attribute that is applicable
to parameters. Since the generators will not be touched at first, they do not
require additional testing.

## Drawbacks, alternatives, and unknowns

The implementation cost is relatively low.

[RFC-0044](/contribute/governance/rfcs/0044_extensible_method_arguments.md) is a possible alternative. Accepting that proposal
introduces some inconsistencies into the language as one way of describing
parameters would enable users to write parameter attributes while the other does
not. Also, RFC-0044 has a performance costs so some protocols, like syscall
should use it sparingly or not at all.

## Prior art and references

Protobuf have
[options](https://developers.google.com/protocol-buffers/docs/proto3#options) on
messages, which has a somewhat similar behavior to parameter attributes. FIDL
already have attributes on some language elements like members and protocols.

<!-- xrefs -->
[grammar]: /reference/fidl/language/grammar.md
