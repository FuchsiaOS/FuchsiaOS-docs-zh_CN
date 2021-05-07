# Troubleshoot zx_err_peer_closed

The error `zx_err_peer_closed` indicates a failed attempt to read or write
from a peered Zircon object. Usually, this means that the remote side of the
peer is closed. This can occur for many different types of Zircon objects,
including:

* Channels
* Sockets
* FIFOs

The error can also occur when failing to signal a peered object, such as
[`zx_object_signal_peer`](/docs/reference/syscalls/object_signal_peer.md).

This document helps you troubleshoot the following causes
of a `zx_err_peer_closed` error.

## Error causes

The following are potential causes of a `zx_err_peer_closed` error.

### Incompatible FIDL protocols

The `zx_err_peer_closed` error is thrown when
incompatible FIDL protocols are used between
the client and server, which closed the connection.

To remedy this, make sure your client and server are using the same FIDL
protocols. See the [FIDL language specification](/docs/reference/fidl/language/language.md#protocols) for more information
on FIDL protocols.

### The requested protocol is not defined in the component manifest

Similar to incompatible FIDIL protocols, if the requested protocol is
not defined in your component manifest, or `.cmx` file, you will get a
`zx_err_peer_closed` error.

Specifically, if you are using components v1, you will get a `zx_err_peer_closed` error
in the following cases:
  * Your `.cmx` file requests a protocol that is not present in your realm. If
    in the sys realm, this means the sysmgr config doesn't include the service.
  * Your `.cmx` file requested a protocol that is present in your realm, but
    the component that is supposed to provide it is misconfigured and not actually
    providing the protocol.

If this is the case, make sure your `.cmx` file includes the protocol that you are requesting. You
can use `fidlcat` to help you diagnose the missing protocol.

For more information, see:

* [Building components](/docs/development/components/build.md) for more information on `.cmx` files.
* [Fidlcat: Monitor and debug your fidl calls](/docs/development/monitoring/fidlcat/README.md) for more
  information on `fidlcat`.
* [Component Realms](/docs/concepts/components/v2/realms.md) for more information on realms.

### A requested file cannot be accessed

The `zx_err_peer_closed` error is thrown when you fail to open a file. This can happen for
the following reasons:

*  You're attempting to open a file that doesn't exist
*  You're trying to open a file that is requesting too many rights.

If this is the case, verify that you are requesting an existing file, or that the file and directory
have the appropriate rights. You can learn more about
[directory rights and how they're specified](/docs/concepts/components/v2/component_manifests.md#directory-rights).

### Peer has crashed and the process has been terminated

If the peer crashes, the system processes the crash and terminates the process that generated
the error and all associated objects, leading to a `zx_err_peer_closed` error.

You can [view logs](/docs/development/diagnostics/logs/viewing.md) to confirm if the peer
has crashed. If this is the case, you can debug the peer using the
[Just in Time Debugger](/docs/development/debugging/just_in_time_debugging.md) to find
the source of the crash, and try the operation again.

## Adding epitaphs to errors as a component author

As a component author, it's important to help users get additional meaningful error messages
when they see the `zx_err_peer_closed` error. To do this, your code should set
an epitaph with the
[binding::Close(zx_status_t epitaph_value)](/sdk/lib/fidl/cpp/binding.h#199) binding.

See examples for setting meaninful epitaphs in
[//src/sys/appmgr/realm.cc](/src/sys/appmgr/realm.cc).
