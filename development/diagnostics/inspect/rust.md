# Rust libraries

This document explains what libraries are available for writing and reading Inspect data in Rust.
For specific documentation of each library, refer to the crate documentation linked on each section.

## Libraries for writing Inspect

### [`fuchsia-inspect`][fuchsia_inspect]

This is the core library. This library offers the core API for creating nodes, properties,
serving Inspect, etc. Internally it implements the buddy allocation algorithm described in
[Inspect vmo format][inspect_vmo_format].

For an introduction to Inspect concepts and the rust libraries, see the
[codelab][codelab].

## [`inspect-runtime`][inspect_runtime]

This library enables components to serve Inspect and make it available to the
Archivist for collection.

It's intended to be used only by component binaries, not libraries. The reason for this
is that this library provides an `expect_includes` check to verify that component manifests are
configured to expose Inspect.

Avoid depending on `inspect-runtime` from libraries so that tests are not forced
to add the Inspect `client.shard.cml`.

### [`fuchsia-inspect-contrib`][fuchsia_inspect_contrib]

This library is intended for contributions to the Inspect library from clients.
These are patterns that clients identify in their usage of Inspect that they can
generalize and share. It’s intended to be at a higher level than
`fuchsia-inspect`.

### [`fuchsia-inspect-derive`][fuchsia_inspect_derive]

This library provides a convenient way to manage Inspect data in a Rust program through a
`#[derive(Inspect)]` procedural macro. This works at a higher level than `fuchsia-inspect`.
For more information on this library, see [Ergonomic Inspect][ergonomic_inspect].

## Libraries for reading Inspect

These libraries are not specific to Inspect and are used for various kinds of diagnostics data.

### [`diagnostics-hierarchy`][diagnostics_hierarchy]

This library includes the convenient macro `assert_data_tree` for testing as well as the
definition of the `DiagnosticsHierarchy`, which is not exclusive to Inspect and
is also used for logs and other diagnostics data sources.

### [`diagnostics-testing`][diagnostics_testing]

This library includes the convenient `EnvForDiagnostics` which is useful for testing Inspect
integration with [legacy components][components-v1].

### [`diagnostics-reader`][diagnostics_reader]

This library includes the convenient `ArchiveReader` which is useful for fetching Inspect
data from an archivist in a test or in production. It wraps the shared logic of
connecting to the `ArchiveAccessor` and fetching data from it.

## Others


### [`inspect_format`][inspect_format]

This library provides an API for reading and writing the blocks of the
[Inspect VMO format][inspect_vmo_format].


[codelab]: development/diagnostics/inspect/codelab/codelab.md#rust
[components-v1]: concepts/components/v1/README.md
[ergonomic_inspect]: development/languages/rust/ergonomic_inspect.md
[inspect_vmo_format]: reference/platform-spec/diagnostics/inspect-vmo-format.md
[inspect_format]: https://fuchsia-docs.firebaseapp.com/rust/inspect_format/index.html
[inspect_runtime]: https://fuchsia-docs.firebaseapp.com/rust/inspect_runtime/index.html
[fuchsia_inspect_derive]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia_inspect_derive/index.html
[fuchsia_inspect]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia_inspect/index.html
[fuchsia_inspect_contrib]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia_inspect_contrib/index.html
[diagnostics_hierarchy]: https://fuchsia-docs.firebaseapp.com/rust/diagnostics_hierarchy/index.html
[diagnostics_reader]: https://fuchsia-docs.firebaseapp.com/rust/diagnostics_reader/index.html
[diagnostics_testing]: https://fuchsia-docs.firebaseapp.com/rust/diagnostics_testing/index.html
