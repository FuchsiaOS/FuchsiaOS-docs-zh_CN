# Subpackaging components

[Packages] can "contain" other packages (referred to as their
[subpackage][glossary.subpackage]s), producing a hierarchy of nested packages.
[Components] can leverage subpackaging to organize a [hierarchy of nested
components][hierarchy-of-nested-components], where each component is
encapsulated in its own package, and brings its own set of dependencies.

_Subpackages enable:_

* Encapsulated dependencies (a packaged component declares its direct
  dependencies only)
* Isolated `/pkg` directories (grouped components don't need to merge their
  files, libraries, and metadata into a single shared namespace)
* Assured dependency resolution (system and build tools ensure subpackages
  always "travel with" their packages)

## Relationship to Fuchsia Components

Fuchsia uses packages to "distribute" its software (for example, to load the
software onto a device). A single component with no included dependencies is
typically contained in a single package. A component that launches other
components can define a package that _includes_ specific versions (determined at
build time) of its child components using subpackaging.

When organized this way, the subpackage hierarchy mirrors the [component
parent-child relationships][component-parent-child-relationship]. Child
components will then be loaded from a declared subpackage of the parent
component's package. This encapsulates [ABI][glossary.abi] dependencies at
package boundaries.

Components can also use a subpackage to declare a dependency on a data-only
package. While components cannot resolve a package directly, a package can
expose data to components selectively, by including a component manifest that
defines and exposes directory capabilities to other components. Each directory
capability maps to a specific package subdirectory. (With a data-only package,
the manifest does not include a `program` declaration.) By treating exposed data
as standard [directory capabilities], the components use [capability routing] to
ensure the right information is only made available to the components that need
it.

### Package dependendencies mirror Component dependencies

A Fuchsia system is defined by a hierarchy of components. Starting with the
first component (the root of the hierarchy), components add capabilities to the
system by launching `children` (child components) that serve those capabilities.
Each component has the opportunity to launch its own subtree of components.

To instantiate a `child` component, the parent identifies the child's source
(implementation software) by its location in the package system using a
"component URL" (a package URL combined with the intra-package resource location
of the component manifest); for example
`fuchsia-pkg://fuchsia.com/package#meta/component.cm`.

Importantly, under the Component Framework, the _only_ place a component refers
to a runtime dependency by component URL is when declaring children. Component
URLs are not used to define a dependency on a peer component or any other
component outside of its local subtree.

When a child component is defined by an absolute component URL like
`fuchsia-pkg://fuchsia.com/package#meta/component.cm`, the component developer
cedes control over the implementation of that dependency, to be determined
(potentially) at product assembly time, or at runtime from an ephemeral source
(a package server).

Subpackaging allows the developer to instead declare package dependencies with
build-time resolution, "baking in" the expected component implementations,
including known ABI and behavior, without compromising the encapsulation and
isolation benefits of package boundaries. This ensures a package with
component dependencies has a hermetic implementation, and the behavior of its
child components will not change without rebuilding the parent component's
package.

Subpackaged component URLs also avoid problems inherent with absolute component
URLs: If a parent component is loaded from (for example) an alternate repository
like `fuchsia-pkg://alt-repo.com/package#meta/parent.cm`, its likely that its
children may also be in that `alt-repo`, and there is no way to statically
define an absolute component URL that can resolve from either `fuchsia.com` or
`alt-repo.com` (or another) not known until runtime.

By using relative package paths, a subpackaged child component's implementation
is identified by a [relative component URL] with subpackage name (a subpackage
URL, with a URI fragment specifying the path to the component manifest), such as
`some-child#meta/default.cm`. The mapping from subpackage name `some-child` is
declared in a build configuration, and resolved at build time, by storing the
subpackage's package hash in the parent component's package metadata, mapped to
the subpackage name.

<aside class="key-point">
This superpackage <-> subpackage relationship between packages naturally mirrors
the parent <-> child relationship between components.
</aside>

### Dependencies are transitive and encapsulated

Component software implementations _do not `use`_ other components. Components
`use` capabilities. A component's capabilities may come from its parent
(routed directly or indirectly by the parent, without the knowledge of the
component) or from a child. Importantly, a capability exposed by a child can
also be either direct or indirect. The child's implementation is encapsulated,
so a capability it exposes may be implemented by that child, or may be routed
from one of the child's children.

Subpackaging allows a component to completely encapsulate its implementation,
including any dependencies on sub-components.

