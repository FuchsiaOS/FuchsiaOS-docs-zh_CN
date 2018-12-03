# Overview

This document is a description of the Fuchsia Interface Definition Language
(FIDL) purpose, high-level goals, and requirements.

## Related Documents

*   [Wire Format Specification]
*   [Language Specification]
*   [Compiler Specification]
*   [API Readability / Style Guide]
*   [C Language Bindings]
*   [C++ Language Bindings]
*   [Examples]: Some small example code used during development
*   [Tutorial]: A tutorial on using FIDL services in several languages

<!-- Reference links because these are used again below. -->

[Wire Format Specification]: ../reference/wire-format/index.md
[Language Specification]: ../reference/language.md
[Compiler Specification]: ../reference/compiler.md
[API Readability / Style Guide]: ../../../api/fidl.md
[C Language Bindings]: ../languages/c.md
[C++ Language Bindings]: ../languages/cpp.md
[Examples]: https://fuchsia.googlesource.com/zircon/+/master/system/host/fidl/examples
[Tutorial]: ../tutorial/README.md

[TOC]

## Overview

The Fuchsia Interface Definition Language (FIDL) is the language used to
describe interprocess communication (IPC) protocols used by programs running on
the Fuchsia Operating System. FIDL is supported by a toolchain (compiler) and
runtime support libraries (bindings) to help developers use IPC effectively.

Goals

Fuchsia extensively relies on IPC since it has a microkernel architecture
wherein most functionality is implemented in user space outside of the kernel,
including privileged components such as device drivers. Consequently the IPC
mechanism must be efficient, deterministic, robust, and easy to use.

**IPC efficiency** pertains to the computational overhead required to generate,
transfer, and consume messages between processes. IPC will be involved in all
aspects of system operation so it must be efficient. The FIDL compiler must
generate tight code without excess indirection or hidden costs. It should be at
least as good as hand-rolled code would be where it matters most.

**IPC determinism** pertains to the ability to perform transactions within a
known resource envelope. IPC will be used extensively by critical system
services such as filesystems which serve many clients and which must perform in
predictable ways. The FIDL wire format must offer strong static guarantees such
as ensuring that structure size and layout is invariant thereby alleviating the
need for dynamic memory allocation or complex validation rules.

**IPC robustness** pertains to the need to consider IPC as an essential part of
the operating system's ABI. Maintaining binary stability is crucial. Mechanisms
for protocol evolution must be designed conservatively so as not to violate the
invariants of existing services and their clients, particularly when the need
for determinism is also considered. The FIDL bindings must perform effective,
lightweight, and strict validation.

**IPC ease of use** pertains to the fact that IPC protocols are an essential
part of the operating system's API. It is important to provide good developer
ergonomics for accessing services via IPC. The FIDL code generator removes the
burden of writing IPC bindings by hand. Moreover, the FIDL code generator can
produce different bindings to suit the needs of different audiences and their
idioms.

TODO: express goal of meeting the needs of different audiences using
appropriately tailored bindings, eg. system programming native vs. event-driven
dispatch vs. async calls, etc... say more things about FIDL as our system API,
SDK concerns, etc.

Requirements

# Purpose

*   Describe data structures and interfaces used by IPC protocols on Zircon.
*   Optimized for interprocess communication only; FIDL must not be persisted to
    disk or used for network transfer across device boundaries.
*   Efficiently transport messages consisting of data (bytes) and capabilities
    (handles) over Zircon channels between processes running on the same
    device.
*   Designed specifically to facilitate effective use of Zircon primitives; not
    intended for use on other platforms; not portable.
*   Offers convenient APIs for creating, sending, receiving, and consuming
    messages.
*   Perform sufficient validation to maintain protocol invariants (but no more
    than that).

# Efficiency

*   Just as efficient (speed and memory) as using hand-rolled data structures
    would be.
*   Wire format uses uncompressed native datatypes with host-endianness and
    correct alignment to support in-place access of message contents.
*   No dynamic memory allocation is required to produce or to consume messages
    when their size is statically known or bounded.
*   Explicitly handle ownership with move-only semantics.
*   Data structure packing order is canonical, unambiguous, and has minimum
    padding.
*   Avoid back-patching pointers.
*   Avoid expensive validation.
*   Avoid calculations which may overflow.
*   Leverage pipelining of interface requests for asynchronous operation.
*   Structures are fixed size; variable-size data is stored out-of-line.
*   Structures are not self-described; FIDL files describe their contents.
*   No versioning of structures, but interfaces can be extended with new methods
    for protocol evolution.

# Ergonomics

*   Programming language bindings maintained by Fuchsia team:
    *   C, C++ (native), C++ (idiomatic), Dart, Go, Rust
*   Keeping in mind we might want to support other languages in the future, such
    as:
    *   Java, Javascript, etc.
