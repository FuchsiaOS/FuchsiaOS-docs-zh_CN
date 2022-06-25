<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0109" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Implement network datagram socket data transport using zircon sockets.
Implement client-side argument validation using client-cached network state.

## Motivation

Increase datagram socket throughput and reduce CPU utilization.

Prior to <https://fxbug.dev/21123> datagram network sockets were implemented
using zircon sockets; clients would `zx_socket_write` to send data and
`zx_socket_read` to receive data. A minimal protocol was used to carry metadata
such as the destination address, if one was provided by the calling
application.

This approach was abandoned because it did not provide for error signaling to
the caller in certain cases. Consider an application that wishes to send a
payload to a remote to which the network stack does not have a route; in order
to comply with the expectations of third party software targeting Linux, the
implementation must return an error to indicate that no action was taken on
behalf of the caller.

The current implementation is defined in
[`fuchsia.posix.socket/DatagramSocket.{Recv,Send}Msg`][fsocket].
Both sending and receiving data requires multiple context switches due to
support FIDL responses. Ignoring FIDL serialization:

```
+-----------------------------+-------------------------+
| Client                      | Netstack                |
+-----------------------------+-------------------------+
| `zx_channel_call(...)`      |                         |
|                             |                         |
|                             | `zx_channel_read(...)`  |
|                             | `zx_channel_write(...)` |
|                             |                         |
| (`zx_channel_call` returns) |                         |
+-----------------------------+-------------------------+
```

The reverse (receiving data) looks much the same. Compare this to
unacknowledged I/O:

```
+-------------------------+------------------------+
| Client                  | Netstack               |
+-------------------------+------------------------+
| `zx_channel_write(...)` |                        |
+-------------------------+------------------------+
| `zx_channel_write(...)` | `zx_channel_read(...)` |
+-------------------------+------------------------+
```

While unacknowledged I/O is possible with zircon channels and FIDL, it does not
provide backpressure, and can lead to runaway memory growth. Thus, we propose
to use zircon *sockets*:

```
+------------------------+-----------------------+
| Client                 | Netstack              |
+------------------------+-----------------------+
| `zx_socket_write(...)` |                       |
+------------------------+-----------------------+
| `zx_socket_write(...)` | `zx_socket_read(...)` |
+------------------------+-----------------------+
```

## Design

### Metadata Validation

Utilizing unacknowledged I/O presupposes that datagram socket metadata (e.g.
destination address) can be entirely validated on the local machine without
interaction with the network, and that the results of this validation can be
cached across multiple interactions with the socket (so that the validation cost
can be amortized).

Note that this assumption notably does not hold for `IPPROTO_ICMP` sockets -
their payloads are checked for validity - so the existing FIDL-based protocol
will be retained and used where performance is not critical and deep
validation is required.

Extract FIDL from existing types to be reused and rename `DatagramSocket` for
clarity:

```fidl
protocol BaseDatagramSocket {
  compose BaseSocket;

  /// Retrieves creation information from the socket.
  GetInfo() -> (Domain domain, DatagramSocketProtocol proto) error fuchsia.posix.Errno;
}

protocol SynchronousDatagramSocket {
  compose BaseDatagramSocket;

  /// Receives a message from the socket.
  RecvMsg(bool want_addr, uint32 data_len, bool want_control, RecvMsgFlags flags) -> (fuchsia.net.SocketAddress? addr, bytes data, RecvControlData control, uint32 truncated) error fuchsia.posix.Errno;
  /// Sends a message on the socket.
  SendMsg(fuchsia.net.SocketAddress? addr, bytes:MAX data, SendControlData control, SendMsgFlags flags) -> (int64 len) error fuchsia.posix.Errno;
}
```

Define the new FIDL protocol with validation functions:

