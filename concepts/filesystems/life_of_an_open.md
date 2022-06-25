# Life of an 'Open'

To provide an end-to-end picture of filesystem access on Fuchsia, this document
dives into the details of each layer used when doing something as
simple as opening a file. It’s important to note: all of these layers exist in
userspace; even when interacting with filesystem servers and drivers, the kernel
is merely used to pass messages from one component to another.

A call is made to:

`open(“foobar”);`

Where does that request go?

## Standard Library: Where 'open' is defined

The ‘open’ call is a function, provided by a [standard library](/docs/concepts/kernel/libc.md). For
C/C++ programs, this will normally be declared in `unistd.h`, which has a
backing definition in [libfdio](/sdk/lib/fdio/).
For Go programs, there is an equivalent (but distinct) implementation in the Go
standard library. For each language and runtime, developers may opt into their
own definition of “open”.

On a monolithic kernel, `open` would be a lightweight shim around a system
call, where the kernel might handle path parsing, redirection, etc. In that
model, the kernel would need to mediate access to resources based on exterior
knowledge about the caller. The Zircon kernel, however, intentionally has no
such system call. Instead, clients access filesystems through **channels** --
when a process is initialized, it is provided a [namespace](/docs/concepts/process/namespaces.md),
which is a table of "absolute path" -> "handle" mappings. All paths accessed
from within a process are opened by directing requests through this namespace
mapping.

In this example, however, involving a request to open “foobar”, a relative path
was used, so the incoming call could be sent over the path representing the
current working directory (which itself is represented as an absolute path
and a handle).

The standard library is responsible for taking a handle (or multiple handles)
and making them appear like file descriptors. As a consequence, the “file
descriptor table” is a notion that exists within a client process (if a client
chooses to use a custom runtime, they can view their resources purely as
handles -- the “file descriptor” wrapping is optional).

This raises a question, however: given a file descriptor to files, sockets,
pipes, etc, what does the standard library do to make all these resources
appear functionally the same? How does that client know what messages to send
over these handles?

## Fdio

A library called [**fdio**](/sdk/lib/fdio/)
is responsible for providing a unified interface to a variety of resources --
files, sockets, services, pipes, and more. This layer defines a group of
functions, such as **read, write, open, close, seek, etc** that may be used on
file descriptors backed by a variety of protocols. Each supported protocol is
responsible for providing client-side code to interpret the specifics of their
interaction. For example, **sockets** provide multiple handles to clients; one
acting for data flow, and one acting as a control plane. In contrast, **files**
typically use only a single channel for control and data (unless extra work has
been done to ask for a memory mapping).  Although both sockets and files might
receive a call to `open` or `write`, they will need to interpret those commands
differently.

For the purposes of this document, we’ll be focusing on the primary protocol
used by filesystem clients: [FIDL](/docs/development/languages/fidl/README.md).

## FIDL

A program calling `open("foo")` will have called into the standard library,
found an “fdio” object corresponding to the current working directory, and will
need to send a request to a remote server to “please open foo”. How can this be
accomplished? The program has the following tools:

  * One or more **handles** representing a connection to the CWD
  * [zx_channel_write](/docs/reference/syscalls/channel_write.md):
    A system call that can send bytes and handles (over a channel)
  * [zx_channel_read](/docs/reference/syscalls/channel_read.md):
    A system call that can receive bytes and handles (over a channel)
  * [zx_object_wait_one](/docs/reference/syscalls/object_wait_one.md):
    A system call that can wait for a handle to be readable / writable

Using these primitives, the client can write a message to the filesystem server
on the CWD handle, which the server can read and then respond to with a
“success” or “failure message” in a write back to the client. While the server
is crunching away, figuring out what to actually open, the client may or may
not choose to wait before trying to read the status message.

It’s important that the client and server agree on the interpretation of those
N bytes and N handles when messages are transmitted or received: if there is
disagreement between them, messages might be dropped (or worse, contorted into
an unintended behavior). Additionally, if this protocol allowed the client to
have arbitrary control over the server, this communication layer would be ripe
for exploitation.

