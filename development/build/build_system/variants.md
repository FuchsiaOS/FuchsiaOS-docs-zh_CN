# Fuchsia Build System: Variants

The Fuchsia GN build machinery allows for separate components to be built
in different "variants".  A variant usually just means using extra compiler
options, but they can do more than that if you write some more GN code.
The variants defined so far enable things like
[sanitizers](https://github.com/google/sanitizers/wiki) and
[LTO](https://llvm.org/docs/LinkTimeOptimization.html).

### Specifying variants

Variant specifications are passed in to the build using the GN build argument
[`select_variant`](/docs/gen/build_arguments.md#select_variant). Note that the order of these
variant selectors matters as explained in the syntax section below.

Using `fx set`, pass string variant selectors with the `--variant=` flag:

```sh
fx set core.x64 --variant=asan/cat --variant=asan/ledger --variant=host_asan
```

This example tells the build to compile the "cat" and "ledger" Fuchsia binaries plus all host tools
using the address sanitizer (see below for the exact syntax of these strings).

If you have an existing build directory, you can add or modify the variants by editing the GN args
directly (substituting your build's GN output directory for `out/default` as necessary):

```sh
fx gn args out/default
```

That command will bring up an editor. Append to that file a line that assigns your variant selectors
as a list of strings to the `select_variant` build argument:

```
select_variant = [ "asan/cat", "asan/ledger", "host_asan" ]
```

### Selector syntax

Normally you will use a set of strings for the variant selectors. Each one defines a variant name
and optionally what it applies to. Selectors are tested in the order you specify them and the first
matching one applies.

  * Apply a named variant globally by using the variant name by itself like `asan` or
   `host_ubsan`. These global selectors should be listed last.

  * Apply a named variant to a specific target with the form `variant_name/target_output_name`
    like `asan-ubsan/ledger` or `host_asan/zxdb_tests`. These should be listed before global
    selectors to override the more general rule.

Variants are matched against binaries (like `executable`, `loadable_module`, `test`,
or`fuchsia_driver`), not Fuchsia packages, Fuchsia components, shared libraries, static libraries, or
source sets. Once the variant matches a target, all libraries it depends on will be compiled with
that variant. Since the Fuchsia packages and components are unrelated to variant selection,
specifying package or component names in variant specifications will have no effect. Each executable
or module inside a package can have its own variant specified.

By default, the target output name is the name in quotes you supply to the GN target definition.
This target defines the `my_program` target and you would apply asan to it with the selector
`asan/my_program`:

```
executable("my_program") { ... }
```

Some targets override the output name using the GN `output_name` variable (this is normally to
provide a globally unique binary name to avoid collisions). In this case, the variant selector
matches the overridden output name so you would still use `asan/my_program` to apply asan to it:

```
executable("bin") {
  output_name = "my_program"
}
```

In some cases, templates might override the output name in a non-obvious way. If you find the
variant is not matching, one easy approach is to just look in the build directory to find the
binary and use that name.

### Advanced selectors

You can also supply a GN "scope" in curly brackets as a variant selector which can give full
control over exactly how matching targets are built. These must be set in the "gn args" rather
than on the "fx set" command line. See the
[`select_variant`](/docs/gen/build_arguments.md#select_variant) build argument documentation for
more details.

To see the list of variants available and learn more about how to define
new ones, see the
[`known_variants`](/docs/gen/build_arguments.md#known_variants)
build argument.

### Common variant names

  * `debug`: Unoptimized compilation.
  * `release`: Optimized compilation.
  * `asan`: [Address sanitizer](https://clang.llvm.org/docs/AddressSanitizer.html) for compile-time
    checks of memory misuse like use-after-free and out-of-bounds array accesses.
  * `ubsan`: [Undefined behavior sanitizer](https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html) for compile-time checks
    of undefined behavior like integer overflows and misaligned pointers.
  * `asan-ubsan`: Combination of asan + ubsan.
  * `lto`: [Link-time optimization](https://llvm.org/docs/LinkTimeOptimization.html) for
    whole-program optimization.
  * `thinlto`: [Thin link-time optimization](https://clang.llvm.org/docs/ThinLTO.html) is
    lighter-weight whole program optimization for faster compiles.
  * `coverage`: Instrumented compilation for generating
    [code coverage](https://clang.llvm.org/docs/SourceBasedCodeCoverage.html) information for C++.
  * `coverage-rust`: Applies coverage to Rust. Can not be used at the same time as `coverage` due
    to LLVM library version skew between the Rust and C++ compilers.
  * `kasan`: Applies asan only to the kernel.
  * `gcc`: Compiles using GCC instead of Clang. This is supported for the bringup configuration
    only and only affects certain targets including the kernel.

Fuzzer variants like `asan-fuzzer` are used when running tests under the
[fuzzer](/docs/development/testing/fuzzing/overview.md) with a sanitizer. These variants aren't
meant for manual selection, instead follow the fuzzing instructions to set up the build.

There are additionally some shorthand selectors that apply variants to host binaries (the tools
that run on the Linux or Mac host computer):
  * `host_asan`
  * `host_asan-ubsan`
  * `host_coverage`
  * `host_coverage-rust`
  * `host_profile`

Some prebuilts might not be available for all variants. For ffmpeg in particular, see
[//src/media/lib/ffmpeg/BUILD.gn](/src/media/lib/ffmpeg/BUILD.gn).

## Troubleshooting notes

### Checking whether a variant was applied to a binary

Each variant has a unique output directory and toolchain name, named as
`<architecture>-<variant name>`. These binaries are then copied to the root build directory as part of
the build. For example, an asan-ubsan variant targeting an x64 device would be compiled with the
`//build/toolchain/fuchsia:x64-asan-ubsan` toolchain and will put binaries in
`out/default/x64-asan-ubsan` (substituting "default" for your build directory).

After running GN (normally the first step of the build) there will be a file `binaries.json`
which contains information for each binary. You can tell by the `dist` file name and `label` (the
toolchain name is in parentheses) which variant was used to compile the binary. If your binary
might be compiled on both target and host, also note the `os` field in the record. This is an
example of a Fuchsia binary compiled for x64 using the "asan-ubsan" variant:

```json
  {
    "cpu": "x64",
    "debug": "x64-asan-ubsan/exe.unstripped/blobfs",
    "dist": "x64-asan-ubsan/blobfs",
    "elf_build_id": "x64-asan-ubsan/blobfs.build-id.stamp",
    "label": "//src/storage/bin/blobfs:blobfs(//build/toolchain/fuchsia:x64-asan-ubsan)",
    "os": "fuchsia",
    "type": "executable"
  },
```

### Replicating ASan failures

Our infrastructure runs tests in an ASan-enabled configuration. To replicate an
ASan-enabled infrastructure build, use `fx repro <build_id>` and run the
commands it emits.

Note that this will build all of the tests that are run by the infrastructure
and install them in the system image. This may be undesirable for two reasons:

 * Building all of the tests is typically slow and unnecessary. Developers may
   find it more effective to limit the package labels to the tests they need.
 * Installing all of the tests in the system image ahead of time means that the
   software deployment workflow does not get exercised.

### Launching executables from within ASan-enabled binaries

If you are trying to use the ASan variant, you may encounter an error that looks
like this:

```sh
launcher: error: Launch: elf_load: handle_interp failed
dlsvc: could not open 'asan/ld.so.1'
```

Fuchsia is structured around packages and components. Each component contains
all of the shared libraries it needs to run. This helps Fuchsia avoid library
versioning issues that plague other operating systems. It also means that, if
you want to run a binary from within a component, you must provide the
appropriate shared library loader for that binary.

There are a set of command line programs located in the `/boot/` directory of
Fuchsia installs that are not contained in packages, but in the boot filesystem.
These programs do not have their own shared library loader, and will use
whatever shared libraries the component executing them provides. This normally
works, as programs like `sh` and `ls` have very minimal, very common
dependencies. However, there's no guarantee that the component's package will
have sufficient or compatible shared libraries for the command line program's
needs. ASan-enabled packages usually do not contain the right launcher for these
programs, so most ASan-enabled components cannot run executables out of
`/boot`. If an ASan-enabled component tries to do so, it gets the error above.

Fortunately, it turns out that the fix involves doing what all packages should
do anyway, which is to declare their dependencies explicitly. If your package
depends on a binary, it should declare it as a dependency, and then use that
declared dependency instead of the one in the `/boot` directory. In the case of
our build system, the `zircon_extras_manifest` rule defined in
`//build/config/fuchsia/zircon_images.gni` will allow you to depend on any of
the binaries found in the `/boot` directory. They will be installed in
`/pkg/bin/`, and you should execute them from there.
