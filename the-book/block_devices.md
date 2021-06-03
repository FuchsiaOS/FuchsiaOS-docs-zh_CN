<!--
# Block Devices

Fuchsia Block device drivers are, like other drivers on the system, implemented
as userspace services which are accessible via IPC. Programs using block devices
will have one or more handles to these underlying drivers. Similar to filesystem
clients, which may send “read” or “write” requests to servers by encoding these
requests within RPC messages, programs may act as clients to block devices, and
may transmit RPC messages to a “device host” (referred to as “devhost” within
Zircon). The devhost process then transforms these requests into
driver-understood “I/O transactions”, where they are actually transmitted to the
particular block device driver, and eventually to real hardware.

Particular block device drivers (USB, AHCI / SATA, Ramdisk, GPT, etc) implement
the [`ZX_PROTOCOL_BLOCK_CORE`
prototol](https://fuchsia.googlesource.com/zircon/+/master/system/public/zircon/device/block.h),
which allows clients to queue transactions and query the block device. -->

# 块设备

Fuchsia 块设备驱动程序,跟其他系统一样,通过 IPC 访问用户空间服务来实现。使用块设备的程序将拥有这些底层驱动程序的一个或多个句柄(handle)。与 filesystem client 类似,客户端可以通过在RPC消息向服务器发送"read"或"write"的请求, 程序可以作为块设备的 clients，也可以将 RPC 消息发送到“设备主机”(Zircon 中称为“devhost”)。然后，devhost 进程将这些请求转换为驱动程序理解的 "I/O事务", 实际上他们被传输到特定的块设备驱动程序，最终传输到真实的硬件。

特定的块设备驱动程序(USB, AHCI / SATA, Ramdisk, GPT等) 实现了 [`ZX_PROTOCOL_BLOCK_CORE`
prototol](https://fuchsia.googlesource.com/zircon/+/master/system/public/zircon/device/block.h),它允许客户端进行队列事务和块设备查询。

<!--
## Fast Block I/O

Block device drivers are often responsible for taking large portions of memory,
and queueing requests to a particular device to either “read into” or “write
from” a portion of memory. Unfortunately, as a consequence of transmitting
messages of a limited size from an RPC protocol into an “I/O transaction”,
repeated copying of large buffers is often required to access block devices.

To avoid this performance bottleneck, the block device drivers implement
another mechanism to transmit reads and writes: a fast, FIFO-based protocol
which acts on a shared VMO. Filesystems (or any other client wishing to
interact with a block device) can acquire FIFOs from a block device, register a
“transaction buffer”, and pass handles to VMOs to the block device. Instead of
transmitting “read” or “write” messages with large buffers, a client of this
protocol can instead send a fast, lightweight control message on a FIFO,
indicating that the block device driver should act directly on the
already-registered VMO. For example, when writing to a file, rather than
passing bytes over IPC primitives directly, and copying them to a new location
in the block device’s memory, a filesystem (representing the file as a VMO)
could simply send a small FIFO message indicating “write N bytes directly from
offset X of VMO Y to offset Z on a disk”. When combined with the “mmap”
memory-mapping tools, this provides a “zero-copy” pathway directly from client
programs to disk (or in the other direction) when accessing files. -->

## Fast块 I/O

块设备驱动程序通常承担大部分内存,用一部分内存向特定设备发送"read into"或者"write from"队列请求。由于将大小有限的消息从RPC协议传输到"I/O事务"中，访问块设备常常需要重复复制大型缓冲区。

为了避免这种性能瓶颈，块设备驱动程序实现了另一种传输读写的机制：一种快速的、基于FIFO协议的共享VMO。 Filesystems(或任何其他客户机与块设备交互) 可以从块设备获取FIFO，注册“事务缓冲区”，并用句柄(handle)从VMOs传递到块设备。该协议的客户端可以在FIFO上发送快速，轻量级的控制消息，而不是使用大缓冲区发送"read into"或"write from"消息，这表明块设备驱动程序应该直接作用于已经注册的 VMO。例如，在写入文件时，不是直接在 IPC 原语上传递字节，而是将它们复制到块设备内存中的新位置,Filesystem (将文件表示为VMO)可以简单地发送一个 small FIFO 消息指示"从VMO Y的偏移X直接写入N字节到磁盘上的Z偏移"。 当与"mmap"内存映射工具结合使用时，它在访问文件时提供了从client直接到磁盘(或在其他方向)的"zero-copy"路径。
