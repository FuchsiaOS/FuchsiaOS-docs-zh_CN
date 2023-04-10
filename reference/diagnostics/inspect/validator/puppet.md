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
[Inspect library API](/docs/development/diagnostics/inspect/README.md)
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

### Dependencies and names

[Validator's BUILD.gn file](/src/diagnostics/validator/inspect/BUILD.gn#21)
defines a `validator_bin` target, which is used by the
[Rust puppet's BUILD.gn file](/src/diagnostics/validator/inspect/lib/rust/BUILD.gn#33)
as a dependency to the `test_package()` named `inspect_validator_test_rust`
which is the test that exercises the Rust puppet.

The Rust puppet itself is
[built as a standard rustc_binary](/src/diagnostics/validator/inspect/lib/rust/BUILD.gn#8).
That build rule produces
`inspect_validator_rust_puppet`, which is included in the binaries of the
`fuchsia_unittest_package()`.

The `validator_bin` target from the
[Validator's Build.gn file](/src/diagnostics/validator/inspect/BUILD.gn)
is included in the `deps` of the
`fuchsia_unittest_package()`.

### CQ/CI

Putting `inspect_validator_test_rust` in the `deps` of `group("tests")` in its
[BUILD.gn](/src/diagnostics/validator/inspect/lib/rust/BUILD.gn)
makes it easy to include `inspect_validator/lib/rust:tests` in the `deps` of
`group("tests")` of [src/diagnostics/BUILD.gn](/src/diagnostics/BUILD.gn).
This will be picked up by the build system and cause the Inspect Validator Rust
Puppet test to be run in CQ and CI.

### Meta .cml files

There are the following CML files in [//src/diagnostics/validator/inspect/meta](/src/diagnostics/validator/inspect/meta):

* [puppet.shard.cml](/src/diagnostics/validator/inspect/meta/puppet.shard.cml)

  Lets the puppet binary run, use the logger and Inspect, and serve the Validate protocol.
* [validator.shard.cml](/src/diagnostics/validator/inspect/meta/validator.shard.cml)

    * `use: protocol:` specifies the services that the Validator needs to run.
        * `fuchsia.diagnostics.ArchiveAccessor` lets it read the puppet's published Inspect data.
        * `test.inspect.validate.Validate` lets it control the puppet.
        * `fuchsia.sys2.LifecycleController` lets it shut down (and thus restart) the puppet
          between trials.
    * `children: name: "puppet"` places the puppet in the component hierarchy.
    * `children: url: "#meta/puppet.cm"` allows the puppet to be found and loaded.
    * `offer: protocol: "fuchsia.logger.LogSink"` is needed for the puppet's logs to be visible.

These shards are used by the puppet's and validator's CML files in each puppet directory.
Currently there are 4 puppets:

* [Rust](/src/diagnostics/validator/inspect/lib/rust)
* [C++](/src/diagnostics/validator/inspect/lib/cpp)
* [Dart](/sdk/dart/fuchsia_inspect/test/validator_puppet)
* [Go](/src/connectivity/network/netstack/inspect/validator)

The puppet's CML is referred to in the `manifest` key of `fuchsia_component()` (for Rust or C++) or
`dart_component()` (for Dart) or `fuchsia_unittest_component()` (for Go).

The Validator conroller's CML is referred to in the `manifest` key of `fuchsia_unittest_package()`
(for Rust or C++) or `fuchsia_test_component()` (for Dart) or `fuchsia_unittest_component()`
(for Go).

    * `program: args` supplies command-line arguments to the Validator. Dart needs special handling,
      and each puppet gets its own printable-name.
    * `program: binary` declares that you want to run the `bin/validator"` binary created by
      `rustc_binary("validator_bin")` and linked via
      `"//src/diagnostics/validator/inspect:validator_bin"` in the `deps` of
      `fuchsia_unittest_package()` (for Rust and C++) or `fuchsia_test_component()` (for Dart) or
      `fuchsia_unittest_component()` (for Go).
