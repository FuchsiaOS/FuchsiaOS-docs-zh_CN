
# FIDL Rust Crates

FIDL is the primary mechanism for structured IPC within Fuchsia. The easiest
way to use FIDL from Rust is by generating a "FIDL crate" from a FIDL library
and then importing it from your Rust library or binary.

See [the FIDL Rust bindings](/docs/reference/fidl/bindings/rust-bindings.md) to
understand how different FIDL constructs map into their Rust equivalents, and
[the FIDL Rust tutorials][tutorials] for examples on using the Rust
bindings.

## Build Instructions {#build}

When a [GN `fidl` rule](/build/fidl/fidl.gni) is defined for a FIDL library,
a correspoding FIDL Rust crate is automatically generated under
the original target name appended with `-rustc`. Transitive dependencies on
other FIDL libraries are resolved automatically.
For example, given the declaration:

```gn
# //src/tictactoe/BUILD.gn

fidl("games.tictactoe") { ... }
```

The FIDL crate target is
`//src/tictactoe:games.tictactoe-rustc`. To use the FIDL crate,
add the target to the `deps` field of the
[`rustc_*` build rule](/docs/development/languages/rust/README.md#build)
for your Rust crate. For example:

```gn
rustc_binary("tictactoe") {
  # ...
  deps = ["//src/tictactoe:games.tictactoe-rustc"]
}
```

The Rust crate will be named `fidl_games_tictactoe` and its items can now be
imported:

```rust
use fidl_games_tictactoe::BOARD_SIZE;
```

In the Fuchsia tree, frequently used FIDL crates are often aliased to
a shorter name for brevity, like so:

```rust
use fidl_fuchsia_io2 as fio2;
use fidl_fuchsia_data as fdata;
```

## Generated Documentation {#documentation}

Documentation in HTML format can be automatically
generated for a FIDL crate using the `fx rustdoc` command. For example:

```bash
fx rustdoc //src/tictactoe:games.tictactoe-rustc --open
```

FIDL crates in the public Fuchsia source tree are published in the
[Fuchsia Rust API reference](https://fuchsia-docs.firebaseapp.com/rust/).

## Generated Rust Code {#code}

To manually inspect the generated Rust code for a FIDL crate, the Rust
source files are available under the `BUILD_DIR/fidling/gen` (refer to the
[Generated code guide][generated-code] for an example). Note that
the FIDL crate must first have been built (e.g. using `fx build`).

<!-- xrefs -->
[generated-code]: /docs/development/languages/fidl/guides/generated-code.md#rust
[tutorials]: /docs/development/languages/fidl/tutorials/rust
