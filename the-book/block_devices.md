# 块设备-Block Devices

Fuchsia块设备驱动程序,跟其他系统一样,通过IPC访问用户空间服务来实现。使用块设备的程序将拥有这些底层驱动程序的一个或多个句柄(handle)。与filesystem client类似,客户端可以通过在RPC消息向服务器发送"read"或"write"的请求, 程序可以作为块设备的clients，也可以将RPC消息发送到“设备主机”(Zircon中称为“devhost”)。然后，devhost进程将这些请求转换为驱动程序理解的"I/O事务", 实际上他们被传输到特定的块设备驱动程序，最终传输到真实的硬件。



特定的块设备驱动程序(USB, AHCI / SATA, Ramdisk, GPT等) implement
the [`ZX_PROTOCOL_BLOCK_CORE`
prototol](https://fuchsia.googlesource.com/zircon/+/master/system/public/zircon/device/block.h),允许客户端进行队列事务和块设备查询。

## Fast块 I/O - Fast Block I/O

块设备驱动程序通常承担大部分内存,用一部分内存向特定设备发送"read into"或者"write from"队列请求。由于将大小有限的消息从RPC协议传输到"I/O事务"中，访问块设备常常需要重复复制大型缓冲区。

为了避免这种性能瓶颈，块设备驱动程序实现了另一种传输读写的机制：一种快速的、基于FIFO协议的共享VMO。 Filesystems(或任何其他客户机与块设备交互) 可以从块设备获取FIFO，注册“事务缓冲区”，并用句柄(handle)从VMOs传递到块设备。该协议的客户端可以在FIFO上发送快速，轻量级的控制消息，而不是使用大缓冲区发送"read into"或"write from"消息，这表明块设备驱动程序应该直接作用于已经注册的VMO。例如，在写入文件时，不是直接在IPC原语上传递字节，而是将它们复制到块设备内存中的新位置,Filesystem (将文件表示为VMO)可以简单地发送一个small FIFO消息指示"从VMO Y的偏移X直接写入N字节到磁盘上的Z偏移". 当与"mmap"内存映射工具结合使用时，它在访问文件时提供了从client直接到磁盘(或在其他方向)的"zero-copy"路径。
