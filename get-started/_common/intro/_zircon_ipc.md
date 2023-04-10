## Inter-process communication

Since processes are isolated by default, the kernel needs to provide a way for
them to securely communicate with each other. Zircon includes the following
kernel object types for inter-process communication (IPC):

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

Among these objects, channels are uniquely suited to assist in launching new
processes because they are capable of transferring handles (and therefore,
capabilities) across to another process.

Channels have exactly two endpoint handles, each owned by a separate process.
Only the owners may read or write messages, but ownership of an endpoint may
be transferred from one process to another. When handles are written into a
channel, they are removed from the sending process. When a message with handles
is read from a channel, the handles are added to the receiving process.

![Diagram showing how processes communicate through shared objects found in the
 kernel. The most common of these connections is the channel.](
  /get-started/images/intro/ipc.png){: width="582"}

Note: You can find more of Zircon's deep technical details in the
[kernel documentation](/concepts/kernel/README.md).

Zircon channels are the basis for service-level IPC protocols described by
the [Fuchsia Interface Definition Language (FIDL)][glossary.FIDL]. FIDL
protocols are the primary method of IPC used by Fuchsia programs. You will
explore creating and consuming FIDL protocols in more detail later on.


[glossary.FIDL]: /glossary/README.md#FIDL
