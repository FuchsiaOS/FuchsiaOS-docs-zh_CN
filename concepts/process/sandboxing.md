# Sandboxing

This document describes how sandboxing works for a process in Fuchsia.

## A new process has nothing

In Fuchsia, a newly created process is empty. It cannot access any kernel
objects, allocate memory, or execute code. Because of this, processes are
usually created with some initial resources and capabilities.

Most commonly, a process starts executing some code with an initial stack, some
command line arguments, some environment variables, and a set of initial
handles.
[Zircon program loading and dynamic linking](/docs/concepts/process/program_loading.md)
describes the resources provided to programs when starting.

## Namespaces are the gateway to the world

Some of the initial handles given to a process are directories that the process
mounts into its _namespace_. These handles let the process discover and
communicate with other processes running on the system, including file systems
and other servers. See [Namespaces](/docs/concepts/process/namespaces.md) for
more details.

The namespace given to a process strongly influences how much of the system the
process can influence. Therefore, configuring the sandbox in which a process
runs amounts to configuring the process's namespace.

## Package namespace

A [component][glossary.component] run from a package is given access to
`/pkg`, which is a read-only view of the package containing the component. To
access these resources at runtime, a process can use the `/pkg` namespace. For
example, the `root_presenter` can access `cursor32.png` using the absolute path
`/pkg/data/cursor32.png`.

## Component capabilities

Processes that are [components][glossary.component] receive an `/svc`
directory in their [namespace][glossary.namespace] containing
[protocols](/docs/concepts/components/v2/capabilities/protocol.md) and
[services](/docs/concepts/components/v2/capabilities/service.md).

A typical component will interact with a number of services from `/svc` in order
to play some useful role in the system. For example, the service
`fuchsia.logger.LogSink` is required if a component wishes to log.

Processes that are not components may or may not have `/svc`. These processes
receive whatever `/svc` their creator provided to them.

[glossary.component]: /docs/glossary/README.md#component
[glossary.environment]: /docs/glossary/README.md#environment
[glossary.namespace]: /docs/glossary/README.md#namespace
