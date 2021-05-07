# Publish a CIPD symbols package for ELF binaries {#publish-a-symbols-cipd-package}
A symbols package is used for symbolization in logs and
debugging crashes. A CIPD prebuilt package that contains Fuchsia ELF binaries
needs to have a companion symbols package that contains debug information for
the binaries.

## Requirements for a symbols package

*   Each ELF binary in the original CIPD package must include
    an ELF `NT_GNU_BUILD_ID` note section, which includes a unique `build-id`
    hash value that uniquely identifies its non-debug-related content.

    This note is created at link time when producing the ELF binary. Recent
    versions of GCC, or the prebuilt Fuchsia Clang toolchain, produce it
    automatically. However, regular Clang requires passing a special linker
    flag (that is, `-Wl,--build-id`).

    Note: To print the `build-id` of an existing library, use either one of
    `file <LIBRARY>` or
    `readelf -n <LIBRARY> | grep "Build ID"`

*   The symbols package must use the directory layout typical of `.build-id`
    directories used to store
    [debug information in separate files](https://sourceware.org/gdb/current/onlinedocs/gdb/Separate-Debug-Files.html){: .external}.

    This means that each file must be stored as `<xx>/<xxxxxxxxxx>.debug`,
    where `<xx>` and `<xxxxxxxxx>` are hex strings derived from the `build-id`
    hash value, of the unstripped binary. Each such file should match one
    stripped ELF binary with the same `build-id` from the original package.

    Note: The symbols package needs to not include the top-level `.build-id`
    directory.

    This example shows the directory structure of a CIPD symbols package with
    unstripped ELF binaries:

    ```none
    1d/
      bca0bd1be33e19.debug
    1f/
      512abdcbe453ee.debug
      90dd45623deab1.debug
    2b/
      0e519bcf3942dd.debug
    3d/
      aca0b11beff127.debug
    5b/
      66bc85af2da641697328996cbc04d62b84fc58.debug
    ```

*   The symbols package must use the same
    [version identifiers](/docs/development/prebuilt_packages/publish_prebuilt_packages_to_cipd.md#set-cipd-package-versioning)
    (tags) as the original CIPD package they refer to. This allows them to
    be rolled together.

*   If several CIPD packages that contain stripped ELF binaries are rolled
    together (using the same version identifiers), then grouping the debug
    symbols for all of them in a single CIPD symbols package is acceptable,
    but not required.

*   The CIPD path for the symbols package needs to use the
    the following suffix:

    ```none
    -debug-symbols-<ARCH>
    ```

    For example,
    `myproject/fuchsia/mypackage-debug-symbols-amd64` contains the symbols
    for the `myproject/fuchsia/mypackage-amd64` prebuilt package.

*   The Jiri checkout path for all symbols package must be
    `${FUCHSIA_DIR}/prebuilt/.build-id`.

## Generate a symbols package

To generate a symbols package, you need to:

*   Compile all your ELF binaries with DWARF debug information (for example,
    passing the `-g` flag to the compiler, even in release mode).

*   Produce an `NT_GNU_BUILD_ID` note for the ELF binaries.

    This is the default on recent GCC versions and the Fuchsia prebuilt
    Clang toolchain, but regular Clang requires passing a special flag
    (`-Wl,--build-id`) to the linker.

You can generate the stripped binary and the corresponding
`build-id` directories with a single call to `llvm-objcopy`, as in the
following example:

```none
# Copy out/libfoo.so to symbols/<xx>/<xxxxxxx>.debug according to its
# `build-id` value (requires the library to be linked with -Wl,--build-id when
# using Clang), and also copy the stripped version of the library to
# stripped/libfoo.so
#
# NOTE: To strip executables, instead of libraries, replace --strip-all below
#       with --strip-sections
#
UNSTRIPPED_LIB=out/libfoo.so
STRIPPED_LIB=stripped/libfoo.so
SYMBOLS_DIR=./symbols

llvm-objcopy --strip-all \
    --build-id-link-dir="${SYMBOLS_DIR}" \
    --build-id-link-input=.debug \
    "${UNSTRIPPED_LIB}" "${STRIPPED_LIB}"
```

Repeat as many times as necessary to populate the `symbols/` directory,
then upload its content (the files under `symbols/`) as your symbols package.

Don't forget to copy the content of the `stripped/` directory to your
prebuilt CIPD package as well.

