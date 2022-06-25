# Build Fuchsia with a custom Rust toolchain

This guide explains how to build Fuchsia with a custom Rust compiler. This is
useful if you need to build Fuchsia with a patched compiler, or a compiler built
with custom options.

### Use another version of Rust

If all you need to do is use a different version than the one currently being
used, most of this doc is not necessary. Fuchsia builders [build Rust] after
every change to Rust's main branch.

1. Find the commit hash you want to use.
2. Run the following commands from your Fuchsia directory:

   ```posix-terminal
   # Replace COMMIT with the full Rust commit hash.
   # This command updates the manifests in the integration repo, which you can
   # then commit or revert as necessary.
   fx roll-compiler --package rust git_revision:{{ '<var>' }}COMMIT{{ '</var>' }}

   # Fetch the package versions you specified and install them in `prebuilt/`.
   jiri fetch-packages -local-manifest
   ```

3. Run the following command to build Fuchsia:

   ```posix-terminal
   fx build
   ```

   The Fuchsia build now uses the updated compiler.

   Note: A clean build is not necessary; the build automatically detects
   the new compiler version.

[build Rust]: https://ci.chromium.org/p/fuchsia/g/rust/console

## Prerequisites

Prior to building Fuchsia with a custom Rust toolchain, you need to do the following:

Note: These instructions are for Debian-based systems, but you should use the
correct package manager for your machine.

1. [Build a custom Rust toolchain](/docs/development/build/rust_toolchain.md)
   for Fuchsia.

1. Complete the following guide to download the Fuchsia source:
   [Get Fuchsia source code](/docs/get-started/get_fuchsia_source.md).
   To confirm that jiri is in your PATH run <code>jiri -help</code>.

   Note: The below commands assume `DEV_ROOT` is set to the parent directory of
   your Fuchsia checkout.

## Build Fuchsia with your custom Rust toolchain

1. Change into your Fuchsia directory.

1. Run the following command to use the newly built toolchain:

   ```posix-terminal
   DEV_ROOT={{ '<var>' }}DEV_ROOT{{ '</var>' }}

   fx set core.x64 \
     --args=rustc_prefix="\"$DEV_ROOT/rust/install/fuchsia-rust\"" \
     --args=rustc_version_string='"1"'
   # plus other settings such as:
   #   --with //bundles:kitchen_sink
   #   --variant=coverage-rust  # to enable coverage profiling of fuchsia binaries
   #   --variant=host_coverage-rust  # to enable coverage profiling of host binaries
   ```

   Note: `rustc_version_string` can be any string, and it’s used to force a
   recompile after a custom toolchain changes. If you rebuild the toolchain,
   change the value so Rust targets get rebuilt.

1. Run the following command to rebuild Fuchsia:

   ```posix-terminal
   fx build
   ```
