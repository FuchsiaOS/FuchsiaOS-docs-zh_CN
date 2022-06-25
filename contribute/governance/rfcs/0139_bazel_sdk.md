<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0139" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

In order to support [Workstation Out of Tree][1], the workstation.git repository
will include a Bazel SDK that will initially support building and testing a
product from pre-built artifacts. There are also plans to support driver
development out of tree. To support both use-cases, this RFC proposes that the
Workstation Bazel SDK is productionized for general distribution. Prototype
rules are currently developed in workstation.git. A detailed design for general
distribution will be included in a future RFC.

"SDK" does not refer to a replacement or change of Fuchsia's current supported
API.

The Fuchsia project makes available an [IDK][2] and a [GN C++ SDK][3].  The IDK
documentation details that the IDK, also distributed with the name "core SDK"
on CIPD, is intended to be a base to build a "frontend SDK" on top of.  There
is an existing generated Bazel SDK frontend in fuchsia.git, however it is only
intended as a test for the IDK, and is not supported for general usage.  To
avoid confusion, it should be renamed to "Selftest SDK"; the rest of this
document will refer exclusively to the proposed out-of-tree Bazel SDK as the
"SDK" or "Bazel SDK", while the IDK, or "core SDK" will be referred to
exclusively as the "IDK".

## Motivation

According to [RFC-0095][1], Workstation will be built out-of-tree using Bazel.
This proposal is for a roadmap to make these rules generally available for
out-of-tree development beyond Workstation. Several important workflows are not
currently supported by the current GN C++ SDK, most notably including product
and driver development. These use cases should be supported separately from
building the Fuchsia platform.

Supporting a generic, language-agnostic build-system such as Bazel is useful
for a variety of reasons:

Generally, products built on Fuchsia – workstation included – can be expected
to rely on a variety of dependencies from different sources. These include
libraries, executables, components, packages, configuration files, and more.
These artifacts may exist as source code or binary files in different languages
and architectures, and may come from a combination of first party, third party,
public, and private sources.

The IDK includes Fuchsia's complete API surface and a suite of development
tools. While these vary in scope and purpose, the IDK by definition does not
include generic build tools, and direct usage is discouraged - some tools are
most conveniently used from scripts as opposed to invoked manually at a
terminal.

