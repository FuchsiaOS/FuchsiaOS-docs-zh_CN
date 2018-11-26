Fuchsia 核心库
======================

本文档从依赖链底层描述Fuchsia系统的核心库。

# Zircon 库

## libzircon

该库定义了Zircon系统ABI。

TODO(kulakowski)描述它不是内核系统调用接口，因为VDSO抽象了它。

## libzx

libzircon定义了C types和这些对象的函数调用。libzx是一个轻量级的C++包装器. It adds type safety beyond `zx_handle_t`, 因此每个内核对象类型都具有相应的C ++类型，并将ownership semantics添加到句柄。否则，它将不采取命名和策略。

有关libzx的更多信息，请参阅
[libzx 文档](https://fuchsia.googlesource.com/zircon/+/master/system/ulib/zx/README.md).

## FBL

Zircon大部分都是用C ++编写的, 包括内核和用户空间。链接C ++标准库并不是太适合这种环境 (它太容易分配、抛出异常等，而且库本身很大). 我们希望使用标准库中许多有用的结构,比如type traits和指针。但是，C ++标准库并不能真的像这样零碎地使用。所以我们创建了一个提供类似FBL结构的库。这个库还包括标准库中没有的结构，但是对于内核和设备驱动程序环境是使用的useful库代码(例如，slab分配)。

有关FBL的更多信息，请
[阅读文档](https://fuchsia.googlesource.com/zircon/+/master/docs/cxx.md#fbl).

# FXL

FXL是一个包含基本的C++构建块的独立于平台的库，例如日记记录(logging)和引用计数(reference count)。FXL依赖于C ++标准库,但不依赖于任何Zircon或Fuchsia特定的库。我们为目标（Fuchsia）和主机（Linux，Mac）系统构建FXL。

一般来说，我们尝试将C++标准库用于基本构建块，但在某些情况下，C++标准库要么没有我们需要的东西（例如，一个功能强大的日志记录系统），要么有我们需要的版本但是不符合我们的要求（如，`std::shared_ptr` 是`fxl::RefPtr`的两倍大）。
