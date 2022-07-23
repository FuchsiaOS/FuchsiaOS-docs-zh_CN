<!-- ## System calls -->
## 系统调用

<!-- 
User space code interacts with the objects in kernel space using
**system calls**. Zircon has system calls to perform low-level operations such
as:
 -->
用户空间代码使用**系统调用**与内核空间对象交互。Zircon 有执行低层操作的系统调用，如：

<!-- 
*   Memory management
*   Task and process management
*   Inter-process communication (IPC) and synchronization
*   Exception handling
*   Hardware support services (clocks, entropy, device I/O)
 -->
*   内存管理
*   任务和进程管理
*   进程间通信（IPC）与同步
*   异常处理
*   硬件支持服务（时钟、熵、设备 I/O）

<!-- 
<aside class="key-point">
  <p>Zircon has fewer system calls than POSIX-oriented operating systems due to
  services like filesystems and drivers being hosted outside the kernel. See the
  full list of available Zircon system calls in the
  <a href="/reference/syscalls.md"> reference documentation.</a></p>
</aside>
 -->
<aside class="key-point">
  <p>与面向 POSIX 的操作系统相比，Zircon 的系统调用数量更少，因为像文件系统和驱动之类的服务位于内核之外。
  请在<a href="/reference/syscalls.md">参考文档</a>中参阅可用的 Zircon 系统调用的完整列表。</p>
</aside>

<!-- 
User space processes access system calls through `libzircon.so` — a
[virtual Dynamic Shared Object (vDSO)][glossary.virtual-dynamic-shared-object].
The Zircon vDSO is a shared library in ELF format that the kernel maps into the
address space of each new process. This library is considered "virtual" because
it is exposed directly by the kernel image rather than being loaded from a file.
 -->
用户空间进程通过 `libzircon.so` 访问系统调用，这是一个
[虚拟动态共享对象][glossary.virtual-dynamic-shared-object]（virtual Dynamic Shared Object，vDSO）。
Zircon vDSO 是 ELF 格式的共享库，它被内核映射到每个新进程的地址空间。
这个库被称为“虚拟”，是因为它是直接由内核映像暴露，而非从文件加载的。

<!-- 
Most system calls operate directly with one or more [handles][glossary.handle]
— process-local references to objects living in kernel space represented as a
32-bit integer (`zx_handle_t`). Each handle declares the privileges, or
**rights**, the holder has to perform actions on the handle itself or the
referenced object.
 -->
大多数系统调用直接操作一个或多个[句柄][glossary.handle]。
句柄是进程内部对内核空间对象的引用，表示为32位整数（`zx_handle_t`）。
每个句柄声明了持有者具有的对句柄自身或引用的对象执行操作的特权，即**权利**（right）。

<aside class="key-point">
 <!-- <b>Handles vs. file descriptors</b> -->
 <b>句柄与文件描述符之间的比较</b>
 <!-- 
 <p>Similar to POSIX file descriptors, handles are references to a specific
 kernel object and they play a role in granting capabilities. However, Zircon
 handles are slightly more flexible with rights applied to the handle rather
 than the calling process. It is possible for a single process to have two
 different handles to the same kernel object with different rights.</p>
 -->
<p>与 POSIX 文件描述符类似，句柄是对特定内核对象的引用，具有授予能力的作用。
但是，Zircon 句柄稍为灵活，因为权利是授予到句柄，而不是调用进程。
一个进程可以持有两个不同的句柄，它们引用同一个内核对象但有不同的权利。</p>
<!-- 
 <p>In addition, handles cannot be referenced by name and Zircon does not
 reserve any identifiers for common streams like stdin and stdout.</p>
 -->
  <p>另外，句柄不能通过名称引用，Zircon 不会为像 stdin 和 stdout 这样的公共流保留任何标识符。</p>
<!-- 
 <p>For more details, see <a href="/concepts/kernel/handles.md">
 Zircon handles</a>.</p>
  -->
  <p>要获取更多详情，请参阅 <a href="/concepts/kernel/handles.md">Zircon 句柄</a>。</p>
</aside>


[glossary.virtual-dynamic-shared-object]: /glossary/README.md#virtual-dynamic-shared-object
[glossary.handle]: /glossary/README.md#handle
