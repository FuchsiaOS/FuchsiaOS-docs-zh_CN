# Filesystem Architecture

This document seeks to describe a high-level view of the Fuchsia filesystems,
from their initialization, discussion of standard filesystem operations (such as
Open, Read, Write, etc), and the quirks of implementing user-space filesystems.
Additionally, this document describes the VFS-level walking through a namespace,
which can be used to communicate with non-storage entities (such as system
services).

## Filesystems are Services

Unlike more common monolithic kernels, Fuchsia’s filesystems live entirely
within userspace. They are not linked nor loaded with the kernel; they are
simply userspace processes that implement servers that can appear as
filesystems. As a consequence, Fuchsia’s filesystems themselves can be changed
with ease -- modifications don’t require recompiling the kernel.

![Filesystem block diagram](images/filesystem.svg "filesystem")

Figure 1: Typical filesystem process block diagram.

Like other native servers on Fuchsia, the primary mode of interaction with a
filesystem server is achieved using the handle primitive rather than system
calls. The kernel has no knowledge about files, directories, or filesystems. As
a consequence, filesystem clients cannot ask the kernel for “filesystem access”
directly.

This architecture implies that the interaction with filesystems is limited to
the following interface:

 * The messages sent on communication channels established with the filesystem
   server. These communication channels may be local for a client-side
   filesystem, or remote.
 * The initialization routine (which is expected to be configured heavily on a
   per-filesystem basis; a networking filesystem would require network access,
   persistent filesystems may require block device access, in-memory filesystems
   would only require a mechanism to allocate new temporary pages).

As a benefit of this interface, any resources accessible via a channel can make
themselves appear like filesystems by implementing the expected protocols for
files or directories. For example, components serve their
[outgoing directory][glossary.outgoing-directory], which contains their
[capabilities][capabilities-overview], in a filesystem-like structure.

## File Lifecycle

### Establishing a Connection

To open a file, Fuchsia programs (clients) send RPC requests to filesystem
servers using a FIDL.

FIDL defines the wire-format for transmitting messages and handles between a
filesystem client and server. Instead of interacting with a kernel-implemented
VFS layer, Fuchsia processes send requests to filesystem services which
implement protocols for Files, Directories, and Devices. To send one of these
open requests, a Fuchsia process must transmit an RPC message over an existing
handle to a directory; for more detail on this process, refer to the [life of an
open document](/concepts/filesystems/life_of_an_open.md).

### Namespaces

On Fuchsia, a [namespace](/concepts/process/namespaces.md) is a small filesystem that exists
entirely within the client. At the most basic level, the idea of the client
saving “/” as root and associating a handle with it is a very primitive
namespace. Instead of a typical singular "global" filesystem namespace, Fuchsia
processes can be provided an arbitrary directory handle to represent "root",
limiting the scope of their namespace. In order to limit this scope, Fuchsia
filesystems [intentionally do not allow access to parent directories via
dotdot](/concepts/filesystems/dotdot.md).

