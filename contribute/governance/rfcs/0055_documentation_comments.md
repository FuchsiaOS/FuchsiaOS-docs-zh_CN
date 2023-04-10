{% set rfcid = "RFC-0055" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-009.

## Summary

Documenting FIDL.

### Relation to other RFCs

This RFC was amended by:

* [RFC-0043: Documentation comment format](0043_documentation_comment_format.md)

## Motivation

Not only is good documentation an important part of scaling a team, documenting
our APIs is an important part of defining a stable API. The Fuchsia API is
primarily implemented in FIDL and a large amount of documentation is being left
in comments that is hard to surface. Even worse, it is not uncommon for people
to look at generated bindings to figure out how to use an interface. This
proposal is the first step of a comprehensive documentation strategy for the
FIDL language and it's interfaces.

## Design

There are two proposed FIDL source language changes. A standard `Doc` attribute
and syntactic sugar to improve the ergonomics of writing documentation.

### Doc Attribute

Arbitrary attributes are an already supported component of the FIDL language.
Standardizing on the `Doc` attribute is for the tooling that generates the
formated documentation. Using an attribute as the basis of the documentation
tooling also allows for additional options to be added to the formatted output
without breaking changes.

```fidl {:.devsite-disable-click-to-copy}
[Discoverable, Doc = "Primary Bluetooth control service to access bluetooth"]
interface Control {
  ...
  [Doc = "Sets the public Bluetooth |name| for this device"]
  10: SetName(string? name) -> (fuchsia.bluetooth.Status status);
}
```

Currently, there can be only one `Doc` attribute per language element. This
makes it so all text must be put into the Attribute braces, which could lead to
excessively long lines.

### Syntactic Sugar

To address the poor ergonomics of using attributes, a layer of syntactic sugar
is proposed.

This involves a small change to the FIDL language specification. Comments are
currently ignored during the lexing of FIDL. This FTP does not address adding
general comments to the AST, only documentation comments.

Attributes are the primary way that the FIDL language expresses the idea of
metadata attached to structures. Having documentation comments as just a special
case of this simplifies the consumption of the metadata in the IR.

The suggested modification of the grammar is in the appendix of the FTP, but
mostly involves adding an additional rule and minor rule re-ordering.

```none {:.devsite-disable-click-to-copy}
documentation-comment = "///", STRING-LITERAL, "\n"
```

```fidl {:.devsite-disable-click-to-copy}
interface Control {
  /// Sent when an adapter with the given |identifier| has been
  /// removed from the system.
  10102: -> OnAdapterRemoved(string identifier);
}

```

This would de-sugar to:

```none {:.devsite-disable-click-to-copy}
[Doc="Sent when an adapter with the given |identifier| has been\n removed from the system\n"]
```

### Documentation comment contents

Doc comments are primarily freeform text. Any specific style of formatting is up
to the author, team, or future style guides. The only primitive added is an
identifier marker, currently proposed as pipes (|) encasing the local
identifier. Unqualified identifiers are scoped to the members beneath the object
that the attribute is attached too. A fully qualified identifier can be used
(Ex: |fuchsia.bluetooth.ErrorCode|) to refer to objects outside of the current
scope.

Eventually, fidldoc documentation generation should fail if any of the
identifiers are missing, but the attribute will still be included and passed
into language bindings. This will prevent documentation rot. Adding the
identifiers to the IR or as part of the parsing step is intentionally avoided
due to complicating these steps. Extracting the identifiers belongs in the
documentation tooling (fidldoc). Documentation generation should be added as a
mandatory part of a standard debug build and the overall build should fail if
the documentation is not successfully generated.

### Additional tooling

A standard tool, called fidldoc, should be added to the tools directory. Fidldoc
will generate markdown after consuming the FIDL JSON IR. Markdown is the current
format that we are using with the other first-class language's documentation
tooling.

### Other

The Wire format is unaffected by these changes. How language bindings chose to
surface docstrings, or if they surface them, is left as an implementation detail
for their respective communities or potentially as additional FTPs.

The style guide should be amended to prefer `///` over the doc attribute, but
otherwise is left alone.

## Documentation and examples

Triple comments are a relatively common way of  denoting documentation comments
and should not be a large barrier to understanding the fidl language. Example
using triple comments should be added to the existing documentation as well as
an explanation of how to use the attribute annotations.

The primary way that people will consume this feature is in the generated
output.

Backwards Compatibility This feature is already backwards compatible with all
recent prior fidlc compilers. While the new functionality will not exist for the
triple comment syntactic sugar, they do not break earlier compilers.

Document attribute comments will work without any language changes.

## Performance

No performance changes are expected except for a small increase in the JSON IR
size. We will also generate documentation at compile time, which will slow down
the build a small amount.

## Security

n/a

## Testing

n/a

## Drawbacks, alternatives, and unknowns

General agreement of the approach and the specific syntax used are required for
adoption. The syntax is easily modified (and bikeshedded) and does alter the
core ideas of the proposal.

Potential alternatives with regards to fidldoc is that the compiler generates
the documentation itself. It may also be worth using the existing backend
generator approach for this. Output format of the generated documentation may
also be up for discussion.

Another alternative is to represent the documentation comments as first-class
citizens in the AST. While there aren't any real drawbacks to this strategy, you
lose some of the extensibility benefits of modeling it as an attribute. Someday
we may want to add additional information for our documentation tooling and the
attribute style makes this possible without breaking changes. For example, we
may want to allow specifying the markdown language of the comment. This would
then keep all of the information for generating the documentation within the
same output (the attributes). It also enforces a nice regularity where the doc
comments and attributes, which have similar placement constraints, are parsed in
the same way.

## Prior art and references

Most languages have documentation tooling. This draws from prior art of dartdoc,
rustdoc, and javadoc (mostly in what not to do)
