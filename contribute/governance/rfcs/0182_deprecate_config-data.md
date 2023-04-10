<!-- Generated with `fx rfc` -->

<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0182" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes:

- Declaring the `[config-data]` mechanism as deprecated.
- Establishing an allowlist for existing use cases.
- Over time, replacing existing usages of `config-data` with other, better
  solutions.
- Eventually removing support for `config-data` from Fuchsia.

## Motivation

The immediate goal of this RFC is to establish rough consensus for creating an
allowlist for existing `config-data` usage, with a preference not to grow said
allowlist. Additional information not pertaining to this immediate goal is
provided for extra context and at the request of various stakeholders.

`[config-data]` is a mechanism for configuration files that modulate the
behavior of a packaged component by making certain files available in that
component's namespace at runtime.

`config-data` creates "spooky action at a distance" using an unenforced contract
or convention over the package name of the target component. This is problematic
because package names are not useful as stable identifiers that can become part
of the Fuchsia SDK surface, be versioned with the SDK, and evolve over time such
as by declaring support windows and allowing for soft transitions. Experience
has shown that contracts based on package names make for brittle runtime
assumptions and have high maintenance costs.

Additionally, since `config-data` mixes content from a developer's package with
content from a base package that contains the configuration files, this creates
a confusing developer experience where developers would push a new revision of
their package but see old configuration files at runtime. Often, developers ask
why is it that they pushed an updated version of their package but they're
seeing stale files under `/config/data`, not realizing that these files come
from a different package. Explaining this behavior to developers requires
exposing them to platform implementation details such as "base packages", which
demonstrates a failure of abstraction and is undesirable. Not to mention the
loss of productivity due to this recurring workflow inconsistency.

`config-data` has been very valuable to solving problems where actors at
different stages of product assembly needed to influence components that were
outside of their direct control. Using `config-data`, configuration values that
originate from one place (a repository, a build system, etc) could be used to
modulate the behavior of components that originate from elsewhere.

In some cases components and configurations have the same origin, but users
still prefer to use `config-data` as an integration point. This allows users for
instance to generate one package and sideload different configuration values to
components included in this package. This is useful when the cost of generating
multiple packages is high, for instance if each package flavor needs to be
uploaded individually to a subsequent integration point.

Nowadays, multiple alternatives to `config-data` exist. These include other
approaches to routing a configuration directory based on
[directory capabilities][directory-capability],
[structured configuration][structured-configuration],
providing configuration via protocol capabilities, and
[packaging configuration files][hermetic-data] with the components that consume
them.

While it is not an immediate priority to deprecate existing usages of
`config-data` and the feature itself, we should steer new usages towards modern
solutions and nudge owners of existing usages to migrate to modern alternatives.

The best practice in such cases is to establish an allowlist for the
`config_data()` GN rule and to initialize it with existing usage. The allowlist
is expected to trend towards zero over time, however new usages can be
allowlisted if it's not clear that the modern alternatives are applicable. When
new usages are admitted the change to allow this should state the rationale
up-front, in order to reduce code review churn.

## Stakeholders

- Component Framework team: created `config-data` and its present alternatives.
- Build team: maintainers of `config-data` build-time logic.
- Software Assembly team: maintainers of `config-data` assembly-time logic.
- Current users of `config-data`, which span many teams.

_Facilitator:_

- hjfreyer@google.com

_Reviewers:_

- aaronwood@google.com
- adamperry@google.com
- awolter@google.com
- ddorwin@google.com
- fmil@google.com
- geb@google.com
- jsankey@google.com
- kpozin@google.com
- wez@google.com
- yaar@google.com
- ypomortsev@google.com

_Socialization:_

A draft for this RFC was reviewed by Component Framework and Software Assembly
technical leads before it was published.

## Background

### How `config-data` works

In legacy `appmgr` (aka CFv1), `config-data` was implemented by matching a
launched component's package name against a subdirectory in the `config-data`
base package. If a match was found, the matching subdirectory was made available
in the launched component's namespace under the path `/config/data`.

