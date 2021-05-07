{% set rfcid = "RFC-0042" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-042.

## Rejection rationale

[Poisson d'avril][april-fools]!

However, the underlying idea of enforcing presence in contexts where
nullability is supported by the layout (e.g. table fields, nullable type
alias) is one that will likely be incorporated in another FTP.

## Summary

FIDL library authors can mark some types and members as nullable with `?`.
This proposes a way to mark types and members as not nullable.

## Motivation

With type aliasing, a library author can make declarations incorporate
nullability into a type, for example:

```fidl
using MiddleName = string:255?;
```

It can be valuable to explicitly mark a nullable type as non nullable,
for example:

```fidl
struct FullName {
  GivenName given_name;
  vector<MiddleName> middle_names; // names shouldn't be null
  FamilyName family_name;
}
```

Another example is for "boxed" types placed out-of-line:

```fidl
// An "out-of-line" full name
struct InappropriateFullName {
    option<FullName> full_name; // full_name shouldn't be null
};
```

## Design

This proposal introduces the non-nullable specifier for types that will mark
a nullable type as non-nullable.
It's a change to the source language but not the wire format, IR or bindings.

In the cases described above, we would use:

```fidl
struct FullName {
  GivenName given_name;
  vector<MiddleName¿> middle_names;
  FamilyName family_name;
}
```

And:

```fidl
struct InappropriateFullName {
    option<FullName¿> full_name;
};
```

For symmetry with the syntax of type nullability the inverted question mark
symbol is used.
This is Unicode code-point `\u00BF`, [easily typed] on any keyboard.

Possible other use cases are adding non-nullable `table` members:

```fidl
table FullName {
  GivenName¿ given_name;
  MiddleName middle-name;
  FamilyName¿ family_name;
};
```

Or specifying that a particular `xunion` member must always be present:

```fidl
xunion PreferredOperatingSystemVersion {
  float32 Windows;
  float32 MacOS;
  float32 Linux;
  float32¿ Fuchsia;
};
```

## Implementation strategy

This is not a breaking change but the need for it has recently increased with
the new support for generalized type aliases in FIDL.

## Ergonomics

Non-nullable types are a popular feature of languages like Kotlin, C#, Swift
and Typescript.
It's a [heavily requested feature](https://github.com/dart-lang/sdk/issues/22)
for Dart.

## Documentation and examples

The grammar and language documentation will have to be updated.

## Backwards compatibility

This feature is backwards compatible.

## Performance

Some non-nullable types (such has structs) have a more compact representation
than their nullable variant so this will allow for more efficient transport.

## Security

Null references have been described by Tony Hoare as a
"[billion dollar mistake]" and can be responsible for security
vulnerabilities.

## Testing

As a compiler change with no impact on IR or bindings we will need to add
tests for the fidl frontend compiler.

## Drawbacks, alternatives, and unknowns

Some concerns have been raised about the ISO 8859-1 taking the place of
US-ASCII characters in FIDL syntax but this character
[gets the job done].

## Prior art and references

As mentioned above Kotlin, C#, Swift and Typescript all allow types to be
declared as non-nullable.

<!-- xrefs -->
[april-fools]: https://fr.wikipedia.org/wiki/Poisson_d%27avril
[easily typed]: https://en.wikipedia.org/wiki/Inverted_question_and_exclamation_marks#GNU/Linux
[heavily requested feature]: https://github.com/dart-lang/sdk/issues/22
[billion dollar mistake]: https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare
[gets the job done]: https://www.youtube.com/watch?v=6_35a7sn6ds
