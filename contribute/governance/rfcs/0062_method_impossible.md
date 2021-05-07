{% set rfcid = "RFC-0062" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-005.

## Rejection rationale

This FTP was rejected because having a hard limit was felt to be too
heavy-handed.
The [types.h][types] file has definitions for `ZX_CHANNEL_MAX_MSG_BYTES`
and `ZX_CHANNEL_MAX_MSG_HANDLES` that can be examined by the implementor
and used to limit the resource consumption.

Possible directions discussed were annotation based constraints (today
we have `[MaxHandles]` and `[MaxBytes]` attributes).

There are also use cases where we need to express (in the language) possibly
unbounded messages (size, or handles).
Typically, such messages are dynamically assembled to meet the requirements
of the underlying transport (e.g. optimize throughput, adapt to older clients).

## Summary

It should be an error to declare an interface method that may require more
than the maximum number of handles or bytes allowed in a Zircon channel
message.

It's easy to declare FIDL types that may be impossible to send in a single
Zircon channel message.
Developers should be able to avoid defining types that may cause unexpected
runtime errors.

Since we foresee other transports for FIDL data like shared memory,
persistent storage and the network, the limit is on interface methods rather
than on types.

## Motivation

Edge-cases are hard to test well and hard to reason about.
FIDL messages that could be impossible to transmit in exceptional
circumstances are likely to expose poorly tested parts of FIDL services.
They may be exposed to untrusted code.

For example, in `fuchsia.sys`, currently a `FlatNamespace` may contain any
number of handles.
A `LaunchInfo` may contain a `FlatNamespace`.
A call to `Launcher.CreateComponent()` could be crafted (maliciously or
accidentally) that would succeed, but then when the appmgr went to pass the
supplied `LaunchInfo` to `Runner.StartComponent()` the additional handles
supplied would block the successful encoding of the message.

## Design

This modifies the FIDL compiler but not the FIDL language, bindings or wire
format.

The FIDL frontend compiler now keeps track of how many handles and bytes may
be required to represent a message.
Currently a Zircon channel message may only contain up to 64 handles.
The compiler should print an error and fail if any method is defined that
could require more than 64 handles to encode its request or response.
Currently a Zircon channel message may only contain up to 64k bytes.
The compiler should print an error and fail if any method is defined that
could require more than 64k bytes to encode its request or response.

There are two main patterns that fall afoul of this restriction: recursive
types with handles and unbounded vectors of handles (or types that contain
them).
It's fairly straight-forward to avoid these.

## Documentation and examples

FIDL documentation should document this constraint and ensure that all
examples are valid.
The error messages produced by the compiler should be clear and useful.

## Backwards compatibility

This breaks FIDL source compatibility.
Many existing interfaces (such as `FlatNamespace` described above) fail to
constrain the number of handles or bytes that may be required.
These interfaces would have to be tightened up.

## Performance

There should be no direct performance impact.
Some interfaces and types may need to be changed if they really want to
support arbitrarily many handles or bytes but they never would have worked
anyway.

## Security

This change reduces the amount of unexpected, under-tested behavior that
applications will experience so it will improve security.

## Testing

The `fidlc` compiler should get some new tests to ensure that its
calculations of handle counts are correct.

## Drawbacks, alternatives, and unknowns

This exposes some additional details of the Zircon channel IPC mechanism to
FIDL interface authors.
This seems like a fair trade-off because currently anyone using those
interfaces needs to be aware of those trade-offs.

We could require all strings and vectors to have explicit bounds.
This would encourage interface authors to design types that might be safely
usable across Zircon channels but might limit the flexibility of FIDL types
across other media.

While the only transport for handles is channel messages, there are other
transports for FIDL encoded bytes such as VMOs, network sockets and
persistent storage.
It may be useful to allow some types to opt-out (for example with an
attribute) from this constraint, even though those types would never be able
to be transmitted over a zircon channel.

It may be worth considering incorporating other mechanism into the FIDL
language (pagination / streaming / etc.) before imposing this constraint on
byte lengths.

## Prior art and references

Unknown

<!-- xrefs -->
[types]: /zircon/system/public/zircon/types.h
