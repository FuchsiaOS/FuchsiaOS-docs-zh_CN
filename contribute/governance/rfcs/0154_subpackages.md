<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0154" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This is a proposal to add support in Fuchsia for defining and resolving
subpackages. A subpackage is a named reference from one package ("A") to another
package ("B"), designating a one-way dependency (from "A" to "B"). Subpackages
can also contain subpackages, creating a directed acyclic graph (DAG) of
related, version-pinned packages from a top-level package. With subpackages, a
resource in package "A" can refer to package "B" by its subpackage name, which
is defined by the containing package. If "B" is a subpackage of "A", then any
system that makes "A" available (for example, a Fuchsia archive, a blobstore)
must also make "B" available. Subpackages, therefore, enable two important
system properties:

  * Hermeticity - Packaging a program (or test) with specific package
    dependencies.
  * Atomicity - The system should guarantee that if a package with subpackages
    is successfully resolved, all of its subpackages will be successfully
    resolved. (The approach proposed in this RFC will guarantee this by eagerly
    resolving the entire hierarchy of subpackages before returning a successful
    result from a package resolution request.)
  * Relocatability - Packages that can be distributed, with all required
    resources, into a single archive (such as for downloading) or to alternative
    Fuchsia package servers.

## Motivation

This RFC proposes a way to express and support the concept of nested
inter-package dependencies (subpackages). Whereas packages are currently not
allowed to depend on other packages, this RFC amends the restriction to be that
packages may not depend on other packages except through containment.

### Why now?

The most prevalent current use cases driving the need for inter-package
dependencies are in test scenarios. Tests need to execute a specific component
(often a fake or mock) in a hermetic environment alongside their component under
test. Depending on and including an exact version of dependencies is important
to prevent global system state (i.e. the set of packages that happen to be
installed) from affecting the correctness of tests. We call this property
"hermetic packaging."

We hermetically package tests in-tree by including all component dependencies in
a single package. Out-of-tree (OOT), however, the status quo is to depend
on the absolute package URL. These tests are not hermetically packaged, which
has the effect of making platform package URLs an implicit ABI.

For example, today a Flutter integration test declares a dependency on Fuchsia's
Scenic component by its component URL,
`fuchsia-pkg://fuchsia.com/scenic#meta/scenic.cm`. This declares nothing more
than an expectation that the Fuchsia _platform_ will make _some_ Scenic instance
available via that URL reference. If Scenic, or one of its dependencies (which
the test developer must also discover and declare), changes its interface,
behavior, or is removed, the test can fail at runtime. This is already a common
source of test breakages, and the situation will only get worse as we scale.

By contrast, with subpackages, the Flutter integration test author could:

  1. Obtain a package containing a specific version of Scenic at build time.
     (The mechanism for this is out of scope.)
  2. Include that package as a subpackage of their test package.
  3. Declare a child component with URL `scenic#meta/scenic.cm` in their test's
     component manifest, rather than
     `fuchsia-pkg://fuchsia.com/scenic#meta/scenic.cm`.

This version of the test is more hermetic and will always behave the same way,
regardless of which version of Scenic happens to be installed on the device on
which it runs.

NOTE: Subpackages enable OOT testing scenarios like this, but the process for
distribution of Fuchsia platform packages for reuse in tests is outside the
scope of the RFC. Once the RFC is approved, however, a follow-on effort to
define this process, as an extension of the Fuchsia SDK, seems likely.

We are motivated to provide a consistent experience between in-tree and OOT
components: a single test package explicitly stating all of its hermetic
dependencies, which are made available to a test as an atomic group.

Rather than reapplying the existing strategy from in-tree (which requires
constructing a new package that contains multiple components), we instead
propose supporting inter-package dependencies in the form of subpackage nesting.
This allows us to independently package and depend on components without
complicated namespacing considerations.

## Stakeholders

_Facilitator:_ hjfreyer@google.com

_Reviewers:_

| Name | Focus Area |
| ---- | ---------- |
| wittrock@google.com | SWD |
| jsankey@google.com | SWD |
| geb@google.com | Component Framework |
| shayba@google.com | Build |
| etryzelaar@google.com | Tooling |
| aaronwood@google.com | Assembly/Package Movement |
| cgonyeo@google.com | RealmBuilder |
| ampearce@google.com | Security |

_Consulted:_ computerdruid@google.com

_Socialization:_

The SWD team was involved in the initial draft of this RFC, prior to publishing.
The Component Framework team was kept informed of the progress of the effort,
and provided feedback on scope and features during weekly meetings, with members
of the CF team among the RFC's authors.

## Design

### Use cases impacted by subpackages

  * In-package declaration of nested package dependencies
    * Packages MAY contain a metadata file listing their subpackages, and build
      systems MUST have the ability to populate this metadata file based on the
      dependencies of a package target.
  * Relative component resolution
    * For example, a developer could replace references to a component
      `fuchsia-pkg://servername/some-package#meta/child_component.cm` with the
      relative reference `child_package#meta/child_component.cm`; where
      `child_package` is the referrer's (or "parent package's") named reference
      to `some-package`, version-locked (by package hash) at build time.
  * Package dependency tree traversal
    * For example, by publishing a single top-level package, a package server
      could use the embedded package references to automatically locate and load
      package dependencies.
  * Package tree archiving
    * To construct a single file, from a given top-level package, that contains
      the package and all dependencies.