The [FIDL IO protocol](https://fuchsia.dev/reference/fidl/fuchsia.io)
describes the wire-format of what these bytes and handles should actually mean
when transmitted between two entities. The protocol describes things like
“expected number of handles”, “enumerated operation”, and “data”. In our case,
`open("foo")` creates an `Open` message, and sets the “data” field of the FIDL
message to the string “foo”. Additionally, if any flags are passed to
open (such as `O_RDONLY, O_RDWR, O_CREAT`, etc) these flags would be placed in
the “arg” field of the FIDL structure. However, if the operation was changed
(to, for example, `write`), the interpretation of this message would be
altered.

Exact byte agreement at this layer is critical, as it allows communication
between drastically different runtimes: **processes that understand FIDL can
communicate easily between C, C++, Go, Rust, Dart programs (and others)
transparently.**

**libfidl** contains both the client and server-side code for the C/C++
implementation of FIDL, and is responsible for automatically verifying the input
and output of both ends.

In the case of the `open` operation, the FIDL protocol expects that the
client will create a channel and pass one end (as a handle) to the server. Once
the transaction is complete, this channel may be used as the mechanism to
communicate with the opened file, just as there had previously been a
communication with the “CWD” handle.

By designing the protocol so FIDL clients provide handles, rather than servers,
the communication is better suited to pipelining. Access to FIDL objects can be
asynchronous; requests to the FIDL object can be transmitted before the object
is actually opened. This behavior is critical for interaction with services
(which will be described in more detail in the “ServiceFS” section).

To recap, an “open” call has gone through the standard library, acted on the
“CWD” fdio object, which transformed the request into a FIDL message, which is
sent to the server using the `zx_channel_write` system call. The client can
optionally wait for the server’s response using `zx_object_wait_one`, or continue
processing asynchronously. Either way, a channel has been created, where
one end lives with the client, and the other end is transmitted to the
“server".

## Filesystems: Server-Side

### Dispatching

Once the message has been transmitted from the client’s side of the channel, it
lives in the server’s side of the channel, waiting to be read. The server is
identified by “whoever holds the handle to the other end of the channel” -- it
may live in the same (or a different) process as the client, use the same (or a
different) runtime than the client, and be written in the same (or a different
language) than the client. By using an agreed-upon wire-format, the
interprocess dependencies are bottlenecked at the thin communication layer that
occurs over channels.

At some point in the future, this server-side end of the CWD handle will need
to read the message transmitted by the client. This process isn’t automatic --
the server will need to intentionally wait for incoming messages on the
receiving handle, which in this case was the “current working directory”
handle. When server objects (files, directories, services, etc) are opened,
their handles are registered with a server-side Zircon **port** that waits for
their underlying handles to be **readable** (implying a message has arrived) or
**closed** (implying they will never receive more messages). This object, which
dispatches incoming requests to appropriate handles, is known as the dispatcher.
It is responsible for redirecting incoming messages to a callback function,
along with some previously-supplied “iostate” representing the open connection.

For C++ filesystems using libfs, this callback function is called
`vfs_handler`, and it receives a couple key pieces of information:

  * The FIDL message, provided by the client (or artificially constructed
    by the server to appear like a “close” message, if the handle was closed)
  * The I/O state representing the current connection to the handle (passed as the
    “iostate” field, mentioned earlier).

`vfs_handler` can interpret the I/O state to infer additional information:

  * The seek pointer within the file (or within the directory, if readdir has been used)
  * The flags used to open the underlying resource
  * The Vnode, which represents the underlying object (and may be shared between
    multiple clients, or multiple file descriptors)

This handler function, equipped with this information, acts as a large
“switch/case” table, redirecting the FIDL message to an appropriate function
depending on the “operation” field provided by the client. In the open case, the
`Open` ordinal is noticed as the operation, so (1) a handle is expected, and
(2) the ‘data’ field (“foo”) is interpreted as the path.

### VFS Layer

In Fuchsia, the “VFS layer” is a filesystem-independent library of code, which
may dispatch and interpret server-side messages, and call operations in the
underlying filesystem where appropriate. Notably, this layer is completely
optional -- if a filesystem server does not want to link against this library,
they have no obligation to use it. To be a filesystem server, a process must
merely understand the FIDL wire format. As a consequence, there could be
any number of “VFS” implementations in a language. There are currently these
implementations:

  * [In-tree C++ VFS](/src/lib/storage/vfs/cpp): Used by Fuchsia's "main" filesystems minfs and
    blobfs. It currently has the most features of any VFS implementation, but can also be the most
    difficult to use.
  * [In-tree Rust VFS](/src/lib/storage/vfs/rust): This is used by some Rust filesystems including
    the fat32 implementation. It is newer and currently has fewer features than the C++
    implementation.
  * [SDK C++ VFS](/sdk/lib/vfs/cpp): A somewhat simplified version of of the "in-tree" C++ version
    for SDK users. This is most commonly used for simpler uses like service discovery.

The VFS layer defines the interface of operations that may be routed to the
underlying filesystem, including:

  * Read/Write to a Vnode
  * Lookup/Create/Unlink a Vnode (by name) from a parent Vnode
  * Rename/Link a Vnode by name
  * And many more

To implement a filesystem (assuming a developer wants to use the shared VFS
layer), one simply needs to define a Vnode implementing this interface and link
against a VFS layer. This will provide functionality like “path walking” and
“filesystem mounting” with minimal effort, and almost no duplicated code. In an
effort to be filesystem-agnostic, the VFS layer has no preconceived notion of
the underlying storage used by the filesystem: filesystems may require access
to block devices, networks, or simply memory to store data -- but the VFS layer
only deals with interfaces acting on paths, byte arrays of data, and vnodes.

### Path Walking

To open a server-side resource, the server is provided some starting point
(represented by the called handle) and a string path. This path is split into
segments by the “/” character, and each component is “looked up” with a
callback to the underlying filesystem. If the lookup successfully returns a
vnode, and another “/” segment is detected, then the process continues until
(1) `lookup` fails to find a component, (2) path processing reaches the last
component in a path, or (3) `lookup` finds a **mountpoint vnode**, which is a
vnode that has an attached “remote” handle. For now, we will ignore mountpoint
vnodes, although they are discussed in a section on [filesystem
mounting](/docs/concepts/filesystems/filesystems.md#Mounting).

Let’s assume `lookup` successfully found the “foo” Vnode. The filesystem server
will proceed to call the VFS interface “Open”, verifying that the requested
resource can be accessed with the provided flags, before calling “GetHandles”
asking the underlying filesystem if there are additional handles required to
interact with the Vnode. Assuming the client asked for the “foo” object
synchronously (which is implied with the default POSIX open call), any
additional handles required to interact with “foo” are packed into a small FIDL
description object and passed back to the client. Alternatively, if "foo" had
failed to open, a FIDL description object would still be returned, but with the
“status” field set to an error code, indicating failure. Let’s assume the “foo”
open was successful. The server will proceed to create an “iostate” object for
“foo” and register it with the dispatcher. Doing so, future calls to “foo” can
be handled by the server. “Foo” has been opened, the client is now ready to send
additional requests.

From the client’s perspective, at the start of the “Open” call, a path and
handle combination was transmitted over the CWD handle to a remote filesystem
server. Since the call was synchronous, the client proceeded to wait for a
response on the handle. Once the server properly found, opened, and initialized
I/O state for this file, it sent back a “success” FIDL description object. This
object would be read by the client, identifying that the call completed
successfully. At this point, the client could create an fdio object
representing the handle to “foo”, reference it with an entry in a file
descriptor table, and return the fd back to whoever called the original “open”
function. Furthermore, if the client wants to send any additional requests
(such as “read” or “write”) to ‘foo’, then they can communicate directly with
the filesystem server by using the connection to the opened file -- there is no
need to route through the ‘CWD’ on future requests.

## Life of an Open: Diagrams

```
             +----------------+
             | Client Program |
+-----------------------------+
|   fd: x    |   fd: y    |
| Fdio (FIDL)| Fdio (FIDL)|
+-------------------------+
| '/' Handle | CWD Handle |
+-------------------------+
      ^            ^
      |            |
Zircon Channels, speaking FIDL                   State BEFORE open(‘foo’)
      |            |
      v            v
+-------------------------+
| '/' Handle | CWD Handle |
+-------------------------+
|  I/O State |  I/O State |
+-------------------------+
|   Vnode A  |   Vnode B  |
+------------------------------+
           | Filesystem Server |
           +-------------------+


             +----------------+
             | Client Program |
+-----------------------------+
|   fd: x    |   fd: y    |
| Fdio (FIDL)| Fdio (FIDL)|
+-------------------------+
| '/' Handle | CWD Handle |   **foo Handle x2**
+-------------------------+
      ^            ^
      |            |
Zircon Channels, speaking FIDL                   Client Creates Channel
      |            |
      v            v
+-------------------------+
| '/' Handle | CWD Handle |
+-------------------------+
|  I/O State |  I/O State |
+-------------------------+
|   Vnode A  |   Vnode B  |
+------------------------------+
           | Filesystem Server |
           +-------------------+


             +----------------+
             | Client Program |
+-----------------------------+
|   fd: x    |   fd: y    |
| Fdio (FIDL)| Fdio (FIDL)|
+-------------------------+--------------+
| '/' Handle | CWD Handle | ‘foo’ Handle |
+-------------------------+--------------+
      ^            ^
      |            |
Zircon Channels, speaking FIDL                  Client Sends FIDL message to Server
      |            |                            Message includes a ‘foo’ handle
      v            v                            (and waits for response)
+-------------------------+
| '/' Handle | CWD Handle |
+-------------------------+
|  I/O State |  I/O State |
+-------------------------+
|   Vnode A  |   Vnode B  |
+------------------------------+
           | Filesystem Server |
           +-------------------+


             +----------------+
             | Client Program |
+-----------------------------+
|   fd: x    |   fd: y    |
| Fdio (FIDL)| Fdio (FIDL)|
+-------------------------+--------------+
| '/' Handle | CWD Handle | ‘foo’ Handle |
+-------------------------+--------------+
      ^            ^
      |            |
Zircon Channels, speaking FIDL                  Server dispatches message to I/O State,
      |            |                            Interprets as ‘open’
      v            v                            Finds or Creates ‘foo’
+-------------------------+
| '/' Handle | CWD Handle |
+-------------------------+
|  I/O State |  I/O State |
+-------------------------+-------------+
|   Vnode A  |   Vnode B  |   Vnode C   |
+------------------------------+--------+
           | Filesystem Server |
           +-------------------+


             +----------------+
             | Client Program |
+-----------------------------+
|   fd: x    |   fd: y    |
| Fdio (FIDL)| Fdio (FIDL)|
+-------------------------+--------------+
| '/' Handle | CWD Handle | ‘foo’ Handle |
+-------------------------+--------------+
      ^            ^          ^
      |            |          |
Zircon Channels, FIDL         |                   Server allocates I/O state for Vnode
      |            |          |                   Responds to client-provided handle
      v            v          v
+-------------------------+--------------+
| '/' Handle | CWD Handle | ‘foo’ Handle |
+-------------------------+--------------+
|  I/O State |  I/O State |  I/O State   |
+-------------------------+--------------+
|   Vnode A  |   Vnode B  |    Vnode C   |
+------------------------------+---------+
           | Filesystem Server |
           +-------------------+


             +----------------+
             | Client Program |
+-----------------------------+----------+
|   fd: x    |   fd: y    |    fd: z     |
| Fdio (FIDL)| Fdio (FIDL)|  Fdio (FIDL) |
+-------------------------+--------------+
| '/' Handle | CWD Handle | ‘foo’ Handle |
+-------------------------+--------------+
      ^            ^           ^
      |            |           |
Zircon Channels, speaking FIDL |                  Client recognizes that ‘foo’ was opened
      |            |           |                  Allocated Fdio + fd, ‘open’ succeeds.
      v            v           v
+-------------------------+--------------+
| '/' Handle | CWD Handle | ‘foo’ Handle |
+-------------------------+--------------+
|  I/O State |  I/O State |  I/O State   |
+-------------------------+--------------+
|   Vnode A  |   Vnode B  |    Vnode C   |
+------------------------------+---------+
           | Filesystem Server |
           +-------------------+
```
