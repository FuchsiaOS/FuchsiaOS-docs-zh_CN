# Generated code output guide

This document outlines the approaches available for viewing generated FIDL
bindings code. For general information about how FIDL definitions get converted
to target language code, refer to the [bindings reference][bindings-ref].

The FIDL library `fuchsia.io` is used throughout as a running example. It is
defined in [//sdk/fidl/fuchsia.io](/sdk/fidl/fuchsia.io).

## Viewing generated code

This section shows how to directly access generated code files.

### GN build

If you would like to look at the generated output for a specific target in the
Fuchsia build, you can first build the target in question, then inspect the
generated files in the build output. The instructions in this section assume
that the target is defined using the standard [`fidl` GN template][fidl-gn], and
that the build directory is set to the default path (`out/default`). `fx status`
can be used to find the current build directory.

You can find most FIDL output in the GN build in the `fidling` toolchain. For
`fuchsia.io`, the root of the output is
`out/default/fidling/gen/sdk/fidl/fuchsia.io`. The rest of this document refers
to this as the root directory. It is structured as follows:

    out/default/fidling/gen/sdk/fidl/fuchsia.io
    ├── fuchsia.io.fidl.json
    └── fuchsia.io
       ├── fidl_fuchsia_io.rs
       ├── fuchsia.io.fidl
       │  └── impl.go
       ├── c
       │  └── fuchsia
       │     └── io
       │        └── c
       │           ├── fidl.h
       │           ├── fidl.client.c
       │           └── fidl.server.c
       ├── llcpp
       │  └── fuchsia
       │     └── io
       │        └── llcpp
       │           ├── fidl.h
       │           ├── fidl.cc
       │           └── fidl_test_base.h
       └── hlcpp
          └── fuchsia
             └── io
                └── cpp
                   ├── fidl.h
                   ├── fidl.cc
                   └── fidl_test_base.h

The exception is [Dart code](#dart), which is generated in the `dartlang`
toolchain. For `fuchsia.io`, the root of the Dart output is
`out/default/dartlang/gen/sdk/fidl/fuchsia.io`. The rest of this document refers
to this as the Dart root directory. It is structured as follows:

    out/default/dartlang/gen/sdk/fidl/fuchsia.io
    └── fuchsia.io_package
       └── lib
          ├── fidl_async.dart
          └── fidl_test.dart

Many generated code paths seem to contain duplicate directory names, such as
`.../fuchsia.io/fuchsia.io/...`. This occurs because FIDL libraries are usually
defined in a directory named after the library, with a target named after the
library. The first `fuchsia.io` comes from the directory, and the second comes
from the GN target name.

#### JSON IR

The JSON IR is generated in the root directory as `${target_name}.fidl.json`.
For example, with `fuchsia.io`:

    out/default/fidling/gen/sdk/fidl/fuchsia.io/fuchsia.io.fidl.json

#### Rust {#rust}

The Rust bindings are generated in the root directory as
`${target_name}/${rust_name}.rs`, where `${rust_name}` is derived from the
library name by replacing dots with underscores. For example, with `fuchsia.io`:

    out/default/fidling/gen/sdk/fidl/fuchsia.io/fuchsia.io/fidl_fuchsia_io.rs

#### Go {#go}

The Go bindings are generated in the root directory as
`${target_name}/${library_name}.fidl/impl.go`. For example, with `fuchsia.io`:

    out/default/fidling/gen/sdk/fidl/fuchsia.io/fuchsia.io/fuchsia.io.fidl/impl.go

#### C++ {#c-family}

C++ bindings use a source layout in subdirectories of the root
directory follow the pattern: `${target_name}/${binding_flavor}/fuchsia.io/cpp`.

From there LLCPP outputs `wire.cc`, `wire.h` and `wire_test_base.h`.
As the unified C++ bindings take shape more bindings will follow this pattern.
See below for how the not-yet unified bindings are generated.

For example, using `fuchsia.io` with the LLCPP bindings creates the following
files:

    out/default/fidling/gen/sdk/fidl/fuchsia.io/fuchsia.io/llcpp/fidl/fuchsia.io/cpp/wire.cc
    out/default/fidling/gen/sdk/fidl/fuchsia.io/fuchsia.io/llcpp/fidl/fuchsia.io/cpp/wire.h
    out/default/fidling/gen/sdk/fidl/fuchsia.io/fuchsia.io/llcpp/fidl/fuchsia.io/cpp/wire_test_base.h

#### HLCPP, and C {#hlcpp-c}

HLCPP and C bindings are generated in subdirectories of the root directory
following the pattern `${target_name}/${binding_flavor}/fuchsia/io`. From there,

- HLCPP outputs `cpp/fidl.cc`, `cpp/fidl.h`, and `cpp/fidl_test_base.h`.
- C outputs `c/fidl.client.c`, `c/fidl.server.c`, and `fidl.h`.

For example, using `fuchsia.io` with the HLCPP bindings creates the
following files:

    out/default/fidling/gen/sdk/fidl/fuchsia.io/fuchsia.io/hlcpp/fuchsia/io/cpp/fidl.cc
    out/default/fidling/gen/sdk/fidl/fuchsia.io/fuchsia.io/hlcpp/fuchsia/io/cpp/fidl.h
    out/default/fidling/gen/sdk/fidl/fuchsia.io/fuchsia.io/hlcpp/fuchsia/io/cpp/fidl_test_base.h

#### Dart {#dart}

The Dart bindings are generating in the Dart root directory as
`${target_name}/${library_name}_package`. For example, with `fuchsia.io`:

    out/default/dartlang/gen/sdk/fidl/fuchsia.io/fuchsia.io_package

Within the package, `lib/fidl_async.dart` contains the bindings code.
`lib/fidl_test.dart` contains utilities for [testing][dart-testing].

## Using fidlbolt

For ad hoc examples or existing FIDL files, another option is to use the
`fidlbolt` tool. By pasting the desired FIDL content into fidlbolt, it is
possible to view the output for each binding, as well as for the JSON IR.
`fidlbolt` also supports viewing libraries defined in the SDK, e.g. `library
fuchsia.io;`, as well as importing them, e.g. `using fuchsia.io;`.

## Viewing generated documentation

### Rust

Documentation for all Rust crates used in Fuchsia, included Rust bindings of
FIDL libraries, is hosted online at
[fuchsia-docs.firebaseapp.com](https://fuchsia-docs.firebaseapp.com/rust).
You can generate offline documentation with [`fx rustdoc`][rustdoc].

<!-- xrefs -->
[bindings-ref]: /docs/reference/fidl/bindings/overview.md
[fidl-gn]: /build/fidl/fidl.gni
[rustdoc]: /docs/development/languages/rust/fidl_crates.md#documentation
[dart-testing]: /docs/reference/fidl/bindings/dart-bindings.md#test-scaffolding