*   The bindings and generated code are available in native or idiomatic flavors
    depending on the intended application.
*   Use compile-time code generation to optimize message serialization,
    deserialization, and validation.
*   FIDL syntax is familiar, easily accessible, and programming language
    agnostic.
*   FIDL provides a library system to simplify deployment and use by other
    developers.
*   FIDL expresses the most common data types needed for system APIs; it does
    not seek to provide a comprehensive one-to-one mapping of all types offered
    by all programming languages.

# Implementation

*   Compiler is written in C++ to be usable by components built in Zircon.

*   Compiler is portable and can be built with a host toolchain.

*   We will not support FIDL bindings for any platform other than Fuchsia.

## Where to Find the Code

- [The compiler](../../system/host/fidl)
- [C bindings](../../system/ulib/fidl)
- [C++ bindings](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/cpp)
- [Go bindings](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/go)
- [Rust bindings](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/rust)

## Constituent Parts of Specification

### FIDL Wire Format

The FIDL wire format specified how FIDL messages are represented in memory for
transmission over IPC.

The FIDL wire format is documented [Wire Format Specification].

### FIDL Language

The FIDL language is the syntax by which interfaces are described in ***.fidl**
files.

The FIDL language is documented [Language Specification].

### FIDL Compiler

The FIDL compiler generates code for programs to use and implement interfaces
described by the FIDL language.

The FIDL compiler is documented [Compiler Specification].

### FIDL Bindings

FIDL bindings are language-specific runtime support libraries and code
generators which provide APIs for manipulating FIDL data structures and
interfaces.

Languages-specific topics:

*   [C Language Bindings]
*   [C++ Language Bindings]

Bindings are available in various flavors depending on the language:

*   **Native bindings**: designed for highly sensitive contexts such as device
    drivers and high-throughput servers, leverage in-place access, avoid memory
    allocation, but may require somewhat more awareness of the constraints of
    the protocol on the part of the developer.
*   **Idiomatic bindings**: designed to be more developer-friendly by copying
    data from the wire format into easier to use data types (such as heap-backed
    strings or vectors), but correspondingly somewhat less efficient as a
    result.

Bindings offer several various ways of invoking interface methods depending on
the language:

*   **Send/receive**: read or write messages directly to a channel, no built-in
    wait loop (C)
*   **Callback-based**: received messages are dispatched asynchronously as
    callbacks on an event loop (C++, Dart)
*   **Port-based**: received messages are delivered to a port or future (Rust)
*   **Synchronous call**: waits for reply and return it (Go, C++ unit tests)

Bindings provide some or all of the following principal operations:

*   **Encode**: in-place transform native data structures into the wire format
    (coupled with validation)
*   **Decode**: in-place transform wire format data into native data structures
    (coupled with validation)
*   **Copy/Move To Idiomatic Form**: copy contents of native data structures
    into idiomatic data structures, handles are moved
*   **Copy/Move To Native Form**: copy contents of idiomatic data structures
    into native data structures, handles are moved
*   **Clone**: copy native or idiomatic data structures (that do not contain
    move-only types)
*   **Call**: invoke interface method

## Workflow

This section describes the workflow of authors, publishers, and consumers of IPC
protocols described using FIDL.

# Authoring FIDL

The author of a FIDL based protocol creates one or more ***.fidl files** to
describe their data structures and interfaces.

FIDL files are grouped into one or more **FIDL libraries** by the author. Each
library represents a group of logically related functionality with a unique
library name. FIDL files within the same library implicitly have access to all
other declarations within the same library. The order of declarations within the
FIDL files that make up a library is not significant.

FIDL files of one library can access declarations within another FIDL library by
**importing** the other FIDL module. Importing other FIDL libraries makes their
symbols available for use thereby enabling the construction of protocols derived
from them. Imported symbols must be qualified by the library name or by an alias
to prevent namespace collisions.

# Publishing FIDL

The publisher of a FIDL based protocol is responsible for making FIDL libraries
available to consumers. For example, the author may disseminate FIDL libraries in
a public source repository or distribute them as part of an SDK.

Consumers need only point the FIDL compiler at the directory which contains the
FIDL files for a library (and its dependencies) to generate code for that
library. The precise details for how this is done will generally be addressed by
the consumer's build system.

# Consuming FIDL

The consumer of a FIDL based protocol uses the FIDL compiler to generate code
suitable for use with their language runtime specific bindings. For certain
language runtimes, the consumer may have a choice of a few different flavors of
generated code all of which are interoperable at the wire format level but
perhaps not at the source level.

In the Fuchsia world build environment, generating code from FIDL libraries will
be done automatically for all relevant languages by individual FIDL build
targets for each library.

In the Fuchsia SDK environment, generating code from FIDL libraries will be done
as part of compiling the applications which use them.