```fidl
/// Matches the definition in //zircon/system/public/zircon/types.h.
const uint32 ZX_WAIT_MANY_MAX_ITEMS = 64;

/// Describes an intent to send data.
type SendMsgArguments = table {
  /// The destination address.
  ///
  /// If absent, interpreted as the method receiver's connected address and
  /// causes the connected address to be returned in [`SendMsgBoardingPass.to`].
  ///
  /// Required if the method receiver is not connected.
  1: to fuchsia.net.SocketAddress;
}

/// Describes a granted approval to send data.
type SendMsgBoardingPass = resource table {
  /// The validated destination address.
  ///
  /// Present only in response to an unset
  /// [`SendMsgArguments.to`].
  1: to fuchsia.net.SocketAddress;
  /// Represents the validity of this structure.
  ///
  /// The structure is invalid if any of the elements' peer is closed.
  /// Datagrams sent to the associated destination after invalidation will be
  /// silently dropped.
  2: validity vector<zx.handle:<EVENTPAIR, zx.RIGHTS_BASIC>>:ZX_WAIT_MANY_MAX_ITEMS;
  /// The maximum datagram size that can be sent.
  ///
  /// Datagrams exceeding this will be silently dropped.
  3: maximum_size uint32;
}

protocol DatagramSocket {
  compose BaseDatagramSocket;

  /// Validates that data can be sent.
  ///
  /// + request `args` the requested disposition of data to be sent.
  /// - response `pass` the constraints sent data must satisfy.
  /// * error the error code indicating the reason for validation failure.
  SendMsgPreflight(SendMsgArguments args) -> (SendMsgBoardingPass pass) error fuchsia.posix.Errno;
};
```

Define FIDL structures to be sent on the zircon socket:

```fidl
/// A datagram received on a network socket, along with its metadata.
type RecvMsgPayload = table {
  1: from fuchsia.net.SocketAddress;
  2: control RecvControlData;
  3: datagram bytes:MAX;
};

/// A datagram to be sent on a network socket, along with its metadata.
type SendMsgPayload = table {
  1: args SendMsgArguments;
  2: control SendControlData;
  3: datagram bytes:MAX;
};
```

