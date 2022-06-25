# Secure

Security and privacy are woven deeply into the architecture of Fuchsia.
The basic building blocks of Fuchsia, the kernel primitives, are exposed to
applications as object-capabilities. This means that applications running on
Fuchsia have no ambient authority: applications can interact only with the
objects to which they have been granted access explicitly.

Software is delivered in hermetic packages and everything is sandboxed. All
software that runs on the system, including applications and system components,
receives the least privilege it needs to perform its job and gains access only
to the information it needs to know. Because capability routing and software
isolation are enforced by the operating system, developers don’t have to build
an additional system for security.

## Fuchsia builds on a kernel designed to securely isolate software

**[Zircon][zircon] is a capability-based, object-oriented kernel**

The Zircon system fully isolates processes by default, and must explicitly grant
capabilities and resources. Fuchsia passes capabilities and resources by handles
rather than name, which leads to a system that only grants software access to
what it needs.

## Components are the fundamental unit of software execution

**[Components][components] are isolated containers for Fuchsia software**

Nearly all user space software is a component, from system services to end-user
applications. The component framework encourages the composition of loosely
coupled software. Capabilities used and exposed must be explicitly declared.

## Software is delivered in self-contained packages

**[Packages][packages] have everything they need to run every time**

Components are distributed through hermetic, or self-contained, packages that
include all needed files. Fuchsia packages are a collection of components,
files, and metadata. Isolated namespaces mean a component only has visibility
to its own package.

## Fuchsia has no global file system or ambient authority

**[Namespaces] prevent programs from escaping their containers**

Fuchsia aims to have no ambient authority, which means every operation is
scoped to an object capability. Similarly, Fuchsia has no global file system.
Instead, each program is given its own local namespace in which to operate.

[zircon]: /docs/concepts/kernel/README.md
[components]: /docs/concepts/components/v2/introduction.md
[packages]: /docs/concepts/packages/package.md
[namespaces]: /docs/concepts/process/namespaces.md