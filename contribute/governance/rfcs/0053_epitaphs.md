{% set rfcid = "RFC-0053" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-008.

_"Here lies your server"_

## Summary {#summary}

The goal of this proposal is to allow a server to send a message prior to
closing a connection that provides an indication of why the connection is being
closed. While
[epitaphs][wire-format-epitaph]
are covered in the specification, they are not implemented yet.

## Motivation

Currently, there is no standard way for servers to communicate to clients why a
connection has been closed. This has the effect that the responsibility for
ensuring error handling falls to the developer.  The developer can either
foresee this, and build special error handling into their message, or simply
ignore error handling (and risk undiagnosable errors).

One use case is for servers where errors are mostly fatal, and when they happen,
all connections to clients are closed. In such cases, developers want a
general-purpose error reporting mechanism, because all active calls to methods
will be terminated with the same error. The alternative of declaring a potential
error for each method would be cumbersome and awkward.

This FTP does not have a goal of providing an extensive error reporting
mechanism. Specifically, the ability to communicate large amounts of detail
(including detailed messages, process state, or propagated causes) to the other
end of the connection is out of scope.

This FTP also does not have a goal of defining a set of common error codes.

## Design

This proposal modifies the wire format, the source language, and the first class
language bindings.

### Wire format

The wire format specification currently has a section on Epitaphs. This section
will be revised to read as follows:


```
Epitaph (Control Message Ordinal 0xFFFFFFFF)

An epitaph is a message with ordinal **0xFFFFFFFF**.  A server may send an
epitaph as the last message prior to closing the connection, to provide an
indication of why the connection is being closed.  No further messages may be
sent through the channel after the epitaph.  Epitaphs are not sent from clients
to servers.

When a client receives an epitaph message, it can assume that it has received
the last message, and the channel is about to be closed. The contents of the
epitaph message explain the disposition of the channel.

The epitaph contains an error status.  The error status of the epitaph is stored
in the reserved uint32 of the message header.  The reserved word is treated as
being of type **zx_status_t**: negative numbers are reserved for system error
codes, positive numbers are reserved for application error codes, and ZX_OK is
used to indicate normal connection closure.  The message is otherwise empty.
```

### Source language

The source language specification currently has a section on
[Epitaphs][epitaphs].
It will be updated appropriately.

### First class language bindings

Implementations should account for the fact that, if an Epitaph message is sent,
it should be the last message prior to closure, and for the fact that errors are
handled differently in different languages (via, for example, delivery of error
codes in C/C++, Result<T, E> in Rust, and exceptions in Dart).

We will add a method fidl_epitaph_write(channel, zx_status_t) to the C bindings,
as well as a fidl_epitaph_t type.

We will add the following documentation to the C bindings to the section on Raw
Bindings:

```
fidl_epitaph_write

Declared in lib/fidl/epitaph.h, defined in epitaph.c.

This function sends an epitaph with the given error number down the given
channel.  An epitaph is a special message, with ordinal 0xFFFFFFFF, which
contains an error code.  The epitaph must be the last thing sent down the
channel before it is closed.
```


CL for the C changes: https://fuchsia-review.googlesource.com/c/zircon/+/178250

We will change the C++ bindings to do the following:

fidl::Binding will immediately close the channel on receipt of an Epitaph.

Developers will be able to close the channel with fidl::Binding::Close

Error codes will be propagated to the error handler set by the client using
set_error_handler().  We will add a new error_handler variant that takes a
closure that takes an int variable representing the error code, and remove the
existing one.  Potential future work involves having a "sensible default" error
handler, although it is not currently clear what this would be.

Any pending reads from this channel will return ```ZX_ERR_PEER_CLOSED```.

CL for C++ bindings: https://fuchsia-review.googlesource.com/c/garnet/+/177939

The other bindings need to be updated, including Dart, Rust, and Go.

## Documentation and examples


The documentation will be updated as described in the previous section.


### Guidance for Developers {#guidance}

The purpose of an epitaph is to enable a server to provide actionable
information to the client regarding the disposition of the channel and requests
that may have been in flight.

This section describes the intended behavior and usage of epitaphs.

1. An epitaph message is only ever sent from a server to a client, never in the
   other direction.  If sent, it must be the last message sent by the server to
   the client before the server closes its end of the channel.

2. When a client receives an epitaph message, it must immediately close its end
   of the channel.  It must not attempt to read any further messages from the
   channel that may have been sent by a non-conforming server implementation.

3. When a client observes peer closed without having received an epitaph, then
   it must proceed as if it has received a ```ZX_ERR_PEER_CLOSED``` epitaph;
   these two states are semantically equivalent.

4. A server is expected to send a ```ZX_OK``` epitaph when the closure of the
   channel was an anticipated side-effect of the protocol reaching its
   designated successful end state.

   a. Example: When a client calls Commit() on an interface representing an
   individual database transaction, the server should attempt to apply the
   requested changes.  If successful, the server must send a ```ZX_OK``` epitaph
   before closing its end of the channel.  The client may reasonably construe
   that the ```ZX_OK``` epitaph indicates that the transaction was successfully
   committed.

   b. Counter-example: Many protocols do not have designated successful end
   states; the client expects to be able to connect to a server and issue an
   unbounded number of requests without observing peer closed until such time as
   the client closes its own end of the channel.  In these situations, the
   server closing its end of the channel constitutes an abnormal end state, so
   the server should never send a ```ZX_OK``` epitaph.

5. A server may send a non-```ZX_OK``` epitaph prior to closing its end of a
   channel for any reason other than the protocol reaching its designated
   successful end state.  We suggest the following convention:

   a. If the server is closing the connection because the client sent it an
   malformed FIDL message, it should send a ```ZX_ERR_INVALID_ARGS``` epitaph.

   b. If the server is closing the connection because the client sent it a
   request that is not valid in its present state, it should send a
   ```ZX_ERR_BAD_STATE``` epitaph.

   c. If the server was unreachable (e.g. could not be started) when the client
   attempted to connect to it via a service discovery mechanism, this mechanism
   should send a ```ZX_ERR_UNAVAILABLE``` epitaph.  (See also this sketch.)

   d. If the server is unable to continue serving the protocol for reasons that
   are not in response to actions performed by the client (e.g. shutting down or
   out of memory), it does not have to send any epitaph.  The client will
   perceive this as ```ZX_ERR_PEER_CLOSED``` as described above.

   e. If a server encounters an application specific error, it should send an
   application-defined error code.  For example, if the server controls a
   filesystem, and the user tries to perform a write that it is not allowed to
   perform, it may wish to close the connection with an error.

   f. This list is not exhaustive.  A server may send other errors as
   appropriate.  As per usual, FIDL authors are advised to clearly document the
   errors their protocols may return, including epitaphs.

## Backwards compatibility

The FIDL documentation currently states that 0x80000001 is the ordinal for an
epitaph.  We are changing it to 0xFFFFFFFF, because 0x80000001 is in use by IO.
Nothing is currently relying on Epitaphs using 0x80000001.  Otherwise, there
are no backwards compatibility concerns.

## Performance

n/a

## Security

Not applicable.

## Testing

Unittests for this feature will be added to the appropriate FIDL bindings. After
each supported FIDL binding gets support, we should augment the set of
[FIDL compatibility tests](/src/tests/fidl/compatibility/).

## Drawbacks, alternatives, and unknowns

We considered making a ```System``` interface containing the ```Epitaph```
event, which would be the parent of all other interface messages.  Epitaphs, on
their own, do not warrant such a large change.  There are also currently two
implementation hurdles to this.  First, derived types do not currently work,
although that is supposed to change soon.  Next, because this proposal changes
the runtime, and the FIDL parser / generator depends on the runtime, introducing
a System message and trying to use it in the runtime would result in a circular
dependency.

The API changes that will result from this FTP will not prevent Epitaph support
from moving into a future System message.

An idea was floated of incorporating some epitaph handling into the source
language, allowing the ```zx_status``` flag to be mapped as a FIDL-defined enum.
This is deferred to future work.


The proposed implementation is racy.  If one thread writes a message
concurrently with another thread closing the channel, the epitaph may be written
prior to the other thread's message, but before the call to
```zx_channel_close()```.  Alternatives include locking the channel or providing
an explicit system call.  We are starting with the thread-unsafe version to
further develop our understanding of the problem space.

<!-- xrefs -->
[wire-format-epitaph]: /docs/reference/fidl/language/wire-format/README.md#epitaph-control-message-ordinal-0xffffffff
[epitaphs]: /docs/reference/fidl/language/language.md#interfaces
