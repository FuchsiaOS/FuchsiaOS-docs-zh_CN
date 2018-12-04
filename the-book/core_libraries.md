<!--
Fuchsia Core Libraries
======================

This document describes the core libraries in the Fuchsia system, starting from
the bottom of the dependency chain. -->

Fuchsia 核心库
======================

本文档从依赖链底层描述 Fuchsia 系统的核心库。

<!--
# Zircon libraries

## libzircon

This library defines the Zircon system ABI.

TODO(kulakowski) Talk about how this is not quite the kernel
syscall interface, since the VDSO abstracts that. -->

<!-- # Zircon libraries -->
# Zircon 库

<!--
## libzircon

This library defines the Zircon system ABI.

TODO(kulakowski) Talk about how this is not quite the kernel
syscall interface, since the VDSO abstracts that. -->

## libzircon

该库定义了 Zircon 系统 ABI。

TODO(kulakowski) 描述它不是内核系统调用接口，因为 VDSO 抽象了它。

<!-- ## libzx

libzircon defines C types and function calls acting on those
objects. libzx is a light C++ wrapper around those. It adds type
safety beyond `zx_handle_t`, so that every kernel object type has a
corresponding C++ type, and adds ownership semantics to those
handles. It otherwise takes no opinions around naming or policy.

For more information about libzx, see
[its documentation](https://fuchsia.googlesource.com/zircon/+/master/system/ulib/zx/README.md). -->

## libzx

libzircon 定义了 C types 和这些对象的函数调用。libzx 是一个轻量级的 C++ 包装器。 It adds type safety beyond `zx_handle_t`, 因此每个内核对象类型都具有相应的 C++ 类型，并将 ownership semantics 添加到句柄。否则，它将不采取命名和策略。

有关libzx的更多信息，请参阅[libzx 文档](https://fuchsia.googlesource.com/zircon/+/master/system/ulib/zx/README.md)。

<!--
## FBL

Much of Zircon is written in C++, both in kernel and in
userspace. Linking against the C++ standard library is not especially
well suited to this environment (it is too easy to allocate, throw
exceptions, etc., and the library itself is large). There are a number
of useful constructs in the standard library that we would wish to use,
like type traits and unique pointers. However, C++ standard libraries
are not really to be consumed piecemeal like this. So we built a
library which provides similar constructs named fbl. This library
also includes constructs not present in the standard library but which
are useful library code for kernel and device driver environments (for
instance, slab allocation).

For more information about FBL,
[read its overview](https://fuchsia.googlesource.com/zircon/+/master/docs/cxx.md#fbl). -->

## FBL

Zirco n大部分都是用 C++ 编写的, 包括内核和用户空间。链接 C++ 标准库并不是太适合这种环境 (它太容易分配、抛出异常等，而且库本身很大)。 我们希望使用标准库中许多有用的结构,比如 type traits 和指针。但是，C++ 标准库并不能真的像这样零碎地使用。所以我们创建了一个提供类似 FBL 结构的库。这个库还包括标准库中没有的结构，但是对于内核和设备驱动程序环境是使用的 useful 库代码(例如，slab 分配)。

有关 FBL 的更多信息，请[阅读文档](https://fuchsia.googlesource.com/zircon/+/master/docs/cxx.md#fbl)。

<!--
# FXL

FXL is a platform-independent library containing basic C++ building blocks, such
as logging and reference counting. FXL depends on the C++ standard library but
not on any Zircon- or Fuchsia-specific libraries. We build FXL both for target
(Fuchsia) and for host (Linux, Mac) systems.

Generally speaking, we try to use the C++ standard library for basic building
blocks, but in some cases the C++ standard library either doesn't have something
we need (e.g., a featureful logging system) or has a version of what we need
doesn't meet our requirements (e.g., `std::shared_ptr` is twice as large as
`fxl::RefPtr`). -->

# FXL

FXL 是一个包含基本的 C++ 构建块的独立于平台的库，例如日记记录(logging)和引用计数( reference count )。FXL依赖于C++ 标准库,但不依赖于任何 Zircon 或 Fuchsia 特定的库。我们为目标（Fuchsia）和主机（Linux，Mac）系统构建 FXL。

一般来说，我们尝试将 C++ 标准库用于基本构建块，但在某些情况下，C++ 标准库要么没有我们需要的东西（例如，一个功能强大的日志系统），要么有我们需要的版本但是不符合我们的要求（如，`std::shared_ptr` 是`fxl::RefPtr`的两倍大）。
