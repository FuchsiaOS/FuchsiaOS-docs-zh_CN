# Integrating C/C++ using `bindgen`

If you need to call some C or C++ APIs from Rust, you can use [`bindgen`] which generates Rust code from C & C++ headers. For more documentation, see [the `bindgen` User Guide](https://rust-lang.github.io/rust-bindgen/).

## Requirements

> Note: This requirement will be lifted when [fxb/78852][static-link-bug] is resolved.

Our `bindgen` prebuilt currently links dynamically to clang, which means you need `libclang.so` available in your library search path.

On Debian-based systems this can usually be achieved with `sudo apt install llvm-dev libclang-dev clang`. On macOS systems with homebrew this can usually be achieved with `brew install llvm`.

## Generating Rust code

> Note: This section and the next will be simplified when [fxb/73858][gn-template-bug] is resolved.

While the generated code will be checked in to git, it is important that it is easy for any contributor to update the generated code. The first step here is to make an executable file in your target's directory called `bindgen.sh`. See [`//src/lib/usb_bulk/bindgen.sh`](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/lib/usb_bulk/bindgen.sh) for an example which uses our prebuilt `bindgen` binary and further customizes the output.

Run the script on your development machine to generate a Rust file that will be committed with your build target.

## Building generated code

Once you have a script that can reliably generate a Rust file from your C++ headers, it needs to be added to the build. The generated file can be its own `rust_library` target or it can be included as a submodule of another Rust target, as it is in [the `//src/lib/usb_bulk` example](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/lib/usb_bulk/rust/BUILD.gn). Make sure that the library target which includes the file from `bindgen` also includes the appropriate external deps in `non_rust_deps`.

[`bindgen`]: https://github.com/rust-lang/rust-bindgen
[static-link-bug]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=78852
[gn-template-bug]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=73858
