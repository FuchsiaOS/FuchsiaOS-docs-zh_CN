<!--

# Filesystem Architecture

-->

# 文件系统架构

<!--

This document seeks to describe a high-level view of the Fuchsia filesystems,
from their initialization, discussion of standard filesystem operations (such as
Open, Read, Write, etc), and the quirks of implementing user-space filesystems
on top of a microkernel. Additionally, this document describes the VFS-level
walking through a namespace, which can be used to communicate with non-storage
entities (such as system services).

-->

本文旨在描述Fuchsia 文件系统的高级视图。
从文件系统的初始化开始，讨论标准文件的操作（例如打开、读取、写入等），
以及在微内核之上实现用户空间文件系统。
此外，本文档描述了 VFS 级别
遍历命名空间，该命名空间可用于与非存储实体（如系统服务）进行通信。

<!--

## Filesystems are Services

-->

## 文件系统服务

<!--

Unlike more common monolithic kernels, Fuchsia’s filesystems live entirely
within userspace. They are not linked nor loaded with the kernel; they are
simply userspace processes that implement servers that can appear as
filesystems. As a consequence, Fuchsia’s filesystems themselves can be changed
with ease -- modifications don’t require recompiling the kernel. In fact,
updating to a new Fuchsia filesystem can be done without rebooting.

-->

与更常见的单核不同，Fuchsia 的文件系统完全存在于用户空间中。 
它们不与内核链接或加载，它们只是实现可以显示为文件系统的服务器的用户空间进程。 
因此，Fuchsia 的文件系统本身可以轻松更改——修改不需要重新编译内核。 
事实上，无需重启即可更新到新的 Fuchsia 文件系统。 

<!--

Like other native servers on Fuchsia, the primary mode of interaction with a
filesystem server is achieved using the handle primitive rather than system
calls. The kernel has no knowledge about files, directories, or filesystems. As
a consequence, filesystem clients cannot ask the kernel for “filesystem access”
directly.

-->

与 Fuchsia 上的其他服务一样，
与文件系统服务端交互的主要模式是使用句柄原语而不是系统调用来实现的。
内核对文件、目录或文件系统一无所知。 
因此，文件系统客户端不能直接向内核请求“文件系统访问”。 

<!--

This architecture implies that the interaction with filesystems is limited to
the following interface:

-->

这种架构意味着与文件系统的交互仅限于以下接口： 

<!--

 * The messages sent on communication channels established with the filesystem
   server. These communication channels may be local for a client-side
   filesystem, or remote.
 * The initialization routine (which is expected to be configured heavily on a
   per-filesystem basis; a networking filesystem would require network access,
   persistent filesystems may require block device access, in-memory filesystems
   would only require a mechanism to allocate new temporary pages).

-->

* 在与文件系统服务建立的通信通道上发送的消息。 
  对于客户端文件系统，这些通信通道可能是本地的，也可能是远程的。
* 常规初始化（预计将在每个文件系统的基础上进行大量配置；
  网络文件系统需要网络访问，持久文件系统可能需要块设备访问，
  内存文件系统只需要一种分配新临时页面的机制） 。

<!--

As a benefit of this interface, any resources accessible via a channel can make
themselves appear like filesystems by implementing the expected protocols for
files or directories. For example, “serviceFS” (discussed in more detail later
in this document) allows for service discovery through a filesystem interface.

-->

作为这个接口的好处，
任何通过通道访问的资源都可以通过实现文件或目录的预期协议使自己看起来像文件系统。 
例如，“serviceFS”（本文档稍后将详细讨论）允许通过文件系统接口进行服务发现。 

<!--

## File Lifecycle

### Establishing a Connection

-->

## 文件生命周期

### 建立连接

<!--

To open a file, Fuchsia programs (clients) send RPC requests to filesystem
servers using a FIDL.

-->

为了打开文件，Fuchsia 程序（客户端）使用 FIDL 向文件系统服务器发送 RPC 请求。 

<!--

FIDL defines the wire-format for transmitting messages and handles between a
filesystem client and server. Instead of interacting with a kernel-implemented
VFS layer, Fuchsia processes send requests to filesystem services which
implement protocols for Files, Directories, and Devices. To send one of these
open requests, a Fuchsia process must transmit an RPC message over an existing
handle to a directory; for more detail on this process, refer to the [life of an
open document](/docs/concepts/system/life_of_an_open.md).

-->

