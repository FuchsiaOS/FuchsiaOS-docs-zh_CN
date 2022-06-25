# Inspect validator puppet architecture

## Overview

The Validator program sends FIDL commands to control a "puppet" program, which
invokes library functionality to modify some state that the Validator then
evaluates. For more information about the Inspect Validator, see the
[README](README.md).

The Puppet includes these parts:

* Serving a FIDL protocol.
* Unpacking the protocol and making library calls.
* Building an integration test that includes the Puppet and Validator programs.

This doc focuses on the Inspect Validator Rust Puppet located at
[//src/diagnostics/validator/inspect/lib/rust/src/main.rs](/src/diagnostics/validator/inspect/lib/rust/src/main.rs).

## FIDL design

The FIDL protocol for Inspect Validator is defined in
[//src/diagnostics/validator/inspect/fidl/validate.test.fidl](/src/diagnostics/validator/inspect/fidl/validate.test.fidl).
The FIDL protocol corresponds closely to the functions in the
[Inspect library API](development/diagnostics/inspect/README.md)
which defines actions to be applied to any Inspect API implementation. The FIDL
API is written to correspond to the
[Rust API](https://fuchsia-docs.firebaseapp.com/rust/fuchsia_inspect/index.html).

(Note: Inspect APIs are allowed to differ from the Rust API; such APIs may
require puppet code architecture modifications.)

## Serving FIDL

The `main()` function performs boilerplate to serve a single FIDL client
through `run_driver_service()`, which receives either `Initialize` or `Act`
events from the FIDL stream. `Act` events are unpacked by the `Actor` object
which maintains the state necessary to control the Inspect library.

## Actor and the Inspect library

`Actor` contains an `Inspector` (the Inspect library's entry-point object),
a hashmap of `nodes`, and a hashmap of `properties`. It implements one
function, `act()`, which contains a giant `match` statement ("switch" or "case"
in other languages) to invoke each action that the library implements.
Puppets can report `Unimplemented` for actions the library doesn't support.

After the Validator invokes each action, it will test the library's effect on
the VMO. The library should handle propagating the effects of actions so that
the Validator can see them.

The hashmaps of `nodes` and `properties` store values that are returned by the
Inspect library. Since Rust is an RAII language that cleans up automatically
when reference to memory is lost, failing to store a node or property would
cause immediate deletion of that node or property. Also, storing properties
allows updating their values in response to FIDL commands.

## Testing and the build system

The Validator and Puppet combination should make a hermetic integration test.

Note: due to limitations in the Dart build-system macros, the Dart test isn't
fully hermetic.

### Dependencies and names

[Validator's BUILD.gn file](/src/diagnostics/validator/inspect/BUILD.gn#21)
defines a `validator_bin` target, which is used by the
[Rust puppet's BUILD.gn file](/src/diagnostics/validator/inspect/lib/rust/BUILD.gn#33)
as a dependency to the `test_package()` named `inspect_validator_test_rust`
which is the test that exercises the Rust puppet.

The Rust puppet itself is
[built as a standard rustc_binary](/src/diagnostics/validator/inspect/lib/rust/BUILD.gn#10).
That build rule produces two names, `inspect_validator_rust_puppet_bin`, which
is included in the deps of the `test_package()` rule, and
`inspect_validator_rust_puppet`, which is included in the binaries of the
`test_package()`.

The `validator_bin` target from the
[Validator's Build.gn file](/src/diagnostics/validator/inspect/BUILD.gn#21)
has a name of `validator`, which is referred to in the `tests` of the
`test_package()`.

### CQ/CI

Putting `inspect_validator_test_rust` in the `deps` of `group("tests")` in its
[BUILD.gn](/src/diagnostics/validator/inspect/lib/rust/BUILD.gn#59)
makes it easy to include `inspect_validator/lib/rust:tests` in the `deps` of
`group("tests")` of [src/diagnostics/BUILD.gn](/src/diagnostics/BUILD.gn).
This will be picked up by the build system and cause the Inspect Validator Rust
Puppet test to be run in CQ and CI.

### Meta .cmx files

There are the following CMX files in [//src/diagnostics/validator/inspect/lib/rust/meta](/src/diagnostics/validator/inspect/lib/rust/meta):

* [inspect-validator-rust-puppet.cmx](/src/diagnostics/validator/inspect/lib/rust/meta/inspect-validator-rust-puppet.cmx)

  Lets the puppet binary run and use the logger. It's referred to in the
`meta` section of `test_package("inspect_validator_test_rust")`.
* [validator.cmx](/src/diagnostics/validator/inspect/lib/rust/meta/validator.cmx)

  This CMX file is implicitly referred to by the `tests: name: "validator"` that you specified in `test_package()`.
    * `sandbox: services` specifies the services that the Validator needs to run.
    * `program: args` supplies command-line arguments to the Validator, including the
     complete URL of the Rust puppet.
    * `program: binary` confirms that you want to run the
    `tests: name: "validator"`.

## Running Validator

For information on how to run the Validator against a puppet, see
[Inspect Validator](/src/diagnostics/validator/inspect/README.md).
