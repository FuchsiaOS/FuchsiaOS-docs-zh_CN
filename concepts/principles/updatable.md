# Updatable

Fuchsia distributes software in packages,
which are hermetically sealed bundles of components, related files, and dependencies.
Fuchsia packages are designed to be updated independently or even delivered ephemerally,
which means they can come and go from the device as needed and the software is always up to date,
like a web page.

Fuchsia aims to provide drivers with a binary-stable interface.
In the future,
drivers compiled for one version of Fuchsia will continue to work
in future versions of Fuchsia without needing to be modified or even recompiled.
This approach means that Fuchsia devices will be able
to update to newer versions of Fuchsia seamlessly while keeping their existing drivers.

## Almost all software on Fuchsia is a component

**[The component framework](/concepts/components/v2/introduction.md)
makes it easier to update the system as new software is created**

The kernel has a minimal set of responsibilities,
nearly everything else is in a user space component.
Components are identified by URLs and
can be resolved, downloaded, and executed on demand like the web.
They are governed by the same mechanisms and they all work together.
Hermetic packaging of components leads to more portable software.

## Software is interchangeable and reusable

**[Fuchsia Interface Definition Language (FIDL)](/concepts/fidl/overview.md)
enables loose coupling between components**

Components exchange capabilities as defined by FIDL protocols.
Software is composed at runtime through protocols
rather than through static composition of libraries.
Fuchsia has no system libraries.
Even the C standard library [(libc)](/concepts/kernel/libc.md)
is expressed as a dependency,
delivered only when software needs it.
Components can be swapped with another implementation
as long they express the same FIDL protocol.

## Push updates and security patches to all products on demand

**[Fuchsia packages](/concepts/packages/package.md)
are the units of software distribution**

All software is delivered in packages that
can be updated independently and delivered on demand, like the web.
This enables a vulnerability patch to be pushed to all Fuchsia products at once
without the need for individual product coordination.

## On the roadmap

This section covers features on
[Fuchsia's roadmap](/contribute/roadmap/index.md).

### Update the system without modifying the driver

**[Drivers](/development/drivers/concepts/getting_started.md)
and system services are designed as user space components that
can be updated independently of the core OS**

We are designing the system so that Fuchsia products can receive system updates
without having to modify or recompile drivers.
Drivers, system services, and end-user applications would be updated
independently through the same mechanism, reducing the maintenance burden.
Device owners could receive Fuchsia updates without having to update
their drivers.
