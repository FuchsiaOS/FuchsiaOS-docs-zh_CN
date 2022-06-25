# Environments

<<../_v2_banner.md>>

*Environments* provide a way to configure certain choices the framework makes
for components in a [realm][doc-realms].
[Component manifests][doc-component-manifests] may define environments and
assign them to child realms. An environment applies to a component instance's
entire realm, unless some sub-realm overrides it (see
[Propagation](#propagation)).

## Properties {#properties}

Environments let you configure the following behavior of a realm:

-   [Component runners](#runners)
-   [Component resolvers](#resolvers)

### Runners {#runners}

The component framework is runtime-agnostic and can support new runtime
environments and programming languages without requiring changes to
component manager or to other components. Runners provide the extension point
for components to interact with component manager and add runtime support to
Fuchsia. Some example runners are:

-   The [ELF runner][elf-runner] runs binaries compiled to the ELF file format.
-   The Dart AOT runner provides a runtime for Dart programs, such as a VM.
-   The Chromium web runner provides a runtime for components implemented as web
    pages.

Component manager identifies _what_ to execute and delegates _how_ execution
works to the runner. Runner implementations are free to choose an appropriate
strategy for executing their components, including:

-   Start a new process for the component.
-   Isolate the component within a virtual machine.
-   Run the component in the same process as the runner.
-   Execute the component as a job on a remote computer.

For more details on using and implementing runners, see
[runner capabilities](capabilities/runners.md).

### Resolvers {#resolvers}

Component resolvers interact with component manager on behalf of a component to
resolve its children from a given [component URL][glossary.component-url].
Resolvers are registered with a particular URL scheme (`http`, `fuchsia-pkg`, etc.)
and provide an implementation to fetch the component from the desired URL and
return a [component declaration][glossary.component-declaration].

If the component being resolved has an associated package, the resolver also
returns a [`fuchsia.io.Directory`][fidl-directory] handle representing the
package directory.

For more details on using and implementing resolvers, see
[resolver capabilities](capabilities/resolvers.md).

## Declaring {#declaring}

Define a new environment by adding an [environments][doc-environments]
declaration to a [component manifest][doc-component-manifests].

For an environment to be used, you must assign it to a child or collection. See
[Propagation](#propagation).

Environments support two modes of extension, [`REALM`][fidl-extends] or
[`NONE`][fidl-extends]:

-   [`REALM`][fidl-extends]: The environment inherits its properties from the
    environment that was assigned to this component (the "parent environment").
    Any new properties will be added on top of those inherited from the parent
    environment. Any properties that overlap with the parent environment will
    override the parent.
-   [`NONE`][fidl-extends]: The environment starts empty, with no initial
    properties.

## Propagation {#propagation}

A component instance is assigned an environment in one of two ways:

-   Its [child][doc-children] or [collection][doc-collections] does not
    have `environment` set. In this case, it will receive its parent's
    environment. This is the most common case.
-   Its [child][doc-children] or [collection][doc-collections] sets
    `environment`, which refers to one of the [`environments`][doc-environments]
    defined by this component.

The [root component][doc-root-component] is assigned an environment by
[component manager][doc-component-manager]. This includes a bootstrap resolver,
the [ELF runner][doc-elf-runner], and default configuration options.

## Environments vs. capability routing {#cap-routing}

The semantics of environments contrast with
[capability routing][doc-capability-routing]. With capability routing, a
capability must be explicitly [exposed][doc-expose] or [offered][doc-offer] by
every component in the path from the provider to the consumer. The explicit
nature of capability routing makes it easy to guarantee that components don't
receive access to capabilities they shouldn't have, thus maintaining the
[principle of least privilege][wiki-least-privilege].

However, there are some configuration choices that don't make sense to configure
on a per-component basis. For example, consider [runners][doc-runners]. Almost
every component needs to use a runner, but defining a new runner is not very
common -- certainly less common than defining a protocol capability, for
instance. Furthermore, access to a runner doesn't inherently grant a component
much privilege, for the component framework mediates access to the runner's
protocol and the component can't use that protocol directly. Therefore, runner
capabilities are registered in an environment, which makes them available to any
component in the realm to which that environment was assigned (unless some
sub-realm decides to set a new environment with the runner absent).

[glossary.component-url]: glossary/README.md#component-url
[glossary.component-declaration]: glossary/README.md#component-declaration
[doc-capability-routing]: ./capabilities/README.md#routing
[doc-children]: https://fuchsia.dev/reference/cml#children
[doc-collections]: https://fuchsia.dev/reference/cml#collections
[doc-component-manager]: ./component_manager.md
[doc-root-component]: ./component_manager.md#booting-the-system
[doc-component-manifests]: ./component_manifests.md
[doc-elf-runner]: ./elf_runner.md
[doc-environments]: https://fuchsia.dev/reference/cml#environments
[doc-expose]: https://fuchsia.dev/reference/cml#expose
[doc-offer]: https://fuchsia.dev/reference/cml#offer
[doc-realms]: ./realms.md
[doc-runners]: ./capabilities/runners.md
[doc-use]: https://fuchsia.dev/reference/cml#use
[elf-runner]: concepts/components/v2/elf_runner.md
[fidl-directory]: /sdk/fidl/fuchsia.io/directory.fidl
[fidl-extends]: /sdk/fidl/fuchsia.component.decl/environment.fidl
[wiki-least-privilege]: https://en.wikipedia.org/wiki/Principle_of_least_privilege
