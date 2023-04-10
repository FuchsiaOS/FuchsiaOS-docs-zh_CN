# API Development

This document is a top-level entry point to documentation related to developing
APIs for Fuchsia.

## What this covers

Although the documentation in this directory applies to all Fuchsia APIs, it
will be enforced for the _public facing surface area_ of Fuchsia: the Fuchsia
APIs that are surfaced to developers via IDK releases.  All public facing API
changes will be reviewed by the [API Council][api-council] for consistency with
these guidelines.

## Guidelines

Before you begin working on a Fuchsia API, it is important to understand how
an API is versioned in Fuchsia:

* [Fuchsia API evolution guidelines](evolution.md)

## Rubrics

The documentation in this directory comes in the form of _rubrics_, which are
established protocols for how to design and build APIs.  Note that the list
below is not complete: as Fuchsia evolves, more rubrics will be added.

 * [API Documentation](documentation.md)
 * CLI and GUI tools
   * [Developer tool guidelines](tools.md)
   * [Command-line tools rubric](cli.md))
   * [CLI tool help requirements](cli_help.md)
 * Languages
   * [C API Readability](c.md)
   * [Dart API Readability](dart.md)
   * [FIDL Style][fidl-style]
   * [FIDL API][fidl-api]
   * [Go rubric](go.md)
   * [Rust rubric](rust.md)
 * Domain-specific areas
   * [Zircon System Interface](system.md)

<!-- xrefs -->
[api-council]: /docs/contribute/governance/api_council.md
[fidl-style]: /docs/development/languages/fidl/guides/style.md
[fidl-api]: /docs/development/api/fidl.md
