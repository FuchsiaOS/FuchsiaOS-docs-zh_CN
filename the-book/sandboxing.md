# Sandboxing

This document describes how sandboxing works in Fuchsia.

## An empty process has nothing

On Fuchsia, a newly created process has nothing. A newly created process cannot
access any kernel objects, cannot allocate memory, and cannot even execute code.
Of course, such a process isn't very useful, which is why we typically create
processes with some initial resources and capabilities.

Most commonly, a process starts executing some code with an initial stack, some
command line arguments, environment variables, a set of initial handles. One of
the most important initial handles is the `PA_VMAR_ROOT`, which the process can
use to map additional memory into its address space.

## Namespaces are the gateway to the world

Some of the initial handles given to a process are directories that the process
mounts into its _namespace_. These handles let the process discover and
communicate with other processes running on the system, including file systems
and other servers. See [Namespaces](namespaces.md) for more details.

The namespace given to a process strongly influences how much of the system the
process can influence. Therefore, configuring the sandbox in which a process
runs amounts to configuring the process's namespace.

## Package namespace

A [component](../glossary.md#Component) run from a package is given access to
`/pkg`, which is a read-only view of the package containing the component. To
access these resources at runtime, a process can use the `/pkg` namespace. For
example, the `root_presenter` can access `cursor32.png` using the absolute path
`/pkg/data/cursor32.png`.

## Services

Processes that are [components](../glossary.md#Component) receive an `/svc`
directory in their namespace. The services available through `/svc` are a
subset of the services provided by the component's
[environment](../glossary.md#Environment). This subset is determined by the
[`sandbox.services`](package_metadata.md#sandbox) whitelist in the
component's [manifest file](package_metadata.md#Component-manifest).

A typical component will interact with a number of services from `/svc` in
order to play some useful role in the system. For example, the service
`fuchsia.sys.Launcher` is required if a component wishes to launch other
components.

Processes that are not components may or may not have `/svc`. These processes
receive whatever `/svc` their creator decided to provide to them.

*NOTE:* In the past, there existed no mechanism for service sandboxing and a
component received all services in its environment. Pre-existing components
have been grandfathered to receive all services with the
`deprecated-all-services` feature, which will eventually be removed. Please do
not use `deprecated-all-services` for new components.

## Configuring additional namespaces

If a process requires access to additional resources (e.g., device drivers),
the package can request access to additional names by including the `sandbox`
property in its  [Component Manifest](package_metadata.md#Component-Manifest)
for the package. For example, the following `meta/sandbox` file requests
direct access to the input driver:

```
{
    "dev": [ "class/input" ]
}
```

In the current implementation, the [AppMgr](../glossary.md#AppMgr) grants all such
requests, but that is likely to change as the system evolves.

## Building a package

To build a package, use the `package()` macro in `gn` defined in
[`//build/package.gni`](https://fuchsia.googlesource.com/build/+/master/package.gni).
See the documentation for the `package()` macro for details about including resources.

For examples, see [https://fuchsia.googlesource.com/garnet/+/master/packages/prod/fortune]
and [https://fuchsia.googlesource.com/garnet/+/master/bin/fortune/BUILD.gn].