["The contents of the IDK represent the most basic contract that the Fuchsia
platform developers offer to prospective developers. The Fuchsia IDK is not
suitable for immediate consumption. It does not contain any reference to
toolchains or build systems, and in fact does not require any specific instance
of these."][4]

The existing GN C++ SDK is productively used by C++ projects already using GN,
such as Flutter and Chromium, however there is no canonical Fuchsia SDK that
supports official Fuchsia languages besides C++. GN cannot easily be extended
to support all of Fuchsia's official languages, and rules cannot be easily
composed unless they are designed to be combined with each other.

Bazel is a natural fit for building Fuchsia products and Fuchsia software.
Bazel is supported by Google and an active community of third-party developers.
It supports a wide range of runtime-targets and language toolchains by means of
build-rules written in the Starlark language. These build rules can be
published, shared across projects, and are used in production by both
independent developers and large corporations. Starlark is fully testable, and
offers testing utilities and a testing guide. Bazel is designed to be extended
with new languages, new build-actions, and new means of integrating external
dependencies. As a project, Bazel is focused on solving generic issues of
distributed software builds and version management and has a clear scope of
responsibility.

## Stakeholders


_Facilitator:_ hjfreyer@google.com

_Reviewers:_ hjfreyer, mangini, chaselatta, amathes, sethladd, maruel, shayba,
crjohns, ejia, chandarren, lijiaming, dworsham

_Consulted:_ Driver team, SDK team, Infra team, Workstation Team.

_Socialization:_ Sent to eng-council-discuss@fuchsia.dev


## Design

A detailed technical design will be a follow-up to this RFC. This RFC outlines
broad requirements for the RFC but does not specify how to implement them.

### Versioning

The rules should at all times be pinned to the latest LTS version of Bazel.
[Bazelisk] and a `.bazelversion` file should be the recommended way of
installing and invoking Bazel for use with the Bazel SDK.

Each release of the Bazel SDK will also be pinned to a release version of the
IDK and any required toolchains, such as Clang. The Bazel SDK will fetch the
pinned IDK and toolchain archives depending on the host OS. The IDK and
toolchain archives should also be overridable to local files for development.

### Code Location

[Workstation.git] includes prototype Bazel rules to support assembling
products, compiling C++ components, and compiling Flutter components. The rules
are pinned against a particular version of the Fuchsia IDK and make use of
[Distributed Product Integration].

Development will move to a new repository called fuchsia-bazel-rules.git.  This
is because releases of the Fuchsia IDK are not created from a single checkout
of fuchsia.git, and in order to consume the latest version of the IDK, the
Bazel rules must exist downstream from fuchsia.git.

### Distribution

A later RFC will clarify with specific design details for distribution.

### IDK

The Bazel SDK will include a repository rule for instantiating a Bazel external
repository with the contents of an IDK archive as well as generated BUILD
targets for the contents of the IDK, including tools, language-specific
libraries and FIDL definitions. The archive should be downloaded by ffx or
directly from CIPD. The rule may download ffx first in order to obtain the IDK.

### Building

The following are the primary items that the Bazel SDK must provide:

C++, Dart, and Flutter toolchains for building libraries and executables. Rust
is not needed initially for building Workstation but can be added later. The
Bazel toolchain definitions should be hosted in the same repository as the
Bazel SDK, but could be moved externally in the future.

Providers and rules for assembling a Fuchsia product from a combination of
local and remote Fuchsia packages.

Actions for deploying a particular product image to a Fuchsia device, or
emulator.

Rules for compiling and packaging drivers that can be included in product
definitions.

A [Bazel toolchain] that wraps tools from the IDK. The IDK is fetched by Bazel
(potentially via ffx, which may also be fetched by Bazel) to build products
according to the version specified in BUILD/WORKSPACE rules. This is for making
IDK binary tools available to Bazel rules.

A provider and rule for Fuchsia packages that contains a package manifest and
associated files, similar to the in-tree `fuchsia_package` rule.

A provider and rule for creating Fuchsia components, with a component manifest
and associated files, similar to the in-tree `fuchsia_component` rule.

### Testing

This section refers to testing-related Bazel rules, not testing the Bazel SDK
itself. The primary requirement is to support [OOT System Testing].

### Dependencies and External Repositories

These will be provided as analysis-phase repository rules.

- Downloading build artifacts that are built by external systems and hosted on
  TUF. [See Phase 3 of Workstation OOT RFC][16].
- Repository rules for building against a local checkout of the Fuchsia tree.
- Repository rules for building against a particular version of the Fuchsia
  IDK, downloaded from CIPD.

## Implementation

- Prototype rules are being developed in workstation.git
- These rules will move to a new git repository (fuchsia-bazel-rules.git).
- A follow-up RFC will be published detailing the specifics of both
  distribution and the public API surface of the Bazel rules.

## Testing

Initially, testing will run continuously on Fuchsia infrastructure as part of
workstation.git's continuous testing and release process. Tests should be able
to run in a Mac or Linux environment. Windows support is not planned at this
time, and will not be considered until there is a Windows distribution of the
IDK. To keep the rules compatible with that possible eventuality, rules will
avoid use of `run_shell` and actions that rely on built-in toolchains. Go's
toolchain is a good candidate, as it is well supported on Linux, macOS, and
Windows, with good cross-compiling capabilities from all three platforms. See
[rules_go].

The Bazel SDK should include Starlark unit testing. Each rule in the public API
of the Bazel SDK will include tests and examples that will be build as part of
CI.

Bazel can download artifacts (dependencies, toolchains, etc) as part of the
`analysis` phase of its build process.  To support this in Fuchsia CI, Bazel
offers several mechanisms for downloading any necessary artifacts in advance:
[Bazel offline builds].

When the rules move from workstation.git to fuchsia-bazel-rules.git, the CI
recipe will follow the same design as the workstation.git recipe.

## Documentation

[Stardoc][10] should run along with the continuous testing to generate
formatted documentation within the repository. This documentation should be
integrated into fuchsia.dev where additional documentation will explain the
recommended way for users to install Bazel (using [bazelisk]).

## Drawbacks, alternatives, and unknowns

### Drawbacks

- Driver development on Windows. The Fuchsia IDK does not support Windows so
  rules that depend on it will not work on Windows systems.

### Alternatives

- Expand the GN C++ SDK to support the above workflows.
    - Pros
        - Builds on existing work, incremental.
        - Some rules may be migrated directly from fuchsia.git with minimal
          changes.
        - Most Fuchsia developers have some experience with GN.
    - Cons
        - There is not a simple version scheme for releases of GN - projects
          are tied to specific GN releases, and there's [no guarantee of
          forwards or backwards compatibility][14]. This is important because
          these rules are expected to be used in conjunction with rules
          distributed by other parties.
        - GN is not designed for build rules to work across multiple projects.
          - Build rules are generally written to support being built in a
            specific project - usually Chromium.
          - It is common for GN rules to define/reference a
            `build_with_chromium` variable to selectively support being built
            as part of Chromium, or use dependencies whose rules were written
            for use in the Chromium project.
        - The above reasons fall below Fuchsia's intended level of API
          stability.