When a component declares children using absolute component URLs, the specific
implementation of that child is selected at runtime. This may be desired, for
certain use cases, but the trade-off is that the parent component is not
hermetic: It can be hard to re-use the parent component in new environments.
Distributing and porting non-hermetic code requires keeping track of all of the
external dependencies as well, and then ensuring the dependencies are always
available in each new environment.

```json5 {:.devsite-disable-click-to-copy}
    children: [
        {
            name: "intl_property_provider",
            url: "fuchsia-pkg://fuchsia.com/intl_property_manager#meta/intl_property_manager.cm",
        },
        ...
    ]
```

When runtime resolution is not required, the parent component can update its
children to use relative path URLs, and declare the child components' packages
as subpackage dependencies, resolved at build time. This way, when a component
subpackages a child component, the child's package brings all of its subpackaged
components inherently, without exposing those dependencies to the other
components and runtime environments that may use it.

```json5 {:.devsite-disable-click-to-copy}
    children: [
        {
            name: "intl_property_provider",
            url: "intl_property_manager#meta/intl_property_manager.cm",
        },
        ...
    ]
```

<aside class="key-point">
The subpackaged component can add, remove, or replace child components without
breaking API compatibility with the top component, as long as the child
component continues to serve the same capabilities, regardless of which
components implement which capabilities. Therefore subpackages provide a way to
mirror the encapsulation model of components.
</aside>

### No ambient authority through the `/pkg` directory

In order to support the basic runtime requirements of a Fuchsia Component,
a component may access a directory containing the contents of its package, via
the [`/pkg`][ambient-pkg-directory] directory capability.

As described above, subpackaging allows packages to declare their component
dependencies as hierarchical, encapsulated packages of components. This model
does not require a separate package per component, but it does encourage it, and
the Fuchsia runtime and tools are designed to make the process of declaring,
building, and running separately-packaged components natural and performant.

Conversely, multiple components combined in a single package share a single,
merged `/pkg` directory. Bundling more than one component in a single package
allows each component to access not only the same data, but also the metadata of
the other components in that package as well, without explicit capability
routing.

In certain cases, where multiple components share access to the same data, this
may be convenient. However, in cases where components need access to different
sets of data, or one component uses data that should not be exposed to the
other, packaging components together may undermine the [principle of least
privilege], making subpackages a better fit.

<<../../../get-started/_common/components/_no_ambient_authority.md>>

The fact that a component might not take advantage of this consequential
privilege is more of a concern than a relief because this might not always be
the case, and the privilege opens up an unexpected opportunity for one component
to exploit the data of another component.

<aside class="key-point">
Subpackages ensure each component has its own isolated <code>/pkg</code>
directory while providing the same benefits of relative URL resolution, and
improvements to software hermeticity and software encapsulation benefits through
hierarchical nesting.
</aside>

## Advantages over using multiple components in a single package

Today, Fuchsia allows a single package to contain multiple components. This
feature predates the existence of subpackages, and it provides another way
to declare child components by a relative URL; that is, by a URI fragment that
identifies the component by resource path to the component manifest. A
component URL of the form `#meta/some-child.cm` informs the Fuchsia component
resolver to load the component implementation for `some-child` from the same
package that contained the parent component's manifest.

### Built-in access controls to share package resources

