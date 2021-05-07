# Common capabilities

This guide describes commonly used protocols that must be in a component's
sandbox services (for [components v1](/docs/glossary.md#components-v1)) or
capabilities (for [components v2](/docs/glossary.md#components-v2)).

> See examples of capability configuration for [components v1][v1_example] or
[components v2][v2_example].

## Networking

Typical protocols used for components that require networking are

* [`fuchsia.posix.socket.Provider`] for creating sockets (`socket`).
* [`fuchsia.net.NameLookup`] for resolving hostnames (`getaddrinfo`).
* [`fuchsia.device.NameProvider`] to get the device name (`uname`).

POSIX sockets are provided by [libc] and [fdio] through `socket` calls that
require the [`fuchsia.posix.socket.Provider`] capability.

Address resolution is similarly provided by [libc] through `getaddrinfo`, which
requires the [`fuchsia.net.NameLookup`] capability.

[`fuchsia.posix.socket.Provider`]: https://fuchsia.dev/reference/fidl/fuchsia.posix.socket#Provider
[`fuchsia.net.NameLookup`]: https://fuchsia.dev/reference/fidl/fuchsia.net#NameLookup
[`fuchsia.device.NameProvider`]: https://fuchsia.dev/reference/fidl/fuchsia.device#NameProvider
[libc]: /docs/concepts/system/libc.md
[fdio]: /docs/concepts/system/life_of_an_open.md
[v1_example]: /docs/concepts/components/v1/component_manifests.md
[v2_example]: /docs/concepts/components/v2/capabilities/protocol.md#consuming_protocol_capabilities
