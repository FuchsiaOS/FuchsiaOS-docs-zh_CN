# Zircon 中的 C++

Zircon 树中使用了 C++ 17 语言的子集。其中包括内核代码和用户空间代码。在这两处，C++ 和 C（以及一些汇编）混合在一起。避免或禁止某些 C++ 语言特性。谨慎使用 C++ 标准库功能。

## 语言特性

- 不允许
  - 异常
  - RTTI 和 `dynamic_cast`
  - 运算符重载
  - 虚继承
  - 静态构造对象
  - 尾随返回类型语法
    - 例外：如果lambda需要其他无法表达的返回类型
  - 初始化列表
  - 内核代码中的 `thread_local`
- 允许
  - 纯接口继承
  - Lambdas
  - `constexpr`
  - `nullptr`
  - `enum class`
  - `template`
  - 默认参数
    - 但要使用判断。末尾有一个可选的 out 参数可接受。四个可选的布尔参数，不可接受。
  - Plain old 类型
  - `auto`
  - 多重继承
    - 但是要谨慎。它广泛用于诸如侵入式容器 mixins。
- 需要更多规则 TODO（cpu）
  - 全局构造函数
    - 目前，为全局数据结构提供了这些特性。

**TODO:** 指向风格指南？

## C++标准版（17 vs 14）

