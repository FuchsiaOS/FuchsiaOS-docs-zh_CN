<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0184" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->
## Summary

Fuchsia aims to expose a POSIX-compatible networking API to components via
`fdio` and the system netstack. It also supports some non-POSIX functionality
that is common across other POSIX-oriented operating systems.

## Motivation

Fuchsia's existing system netstack is built around a core that targets Linux
compatibility. With a planned replacement of this netstack in the works,
questions of compatibility have repeatedly been raised. This proposal puts those
to rest by requiring any system netstack to target a POSIX-like API.

The POSIX networking interface describes a standard way for components to
access network resources. Supporting the networking subset of POSIX for Fuchsia
components makes it easy to 1) reuse existing code on Fuchsia, and 2) write new
code for Fuchsia using a familiar API.

## Stakeholders

Who has a stake in whether this RFC is accepted? (This section is optional but
encouraged.)

_Facilitator:_

hjfreyer@google.com

_Reviewers:_

- abarth@google.com (RFC-0082 author)
- brunodalbo@google.com (Netstack)
- dhobsd@google.com (Network Policy)

_Consulted:_

brunodalbo@google.com, hanjh@google.com, hjfreyer@google.com,
martinjeffrey@google.com, nickbrow@google.com, tamird@google.com,
wildenhain@google.com

_Socialization:_

This RFC went through a design review with the Netstack team.

## Design

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in
[IETF RFC 2119](https://tools.ietf.org/html/rfc2119).

On a Fuchsia device, the system networking stack provides networking
functionality to components by exposing several fuchsia.posix.socket FIDL
services. While FIDL-aware components can target them, such direct usage is
actively discouraged. Instead, components written for the POSIX system call
API SHOULD link in the `fdio` compatibility library to translate libc system
calls into FIDL service calls.

Fuchsia's [`fdio`][fdio] library acts as a translation layer for a [limited
subset](/contribute/governance/rfcs/0082_starnix.md#motivation) of POSIX to
the appropriate FIDL services. For networking functionality, `fdio` provides
implementations of a number of POSIX calls, including [`socket`][socket],
[`setsockopt`][setsockopt] and [`getsockopt`][getsockopt], [`read`][read],
[`write`][write], [`send`][send], [`recv`][recv], and more. Together, the
implementation of the FIDL services layered with `fdio` provide complete
implementations of these, and other, calls.

[fdio]: /glossary#fdio
[socket]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html
[setsockopt]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
[getsockopt]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
[read]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/read.html
[write]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/write.html
[send]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/send].into_iter().html
[recv]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recv.html

Instead of providing a complete list of system calls, the goal of this document
is to define the criteria used for deciding when to implement POSIX- or
peer-system-defined networking functionality. The expectation is that `fdio` and
networking stack will implement system calls and options as-needed, primarily
for first-party Fuchsia code, and then when required by other applications.
POSIX functions that can be implemented by using the `fdio`-provided system
calls are out of the scope of this document.

Note that while this proposal mandates that Fuchsia provide a POSIX-compatible
networking interface, it does not require its usage. In particular, nothing
precludes the future specification or development of a Fuchsia-first API to be
implemented alongside POSIX.

## Implementation

The existing `fdio` library and system networking stack already provide a
mostly-POSIX-compatible interface to components. This proposal is intended to
codify the as-yet informal decision to aim for parity with POSIX-like operating
systems, and to guide future development on the networking stack and `fdio`
library. When making changes to the system netstack and `fdio`, the following
three principles SHOULD be considered:

### POSIX compliance

Fuchsia's system netstack and the `fdio` library aim to provide compatibility
for the networking API specified by POSIX. Components that target the POSIX
networking API SHOULD work as expected when linked with `fdio` and routed the
appropriate socket creation capabilities.

### Compatibility with peer systems

The POSIX specification leaves the behavior of some interactions undefined, and
so components written against the POSIX interface often expect and account for
the behavior of a particular operating system or family of operating systems.
Where this behavior is well-defined and consistent across multiple POSIX-like
operating systems, Fuchsia's networking subsystem SHOULD match it (except in
limited cases, as described below). When the behavior of peer systems is
inconsistent, Fuchsia is not guaranteed to match any particular peer system's
behavior.

### Allowance for divergent behavior

The Fuchsia networking subsystem may need to implement behavior that is
different from that of peer systems. Such divergence SHOULD arise when

- The behaviors of peer systems are inconsistent with one another,
- Implementing behavior consistent with a peer system would introduce a security
  risk, or
- Implementing consistent behavior would be difficult or impossible due to
  Fuchsia's architectural constraints.

In those cases, the divergence SHOULD be well-motivated, well-documented, and
well-tested. Furthermore, the difference in behavior SHOULD be easily observable
by components (e.g. a POSIX system call returning an error).

### Known Limitations

POSIX makes use of several global identifier spaces, including UIDs, GIDs, PIDs,
and file paths. Many of these identifiers are used alongside built-in support
for capabilities to limit access to networking operations on POSIX-like systems.
These include (but are not limited to):

- Binding sockets on the same address with `SO_REUSEPORT` and `SO_REUSEADDR` is
  restricted to components running with the same UID.
- On Linux, the ability to clear the `SO_BINDTODEVICE` socket option is limited
  to applications running with `CAP_NET_RAW`.
- On Linux, the ability to create raw IP sockets is limited to applications
  running with `CAP_NET_RAW`.
- On Linux, binding sockets to low-numbered ports requires an application to
  have `CAP_NET_BIND_SERVICE` (though it is an unprivileged operation on recent
  macOS versions).

