<!-- **Components** are the foundational building blocks of software running in
Fuchsia. Each component is a composable, sandboxed module that interacts with
other components through capabilities. This promotes system security and
creates clear interfaces between individual components, making them easier to
update or replace. -->
**组件**是 Fuchsia 中运行的软件的基石。每一个组件是可以组合的沙盒模块，相互之间通过功能交互。
这提高了系统安全性并在各个组件之间建立了清晰的接口，使它们更容易更新或替换。

<!-- In Fuchsia, **everything is a component** (almost). Recall from the previous
discussion of Zircon that the surface area of the kernel is intentionally small,
with most core services being implemented in user space. This means that most
software running on Fuchsia is implemented using the component framework,
including: -->
Fuchsia 中**一切都是组件**（几乎）。回想一下之前对 Zircon 的讨论，内核有意设计得很小，
大多数核心服务都是在用户空间中实现。这意味着在 Fuchsia 上运行的大多数软件都是使用组件框架实现的，包括：

<!-- *   User-facing applications
*   Device drivers
*   Filesystems
*   Media codecs
*   Network stacks -->
* 面向用户的应用
* 设备驱动
* 文件系统
* 媒体编解码器
* 网络栈

<!-- Outside the kernel there are only a few low-level exceptions not using the
component framework, such as bootloaders and the `userboot` process. -->
内核之外有少数不使用组件框架的底层例外，如引导程序和 `userboot` 进程。
