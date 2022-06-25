# FIDL in drivers

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

Devices may implement Interfaces, which are
[FIDL](development/languages/fidl/README.md) RPC protocols
that clients (services, applications, etc) use. The base device interface
supports POSIX style open/close/read/write IO. Interfaces are supported through
the `message()` operation in the base device interface.

Examples:

* [GPIO interface](https://fuchsia.dev/reference/fidl/fuchsia.hardware.gpio)
* [Ethernet interface](https://fuchsia.dev/reference/fidl/fuchsia.hardware.ethernet)
