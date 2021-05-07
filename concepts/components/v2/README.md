# Components v2

This section contains documentation about components in the new component
framework ([components v2][glossary-components-v2]).

Components are the basic unit of executable software on Fuchsia.

Note: The component framework is under active development. This document
only covers the new architecture (components v2) implemented by
`component_manager`. The old architecture ([components v1][glossary-components-v1])
implemented by `appmgr` is still in use but will be removed once
the transition to the new architecture is complete.

## Architectural concepts

- [Introduction](introduction.md): What are components and the component
  framework.
- [Component manager](component_manager.md): The runtime.
- [Declarations](declarations.md): Describe components themselves.
- [Lifecycle](lifecycle.md): Component instance progression from creation to
  destruction.
- [Topology](topology.md): The relationships among component instances.
- [Realms](realms.md): Sub-trees of the component instance topology.
- [Monikers](monikers.md): Identifiers for component instances based on
  the component topology.

## Developing components

- [Capabilities](capabilities/README.md): Different types of capabilities and
  how to route them between components.
- [Component manifests](component_manifests.md): How to define a component for
  the framework.
- [ELF runner](elf_runner.md): How to launch a component from an ELF file.
  Typically useful for developing system components in C++, Rust, or Go.

## Extending the component framework

- [Runners](capabilities/runners.md): Instantiate components; add support for more
  runtimes.
- [Resolvers](capabilities/resolvers.md): Find components from URLs; add support for
  methods of software packaging and distribution.

## Debugging and troubleshooting

- [Hub](hub.md): A live view of the component topology at runtime.
- [OpaqueTest](opaque_test.md): Hermetic testing framework.

## Components (either version)

- [Component URLs][doc-component-urls] are URLs that identify components.
- [Components vs. processes](components_vs_processes.md): how the concepts differ

## Testing

- [Test components][test-components]:
  defining components that implement tests and running them.
- [Test Runner Framework][trf]:
  writing idiomatic tests in different languages that use common testing frameworks.
- [Complex topologies and integration testing][integration-testing]:
  testing interactions between multiple components in isolation from the rest of the
  system.

## Meta

- [State of the Components v2 migration](migration.md): a work in progress.

## Internals

- [Design principles](design_principles.md): Guidelines for arriving at
  architectural decisions.

[doc-component-urls]: /docs/concepts/components/component_urls.md
[glossary-components-v1]: /docs/glossary.md#components-v1
[glossary-components-v2]: /docs/glossary.md#components-v2
[test-components]: /docs/concepts/testing/v2/v2_test_component.md
[trf]: /docs/concepts/testing/v2/test_runner_framework.md
[integration-testing]: /docs/concepts/testing/v2/v2_integration_testing.md
