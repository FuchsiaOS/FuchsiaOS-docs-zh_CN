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
which allows clients to queue transactions and query the block device.

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
programs to disk (or in the other direction) when accessing files.
