<!-- 
[Zircon][glossary.zircon] is the core that powers Fuchsia.
It is composed of a kernel and a small set of userspace services, drivers,
and libraries necessary for core system functions such as booting.
 -->
[Zircon][glossary.zircon] 是驱动 Fuchsia 的核心。
它由一个内核和一些对核心系统功能必要的用户空间服务、驱动和库组成。

<!-- 
Although [Zircon][glossary.zircon] applies many of the concepts popularized by
microkernels, it does not strive to be minimal. Instead, the microkernel-like
architecture of Zircon enables Fuchsia to reduce the amount of trusted code
running in the system to a few core functions:
 -->
尽管 [Zircon][glossary.zircon] 应用了许多流行于微内核的概念，但它并非为了微而微。
相反，Zircon 的类微内核架构使 Fuchsia 能将系统中运行的受信任代码量减少到一些核心功能：

<!-- 
* Memory management
* Scheduling
* Inter-process communication
 -->
* 内存管理
* 调度
* 进程间通信

<!-- 
![Data table showing a comparison between kernel services in Fuchsia and a
typical operating system, indicating Fuchsia includes fewer services in its
kernel.]
(/get-started/images/intro/kernel-services.png){: width="799"}
 -->
![展示了 Fuchsia 内核服务和一种典型操作系统中的内核服务的比较的数据表，显示 Fuchsia 在内核中包含的服务更少。]
(/get-started/images/intro/kernel-services.png){: width="799"}

[glossary.zircon]: /glossary/README.md#zircon