Fuchsia processes may additionally redirect certain path operations to separate
filesystem servers. When a client refers to “/bin”, the client may opt to
redirect these requests to a local handle representing the “/bin” directory,
rather than sending a request directly to the “bin” directory within the “root”
directory. Namespaces, like all filesystem constructs, are not visible from the
kernel: rather, they are implemented in client-side runtimes (such as
[libfdio](/concepts/filesystems/life_of_an_open.md#Fdio)) and are interposed between most client code
and the handles to remote filesystems.

Since namespaces operate on handles, and most Fuchsia resources and services
are accessible through handles, they are extremely powerful concepts.
Filesystem objects (such as directories and files), services, devices,
packages, and environments (visible by privileged processes) all are usable
through handles, and may be composed arbitrarily within a child process. As a
result, namespaces allow for customizable resource discovery within
applications. The services that one process observes within “/svc” may or may
not match what other processes see, and can be restricted or redirected
according to application-launching policy.

For more detail the mechanisms and policies applied to restricting process
capability, refer to the documentation on
[sandboxing](/concepts/process/sandboxing.md).

### Passing Data

Once a connection has been established, either to a file, directory, device,
or service, subsequent operations are also transmitted using RPC messages.
These messages are transmitted on one or more handles, using a wire format that
the server validates and understands.

In the case of files, directories, devices, and services, these operations use the
FIDL protocol.

As an example, to seek within a file, a client would send a `Seek`
message with the desired position and “whence” within the FIDL message, and the
new seek position would be returned. To truncate a file, a `Truncate`
message could be sent with the new desired filesystem, and a status message
would be returned. To read a directory, a `ReadDirents` message could be
sent, and a list of direntries would be returned. If these requests were sent to
a filesystem entity that can’t handle them, an error would be sent, and the
operation would not be executed (like a `ReadDirents` message sent to a text
file).

### Memory Mapping

For filesystems capable of supporting it, memory mapping files is slightly more
complicated. To actually “mmap” part of a file, a client sends an “GetVmo”
message, and receives a Virtual Memory Object, or VMO, in response. This object
is then typically mapped into the client’s address space using a Virtual Memory
Address Region, or VMAR. Transmitting a limited view of the file’s internal
“VMO” back to the client requires extra work by the intermediate message
passing layers, so they can be aware they’re passing back a server-vendored
object handle.

By passing back these virtual memory objects, clients can quickly access the
internal bytes representing the file without actually undergoing the cost of a
round-trip IPC message. This feature makes mmap an attractive option for
clients attempting high-throughput on filesystem interaction.

At the time of writing, on-demand paging is not supported by the
kernel, and has not been wired into filesystems. As a result, if a client
writes to a “memory-mapped” region, the filesystem cannot reasonably identify
which pages have and have not been touched. To cope with this restriction, mmap
has only been implemented on **read-only filesystems**, such as blobfs.

### Other Operations acting on paths

In addition to the “open” operation, there are a couple other path-based
operations worth discussing: “rename” and “link”. Unlike “open”, these
operations actually act on multiple paths at once, rather than a single
location. This complicates their usage: if a call to “rename(‘/foo/bar’,
‘baz’)” is made, the filesystem needs to figure out a way to:

  * Traverse both paths, even when they have distinct starting points (which is the
    case this here; one path starts at root, and other starts at the CWD)
  * Open the parent directories of both paths
  * Operate on both parent directories and trailing pathnames simultaneously

To satisfy this behavior, the VFS layer takes advantage of a Zircon concept
called “cookies”. These cookies allow client-side operations to store open
state on a server, using a handle, and refer to it later using that same
handles. Fuchsia filesystems use this ability to refer to one Vnode while
acting on the other.

These multi-path operations do the following:

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

## Filesystem Lifecycle

### Mounting

Fshost is responsible for mounting filesystems on the system. At time of
writing, changes are underway to make filesystems run as components (although
fshost will still control mounting of these filesystems). Where possible,
static routing will be used. See the fuchsia.fs.startup/Startup protocol.

### Filesystem Management

There are a collection of filesystem operations that are considered related to
"administration", including "unmounting the current filesystem".  These
operations are defined by the fs.Admin interface within
[admin.fidl](/sdk/fidl/fuchsia.fs/admin.fidl). Filesystems export this service
alongside access to the root of the filesystem.

## Current Filesystems

Due to the modular nature of Fuchsia’s architecture, it is straightforward to
add filesystems to the system. At the moment, a handful of filesystems exist,
intending to satisfy a variety of distinct needs.

### MemFS: An in-memory filesystem

[MemFS](/src/storage/memfs)
is used to implement requests to temporary filesystems like `/tmp`, where files
exist entirely in RAM, and are not transmitted to an underlying block device.
This filesystem is also currently used for the “bootfs” protocol, where a
large, read-only VMO representing a collection of files and directories is
unwrapped into user-accessible Vnodes at boot (these files are accessible in
`/boot`).

### MinFS: A persistent filesystem

[MinFS](/src/storage/bin/minfs/)
is a simple, traditional filesystem that is capable of storing files
persistently. Like MemFS, it makes extensive use of the VFS layers mentioned
earlier, but unlike MemFS, it requires an additional handle to a block device
(which is transmitted on startup to a new MinFS process). For ease of use,
MinFS also supplies a variety of tools: “mkfs” for formatting, “fsck” for
verification, as well as “mount” and “umount” for adding and subtracting MinFS
filesystems to a namespace from the command line.

### Blobfs: An immutable, integrity-verifying package storage filesystem

[Blobfs](/src/storage/bin/blobfs/)
is a simple, flat filesystem optimized for “write-once, then read-only” [signed
data](/concepts/packages/merkleroot.md), such as
[packages](/concepts/packages/package.md).
Other than two small prerequisites (file names, which are deterministic, content
addressable hashes of a file’s Merkle Tree root, for integrity-verification)
and forward knowledge of file size (identified to Blobfs by a call to
“ftruncate” before writing a blob to storage), Blobfs appears like a
typical filesystem. It can be mounted and unmounted, it appears to contain a
single flat directory of hashes, and blobs can be accessed by operations like
“open”, “read”, “stat” and “mmap”.

### FVM

[Fuchsia Volume Manager](/src/storage/fvm/driver/)
is a "logical volume manager" that adds flexibility on top of existing block
devices. The current features include ability to add, remove, extend and
shrink virtual partitions. To make these features possible FVM internally
maintains physical to virtual  mapping from (virtual partitions, blocks) to
(slice, physical block). To keep maintenance overhead minimal, it allows
partitions to shrink/grow in chunks called slices. A slice is a multiple of the
native block size. Metadata aside, the rest of the device is divided into
slices. Each slice is either free or it belongs to one and only one partition.
If a slice belongs to a partition, FVM maintains metadata about which
partition is using the slice, and the virtual address of the slice within
that partition.

The on-disk layout of the FVM looks like the following, and is declared
[here](/src/storage/fvm/format.h#27).

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

The partition table is made of several virtual partition
entries (`VPartitionEntry`). In addition to containing name and partition
identifiers, each of these vpart entries contains the number of allocated
slices for this partition.

The slice allocation table is made up of tightly packed slice entries
(`SliceEntry`). Each entry contains

 * allocation status
 * if it is allocated,
   * what partition it belongs to and
   * what logical slice within the partition the slice maps to

FVM library can be found
[here](/src/storage/fvm/). During
[paving](/development/build/fx.md#what-is-paving),
some partitions are copied from host to target. So the partitions and FVM
file itself may be created on host. To do this there is host side utility
[here](/src/storage/bin/fvm).
Integrity of the FVM device/file can be verbosely verified with
[fvm-check](/src/devices/block/bin/fvm-check)

[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[capabilities-overview]: /concepts/components/v2/capabilities/README.md
