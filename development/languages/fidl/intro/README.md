# FIDL:概述

本文档的目的是用来描述Fuchsia接口定义语言（FIDL）的高层目标与需求。

## 相关文档

*   [有限格式规范]
*   [语言规范]
*   [编译规范]
*   [API 可读性 / 样式指南]
*   [绑定C语言]
*   [绑定C++语言]
*   [示例]
*   [教程]

<!-- Reference links because these are used again below. -->

[有限格式规范]: ../reference/wire-format/index.md
[语言规范]: ../reference/language.md
[编译规范]: ../reference/compiler.md
[API 可读性 / 样式指南]: ../../../api/fidl.md
[绑定C语言]: ../languages/c.md
[绑定C++语言]: ../languages/cpp.md
[示例]: https://fuchsia.googlesource.com/zircon/+/master/system/host/fidl/examples
[教程]: ../tutorial/README.md

[TOC]

## 概述

Fuchsia接口定义语言（FIDL）是用来描述Fuchsia操作系统中进程间通信协议（IPC）的语言。FIDL的工具链（编译器）和运行时支持库(绑定)用于帮助开发者高效的使用IPC。

## 目标

由于Fuchsia是微内核操作系统，其中大部分的功能在用户空间中实现，包括设备驱动等特权组件，所以它大量地依赖IPC进行通信。因此，IPC机制在设计上必须具有高效性、确定性、稳健性和易用性。

**IPC的高效性**衡量生成、传输和处理进程间消息所需的计算开销。IPC将参与所有的系统操作，所以它必须高效。FIDL的编译器必须生成紧凑的代码，没有额外的间接跳转或者隐形开销。最重要的是，它应该至少要你特定优化的代码一样好。

**IPC的确定性**衡量在已知的封装资源大小的执行事务能力。IPC将被广泛的用于关键系统服务，例如，服务于许多客户端的文件系统，必须按照可预测的方式进行工作。FIDL的有线格式必须对确保结构体大小与布局的不变性提供强静态保证，从而减轻对动态内存分配或复杂验证规则的需求。

**IPC的鲁棒性**衡量考虑IPC作为操作系统ABI的重要组成部分的需要。保持二进制的稳定性至关重要，协议演变的机制必须谨慎设计，以便使现在的服务与他们的客户端不违反不变性，特别是在确定性的需求也被考虑其中时。FIDL的绑定也必须高效，轻量并且经过严格验证。

**IPC的易用性**衡量IPC协议作为操作系统API的重要组成部分，为通过IPC访问服务提供好的开发者使用方法是很重要的。FIDL的代码生成器减轻了手工编写IPC绑定代码的负担。此外，FIDL的代码生成器可以提供不同的绑定来适应不同开发者以及他们的习惯。

TODO: 解释为满足不同受众使用合适的定制化绑定的目标是什么，例如，本地系统编程 vs.事件驱动调度 vs. 异步调用等... 以及关于更多FIDL的介绍，例如系统API，SDK的关注点等。

## 需求

# 目的

*   描述用于Zircon上使用的IPC协议的数据结构与接口。
*   针对特定于进程间通信的优化; FIDL不能用于磁盘上的持久化操作或者跨设备的网络传输。
*   同一设备上进程间的有效传输消息由数据(字节)与Zircon中处理通道的能力组成。
*   专为促进Zircon原语的有效使用而设计；不打算在其它平台上使用，并且不可移植。
*   为创建、发送、接收与消费消息提供方便的API。
*   执行足够的验证来维护协议不变性（并仅是如此而已）。

# 性能

*   Just as efficient (speed and memory) as using hand-rolled data structures
    would be.
*   Wire format uses uncompressed native datatypes with host-endianness and
    correct alignment to support in-place access of message contents.
*   No dynamic memory allocation is required to produce or to consume messages
    when their size is statically known or bounded.
*   Explicitly handle ownership with move-only semantics.
*   Data structure packing order is canonical, unambiguous, and has minimum
    padding.
*   Avoid back-patching pointers.
*   Avoid expensive validation.
*   Avoid calculations which may overflow.
*   Leverage pipelining of interface requests for asynchronous operation.
*   Structures are fixed size; variable-size data is stored out-of-line.
*   Structures are not self-described; FIDL files describe their contents.
*   No versioning of structures, but interfaces can be extended with new methods
    for protocol evolution.

# Ergonomics

*   Programming language bindings maintained by Fuchsia team:
    *   C, C++ (native), C++ (idiomatic), Dart, Go, Rust
*   Keeping in mind we might want to support other languages in the future, such
    as:
    *   Java, Javascript, etc.
*   The bindings and generated code are available in native or idiomatic flavors
    depending on the intended application.
