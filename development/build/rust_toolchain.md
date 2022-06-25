# Build a custom Rust toolchain for Fuchsia

This guide explains how to build a Rust compiler for use with the Fuchsia. This
is useful if you need to build Fuchsia with a patched compiler, or a compiler
built with custom options. Building a custom Rust toolchain is not always
necessary for building Fuchsia with a different version of Rust; see
[Build Fuchsia with a custom Rust toolchain](/docs/development/build/fuchsia_custom_rust.md)
for details.

## Prerequisites

Prior to building a custom Rust toolchain for Fuchsia, you need to do the following:

1. Run the following command to install cmake:

   ```posix-terminal
   sudo apt-get install cmake ninja-build
   ```

1. Run the following command to obtain the infra sources:

   ```posix-terminal
   DEV_ROOT={{ '<var>' }}DEV_ROOT{{ '</var> '}} # parent of your Rust directory
   mkdir -p $DEV_ROOT/infra && \
   ( \
     builtin cd $DEV_ROOT/infra && \
     jiri init && \
     jiri import -overwrite -name=fuchsia/manifest infra \
         https://fuchsia.googlesource.com/manifest && \
     jiri update \
   )
   ```

   Note: Running `jiri update` from the `infra` directory ensures that you
   have the most recent configurations and tools.

1. Download and extract the Fuchsia core IDK to `$DEV_ROOT/sdk`. For more
   information, see [Download the Fuchsia IDK](/docs/development/idk/download.md).

1. Run the following command to use `cipd` to get the linux `sysroot` package
   for your host platform:

   ```posix-terminal
   # You may want to: rm -rf $DEV_ROOT/sysroot
   mkdir -p $DEV_ROOT/sysroot
   cipd install fuchsia/third_party/sysroot/linux latest -root $DEV_ROOT/sysroot/linux
   ```

1. If you haven't already, clone the Rust source. The
   [Guide to Rustc Development] is a good resource to reference whenever you're
   working on the compiler.

   ```posix-terminal
   git clone https://github.com/rust-lang/rust.git $DEV_ROOT/rust
   ```

[Guide to Rustc Development]: https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html

## Configure Rust for Fuchsia

1. Change into your Rust directory.
1. Run the following command to generate a configuration for the Rust toolchain:

   ```posix-terminal
   DEV_ROOT={{ '<var>' }}DEV_ROOT{{ '</var>' }}

   $DEV_ROOT/infra/fuchsia/prebuilt/tools/vpython \
     $DEV_ROOT/infra/fuchsia/recipes/recipes/contrib/rust_toolchain.resources/generate_config.py \
       config_toml \
       --clang-prefix=$DEV_ROOT/fuchsia/prebuilt/third_party/clang/linux-x64 \
       --host-sysroot=$DEV_ROOT/sysroot/linux \
       --prefix=$(pwd)/install/fuchsia-rust \
      | tee fuchsia-config.toml

   $DEV_ROOT/infra/fuchsia/prebuilt/tools/vpython \
       $DEV_ROOT/infra/fuchsia/recipes/recipes/contrib/rust_toolchain.resources/generate_config.py \
         environment \
         --eval \
         --clang-prefix=$DEV_ROOT/fuchsia/prebuilt/third_party/clang/linux-x64 \
         --sdk-dir=$DEV_ROOT/sdk \
         --linux-amd64-sysroot=$DEV_ROOT/sysroot/linux \
         --linux-arm64-sysroot=$DEV_ROOT/sysroot/linux \
      | tee fuchsia-env.sh
   ```

1. (Optional) Run the following command to tell git to ignore the generated files:

   ```posix-terminal
   echo fuchsia-config.toml >> .git/info/exclude
   echo fuchsia-env.sh >> .git/info/exclude
   ```

1. (Optional) Customize `fuchsia-config.toml`.

## Build and install Rust

1. Change into your Rust source directory.
1. Run the following command to build and install Rust plus the Fuchsia runtimes spec:

   ```posix-terminal
   DEV_ROOT={{ '<var>' }}DEV_ROOT{{ '</var>' }}

   rm -rf install/fuchsia-rust
   mkdir -p install/fuchsia-rust

   # Copy and paste the following subshell to build and install Rust, as needed.
   # The subshell avoids polluting your environment with fuchsia-specific rust settings.
   ( source fuchsia-env.sh && ./x.py install --config fuchsia-config.toml ) && \
   rm -rf install/fuchsia-rust/lib/.build-id && \
   $DEV_ROOT/infra/fuchsia/prebuilt/tools/vpython \
     $DEV_ROOT/infra/fuchsia/recipes/recipes/contrib/rust_toolchain.resources/generate_config.py \
       runtime \
     | $DEV_ROOT/infra/fuchsia/prebuilt/tools/vpython \
         $DEV_ROOT/infra/fuchsia/recipes/recipe_modules/toolchain/resources/runtimes.py \
           --dir install/fuchsia-rust/lib \
           --readelf fuchsia-build/*/llvm/bin/llvm-readelf \
           --objcopy fuchsia-build/*/llvm/bin/llvm-objcopy \
     > install/fuchsia-rust/lib/runtime.json
   ```

### Build only (optional)

If you want to skip the install step, for instance during development of Rust
itself, you can do so with the following command.

```posix-terminal
( source fuchsia-env.sh && ./x.py build --config fuchsia-config.toml )
```

### Troubleshooting

If you are getting build errors, try deleting the Rust build directory:

```posix-terminal
rm -rf fuchsia-build
```

Then re-run the command to build Rust.

## Building Fuchsia with a custom Rust toolchain

With a newly compiled custom Rust toolchain, you're ready to use it to build
Fuchsia. Directions on how to do so are available in a [dedicated guide].

[dedicated guide]: /docs/development/build/fuchsia_custom_rust.md
