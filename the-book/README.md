# Fuchsia is not Linux
_A modular, capability-based operating system_

This document is a collection of articles describing the Fuchsia operating system,
organized around particular subsystems. Sections will be populated over time.

[TOC]

## Zircon Kernel

Zircon is the microkernel underlying the rest of Fuchsia. Zircon
also provides core drivers and Fuchsia's libc implementation.

 - [Concepts][zircon-concepts]
 - [System Calls][zircon-syscalls]
 - [vDSO (libzircon)][zircon-vdso]

## Zircon Core

 - Device Manager & Device Hosts
 - [Device Driver Model (DDK)][zircon-ddk]
 - [C Library (libc)](libc.md)
 - [POSIX I/O (libfdio)](life_of_an_open.md)
 - [Process Creation](process_creation.md)

## Framework

 - [Core Libraries](core_libraries.md)
 - Application model
   - [Interface definition language (FIDL)][FIDL]
   - Services
   - Environments
 - [Boot sequence](boot_sequence.md)
 - Device, user, and story runners
 - Components
 - [Namespaces](namespaces.md)
 - [Sandboxing](sandboxing.md)
 - [Story][framework-story]
 - [Module][framework-module]
 - [Agent][framework-agent]
 - Links

## Storage

 - [Block devices](block_devices.md)
 - [File systems](filesystems.md)
 - Directory hierarchy
 - [Ledger][ledger]
 - Document store
 - Application cache

## Networking

 - Ethernet
 - [Wireless](wireless_networking.md)
 - [Bluetooth][bluetooth]
 - [Telephony][telephony]
 - Sockets
 - HTTP

## Graphics

 - [Magma (vulkan driver)][magma]
 - [Escher (physically-based renderer)][escher]
 - [Scenic (compositor)][scenic]
 - [Input manager][input-manager]
 - [View manager][view-manager]
 - [Flutter (UI toolkit)][flutter]

## Media

 - Audio
 - Video
 - DRM

## Intelligence

 - Context
 - Agent Framework
 - Suggestions

## User interface

 - Device, user, and story shells
 - Stories and modules

## Backwards compatibility

 - POSIX lite (what subset of POSIX we support and why)
 - Web runtime

## Update and recovery

 - Verified boot
 - Updater

[zircon-concepts]: https://fuchsia.googlesource.com/zircon/+/master/docs/concepts.md
[zircon-syscalls]: https://fuchsia.googlesource.com/zircon/+/master/docs/syscalls.md
[zircon-vdso]: https://fuchsia.googlesource.com/zircon/+/master/docs/vdso.md
[zircon-ddk]: https://fuchsia.googlesource.com/zircon/+/HEAD/docs/ddk/overview.md
[FIDL]: https://fuchsia.googlesource.com/docs/+/master/development/languages/fidl/README.md
[framework-story]: https://fuchsia.googlesource.com/peridot/+/master/docs/modular/story.md
[framework-module]: https://fuchsia.googlesource.com/peridot/+/master/docs/modular/module.md
[framework-agent]: https://fuchsia.googlesource.com/peridot/+/master/docs/modular/agent.md
[ledger]: https://fuchsia.googlesource.com/peridot/+/master/docs/ledger/README.md
[bluetooth]: https://fuchsia.googlesource.com/garnet/+/HEAD/bin/bluetooth/README.md
[telephony]: https://fuchsia.googlesource.com/garnet/+/HEAD/bin/telephony/README.md
[magma]: https://fuchsia.googlesource.com/garnet/+/master/lib/magma/
[escher]: https://fuchsia.googlesource.com/garnet/+/master/public/lib/escher/
[scenic]: https://fuchsia.googlesource.com/garnet/+/master/docs/ui/scenic.md
[input-manager]: https://fuchsia.googlesource.com/garnet/+/master/docs/ui_input.md
[view-manager]: https://fuchsia.googlesource.com/garnet/+/master/bin/ui/view_manager/
[flutter]: https://flutter.io/
