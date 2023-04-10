# Fuchsia Programming Language Policy

[TOC]

## Scope

This document describes which programming languages the Fuchsia project uses and
supports for production software on the target device, both within the Fuchsia
Platform Source Tree and for end-developers building for Fuchsia outside the
Fuchsia Source Platform Tree. The policy does not apply to (a) developer
tooling, either on target or host devices, or (b) software on the target device
that is not executed in normal, end-user operation of the device. For example,
this policy does not apply to zxdb (a debugger) because zxdb is a developer
tool; the policy does apply to pkgfs because pkgfs (a file system) executes in
the normal, end-user operation of the device.

## Definitions

The *Fuchsia Platform Source Tree* is the source code hosted on
fuchsia.googlesource.com. The Fuchsia Platform Source Tree can absorb larger
changes to the Fuchsia system and its underlying technologies than
end-developers because changes that impact only the Fuchsia Platform Source Tree
can be executed without coordination with other groups of people.

*End-developers* are people who write software for Fuchsia outside of the
Fuchsia Platform Source Tree. Changes that impact end-developers require more
coordination and take longer to execute than changes that impact only the
Fuchsia Platform Source Tree.

*Supported for end-developers* means that the Fuchsia IDK contains tools and
libraries that help people use the language to develop software for Fuchsia,
including a language-specific backend (and supporting libraries) for FIDL.
Support also implies some level of documentation, including tutorials and
examples, as well as investment from developer relations.

*Strong support for asynchronous programming* means asynchronous programs can be
written using straight-line code (e.g., using async/await in languages like Rust
and Dart).

*Strong encapsulation* is a term of art in object-oriented programming
languages. It means that the language allows class designers to enforce
visibility rules (e.g. `private` in C++ or Java) for the fields of a class and
objects of that class.

## Languages

### C

#### Analysis

*   Pro: C is a widely used language. The language has properties that are
    well-understood, have been stable over a long period of time, and have been
    used to build similar systems in the past. The language has a mature
    toolchain and associated developer tools.
*   Pro: C has a stable ABI, which lets the Fuchsia IDK contain prebuilt
    binaries that end-developers can re-use.
*   Pro: Many languages can interoperate with C using a foreign function
    interface. Supporting C makes it easier for end-developers to integrate
    these languages with Fuchsia.
*   Pro: Our current end-developers already use the language.
*   Con: Support for asynchronous programming is weak.
*   Con: Programs written in the language often have security bugs arising from
    the language’s lack of memory safety.
*   Con: Programs written in the language often contain resource leaks because
    the language does not provide a facility for automatically releasing
    resources.
*   Con: Type safety is weak compared to C++. Simply recompiling some of our C
    code as C++ often results in compiler errors that surface latent bugs in the
    code.

#### Decision

