<!--
# Development

This document is a top-level entry point to all of Fuchsia documentation related
to **developing** Fuchsia and software running on Fuchsia.
-->

# 开发 

本文档是有关 Fuchsia **开发** 以及在 Fuchsia 上运行软件的所有文档的一个入口（概述）。

<!--
## Developer workflow

This sections describes the workflows and tools for building, running, testing
and debugging Fuchsia and programs running on Fuchsia.

 - [Getting started](../getting_started.md) - **start here**. This document
   covers getting the source, building and running Fuchsia.
 - [Source code](source_code/README.md)
 - [Multiple device setup](workflows/multi_device.md)
 - [Pushing a package](workflows/package_update.md)
 - [Changes that span layers](workflows/multilayer_changes.md)
 - [Debugging](workflows/debugging.md)
 - [Tracing][tracing]
 - [Trace-based Benchmarking][trace_based_benchmarking]
 - [LibFuzzer-based fuzzing](workflows/libfuzzer.md)
 - [Build system](build/README.md)
 - [Workflow FAQ](workflows/workflow_faq.md)
 - [Testing FAQ](workflows/testing_faq.md)
-->
 
 ## 开发人员工作流程

本节介绍了用于构建、运行、测试和调试 Fuchsia，以及在 Fuchsia 上运行程序的工作流程和工具。
 - [入门](../getting_started.md) - **从这里开始**：本文档介绍获取源代码，构建和运行 Fuchsia。
 - [源代码](source_code/README.md)
 - [在多个设备上](workflows/multi_device.md)
 - [Fuchsia 包](workflows/package_update.md)
 - [Changes that span layers](workflows/multilayer_changes.md)
 - [调试](workflows/debugging.md)
 - [追踪][tracing] <!-- Error -->
 - [可追踪的基准测试][trace_based_benchmarking]
 - [基于 LibFuzzer 的模糊测试](workflows/libfuzzer.md)
 - [构建系统](build/README.md)
 - [工作流程 FAQ](workflows/workflow_faq.md)
 - [测试 FAQ](workflows/testing_faq.md)

<!--
## Languages

 - [C/C++](languages/c-cpp/README.md)
 - [Dart](languages/dart/README.md)
 - [FIDL](languages/fidl/README.md)
 - [Go](languages/go/README.md)
 - [Rust](languages/rust/README.md)
 - [Flutter modules](languages/dart/mods.md) - how to write a graphical module
   using Flutter
-->
   
## 编程语言

 - [C/C++](languages/c-cpp/README.md)
 - [Dart](languages/dart/README.md)
 - [FIDL](languages/fidl/README.md)
 - [Go](languages/go/README.md)
 - [Rust](languages/rust/README.md)
 - [Flutter modules](languages/dart/mods.md) - 如何使用Flutter编写图形模块

<!--
## API

 - [System](api/system.md) - Rubric for designing the Zircon System Interface
 - [FIDL](api/fidl.md) - Rubric for designing FIDL protocols
 - [C](api/c.md) - Rubric for designing C library interfaces
-->

## API

 - [系统](api/system.md) - 用于设计 Zircon 系统接口的说明
 - [FIDL](api/fidl.md) - 设计 FIDL 协议的说明
 - [C](api/c.md) - 设计 C 库接口的说明

<!--
## SDK

 - [SDK](sdk/README.md) - information about developing the Fuchsia SDK
-->
 
## SDK

 - [SDK](sdk/README.md) - 有关开发 Fuchsia SDK 的信息

<!--
## Hardware

This section covers Fuchsia development hardware targets.

 - [Acer Switch Alpha 12][acer_12]
 - [Intel NUC][intel_nuc] (also [this](hardware/developing_on_nuc.md))
 - [Pixelbook](hardware/pixelbook.md)
-->
 
## 硬件

本节介绍了 Fuchsia 开发所支持的硬件设备。

 - [Acer Switch Alpha 12][acer_12] <!-- Error -->
 - [Intel NUC][intel_nuc] <!-- (also [this](hardware/developing_on_nuc.md)) --> <!-- Error -->
 - [Pixelbook](hardware/pixelbook.md)

<!--
## Conventions

This section covers Fuchsia-wide conventions and best practices.

 - [Layers](source_code/layers.md) - the Fuchsia layer cake, ie. how Fuchsia
   subsystems are split into a stack of layers
 - [Repository structure](source_code/layer_repository_structure.md) - standard way
   of organizing code within a Fuchsia layer repository
 - [Documentation standards](/best-practices/documentation_standards.md)
-->

## 约定

本节涵盖了有关 Fuchsia 的所有公约和最佳实践。

 - [Layers](source_code/layers.md) - Fuchsia 子系统如何拆分为多层
 - [仓库结构](source_code/layer_repository_structure.md) - Fuchsia layer 仓库组织代码的标准方法
 - [文档标准](/best-practices/documentation_standards.md)

<!--
## Miscellaneous

 - [CTU analysis in Zircon](workflows/ctu_analysis.md)
 - [Persistent disks in QEMU](workflows/qemu_persistent_disk.md)
-->

## 其它

 - [Zircon 交叉编译的静态分析](workflows/ctu_analysis.md)
 - [在硬盘上运行 QEMU](workflows/qemu_persistent_disk.md)


[acer_12]: https://fuchsia.googlesource.com/zircon/+/master/docs/targets/acer12.md "Acer 12"
[intel_nuc]: https://fuchsia.googlesource.com/zircon/+/master/docs/targets/nuc.md "Intel NUC"
[pixelbook]: hardware/pixelbook.md "Pixelbook"
[tracing]: https://fuchsia.googlesource.com/garnet/+/master/docs/tracing_usage_guide.md
[trace_based_benchmarking]: benchmarking/trace_based_benchmarking.md
