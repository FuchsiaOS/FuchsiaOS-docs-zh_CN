# GN toolchains and the Fuchsia Build

## Overview of GN toolchains

The GN build tool allows one build to compile the same target in different
ways, using multiple [toolchains][gn-toolchain]{:.external}.

Each `toolchain()` instance corresponds to:

- **A unique name, expressed as a [GN label][gn-label]{:.external}**.

  For example '//build/toolchain/fuchsia:x64' names the toolchain instance
  defined in the `//build/toolchain/fuchsia/BUILD.gn` file, with a
  `toolchain("x64")` definition.

- **A set of commands and build flags used to compile the source code and link
  binaries**.

  For example, using one toolchain to invoke Clang, and another one to invoke
  Microsoft Visual C++, allows a single build to generate binaries using both
  compiler suites.

- **A build graph node namespace**.

  Separating targets with the same GN path, but compiled with different
  toolchain instances. This is reflected in the format of
   _fully-qualified GN labels_, that look like
  `"//<dir>:<target>(<toolchain_dir>:<toolchain_target>)"`.

  For example `//src/foo:bar(//toolchain:debug)` corresponds to the `bar`
  target defined in `//src/foo/BUILD.gn`, when it is compiled with the
  commands of the `//toolchain:debug`  toolchain.

- **A separate GN execution context**.

  Each toolchain instance executes its own parse of the GN
  [buildconfig file][gn-buildconfig]{:.external}, which sets up global
  variables and default values for all rules defining targets in
  that toolchain.

  In practice, if the same target is built with two different toolchains,
  the corresponding `BUILD.gn` file will be parsed twice, but each time
  with a different set of global variables, default configs and custom
  templates defined in `BUILDCONFIG.gn`.

- **A separate root directory for target outputs**.

  While the targets built in the default toolchain are placed under
  [`root_build_dir`][gn-root_build_dir]{:.external},  the ones that are built
  with a `//<toolchain_dir>:<toolchain_name>` instance are placed
  under `${root_build_dir}/<toolchain_name>` instead.

  This location is available at GN gen time through the
  [`root_out_dir`][gn-root_out_dir]{:.external} variable.

There is always at least one toolchain, called the _default toolchain_, which
is determined by calling
[`set_default_toolchain()`][gn-set_default_toolchain]{:.external}
from the [buildconfig file][gn-buildconfig]{:.external}.

For more information read the [`toolchain()`][gn-toolchain]{:.external}
reference documentation.

## How the Fuchsia build uses GN toolchains

The Fuchsia build uses GN toolchains in several ways:

- _To build host and device executables_.

  The build currently defines `//build/toolchain/fuchsia:x64` and
  `//build/toolchain/fuchsia:arm64` to build Fuchsia executable binaries
  for the 64-bit Intel and ARM architectures.

  It also defines `//build/toolchain:host_x64` to build code for the host machine
  (i.e. the one where the build happens).

  It also defines `//build/toolchain:linux_x64` and `//build/toolchain:linux_arm64`
  to generate Linux 64-bit code as well, even if the host is not running one of
  these architectures.

  There are also a number of specialized toolchains used to compile bootloaders
  and parts of the kernel, described later.

- _To build ELF shared libraries_.

  On Fuchsia, machine code that goes into shared objects (i.e. `shared_library()`
  and `loadable_module()` instances in GN speak) must be built with the `-fPIC`
  compiler and linker option.

  This is unlike executable code, that uses `-fPIE` instead.

  To deal with this, separate toolchain instances are defined to compile
  code for shared librarie.

  See [ELF Shared Library Redirection](elf_shared_library_redirection.md) for
  more details.

- _To build different variants (e.g. instrumented or optimized) of binaries_.

  The Fuchsia build supports a number of "build variants" which allow building
  machine code in a slightly different way, for example:

  - The `asan` and `ubsan` variants are used to build machine code with
    Clang's Address Sanitizer, and Undefined Behaviour Sanitizer, respectively.
    There is even an `asan-ubsan` variant that combines both.

  - The `coverage` variant is used to build machine code with Clang's
    instrumentation-based profiling enabled, to support code coverage
    collection.

  - The `profile` variant is used to build instrumented code as well,
    but to support profile-guided optimization.

  - The `thinlto` and `lto` variants are used to build binaries with
    link-time optimization enabled.

  - The `gcc` variant is used to build certain pieces of the Zircon
    kernel with the GCC compiler instead of Clang (which has been useful
    to weed out subtle machine code generation issues that can affect
    the kernel in very important ways).

  There are many other variants defined in the build's `BUILDCONFIG.gn` file
  as well.

- _To generate (or process) source files_.

  The build requires generating source files to be used in other
  targets in many places. For example, FIDL protocol definition
  files are processed to generate bindings for various languages
  (C++, Rust, Go and Dart), which are later used by other
  `source_set()`, or similar, targets.

  Because the targets that use these sources can be defined in
  different toolchain instances, it is useful to ensure that this
  generation is only performed once, instead of once per toolchain
  instance, since the output will be exactly the same in all cases.

  The Fuchsia build thus defines the `fidling` toolchain to perform
  FIDL bindings generation. Note that this toolchain is only used to
  run a few scripts using `action()` targets, never to actually compile
  them.

  Similarly, a number of other "basic" toolchains are defined in the
  build to perform processing tasks that should not be repeated
  needlessly.

## How the Fuchsia build defines `toolchain()` instances.

The Fuchsia build provides these templates that define `toolchain()` instances
with various features:

- `basic_toolchain()` defines a "basic" toolchain, i.e.
  one that does only `copy()` or `action()` target, and never needs to
  use GN's builtin support for C++ and Rust compilation.

  These are used to generate outputs that are used by other targets in
  several other toolchains (e.g. language bindings) to avoid duplicate work.

  Note that one basic toolchain is used to build Go binaries, and another
  one for Dart ones, since GN doesn't support these languages at all.

- `clang_toolchain()` defines a toolchain instances that invokes
  the Clang compiler. It provides support for building C++ and Rust
  sources using GN's builtin rules.

  Supported target platforms are Fuchsia, Linux, MacOS, Win32 PE/COFF
  (as required by the UEFI bootloader) and even WebAssembly!

- `clang_toolchain_suite()` defines one more toolchain instances
  based on the current build variant configuration. It is preferred over
  calling `clang_toolchain()` directly because this is what allows
  build variants to work.

- `clang_host_toolchain_suite()` is used for toolchains that generate
  host machine code.

- `zircon_toolchain()` defines a toolchain instances that can be used to
  build part of the Zircon kernel, bootloaders or even the C library.
  These binaries typically require non-standard compile and linker commands
  (e.g. a different ABI, or lack of standard link environment).

  One notable feature of this template is that it also supports
  building binaries using the GCC compiler, instead of Clang. This has
  proven useful to find low-level code generation issues that the
  kernel is very sensitive about, by its nature.

  NOTE: There is no plan to support building the rest of the platform with GCC.

- `zircon_toolchain_suite()` is used to define one or more toolchain
  instances based on the current build variant configuration. It is
  preferred over calling `zircon_toolchain()` directly.

Note that the distinction between `zircon_toolchain()` and
`clang_toolchain()` is mostly historical, they might be merged into
a common template in the future.

[gn-label]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#labels
[gn-toolchain]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#func_toolchain
[gn-buildconfig]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#other-help-topics-gn-file-variables
[gn-set_default_toolchain]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#func_set_default_toolchain
[gn-root_build_dir]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#var_root_build_dir
[gn-root_out_dir]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#var_root_out_dir
