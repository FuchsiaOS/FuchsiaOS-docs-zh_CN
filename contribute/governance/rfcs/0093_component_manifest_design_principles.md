{% set rfcid = "RFC-0093" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }} - {{ rfc.title }}
<!-- *** DO NOT EDIT ABOVE THIS LINE -->

## Summary

This document captures design deliberations, principles, and decisions made
regarding the `.cml` and `.cm` formats used for
[component manifests][component-manifest].

Most of the decisions described below were made in the years 2018-2019.

1. [Component manifests have a frontend and backend](#frontend-backend)
1. [A component is defined by a manifest](#defined-by-manifest)
1. [Component manifests are declarative](#declarative)
1. [Component manifests can come from various sources](#manifest-sources)

## Glossary

-   `.cml` is the common filename extension for
    [component manifest][component-manifest] source files. CML files are a JSON5
    [DSL] for declaring a component.
-   [ComponentDecl] is a FIDL table which is the canonical wire and storage
    format for component manifests.
-   `.cm` is the common filename extension for files that contain a
    [ComponentDecl] in the [FIDL envelope][fidl-envelope] binary format.
-   [`cmc`][cmc] is a command line host tool for generating `.cm` files from
    `.cml` files. It is built in the Fuchsia source tree and distributed as a
    prebuilt executable through the [Fuchsia SDK][sdk].

## #1: Component manifests have a frontend and backend {#frontend-backend}

Humans and machines have different needs and preferences. In designing the
component manifest syntax and formats, a key design principle is to create a
single backend format for the Component Framework to read, and a separate
frontend (singular at first) for developers to write. This design decision has
the following benefits:

1.  It is possible to optimize the backend and frontend(s) separately to satisfy
    different design goals.
1.  It is possible to evolve the frontend without modifying the backend, and
    vice versa.
1.  Although currently there exists only one frontend, it is possible to
    introduce additional frontends, such as to satisfy different design goals
    for different audiences or to cater to preferences of convenience,
    familiarity, or style.

To support these goals, the SDK provides `[cmc]` ("component manifest
compiler"), the standard tool to convert component manifest source files
(`.cml`) to manifest binary files (`.cm`). `cmc` is usually integrated
transparently with the build system, which means that developers normally
interact with source files unless they are debugging or performing analysis on
the manifest.

### ComponentDecl: the component manifest backend

[ComponentDecl] is the canonical storage and wire format for component
manifests. It is designed to be written and interpreted by code such as
component manager, [component resolvers][component-resolvers] and manifest
analysis tools such as [`fx scrutiny`][fx-scrutiny]. Such a format needs to meet
the following goals:

1.  It must be unambiguous and self-describing. It should be possible to
    directly derive the meaning of a manifest from its contents.
1.  It must be able to evolve over time, supporting forward and backward
    compatibility.
1.  It must be straightforward to parse and avoid gratuitous format conversions.
    Otherwise it would needlessly increase the risk for bugs or attacks in code
    that handles component manifests, including component manager.
1.  It must be easy to integrate with the runtime, components and tools that
    interface with component manifests.

These design goals led to a natural choice of format: [FIDL]. FIDL is the
standard wire format for IPC in Fuchsia. FIDL value types (i.e. [types with
no handles][rfc-0057]) can be persisted and used as a storage format.
Specifically [FIDL envelopes][fidl-envelope] are used because they have the
added benefit that they skip over unknown fields, which is useful for backward
and forward compatibility. Since FIDL bindings are already present for any
runtime that runs on Fuchsia, it is easy to integrate with code and requires no
additional support for parsing or conversion.

ComponentDecl is structured in such a way that there are no defaults: in other
words, a field is only unpopulated if it is not applicable or it came from a
version of the manifest that did not have that field. Also, to support forwards
and backwards compatibility, ComponentDecl and the structures nested in it are
[FIDL tables][fidl-tables] or [FIDL flexible unions][fidl-unions].

### CML: the component manifest frontend

[CML][component-manifest] ("component manifest language") is the source format
for component manifests. It is designed to be read and written by humans, but
may also be read and written by development tools such as formatters and
language servers.

1.  It should be readable by a novice who understands basic Component Framework
    concepts.
1.  It should be convenient to represent common patterns without excessive
    boilerplate.
1.  Changes that affect the syntax but not the semantic meaning of the manifest
    should be possible without requiring any changes to the manifest's binary
    representation. For example, a change from a singleton array to a singular
    value should not affect the output.
1.  It should promote maintainability. In particular, it should allow comments.
1.  It should be machine-friendly enough to support automated transformations,
    for example to support large scale refactors.

CML was invented to meet these goals. CML is a [JSON5]-based configuration
language which acts as a simple [DSL] which generates ComponentDecl. By using
JSON5, CML leverages a language that is already familiar to many developers and
is widely used elsewhere in Fuchsia. Unlike ComponentDecl, CML provides some
affordances that make it possible to write manifests more succinctly:

-   It allows defaults to be elided for some fields.
-   It allows multiple capabilities to be grouped into a single declaration as
    long as they share the same options.
-   It allows manifests to [include manifest shards][component-manifest-include]
    that contribute contents to the manifest. For example, you can depend on a
    library and include the library's shard to get all the capabilities required
    by that library.

Finally, the translation from CML to ComponentDecl, while not one-to-one, should
be straightforward for users to understand without having to learn the rules.

## #2: A component is defined by a manifest {#defined-by-manifest}

A component is described by a manifest. The manifest is resolved at start time
via the URL of the component. For example, a component launched via a
[`fuchsia-pkg://`][package-url] URL with will have a manifest with the `.cm`
extension, containing a serialized `ComponentDecl`, that is [resolved][resolver]
from a [package]. In addition to the manifest, a component may also incorporate
resources from the same package. For example, a component that uses the [ELF
runner][elf-runner] specifies the location of the [ELF][elf] binary in that
package. On the other hand, a component with an `https://` URL may have a
`ComponentDecl` that is generated by the `https` resolver but is not backed by a
resource obtainable via a URL.

A component's manifest fully describes its inputs, outputs, and internal
composition. Currently, component manifests cannot have any parameters or
"dangling" values that are filled in at runtime.[^1]

That isn't to say a component's behavior is completely described by the
manifest, however. For one, the capabilities offered to a component are
determined by the parent; the component has no control over who provides those
capabilities. Also, every component is part of an environment which provides
certain types of configuration for the component, such as what resolvers are
used to resolve component URLs.

Following the URLs between manifests yields a
[component instance tree][topology]. The component instance tree is a
comprehensive description of the software that constitutes a Fuchsia image. This
makes it possible to perform security auditing with confidence over a given
system image, such as with `[fx
scrutiny](https://fuchsia.dev/reference/tools/fx/cmd/scrutiny)`.

## #3: Component manifests are declarative {#declarative}

While configuration languages with imperative features are powerful, they come
at cost of lost readability, predictability, and auditability. Precedent shows
that configuration languages with too much imperative flavor are brittle and
less user-friendly.[^2] In Component Framework's case, component definitions
must be auditable and readily understandable, which makes an imperative-style
configuration language a non-option.

Thus, CML is a _declarative_ language. To the extent that CML supports
generating parts of the manifest, this is only supported in cases that have very
predictable outcomes. For example, manifests support default values and
inclusion, but they do not provide templatizing or parameterization features.

CML is a language designed to be read and written by _humans_. With the
exception of developer tooling integration (for example, formatting tools or IDE
templates), CML is not intended to be generated by tools. Generating CML files
carries an elevated risk of obscuring the underlying contents of the manifest,
since there are now three layers involved: CM, CML, and the tool. If for some
reason a manifest has to be generated, you should write a separate frontend to
generate CM.

## #4: Component manifests can come from various sources {#manifest-sources}

Component manifests, in general, are not bound to any single distribution
mechanism. It is ultimately the responsibility of the
[component resolver][component-resolvers] to retrieve the component manifest for
a URL. How a resolver accomplishes this is particular to a given URL scheme. For
example, a `fuchsia-pkg://` resolver will retrieve the package and read the
manifest from it designated by the fragment identifier part of the URL. A web
resolver might generate a manifest whose contents may vary based on domain,
security policy, and user preferences.

Currently, the most common way to distribute a component is through a Fuchsia
package. Such components are identified by a `fuchsia-pkg://` URL. The
component's manifest is shipped as a blob in this package, usually in `meta/`.

## Inspiration

-   [Declarative application management in Kubernetes][k8s-design]: principles
    used in designing the configuration language for Kubernetes, and a study of
    alternatives.
-   [Imperative vs declarative][imperative-declarative]: expands on the titular
    topic.
-   [Starlark]: a Python-based DSL and an imperative configuration language.
-   [Jsonnet]: an extension of JSON into a data templating language, where any
    program produces a JSON document.
-   [borgcfg], [GCL], and [borgmon]: Functional configuration languages used by
    Google, roughly analogous to Kubernetes, whose history helped inform us on
    the tradeoffs between imperative and declarative syntax.

## Notes

[^1]:
     In the future, there is a high probability that manifests need to
     support some sort of parameterization feature to support variants and
     product configurability. When we do so, we should approach this in a way
     that avoids the common pitfalls associated with parameterizable
     configurations.

[^2]:
     For more context on this point, Kubernetes has extensive
     [documentation][k8s-declarative-configuration] explaining many of the
     downsides of non-declarative configuration.

[blobfs]: concepts/filesystems/blobfs.md
[borgcfg]: https://storage.googleapis.com/pub-tools-public-publication-data/pdf/43438.pdf
[borgmon]: https://cloud.google.com/blog/products/devops-sre/welcome-to-the-museum-of-modern-borgmon-art
[cmc]: /tools/cmc
[component-manifest]: concepts/components/v2/component_manifests.md
[component-manifest-include]: concepts/components/v2/component_manifests.md#include
[component-resolvers]: concepts/components/v2/capabilities/resolvers.md
[ComponentDecl]: /sdk/fidl/fuchsia.component.decl/component.fidl
[DSL]: https://en.wikipedia.org/wiki/Domain-specific_language
[elf]: concepts/process/program_loading.md#elf_and_the_system_abi
[elf-runner]: concepts/components/v2/elf_runner.md
[FIDL]: development/languages/fidl/README.md
[fidl-envelope]: reference/fidl/language/wire-format/README.md#envelopes
[fidl-tables]: reference/fidl/language/wire-format/README.md#tables
[fidl-unions]: reference/fidl/language/wire-format/README.md#unions
[fx-scrutiny]: https://fuchsia.dev/reference/tools/fx/cmd/scrutiny
[imperative-declarative]: https://dominik-tornow.medium.com/imperative-vs-declarative-8abc7dcae82e
[JSON5]: https://www.json5.org
[Jsonnet]: https://www.jsonnet.org
[k8s-design]: https://github.com/kubernetes/community/blob/master/contributors/design-proposals/architecture/declarative-application-management.md
[k8s-declarative-configuration]: https://github.com/kubernetes/community/blob/master/contributors/design-proposals/architecture/declarative-application-management.md#declarative-configuration
[GCL]: https://storage.googleapis.com/pub-tools-public-publication-data/pdf/43438.pdf
[package]: concepts/packages/package.md
[package-url]: concepts/packages/package_url.md
[resolver]: https://fuchsia.dev/reference/fidl/fuchsia.component.resolution#Resolver
[rfc-0057]: contribute/governance/rfcs/0057_default_no_handles.md
[sdk]: https://fuchsia.dev/reference/tools/sdk/README.md
[Starlark]: https://github.com/bazelbuild/starlark
[topology]: concepts/components/v2/topology.md