FIDL 定义了在文件系统客户端和服务端之间传输消息和句柄的有线格式。 
Fuchsia 进程不与内核实现的 VFS 层交互，而是将请求发送到文件系统服务，
这些服务实现了文件、目录和设备的协议。 
要发送这些打开请求之一，Fuchsia 进程必须通过现有句柄将 RPC 消息传输到目录； 
有关此过程的更多详细信息，请参阅 [“打开”的生命周期](/docs/concepts/system/life_of_an_open.md)。 

<!--

### Namespaces

-->

命名空间

<!--

On Fuchsia, a [namespace](/docs/concepts/process/namespaces.md) is a small filesystem that exists
entirely within the client. At the most basic level, the idea of the client
saving “/” as root and associating a handle with it is a very primitive
namespace. Instead of a typical singular "global" filesystem namespace, Fuchsia
processes can be provided an arbitrary directory handle to represent "root",
limiting the scope of their namespace. In order to limit this scope, Fuchsia
filesystems [intentionally do not allow access to parent directories via
dotdot](/docs/concepts/filesystems/dotdot.md).

-->

在 Fuchsia 上，[命名空间](/docs/concepts/process/namespaces.md) 
是一个完全存在于客户端中的小文件系统。 
在最基本的层面上，客户端将`/`保存为 root 并将句柄与其关联的想法是一个非常原始的命名空间。 
可以为 Fuchsia 进程提供一个任意目录句柄来表示“root”，而不是典型的单一“global”文件系统命名空间，
从而限制它们的命名空间范围。 
为了限制这个范围，Fuchsia 文件系统 [故意不允许通过 dotdot 访问父目录](/docs/concepts/filesystems/dotdot.md)。 


<!--

