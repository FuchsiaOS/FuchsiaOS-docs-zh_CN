{% set rfcid = "RFC-0099" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

Introduce `zx_socket_set_disposition` to replace `zx_socket_shutdown`. The new
syscall extends the old by allowing the shutdown operation to be reversed.

Introduce `ZX_RIGHT_MANAGE_SOCKET` and require it in the new syscall. Newly
minted handles via `zx_socket_create` will have this right.

## Motivation

### Motivation for replacing shutdown with a reversible operation

We have an elaborate state machine in fdio to cope with the fact that
unconnected stream network sockets should not accept writes. The state machine
has local state which is difficult to propagate to duplicated sockets in other
processes, and remote state (user signals on the zircon socket) that is used to
drive the local state. These gymnastics are necessary because sockets are
created "open", which makes it necessary to prevent I/O on them via external
means until they become "connected" (also via external means).

### Motivation for requiring a new right

Zircon socket shutdown is already today too permissive. Since zircon socket
handles can be (and are in practice) cloned, a single handle with the write
privilege can mutate the socket state for all handles. This problem is made
more severe by allowing the shutdown to be reversed as proposed here.

### Putting it together

Replacing socket shutdown with a reversible operation while also disallowing
unprivileged handles from initiating it allows a network stack implementation
to drive the socket state completely.

Sockets can be initially shut down before being vended to clients, removing the
need for the aforementioned state tracking in fdio.

Socket shutdown can be fully mediated by the network stack, initiated by client
FIDL calls, eliminating race conditions that exist today (e.g.
https://fxbug.dev/61714).

## Design

### Define `ZX_RIGHT_MANAGE_SOCKET` in [FIDL][rights-fidl]:

Extend `bits rights`:

```fidl
library zx;

bits rights : uint32 {
  MANAGE_SOCKET = 0x00100000;
};
```

### Document `ZX_RIGHT_MANAGE_SOCKET` in [rights.md][rights-doc]:

Append to table:

| Right | Conferred Privileges |
| ----- | -------------------- |
| **ZX_RIGHT_MANAGE_SOCKET** | Allows changing socket disposition via `zx_socket_set_disposition` |

### Define `zx_socket_set_disposition` in [FIDL][socket-fidl]

Add to `protocol socket`:

```fidl
library zx;

protocol socket {
  /// Set disposition of writes.
  socket_set_disposition(handle:<SOCKET, rights.MANAGE_SOCKET> handle, uint32 disposition, uint32 disposition_peer) -> (status status);
}
```

### Document `zx_socket_set_disposition` in `/docs/reference/syscalls/socket_set_disposition.md`

#### DESCRIPTION

`zx_socket_set_disposition` sets the disposition of
[`zx_socket_write`][socket_write] calls for a socket handle and its peer.

Valid disposition flags that can be used:

**ZX_SOCKET_DISPOSITION_WRITE_DISABLED** - Disable writes for the specified
socket endpoint. Once set, writes to the specified socket endpoint will fail
with **ZX_ERR_BAD_STATE**. Reads from the specified socket endpoint will
succeed until all data buffered in the specified socket endpoint is consumed,
and fail with **ZX_ERR_BAD_STATE** thereafter.

**ZX_SOCKET_DISPOSITION_WRITE_ENABLED** - Enable writes for the specified
socket endpoint. Once set, writes to and reads from the specified socket
endpoint will behave as specified in [`zx_socket_write`][socket_write] and
[`zx_socket_read`][socket_read], respectively.

It is invalid to specify **ZX_SOCKET_DISPOSITION_WRITE_ENABLED** on a socket
endpoint that has buffered data; doing so will result in
`zx_socket_set_disposition` returning **ZX_ERR_BAD_STATE** and no action being
taken.

It is invalid to specify both **ZX_SOCKET_DISPOSITION_WRITE_DISABLED** and
**ZX_SOCKET_DISPOSITION_WRITE_ENABLED** in *disposition* or *disposition_peer*;
doing so will result in `zx_socket_set_disposition` returning
**ZX_ERR_INVALID_ARGS** and no action being taken.

#### RETURN VALUE

`zx_socket_set_disposition()` returns **ZX_OK** on success.

#### ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_BAD_STATE**  *disposition* or *disposition_peer* contains
**ZX_SOCKET_DISPOSITION_WRITE_ENABLED** and *handle* refers to a socket with
buffered data on the specified socket endpoint.

**ZX_ERR_WRONG_TYPE**  *handle* is not a socket handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_MANAGE_SOCKET**.

**ZX_ERR_INVALID_ARGS**  *disposition* or *disposition_peer* contains flags
outside of the ones listed above or an invalid combination of flags.

### Migration

Once implemented, existing usages of `zx_socket_shutdown` will be replaced with
equivalent calls to `zx_socket_set_disposition`. Once necessary ABI transitions
are complete, `zx_socket_shutdown` and its associated options will be removed.

## Implementation

Implementation should be possible entirely within the socket dispatcher.

## Performance

This change has no meaningful impact on performance.

## Ergonomics

This change has no meaningful impact on ergonomics.

## Backwards Compatibility

This change is backwards compatible because clients that do not use the new API
surface are unaffected.

## Security considerations

This change allows fdio code to be simpler, which can improve security. This
change otherwise has no meaningful impact on security.

## Privacy considerations

This change has no meaningful impact on privacy.

## Testing

This feature will be tested using unit tests of the syscalls and by replacing
parts of the already-tested fdio state machine with the new machinery.

## Documentation

[`zx_socket_write`][socket_write] and [`zx_socket_read`][socket_read] will be
updated to refer to `zx_socket_set_disposition` instead of
`zx_socket_shutdown`.

Other documentation will be updated as described in the implementation section.

## Drawbacks, alternatives, and unknowns

Instead of a new syscall, a new flag could be added to `zx_socket_shutdown` to
allow its behavior to be reversed. That approach has the benefit of avoiding
introducing a new term (disposition) which isn't an obviously good match for
the behavior. The primary downside of continuing to use `zx_socket_shutdown` is
that the flags it accepts are not intuitive; `ZX_SOCKET_SHUTDOWN_READ` doesn't
do what it says on the tin (it disallows peer writes, rather than disallowing
reads).

The behavior of "unshutdown" is specified as producing an error if data exists
in the specified direction in the socket. An alternative choice is to allow
that operation to succeed; we select the stricter option in the interest of
preventing unintended consequences.

Instead of creating a new right, an existing one could be used. A survery of
existing rights suggests that no existing right is a good match for this use
case.

This design alone doesn't fully solve the state-propagation problem of stream
sockets. An alternative to this proposal is a more holistic approach that
strives to eliminate the fdio stream socket state machine entirely. Such a
proposal would necessarily include this proposal as well.

## Prior art and references

Stream socket semantics are de-facto defined by their behavior in other
operating systems, from which the need to distinguish connected sockets from
unconnected ones arises.

[rights-fidl]: /zircon/vdso/rights.fidl
[rights-doc]: /docs/concepts/kernel/rights.md
[socket-fidl]: /zircon/vdso/socket.fidl
[socket_write]: /docs/reference/syscalls/socket_write.md
[socket_read]: /docs/reference/syscalls/socket_read.md