Defining `config-data` is done by specifying files to be made available, paths
at which they should be made available in a component's incoming namespace (by
convention, under `/config/data`), and one or more package names for components
that should have these files available at these paths.

For instance, a component might request to use configuration files like so:

```json5
{
    use: [
        {
            directory: "config-data",
            rights: [ "r*" ],
            path: "/config/data",
        },
    ],
}
```

This expresses that the component requires embedders to provide a read-only
directory capability named "config-data", that will be presented by the
framework at `/config/data`, to allow existing code relying on `/config/data`
to work unmodified.

The parent component or realm might have these declarations:

```json5
{
    children: [
        {
            name: "font-provider",
            url: "fuchsia-pkg://fuchsia.com/fonts#meta/font-provider.cm",
        },
    ],
    offer: [
        {
            directory: "config-data",
            from: "parent",
            to: [ "#font-provider" ],
            subdir: "fonts",
        },
    ],
}
```

This accepts a `config-data` containing multiple sub-directories, from the
parent, and routes the `example` sub-directory, containing configuration data
for the font-provider, to that component.

The contents of `config-data` might come from build definitions such as this:

```gn
{% verbatim %}
import("//build/config.gni")

config_data("example_config_data") {
  for_pkg = "example"
  sources = [
    "file1.dat",
    "file2.dat",
    ...
  ]
  outputs = [ "{{source_file_part}}" ]
}
{% endverbatim %}
```

The target above, and others like it, are collected by the build system to form
the contents of a package named `config-data`. Then, purpose-built routing rules
route all the contents of the `config-data` package, organized into
subdirectories as indicated by the `for_pkg` parameter shown above, to the core
realm and elsewhere, where they are distributed as further configured to various
components that consume them.

### How `config-data` is used today

`config-data` has many use cases. Below are some illustrative examples:

- ICU data and tzdata: data for the ICU library, and specifically timezone data
  ("tzdata"), is provided at runtime as `config-data`. Defining a single source
  of truth for these data files in the Fuchsia platform sources and providing
  the files to components from various sources (such as Chromium, Flutter,
  internal Google repositories etc) is used to ensure that exactly one revision
  of these files is used.
  This achieves consistency (e.g. all components agree on a single tzdata
  regardless of their origin) and efficiency (all components share the same ICU
  blobs regardless of when or where they were built).
- Values for the `buildinfo` and `hwinfo` components are provided as
  `config-data`. These components are built from platform source code but may
  need to be configured by products. Currently `config-data` serves as this
  configuration mechanism.
- The Settings UI component, defined in platform sources, can be configured to
  behave differently on different devices that expose different hardware
  toggles. For instance SetUI's behavior is different on Astro vs on Sherlock
  devices in a manner currently governed by `config-data`.
- The platform font provider component can be configured to serve
  product-specific fonts. The font files and a manifest describing their
  properties are provided as `config-data` that is added in out-of-tree product
  assembly.

## Design & implementation

A build-time regression stop will be established to prevent new uses of
`config-data` without explicit approval. An allowlist of existing usages will be
checked in, and changes to this allowlist will be governed by an `OWNERS` file.
Owners will be assigned from the Component Framework team to represent
fuchsia.git, and from petals that use `config-data` to represent their usage.
Representatives will be responsible to manage their respective allowlist
entries, for instance assist in refactors or targeted burndowns.

It's not important how the regression stop is implemented. A common and likely
implementation strategy is to change the `config-data` GN template to add a
dependency on a target with a set visibility list. Notably, this only covers
in-tree usage, but restricting new usage of `config-data` in out-of-tree
product assembly is also important. An appropriate mechanism may be developed
here as well, in coordination with the PDK team.

An in-depth of the alternatives to `config-data` is outside the scope of this
RFC. Review the documentation linked above when referring to these alternatives.

Establishing best practices for providing configuration to components, or
seeding a migration guide away from `config-data`, are outside the scope of this
RFC. There is ongoing work to produce such documentation, and it will be
published separately from this RFC.

## Performance

