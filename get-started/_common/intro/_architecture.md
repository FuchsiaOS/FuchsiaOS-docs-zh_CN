<!-- 
![Data table showing high-level diagram of the entire Fuchsia system
architecture, highlighting core components and subsystems.]
(/get-started/images/intro/fuchsia-architecture.png){: width="1080"}
 -->
![展示整个 Fuchsia 系统架构的高层图表的数据表，强调了核心组件和子系统]
(/get-started/images/intro/fuchsia-architecture.png){: width="1080"}

<!-- The following architectural principles guide Fuchsia's design and development: -->

以下架构性原则指导了 Fuchsia 的设计与开发：
<!-- 
* [**Simple:**][simple]
  Fuchsia makes it easy to create, maintain, and integrate software and hardware across a wide range of devices.
 -->

* [**简单：**][simple]Fuchsia 让创建、维护和集成软件与硬件在各种设备中都变得容易。
<!-- 
* [**Secure:**][secure]
  Fuchsia has a kernel and software model designed for modern computing.
 -->

* [**安全：**][secure]Fuchsia 有着为现代计算设计的内核和软件模型。
<!-- 
* [**Updatable:**][updatable]
  As a modular operating system, Fuchsia allows the kernel, drivers, and software components to be independently updatable.
 -->

* [**可升级：**][updatable]作为模块化操作系统，Fuchsia 允许内核、驱动和软件组件独立升级。
<!-- 
* [**Performant:**][performant]
  Fuchsia is designed for real world product requirements and optimized for performance.
 -->
* [**高性能：**][performant]Fuchsia 为真实世界产品需求设计，并为性能优化。

<!-- 
The core of the system is [Zircon][glossary.zircon], a kernel and collection of
libraries for handling system startup and bootstrapping.
All other system components are implemented in user space and isolated,
reinforcing the **principle of least privilege**. This includes:
 -->

系统的核心是 [Zircon][glossary.zircon]，它是处理系统启动与引导的内核和一组库。其他所有系统组件都实现于用户空间并被隔离，再次强化了**最小特权原则**。这些组件包括：
<!-- 
*   Device drivers
*   Filesystems
*   Network stacks
 -->
*   设备驱动
*   文件系统
*   网络栈

[glossary.zircon]: /glossary/README.md#zircon
[simple]: /concepts/principles/simple.md
[secure]: /concepts/principles/secure.md
[updatable]: /concepts/principles/updatable.md
[performant]: /concepts/principles/performant.md