Fuchsia processes may additionally redirect certain path operations to separate
filesystem servers. When a client refers to “/bin”, the client may opt to
redirect these requests to a local handle representing the “/bin” directory,
rather than sending a request directly to the “bin” directory within the “root”
directory. Namespaces, like all filesystem constructs, are not visible from the
kernel: rather, they are implemented in client-side runtimes (such as
[libfdio](/docs/concepts/system/life_of_an_open.md#Fdio)) and are interposed between most client code
and the handles to remote filesystems.

-->

Fuchsia 进程可以另外将某些路径操作重定向到单独的文件系统服务端。 
当客户端引用`/bin`时，客户端可以选择将这些请求重定向到代表`/bin`目录的本地句柄，
而不是直接向“root”目录中的“bin”目录发送请求。 
与所有文件系统结构一样，命名空间在内核中是不可见的：
相反，他们在客户端一侧运行中实现（例如 [libfdio](/docs/concepts/system/life_of_an_open.md#Fdio)），
并且被插入在大多数客户端代码和远程文件系统的句柄之间。

<!--

Since namespaces operate on handles, and most Fuchsia resources and services
are accessible through handles, they are extremely powerful concepts.
Filesystem objects (such as directories and files), services, devices,
packages, and environments (visible by privileged processes) all are usable
through handles, and may be composed arbitrarily within a child process. As a
result, namespaces allows for customizable resource discovery within
applications. The services that one process observes within “/svc” may or may
not match what other processes see, and can be restricted or redirected
according to application-launching policy.

-->

由于命名空间对句柄进行操作，并且大多数 Fuchsia 资源和服务都可以通过句柄访问，
因此它们是非常强大的概念。 
文件系统对象（例如目录和文件）、服务、设备、包和环境（特权进程可见）都可以通过句柄使用，
并且可以在子进程中任意组合。 
因此，命名空间允许在应用程序中进行可定制的资源发现。 
一个进程在`/svc`中观察到的服务可能与其他进程看到的相匹配，也可能不匹配，
并且可以根据应用程序启动策略进行限制或重定向。 

<!--

For more detail the mechanisms and policies applied to restricting process
capability, refer to the documentation on
[sandboxing](/docs/concepts/process/sandboxing.md).

-->

有关用于限制过程能力的机制和策略的更多详细信息，请参阅关于
[sandboxing](/docs/concepts/process/sandboxing.md)。 

<!--

### Passing Data

-->

### 传递数据 

<!--

Once a connection has been established, either to a file, directory, device,
or service, subsequent operations are also transmitted using RPC messages.
These messages are transmitted on one or more handles, using a wire format that
the server validates and understands.

-->

一旦建立了与文件、目录、设备或服务的连接，后续操作也将使用 RPC 消息进行传输。 
这些消息使用服务器验证和理解的有线格式在一个或多个句柄上传输。 

<!--

In the case of files, directories, devices, and services, these operations use the
FIDL protocol.

-->

对于文件、目录、设备和服务，这些操作使用 FIDL 协议。 

<!--

As an example, to seek within a file, a client would send a `Seek`
message with the desired position and “whence” within the FIDL message, and the
new seek position would be returned. To truncate a file, a `Truncate`
message could be sent with the new desired filesystem, and a status message
would be returned. To read a directory, a `ReadDirents` message could be
sent, and a list of direntries would be returned. If these requests were sent to
a filesystem entity that can’t handle them, an error would be sent, and the
operation would not be executed (like a `ReadDirents` message sent to a text
file).

-->

例如，要在文件中查找，客户端将发送一条“查找”消息，
其中包含所需位置和 FIDL 消息中的“来源”，然后将返回新的查找位置。 
要截断文件，可以使用新的所需文件系统发送“截断”消息，并返回状态消息。 
要读取目录，可以发送“ReadDirents”消息，并返回目录列表。 
如果这些请求被发送到无法处理它们的文件系统实体，则会发送错误，
并且不会执行操作（就像发送到文本文件的“ReadDirents”消息一样）。 

<!--

### Memory Mapping

-->

### 内存映射

<!--

For filesystems capable of supporting it, memory mapping files is slightly more
complicated. To actually “mmap” part of a file, a client sends an “GetVmo”
message, and receives a Virtual Memory Object, or VMO, in response. This object
is then typically mapped into the client’s address space using a Virtual Memory
Address Region, or VMAR. Transmitting a limited view of the file’s internal
“VMO” back to the client requires extra work by the intermediate message
passing layers, so they can be aware they’re passing back a server-vendored
object handle.

-->

对于能够支持它的文件系统，内存映射文件稍微复杂一些。 
要真正“mmap”文件的一部分，客户端会发送“GetVmo”消息，并接收虚拟内存对象或 VMO 作为响应。 
然后，此对象通常使用虚拟内存地址区域或 VMAR 映射到客户端的地址空间。 
将文件内部“VMO”的有限视图传输回客户端需要中间消息传递层的额外工作，
因此他们可以知道他们正在传回服务器供应的对象句柄。 

<!--

By passing back these virtual memory objects, clients can quickly access the
internal bytes representing the file without actually undergoing the cost of a
round-trip IPC message. This feature makes mmap an attractive option for
clients attempting high-throughput on filesystem interaction.

-->

通过传回这些虚拟内存对象，客户端可以快速访问表示文件的内部字节，而无需实际承担往返 IPC 消息的成本。 
此功能使 mmap 成为尝试高吞吐量文件系统交互的客户端的一个有吸引力的选择。 

<!--

At the time of writing, on-demand paging is not supported by the
kernel, and has not been wired into filesystems. As a result, if a client
writes to a “memory-mapped” region, the filesystem cannot reasonably identify
which pages have and have not been touched. To cope with this restriction, mmap
has only been implemented on **read-only filesystems**, such as blobfs.

-->

在撰写本文时，内核不支持按需分页，并且还没有连接到文件系统中。 
因此，如果客户端写入“内存映射”区域，文件系统将无法合理地识别哪些页面已被触及，哪些页面未被触及。 
为了应对这一限制，mmap 仅在**只读文件系统**上实现，例如 blobfs。 

<!--

### Other Operations acting on paths

-->

### 作用于路径的其他操作 

<!--

In addition to the “open” operation, there are a couple other path-based
operations worth discussing: “rename” and “link”. Unlike “open”, these
operations actually act on multiple paths at once, rather than a single
location. This complicates their usage: if a call to “rename(‘/foo/bar’,
‘baz’)” is made, the filesystem needs to figure out a way to:

-->

除了“打开”操作之外，还有一些基于路径的操作值得讨论：“重命名”和“链接”。 
与“开放”不同，这些操作实际上一次作用于多条路径，而不是单个位置。 
这使得它们的使用变得复杂：如果调用`rename(‘/foo/bar’, ‘baz’)`可以实现，
文件系统需要： 

<!--

  * Traverse both paths, even when they have distinct starting points (which is the
    case this here; one path starts at root, and other starts at the CWD)
  * Open the parent directories of both paths
  * Operate on both parent directories and trailing pathnames simultaneously

-->

* 遍历两条路径，即使它们有不同的起点（这里就是这种情况；一条路径从根开始，另一条从 CWD 开始）
* 打开两个路径的父目录
* 同时对父目录和尾随路径名进行操作 

<!--

To satisfy this behavior, the VFS layer takes advantage of a Zircon concept
called “cookies”. These cookies allow client-side operations to store open
state on a server, using a handle, and refer to it later using that same
handles. Fuchsia filesystems use this ability to refer to one Vnode while
acting on the other.

-->

为了满足这种行为，VFS 层利用了称为“cookies”的 Zircon 概念。 
这些 cookie 允许客户端操作使用句柄在服务器上存储打开状态，并在以后使用相同的句柄引用它。 
Fuchsia 文件系统使用这种能力来引用一个 Vnode，同时作用于另一个。 

<!--

These multi-path operations do the following:

-->

这些多路径操作执行以下操作： 

<!--

  * Open the parent source vnode (for “/foo/bar”, this means opening “/foo”)
  * Open the target parent vnode (for “baz”, this means opening the current
    working directory) and acquire a vnode token using the operation
    `GetToken`, which is a handle to a filesystem cookie.
  * Send a “rename” request to the source parent vnode, along with the source
    and destination paths (“bar” and “baz”), along with the vnode token acquired
    earlier. This provides a mechanism for the filesystem to safely refer to the
    destination vnode indirectly -- if the client provides an invalid handle, the
    kernel will reject the request to access the cookie, and the server can return
    an error.

-->

* 打开父源vnode（对于“/foo/bar”，这意味着打开“/foo”）
* 打开目标父 vnode（对于“baz”，这意味着打开当前工作目录）并使用操作 `GetToken` 获取 vnode 令牌，
  它是文件系统 cookie 的句柄。
* 向源父 vnode 发送“重命名”请求，以及源和目标路径（“bar”和“baz”），以及之前获取的 vnode 令牌。 
  这为文件系统提供了一种机制，可以安全地间接引用目标 vnode——如果客户端提供了无效句柄，
  内核将拒绝访问 cookie 的请求，服务端可以返回错误。 

<!--

## Filesystem Lifecycle

### Mounting

-->

## 文件系统生命周期

### 挂载 

<!--

When Fuchsia filesystems are initialized, they are created with typically two
handles: One handle to a channel used to communicate with the mounting
filesystem (referred to as the “mount point” channel -- the “mounting” end of
this channel is saved as a field named “remote” in the parent Vnode, the other
end will be connected to the root directory of the new filesystem), and
(optionally) another to contact the underlying
[block device](/docs/concepts/filesystems/block_devices.md).
Once a filesystem has been initialized (reading initial state off the block
device, finding the root vnode, etc) it starts servicing [`fuchsia.io/Node`]
requests on the mount point channel.

-->

初始化 Fuchsia 文件系统时，通常会使用两个句柄创建它们：
一个句柄指向用于与挂载文件系统通信的通道
（称为“挂载点”通道——该通道的“挂载”端保存为 父Vnode中名为“remote”的字段，
另一端将连接到新文件系统的根目录），并且（可选）另一个连接底层
[block device](/docs/concepts/filesystems/block_devices.md) 。 
一旦文件系统被初始化（从块设备读取初始状态，找到根 vnode 等），
它就会开始在挂载点通道上为 [`fuchsia.io/Node`] 请求提供服务。 

<!--

At this point, the parent (mounting) filesystem “pins” the connection to the
remote filesystem on a Vnode. The VFS layers capable of path walking check for
this remote handle when observing Vnodes: if a remote handle is detected, then
the incoming request (open, rename, etc) is forwarded to the remote filesystem
instead of the underlying node. If a user actually wants to interact with the
mountpoint node, rather than the remote filesystem, they can pass the
`O_NOREMOTE` flag to the “open” operation identify this intention.

-->

此时，父（挂载）文件系统将连接“pins”到 Vnode 上的远程文件系统。 
VFS 层能够在观察 Vnode 时检查此远程句柄的路径：
如果检测到远程句柄，则传入请求（打开、重命名等）被转发到远程文件系统而不是底层节点。 
如果用户实际上想要与挂载点节点而不是远程文件系统交互，
他们可以将“O_NOREMOTE”标志传递给“打开”操作来识别此意图。 

<!--

Unlike many other operating systems, the notion of “mounted filesystems” does
not live in a globally accessible table. Instead, the question “what
mountpoints exist?” can only be answered on a filesystem-specific basis -- an
arbitrary filesystem may not have access to the information about what
mountpoints exist elsewhere.

-->

与许多其他操作系统不同，“挂载文件系统”的概念并不存在于全局可访问的表中。 
相反，问题是“存在哪些挂载点？” 
只能在特定于文件系统的基础上回答——任意文件系统可能无法访问有关其他地方存在哪些挂载点的信息。 

<!--

### Filesystem Management

-->

### 文件系统管理

<!--

There are a collection of filesystem operations that are considered related to
"administration", including "unmounting the current filesystem", "querying for
the underlying block device path", etc. These operations are defined by the
DirectoryAdmin interface within [io.fidl](/sdk/fidl/fuchsia.io/io.fidl).
A connection to this interface allows access to "filesystem-wide" state, and is
restricted by an access flag `ZX_FS_RIGHT_ADMIN`. This access right must be
requested explicitly, and is not granted when requested on a connection lacking
`ZX_FS_RIGHT_ADMIN`. This right is provided to the root connection of a
filesystem once it is mounted - a reasonable bootstrapping point for
administration - but must be preserved by the mounting tools to propagate this
access, or must be dropped when vending connections from the filesystem to less
privileged clients.

-->

有一组文件系统操作被认为与“管理”相关，
包括“卸载当前文件系统”、“查询底层块设备路径”等。
这些操作由 [io.fidl](/sdk/fidl/fuchsia.io/io.fidl) 中的 DirectoryAdmin 接口定义。 
与此接口的连接允许访问“文件系统范围”状态，并受访问标志`ZX_FS_RIGHT_ADMIN`的限制。 
此访问权限必须明确请求，并且在缺少`ZX_FS_RIGHT_ADMIN`的连接上请求时不会授予。 
此权限在文件系统安装后提供给根连接
—— 一个合理的管理引导点 
—— 但必须由安装工具保留以传播此访问权限，或者在将文件系统的连接出售给特权较低的客户端时必须删除。 

<!--

This `ZX_FS_RIGHT_ADMIN` mechanism (occasionally referred to as `O_ADMIN`, for
the POSIX interop declaration) will be superceded by an explicit service for
filesystem administration. Rather than existing as an "implicit right" attached
silently to limited directory connections, it will be a separate interface
exposed by filesystem components. This will (in the abstract) allow filesystems
to expose a "root directory" handle and an "administraction" handle separately,
rather than overloading them on the same connection. Once this transition has
occurred, the `ZX_FS_RIGHT_ADMIN` (and `O_ADMIN`) flags will be deprecated.

-->

这种`ZX_FS_RIGHT_ADMIN`机制（有时称为`O_ADMIN`，用于 POSIX 互操作声明）
将被用于文件系统管理的显式服务取代。 
它不是作为一种“隐式权限”默默地附加到有限的目录连接上，
而是由文件系统组件公开的一个单独的接口。 
这将（抽象地）允许文件系统分别公开“root directory”句柄和“administraction”句柄，
而不是在同一连接上重载它们。 一旦发生这种转变，`ZX_FS_RIGHT_ADMIN`（和`O_ADMIN`）标志将被弃用。 

<!--

## Current Filesystems

-->

## 当前的文件系统

<!--

Due to the modular nature of Fuchsia’s architecture, it is straightforward to
add filesystems to the system. At the moment, a handful of filesystems exist,
intending to satisfy a variety of distinct needs.

-->

由于 Fuchsia 架构的模块化特性，可以直接向系统添加文件系统。 
目前，存在少数文件系统，旨在满足各种不同的需求。 

<!--

### MemFS: An in-memory filesystem

[MemFS](/src/storage/memfs)
is used to implement requests to temporary filesystems like `/tmp`, where files
exist entirely in RAM, and are not transmitted to an underlying block device.
This filesystem is also currently used for the “bootfs” protocol, where a
large, read-only VMO representing a collection of files and directories is
unwrapped into user-accessible Vnodes at boot (these files are accessible in
`/boot`).

-->

### MemFS：内存文件系统 

[MemFS](/src/storage/memfs)用于实现对像`/tmp`这样的临时文件系统的请求，
其中文件完全存在于RAM中，不会传输到底层块设备。
这个文件系统目前也用于 “bootfs”协议，
其中表示文件和目录集合的大型只读 VMO 在启动时被解包到用户可访问的 Vnode 中
（这些文件可在 `/boot` 中访问）。

<!--

### MinFS: A persistent filesystem

[MinFS](/src/storage/bin/minfs/)
is a simple, traditional filesystem that is capable of storing files
persistently. Like MemFS, it makes extensive use of the VFS layers mentioned
earlier, but unlike MemFS, it requires an additional handle to a block device
(which is transmitted on startup to a new MinFS process). For ease of use,
MinFS also supplies a variety of tools: “mkfs” for formatting, “fsck” for
verification, as well as “mount” and “umount” for adding and subtracting MinFS
filesystems to a namespace from the command line.

-->

### MinFS：持久文件系统

[MinFS](/src/storage/bin/minfs/) 是一个简单的传统文件系统，能够持久存储文件。 
与 MemFS 一样，它广泛使用了前面提到的 VFS 层，但与 MemFS 不同的是，
它需要额外的块设备句柄（在启动时传输到新的 MinFS 进程）。 
为了便于使用，MinFS 还提供了多种工具：
“mkfs”用于格式化，“fsck”用于验证，
以及“mount”和“umount”用于从命令行将 MinFS 文件系统添加和减去命名空间。 

<!--

### Blobfs: An immutable, integrity-verifying package storage filesystem

[Blobfs](/src/storage/bin/blobfs/)
is a simple, flat filesystem optimized for “write-once, then read-only” [signed
data](/docs/concepts/packages/merkleroot.md), such as
[packages](/docs/concepts/packages/package.md).
Other than two small prerequisites (file names, which are deterministic, content
addressable hashes of a file’s Merkle Tree root, for integrity-verification)
and forward knowledge of file size (identified to Blobfs by a call to
“ftruncate” before writing a blob to storage), Blobfs appears like a
typical filesystem. It can be mounted and unmounted, it appears to contain a
single flat directory of hashes, and blobs can be accessed by operations like
“open”, “read”, “stat” and “mmap”.

-->

### Blobfs：一个不可变的、完整性验证的包存储文件系统

[Blobfs](/src/storage/bin/blobfs/) 是一个简单的平面文件系统，
针对“一次写入，然后只读”[signed data](/docs/concepts/packages/merkleroot.md) 进行了优化，
例如，作为[package](/docs/concepts/packages/package.md)。 
除了两个小的先决条件（文件名是确定性的，文件的 Merkle 树根的内容可寻址哈希，用于完整性验证）
和文件大小的前向知识（通过在将 blob 写入之前调用“ftruncate”识别给 Blobfs storage)，
Blobfs 看起来像一个典型的文件系统。 它可以挂载和卸载，它似乎包含一个单一的散列平面目录，
并且可以通过“open”、“read”、“stat”和“mmap”等操作访问 blob。 

<!--

### ThinFS: A FAT filesystem written in Go

[ThinFS](/src/lib/thinfs/) is an implementation of a
FAT filesystem in Go. It serves a dual purpose: first, proving that our system
is actually modular, and capable of using novel filesystems, regardless of
language or runtime. Secondly, it provides a mechanism for reading a universal
filesystem, found on EFI partitions and many USB sticks.

-->

### ThinFS：用 Go 编写的 FAT 文件系统

[ThinFS](/src/lib/thinfs/) 是 Go 中 FAT 文件系统的实现。 
它有双重目的：首先，证明我们的系统实际上是模块化的，
并且能够使用新颖的文件系统，而不管语言或运行时。 
其次，它提供了一种读取通用文件系统的机制，可以在 EFI 分区和许多 USB 记忆棒上找到。 


<!--
### FVM

[Fuchsia Volume Manager](/src/storage/fvm/driver/)
is a "logical volume manager" that adds flexibility on top of existing block
devices. The current features include ability to add, remove, extend and
shrink virtual partitions. To make these features possible, internally, fvm
maintains physical to virtual  mapping from (virtual partitions, blocks) to
(slice, physical block). To keep maintenance overhead minimal, it allows to
partitions to shrink/grow in chunks called slices. A slice is multiple of the
native block size. Metadata aside, the rest of the device is divided into
slices. Each slice is either free or it belongs to one and only one partition.
If a slice belongs to a partition then FVM maintains metadata about which
partition is using the slice, and the virtual address of the slice within
that partition.

-->

#### FVM

[Fuchsia Volume Manager](/src/storage/fvm/driver/)是一个“逻辑卷管理器”，
它在现有块设备之上增加了灵活性。 
当前的功能包括添加、删除、扩展和缩小虚拟分区的能力。 
为了使这些功能成为可能，fvm 在内部维护了从（虚拟分区、块）到（切片、物理块）的物理到虚拟映射。 
为了使维护开销最小化，它允许分区以称为切片的块的形式收缩/增长。 
切片是原生块大小的倍数。 抛开元数据不谈，设备的其余部分被分割成片。 
每个切片要么是空闲的，要么属于一个且仅属于一个分区。 
如果一个分片属于一个分区，那么 FVM 会维护有关哪个分区正在使用该分片的元数据，
以及该分片在该分区内的虚拟地址。 

<!--

[Superblock](/src/storage/fvm/format.h#27)
at block zero describe the on-disk layout of the FVM, which may look like

-->

[Superblock](/src/storage/fvm/format.h#27)在第0块描述FVM的磁盘布局，可能看起来像

<!--

```c
      +---------------------------------+ <- Physical block 0
      |           metadata              |
      | +-----------------------------+ |
      | |       metadata copy 1       | |
      | |  +------------------------+ | |
      | |  |    superblock          | | |
      | |  +------------------------+ | |
      | |  |    partition table     | | |
      | |  +------------------------+ | |
      | |  | slice allocation table | | |
      | |  +------------------------+ | |
      | +-----------------------------+ | <- Size of metadata is described by
      | |       metadata copy 2       | |    superblock
      | +-----------------------------+ |
      +---------------------------------+ <- Superblock describes start of
      |                                 |    slices
      |             Slice 1             |
      +---------------------------------+
      |                                 |
      |             Slice 2             |
      +---------------------------------+
      |                                 |
      |             Slice 3             |
      +---------------------------------+
      |                                 |
```
-->

```c
      +---------------------------------+ <- 物理块 0
      |              元数据              |
      | +-----------------------------+ |
      | |           元数据副本  1       | |
      | |  +------------------------+ | |
      | |  |         超级块          | | |
      | |  +------------------------+ | |
      | |  |         分区表          | | |
      | |  +------------------------+ | |
      | |  |       切片分配表         | | |
      | |  +------------------------+ | |
      | +-----------------------------+ | <- 元数据的大小在超级块中描述
      | |           元数据副本 2        | |    
      | +-----------------------------+ |
      +---------------------------------+ <- 超级块描述切片的开始 
      |                                 |   
      |               切片 1             |
      +---------------------------------+
      |                                 |
      |               切片 2             |
      +---------------------------------+
      |                                 |
      |               切片 3             |
      +---------------------------------+
      |                                 |
```

<!--

The partition table is made of several virtual partition
entries(`VPartitionEntry`). In addition to containing name and partition
identifiers, each of these vpart entries contains the number of allocated
slices for this partition.

-->

分区表由几个虚拟分区条目（`VPartitionEntry`）组成。 
除了包含名称和分区标识符之外，这些 vpart 条目中的每一个都包含为此分区分配的片数。 

<!--

The slice allocation table is made up of tightly packed slice entries
(`SliceEntry`). Each entry contains

 * allocation status
 * if it is allocated,
   * what partition it belongs to and
   * what logical slice within partition the slice maps to

-->

切片分配表由紧密打包的切片条目（`SliceEntry`）组成。 每个条目包含

* 分配状态
* 如果已分配，
  * 它属于哪个分区
  * 切片映射到分区内的逻辑切片 

<!--

FVM library can be found
[here](/src/storage/fvm/). During
[paving](/docs/development/hardware/paving.md),
some partitions are copied from host to target. So the partitions and FVM
file itself may be created on host. To do this there is host side utility
[here](/src/storage/bin/fvm).
Integrity of the FVM device/file can be verbosely verified with
[fvm-check](/src/devices/block/bin/fvm-check)

-->

FVM 库可以在 [这里](/src/storage/fvm/) 找到。 
在 [paving](/docs/development/hardware/paving.md) 中，一些分区从主机复制到目标。 
所以分区和 FVM 文件本身可以在主机上创建。 
为此，有主机端实用程序 [这里](/src/storage/bin/fvm)。 
可以使用 [fvm-check](/src/devices/block/bin/fvm-check) 详细验证 FVM 设备/文件的完整性 
