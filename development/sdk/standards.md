SDK Standards
=============

This document describes the standards for how we develop the Fuchsia SDK within
the Fuchsia source tree. Some of the information in this document might be of
interest to clients of the Fuchsia SDK, but the primary focus of the document is
how the Fuchsia project develops the SDK.

## FIDL interfaces and libzircon are the system ABI

Broadly speaking, the binary interface to the system is defined by the FIDL
wireformat used by programs to communicate with the rest of the system and the
syscalls exposed in `libzircon`. In particular, the system should not rely upon
programs using any specific client libraries, including `libc`.

The Fuchsia SDK contains a number of client libraries (i.e., libraries that
clients of the SDK can link into their programs), but all of these libraries are
optional and provided for the convenience of clients, not for the convenience of
the system.

## FIDL Interface Definitions

### Binary stability

FIDL interfaces are defined in `.fidl` files, which are contained in the SDK.
All the FIDL definitions that have been published in an SDK should be considered
public ABI for the system. The system might also contain additional FIDL
definitions that have not been published in an SDK. Those definitions are
subject to change without notice and programs that rely upon their ABI might not
work properly in future versions of the system.

### Source stability

FIDL definitions in the SDK might evolve in source-incompatible ways. For
example, we might rename a method in an interface while maintaining the its
ordinal and semantics. Such a change preserves the ABI but breaks source
compatibility.

We do not currently have any standards about when we should break source
compatibility.

### Naming

Public FIDL definitions are located in the source tree at a path with the
following pattern: `$LAYER/public/lib/$NAME/fidl`. The target name should be
`fidl`. Related FIDL libraries can be grouped in a directory. For example,
`$LAYER/public/lib/$GROUP/$NAME/fidl`.

### Style

FIDL definitions in the SDK should follow the [FIDL API readability rubric].

## Client Libraries

### Stability

Client libraries are neither source nor binary stable. Clients that wish to use
these libraries should link them into their programs, either statically or
dynamically.

Programs load dynamic libraries from their own package, which means different
programs on the system might be using different versions of the same dynamic
library concurrently. Programs that wish to use dynamic libraries (including
`libc`) should include those libraries in the `lib` directory of their package.

### Precompiled libraries

The Fuchsia SDK does not require clients to use a specific toolchain. For this
reason, precompiled libraries that clients link against must have C linkage. For
example, a precompiled library cannot export C++ symbols because C++ does not
have a standard ABI across toolchains (or even toolchain versions).

The SDK can also contain precompiled shared libraries with C++ linkage that are
linked by other precompiled libraries in the SDK. Clients are not expected to
link against these libraries directly.

### Dependencies

A client that takes a dependency on a client library must also take a dependency
on all the dependencies of that library. For this reason, client libraries
should have minimal dependencies. For example, client libraries should avoid
dependencies on FBL, FXL, FSL, or other "base" libraries.

Client libraries that need to perform asynchronous operations should depend on
`libasync.a` and `libasync-default.so`. However, these libraries should not
assume the client is using any specific implementation of `async_dispatcher_t*`.
For example, these libraries should not assume the `async_dispatcher_t*` is
actually implemented by `libasync-loop.a`. Libraries that require
`async_get_default_dispatcher` to be populated should state that requirement in
their documentation.

Precompiled libraries can have more extensive dependencies if those dependencies
are hidden from their client. For example, a precompiled shared library should
not export symbols from these dependences and should not have headers that
transitively include headers from these dependencies.

### Naming

Client libraries should be named according to the language the expect their
clients to use. For example, the C++ variant of the `$NAME` library should be
located in the source tree at a path with the following pattern:
`$LAYER/public/lib/$NAME/cpp`.

In some cases, a library makes sense only for one language. In that case, the
language suffix may be omitted unless the client library has an associated
FIDL interface.

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


[FIDL API readability rubric]: ../api/fidl.md