Zircon 代码是用 `-std=c++17` 构建的，通常可以自由使用 C++ 17 语言和库特性（受[上述](#语言特性)风格/特性约束和[下述](#标准库))库使用指南的约束）。保持与 C++ 14 或更早版本的兼容性并不是一个普遍关心的问题。当标准的 C++ 17 特性足以完成某事时，就使用 C++ 17。

但是， 发布到 IDK 的任何库都必须与以 C++ 14 和 C++ 17 模式构建的 IDK 用户兼容。因此，任何导出到 IDK 的库都必须具有与 `-std=c++14` 和 `-std=c++17` 兼容的公共头文件。如果库作为源码而不是二进制文件导出到 IDK，则其源码也必须与 `-std=c++14` 和
`-std=c++17` 完全兼容(不需要其他特殊选项)。**TODO(mcgrathr):** _指向有关维护要导出到 IDK 的代码的构建系统文档_

所有纯 C 代码（`.c` 源文件及其使用的头文件）都是 C 11。对于要由树外引导加载程序重用的代码，有一些特殊的例外情况，这些加载程序坚持使用保守的 C 89 子集来嵌入代码。

## 标准库

C++ 标准库 API 有许多特性迥异的接口。根据每个特定接口的代码生成以及使用机器和操作系统设备的可预测性和复杂性，标准库 API 可以细分为以下几个类别。它们可以看作是将 API 的同心圆从最小的类 C 子集扩展到完整的 C++ 17 API。

#### 上下文事项

本部分提供了有关如何考虑使用特定标准 C++ 库 API 对整个系统的影响的指南。除了内核（见下一部分）和实现约束（人们总是希望这些约束应该是暂时的），没有硬性的规则。

最重要的原则是**要谨慎**。

 * 考虑一下您对时间和空间复杂性，动态分配行为（如果有）以及使用的每个 API 的故障模式的理解程度。

 * 然后考虑使用它的特定上下文，以及该上下文对各种问题的敏感程度。

 * 尤其要警惕依赖于输入的行为，在使用非平凡的库工具时，这种行为很快就会变得难以预测。

如果在驱动程序中编写主 I/O 逻辑，或者在任何类型的系统服务中为延迟、吞吐量或可靠性而在热路径中编写任何东西，那么应该对所依赖的库工具非常保守。它们在技术上都可以在用户空间中使用（尽管在内核中要少得多；见下一部分）。但实际使用的并不多。您可能不想依靠很多在后台进行动态分配的`std` 容器。它们将使您难以理解、预测和控制服务的存储/内存占用、分配行为、性能和可靠性。

尽管如此，驱动程序也是一个用户空间程序，它会启动并解析配置文件或参数等。对于所有不属于热路径的非必需或启动时函数，使用更复杂的库工具可能会更好，这会使工作变得更容易。只需记住注意代码的总体指标，例如最小/总/峰值运行时内存使用量、代码膨胀（同时使用设备存储和运行时内存）以及对意外故障模式的恢复能力。也许不要仅仅为了利用那个奇特的配置解析库而将驱动程序的代码大小和内存占用增加一倍。

#### 内核中无 `std`

C++ `std` 命名空间不能用于[内核](/zircon/kernel)代码，也包括[bootloader](/src/firmware/gigaboot)。不涉及 `std::` API 的几个 C++ 标准库头文件仍然可以直接使用。请参阅下一部分。

在内核代码中不应使用其他 C++ 标准头文件。相反，任何值得在内核中使用的库工具（如`std::move`）都是通过内核特定的 API（如 `ktl::move`）提供的。这些 API 的内核实现实际上可能依赖于工具链头文件，这些头文件提供了别名为内核 API 名称的 `std::` 实现。但是只有那些 API 实现和某些库头文件中的非常特殊的情况才应该在内核内置的源代码中使用 `std::`。

#### 通用头文件

这些头文件 API 在任何地方都可以安全使用，甚至在内核中也可以使用。

它们包括内核支持的标准 C 接口子集上的 C++ 包装器：

 * [`<cstdarg>`](https://en.cppreference.com/w/cpp/header/cstdarg)
 * [`<cstddef>`](https://en.cppreference.com/w/cpp/header/cstddef)
 * [`<climits>`](https://en.cppreference.com/w/cpp/header/climits)
 * [`<cstdint>`](https://en.cppreference.com/w/cpp/header/cstdint)
 * [`<cinttypes>`](https://en.cppreference.com/w/cpp/header/cinttypes)
 * [`<cassert>`](https://en.cppreference.com/w/cpp/header/cassert)
 * [`<cstring>`](https://en.cppreference.com/w/cpp/header/cstring)

这些头文件中 C 库 API 的 `std` 命名空间别名不应在内核代码中使用。

即使在内核中，也可以使用一个纯 C++ 头文件：

 * [`<new>`](https://en.cppreference.com/w/cpp/header/new)

   普通非放置 `operator new` 和 `operator new[]` 在内核中不可用。请改用 [`fbl::AllocChecker`
   `new`](/zircon/system/ulib/fbl/include/fbl/alloc_checker.h)。

#### 保守用户空间

这些头文件 API 可以在任何地方安全使用。不允许在内核中使用它们，因为它们都完全在 `std` 命名空间中。但是，如果有很好的理由在内核代码中使用这样的 API，则这些 API 的子集很可能会获得内核内 API 别名。

这些都是纯头文件类型和模板。它们自己不做任何动态分配。每个函数的时间和空间复杂性应从其描述中清晰可见。

 * [`<algorithm>`](https://en.cppreference.com/w/cpp/header/algorithm)
 * [`<array>`](https://en.cppreference.com/w/cpp/header/array)
 * [`<atomic>`](https://en.cppreference.com/w/cpp/header/atomic)
 * [`<bitset>`](https://en.cppreference.com/w/cpp/header/bitset)
 * [`<initializer_list>`](https://en.cppreference.com/w/cpp/header/initializer_list)
 * [`<iterator>`](https://en.cppreference.com/w/cpp/header/iterator)
 * [`<limits>`](https://en.cppreference.com/w/cpp/header/limits)
 * [`<optional>`](https://en.cppreference.com/w/cpp/header/optional)
 * [`<tuple>`](https://en.cppreference.com/w/cpp/header/tuple)
 * [`<type_traits>`](https://en.cppreference.com/w/cpp/header/type_traits)
 * [`<utility>`](https://en.cppreference.com/w/cpp/header/utility)
 * [`<variant>`](https://en.cppreference.com/w/cpp/header/variant)

这些涉及一些动态分配，但仅限于显式分配：

 * [`<any>`](https://en.cppreference.com/w/cpp/header/any)
 * [`<memory>`](https://en.cppreference.com/w/cpp/header/memory)

   绝对不能使用 `std::shared_ptr`、`std::weak_ptr` 和 `std::auto_ptr` API。请改用 `std::unique_ptr` 和 `fbl::RefPtr`。

##### 仅限用户空间

这些在内核中不可用，也不可能通过内核中任何类似的 API 或名称来使用。但它们在用户空间的任何地方都是无害的。它们不涉及动态分配。

 * 浮点在内核代码中永远不可用，但可以在所有用户空间代码中使用（视性能而定）。
   * [`<cfenv>`](https://en.cppreference.com/w/cpp/header/cfenv)
   * [`<cfloat>`](https://en.cppreference.com/w/cpp/header/cfloat)
   * [`<cmath>`](https://en.cppreference.com/w/cpp/header/cmath)
   * [`<complex>`](https://en.cppreference.com/w/cpp/header/complex)
   * [`<numeric>`](https://en.cppreference.com/w/cpp/header/numeric)
   * [`<ratio>`](https://en.cppreference.com/w/cpp/header/ratio)
   * [`<valarray>`](https://en.cppreference.com/w/cpp/header/valarray)

 * 完整的 C 11 标准库，通过 C++ 包装器或标准 C `<*.h>`。
   * [`<csetjmp>`](https://en.cppreference.com/w/cpp/header/csetjmp)
   * [`<cstdlib>`](https://en.cppreference.com/w/cpp/header/cstdlib)
   * [标准 C11 接口](https://en.cppreference.com/w/c/header)

 * 同步和线程。这些标准 API 在所有用户空间代码中使用都是安全的。但是，对于类似的同步和线程，使用 Zircon 自身的库 API 可能会更好，例如 [<lib/sync/...>](/zircon/system/ulib/sync/include)。
   * [`<condition_variable>`](https://en.cppreference.com/w/cpp/header/condition_variable)
   * [`<execution>`](https://en.cppreference.com/w/cpp/header/execution)
   * [`<mutex>`](https://en.cppreference.com/w/cpp/header/mutex)
   * [`<shared_mutex>`](https://en.cppreference.com/w/cpp/header/shared_mutex)
   * [`<thread>`](https://en.cppreference.com/w/cpp/header/thread)

#### Kitchen sink

这些涉及动态分配，很难预测，而且通常无法控制。确切的运行时行为和内存需求通常很难解释。在任何关键路径中使用这些接口以获得可靠性或性能之前，或者在任何旨在精简和节省空间的组件中使用这些接口之前，请三思而后行。

 * 整个[容器库](https://en.cppreference.com/w/cpp/container)

 * [`<functional>`](https://en.cppreference.com/w/cpp/header/functional)

   请参阅 [`<lib/fit/function.h>`](/sdk/lib/fit/include/lib/fit/function.h)
   了解本地替代方法。

 * [`<memory_resource>`](https://en.cppreference.com/w/cpp/header/memory_resource)
 * [`<scoped_allocator>`](https://en.cppreference.com/w/cpp/header/scoped_allocator)

 * [`<filesystem>`](https://en.cppreference.com/w/cpp/header/filesystem)
 * [`<regex>`](https://en.cppreference.com/w/cpp/header/regex)

## FBL

FBL 是 Fuchsia Base Library（基本库），在内核和用户空间之间共享。因此，FBL 有非常严格的依赖性。例如，FBL 不能依赖 syscall 接口，因为 syscall 接口在内核中不可用。同样，FBL 不能依赖于内核中不可用的 C 库特性。

1. [system/ulib/fbl](/zircon/system/ulib/fbl)，可从内核和用户空间使用。
2. [kernel/lib/fbl](/zircon/kernel/lib/fbl)，仅在内核中可用。

注意：在适当的情况下，如果用户空间代码已迁移到使用标准 C++ 库工具，则某些与标准 C++ 库接口重叠的 FBL 接口可能会被完全删除，或者变为仅内核（并可能在内核内部重命名）。

FBL 提供：

- 实用代码
  - [基本算法](/zircon/system/ulib/fbl/include/fbl/algorithm.h)
  - [alloc 检查 new](/zircon/system/ulib/fbl/include/fbl/alloc_checker.h)
- 分配器
  - [slab 分配](/zircon/system/ulib/fbl/include/fbl/slab_allocator.h)
  - [slab malloc](/zircon/system/ulib/fbl/include/fbl/slab_malloc.h)
- 数组
  - [固定大小的数组](/zircon/system/ulib/fbl/include/fbl/array.h)
  - [固定大小的数组](/zircon/system/ulib/fbl/include/fbl/inline_array.h)，栈分配小数组
- 内联容器
  - [双链表](/zircon/system/ulib/fbl/include/fbl/intrusive_double_list.h)
  - [哈希表](/zircon/system/ulib/fbl/include/fbl/intrusive_hash_table.h)
  - [单链表](/zircon/system/ulib/fbl/include/fbl/intrusive_single_list.h)
  - [wavl 树](/zircon/system/ulib/fbl/include/fbl/intrusive_wavl_tree.h)
- 智能指针
  - [侵入式重计数混合](/zircon/system/ulib/fbl/include/fbl/ref_counted.h)
  - [侵入式重计数指针](/zircon/system/ulib/fbl/include/fbl/ref_ptr.h)
- raii 实用程序
  - [自动锁](/zircon/system/ulib/fbl/include/fbl/auto_lock.h)

FBL 对内存分配有严格的控制。内存分配应该是显式的，使用 AllocChecker 让客户端从分配失败中恢复。在某些情况下，允许隐式内存分配，但隐式分配内存的函数必须 #ifdef 后才能在内核中不可用。

FBL 在平台源代码树之外不可用。

## ZX

ZX 包含用于 Zircon [对象](/docs/reference/kernel_objects/objects.md)和 [系统调用](/docs/reference/syscalls/README.md) 的 C++ 包装器。这些包装器为句柄提供类型安全和移动语义，但除了syscalls.abigen 中的内容之外没有提供任何意见。在未来的某个时候，可能会从 syscalls.abigen 自动生成 ZX ，类似于用其他语言中自动生成系统调用包装器的方式。

ZX 是 Fuchsia SDK 的一部分。

## FZL

FZL 是 Fuchsia Zircon 库。该库为涉及内核对象的常见操作提供了增值服务，并且可以自由选择如何与 Zircon 系统调用交互。如果一段代码不依赖 Zircon 系统调用，则该代码应改为 FBL。

FZL 在平台源代码树之外不可用。

## 封闭 C++

鼓励使用 C++ 而不是 C 作为整个 Fuchsia 的实现语言。然而，在许多情况下，需要一个狭窄的 ABI 瓶颈来简化防止、跟踪或适应 ABI 漂移的问题。保持 ABI 简单的第一个关键方法是将其建立在纯 C API（可以直接从 C++ 中使用，也可以通过许多其他语言的外部函数接口）而不是 C++ API 基础上。当使用纯 C 外部 API 和 ABI 将一段代码链接到一个模块中，但在内部使用 C++ 实现时，这就称 _封闭 C++_。

 * 内核本身可以说是用封闭 C++ 实现的。
 * [vDSO](/docs/concepts/kernel/vdso.md) 是用封闭 C++ 实现的共享库。
 * Fuchsia 的标准 C 库虽然主要是用 C 语言实现的，但在其实现中也使用了封闭 C++。
 * 大多数 Fuchsia 设备驱动程序都是用封闭 C++ 实现的。

Fuchsia 的公有 SDK 中导出的二进制文件有一个硬性规定，即共享库必须具有纯 C API 和 ABI。这样的库可以而且应该在其实现中使用 C++ 而不是 C，并且它们可以将其他静态链接库与 C++ API 一起使用，只要那些内部 C++ API 的 ABI 方面不会泄漏到共享库的公有 ABI 中。

“可加载模块”（有时称为“插件”模块）与共享库非常相似。关于纯 C ABI 瓶颈的相同规则也适用于可加载模块ABI。Fuchsia 设备驱动程序就是这样的可加载模块，必须满足驱动程序（纯 C）ABI。因此，每个用 C++ 实现的驱动程序都必须使用封闭 C++。

Fuchsia C++ 工具链使用 [libc++](https://libcxx.llvm.org/) 实现提供了完整的 C++17 标准库。在 C++ 可执行文件（以及使用 C++ ABI 的共享库）中，这通常是动态链接的，这是编译器的默认行为。该工具链还通过 `-static-libstdc++` 切换到编译器（`clang++`）提供了用于封闭静态链接的 `libc++`。在 Zircon GN 编译系统中，链接目标（如`executable()`、 `test()` 或 `library()`（与 `shared = true`））使用以下行请求封闭 C++ 标准库：

```gn
    configs += [ "//zircon/public/gn/config:static-libc++" ]
```

在通过 `sdk = "shared"` 以二进制形式导出到公有 IDK 的每个 `library()` 中都需要这样做。

每个 `driver()` 自动使用封闭 C++，因此它们不需要此行。（驱动程序不能依赖于它们自己的共享库，而只能依赖于驱动程序 ABI 提供的动态链接环境。）

对于可执行文件和非导出共享库，是否使用静态链接或动态链接来判断标准 C++ 库是一种判断调用。在 Fuchsia 的包部署模型中，与在许多其他系统中一样，使用共享库没有特别的可更新性改进。主要的权衡是使用完全相同的共享库二进制文件和单个包的紧凑性（有时是性能）在使用完全相同的共享库二进制文件的系统上运行的进程与从多个存储包节省的内存和存储空间之间进行权衡。由于系统构建中的许多包都将使用相同的共享 `libc++` 库，因此除非有特殊情况，否则这通常是正确的做法。这是编译器和编译系统中的默认设置。
