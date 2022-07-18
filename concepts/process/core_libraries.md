<!-- Fuchsia Core Libraries


This document describes the core libraries in the Fuchsia system, starting from
the bottom of the dependency chain. -->

Fuchsia 核心库
======================

这部分文档从依赖链的最底端开始，自底向上地对 Fuchsia 系统的核心库进行描述。

<!-- # Zircon libraries

## libzircon

This library defines the Zircon system ABI.

TODO(kulakowski) Talk about how this is not quite the kernel
syscall interface, since the VDSO abstracts that. -->

# Zircon 库

## libzircon

这个库定义了 Zircon 的系统 ABI。  
由于 VDSO 对其进行了抽象，因此 libzircon 并不是系统调用接口。

<!-- ## libzx

libzircon defines C types and function calls acting on those
objects. libzx is a light C++ wrapper around those. It adds type
safety beyond `zx_handle_t`, so that every kernel object type has a
corresponding C++ type, and adds ownership semantics to those
handles. It otherwise takes no opinions around naming or policy.

For more information about libzx, see
[its documentation](/zircon/system/ulib/zx/README.md). -->

## libzx

libzircon 中定义了 C 语言类型以及相关的函数调用。libzx 则使用 C++ 对 libzircon 进行了包装。它在`zx_handle_t`之外增加了类型安全，因此每个内核对象类型都有一个与之相关的 C++　类型，并且为这些句柄添加了所有权语义。此外，它不在命名和策略上做任何改变。

更多关于 libzx 的信息，参见[其文档](/zircon/system/ulib/zx/README.md)。

<!-- ## FBL

Much of Zircon is written in C++, both in kernel and in
userspace. Linking against the C++ standard library is not especially
well suited to this environment (it is too easy to allocate, throw
exceptions, etc., and the library itself is large). There are a number
of useful constructs in the standard library that we would wish to use,
like type traits and unique pointers. However, C++ standard libraries
are not really to be consumed piecemeal like this. So we built a
library that provides similar constructs named fbl. This library
also includes constructs not present in the standard library but
are useful library code for kernel and device driver environments (for
instance, slab allocation).

For more information about FBL,
[read its overview](/docs/development/languages/c-cpp/cxx.md#fbl). -->

## FBL

不论是在内核空间还是用户空间，Zircon 大部分都是使用 C++ 编写的。在这种环境下链接 C++ 标准库并不是非常合适（太容易出现内存的不当分配、经常出现异常、标准库本身过于庞大等等）。在标准库中有相当大一部分我们想要使用的内容，例如 `type_traits` 和 `unique_ptr`。然而，我们不想因为这些细微的理由而选择忍受标准库的缺点。因此，我们自己构建了一个与标准库具有相似功能的库，称为 fdl。这个库也包含了一些标准库中没有，但是对内核和设备驱动环境有帮助的功能，例如 Slab 内存分配。

更多关于 FBL 的信息，参见[其文档](/docs/development/languages/c-cpp/cxx.md#fbl)

<!-- # FXL

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

FXL 是一个包含基础 C++ 编译模块（例如日志和引用计数）的平台无关库。FXL 只依赖于 C++ 标准库，不包含任何与 Zircon 或 Fuchsia 相关的库。
我们在目标系统（Fuchsia）和主机系统（Linux，Mac）上同时构建了 FXL。

一般来说，我们尝试过使用 C++ 标准库来构造基础模块，但是在某些情况下，C++ 标准库没有我们所需的功能（如，功能强大的日志系统）或者没有符合我们需求的合适版本（例如，`std::shared_ptr`是 `fxl::RefPtr` 的两倍大小）。