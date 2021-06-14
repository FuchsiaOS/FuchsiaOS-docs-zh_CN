<!-- # Development guides

This document is a top-level entry point to all of Fuchsia documentation related
to developing Fuchsia and software running on Fuchsia.
-->
# 开发指南

本文档是开发Fuchsia及在其上运行的软件的所有相关文档的顶层入口。
<!--
## Developer workflow

This sections describes the workflows and tools for building, running, testing
and debugging Fuchsia and programs running on Fuchsia.
-->
## 开发流程
本节描述了构建、运行、测试和调试Fuchsia及在其上运行的应用程序的流程和工具。

<!--
 - [Getting started](/docs/get-started/README.md) - This document
   covers getting the source, building and running Fuchsia.
 - [Source code](/docs/get-started/get_fuchsia_source.md)
 - [fx workflows](build/fx.md)
 - [Pushing a package](/docs/concepts/packages/package_update.md)
 - [Working across different petals](source_code/working_across_petals.md)
 - [Build system](/docs/concepts/build_system/index.md)
 - [Workflow tips and FAQ](source_code/workflow_tips_and_faq.md)
 - [Testing FAQ](testing/faq.md)
-->
 - [入门](/docs/get-started/README.md) - 本文档涵盖如何获取源码、构建和运行Fuchsia。
 - [源码](/docs/get-started/get_fuchsia_source.md)
 - [fx工作流](build/fx.md)
 - [推送一个包](/docs/concepts/packages/package_update.md)
 - [跨分支工作](source_code/working_across_petals.md)
 - [构建系统](/docs/concepts/build_system/index.md)
 - [工作流提示及常见问题](source_code/workflow_tips_and_faq.md)
 - [测试常见问题](testing/faq.md)

<!--
## Languages

 - [README](languages/README.md) - Language usage in Fuchsia
 - [C/C++](languages/c-cpp/README.md)
 - [Dart](languages/dart/README.md)
 - [FIDL](languages/fidl/README.md)
 - [Go](languages/go/README.md)
 - [Rust](languages/rust/README.md)
 - [Python](languages/python/README.md)
 - [Flutter modules](languages/dart/mods.md) - how to write a graphical module
   using Flutter
 - [New language](languages/new/README.md) - how to bring a new language to Fuchsia
-->

## 语言
 - [说明](languages/README.md) - Fuchsia支持的编程语言
 - [C/C++](languages/c-cpp/README.md)
 - [Dart](languages/dart/README.md)
 - [FIDL](languages/fidl/README.md)
 - [Go](languages/go/README.md)
 - [Rust](languages/rust/README.md)
 - [Python](languages/python/README.md)
 - [Flutter 模块](languages/dart/mods.md) - 如何使用Flutter编写图形化模块
 - [其他语言](languages/new/README.md) - 如何使Fuchsia支持其他新语言

<!--
## API

 - [README](/docs/concepts/api/README.md) - Developing APIs for Fuchsia
 - [API Council](/docs/contribute/governance/api_council.md) - Definition of the API council
 - [System](/docs/concepts/api/system.md) - Rubric for designing the Zircon System Interface
 - [FIDL API][fidl-api] - Rubric for designing FIDL protocols
 - [FIDL style][fidl-style] - FIDL style rubric
 - [C](/docs/concepts/api/c.md) - Rubric for designing C library interfaces
 - [Tools](/docs/concepts/api/tools.md) - Rubrics for designing developer tools
 - [Devices](/docs/concepts/api/device_interfaces.md) - Rubric for designing device interfaces
-->
## API

 - [说明](/docs/concepts/api/README.md) - 为Fuchsia开发各种API
 - [API 约定](/docs/contribute/governance/api_council.md) - API 约定的定义
 - [系统](/docs/concepts/api/system.md) - Zircon系统接口设计准则
 - [FIDL API][fidl-api] - FIDL协议设计准则
 - [FIDL风格][fidl-style] - FIDL风格准则
 - [C](/docs/concepts/api/c.md) - C库接口设计准则
 - [工具](/docs/concepts/api/tools.md) - 开发工具设计准则
 - [设备](/docs/concepts/api/device_interfaces.md) - 设备接口设计准则

<!--
## ABI

 - [System](/docs/concepts/system/abi/system.md) - Describes scope of the binary-stable Fuchsia System Interface
-->

## ABI

 - [系统](/docs/concepts/system/abi/system.md) - 二进制类型的 Fuchsia 系统接口

<!--
## SDK

 - [SDK](idk/README.md) - information about developing the Fuchsia SDK
-->
## SDK

 - [SDK](idk/README.md) - 关于 Fuchsia SDK 开发

<!--
## Hardware

