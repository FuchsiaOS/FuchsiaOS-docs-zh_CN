<!-- # Block Devices -->

# 块设备

<!-- Fuchsia Block device drivers are, like other drivers on the system, implemented
as userspace services that are accessible via IPC. Programs using block devices
will have one or more handles to these underlying drivers. Similar to filesystem
clients, which may send “read” or “write” requests to servers by encoding these
requests within RPC messages, programs may act as clients to block devices, and
may transmit RPC messages to a “device host” (referred to as “devhost” within
Zircon). The devhost process then transforms these requests into
driver-understood “I/O transactions”, where they are actually transmitted to the
particular block device driver, and eventually to real hardware. -->

Fuchsia 块设备驱动程序与系统上的其他驱动程序一样，作为用户的服务实现，可通过 IPC 访问。使用块设备的程序将拥有这些底层驱动程序的一个或多个句柄。与文件系统客户端类似，文件系统客户端可以通过在 RPC 消息中编码这些请求来向服务端发送“读”或“写”请求，程序可以充当块设备的客户端，并且可以将 RPC 消息传输到 “device host”（在 Zircon 中被称为 “Devhost”）。然后，devhost 进程将这些请求转换为驱动程序理解的“I/O 事务”，在这些事务中，它们实际上被传输到特定的块设备驱动程序，并最终传输到真正的硬件。

<!-- Particular block device drivers (USB, AHCI / SATA, Ramdisk, GPT, etc) implement
the [`ZX_PROTOCOL_BLOCK_CORE` protocol](/zircon/system/public/zircon/device/block.h),
which allows clients to queue transactions and query the block device. -->

特定的块设备驱动程序（USB、AHCI/SATA、Ramdisk、GPT 等）实现 [`ZX_PROTOCOL_BLOCK_CORE` 协议](/zircon/system/public/zircon/device/block.h) ，其允许客户端对事务进行排队并查询块设备。

## Fast Block I/O

<!-- Block device drivers are often responsible for taking large portions of memory,
and queueing requests to a particular device to either “read into” or “write
from” a portion of memory. Unfortunately, as a consequence of transmitting
messages of a limited size from an RPC protocol into an “I/O transaction”,
repeated copying of large buffers is often required to access block devices. -->

块设备驱动程序通常负责占用很大一部分内存，并对对特定设备的请求进行排队，以便对内存的一部分进行“read into”或“write from”。
不幸的是，由于将有限大小的消息通过RPC协议传输到“I/O事务”，访问块设备通常需要重复复制到大缓冲区。

<!-- To avoid this performance bottleneck, the block device drivers implement
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

为了避免这种性能瓶颈，块设备驱动程序实现了另一种传输读写的机制：基于FIFO的快速协议，作用于共享的VMO。 文件系统(或称希望与块设备交互的客户端)可以从块设备获取FIFO，注册“transaction buffer”，并将VMO的句柄传递给块设备。 该协议的客户端可以在FIFO上发送快速、轻量级的控制消息，指示块设备驱动程序应该直接作用于已经注册的VMO，而不是发送具有大缓冲区的“read”或“write”消息。例如，在写入文件时，文件系统(将文件表示为VMO)可以发送一个小的FIFO消息，指示“从VMO Y的偏移量X开始将N个字节直接在磁盘上的偏移量Z位置开始写入”，而不是直接在IPC原语上传递字节，并将它们复制到块设备的内存中的新位置。当与“mmap”内存映射工具结合使用时，这在访问文件时提供了直接从客户端程序到磁盘(或相反方向)的“zero-copy”路径。
