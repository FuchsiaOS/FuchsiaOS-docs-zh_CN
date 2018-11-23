# The Fuchsia layer cake

Fuchsia is the name of the open source project and the complete technical
artifact produced by the open source project. The name "Fuchsia" appears in many
places throughout the codebase, and will be baked into API names exposed to
third-party developers. The names of the individual layers below (with the
exception of Zircon) are implementation details of how we develop Fuchsia, and
we should avoid baking those names into public APIs.

## Zircon

Zircon is the operating system's foundation: it mediates hardware access,
implements essential software abstractions over shared resources, and provides a
platform for low-level software development.

For example, Zircon contains the kernel, device manager, most core and
first-party device drivers, and low-level system libraries, such as libc and
fdio. Zircon also defines the Fuchsia IDL (FIDL), which is the protocol
spoken between processes in the system, as well as backends for C and C++. The
backends for other languages will be added by other layers.

## Garnet

Garnet provides device-level system services for software installation,
administration, communication with remote systems, and product deployment.

For example, Garnet contains the network, media, and graphics services. Garnet
also contains the package management and update system.

## Peridot

Peridot provides the services needed to create a cohesive, customizable,
multi-device user experience assembled from modules, stories, agents, entities,
and other components.

For example, Peridot contains the device, user, and story runners. Peridot also
contains the ledger and resolver, as well as the context and suggestion engines.

## Topaz

Topaz augments system functionality by implementing interfaces defined by
underlying layers. Topaz contains four major categories of software: modules,
agents, shells, and runners.

For example, modules include the calendar, email, and terminal modules, shells
include the base shell and the user shell, agents include the email and chat
content providers, and runners include the Web, Dart, and Flutter runners.