*   Use compile-time code generation to optimize message serialization,
    deserialization, and validation.
*   FIDL syntax is familiar, easily accessible, and programming language
    agnostic.
*   FIDL provides a library system to simplify deployment and use by other
    developers.
*   FIDL expresses the most common data types needed for system APIs; it does
    not seek to provide a comprehensive one-to-one mapping of all types offered
    by all programming languages.

# Implementation

*   Compiler is written in C++ to be usable by components built in Zircon.

*   Compiler is portable and can be built with a host toolchain.

*   We will not support FIDL bindings for any platform other than Fuchsia.

## Where to Find the Code

- [The compiler](../../system/host/fidl)
- [C bindings](../../system/ulib/fidl)
- [C++ bindings](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/cpp)
- [Go bindings](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/go)
- [Rust bindings](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/rust)

## Constituent Parts of Specification

### FIDL Wire Format

The FIDL wire format specified how FIDL messages are represented in memory for
transmission over IPC.

The FIDL wire format is documented [Wire Format Specification].

### FIDL Language

The FIDL language is the syntax by which interfaces are described in ***.fidl**
files.

The FIDL language is documented [Language Specification].

### FIDL Compiler

The FIDL compiler generates code for programs to use and implement interfaces
described by the FIDL language.

The FIDL compiler is documented [Compiler Specification].

### FIDL Bindings

FIDL bindings are language-specific runtime support libraries and code
generators which provide APIs for manipulating FIDL data structures and
interfaces.

Languages-specific topics:

*   [C Language Bindings]
*   [C++ Language Bindings]

Bindings are available in various flavors depending on the language:

*   **Native bindings**: designed for highly sensitive contexts such as device
    drivers and high-throughput servers, leverage in-place access, avoid memory
    allocation, but may require somewhat more awareness of the constraints of
    the protocol on the part of the developer.
*   **Idiomatic bindings**: designed to be more developer-friendly by copying
    data from the wire format into easier to use data types (such as heap-backed
    strings or vectors), but correspondingly somewhat less efficient as a
    result.

Bindings offer several various ways of invoking interface methods depending on
the language:

*   **Send/receive**: read or write messages directly to a channel, no built-in
    wait loop (C)
*   **Callback-based**: received messages are dispatched asynchronously as
    callbacks on an event loop (C++, Dart)
*   **Port-based**: received messages are delivered to a port or future (Rust)
*   **Synchronous call**: waits for reply and return it (Go, C++ unit tests)

Bindings provide some or all of the following principal operations:

*   **Encode**: in-place transform native data structures into the wire format
    (coupled with validation)
*   **Decode**: in-place transform wire format data into native data structures
    (coupled with validation)
*   **Copy/Move To Idiomatic Form**: copy contents of native data structures
    into idiomatic data structures, handles are moved
*   **Copy/Move To Native Form**: copy contents of idiomatic data structures
    into native data structures, handles are moved
*   **Clone**: copy native or idiomatic data structures (that do not contain
    move-only types)
*   **Call**: invoke interface method

## Workflow

This section describes the workflow of authors, publishers, and consumers of IPC
protocols described using FIDL.

# Authoring FIDL

The author of a FIDL based protocol creates one or more ***.fidl files** to
describe their data structures and interfaces.

FIDL files are grouped into one or more **FIDL libraries** by the author. Each
library represents a group of logically related functionality with a unique
library name. FIDL files within the same library implicitly have access to all
other declarations within the same library. The order of declarations within the
FIDL files that make up a library is not significant.

FIDL files of one library can access declarations within another FIDL library by
**importing** the other FIDL module. Importing other FIDL libraries makes their
symbols available for use thereby enabling the construction of protocols derived
from them. Imported symbols must be qualified by the library name or by an alias
to prevent namespace collisions.

# Publishing FIDL

The publisher of a FIDL based protocol is responsible for making FIDL libraries
available to consumers. For example, the author may disseminate FIDL libraries in
a public source repository or distribute them as part of an SDK.

Consumers need only point the FIDL compiler at the directory which contains the
FIDL files for a library (and its dependencies) to generate code for that
library. The precise details for how this is done will generally be addressed by
the consumer's build system.

# Consuming FIDL

The consumer of a FIDL based protocol uses the FIDL compiler to generate code
suitable for use with their language runtime specific bindings. For certain
language runtimes, the consumer may have a choice of a few different flavors of
generated code all of which are interoperable at the wire format level but
perhaps not at the source level.

In the Fuchsia world build environment, generating code from FIDL libraries will
be done automatically for all relevant languages by individual FIDL build
targets for each library.

In the Fuchsia SDK environment, generating code from FIDL libraries will be done
as part of compiling the applications which use them.
