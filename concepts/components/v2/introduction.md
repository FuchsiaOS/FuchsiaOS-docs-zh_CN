# Introduction to the Fuchsia component framework (Components v2)

<<../_v2_banner.md>>

This document offers a brief conceptual overview of the component framework
along with links to more detailed documents on specific topics.

## Components and the component framework

A component is a unit of software that is sandboxed and interacts with other
components through [capabilities][glossary-capability].

The component framework is a [framework][wiki-software-framework] for developing
[component-based software][wiki-component-based-software] for Fuchsia.

The component framework is responsible for running nearly all software on
Fuchsia so it is important for developers to learn how it works and how to use
it effectively.

## Purpose

The component framework empowers developers to write software for Fuchsia with
an emphasis on [separation of concerns][wiki-separation-of-concerns],
modularity, and composition.

Each component typically has a small number of responsibilities and exposes
services that other components use to perform more complex tasks. For example,
the ethernet driver component exposes a service for sending ethernet frames. The
network stack component connects to the ethernet driver component and uses its
services to communicate with the hardware. These two components can be authored
by different parties and be distributed separately because they agree on a
common set of protocols that let them work together.

Emphasizing software composition has numerous advantages for a modern operating
system:

-   Configurability: The behavior of the system can be changed easily by adding,
    upgrading, removing, or replacing individual components.
-   Extensibility: As components are added, the functionality of the system
    grows.
-   Inclusion: Anyone can author new components.
-   Reliability: The system can recover from faults gracefully by stopping or
    restarting individual components.
-   Reuse: Existing components can be reused and composed with other components
    to solve new problems.
-   Testability: Prior to integration, each component can be verified separately
    so it is easier to isolate bugs.
-   Uniformity: All components describe their capabilities in the same way
    independent of their origin, purpose, or implementation language.

Overall, the component framework makes it easier to update and improve the
system incrementally as new software is created.

## A component is a hermetic composable isolated unit of software

A component is a **unit of software**.

-   It may be executable (a program), or it may present or
    delegate capabilities without containing any executable code.
-   It is identified by the [URL][doc-component-urls] from which its declaration
    and assets are retrieved.
-   It can be implemented with any runtime framework provided by a
    [component runner][doc-runners].
-   It has a [declaration][doc-declarations] that describes what it can do and
    how to run it.

A component is an **isolated** unit of software.

-   Each [instance][doc-instances] of a component runs in its own sandbox with
    its own lifecycle, state, and [capabilities][glossary-capability].
-   It cannot access capabilities other than those it has been granted.
-   Its capabilities cannot be accessed by other components unless they are
    explicitly granted.
-   It primarily communicates with other components via IPC.
-   It cannot compromise the integrity of the entire system, even if it crashes.

A component is a **composable** isolated unit of software.

-   It can combine and be combined with other components to form composite
    components with parent-child relationships.
-   It declares the capabilities it needs to use and those it needs to delegate
    to its children with [capability routing][doc-capability-routing].

A component is a **hermetic** composable isolated unit of software.

-   It encapsulates its implementation, state, capabilities, and children.
-   It can be seamlessly replaced with a different implementation as long as the
    new implementation uses and exposes the same capabilities.
-   The component's assets and [runner][doc-runners] together include everything
    needed to run the component; the system does not provide language-specific
    assets such as the C runtime library.

## Everything is a component (almost)

Components are ubiquitous. They are governed by the same mechanisms and they all
work together seamlessly.

Almost all software that runs on Fuchsia is a component, including:

-   Device drivers
-   End-user applications
-   Filesystems
-   Media codecs
-   Network stacks
-   Tests
-   Web pages

There are only a few exceptions, notably:

-   Bootloaders
-   Device firmware
-   Kernels
-   Bootstrap for the component manager itself
-   Virtual machine guest operating systems

## Further Reading

-   [Component manager][doc-component-manager]
-   [Component declarations][doc-declarations]
-   [Component topology][doc-topology]
-   [Component lifecycle][doc-lifecycle]
-   [Design principles][doc-design-principles]

[doc-capability-routing]: /docs/concepts/components/v2/topology.md#capability-routing
[doc-component-manager]: /docs/concepts/components/v2/component_manager.md
[doc-declarations]: /docs/concepts/components/v2/declarations.md
[doc-design-principles]: /docs/concepts/components/v2/design_principles.md
[doc-instances]: /docs/concepts/components/v2/topology.md#component-instances
[doc-lifecycle]: /docs/concepts/components/v2/lifecycle.md
[doc-runners]: /docs/concepts/components/v2/capabilities/runners.md
[doc-topology]: /docs/concepts/components/v2/topology.md
[doc-component-urls]: /docs/concepts/components/component_urls.md
[glossary-capability]: /docs/glossary.md#capability
[glossary-components-v1]: /docs/glossary.md#components-v1
[glossary-components-v2]: /docs/glossary.md#components-v2
[wiki-component-based-software]: https://en.wikipedia.org/wiki/Component-based_software_engineering
[wiki-separation-of-concerns]: https://en.wikipedia.org/wiki/Separation_of_concerns
[wiki-software-framework]: https://en.wikipedia.org/wiki/Software_framework