The component framework helps to enforce Fuchsia's capability access control
policies by requiring components to declare their capability needs explicitly,
and by making the parent component responsible for routing any external
capabilities (including resources) from known capability sources (from the
parent's parent, or from another child).

If one component needs a resource from another component's package, the
Component Framework capability routing declarations allow the source component
to expose the specific subdirectory such that the target component can access
only what is required, and explicitly offered by its parent component.

This supports any use case that might otherwise have been satisfied by relying
on access to a shared `/pkg` directory from a common package, without exposing
the entire `/pkg` directory.

Subpackage-isolated `/pkg` directories combined with Component Framework
capability routing provide Fuchsia architecture-consistent way to control access
to and share package resources.

### Changes to transitive dependencies to not break encapsulation

When combining component dependencies into a single package, all components
share a single, flat namespace, and transitive dependencies must also be
included.

<!-- TODO(fxbug.dev/116980): Add a diagram to help visualize this example. -->

For example, if single package `SP` bundles component `A` and component `B`, but
`B` also depends on `C` by relative URI fragment (`#meta/C.cm`), package `SP`
must bundle `A`, `B`, and `C`. If `B` is later modified to replace `C` with two
new components `D` and `E`, the definition of package `SP` must change to bundle
`A`, `B`, `D`, and `E`, and drop `C` _unless_ (for the sake of argument) either
`D` or `E` (or both) also depend on `C`.

Although some build environments allow a component build target to declare
transitive component dependencies, this practice amplifies the risks of merging
the contents of these components into a single namespace. If a component _or any
of its dependencies_ changes, new files could overwrite files from other
components in any part of the component subtree in that package, breaking
implementations in undefined and potentially catastrophic ways.

Subpackages greatly simplify transitive dependencies by encapsulating them in
the definition of each subpackage, so package `SP` can be replaced with package
`A` (containing component `A`) having a dependency on _only_ subpackage `B`
(containing component `B`). Package `A` requires no other dependencies, and
does not change, even if `B`'s dependencies change.

### Subpackaged implementations are build-time guarantees

Using relative URI fragment component URLs (like, `#meta/some-child.cm`), does
not actually guarantee ABI or even API compatibility between parent and child
components "in the same package" because they could in fact be resolved from
different versions of that package.

If the package is resolved ephemerally (from a package server). A new version of
the same package can be re-published between the time the parent component was
resolved and a later time when the child component is required and loaded. The
child implementation might be different from the implementation included in the
original version of the package.

This is not a rare or contrived use case: In Component Framework, components are
(by default) resolved only when needed. A component that exposes a single
service `S` will not be loaded until and unless some other component actually
requires service `S`. Depending on the business logic of the program, `S` might
be called upon minutes or hours (or more) after the parent component was
launched.

## Examples

### Declaring build dependencies to subpackages

Fuchsia-enabled build frameworks should include a pattern for declaring a
Fuchsia package and its contents. If also enabled to support subpackages, a
package declaration will list the subpackages it depends on, by direct
containment.

For example, in fuchsia.git, the GN templates for declaring Fuchsia packages
support two optional lists, `subpackages` and (less commonly used)
`renameable_subpackages`. One or both can be included. The `renameable_`
version allows the package to assign a package-specific name to the subpackage,
used when referring to the subpackage by package URL or component URL:

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/subpackages/BUILD.gn" region_tag="declare_subpackages" adjust_indentation="auto" %}
```

The `subpackages` list contains a list of GN `fuchsia_package` build targets. By
default, the subpackage name (the name the containing package will use to refer
to the package) is taken from the defined `package_name` of the subpackage's
`fuchsia_package` target.

Subpackage targets can also be declared using the `package` variable in
the `renameable_subpackages` list. `renameable_targets` also include an optional
`name` variable, to override the default name for the subpackage.

### Declaring subpackaged children

A subpackage is only visible to its parent package, and the component(s) in that
package. Consequently, subpackage names only need to be unique within that
parent package. If two subpackage targets have the same name (or for any other
reason), the parent is free to assign its own subpackage names (via
`renameable_subpackages` in GN, for instance).

When declaring subpackaged child components in CML, the `url` should be the
relative subpackaged component URL, as shown in the following example:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/subpackages/meta/echo_client_with_subpackaged_server.cml" region_tag="declare_children_statically" adjust_indentation="auto" %}
```

Subpackaged child components can also be referenced in runtime declarations,
such as when declaring children through [Realm Builder] APIs. For example:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/subpackages/src/lib.rs" region_tag="declare_children_dynamically" adjust_indentation="auto" %}
```

[Packages]: /docs/concepts/packages/package.md
[Realm Builder]: /docs/development/testing/components/realm_builder.md
[Components]: /docs/concepts/components/v2/introduction.md
[ambient-pkg-directory]: /docs/concepts/components/v2/capabilities/life_of_a_protocol_open.md#offered-vs-ambient-capabilities
[component-parent-child-relationship]: /docs/concepts/components/v2/topology.md
[capability routing]: /docs/concepts/components/v2/topology.md#capability-routing
[directory capabilities]: /docs/concepts/components/v2/capabilities/directory.md
[hierarchy-of-nested-components]: /docs/concepts/components/v2/components_as_classes.md#component-manifests-as-classes
[principle of least privilege]: /docs/get-started/sdk/learn/intro/sandboxing.md
[relative component URL]: /docs/reference/components/url.md#relative-path-urls-to-subpackaged-components
[glossary.abi]: /docs/glossary/README.md#abi
[glossary.subpackage]: /docs/glossary/README.md#subpackage
