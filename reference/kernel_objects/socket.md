# Socket

## NAME

Socket - Bidirectional streaming IPC transport

## SYNOPSIS

Sockets are a bidirectional stream transport. Unlike channels, sockets
only move data (not handles).

## DESCRIPTION

Data is written into one end of a socket via [`zx_socket_write()`] and
read from the opposing end via [`zx_socket_read()`].

Upon creation, both ends of the socket are writable and readable. Via the
**ZX_SOCKET_SHUTDOWN_READ** and **ZX_SOCKET_SHUTDOWN_WRITE** options to
[`zx_socket_shutdown()`], one end of the socket can be closed for reading and/or
writing.

## PROPERTIES

The following properties may be queried from a socket object:

**ZX_PROP_SOCKET_RX_THRESHOLD** size of the read threshold of a socket, in
bytes. When the bytes queued on the socket (available for reading) is equal to
or greater than this value, the **ZX_SOCKET_READ_THRESHOLD** signal is asserted.
Read threshold signalling is disabled by default (and when set, writing
a value of 0 for this property disables it).

**ZX_PROP_SOCKET_TX_THRESHOLD** size of the write threshold of a socket,
in bytes. When the space available for writing on the socket is equal to or
greater than this value, the **ZX_SOCKET_WRITE_THRESHOLD** signal is asserted.
Write threshold signalling is disabled by default (and when set, writing a
value of 0 for this property disables it).

From the point of view of a socket handle, the receive buffer contains the data
that is readable via [`zx_socket_read()`] from that handle (having been written
from the opposing handle), and the transmit buffer contains the data that is
written via [`zx_socket_write()`] to that handle (and readable from the opposing
handle).

## SIGNALS

The following signals may be set for a socket object:

**ZX_SOCKET_READABLE** data is available to read from the socket

**ZX_SOCKET_WRITABLE** data may be written to the socket

**ZX_SOCKET_PEER_CLOSED** the other endpoint of this socket has
been closed.

**ZX_SOCKET_PEER_WRITE_DISABLED** writing is disabled permanently for the other
endpoint either because of passing **ZX_SOCKET_SHUTDOWN_READ** to this endpoint
or passing **ZX_SOCKET_SHUTDOWN_WRITE** to the peer. Reads on a socket endpoint
with this signal raised will succeed so long as there is data in the socket that
was written before writing was disabled.

**ZX_SOCKET_WRITE_DISABLED** writing is disabled permanently for this endpoint
either because of passing **ZX_SOCKET_SHUTDOWN_WRITE** to this endpoint or
passing **ZX_SOCKET_SHUTDOWN_READ** to the peer.

**ZX_SOCKET_READ_THRESHOLD** data queued up on socket for reading exceeds
the read threshold.

**ZX_SOCKET_WRITE_THRESHOLD** space available on the socket for writing exceeds
the write threshold.

## SYSCALLS

 - [`zx_socket_create()`] - create a new socket
 - [`zx_socket_read()`] - read data from a socket
 - [`zx_socket_shutdown()`] - prevent reading or writing
 - [`zx_socket_write()`] - write data to a socket

[`zx_socket_create()`]: /docs/reference/syscalls/socket_create.md
[`zx_socket_read()`]: /docs/reference/syscalls/socket_read.md
[`zx_socket_shutdown()`]: /docs/reference/syscalls/socket_shutdown.md
[`zx_socket_write()`]: /docs/reference/syscalls/socket_write.md