### Subpackage Representation

A package declares references to subpackages by a package-scoped name for the
subpackage, mapped to a
[package hash](/docs/get-started/learn/intro/packages.md) of a package defined
in the same package store and same package set. (For example, for a parent
resolved from the "universe" package set, its subpackages MUST also be resolved
from the "universe" package set). The rest of the RFC describes a notional
representation of these declarations as a file called `meta/subpackages`
matching the format currently used by `meta/contents` (<name>=<hash>).

If present, such a file MUST include at least the name of the subpackage and its
package hash (the "content address" of the `meta.far` of the referenced
subpackage).

Subpackage names MUST be unique within a single `meta/subpackages` file, but
they do not need to be unique across packages. A package therefore fully
controls the naming of its subpackages, and multiple packages may contain the
same subpackage under different local names.

Subpackage references are considered "private data" to the referencing package.
For example, a parent package, `A`, may include two subpackages: packages `B`
and `C` (referenced by their respective package hashes). Package `B` might also
include a subpackage reference to the same package hash `C`. Neither the
top-level parent `A` nor its subpackage `B` would have knowledge of the common
dependency. (In other words, a parent package can only resolve it's subpackages,
one level down. A parent cannot--directly--resolve a package URL to a subpackage
of a subpackage. In fact, this RFC does not define a syntax for representing a
subpackage of a subpackage.)

NOTE: An individual subpackage can also be independently published and served as
a top-level package. (For example, a production component can be published as a
top-level component identified by `fuchsia-pkg` URL, but that component's
package could also be included as a subpackage in one or more hermetic tests.)
When resolving a top-level package, by Fuchsia Package URI, the default behavior
is to retrieve the most recently published version of that package. Subpackages,
on the other hand, always refer to specific versions, by "package hash".

### Building a package with subpackages

Adding a subpackage to a package follows a similar pattern to adding a regular
file, albeit with some special considerations.

The various Fuchsia SDK build systems (today, GN and Bazel) generally support
creating a "package" target, which can take dependencies on other targets for
inclusion in the package. In those build systems, adding a dependency on a
package informs the build system to also generate the dependent package, but
currently does not encode that dependency in the generated package. For example,
the following GN code will only cause package "B" to be generated when generating
package "A".

```gn
# GN format
fuchsia_package("A") {
  deps = [
    ":B",
    other_deps,
    ...
  ]
}

fuchsia_package("B") {
  ...
}
```

A cursory investigation of existing inter-package dependencies in fuchsia.git
revealed that, in most cases where a target package depends on another package,
a component in the target package expects to load the dependent package. In
these cases, the dependent packages could be bundled with the target, as
subpackages. But since existing GN rules don't enforce this interpretation, we
cannot infer it.

Therefore, this RFC recommends an explicit variable---`subpackages`---be used to
declare subpackage targets. Each target in the `subpackages` list MUST result in
a corresponding entry in the generated `fuchsia_package`'s `meta/subpackages`
file.

The targets in this list also infer a build dependency, so targets in
`subpackages` do not also need to appear in `deps`.  To convert a package with
declared package dependencies to a package with contained subpackages,
move all (or selected) package targets from `deps` to `subpackages`. Notionally,
the change to `fuchsia_package` could appear as:

```gn
# GN format
fuchsia_package("A") {
  subpackages = [ ":B" ]

  deps = [
    other_deps,
    ...
  ]
}

fuchsia_package("B") {
  ...
}
```

Build systems SHOULD default the subpackage name to that of the included target,
but they MAY support overriding this name, using common idioms.

