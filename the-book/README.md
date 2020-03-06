<!--
# Fuchsia is not Linux
_A modular, capability-based operating system_

This document is a collection of articles describing the Fuchsia operating system,
organized around particular subsystems. Sections will be populated over time.

[TOC] -->

# Fuchsia 不是 Linux (并非基于linux内核)
_而是一种基于功能的模块化系统_

本文档是一系列关于 Fushsia 操作系统的描述，部分内容将在后续补充。

[TOC]

<!--
## Zircon Kernel

Zircon is the microkernel underlying the rest of Fuchsia. Zircon
also provides core drivers and Fuchsia's libc implementation.

 - [Concepts][zircon-concepts]
 - [System Calls][zircon-syscalls]
 - [vDSO (libzircon)][zircon-vdso] -->

## Zircon 内核
Zircon 是运行在 Fuchsia 下的微内核，Zircon 同时也提供系统所需的驱动程序和库。

 - [概念][zircon-concepts]
 - [系统调用][zircon-syscalls]
 - [vDSO (libzircon)][zircon-vdso]

<!--
## Zircon Core

 - Device Manager & Device Hosts
 - [Device Driver Model (DDK)][zircon-ddk]
 - [C Library (libc)](libc.md)
 - [POSIX I/O (libfdio)](life_of_an_open.md)
 - [Process Creation](process_creation.md) -->

## Zircon 核心

 - 设备管理器 & 设备 Hosts
 - [Device Driver Model (DDK)][zircon-ddk]
 - [C Library (libc)](libc.md)
 - [POSIX I/O (libfdio)](life_of_an_open.md)
 - [Process Creation](process_creation.md)

<!--
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
 - Links -->

## Zircon 框架

 - [核心库](core_libraries.md)
 - 应用模型
   - [接口定义语言 (FIDL)][FIDL]
   - 服务
   - 环境
 - [Boot 顺序](boot_sequence.md)
 - Device, user, and story runners
 - 组件
 - [命名空间](namespaces.md)
 - [沙箱](sandboxing.md)
 - [存储][framework-story]
 - [模块][framework-module]
 - [代理][framework-agent]
 - Links

<!--
## Storage

 - [Block devices](block_devices.md)
 - [File systems](filesystems.md)
 - Directory hierarchy
 - [Ledger][ledger]
 - Document store
 - Application cache -->

## 存储

 - [块设备](block_devices.md)
 - [文件系统](filesystems.md)
 - 目录层次结构
 - [Ledger分布式存储系统][ledger]
 - 文件管理器
 - 应用缓存

<!--
## Networking

 - Ethernet
 - [Wireless](wireless_networking.md)
 - [Bluetooth][bluetooth]
 - [Telephony][telephony]
 - Sockets
 - HTTP -->

## 网络

 - 以太网
 - [无线网](wireless_networking.md)
 - [蓝牙][bluetooth]
 - [电话/蜂窝][telephony]
 - Sockets
 - HTTP

<!--
## Graphics

 - [Magma (vulkan driver)][magma]
 - [Escher (physically-based renderer)][escher]
 - [Scenic (compositor)][scenic]
 - [Input manager][input-manager]
 - [View manager][view-manager]
 - [Flutter (UI toolkit)][flutter] -->

## 图像

 - [Magma-Fuchsia的GPU驱动架构 (vulkan 设备)][magma]
 - [Escher (物理渲染器)][escher]
 - [Scenic (合成器)][scenic]
 - [Input manager][input-manager]
 - [View manager][view-manager]
 - [Flutter (UI 工具包)][flutter]

<!--
## Media

 - Audio
 - Video
 - DRM -->
## 媒体

 - 音频
 - 视频
 - DRM

<!--
## Intelligence

 - Context
 - Agent Framework
 - Suggestions -->

## Intelligence

 - 上下文
 - 代理框架
 - 建议

<!--
## User interface

  - Device, user, and story shells
  - Stories and modules -->

## 用户界面

 - 设备、用户和存储shells
 - 存储和模块化

<!--
## Backwards compatibility

 - POSIX lite (what subset of POSIX we support and why)
 - Web runtime -->

## 向下兼容

 - POSIX lite (我们支持POSIX的哪个子集以及原因)
 - Web runtime

<!--
## Update and recovery

 - Verified boot
 - Updater -->
## 更新和恢复

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
