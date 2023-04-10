# Fuchsia API evolution guidelines

This section contains guidelines for Fuchsia contributors making changes to
Fuchsia Platform APIs. Before you begin, you should be familiarized with the
following concepts:

- [FIDL Versioning RFC](/docs/contribute/governance/rfcs/0083_fidl_versioning.md)

## The lifecycle of a platform API {#lifecycle}

Fuchsia platform APIs should follow the lifecycle:
_Added → Deprecated → Removed → Deleted_, as illustrated below:

![This image shows the lifecycle of an API on the Fuchsia platform starting
  with the version when the API was added, then deprecated, then removed, and
  finally deprecated](images/platform-api-lifecycle.png "Fuchsia platform API
  lifecycle")

The following sections explain how to manage this lifecycle as an API developer.

### Adding FIDL APIs {#adding}

Always annotate new FIDL APIs with an
[@available](/docs/reference/fidl/language/versioning.md)
attribute. Unstable APIs should be added at the HEAD API level. For example:

```fidl
@available(added=HEAD)
library fuchsia.examples.docs;
```

Stable APIs should be added at the
[in-development API level](/build/config/fuchsia/platform_version.json).
For example:

```fidl
// At the time of writing the in development level was 10.
@available(added=10)
library fuchsia.examples.docs;
```

When a FIDL library has more than one `.fidl` file, the library should include a
separate `overview.fidl` file and the `@available` attribute should be written in
that file along with a documentation comment describing the library. See
[the FIDL style guide](/docs/development/languages/fidl/guides/style.md#library-overview)
for more information.

Every API in the partner [SDK category](/docs/contribute/sdk/categories.md)
is opted into static compatibility testing in CI/CQ. These tests fail when
an API changes in backward incompatible ways. If your API is unstable, consider
adding it to the internal or experimental SDK categories to prevent partners from
depending on it and to opt out of static compatibility tests, allowing the API
to change freely. Once the API is stable, add it to the partner category.

### Deprecating FIDL APIs {#deprecating}

You should always deprecate an API at an earlier level than you remove it. When
an end developer targets a deprecated API, they see a warning at build time that
the API is deprecated and they should migrate to an alternative. You should
include a note to help the end developer find an alternative. For example:

```fidl
protocol Example {
    // (Description of the method.)
    //
    // # Deprecation
    //
    // (Detailed explanation of why the method is deprecated, the timeline for
    // removing it, and what should be used instead.)
    @available(deprecated=5, removed=6, note="use Replacement")
    Deprecated();

    @available(added=5)
    Replacement();
};
```

There must be at least one API level between an API's deprecation and removal.
It is perfectly fine, however, to deprecate an API at the same level that it was
added. For example:

```fidl
// These are OK.
@available(deprecated=5, removed=6)
@available(deprecated=5, removed=100)
@available(added=5, deprecated=5)

// These will not compile.
@available(deprecated=5, removed=5)
@available(deprecated=5, removed=3)
```

### Removing FIDL APIs {#removing}

Note that you should always [deprecate](#deprecating) an API
before removing it, and you should [preserve the ABI](#preserving)
when removing an API whenever possible.

The recommended way to remove an API is to use its @available attribute. This is the
method we generally recommend. For example, if an API was added at level 11, it
can be removed at level 12 like this:

```
@available(added=10, removed=12)
library fuchsia.examples.docs;
```

In this example, an end developer targeting levels 10 or 11 would see client
bindings for the fuchsia.examples.docs library, but a developer targeting level
12 or greater would not. If this API's source is removed before the platform
drops support for API level 12, the API's static compatibility tests will fail
and special approval from //sdk/history/OWNERS will be required to submit the
change. When the Fuchsia platform drops support for API level 12 the API's source
code can be deleted.

Alternatively, you can delete the API's source code which is not recommended for
most use cases. If the API was added at the in development API level or a
previous API level which is currently supported,  this removes the API from
Fuchsia's history which is generally not allowed. Static compatibility tests
will fail in this case and you will need special approval from
`//sdk/history/OWNERS` to submit the changes. If the API was added at any level
greater than the in development API level - including the special `HEAD` API
level  - then this method of removal is fine.

### Preserving ABI when removing FIDL APIs {#preserving}

It's possible to remove an API's client bindings from SDKs - preventing future
end developers from targeting the API - while preserving the platform's
implementation of the API (The ABI). This feature allows existing applications
to run on newer versions of the platform. When an API has been removed from SDKs
and the platform still supports its ABI, we say the platform has legacy
[support](/docs/reference/fidl/language/versioning.md#legacy)
for that API.

To maintain legacy support for an API, set legacy=true when removing the API.
For example:

```fidl
protocol LegacyExample {
    @available(added=10, deprecated=11, removed=12, legacy=true)
    LegacyMethod();
};
```

All methods in the Fuchsia platform should retain legacy support when they are
removed. Once the Fuchsia platform drops support for all API levels before the
method's removal, it is safe to remove `legacy=true` and the method's
implementation.