<!---

# FIDL in drivers

Devices may implement Interfaces, which are
[FIDL](/docs/development/languages/fidl/README.md) RPC protocols
that clients (services, applications, etc) use. The base device interface
supports POSIX style open/close/read/write IO. Interfaces are supported through
the `message()` operation in the base device interface.

Examples:

* [GPIO interface](https://fuchsia.dev/reference/fidl/fuchsia.hardware.gpio)
* [Ethernet interface](https://fuchsia.dev/reference/fidl/fuchsia.hardware.ethernet)

--->

# 驱动中的 FIDL

设备可以实现接口，这些接口是客户端（服务端，应用等）使用的 [FIDL](/docs/development/languages/fidl/README.md) 远程过程调用协议。基础设备接口支持 POSIX 类型的开/关/读/写 IO。通过在基本设备接口中的 `message()`  操作来做接口支持。

示例：

* [GPIO interface](https://fuchsia.dev/reference/fidl/fuchsia.hardware.gpio)
* [Ethernet interface](https://fuchsia.dev/reference/fidl/fuchsia.hardware.ethernet)

