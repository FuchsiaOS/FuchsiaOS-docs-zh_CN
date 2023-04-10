# Fuchsia's software model

Fuchsia is an operating system that does many things differently from
traditional operating systems. Before you attempt to develop for Fuchsia, it is
important to understand how the software model works in Fuchsia.

In Fuchsia, almost everything is a component and it is the unit of
executable software. The Component framework is what runs all components on
Fuchsia. For more information on components and the component framework, see
[Introduction to the Fuchsia component framework](/docs/concepts/components/v2/introduction.md).

An API dependency allows different components to define a dependency on another
component. These API dependencies are abstract which means that the implementation
of the interface is defined by another component and resolved by the component
framework. Components communicate with their dependencies through FIDL, which is
the language that Fuchsia uses to describe interprocess communication (IPC)
protocols of components that run on Fuchsia. For more information on FIDL, see
[FIDL overview](/docs/concepts/fidl/overview.md).

## Distributing components through packages

In Fuchsia, components and their dependent files and images are often
distributed through packages which is the unit of distribution in Fuchsia.

Note: For more information on packages, see [Fuchsia packages](/docs/concepts/packages/package.md)

Fuchsia resolves packaged dependencies at install time, creating an ABI
dependency. References to resources from an _external_ package are resolved at
runtime, creating an API dependency, but not an ABI dependency. (Runtime
resolution is similar to a web services architectural model.)

Components are organized to keep critical dependencies in a package; and this
extends to [subpackages](/docs/concepts/components/v2/subpackaging.md) which are
bound to their containing package at build time, allowing ABI dependencies to be
resolved statically.

Note: Executable components (programs) implement API dependencies through
[FIDL](/docs/concepts/fidl/overview.md)

A logical way to package components that launch other components is to use
subpackages to mirror the component parent-child relationship hierarchy such
that, if a component declares a child component, the child is loaded from a
declared subpackage of the parent component's package. This encapsulates the ABI
dependencies and ensures the presence of the expected ABI version of the child
component. Components model API dependencies through capability routing
(services exposed, routed, and used, by capability name, such as a FIDL
protocol). Package dependencies are less relevant to capability routing, except
that a parent component can orchestrate the creation of independently-packaged
peer components (subpackaged or not) and declare the capability connections
between them.
