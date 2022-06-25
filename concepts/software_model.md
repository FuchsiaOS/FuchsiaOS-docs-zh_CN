# Fuchsia's software model

Fuchsia is an operating system that does many things differently from
traditional operating systems. Before you attempt to develop for Fuchsia, it is
important to understand how the software model works in Fuchsia.

In Fuchsia, almost everything is a component and it is the unit of
executable software. The Component framework is what runs all components on
Fuchsia. For more information on components and the component framework, see
[Introduction to the Fuchsia component framework](concepts/components/v2/introduction.md).

An API dependency allows different components to define a dependency on another
component. These API dependencies are abstract which means that the implementation
of the interface is defined by another component and resolved by the component
framework. Components communicate with their dependencies through FIDL, which is
the language that Fuchsia uses to describe interprocess communication (IPC)
protocols of components that run on Fuchsia. For more information on FIDL, see
[FIDL overview](concepts/fidl/overview.md).

## Distributing components through packages

In Fuchsia, components and their dependent files and images are often
distributed through packages which is the unit of distribution in Fuchsia.
Components can only use shared libraries that are included in the same
package as the component. This allows multiple components in the same package
to use the same shared library. This is known as an ABI dependency.

Note: For more information on packages, see [Fuchsia packages](concepts/packages/package.md)

There is no concept of inter-package dependencies because transitive dependency
closures have unbounded resolution time. Components are organized to keep
critical dependencies in a package, and service dependencies are resolved by
runtime resolution, and do not specify exact peer implementations or versions.
This is known as an API dependency.

Note: Executable components (programs) implement API dependencies through
[FIDL](concepts/fidl/overview.md)

Fuchsia resolves dependencies at runtime instead of resolving at install
time like most other operating systems. This applies to API dependencies,
but not to ABI dependencies. This approach is similar to a web services
architectural model.
