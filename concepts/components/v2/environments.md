# Environments (Components v2)

<<../_v2_banner.md>>

*Environments* provide a way to configure certain choices the framework makes
for components in a [realm][doc-realms].
[Component manifests][doc-component-manifests] may define environments and
assign them to child realms. An environment applies to a component instance's
entire realm, unless some sub-realm overrides it (see
[Propagation](#propagation)).

## Properties {#properties}

Environments let you configure the following behavior of a realm:

-   [Setting the runners available to components](#runners)
-   [Setting the resolvers available to components](#resolvers)

### Runners {#runners}

By registering a runner in an [environment declaration][doc-environments], you
make it available to any component instance that has that environment assigned
to it. Components specify which runner they use with a [`use`][doc-use]
declaration naming the runner.

### Resolvers {#resolvers}

Resolvers registered to an [environment declaration][doc-environments] participate
in component URL resolution for any child components which have that environment
assigned to them through [propagation](#propagation).

See the [resolver capability](capabilities/resolvers.md) for more information.

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

[doc-capability-routing]: ./component_manifests.md#capability-routing
[doc-children]: ./component_manifests.md#children
[doc-collections]: ./component_manifests.md#collections
[doc-component-manager]: ./component_manager.md
[doc-root-component]: ./component_manager.md#booting-the-system
[doc-component-manifests]: ./component_manifests.md
[doc-elf-runner]: ./elf_runner.md
[doc-environments]: ./component_manifests.md#environments
[doc-expose]: ./component_manifests.md#expose
[doc-offer]: ./component_manifests.md#offer
[doc-realms]: ./realms.md
[doc-runners]: ./capabilities/runners.md
[doc-use]: ./component_manifests.md#use
[fidl-extends]: /sdk/fidl/fuchsia.sys2/decls/environment_decl.fidl
[wiki-least-privilege]: https://en.wikipedia.org/wiki/Principle_of_least_privilege
