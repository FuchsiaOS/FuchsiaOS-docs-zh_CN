Integrator Development Kit (IDK) Standards
=============

This document describes the standards for how we develop the Fuchsia IDK within
the Platform Source Tree. Some of the information in this document might be of
interest to clients of the Fuchsia SDK, but the primary focus of the document is
how the Fuchsia project develops the SDK.

## Governance

The contents of the Fuchsia IDK are governed by the [Fuchsia API Council]. The
IDK does not contain elements (libraries, tools, images, etc) developed outside
the Fuchsia project because those libraries are not subject to the governance
of the Fuchsia API Council.

Client libraries in the IDK do not depend on libraries outside the IDK unless
the external library has been approved by the Fuchsia API Council. Typically,
the council will not approve a dependency unless the dependency has strict
evolution criteria (e.g., the standard libraries for the various supported
languages).

### Example: Google Test

The Fuchsia IDK does not include the _Google Test_ library because the
governance for the _Google Test_ library is provided by Google, not by the
Fuchsia API Council.

The Fuchsia IDK does not depend on the _Google Test_ library because the
[promises made by the governing body](https://abseil.io/about/philosophy#upgrade-support)
for the _Google Test_ library are not compatible with the model used by the
Fuchsia IDK.

## Fuchsia System Interface

The Fuchsia System Interface is defined in [Fuchsia System
Interface](/docs/concepts/system/abi/system.md). Generally speaking, the binary interface to the system
is only the FIDL wireformat used by programs to communicate with the system and the syscalls exposed
in `libzircon`.

## FIDL Protocol Definitions

### Binary stability

FIDL protocols are defined in `.fidl` files, which are contained in the SDK.
All the FIDL definitions that have been published in an IDK should be considered
public ABI for the system. The system might also contain additional FIDL
definitions that have not been published in an IDK. Those definitions are
subject to change without notice and programs that rely upon their ABI might not
work properly in future versions of the system.

### Source stability

FIDL definitions in the IDK might evolve in source-incompatible ways. For
example, we might rename a method in a protocol while maintaining its ordinal
and semantics (the ordinal can be maintained by adding a `Selector` attribute
that is set to the original name). Such a change preserves the ABI but breaks
source-compatibility.

We do not currently have any standards about when we should break source
compatibility.

### Naming

Public FIDL definitions are located in the source tree under
`//sdk/fidl/$LIBRARY_NAME`.
The target name should be the name of the library.

### Style

FIDL definitions in the IDK should follow the [FIDL API style rubric].

## Client Libraries

The Fuchsia IDK contains a number of "client libraries" (libraries that clients of an SDK integrating
the IDK can link into their programs). All of these client libraries are optional
and provided for the convenience ofclients, not for the convenience of the system.
The system must not rely upon programs using any specific client libraries.
Note that `libc` is a client library (not a system library).

### Stability and Packaging

Only the [Fuchsia System Interface](#fuchsia_system_interface) is ABI stable. Client libraries are
neither API nor ABI stable. Binaries and libraries must be built against the same IDK version as the
client libraries they are linked with.

All libraries a program links beyond the [Fuchsia System Interface](#fuchsia_system_interface),
including client libraries, must be included inside the program's package. Dynamic libraries should
be placed in the `lib` directory of the program's package.

Packages are the unit of software mobility, delivery, and linkage. Different packages can contain
different versions of the same library.  When running a program, the system provides that program
the libraries from its own package, preventing the different libraries used by different packages
from conflicting in the same program.


### Precompiled libraries

The Fuchsia IDK does not require clients to use a specific toolchain. For this reason, precompiled
client libraries must have C linkage. For example, a precompiled client library cannot export C++
symbols because C++ does not have a standard ABI across toolchains (or even toolchain versions).

### Dependencies

A client that takes a dependency on a client library must also take a dependency
on all the dependencies of that library. For this reason, client libraries
should have minimal dependencies. For example, client libraries should avoid
dependencies on FBL, FXL, FSL, or other "base" libraries that are not in
the SDK.

Client libraries that need to perform asynchronous operations should depend on
`libasync.a` and `libasync-default.so`. However, these libraries should not
assume the client is using any specific implementation of `async_dispatcher_t*`.
For example, these libraries should not assume the `async_dispatcher_t*` is
actually implemented by `libasync-loop.a`. Libraries that require
`async_get_default_dispatcher` to be populated should state this requirement in
their documentation.

Precompiled libraries can have more extensive dependencies if those dependencies
are hidden from their client. For example, a precompiled shared library should
not export symbols from these dependencies and should not have headers that
transitively include headers from these dependencies.

### Naming

Client libraries should be named according to the language they expect their
clients to use.
For example, the C++ variant of the `$NAME` library should be located in the
source tree under `//sdk/lib/$NAME/cpp`.
The C variant should simply be under `//sdk/lib/$NAME`.

### Style

Client libraries should follow the Fuchsia style guide for the language in which
they are written.

### Logging

Client libraries should avoid logging messages. Instead, client libraries should
return errors to their clients, who can decide whether to log the error.

### Assertions

C and C++ client libraries should use `ZX_DEBUG_ASSERT` and `ZX_ASSERT`, defined
in `<zircon/assert.h>`, to assert invariants. Client libraries may also use the
`_MSG` variants to provide a message when the assertion fails.

## Recommendations for client code

### C

The Fuchsia System Interface uses symbols with the `zx_` and `fuchsia_` prefixes and
preprocessor macros with the `ZX_` and `FUCHSIA_` prefixes. To avoid collisions, these
prefixes are reserved for use by the Fuchsia IDK. Clients of the Fuchsia IDK should not
declare symbols or preprocessor macros with these prefixes.

### C++

The FIDL protocols included in the Fuchsia System Interface resides in the top-level
`fuchsia` namespace. To avoid collisions, this namespace is reserved for use by the
Fuchsia IDK. Clients of the Fuchsia IDK should not declare names in the top-level
`fuchsia` namespace.

[Fuchsia API Council]: /docs/contribute/governance/api_council.md
[FIDL API style rubric]: /docs/development/languages/fidl/guides/style.md
