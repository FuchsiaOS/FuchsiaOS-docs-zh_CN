# FIDL API compatibility testing

We use API compatibility tests to ensure that SDK Users targeting supported platform
versions aren't broken by changes to FIDL APIs at tip of tree. All FIDL APIs published in
the [partner Fuchsia SDKs][SDK Categories] are automatically tested for backward API
compatibility. This document describes what API compatibility tests are and how to use
them.

## Concepts

### FIDL versioning

The reader should be familiar with [FIDL versioning].

### API levels

To understand API compatibility tests, it's important to have a basic understanding of
[Fuchsia API levels]. An API level denotes the set of APIs available when building an
application. They are unsigned 64-bit integers which are assigned to the Fuchsia platform
in increasing order.

There are two API levels that are useful to keep in mind:

1. The active API level - This is what Fuchsia developers make additive changes to.
2. The current API level - This is what the petals target.

Usually active == current, except during API freezes when active == current + 1.

The current implementation of platform versioning does not yet reflect this:
In the Fuchsia source tree we track the "current" API level and the set of
"supported" levels at [//build/config/fuchsia/platform_version.json](/build/config/fuchsia/platform_version.json).
Supported levels are levels that cannot be changed, and we do not explicitly record
the "active" level in this file.

The above file is not to be confused with [//sdk/version_history.json](/sdk/version_history.json)
which records API and ABI version history.

### API level evolution

An API level goes through several phases, illustrated by the following diagram:

```
         +--------+ freeze +--------+  bump  +-----------+  drop  +-------------+
START -> | active | -----> | stable | -----> | supported | -----> | unsupported |
         +--------+        +--------+        +-----------+        +-------------+
```

__Active__

In this phase the API level is in active development. End users target this level and
Fuchsia contributors make additive changes to it. Compatibility tests must pass on CI/CQ.
Breaking changes to APIs introduced at this level are not allowed and contributors
should make sure there are no partners still relying on APIs removed at this level.

__Stable__

The API level can no longer receive changes. Contributors should start introducing APIs at the
next level. When we "freeze" an API level, we enter a week-long stabilization period during which
the level may no longer receive changes. This usually happens immediately before a branch cut.

__Supported__

When we bump the active level from N to N+1, we say that N is now supported and we officially stop
accepting changes to it. It will remain supported for at least 6 weeks or until the Fuchsia platform
drops support for it. APIs can be deprecated at this level but not deleted.

__Unsupported__

When we drop support for a level, Fuchsia contributors are free to delete or modify any APIs at this level and we
stop running compatibility tests for this level. There's no longer any guarantee that end users can successfully
target this API level.

## Resolving compatibility issues

Usually compatibility issues can be fixed by adding `@available` annotations on FIDL
declarations.

{% set in_development_api_level = 12 %}
Below are some good guidelines to follow when changing FIDL APIs.

1. Annotate new, unstable APIs with `@available(added=HEAD)`.
1. Annotate new, stable APIs with `@available(added={{ in_development_api_level }})`.
1. When removing an API, first make sure no parters are still using the API, then
   annotate the old API with `@available(removed={{ in_development_api_level+1 }})`.

For more examples, see the [FIDL compatibility guide].

[FIDL versioning]: /docs/reference/fidl/language/versioning.md
[Fuchsia API levels]: /docs/contribute/governance/rfcs/0002_platform_versioning.md
[SDK Categories]: /docs/contribute/governance/rfcs/0165_sdk_categories.md
[FIDL compatibility guide]: /docs/development/languages/fidl/guides/compatibility/README.md
