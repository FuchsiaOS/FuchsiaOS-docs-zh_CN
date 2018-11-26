<!-- Overview -->
# FIDL 概述

<!--
This document is a description of the Fuchsia Interface Definition Language (FIDL) purpose, high-level goals, and requirements.
-->
本文档描述了Fuchsia接口定义语言（FIDL）的目的，高级目标和要求。

<!-- ## Related Documents -->
## 相关文档

*   [有线格式规范]
*   [语言规范]
*   [编译器规范]
*   [API 可读性 / 样式指南]
*   [C 语言绑定]
*   [C++ 语言绑定]
*   [示例]
*   [教程]

<!-- Reference links because these are used again below. -->

[有线格式规范]: ../reference/wire-format/index.md
[语言规范]: ../reference/language.md
[编译器规范]: ../reference/compiler.md
[API 可读性 / 样式指南]: ../../../api/fidl.md
[C 语言绑定]: ../languages/c.md
[C++ 语言绑定]: ../languages/cpp.md
[示例]: https://fuchsia.googlesource.com/zircon/+/master/system/host/fidl/examples
[教程]: ../tutorial/README.md

[TOC]

## 概述

Fuchsia 接口定义语言（FIDL）是用来描述Fuchsia操作系统中进程间通信协议（IPC）的语言。FIDL 的工具链（编译器）和运行时支持库(绑定)用于帮助开发者高效的使用 IPC。

## 目标

由于 Fuchsia 是微内核操作系统，其中大部分的功能在用户空间中实现，包括设备驱动等特权组件，所以它大量地依赖 IPC 进行通信。因此，IPC 机制在设计上必须具有高效性、确定性、鲁棒性和易用性。

**IPC的高效性** 衡量生成、传输和处理进程间消息所需的计算开销。IPC  将参与所有的系统操作，所以它必须高效。FIDL 的编译器必须生成紧凑的代码，没有额外的间接跳转或者隐形开销。最重要的是，它应该至少要你特定优化的代码一样好。

**IPC的确定性** 衡量在已知的封装资源大小的执行事务能力。IPC 将被广泛的用于关键系统服务，例如，服务于许多客户端的文件系统，必须按照可预测的方式进行工作。FIDL 的有线格式必须对确保结构体大小与布局的不变性提供强静态保证，从而减轻对动态内存分配或复杂验证规则的需求。

**IPC的鲁棒性** 衡量考虑 IPC 作为操作系统 ABI 的重要组成部分的需要。保持二进制的稳定性至关重要，协议演变的机制必须谨慎设计，以便使现在的服务与他们的客户端不违反不变性，特别是在确定性的需求也被考虑其中时。FIDL 的绑定也必须高效，轻量并且经过严格验证。

<!--
**IPC ease of use** pertains to the fact that IPC protocols are an essential part of the operating system's API. It is important to provide good developer ergonomics for accessing services via IPC. The FIDL code generator removes the burden of writing IPC bindings by hand. Moreover, the FIDL code generator can produce different bindings to suit the needs of different audiences and their idioms.
-->

**IPC的易用性** 衡量涉及 IPC 协议作为操作系统 API 的重要组成部分的事实，为通过 IPC 访问服务提供良好的开发者使用方法是很重要的。FIDL 的代码生成器减轻了手工编写 IPC 绑定代码的负担。此外，FIDL 的代码生成器可以提供不同的绑定来适应不同开发者以及他们的习惯。

TODO: 解释为满足不同受众使用合适的定制化绑定的目标是什么，例如，本地系统编程 vs.事件驱动调度 vs. 异步调用等... 以及关于更多 FIDL 的介绍，例如系统 API，SDK 的关注点等。

## 需求

# 目的

<!--
*   Describe data structures and interfaces used by IPC protocols on Zircon.
*   Optimized for interprocess communication only; FIDL must not be persisted to
    disk or used for network transfer across device boundaries.
*   Efficiently transport messages consisting of data (bytes) and capabilities
    (handles) over Zircon channels between processes running on the same
    device.
*   Designed specifically to facilitate effective use of Zircon primitives; not
    intended for use on other platforms; not portable.
*   Offers convenient APIs for creating, sending, receiving, and consuming
    messages.
*   Perform sufficient validation to maintain protocol invariants (but no more
    than that).
-->

*   描述用于 Zircon 上使用的 IPC 协议的数据结构与接口。
*   针对特定于进程间通信的优化; FIDL 不能用于磁盘上的持久化操作或者跨设备的网络传输。
*   在同一设备上运行的进程间通过 Ziron 通道高效传输由数据（bytes）和功能（handles）组成的消息。
*   专为促进 Zircon 原语的高效使用而设计；不打算在其它平台上使用，并且不可移植。
*   提供用于创建，发送，接收和使用消息的便捷 API。
*   执行足够的验证来维护协议不变性（仅此而已）。

# 性能

*   与使用手动定义数据的方式一样（在速度与内存使用上）高效。
*   在有线格式上，使用未压缩的本地主机大小端数据类型，并纠正数据对齐来支持消息内容的原地访问。
*   如果消息大小静态已知或者有界时，无需分配动态内存以产生或消费消息。
*   利用 move-only 语义来显式处理所有权
*   数据结构打包顺序是规范的，无歧义的，并且是最小对齐的。
*   避免反向修改指针。
*   避免高性能损耗的验证操作。
*   避免可能溢出的计算。
*   将接口请求流水化实现异步操作。
*   结构体固定大小；将可变大小的数据存储在外部。
*   结构体不能自描述；FIDL 文件用于描述结构体的内容。
*   结构没有版本控制，但是接口可扩展新的方法来演化协议。