To facilitate building packages with subpackages using existing tools, build
rules and scripts MUST be updated in the Fuchsia in-tree build system, and
SHOULD be updated in the Fuchsia GN SDK and downstream build environments.
(Affected build environments and required changes, to the extent known, are
described in the [Implementation](#implementation) section.)

### Resolving a subpackage

Subpackage resolution resolves a relative reference to a named subpackage of a
known package.

The `Resolve` method of the `fuchsia.pkg.PackageResolver` protocol will be
modified to return a `ResolutionContext`. `Resolve` will only resolve absolute
package URLs. An additional method, `ResolveWithContext`, will be added to take
an additional `context` argument (a `ResolutionContext`), and also return a new
`ResolutionContext` for the resolved package. `ResolveWithContext` will resolve
either an absolute package URL (ignoring the `context`) or a relative package
URL. These changes are represented by the following notional FIDL snippet:

```fidl
   library fuchsia.pkg;

   ...

   const MAX_RESOLUTION_CONTEXT_LENGTH = <TBD>;
   type ResolutionContext = bytes:MAX_RESOLUTION_CONTEXT_LENGTH;

   protocol PackageResolver {

     /// Resolves an absolute component URL.
     /// ...
     Resolve(resource struct {
         package_url string;
         dir server_end:fuchsia.io.Directory;
     }) -> (struct {
         resolved_context ResolutionContext;
     }) error ResolveError;

     /// Resolves a component URL, which may be absolute or relative. If
     /// relative, the component will be resolved relative to the supplied
     /// `context`.
     ResolveWithContext(resource struct {
         package_url string;
         context ResolutionContext;
         dir server_end:fuchsia.io.Directory;
     }) -> (struct {
         resolved_context ResolutionContext;
     }) error ResolveError;

     ...
   }
```

NOTE: The choice of a byte array for context was the result of several
constraints, some FIDL limitations, and carefully considered opionions. For
additional background, see the "Alternatives" sections:
[Use context-specific `Resolver`s](#alternative-use-context-specific-componentresolvers)
and
[Alternative: FIDL Type representation for resolution context values](#alternative-fidl-type-representation-for-resolution-context-values).

For a package at `package_url` that declares subpackages (a "parent" package),
the returned `resolved_context` MUST be passed as the `context` input parameter
to a follow-up call to `ResolveWithContext` when resolving subpackages. (If the
caller does not resolve subpackages of the resolved package, the caller MAY
ignore the returned `resolved_context`.)

The Subpackages implementation MUST NOT reduce the robustness and survivability
of existing services and components. In other words, Subpackages must not cause
a non-critical (restartable) component to need to be marked critical, and
stateless protocols (such as `fuchsia.pkg.Resolve()`, but also the proposed
`ResolveWithContext`, which will replace some calls to `Resolve`) MUST remain
stateless.

The implementation of the `context` value may be impacted by this constraint.
`ResolveWithContext` must accept any `context` returned by `Resolve` or
`ResolveWithContext` as long as the parent package is in active use in the
system. (Conceptually, the package hash of the parent package is sufficient to
statelessly resolve a subpackage from the same Merkle-hash-indexed package
store.)

NOTE: Determining whether the parent is still "in active use" is a concern for
package garbage collection that will need to be addressed during implementation
of Subpackages in the package resolver.

For absolute `package_url`s, the `context` argument to `ResolveWithContext` is
ignored.

NOTE: Since this RFC restricts subpackage resolution to the declared subpackages
of a parent (one level down), a relative path MUST NOT contain a slash (`/`).

### Loading a Component from a Subpackage

Component Resolvers are responsible for parsing Component URLs, converting them
to a Package URL that is then Resolved, and then loading a component manifest
from the resolved package. The loaded manifest and package contents are used to
create a new component, including a context for resolving subpackages by
relative path.

The `Resolve` method of the `fuchsia.component.resolution.Resolver` protocol
will only resolve absolute component URLs. An additional method,
`ResolveWithContext`, will be added, taking an additional `context` argument (a
`fuchsia.component.resolution.Context`). `ResolveWithContext` will resolve
either an absolute component URL (ignoring the `context`) or a relative
component URL. These changes are represented by the following notional FIDL
snippets:

```fidl
   library fuchsia.component.resolution;

   ...

   // Note the context length, in bytes, must be at least the size of
   // fuchsia.pkg.MAX_RESOLUTION_CONTEXT_LENGTH, plus the size required to
   // accommodate additional component context information, if any.

   /// The maximum number of bytes for a `Context`.
   const MAX_RESOLUTION_CONTEXT_LENGTH uint32 = 8192;

   /// A byte array that persists a value used by the target `Resolver` to locate
   /// and resolve a component by relative URL (for example, by a subpackage
   /// name).
   alias Context = bytes:MAX_RESOLUTION_CONTEXT_LENGTH;

   protocol Resolver {

     /// Resolves a component with the given absolute URL.
     /// ...
     Resolve(struct {
         component_url string;
     }) -> (resource struct {
         component Component;
     }) error ResolverError;

     /// Resolves a component with the absolute or relative URL. If relative, the
     /// component will be resolved relative to the supplied `context`.
     ///
     /// `component_url` is the unescaped URL of the component to resolve, the
     /// format of which can be either:
     ///
     ///   * a fully-qualified absolute component URL
     ///   * a subpackaged-component reference, prefixed by a URI relative
     ///     path to its containing subpackage (for example,
     ///     `child_package#meta/some_component.cm`); or
     ///   * a URI fragment to a component in the current package (for
     ///     example,`#meta/other_component.cm`)
     ///
     /// `context` is the `resolution_context` of a previously-resolved
     /// `Component`, providing the context for resoving a relative URL.
     ResolveWithContext(struct {
         component_url string;
         context Context;
     }) -> (resource struct {
         component Component;
     }) error ResolverError;
   }
```

The returned `Component` type will be modified to include a
`resolution_context` to be used when resolving other components relative to this
`Component`.

```fidl
   type Component = resource table {

       ...

       /// The context used to resolve `component_url`s relative to this
       /// component.
       5: resolution_context Context;
   };
```

For example, after resolving the component called `parent`, a subpackaged
component can be resolved by the subsequent call to
`ResolveWithContext(subpackaged_url, parent.resolution_context)`.

For absolute `component_url`s, the `context` argument to `ResolveWithContext` is
ignored.

For relative `component_url`s:

  * The client must use `ResolveWithContext`. Calling `Resolve` with a relative
    component URL will return the error code `ResolveError::INVALID_ARGS`.

For relative, subpackaged component URLs:

  * The `component_url` begins with a relative path (a subpackage name, followed
    by the `#`-based fragment for the specific component, such as
    `child_package#meta/some_component.cm`).
  * If the package resolver returns `PACKAGE_NOT_FOUND` (or some equivalent
    package store-dependent error), the component resolver MUST return
    `ResolveError:PACKAGE_NOT_FOUND`.

For relative resource component URLs (from the same package as another
component):

  * The `component_url` is a URI fragment (for example,
    `#meta/other_component.cm`, as documented in [RFC-0104: Relative Component
    URLs](/docs/contribute/governance/rfcs/0104_relative_urls.md):
  * The fragment MUST refer to a component in the same package as another
    component (a "peer" component) that was previously resolved.
  * The `context` value MUST refer to the same package version (same package
    hash) as its peer.

NOTE: Using a `context` to resolve relative resource component URLs (by URI
fragment)
alters the current behavior of the Relative Resolver. Currently the Relative
Resolver constructs an absolute package URL from its parent component, and
appends the relative fragment part. This behavior does not guarantee that the
parent component and child component are retrieved from the same package
version. With subpackages, it is not always possible to construct an absolute
package url from a parent or peer component. On the other hand, pinning the
package to a specific package hash used to resolve all components bundled in
a package is probably a desired behavior; in which case, using a context is
an improvement.

When resolving a component URL using a relative URI (path or fragment) on behalf
of an existing component (the "parent component"), Component Manager MUST
delegate the resolution to the same `Resolver` that was used to resolve the
parent component.

The context value should be considered an implementation detail internal to the
Component Resolver, and opaque to clients. The `Resolver` MAY
forward the `resolved_context` value (as returned from
`PackageResolver::Resolve...()`) as the component resolution `context` value.
(The API does not prevent the `Resolver` from returning a different or
augmented context value, but this may not be necessary.)

A notional example of the process for resolving a component with a subpackaged
child (assuming a post-boot environment) follows:

  1. To load Component `A` from a top-level package `P`:

    a. Component Manager gets the registered `Resolver` capability for
       the `fuchsia-pkg` scheme, and calls
       `Resolver::Resolve("fuchsia-pkg://fuchsia.com/package-p#meta/component-a.cm")`
       .

    b. The `Resolver` extracts the package URL and calls
       `PackageResolver::Resolve("fuchsia-pkg://fuchsia.com/package-p", ...)`,
       returning the `fuchsia.io.Directory` from which to load the component,
       and the `resolved_context` value.

    c. `Resolver::Resolve()` constructs and returns the resolved
       `Component`, with `resolution_context`, which Component Manager caches in
       that component's state.

  2. To load a child component `B` from a subpackage `child-package`, relative
     to component `A` above:

    a. Component Manager gets the `Resolver` capability used to resolve
       `component-a`, and calls
       `Resolver::ResolveWithContext("child-package#meta/component-b.cm", component_a.resolution_context)`.

    b. The `Resolver` extracts the relative package path (the
       subpackage name) from the component URL, and extracts the
       `package_context` from the component `context` input parameter, and calls
       `PackageResolver::ResolveWithContext("child-package", package_context)`,
       returning the `fuchsia.io.Directory` from which to load the component,
       and the subpackage's own `resolved_context` value.

    c. `Resolver::ResolveWithContext()` constructs and returns the
       resolved `Component`, with the new component's `resolution_context`,
       which Component Manager caches in that component's state.

### Future work

* **Consolidating `subpackages` and `contents` files** - If and when the
  proposed RFC for [Persistent FIDL package
  contents](https://fuchsia-review.googlesource.com/c/fuchsia/+/586937) is
  approved, the subpackages file SHOULD be updated to either match the new
  format that replaces `contents`, or the files could be merged, using expanded
  fields to differentiate between content entries and subpackage entries.
* **Annotating `subpackages` file entries with summary statistics** - The
  "Persistent FIDL package contents" RFC would provide a mechanism for including
  additional data with each subpackage-to-package-hash mapping. It may be
  helpful to include summary statistics, such as content size (including the
  rolled up sizes of each subpackage and its nested dependencies). This could be
  used, for example, for progress reporting during package fetch operations,
  when loading a package with a significantly large set of dependencies.
* **Improving runtime dependency resolution** - A concept also under
  consideration (for a future RFC) is to support relative package URLs with an
  absolute path (that is, prefixed by a slash "/"), to request a top-level
  package from the "same package server" that served the "current" package (or
  current component). (The current package server is knowable in the same way
  that resolving a subpackage URL from the current package is knowable, as
  described in this RFC.)
* **Generalized FIDL service for resolving and loading asset package content** -
  Today the only way for Fuchsia software to load content from a package is by
  including a component in that package. The custom component would be
  responsible for routing a directory or serving a FIDL protocol to provision
  assets from the package. A future proposal is being considered to include a
  more accessible FIDL API for loading a subpackage and/or assets within that
  package. This could be used, for example, to distribute visual assets or
  selectable user interface "themes" as resource bundles, without requiring the
  additional step of implementing a component intermediary.
* **Lazy Loading of Subpackages** - This RFC proposes that a package and all of
  its subpackages (recursively) will be loaded when the top-level package is
  loaded (that is, eagerly). A proposed future extension would allow a given
  subpackage to be declared "lazily loadable". This could be used to avoid the
  cost of importing a package from a remote server to a storage-constrained
  device until a given package is specifically requested. (See
  [Eager package loading](#eager-package-loading) for more details.)
* **Hot Reload for Subpackaged Components** - This RFC proposes a strict
  implementation of inter-package versioning: A package hash is computed from
  the hashes of its contents, _including_ its subpackage hashes. Therefore, if a
  subpackage changes (indicated by a change in the subpackage's package hash),
  the parent package has also changed. It may be desirable to reload a
  subpackage without reloading the parent (such as to "hot-reload" a component
  at development time), or vice-versa. Allowing this kind of behavior could
  reduce some of the intended benefits of subpackages, including reliability and
  security. Additionally, there are known gaps in how Component Manager resolves
  and reloads a new version of a component (see http://fxbug.dev/66381). If
  these known issues are resolved, subpackage dependency constraints could be
  revised (in a future proposal) to allow a parent package to declare certain
  dependencies based on a looser contract (behavior, API, and/or ABI guarantees,
  for example).

## Implementation

### Fuchsia in-tree build system (GN rules and scripts)

Build rules, scripts, and file formats (including intermediate file formats for
Fuchsia package generation) MUST be updated to forward the list of `subpackages`
through each phase of package build and/or archive, and ultimately to generate
the new `meta/subpackages` file alongside the package `contents` file.

For each package target in `subpackages`, the `meta/subpackages` file MUST map a
declared subpackage name (defaulting to the subpackage target name) to the
package hash of the subpackage target (that is, the blob hash of the
subpackage's `meta.far`.)

Some of the file formats that MAY require either a format change (to add
subpackage references) or a supplementary file for the same stage (like how
`contents` would be paired with `subpackages`) include:

  * The package "build manifest" (or `archive_manifest`), which typically ends
    in the `.manifest` extension
  * The `package_manifest.json`

### Fuchsia out-of-tree build environments (GN, Bazel, and supporting scripts)

Build rules and scripts SHOULD be updated, in a similar fashion to the Fuchsia
in-tree build rules and scripts, to enable subpackages in out-of-tree
repositories. Impacted repositories include chromium and flutter/engine. (Note
that flutter/engine currently implements a modified copy of GN SDK build rules
and scripts, so similar---if not identical---changes will need to be made in
both the GN SDK repository and in flutter/engine.)

  * `package.gni` invokes `prepare_package_inputs.py` with `--manifest-path`, to
    generate `${package_name}.manifest` (referred to as the `archive_manifest`
    in the script, or the "build manifest" in `pm` CLI documentation)
  * `pm_tool.gni` invokes `pm build` with `-m ${archive_manifest}` and
    `-output-package-manifest` to generate `package_manifest.json`

### Package Manager package command line interfaces (CLIs)

Package Manager commands will be modified, by changing `ffx package`.

NOTE: The `pm` command is being deprecated, in favor of `ffx package`. Changes
to `pm` will only be made if workflows require them, and cannot be updated to
use the `ffx package` replacement commands.

  * `ffx package build` (was `pm build`)
    * This command will need to change to accept either additional arguments and
      files, or a revised file format for inputs and outputs, to include the
      additional subpackage declarations.
  * `ffx package export` (was `pm archive`)
    * This command reads the `contents` file and generates a `.far` file
      containing all of the referenced blobs.
    * The `export` behavior will be extended, for packages with a `subpackages`
      file, to bundle those subpackages (recursively) into the generated archive
      (see [Bundling package dependencies for
      distribution](#bundling-package-dependencies-for-distribution)
  * Other `ffx package` commands may also require changes to accommodate
    `subpackages` (such as `download` and `import`). The impact to these
    commands will be investigated at implementation time. These changes may
    affect or be affected by other planned changes, as part of
    [RFC-0124: Decentralized Product Integration: Artifact Description and Propagation](/docs/contribute/governance/rfcs/0124_decentralized_product_integration_artifact_description_and_propagation.md).

### Bundling package dependencies for distribution

One of the key use cases for subpackages is to support package distribution by
ensuring a package and all of its direct and indirect dependencies can be
relocated as a bundle. The bundle format is less important than developing a
procedure for recursively traversing subpackage dependencies.

The `ffx package` tool is the logical tool to implement a way of identifying and
locating each subpackage's content (expanded package directory or `.far`
archive).

Rather than placing the burden of implementing custom scripts for hermetic
packaging on  each out-of-tree user, this RFC recommends extending package
tooling to be subpackage-aware, to support distribution of packages with their
subpackage dependencies. For example, `ffx package` would be extended to bundle
a package with its dependencies, into a single-file archive.

The proposed format of a hermetic package archive, as a single file, would be
the expanded contents of all contributing packages, resulting in an indexed,
flat collection of all blobs across all packages.

### Package resolver

Implementations of the `fuchsia.pkg.PackageResolver` protocol MUST be updated
to implement the behavior described in the above design section.

#### Eager package loading

The package resolver MUST internally resolve all subpackages recursively, until
all subpackages have been fetched, before returning a resolution result for the
root package. This approach was chosen to simplify the implementation of the
atomicity property, so all required subpackages are guaranteed to have been
resolved before the component starts.

Enforcing eager package resolution may also support some of the requirements in
the approved [RFC-0145: Eager Package Updates](/docs/contribute/governance/rfcs/0145_eager_package_updates.md),
Notably, a package and its subpackage tree could be used to implement the Eager
Package Updates RFC's prerequisite for a "package group".

### Component resolver

Implementations of the `fuchsia.component.resolution.Resolver` protocol MUST be
updated to implement the behavior described in the above design section.

### RealmBuilder

RealmBuilder MUST be updated to include the ability to declare subpackages, and
to respond to component resolution requests that use relative paths in component
URLs. Hermetic tests using RealmBuilder currently do not have access to a
package resolver. Support for loading hermetically-bundled packages, via
subpackage declarations, MUST be implemented and made available to both hermetic
and system tests.

## Performance

Traversal of subpackages can increase latency of identifying all blobs to
download by following multiple levels of indirection, though this is unlikely to
be significant in practice. For example, compare loading a single package with N
blobs to loading a package with subpackages who together have N unique blobs:

  * Parent
    * N blobs

versus

  * Parent
    * N_0 blobs
    * Child1
      * N_1 blobs
      * Child2
        * N_2 blobs
        * ...

In the first case all N blobs are known up front and may be loaded in parallel.
In the second case the package resolver must serially follow links between
parent and child to accumulate the full set of N blobs to load. This traversal
incurs additional latency bounded by the depth of the tree. If the latency of
recursive blob loading becomes a problem, several implementation options could
be used to improve latency. For example, subpackage links could be traversed in
parallel with loading blobs or the complete set of subpackage content addresses
could be included when building a package.

## Backwards Compatibility

### Resolution behavior for relative resource component URLs

The behavior for resolving existing package URLs and component URLs is
unchanged, with one exception: relative resource component URLs (represented by
URI
fragments, such as `#meta/child.cm`) will require the component `context` (which
the proposed implementation will guarantee), but the behavior changes slightly.
The current Relative Resolver concatenates the fragment to the parent
component's package URL, then re-resolves the package and component. But the
resolver may be loading a new version of the package. This is thought to be a
potential risk area of the original implementation. The `context` will
guarantee that a relative component will be resolved from the same package from
which the "parent component" was resolved.

### Interpreting a resolved component's `fuchsia.component.resolution.Package`

After resolving a component, the returned
`fuchsia.component.resolution.Component` type includes a `package` field of type
`fuchsia.component.resolution.Package`, which includes a reference to the
`package_url` of the package that contained the returned `Component`.

```fidl
   type Package = resource table {
       /// The URL of the package itself.
       1: url string;

       /// The package's content directory.
       2: directory client_end:fuchsia.io.Directory;
   };
```

Since subpackages are always relative, any existing uses of `package_url` could
be affected. The subpackage resolution process, described in this RFC, does not
use any information stored in the `Package` type, so any change to `Package`, or
how that type is interpreted, does not have a material effect on the Subpackages
RFC design.

Alternatives to be considered at implementation time include:

  1. Storing a `url` that references only the package server and package
     hash (for example, `fuchsia-pkg://fuchsia.com?hash=123456789`), which is
     sufficient to re-resolve the contents of the subpackage, but does not
     retain any information about the parent component or its subpackage name.
  2. Storing the subpackage path in the `url`, and adding an optional
     resolution context field (the context that the component's package was
     resolved from) to the `Package`.

### FIDL method changes

In both `fuchsia.pkg.PackageResolver` and
`fuchsia.component.resolution.Resolver`, the `Resolve()` methods will both
change to return a new resolved context, and additional methods called
`ResolveWithContext()` will be added to support a `context` input parameter.
This may not require a soft transition.

### Changes to package representation

The addition of subpackages to Fuchsia will not change how a package without
subpackages is represented. Existing package URLs and component URLs are not
affected. Developers will have the option of replacing a fully-qualified
component URL (using `fuchsia-pkg://fuchsia.com/top-level-package#...`, for
instance) with a reference to one of its subpackages (using
`child-package#...`, mapping `child-package` to the package hash of
`top-level-package`).

### Changes to tools

Older versions of Fuchsia, package server, and some host-side tools (like `pm`
and `ffx package`) will not support packages published with subpackages, or
software using relative paths to packages. Fuchsia systems and host-side tools
will need to be updated and recompiled.

### Package name syntax

Subpackage names are scoped to the parent package, and are effectively aliases
for any top-level package name with the same package hash. This means the syntax
for subpackage names does not need to mirror the syntax for package names.

Nevertheless, it may be common to use the same name to refer to a package
independently and as a subpackage.

Since subpackages are hierarchical, it is natural to think of nested subpackages
as nameable, perhaps using slashes as delimiters. For example,
`fuchsia-pkg://fuchsia.com/parent/child/grandchild#gc-component.cm`, or
`child/grandchild#gc-component.cm`. Note that this RFC neither sanctions nor
forbids such representations, but the use of slash delimiters exposes a
potential pitfall.

Package names can currently contain a single slash. The content after them is
treated as a 'variant', for example, `/0`, which is currently common in Fuchsia.

Notably, the use of `/0` is deprecated, and if removed from common use, it may
be possible to free up the meaning of `/` for subpackages (though this is not
currently proposed, and would have to be subject to a future RFC).

If there are competing use cases for package naming hierarchies, an alternative
delimiter (such as `:`, for example) could be used for subpackage nesting, or
subpackages could use `/` and the alternative delimiter could be used for
another meaning.

This RFC proposes reserving at least `/` and `:` from permitted subpackage name
syntax.

## Security considerations

### Auditability and Executability

The proposed design has taken into account comments, to date, from Fuchsia
Security leads regarding the need to minimize changes to the security posture of
the system, and simplify review of changes that could impact auditability and
executability.

For example, a parent and all of its subpackages (recursively) must originate
from the same package store, and are considered part of the same package set
(for example, all from "base" or all from "universe"). Using the same package
set means all packages in a subpackage hierarchy are subject to the same
executability policy.

### System Allowlists based on package or component URL

Packages or components currently identified in privileged allowlists---due to
their sensitive capabilities---may be included as subpackages, but require some
additional constraints in order to retain their required privileges. Outside of
tests, a parent package MUST NOT include a subpackage for a child component that
requires more privileges than the parent. Components run in hermetic tests may
require an exception to this rule, but if an allowlisted package or component is
used as a subpackage in a hermetic test, the privileged features should be
replaced with mocks or an equivalent replacement, if possible.

If this restriction turns out to be a barrier to other system improvements, the
security team should be consulted regarding alternative accommodations.

### Controlled access to privileged package resolution operations

Today, given the flat package namespace, a PackageResolver client is able to
read all content in `blobfs`; therefore, the ability to request a package from
the Package Server, and read its BLOBs, are privileged operations.

Subpackages would offer only the specified subset of `blobfs` packages
(one-level deep, for now), but a package that declares a dependency on another
package is no more privileged to read that package's content. This RFC
recommends maintaining the same limits on what non-privileged clients can do
when requesting capabilities or assets from subpackages.

Components running from a specific package may not view the contents of their
subpackages, but they may view the `meta/subpackages` file and resolve its
contents manually if they have the PackageResolver capability.

This design increases security compared to the status quo of including dependent
components alongside each other in a single package. Under that solution each of
the components may arbitrarily view each others' contents. Because components
may see only the package they are running from, and not the contents of
subpackages, this design hides those implementation details.

### Implementation risk: Head of line blocking

If we implement depth-first package resolution, a nested subpackage DAG could
block resolutions of other packages. This could induce deadlock of the resolver
continues to hold blocking resources for a parent while resolving its
subpackages, recursively.

Since the RFC states that it will implement eager resolution only, the
implementor could mitigate this risk by ensuring a parent does not need to be
fully resolved before releasing resolver resources to resolve subpackages.

## Privacy considerations

No effect on privacy posture is anticipated.

## Testing

Unit tests will be added to validate the additional functionality. Existing
tests will help identify unexpected regressions.

Host-side packaging tests will validate the behavior of `ffx package` (and/or
`pm`, if necessary) and related tools, when processing subpackages.

Hermetic integration tests will be written to validate component resolution from
subpackages.

## Documentation

The following known documents will need to be updated:

  * Existing documentation describing Fuchsia package URLs and component URLs
    (to document how to reference subpackages), Software Delivery documentation,
    and Component Framework concept documents. Examples that currently show
    CML examples, with `fuchsia-pkg://` URL references, could be augmented with
    examples using subpackages).
  * Documentation on relative component references should be updated to explain
    the similarities and differences, compared with subpackaged components.

## Drawbacks, alternatives, and unknowns

See the [Future work](#future-work) subsection (in the Design section), which
describes some future enhancements, with alternative approaches and additional
features that were considered, before downscoping to the current version of this
RFC.

In addition, the following subsections describe some alternatives to the
proposed plan.

### Alternative: Injecting version hashes in child component URLs

Instead of introducing subpackages to the Software Delivery stack, we could
add tooling to make it possible for components to declare versioned dependencies
by adding a package hash qualifier to child component URLs. For example, a
hermetically-packaged dependency in the `children:` list of
`parent.cm` would appear as
`url: "fuchsia-pkg://fuchsia.com/some_package?hash=1234#meta/a_component.cm"`.

As long as tools and infrastructure can guarantee that version of `some_package`
is in the package store, this reference works with existing resolvers. A new
file (such as the notional `subpackages` file described in this RFC), and SWD
changes to interpret that file, would not be required.

This alternative was rejected because tooling and build infrastructure required
to make this work add more complexity, particularly to tooling and build
workflows, compared to the proposed solution. For example:

  * This alternative assumes package hashes are not embedded in developer CML,
    or in code where component URLs are resolved at runtime, for instance to
    add them to a `collection`. Some mechanism would need to be added to alter
    the compiled representations of component manifests and static references in
    source, to inject the `?hash=<blobid>` qualifier in the URLs, when the
    developer indicates (in some way) that they want the component URL to be
    pinned to a specific hash at build time.
  * Hermetic dependencies in component manifests enable runtime resolution, but
    a separate mechanism is required to support relocatable packaging with
    dependencies. Parsing the modified manifests and code is likely impractical,
    so whatever mechanism is used to determine where to make those modifications
    would also need to generate some additional metadata that describes the
    dependency graph. Tooling could then be changed to read that metadata in
    order to, for example, archive a package with all of its dependencies, or
    publish a package and all of its hash-pinned dependencies into a package
    store.

### Alternative: Use context-specific resolvers

Instead of passing and returning a `resolution_context`, when resolving a
component or package, the `Resolve` method could be modified to include a
`context_resolver` request handle (`server_end`) as a new input parameter.

Conceptually, either or both PackageResolver and Resolver could
model this FIDL protocol pattern:

```fidl
protocol Resolver {
  Resolve(struct {
      component_url string;
      context_resolver server_end:Resolver;
  }) -> (resource struct {
      component Component;
  }) error ResolverError;

  ...
}
```

In this alternative, the client end of the `context_resolver` MUST be used in
subsequent calls, to resolve component references on behalf of the returned
`Package` or `Component` (such as when resolving the Component URLs of
CML-declared `children`).

The primary reason this approach was rejected was that it adds the complexity of
maintaining a live connection to the resolvers. If a situation arose causing the
resolver server to restart, any or all of those channels might need to be
reestablished, which adds unnecessary complexity.

The context parameter should encode the information needed to map
a given subpackage name to the pinned package hash in the parent package's
`meta/subpackages` file, allowing `PackageResolver` and `Resolver`
implementations to survive resolver restarts, if necessary.

### Alternative: FIDL Type representation for resolution context values

We considered several approaches for how to represent the resolution context,
ultimately deciding to use a generic byte array for both `PackageResolver` and
`ContextResolver`. One advantage of this approach is that a byte array is a
common type for encoded values, and doesn't preclude encoding arbitrary content
(including null bytes).

Using a byte array was recommended by the FIDL team since there is not any other
explicit FIDL type for this pattern (which can be thought of as the "cookie
pattern", in web browser parlance).

Ultimately, we summarized the constraints for this type as follows:

  * The types should be marshallable without introducing an API dependency
    between fuchsia.pkg and fuchsia.component.resolution.
  * The context values need not survive the restart of component manager, but
    the Subpackages implementation should not require converting existing
    "non-critical" (that is, restartable) package serving components to
    "critical". (This constraint seems to rule out using a handle to a service.)
    Note that if component manager restarts, all components are (currently)
    re-resolved and re-started, with new context values. Context values do not
    need to persist through a component manager restart, let alone a reboot or
    power cycle.
  * The context can notionally be small (about the size of a package server name
    and a package hash). This means a handle to a VMO, for every subpackage,
    would probably be prohibitively expensive.

### Alternative: Component Manager uses the `resolution_context` to get the Resolver

When resolving a relative subpackaged component URL, we considered including the
URI scheme (such as `fuchsia-pkg`) as part of the
`Component::resolution_context`, to allow component manager to get the
`Resolver` via the resolver registry by scheme lookup. This was rejected because
doing so would, in theory, allow a `Resolver` to return a context with a
different scheme for resolving relative subpackaged components, which was
considered an architectural risk. Instead, component manager will keep track of
the `Resolver` used to resolve a component. Component Manager will always use
the `Resolver` of the component that requests another component by relative URL.

### Alternative: Relative components instead of relative packages

In Fuchsia, a package is a unit of software distribution, and the unit of
installation of software (executable code and other files). Although Fuchsia
components provide some of the more familiar use cases, cross-package
dependencies can exist between packages that may not involve components or
component-to-component dependencies (for example, a Fuchsia shell package might
depend on another package of assets, which does not have its own component).
Also, there is a not a convenient way to independently distribute prebuilt
components in the SDK.

### Alternative: Refer to subpackages using a special `fuchsia-subpkg://` scheme

This RFC proposes to interpret URI relative paths (a URI that begins with a path
segment, and omits the scheme and authority prefixes) as a reference to a
resource in a subpackage. The current proposal also limits subpackage references
to an immediate "child" package only, so the path MUST NOT have a slash. (Use of
a slash in subpackage references is reserved for possible future use.)

An alternative also considered was to require a special scheme prefix (such as
`fuchsia-subpkg://`) when referring to subpackages, in order to ensure the given
string is clearly intended for subpackage resolution.

Also, using the scheme prefix `fuchsia-subpkg` appears to imply a dependence on the
same resolver that handles the `fuchsia-pkg` scheme, which can be confusing.

The URI standard recommends beginning with the relative path only.
Requiring a special the scheme prefix can imply a dependence on a specific
scheme handler, limiting generality. Schema-less relative paths are widely
implemented and well understood (for example, in HTML,
`<a href="sub-path/page">` is implicitly handled as a relative location
reference, without requiring a special scheme).

## Prior art and references

**Standards**

* [IETF RFC 3986: Uniform Resource Identifier (URI): Generic Syntax](https://datatracker.ietf.org/doc/html/rfc3986)

**Accepted Fuchsia RFCs**

* [RFC-0104: Relative Component URLs](/docs/contribute/governance/rfcs/0104_relative_urls.md)
* [RFC-0124: Decentralized Product Integration: Artifact Description and Propagation](/docs/contribute/governance/rfcs/0124_decentralized_product_integration_artifact_description_and_propagation.md)
* [RFC-0145: Eager Package Updates](/docs/contribute/governance/rfcs/0145_eager_package_updates.md)

**Potentially related draft Fuchsia RFCs**

* [Component-scoped executability](https://fuchsia-review.googlesource.com/c/fuchsia/+/543282)
* [Configurable Package URL Authorities](https://fuchsia-review.googlesource.com/c/fuchsia/+/557361)