A specific and interesting aspect of the `config-data` package is that all files
are packaged under the path `meta/`. This means that the files are archived in
the `meta.far` file. When stored in the underlying blobfs filesystem, file sizes
on disk are rounded up to a block size, typically 8KiB. By archiving
configuration files together, the added overhead of rounding up is removed. This
is important because configuration files are often numerous and small, so the
total overhead can actually be greater than the sum of compressed sizes of these
files.

When using alternative solutions, in instances where storage space matters, the
same technique or an equivalent should be used to ensure parity on storage
efficiency.

## Ergonomics

Alternatives to `config-data` have better ergonomics, most importantly because
they don't rely on brittle contracts based on package names and on "action at a
distance".

## Backwards Compatibility

Migrations away from `config-data` will sometimes need to be done as soft
transitions. During the transition period, the component that is consuming the
configuration data must be able to accept both forms of input, `config-data` and
the chosen alternative.

## Security considerations

This RFC is not introducing any new configuration mechanisms, all the mechanisms
we would use as an alternative to config-data are already present in the system
and have been through their own security review. Component authors should
consult with security when designing or changing the configuration for
security-relevant features.

## Privacy considerations

This RFC is not introducing any new configuration mechanisms, all the mechanisms
we would use as an alternative to config-data are already present in the system
and have been through their own privacy review. Component authors should consult
with privacy when designing or changing the configuration for privacy sensitive
features.

## Testing

The alternatives presented to `config-data` all have established best practices
for testing. Refer to documentation for specific features for testing
information. For instance, see the guide for [testing structured
configuration][structured-configuration-testing] using [Realm
Builder][realm-builder]. Replacing structured configuration values
under test is as easy as:

```cpp
realm_builder.SetConfigValue(child_name, key, value_for_test);
```

When we need to provide configuration data as files to a component under test,
such as in an integration test, the files can be packaged with the test and then
routed from the test component (at the test realm root) to the component under
test as a data directory. For instance:

```json5
{
    ....
    capabilities: [
        {
            directory: "test_config",
            rights: [ "r*" ],
            path: "/test_config",
        },
    ],
    children: [
        {
            name: "component_under_test",
            ...
        },
    ],
    offer: [
        {
            directory: "test_config",
            from: "self",
            as: "config",
        },
    ],
}
```

## Documentation

To support transitions away from `config-data`, a migration guide will be
provided. The migration guide will help developers choose the best alternative
for their use case, and refer them to additional documentation.

## Notes on staffing and churn

Accepting this RFC does not impose any sort of requirement on existing users of
`config-data` to migrate to modern alternatives. If a mandate for a migration is
given, then there should be a core migration team staffed to perform the
majority of the work. The author proposes that no such work should begin until
after the CFv2 migration is complete, but we should hurry up and land a
regression stop.

Until such a time that there is dedicated staffing for a migration, updates to
the allowlist such as to support refactors should be reviewed and approved
within one business day.

## Drawbacks, alternatives, and unknowns

Structured configuration is a new and evolving mechanism. For instance some
features that are needed for expressing configurations, such as additional data
types, are not yet implemented.

Capability routing is not currently subject to platform versioning. It is the
plan of record to build support for
[modulating capability routes on ABI revisions][rfc-0002-cf], but this mechanism
has not been designed yet.

[directory-capability]: /docs/concepts/components/v2/capabilities/directory.md
[hermetic-data]: /docs/development/components/data.md#hermetic_data_files_with_resource
[hwinfo]: /sdk/fidl/fuchsia.hwinfo/hwinfo.fidl
[realm-builder]: /docs/development/testing/components/realm_builder.md
[rfc-0002-cf]: /docs/contribute/governance/rfcs/0002_platform_versioning.md#component_framework
[rfc-0173]: /docs/contribute/governance/rfcs/0173_structured_config_cf_apis.md
[structured-configuration]: /docs/development/components/configuration/structured_config.md
[structured-configuration-testing]: /docs/development/components/configuration/structured_config.md#testing_with_realm_builder
[config-data]: /docs/development/components/data.md#product-specific_configuration_with_config_data
