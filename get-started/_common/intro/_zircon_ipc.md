<!-- ## Inter-process communication -->
## 进程间通信

<!-- 
Since processes are isolated by default, the kernel needs to provide a way for
them to securely communicate with each other. Zircon includes the following
kernel object types for inter-process communication (IPC):
 -->
因为进程间默认有隔离，所以内核需要提供一种方式使进程能安全地互相通信。
Zircon 包含了以下内核对象类型用于进程间通信（inter-process communication，IPC）：

<!-- 
* [Event](/reference/kernel_objects/event.md):
  Signaling interface between two processes.
* [Socket](/reference/kernel_objects/socket.md):
  Streaming data transport, similar to a pipe.
* [Stream](/reference/kernel_objects/stream.md):
  Streaming data transport that is seekable, like a file.
* [Channel](/reference/kernel_objects/channel.md):
  Message-based transport capable of passing both data and a set of handles.
* [FIFO](/reference/kernel_objects/fifo.md):
  Control plane for shared memory access, optimized for small data payloads.
 -->
* [事件](/reference/kernel_objects/event.md):
  两个进程之间的信号接口。
* [套接字](/reference/kernel_objects/socket.md):
  流式数据传输，类似管道。
* [流](/reference/kernel_objects/stream.md):
  可定位的流式数据传输，类似文件。
* [通道](/reference/kernel_objects/channel.md):
  基于消息的传输，可以传递数据和多个句柄。
* [FIFO（先进先出）](/reference/kernel_objects/fifo.md):
  共享内存访问的控制平面，为小数据负载优化。

<!-- 
Among these objects, channels are uniquely suited to assist in launching new
processes because they are capable of transferring handles (and therefore,
capabilities) across to another process.
 -->
在这些对象之中，通道的独特性在于其适合帮助启动新进程，因为它能够向另一个进程传递句柄（即能力）。

<!-- 
Channels have exactly two endpoint handles, each owned by a separate process.
Only the owners may read or write messages, but ownership of an endpoint may
be transferred from one process to another. When handles are written into a
channel, they are removed from the sending process. When a message with handles
is read from a channel, the handles are added to the receiving process.
 -->
通道有两个端点句柄，每个都属于不同的进程。只有双方所有者可以读取或写入消息，但是端点的所有权可以从一个进程转移到另一个进程。
当句柄被写入通道时，它会从发送进程中移除。当一条带有句柄的消息被读取时，句柄会被添加到接收进程中。

<!-- 
![Diagram showing how processes communicate through shared objects found in the
kernel. The most common of these connections is the channel.]
(/get-started/images/intro/ipc.png){: width="582"}
 -->
![该图显示进程如何通过内核中的共享对象来通信。这些连接中最常见的是通道。]
(/get-started/images/intro/ipc.png){: width="582"}

<!-- 
Note: You can find more of Zircon's deep technical details in the
[kernel documentation](/concepts/kernel/README.md).
 -->
注意：您可以在[内核文档](/concepts/kernel/README.md)中找到更多关于 Zircon 的深入技术细节。

<!-- 
Zircon channels are the basis for service-level IPC protocols described by
the [Fuchsia Interface Definition Language (FIDL)][glossary.FIDL]. FIDL
protocols are the primary method of IPC used by Fuchsia programs. You will
explore creating and consuming FIDL protocols in more detail later on.
 -->
Zircon 通道是 [Fuchsia 接口定义语言（Fuchsia Interface Definition Language，FIDL)][glossary.FIDL]
描述的服务层 IPC 协议的基础。FIDL 协议是 Fuchsia 程序使用的主要 IPC 方式。
您将在稍后更详细地探索创建和消费 FIDL 协议。

[glossary.FIDL]: /glossary/README.md#FIDL