Where possible, these behaviors will be supported on Fuchsia, though doing so is
subject to the feasibility of mapping their functionality to Fuchsia concepts,
and may require allowances for Fuchsia's architectural constraints. As an
example, POSIX-like systems implicitly use a process's UID to scope port
sharing permissions. Since Fuchsia doesn't have UIDs, components will need to
take explicit action to opt in to port sharing, likely in the form of additional
calls into `fdio`.

## Performance

As part of the formalization of support for the POSIX networking interface,
Fuchsia's networking subsystem will provide a high-performance implementation of
the API. The Fuchsia networking stack and `fdio` already have significant
benchmarking tooling that exercises the POSIX interface. This will be used to
measure performance improvements and detect regressions.

## Ergonomics

By targeting POSIX, a well-known and common interface for applications, Fuchsia
makes it easy for developers to port existing code and provides a familiar
interface for writing new code. Though some POSIX concepts do not map directly
to Fuchsia (e.g. UIDs), the vast majority of networking concepts do. Targeting a
familiar interface for networking will significantly improve the experience of
developing on and porting code to Fuchsia.

## Backwards Compatibility

This proposal does not represent a change of principles, just a codification of
informal ones. Since no change is being introduced, considerations for backwards
compatibility are minimal.

## Security considerations

This RFC doesn't introduce any new security considerations as it codifies an
existing set of informal principles. Furthermore, the commitment to providing a
POSIX-compatible API does not preclude future per-component isolation or
sharding of the networking stack to address security issues.

## Privacy considerations

This proposal does not introduce any new privacy considerations as it only
codifies the support of already-in-use POSIX APIs.

## Testing

The Fuchsia system netstack is tested using an [existing compatibility
suite][compatibility suite] that checks conformance against POSIX and Linux
(though the latter is a matter of convenience and does not imply an implicit
endoresement of Linux behavior). This test suite helps prevent regression and
guides future feature development by encoding the expected behavior of the
system in response to POSIX calls. Intentional behavior differences between
Fuchsia's networking subsystem and POSIX or POSIX-like systems are encoded and
documented in the test suite. Known unintentional differences are also encoded
and documented, and tagged in Fuchsia's bug-tracking system. This
integration-level testing, plus existing unit tests for the internals of the
system netstack, provide sufficient coverage for POSIX compatibility.

[compatibility suite]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/connectivity/network/tests/;drc=e9389413d12cb940a705e6efc8c4b31f8d92f8a4

## Documentation

This proposal requires two additional elements of documentation:

1. Instructions for how to use the `fdio` API to communicate with the system
   netstack, and
2. A list of divergent behaviors between the Fuchsia netstack/`fdio` and
   POSIX/POSIX-like system behavior.

## Drawbacks, alternatives, and unknowns

This proposal requires committing to implementing a significant subset of POSIX
and peer-compatible behavior across the Fuchsia system netstack and `fdio`
library. Because this proposal is a formalization of existing plans, much of the
functionality already exists. This proposal commits Fuchsia to expanding the
existing API surface, and to supporting it in the long term.

While POSIX is a well-known standard, it was designed without support for
capabilities or the rich IPC specification mechanisms that Fuchsia has.
Supporting compatibility with POSIX-like systems requires both providing to
components a more limited interface (synchronous system calls, untyped file
descriptors) and shoehorning those concepts into Fuchsia primitives. In
addition, adopting a POSIX-like interface for Fuchsia components may hinder
development of a useful Fuchsia-first networking API.

As a more radical option, Fuchsia could explicitly eschew POSIX compatibility in
favor of a Fuchsia-first API. Given that a large amount of Fuchsia system
service code is already written to target a POSIX-like API, this seems both
counterproductive and short-sighted.

Regarding unknowns, the major expected categories are areas of incomplete
implementation for POSIX support, and incompatible behavior for Fuchsia relative
to peer operating systems. These will need to be addressed and documented as
instances arise or are discovered.

## Prior art and references

- The POSIX 2017 [specification][POSIX specification] lists the requirements of
  a POSIX-compatible system.
- [RFC-0082][RFC-0082] describes Fuchsia's goal of running unmodified Linux
  programs on Fuchsia.
- The [gVisor][gVisor] project's networking code forms the core of the existing
  Fuchsia system netstack.

[POSIX specification]: https://pubs.opengroup.org/onlinepubs/9699919799/
[RFC-0082]: /contribute/governance/rfcs/0082_starnix.md
[gvisor]: https://github.com/google/gvisor

## Appendix: Implementation decision case study

POSIX's `setsockopt` function provides a method for code to set options on a
socket that affect its behavior. POSIX defines several option flags, but
compliant systems are allowed to add their own custom flags. One fairly
commonly-used option is `SO_REUSEPORT` option, which, when set, allows `bind`ing
sockets on the exact same address and port. Since its semantics are well-defined
and consistent across multiple systems (including FreeBSD, macOS, and other BSD
derivatives), Fuchsia's networking stack allows components to set `SO_REUSEPORT`
option on UDP sockets.

One of the differences between Linux and BSD-derived implementations of
`SO_REUSEPORT` is Linux's requirement that sockets being bound to the same
address belong to processes with the same user ID. Since Fuchsia's architecture
precludes a similar notion of user ID, this constraint is not implemented in
Fuchsia.

Furthermore, Linux's implementation of `SO_REUSEPORT` results in inconsistent
behavior depending on whether the option is set on a socket before calling
`bind` and then cleared after, or not set at all on the socket. Dependence on
invisible system state and poorly-defined behavior inform a decision not to
emulate Linux's behavior.

More details are available in https://fxbug.dev/100840.