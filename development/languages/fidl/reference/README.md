# Introduction

The reference section provides the following material:

* [Attributes](attributes.md) &mdash; describes the available FIDL attributes
* [Compiler](compiler.md) &mdash; describes the organization of the compiler
* [Editors](editors.md) &mdash; discusses support for FIDL in IDEs and stand-alone editors
* [Grammar](grammar.md) &mdash; the FIDL grammar
* [JSON IR](json-ir.md) &mdash; a tour of the JSON Intermediate Representation (**JSON IR**) generator
* [Language](language.md) &mdash; defines the syntax of the FIDL language
* [Wire Format](wire-format/index.md) &mdash; details the byte-by-byte organization of data
* [Host](host.md) &mdash; summary of the parts of FIDL that are allowed on host

### Readability rubric

Fuchsia has adopted a
[readability rubric](https://fuchsia.googlesource.com/docs/+/master/development/api/fidl.md)
for FIDL libraries.

## Bindings

### C

- [Documentation](../languages/c.md)
- [Echo server example](https://fuchsia.googlesource.com/garnet/+/master/examples/fidl/echo2_server_c/)

### C++

- [Documentation](../languages/cpp.md)
- [Echo server example](https://fuchsia.googlesource.com/garnet/+/master/examples/fidl/echo2_server_cpp/)
- [Echo client example](https://fuchsia.googlesource.com/garnet/+/master/examples/fidl/echo2_client_cpp/)

### Dart

- [Documentation](../languages/dart.md)
- [Echo server example](https://fuchsia.googlesource.com/topaz/+/master/examples/fidl/echo_server_dart/)
- [Echo client example](https://fuchsia.googlesource.com/topaz/+/master/examples/fidl/echo_client_dart/)

### Go

- [Documentation](../languages/go.md)
- [Echo server example](https://fuchsia.googlesource.com/garnet/+/master/examples/fidl/echo2_server_go/)
- [Echo client example](https://fuchsia.googlesource.com/garnet/+/master/examples/fidl/echo2_client_go/)

### Rust

- [Documentation](../languages/rust.md)
- [Echo server example](https://fuchsia.googlesource.com/garnet/+/master/examples/fidl/echo2_server_rust/)
- [Echo client example](https://fuchsia.googlesource.com/garnet/+/master/examples/fidl/echo2_client_rust/)

## Learning

See the [tutorial](../tutorial/README.md) to learn about FIDL service development.

FIDL Plugins exist for multiple editors and IDEs.  See the
[editor page](editors.md) to learn more.

## FIDL Tuning Proposals

Substantial changes to FIDL (whether the language, the wire format, or
language bindings) are described in [FIDL Tuning Proposals]. These
decisions are recorded here for posterity. This includes both accepted
and rejected designs. [FTP-001] describes the proposal process itself.

[FIDL Tuning Proposals]: ftp/README.md
[FTP-001]: ftp/ftp-001.md
