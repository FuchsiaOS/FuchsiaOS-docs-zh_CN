# FIDL

FIDL (**F**uchsia **I**nterface **D**efinition **L**anguage) is the IPC system for Fuchsia.

## Start here

The [language tutorials][tutorials] presents a simple "*Hello, world*" client
and server, showing the FIDL language definitions and continuing with sections
specific to each supported target language (e.g., C++, Dart).

Read the [concepts doc][concepts] to get a brief overview of what FIDL is,
including some of its design goals, requirements, and workflow.

## Developer Guides

* [Running FIDL][cli] &mdash; quick CLI overview of the various programs in the
  FIDL toolchain
* [Style Guide][style]
* [Designing APIs][designing-apis]
* [ABI and API compatibility guide][abi-api-compat] &mdash; details the possible
  transitions for each FIDL type, taking into account ABI and API compatibility
* [Maxing Out Pagination][pagination] &mdash; help on determining how much data
   can fit into a single message
* [C Family Binding Comparison][c-family] &mdash; how to decide, which binding
  (new C++, HLCPP, or C) to use

## References

The [bindings reference][bindings-ref] includes references of generated code in each binding and the FIDL
utility libraries available to each language.

The [language reference][language-ref] is for everything else, like the
[wire format][wire-format] and [RFCs][rfc].

## Contributing
Please read the [contributing doc][contributing] for more information.

<!-- xrefs -->
[cli]: /development/languages/fidl/guides/cli.md
[style]: /development/languages/fidl/guides/style.md
[designing-apis]: /development/languages/fidl/guides/api-design.md
[abi-api-compat]: /development/languages/fidl/guides/compatibility/README.md
[pagination]: /development/languages/fidl/guides/max-out-pagination.md
[c-family]: /development/languages/fidl/guides/c-family-comparison.md

[tutorials]: /development/languages/fidl/tutorials/overview.md
[concepts]: /concepts/fidl/overview.md
[contributing]: /contribute/contributing-to-fidl
[bindings-ref]: /reference/fidl/bindings/overview.md
[language-ref]: /reference/fidl/language/language.md
[wire-format]: /reference/fidl/language/wire-format
[rfc]: /contribute/governance/rfcs/README.md