This section covers Fuchsia development hardware targets.

 - [Acer Switch Alpha 12][acer_12]
 - [Intel NUC][intel-nuc]
 - [Pixelbook][pixelbook]
 - [Toulouse][toulouse]
 - [Khadas VIM2][khadas-vim]
 - [iMX8M EVK][imx8mevk]
-->
## 硬件

本节涵盖 Fuchsia 开发的硬件对象

 - [Acer Switch Alpha 12][acer_12]
 - [Intel NUC][intel-nuc]
 - [Pixelbook][pixelbook]
 - [Toulouse][toulouse]
 - [Khadas VIM2][khadas-vim]
 - [iMX8M EVK][imx8mevk]

<!--
## Drivers

This section covers developing drivers on Fuchsia.

 - [Getting started][drivers-start]
-->
## 驱动

本节描述在Fuchsia上开发驱动。

 - [入门][drivers-start]
 
 <!--
## Testing

 - [Debugging workflow](/docs/development/debugging/debugging.md)
 - [Fuzz testing with LibFuzzer](/docs/development/testing/fuzzing/overview.md)
 - [Test components](/docs/concepts/testing/v1_test_component.md)
 - [Test environments](/docs/concepts/testing/environments.md)
 - [Testability rubrics](/docs/concepts/testing/testability_rubric.md)
 - [Test flake policy](/docs/concepts/testing/test_flake_policy.md)
 - [Testing Isolated Cache Storage](/docs/concepts/testing/testing_isolated_cache_storage.md)
 - [Host-target interaction tests](/docs/development/testing/host_target_interaction_tests.md)
 - [Testing for Flakiness in CQ](/docs/development/testing/testing_for_flakiness_in_cq.md)
-->
## 测试

 - [调试流程](/docs/development/debugging/debugging.md)
 - [用LibFuzzer进行模糊测试](/docs/development/testing/fuzzing/overview.md)
 - [测试组件](/docs/concepts/testing/v1_test_component.md)
 - [测试环境](/docs/concepts/testing/environments.md)
 - [可测性准则](/docs/concepts/testing/testability_rubric.md)
 - [小范围测试策略](/docs/concepts/testing/test_flake_policy.md)
 - [测试隔离缓存](/docs/concepts/testing/testing_isolated_cache_storage.md)
 - [测试主机-目标交互](/docs/development/testing/host_target_interaction_tests.md)
 - [CQ中的小范围测试](/docs/development/testing/testing_for_flakiness_in_cq.md)

<!--
## Conventions

This section covers Fuchsia-wide conventions and best practices.

 - [Documentation standards](/docs/contribute/docs/documentation-standards.md)
 - [Endian policy](/docs/development/languages/endian.md)
-->

## 约定

本节描述整个Fuchsia层面的约定及最佳实践。

 - [文档标准](/docs/contribute/docs/documentation-standards.md)
 - [端序策略](/docs/development/languages/endian.md)

<!--
## Tracing

 - [Fuchsia tracing system](/docs/concepts/tracing/README.md)
 - [Tracing guides](/docs/development/tracing/README.md)
-->

## 追踪

 - [Fuchsia追踪系统](/docs/concepts/tracing/README.md)
 - [追踪指导](/docs/development/tracing/README.md)

<!--
## Internationalization

 - [Internationalization, localization and input methods](internationalization/README.md)
-->
## 国际化

 - [国际化, 本地化及输入方法](internationalization/README.md)

<!--
## Miscellaneous

 - [CTU analysis in Zircon](kernel/ctu_analysis.md)
 - [Packet capture](debugging/packet_capture.md)
 - [Editor configurations](/docs/development/editors/README.md)
 - [Using the Fuchsia Installer](/docs/development/hardware/installer.md)
 - [Enable verbose logging for input events](/docs/development/components/v1/verbose_logging.md)
-->
## 其他

 - [Zircon中的CTU分析](kernel/ctu_analysis.md)
 - [抓包](debugging/packet_capture.md)
 - [编辑器配置](/docs/development/editors/README.md)
 - [Fuchsia安装器的使用](/docs/development/hardware/installer.md)
 - [为输入事件开启详细日志](/docs/development/components/v1/verbose_logging.md)

[acer_12]: /docs/development/hardware/acer12.md "Acer 12"
[pixelbook]: /docs/development/hardware/pixelbook.md "Pixelbook"
[toulouse]: /docs/development/hardware/toulouse.md "Toulouse"
[khadas-vim]: /docs/development/hardware/khadas-vim.md "Khadas VIM2"
[imx8mevk]: /docs/development/hardware/imx8mevk.md "iMX8M EVK"
[intel-nuc]: /docs/development/hardware/intel_nuc.md "Intel NUC"
[fidl-style]: /docs/development/languages/fidl/guides/style.md
[fidl-api]: /docs/concepts/api/fidl.md
[drivers-start]: /docs/development/drivers/developer_guide/driver-development.md
