# API Development

This document is a top-level entry point to documentation related to developing
APIs for Fuchsia.

## What this covers

Although the documentation in this directory applies to all Fuchsia APIs, it
will be enforced for the _public facing surface area_ of Fuchsia: the Fuchsia
APIs that are surfaced to developers via IDK releases.  All public facing API
changes will be reviewed by the [API Council][api-council] for consistency with
these guidelines.

## Rubrics

The documentation in this directory comes in the form of _rubrics_, which are
established protocols for how to design and build APIs.  Note that the list
below is not complete: as Fuchsia evolves, more rubrics will be added.

 * [API Documentation](documentation.md)
 * [CLI and GUI tools](tools.md)
 * Languages
   * [C API Readability](c.md)
   * [Dart API Readability](dart.md)
   * [FIDL Style][fidl-style]
   * [FIDL API][fidl-api]
 * Domain-specific areas
   * [Zircon System Interface](system.md)
   * [Fuchsia Device Interface](device_interfaces.md)

<!-- xrefs -->
[api-council]: /docs/contribute/governance/api_council.md
[fidl-style]: /docs/development/languages/fidl/guides/style.md
[fidl-api]: /docs/concepts/api/fidl.md