# 人类工程学

*   Fuchsia 团队维护的编程语言绑定:
    *   C, C++ (native), C++ (idiomatic), Dart, Go, Rust
*   同时留意我们希望去支持更多语言，例如:
    *   Java, Javascript, etc.
*   绑定与代码生成根据原定的用途适应原有的风格。
*   在编译时生成代码，优化消息序列，反序列化，并且验证。
*   FIDL 语法是熟悉的，易于访问的，并且编程语言不可知。
*   FIDL 提供一个库来简化其它开发者的部署与使用。
*   FIDL 表述系统 API 所需要的最常见的数据类型；它不寻求提供支持所有编程语言一对一的全面映射。

# 实现

*   编译器使用 C++ 编写，用于 Zircon 中的组件使用。

*   编译器是可移植的，并可利用宿主工具链来构建它。

*   我们不会在 Fuchsia 以外的平台支持 FIDL 绑定。

## 代码位置

- [编译器](../../system/host/fidl)
- [C 绑定](../../system/ulib/fidl)
- [C++ 绑定](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/cpp)
- [Go 绑定](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/go)
- [Rust 绑定](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/rust)

## 规范的组成部分

### FIDL 有线格式

FIDL 有线格式规范了 FIDL 消息是如何在内存中表示以支持IPC传输。

有线格式的文档 [有线格式规范].

### FIDL 语言

FIDL语言是**.fidl**文件来描述接口的语法。

FIDL语言的文档 [语言规范].

### FIDL 编译器

FIDL编译器的功能是为使用与实现FIDL语言描述的接口的程序生成代码。

FIDL编译器的文档 [编译器规范].

### FIDL 绑定

FIDL绑定是语言特定的运行时支持库与代码生成器，以提供用于操作FIDL数据结构和接口的API。

语言规范相关的主题:

*   [C 语言绑定]
*   [C++ 语言绑定]

<!--
Bindings are available in various flavors depending on the language:

*   **Native bindings**: designed for highly sensitive contexts such as device
    drivers and high-throughput servers, leverage in-place access, avoid memory
    allocation, but may require somewhat more awareness of the constraints of
    the protocol on the part of the developer.
*   **Idiomatic bindings**: designed to be more developer-friendly by copying
    data from the wire format into easier to use data types (such as heap-backed
    strings or vectors), but correspondingly somewhat less efficient as a
    result.
-->

绑定的风格根据语言而有所不同:

*   **本地绑定**: 为高度敏感的上下文，如设备驱动与高吞吐量的服务器而设计，充分利用原地访问，避免内存分配，但是需要开发者了解更多关于协议上的限制。
*   **惯用绑定**: 通过将数据从有线格式拷贝到易于使用的数据类型上（例如堆上字符串或者向量），旨在使开发人员更加友好，但是这种方式对应的效率较低。

绑定提供了根据不同语言调用接口的多种方法:

*   **发送/接收**: 直接通过 channel 读写消息，无内置等待循环（C）
*   **基于回调**: 异步分配的方式接收消息，并作为事件循环的回调（C++，Dart）
*   **基于端口**: 接收的消息传递到 port 或 future 上（Rust）
*   **同步调用**: 等待应答后返回结果（Go，C++ 单元测试）

绑定提供以下主要操作的部分或全部:

*   **编码**: 将本地数据结构原地转换为有线格式（与验证相结合）
*   **解码**: 将有线格式原地转换为本地数据结构（与验证相结合）
*   **复制/移动到惯用形式**: 本地数据结构的内容复制为惯用数据结构中，同时移除handle
*   **复制/移动到本机格式**: 把惯用数据结构的内容复制为本地数据结构中，同时移除 handle
*   **克隆**: 复制本地或惯用数据结构 (不包含只可移动的类型)
*   **调用**: 调用接口方法

## 工作流

本章节介绍了使用 FIDL 描述 IPC 协议的作者，发布者，使用者的工作流。

# 创作 FIDL

基于 FIDL 协议的作者创建一个或多个**.fidl**文件来描述他们的数据结构与接口。

作者将FIDL文件分组为一个或多个**FIDL库**。每个库表示一组逻辑相关的功能，并具有唯一命名。在同一个库中的 FIDL 文件可以访问库中所有其它的声明，并且FIDL文件中的声明顺序并不重要。

一个 FIDL 库中的 FIDL 文件通过 **importing** 声明可以访问另一个库中的模块。导入其它库使得它们的符号可供使用，从而可以构建它们的派生协议。导入的符号必须通过库名或别名来限定，以防止命名空间冲突。

# 发布 FIDL

基于 FIDL 协议的发布者负责向使用者提供 FIDL 库。例如，作者可以在公共源仓库中分发 FIDL 库，或者将他们以 SDK 中的一部分的方式进行分发。

使用者只需向 FIDL 编译器指定包含 FIDL 库文件的目录（以及它们的依赖项）。这项工作的具体细节通常将由使用者的构建系统来解决。

# 使用 FIDL

基于 FIDL 的协议使用者利用FIDL编译器生成适用于他们语言运行时特定绑定的代码。对于某些语言运行时，使用者可以选择几种不同风格来生成代码，他们可以有线格式上互操作，但可能在源码级别上可能不行。

在 Fuchsia 的构建环境中，从 FIDL 库生成代码将自动为每个相关的语言通过每个 FIDL 构建目标自动完成。

在 Fuchsia 的 SDK 环境中，从 FIDL 库中生成代码的过程将作为编译使用他们的应用程序的构建过程一部分。
