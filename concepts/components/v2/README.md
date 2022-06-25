# Components

Components are the basic unit of executable software on Fuchsia.

Note: This section contains documentation about components using the modern
component framework ([Components v2][glossary.components v2]). The Fuchsia
platform team is currently [migrating legacy components][migration] to the
modern component framework. For details on legacy components, see
[Legacy components][legacy-components].

## Architectural concepts

-   [Introduction](introduction.md): Understanding components and the component
    framework.
-   [Component manager](component_manager.md): The runtime.
-   [Lifecycle](lifecycle.md): Component instance progression from creation to
    destruction.
-   [Topology](topology.md): The relationships among component instances.
-   [Realms](realms.md): Sub-trees of the component instance topology.
-   [Identifiers](identifiers.md): Identifiers for components and
    component instances.

## Developing components

-   [Capabilities](capabilities/README.md): Different types of capabilities and
    how to route them between components.
-   [Component manifests](component_manifests.md): How to define a component for
    the framework.
-   [ELF runner](elf_runner.md): How to launch a component from an ELF file.
    Typically useful for developing system components in C++, Rust, or Go.

## Extending the component framework

-   [Runners](capabilities/runners.md): Instantiate components; add support for
    more runtimes.
-   [Resolvers](capabilities/resolvers.md): Find components from URLs; add
    support for methods of software packaging and distribution.

## Diagnostics

-   [Hub](hub.md): A live view of the component topology at runtime.

## Internals

-   [Component manifest design principles][rfc0093]
-   [Components vs. processes](components_vs_processes.md): how the concepts
    differ.

[glossary.components v1]: glossary/README.md#components-v1
[glossary.components v2]: glossary/README.md#components-v2
[legacy-components]: concepts/components/v1/README.md
[migration]: contribute/open_projects/components/migration.md
[rfc0093]: contribute/governance/rfcs/0093_component_manifest_design_principles.md