- Use another build system such as CMake, Buck, Meson, etc.
    - Pros
        - Another system may be more popular with users, and therefore more
          familiar to work with.
        - Bazel is a pretty large dependency, and runs a Java daemon. Some
          alternatives are much leaner.
    - Cons
        - None are supported directly by Google
        - Most do not have an as large multi-language rule ecosystem as Bazel.
        - Most do not support content-addressed storage for distributed builds.

## Prior art and references

- [Non-public use Bazel SDK][11]
- [GN C++ SDK][3]
- [Unofficial Dart rules][12]

[1]: /docs/contribute/governance/rfcs/0095_build_and_assemble_workstation_out_of_tree.md
[2]: /docs/development/idk
[3]: /docs/development/idk/gn
[4]: /docs/development/idk#strategy
[5]: https://docs.bazel.build/versions/main/skylark/repository_rules.html
[6]: https://docs.bazel.build/versions/main/skylark/rules.html
[7]: https://docs.bazel.build/versions/main/skylark/lib/Provider.html
[8]: /docs/concepts/testing/sl4f
[9]: https://docs.bazel.build/versions/main/skylark/deploying.html
[10]: https://github.com/bazelbuild/stardoc
[11]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/scripts/sdk/bazel
[12]: https://github.com/cbracken/rules_dart
[13]: https://docs.bazel.build/versions/main/skylark/testing.html
[14]: https://gn.googlesource.com/gn/#versioning-and-distribution
[15]: https://fuchsia-review.googlesource.com/c/fuchsia/+/559197
[16]: /docs/contribute/governance/rfcs/0095_build_and_assemble_workstation_out_of_tree.md#phase_3
[17]: https://docs.bazel.build/versions/main/toolchains.html
[Distributed Product Integration]: /docs/contribute/governance/rfcs/0124_decentralized_product_integration_artifact_description_and_propagation.md
[Bazel's CI]: https://github.com/bazelbuild/continuous-integration/blob/master/buildkite/README.md
[Bazel's style for distribution]: https://docs.bazel.build/versions/main/skylark/deploying.html
[Bazel offline builds]: https://docs.bazel.build/versions/main/external.html#offline-builds
[bazelisk]: https://github.com/bazelbuild/bazelisk
[rules_go]: https://github.com/bazelbuild/rules_go
[Workstation.git]: https://fuchsia.googlesource.com/workstation/+/refs/heads/main/
[Bazel toolchain]: https://docs.bazel.build/versions/main/toolchains.html
[OOT System Testing]: /docs/contribute/roadmap/2021/oot_system_testing.md