Note: The actual data format and serialization protocol is bespoke and
specified in detail under [Serialization Protocol](#serialization-protocol). In
this section, it's sufficient to presuppose the above format and serialization
using [FIDL at rest][fidl-at-rest].

Clients attempting to send data follow a procedure depicted by the following
(horizontally squished) diagram:

```
+--------------------------------+               +---------------------------+               +----------------+
| cache.getSendMsgBoardingPass() | - Present ->  | checkPeerClosed(validity) |   +- ZX_OK -> | Return success |
+--------------------------------+               +---------------------------+   |           +----------------+
 |    ^                                            ^                     |  |    |
 |    |                                            |                     |  |  +------------------------------+
 |  +------+                  +----------------------------------+       |  |  | socket.write(SendMsgPayload) | - != ZX_OK -----+
 |  | Send |    +- Success -> | cache.storeSendMsgBoardingPass() |       |  |  +------------------------------+                 |
 |  +------+    |             +----------------------------------+       |  |                         ^                         |
 |            +--------------------+                                     |  |                         |                         |
 +- Absent -> | SendMsgPreflight() |  +- (ZX_OK, ZX_SIGNAL_PEER_CLOSED) -+  +- (ZX_ERR_TIMED_OUT) -+  |                         |
              +--------------------+  |                                                            |  +- No -+                  |
                |                ^    |   +-----------------------------------+                    |         |                  |
                |                |    +-> | cache.removeSendMsgBoardingPass() |                    |   +---------------------+  |
                |                |        +-----------------------------------+                    +-> | size > maximum_size |  |
                |                |          |                                                          +---------------------+  |
                |                |          |  +--------------+                                              |                  |
                |                +----------+  |              | <-------------------------------------- Yes -+                  |
                |                              | Return error |                                                                 |
                +- Failure ------------------> |              | <---------------------------------------------------------------+
                                               +--------------+
```

Where the client's `cache` "implements" `SendMsgPreflight`; it is roughly a map
from `fuchsia.net.SocketAddress` to `(vector<zx::eventpair>, maximum_size)`.

Note that the cache formed by this strategy is eventually-consistent; it is
possible for invalidation to interleave a client's validity check and its
payload arriving at the network stack. This is acceptable for datagram sockets
whose delivery semantics are best-effort.

### Guaranteeing Synchronous Behavior

#### Background

Under POSIX semantics, clients expect the system to provide synchronous `send`
and `recv` behavior with respect to other client-initiated changes to
incoming/outgoing packet processing. Specifically, clients expect that those
changes apply to ALL OF the payloads not yet received and to NONE OF the
payloads already sent.

In the current implementation, synchronous behavior is guaranteed because:

1. Netstack is responsible for all relevant packet processing.
1. Clients send and receive payloads via synchronous FIDL calls.
1. Clients request changes to packet processing by setting socket options,
   which are also implemented as synchronous FIDL calls.

Replacing synchronous FIDL calls with unacknowledged I/O over a zircon socket
transport breaks these semantics. The resulting problems and solutions are
described below.

#### Send Path

##### Problem

On the **Send Path**, clients modify how payloads are processed by setting a
socket option. Because `setsockopt` is implemented via a synchronous FIDL, the
new value of the option might be applied to packets enqueued before the call was
made. For example:

1. Client sets `IP_TOS` to some value A, synchronously updating state
   in Netstack.
1. Client calls `sendto`, which enqueues a payload in the zircon socket.
1. Client sets `IP_TOS` to some value B.
1. Netstack dequeues the payload and sends it out onto the network with
   `IP_TOS=B`.

##### Solution

Amend the FIDL handlers used to set socket options relevant to the send
path to block until the zircon socket has been drained by the Netstack's
dequeuing goroutine. Afterwards, the handler modifies the relevant state and
returns to the client. Since all payloads sent prior to the call to `setsockopt`
have already been dequeued from the zircon socket into the Netstack, none of
them will be processed with the new setting.

Note: This FIDL call could hang indefinitely in the pathological scenario
in which a client uses one thread to set an option while another enqueues data
without pause. It's not clear whether this usage exists in the wild, so we don't
intend to implement a fix on the first pass. If that changes, one readily
available solution would be to disable writes on the socket with
`zx_socket_set_disposition` before blocking.

#### Recv Path

##### Problem

On the **Recv Path**, clients can request control messages that provide
ancillary data about the payload and its delivery by setting a socket option.
Again, because `setsockopt` is synchronous, there is room for skew. For example:

1. Netstack enqueues a payload into the zircon socket.
1. Client sets `IP_RECVTOS`.
1. Client dequeues a payload and returns it to the user without the `IP_TOS`
   control message.

##### Solution

In the Netstack, enqueue each payload alongside the minimal state needed to
derive any of the supported control messages.

Define a FIDL method to retrieve the current set of requested control messages:

```fidl
protocol DatagramSocket {
  compose BaseDatagramSocket;

  /// Returns the set of requested control messages.
  ///
  /// - response `cmsg_set` the set of currently requested control messages.
  RecvMsgPostflight() -> (struct {
    cmsg_set @generated_name("RequestedCmsgSet") table {
      /// Represents the validity of this structure.
      ///
      /// The structure is invalid if the peer is closed.
      1: validity zx.handle:<EVENTPAIR, zx.RIGHTS_BASIC>;
      /// Identifies whether the `IP_RECVTOS` control message is requested.
      2: ip_recvtos bool;
    }
  });
};
```

In the client, cache the set of currently requested control messages and use
that set to filter the control message state passed by the netstack over the
socket, according to the following procedure:

```
+-----------------------------------------+
| socket.read() -> (Payload, RecvMsgMeta) | -----------> ZX_OK
+-----------------------------------------+                |
  |                                                        |
  |                    +-----------------------------+     |
  |                    | cache.getRequestedCmsgSet() | <---+
  |                    +-----------------------------+
  |                               |    |
  |                               |    |
  |                               |    |
  |  Absent  <--------------------+    +-----------------------------> Present
  |    |                                                                   |
  |    |  +-----------------------+                                        |
  |    |  | Return Payload, cmsgs |     +---------------------------+      |
  |    |  +-----------------------+     | checkPeerClosed(validity) |<-----+
  |    |           ^                    +---------------------------+
  |    |           |                      |          |         ^
  |    |           |                      |          |         |
  |    |   (ZX_ERR_TIMED_OUT) <-----------+          |         |
  |    |                                             |         |
  |    |                                             |         |
  |    |     +--(ZX_OK, ZX_SIGNAL_PEER_CLOSED) <-----+         |
  |    |     |                                                 |
  |    |     |                                                 |
  |    |     |      +--------------------------------+         |
  |    |     +--->  | cache.removeRequestedCmsgSet() |         |
  |    |            +--------------------------------+         |
  |    |                                 |                     |
  |    |                                 |                     |
  |    |    +---------------------+      |                     |
  |    +--> | RecvMsgPostflight() |  <---+                     |
  |         +---------------------+                            |
  |                     |      |                               |
  |                     |      |                +------------------------------+
  |  +-------Failure <--+      +--> Success --> | cache.storeRequestedCmsgSet()|
  |  |                                          +------------------------------+
  |  |
  |  |
  |  |    +--------------+
  |  +--> |              |
  |       | Return error |
  +-----> |              |
          +--------------+
```

Adding control message state to each payload adds overhead when copying
memory into and out of the zircon socket. We believe this is an acceptable
tradeoff for two reasons:

1. The control messages we're likely to support can fit in ~100 bytes.
   This is a copy overhead of < 10%, assuming an MTU of ~1500 bytes.
1. The vast majority of control message state is "per-packet", meaning that
   the Netstack holds it in memory alongside each packet and frees that memory
   once the packet is enqueued into the socket. For this reason, the system's
   overall memory consumption should not increase.

Additionally, we will use a microbenchmark to track relevant metrics and run
it whenever a new control message is added. If and when the results suggest
that the tradeoff is no longer worth making, we can revert to the slow path
(which we'll need to keep around anyway in order to support ICMP datagrams).

### Serialization Protocol

The simplest way to perform I/O over the zircon socket is to define a single
FIDL table to hold the UDP payload and its metadata and serialize it using [FIDL
at rest][fidl-at-rest]. The disadvantage of this method is that it forces the
sender and receiver to serialize the payload and metadata into a temporary
buffer, guaranteeing at least one copy of both.

Since [vectorized socket
reads and writes](https://fuchsia-review.googlesource.com/c/fuchsia/+/526346/)
are likely to become available soon, it's preferable to construct a protocol
that can avoid that copy by taking advantage of the vectorized APIs.

#### Protocol

Define FIDL tables for `send` and `recv` metadata:

```fidl
/// Metadata used when receiving a datagram payload.
type RecvMsgMeta = table {
  1: fuchsia.net.SocketAddress from;
  2: RecvControlData control;
};

/// Metadata used when sending a datagram payload.
type SendMsgMeta = table {
  1: SendMsgArguments args;
  2: SendControlData control;
};
```

In the [`fuchsia.io/NodeInfo`][fuchsia-io-NodeInfo] returned to the client in
`Describe`, specify the size of the buffers used to receive the bytes containing
the serialized metadata tables:

```fidl
type NodeInfo = strict resource union {
  // Other variants omitted.
  9: datagram_socket resource struct {
    /// See [`fuchsia.posix.socket.DatagramSocket`] for details.
    socket zx.handle:<SOCKET, zx.rights.TRANSFER | zx.RIGHTS_IO | zx.rights.WAIT | zx.rights.INSPECT>;
    /// Size of the buffer used to receive Tx metadata.
    tx_meta_buf_size uint64;
    /// Size of the buffer used to receive Rx metadata.
    rx_meta_buf_size uint64;
  };
};
```

Let:

```
tx_meta_bytes = fidl_at_rest::serialize(SendMsgMeta)
tx_meta_size = len(tx_meta_bytes)
```

When sending a payload, the client constructs and enqueues the following buffer:

```
      ( 2 )       (tx_meta_size)   (tx_meta_buf_size - tx_meta_size)
+--------------+-----------------+----------------------------------+---------+
| tx_meta_size |  tx_meta_bytes  |             Padding              | Payload |
+--------------+-----------------+----------------------------------+---------+
```

The Netstack:

* Allocates a buffer with room for `2 + tx_meta_buf_size + max_payload_size`
  bytes and dequeues a message into that buffer.
* Interprets the first two bytes as a `uint16` identifying the size of the
  region of the buffer holding the serialized metadata, in bytes.
* Deserializes the `SendMsgMeta` using [FIDL at rest][fidl-at-rest].

The receive path works in an exactly symmetrical manner.

Note: Once vectorized I/O lands, the client will instead make a vectorized write
from two buffers (one holding the metadata size + metadata bytes + padding, and
the other holding the payload). The Netstack then makes a vectorized read into
two buffers. In this way, the payload can be copied directly from/to its
original/final destination into/from the zircon socket.

Note: The Netstack needs to be able to guarantee that `tx_meta_buf_size` and
`rx_meta_buf_size` can fit the maximum possible sizes of the corresponding
serialized FIDL tables. This means that those tables must have bounded size and
therefore that all table members have bounded size. Given that assumption,
the Netstack can compute an upper bound for each message using FIDL's
[MaxSizeInChannel][max-size-in-channel]
method:

```cpp
uint32_t tx_meta_buf_size = fidl::MaxSizeInChannel<fuchsia_posix_socket::wire::SendMsgMeta, fidl::MessageDirection::kSending>();
```

which computes a bound on a message's size in the sending direction, assuming
that no unknown fields are present. The Netstack can safely assume that all
fields in `RecvMsgMeta` are known because the Netstack itself serializes that
message. It can assume that all fields in `SendMsgMeta` are known because:

1. The client's ABI revision [circumscribes][abi-revision] the set of all fields
   in client-serialized tables.
1. The platform will [refuse][package-abi-revision] to run a client if it
   doesn't support the client's ABI revision.
1. The Netstack will always be built with the platform. This assumption has
   been silently relied upon throughout the existing system. Here, we note it
   explicitly.

## Implementation

Add the new implementation to [`fuchsia.io/NodeInfo`][fuchsia-io-NodeInfo]:

```fidl
resource union NodeInfo {
    /// The connection composes [`fuchsia.posix.socket/DatagramSocket`].
    N: DatagramSocket datagram_socket;
};

/// A [`NodeInfo`] variant.
// TODO(https://fxbug.dev/74683): replace with an anonymous struct inline.
resource struct DatagramSocket {
    zx.handle:<SOCKET, zx.RIGHTS_BASIC | zx.RIGHTS_IO> socket;
};
```

Change the return type of
[`fuchsia.posix.socket/Provider.DatagramSocket`][fsocket-provider-dgram-sock]
to a variant:

```fidl
/// Contains a datagram socket implementation.
resource union DatagramSocketImpl {
    1: DatagramSocket datagram_socket;
    2: SynchronousDatagramSocket synchronous_datagram_socket;
}
```

...and change the behavior such that a `DatagramSocket` is returned whenever
permitted by the arguments (i.e. the caller did not request an ICMP socket).

The initial implementation is expected to supply two elements in each
`SendMsgBoardingPass.validity`:

1. Represents a known state of the routing table; shared by all sockets and
   invalidated on any change to the routing table.
2. Represents a known state of the particular socket; invalidated on any change
   to the socket that may change the socket's behavior e.g. calls to `bind`,
   `connect`, `setsockopt(..., SO_BROADCAST, ...)`,
   `setsockopt(..., SO_BINDTODEVICE, ...)`, etc.

## Performance

Throughput of `SOCK_DGRAM` sockets is expected to approximately double; this
estimate is based on the performance regression seen after
<https://fxbug.dev/21123>.

CPU utilization is expected to decrease by a meaningful but unknown magnitude.

## Ergonomics

This change does not have meaningful impact on ergonomics as downstream users do
not directly consume the interfaces presented here.

## Backwards Compatibility

Preserve ABI compatibility by initially leaving
[`fuchsia.posix.socket/Provider.DatagramSocket`][fsocket-provider-dgram-sock]
unchanged and implementing the new functionality as `DatagramSocket2`.
Following necessary ABI transition, rename `DatagramSocket2` to
`DatagramSocket` and remove the previous implementation. Following another ABI
transition, remove `DatagramSocket2`.

## Security considerations

This proposal has no impact on security.

## Privacy considerations

This proposal has no impact on privacy.

## Testing

Existing unit tests cover the functionality affected.

## Documentation

No documentation is necessary apart from FIDL doc comments presented here.

## Drawbacks, alternatives, and unknowns

This proposal addresses the motivation by building machinery in userspace.
Another possibility is to build this machinery in the kernel.

A sketch for translating it to kernel:

1. With each `zx::socket` endpoint, the kernel would maintain a map from
   `SocketAdddres` to `max_size`.
1. We'd add some `zx_socket_add_route` / `zx_socket_remove_route` system calls
   for modifying that map on the peer endpoint.
1. We'd add some `zx_socket_send_to` / `zx_socket_receive_from` system calls
   that would consume/provide addresses.

If userspace called `zx_socket_send_to` with an address that wasn't in the map,
the operation would fail and userspace would need to send a synchronous message
to the netstack to request that route be added to the `zx::socket`. If that
request failed, then the address operation fails with an error.

### Pros

In the kernel approach, sending a UDP packet (in the fast case) is a single
syscall (`zx_socket_send_to`) rather than two syscalls (`zx_object_wait_many`,
`zx_socket_write`).

This is potentially a non-pro because of possible optimizations to the
userspace approach. Realizing that we always `zx_object_wait_many` with
`time::infinite_past`, we could optimized the operation to do its work without
a system call, provided that the necessary state is maintained using atomic
operations. This might require the handle table to be in the vDSO as well,
which may not be the case and may not be possible.

An alternative for clients with runtimes is to use `zx_object_wait_async`
instead of `wait_many` to maintain the local cache, allowing the fast path to
avoid the extra syscall.

We also avoid the dependency on [FIDL at rest][fidl-at-rest] *and* the
extra data copy inherent in FIDL because the message payload is baked into the
system calls, which can copy the payload directly to the final destination.

### Cons

In the kernel approach, there isn't an obvious way to do O(1) route
cancellation when the routing table changes. As described, we could add a flag
to `zx_socket_remove_route` that removes all the routes (probably desirable
anyway), but the netstack would need to issue a `zx_socket_remove_route` on
every socket.

We could get super fancy and have `zx_socket_add_route` take an eventpair for
cancellation, but that's getting pretty baroque.

Baking these structures into the kernel is also costly in terms of introducing
yet another protocol evolution model to the platform; we'd now have a much
tighter coupling between particular FIDL types and system calls, which would
not automatically remain in sync.

### Future Work

The use of an eventpair to signal the validity of the client's cache incurs an
additional system call on both the `send` and `recv` paths. This system call
can be eliminated by instead using a VMO to signal validity. In such a design,
the eventpair is logically replaced by an offset into a VMO mapped into the
client's address space. Subsequently, the client can check validity with a
simple read into the VMO.

### Unknowns

There's also a question about how to deal with `SendControlData`. Perhaps that
would need to be an additional parameter to `zx_socket_send_to` or maybe a flag
on the operation.

[fsocket]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.posix.socket/socket.fidl;l=292-295;drc=0661adfd75b2c6c49b7cdb2c4edba7507c1e12ea
[fsocket-provider-dgram-sock]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.posix.socket/socket.fidl;l=575;drc=0661adfd75b2c6c49b7cdb2c4edba7507c1e12ea
[fuchsia-io-NodeInfo]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.io/io.fidl;l=15;drc=0661adfd75b2c6c49b7cdb2c4edba7507c1e12ea
[max-size-in-channel]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/fidl/llcpp/include/lib/fidl/llcpp/sync_call.h;l=30;drc=c58104675245f8b9ab9eec0101673a4730c63443
[fidl-at-rest]: 0120_standalone_use_of_fidl_wire_format.md
[abi-revision]: 0002_platform_versioning.md#fidl
[package-abi-revision]: 0135_package_abi_revision.md#design
