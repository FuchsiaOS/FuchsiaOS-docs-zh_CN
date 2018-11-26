# Fuchsia 不是 Linux (并非基于linux内核)
_而是一种模块化的，基于功能的操作系统_

本文档是一系列关于Fushsia操作系统的描述，部分内容将在后续补充。

[TOC]

## Zircon 内核
Zircon 是运行在Fushia下的微内核，Zircon同时也提供系统所需的驱动程序和库。

 - [概念][zircon-concepts]
 - [系统调用][zircon-syscalls]
 - [vDSO (libzircon)][zircon-vdso]

## Zircon 核心

 - Device Manager & Device Hosts
 - [Device Driver Model (DDK)][zircon-ddk]
 - [C Library (libc)](libc.md)
 - [POSIX I/O (libfdio)](life_of_an_open.md)
 - [Process Creation](process_creation.md)

## Zircon 框架

 - [核心库](core_libraries.md)
 - 应用模型
   - [接口定义语言 (FIDL)][FIDL]
   - 服务
   - 环境
 - [Boot 顺序](boot_sequence.md)
 - Device, user, and story runners
 - 组件
 - [Namespaces](namespaces.md)
 - [沙箱](sandboxing.md)
 - [存储][framework-story]
 - [模块][framework-module]
 - [Agent][framework-agent]
 - Links

## 存储

 - [块设备](block_devices.md)
 - [文件系统](filesystems.md)
 - 目录层次结构
 - [Ledger分布式存储系统][ledger]
 - 文件管理器
 - 应用缓存

## 网络

 - 以太网
 - [无线网](wireless_networking.md)
 - [蓝牙][bluetooth]
 - [电话/蜂窝][telephony]
 - Sockets
 - HTTP

## 图像

 - [Magma-Fuchsia的GPU驱动架构 (vulkan 设备)][magma]
 - [Escher (物理渲染器)][escher]
 - [Scenic (合成器)][scenic]
 - [Input manager][input-manager]
 - [View manager][view-manager]
 - [Flutter (UI 工具包)][flutter]

## 媒体

 - 音频
 - 视频
 - DRM

## Intelligence

 - Context
 - 代理框架
 - 建议

## 用户界面

 - 设备、用户和存储shells
 - 存储和模块化

## 向下兼容

 - POSIX lite (我们支持POSIX的哪个子集以及原因)
 - Web runtime

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
