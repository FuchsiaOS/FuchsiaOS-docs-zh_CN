# Using cargo on Fuchsia

Many tools in the Rust ecosystem assume you are using cargo. For tools like
this, which don't have a counterpart in our build, there is a utility for
generating `Cargo.toml` files. You can run cargo itself (`cargo check` for
instance) using it.

This functionality is maintained by volunteers. GN and Cargo have some design
mismatches that may result in the generated `Cargo.toml` files not working at
all or needing manual tweaks. Because of this, cargo in the Fuchsia tree is
**not** officially supported; things may break from time to time.

### Generating `Cargo.toml` files {#cargo-toml-gen}

In order to generate the cargo files based on the build graph of GN, add `--cargo-toml-gen` to
the `fx set` command or the `//build/rust:cargo_toml_gen` target to `$OUT_DIR/args.gn`'s
`universe_package_labels`. This adds a few seconds to `gn gen`. Make sure to run
a full `fx build` after updating the setting:

```sh
fx set PRODUCT.BOARD --cargo-toml-gen <other fx args>
fx build
```

**Note:** If a `Cargo.toml` is required to complete an `fx build`, such as in the
case `cbindgen` needs to be run to generate new C bindings for a Rust crate, you may
need to use the `//build/rust:cargo_toml_gen` build target instead. This target will only
build the `Cargo.toml` files.

Most editors require the `Cargo.toml` file to be in a location that is adjacent to
the `src/` directory. Symlinks to these files can be generated using the following
commands, where `//garnet/foo/path/to/target:label` is the GN target that you want
to work on:

```sh
fx gen-cargo garnet/foo/path/to/target:some_label
```

**Note:** The above will not work for `rustc_staticlib` targets, e.g. Rust
crates used to generate C bindings generally use the `rustc_staticlib` target
template. For `rustc_staticlib` targets, you should use the following command instead.

```sh
fx gen-cargo garnet/foo/path/to/target:_some_label_rustc_static
```

Note that this label must point to a [`rustc_...` GN template](README.md#build)
(not a Fuchsia package or other GN target). For example:

```
rustc_binary("some_label") {
   ...
}
```

### Generating .cargo/config files {#cargo-config-gen}

Some plugins require a `.cargo/config` file to allow cargo to operate correctly for Fuchsia
(e.g. to run `cargo check`). To easily generate this file, use the [`fargo`][fargo] tool.

1. [Install rustup](https://rustup.rs/)
2. Configure `rustup` to use the Fuchsia Rust toolchain by running:

    ```sh
    rustup toolchain link fuchsia $($FUCHSIA_DIR/scripts/youcompleteme/paths.py VSCODE_RUST_TOOLCHAIN)
    rustup default fuchsia
    ```

3. Clone and install the `fargo` tool within your `$FUCHSIA_DIR` by following the
[getting started instructions][fargo] for fargo.
4. Create your config:

    ```sh
    cd $FUCHSIA_DIR && fargo write-config
    # Note the caveats about changing architecture in the fargo readme
    # https://fuchsia.googlesource.com/fargo/#creating-a-cargo_config
    ```

[fargo]: https://fuchsia.googlesource.com/fargo/
