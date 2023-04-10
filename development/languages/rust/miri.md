# Running tests in miri

Instructions for running Fuchsia host-side tests in [miri]. The examples use the
`//src/sys/lib/cm_rust:cm_rust_test($host_toolchain)` target.

miri is an interpreter for Rust code which you can use as a powerful tool to
find undefined behavior (UB) in Rust code, analogous to sanitizers for C and
C++ like UBSan.

Note: it is [not currently possible][miri-monorail] to run tests under miri
using Fuchsia's build system or infrastructure. This document describes a manual
workflow which only supports host-side (Linux/MacOS) targets and is not
continuously tested. Fixes and updates are welcome!

## Prerequisites

Install [rustup] and run `rustup update`.

Add a nightly toolchain if you don't already have one:

```sh
$ rustup install nightly
```

Add the miri component:

```sh
$ rustup +nightly component add miri
```

## Generate `Cargo.toml`

Running `cargo miri` requires a `Cargo.toml` manifest for your crate.

Configure your build to include the test you want to run and to generate Cargo.toml's:

```sh
$ fx set PRODUCT.BOARD --with //src/sys/lib/cm_rust:tests --cargo-toml-gen
```

Run a build:

```sh
$ fx build
```

Link the generated manifest into your source repository:

```sh
$ fx gen-cargo '//src/sys/lib/cm_rust:cm_rust_test_executable(//build/toolchain:host_x64)'
```

Note: The target given to `fx gen-cargo` must specify the actual executable
target of the test (`cm_rust_test_executable`) and the toolchain of the library
without using GN variables (`//build/toolchain:host_x64`).

See the [cargo on Fuchsia][cargo-toml-gen] docs for more information.

## Setup miri

`cargo miri` must compile a fresh Rust sysroot, which requires access to
external crates which are not vendored in the Fuchsia tree.

Open `src/.cargo/config` in your editor and comment out the configuration for
vendoring crates:

```diff
--- a/src/.cargo/config
+++ b/src/.cargo/config
@@ -1,8 +1,8 @@
-[source.crates-io]
-replace-with = "vendored-sources"
+#[source.crates-io]
+#replace-with = "vendored-sources"

-[source.vendored-sources]
-directory = "../third_party/rust_crates/vendor"
+#[source.vendored-sources]
+#directory = "../third_party/rust_crates/vendor"
```

Run `cargo +nightly miri setup`.

## Run miri

Change to the same directory as the generated `Cargo.toml`:

```sh
$ cd src/sys/lib/cm_rust
```

Run miri:

```sh
$ cargo +nightly miri test
```

[miri]: https://github.com/rust-lang/miri
[miri-monorail]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=88691
[rustup]: https://rustup.rs/
[cargo-toml-gen]: /docs/development/languages/rust/cargo.md#cargo-toml-gen