*   C is supported for end-developers. (See
    [the list of supported versions of C](/development/api/c.md#Language-versions).)
*   Within the Fuchsia Platform Source Tree, new uses of C are discouraged. Ask
    the relevant OWNERS for guidance about whether to use C for new code.
*   C is approved for use in the Fuchsia Platform Source Tree:
    *   for low-level systems programming, including within the kernel, and
    *   for defining ABI-stable interfaces to shared libraries and other system
        components.

### C++

#### Analysis

*   Pro: Many of our current end-developers use C++ extensively.
*   Pro: The Fuchsia Platform Source Tree uses C++ extensively.
*   Pro: C++ is a widely used language. The language has properties that are
    well-understood, have been stable over a long period of time, and have been
    used to build similar systems in the past. The language has a mature
    toolchain and associated developer tools.
*   Con: Support for asynchronous programming is weak.
*   Con: Programs written in the language often have security bugs arising from
    the language's lack of memory safety.

#### Decision

*   C++ is supported for end-developers. (See
    [the list of supported versions of C++](/development/api/c.md#Language-versions).)
*   C++ is approved for use throughout the Fuchsia Platform Source Tree.

### Dart

#### Analysis

*   Pro: Asynchronous programs can be written using straight-line code.
*   Pro: People using the language are highly productive.
*   Pro: The language provides memory safety guarantees, which reduces the risk
    of software developed in the language having security bugs.
*   Con: The language uses garbage collection to manage memory, which is more
    resource intensive than other techniques for managing memory.
*   Con: The language has a substantial runtime environment.
*   Con: The toolchain forces a trade-off between binary size, performance, and
    startup latency that is worse than the tradeoff provided by toolchains for
    other languages.
*   Con: Updates of the Dart language, runtimes, and libraries in the Fuchsia
    Platform Source Tree have proven to be a maintenance burden. They make heavy
    usage of the SDK, but are developed out-of-tree.  This ties together Dart
    and Fuchsia versions in a way that makes updates difficult.

#### Decision

*   Dart is supported for end-developers targeting non-drivers.
*   Per
    [RFC-0176](/contribute/governance/rfcs/0176_disallow_new_dart_programs.md),
    use of Dart is allowlisted in the Fuchsia Platform Source Tree, and new
    additions to the allowlist require an exemption.

### Rust

#### Analysis

*   Pro: The Fuchsia Platform Source Tree has had positive implementation
    experience using Rust.
*   Pro: The language provides memory safety guarantees, which reduces the risk
    of software developed in the language having security bugs.
*   Pro: Asynchronous programs can be written using straight-line code.
*   Pro: The Fuchsia project has the opportunity to influence the evolution of
    the language.
*   Con: Rust is not a widely used language. The properties of the language are
    not yet well-understood, having selected an unusual language design point
    (e.g., borrow checker) and having existed only for a relatively short period
    of time.
*   Con: None of our current end-developers use Rust.

#### Decision

*   Rust is not supported for end-developers.
*   Rust is approved for use throughout the Fuchsia Platform Source Tree, with
    the following exceptions:
    *   *kernel*. The Zircon kernel is built using a restricted set of
        technologies that have established industry track records of being used
        in production operating systems.

### Go

#### Analysis

*   Pro: Go is a widely used language within Google.
*   Pro: gVisor has implemented a network stack using the language and that
    network stack has been integrated with Fuchsia.
*   Pro: People using the language are highly productive.
*   Pro: The Fuchsia project has the opportunity to influence the evolution of
    the language.
*   Pro: The language provides memory safety guarantees, which reduces the risk
    of software developed in the language having security bugs.
*   Pro: The language has an extensive ecosystem of libraries that are likely to
    be useful on Fuchsia.
*   Con: The language uses garbage collection to manage memory, which is more
    resource intensive than other techniques for managing memory.
*   Con: The language has a substantial runtime environment.
*   Con: The Fuchsia Platform Source Tree has had negative implementation
    experience using Go. The system components the Fuchsia project has built in
    Go have used more memory and kernel resources than their counterparts (or
    replacements) the Fuchsia project has built using C++ or Rust.
*   Con: The toolchain produces large binaries.

#### Decision

*   Go is not approved, with the following exceptions:
    *   *netstack*. Migrating netstack to another language would require a
        significant investment. In the fullness of time, we should migrate
        netstack to an approved language.
*   All other uses of Go in the Fuchsia Platform Source Tree for production
    software on the target device must be migrated to an approved language.

### Python

#### Analysis

*   Pro: The language is widely used.
*   Pro: The language is used in the build systems of many open-source
    dependencies of Fuchsia.
*   Pro: People using the language are highly productive.
*   Pro: The language has a robust ecosystem of contributors and libraries that
    are likely to be useful on Fuchsia.
*   Pro: The language provides memory safety guarantees, which reduces the risk
    of software developed in the language having security bugs.
*   Con: The language uses garbage collection to manage memory, which is more
    resource intensive than other techniques for managing memory.
*   Con: The language has a substantial runtime environment.
*   Con: The language is not performance-competitive with C++, Rust, or Dart.
*   Con: Programming errors are often detected during execution rather than
    statically.
*   Con: The language lacks strong encapsulation.

#### Decision

*   Python is not supported for end-developers.
*   Python 3 is approved for use in the Fuchsia Platform Source Tree for the
    following purposes:
    *   Build
    *   Host tools
*   Python 2.7 is deprecated and existing uses in the Fuchsia Platform Source
    Tree must be migrated to an approved language.
