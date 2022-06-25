# Contributing to FIDL

[TOC]

## Overview

The [FIDL][fidl-readme] toolchain is composed of roughly three parts:

1. Front-end, a.k.a. `fidlc`
    *   Parses and validates `.fidl` files
    *   Calculates size, alignment, and offset of various structures
    *   Produces a [JSON IR][jsonir] (Intermediate Representation)
2. Back-end
    *   Works off the IR (except the C back-end)
    *   Produces target language specific code, which ties into the libraries for that language
3. Runtime Libraries
    *   Implement encoding/decoding/validation of messages
    *   Method dispatching mechanics

### Code location

#### Compiler front-end

The front-end lives at [//tools/fidl/fidlc/][fidlc-source],
with tests in [//tools/fidl/fidlc/tests][fidlc-compiler-tests].

#### Compiler back-ends {#compiler-backends}

Target | Codegen | Runtime Libraries | Tests
-------|---------|-------------------|-------
C | [/tools/fidl/fidlc/lib/c_generator.cc] | [/zircon/system/ulib/fidl] | [/src/lib/fidl/c]
Coding Tables | [/tools/fidl/fidlc/lib/tables_generator.cc] | - | [/src/lib/fidl/c]
HLCPP | [/tools/fidl/fidlgen_hlcpp] | [/sdk/lib/fidl/cpp] | *(located alongside runtime libraries)*
LLCPP | [/tools/fidl/fidlgen_cpp] | [/sdk/lib/fidl/llcpp] | [/src/lib/fidl/llcpp]
Unified C++ | [/tools/fidl/fidlgen_cpp] | [/src/lib/fidl/cpp] | *(located alongside runtime libraries)*
Go | [/tools/fidl/fidlgen_go] | [/third_party/go/src/syscall/zx/fidl](https://fuchsia.googlesource.com/third_party/go/+/main/src/syscall/zx/fidl) | *(located alongside runtime libraries)*
Rust | [/tools/fidl/fidlgen_rust] | [/src/lib/fidl/rust] | *(located alongside runtime libraries)*
Dart | [/tools/fidl/fidlgen_dart] | [/sdk/dart/fidl] | [/src/tests/fidl/dart_bindings_test]

Note: The tests column refers to hand-written tests that exercise both the
generated code and the runtime libraries. There are also other tests, like unit
tests for the codegen itself and GIDL generated tests. Refer to the [tests
section](#all-tests) for details.

Supporting code for the target specific backends is located in
[/tools/fidl/lib/fidlgen].

#### Testing tools

##### GIDL

GIDL is a tool used to create general "write once, generate for every backend"
programs. Currently GIDL is used to generate encode or decode tests
("conformance tests") as well as benchmarks.

Path | Description
-----|------------
[/tools/fidl/gidl] | Source code and build templates for the GIDL tool itself.
[/src/tests/fidl/conformance_suite] | Test definitions (`.fidl` and `.gidl` files) for conformance tests.
[/sdk/cts/tests/pkg/fidl/cpp/test/{test,handle}_util.h](/sdk/cts/tests/pkg/fidl/cpp/test) | Runtime support for HLCPP conformance tests.
[/src/lib/fidl/llcpp/tests/conformance/conformance_utils.h] | Runtime support for LLCPP conformance tests.
[/src/lib/fidl/rust/gidl_util] | Runtime support for Rust conformance tests.
[/third_party/go/src/syscall/zx/fidl/fidl_test] | Runtime support for Go conformance tests.
[/src/lib/fidl/dart/gidl] | Runtime support for Dart conformance tests.
[/src/tests/benchmarks/fidl/benchmark_suite] | Benchmark definitions (`.fidl` and `.gidl` files).
[/src/tests/benchmarks/fidl] | Runtime support for benchmarks.

The actual test targets for the conformance tests in each backend are generally
defined alongside the [corresponding tests for that backend](#compiler-backends).
Refer to the [Bindings tests](#bindings-tests) section for details.

##### Source compatibility

Source compatibility tests are used to test FIDL's
[source compatibility guarantees][abi-api-compat]. They are found in
[/src/tests/fidl/source_compatibility].

##### Compatibility

Compatibility tests are integration tests that run FIDL clients
and servers from different bindings with each other in order to test that they
are compatible. Compatibility tests are found at
[/src/tests/fidl/compatibility/].

##### Dangerous identifiers

Dangerous identifier tests are found in
[/src/tests/fidl/dangerous_identifiers].

#### Other

Some other FIDL related areas are:

Path | Contents
-----|---------
[/tools/fidl/fidlgen_*](/tools/fidl/) | Various other compiler back-ends.
[/tools/fidl/fidlc/formatter] | FIDL formatter.
[/tools/fidl/fidlc/linter] | FIDL linter.
[/tools/fidl/fidldoc] | Generate documentation for FIDL.
[/tools/fidl/fidlmerge] | Tool for generating code from FIDL JSON.
[/tools/fidl/measure-tape] | Tool to [max out pagination][pagination].
[/tools/fidl/scripts] | Mostly one-off scripts for e.g. performing migrations that are kept for future reference.
[/src/lib/fostr] | `fidlmerge` based tool to generate formatting code in C++.
[/src/lib/fostr/build] | Build templates for the `fostr` formatting library.
[/src/lib/fidl_codec] | Library for encoding/decoding FIDL messages (used by `fidlcat`).

Note: The FIDL team does not necessarily
own all of these areas, but they may need to be updated when making changes to
the FIDL API surface, such as when changing the FIDL JSON IR.
Refer to the respective READMEs and OWNERS files for details.

### Other FIDL tools

A number of FIDL tools are located in the [`fidl-misc` repo][fidl-misc]. To
clone this repo, run

```sh
git clone https://fuchsia.googlesource.com/fidl-misc
```

It is then recommended to export the path to this directory, to make setting
aliases easier:

```sh
export FIDLMISC_DIR=...
```

### Common development tools

This is a crowdsourced section from the FIDL team on useful tools that they
use for working on the FIDL codebase.

#### IDEs

Most of the FIDL team uses VS Code for development. Some useful plugins and workflows:

* The [remote ssh](https://code.visualstudio.com/docs/remote/ssh) feature works
really well for doing remote work from your laptop.
  * Setting up tmux or screen is also helpful for remote work, to preserve
  history and manage multiple sessions in the shell.
* The Fuchsia documentation has instructions for setting up language servers:
  * [clangd](development/languages/c-cpp/editors.md) for c++
  * [rust-analyzer](development/languages/rust/editors.md) for rust
* The [rewrap extension](https://marketplace.visualstudio.com/items?itemName=stkb.rewrap) is useful
  for automatically reflowing lines to a certain length (e.g. when editing markdown files).
* To get automatic syntax highlighting for the bindings golden files, update the
  `file.associations` setting:

  ```json
  "files.associations": {
        "*.json.golden": "json",
        "*.rs.golden": "rust",
        "*.cc.golden": "cpp",
        "*.h.golden": "cpp",
        "*.go.golden": "go",
        "*.dart.golden": "dart",
  },
  ```

### Commit message style guide

When writing a change message, follow the [Commit message style
guide][commit-message].

### C++ style guide

We follow the [Fuchsia C++ Style Guide][cpp-style], with additional rules to
further remove ambiguity around the application or interpretation of guidelines.

#### Comments

Comments must respect 80 columns line size limit, unlike code, which can extend
to 100 lines size limit.

#### Lambda captures

* If a lambda escapes the current scope, capture all variables explicitly.
* If the lambda is local (does not escape the current scope), prefer using a default capture by
  reference ("`[&]`").

Seeing `[&]` is a strong signal that the lambda exists within the current scope only, and can be
used to distinguish local from non-local lambdas.

```cpp
// Correct.
std::set<const flat::Library*, LibraryComparator> dependencies;
auto add_dependency = [&](const flat::Library* dep_library) {
  if (!dep_library->HasAttribute("Internal")) {
    dependencies.insert(dep_library);
  }
};
```

## General setup

### Fuchsia setup

Read the [Fuchsia Getting Started][getting_started] guide first.

### `fx set`

If you are working on the FIDL toolchain, use:

```sh
fx set core.x64 --with //bundles/fidl:tests --with-base //src/dart:dart_jit_runner
```

The `--with-base` flag is necessary to run Dart tests and benchmarks.

If you are working on an LSC:

```sh
fx set terminal.x64 --with //bundles:kitchen_sink \
                    --with //sdk:modular_testing
```

### symbolizer

To symbolize backtraces, you'll need a symbolizer in scope:

```sh
export ASAN_SYMBOLIZER_PATH="$(find `pwd` -name llvm-symbolizer | grep clang | head -1)"
```

## Compiling and running tests

We provide mostly one-liners to run tests for the various parts.
When in doubt, refer to the "`Test:`" comment in the git commit message;
we do our best to describe the commands used to validate our work there.

Tests are run using the [fidldev][fidldev] tool. Examples assume that the
`fidldev` script is somewhere on your PATH, e.g. by adding an alias:

```sh
alias fidldev=$FIDLMISC_DIR/fidldev/fidldev.py
```

### `fidlc`

```sh
# optional; builds fidlc for the host with ASan <https://github.com/google/sanitizers/wiki/AddressSanitizer>
fx set core.x64 --variant=host_asan

fx build host_x64/fidlc
```

If you're doing extensive edit-compile-test cycles on `fidlc`, building with
fewer optimizations can make a significant difference in the build speed. To do
this, change the `optimization` setting in `zircon/public/gn/config/levels.gni`
from `default` to `debug` or `none`.

Warning: The kernel is not regularly tested with `debug`, and only supports
`none` for building. Running with `none` can cause kernel panics from stack
overflows in the kernel.

To avoid accidentally committing this change, run:

```
git update-index --skip-worktree zircon/public/gn/config/levels.gni
```

If you want to allow the changes to be committed again, run:

```
git update-index --no-skip-worktree zircon/public/gn/config/levels.gni
```

#### `fidlc` tests

`fidlc` tests are at:

* [//tools/fidl/fidlc/{tests,goldens,testdata}][fidlc-compiler].

To build and run `fidlc` tests:

```sh
fx test //tools/fidl/fidlc
```

If you prefer to use `ninja` directly:

```sh
fx_build_dir=$(cat .fx-build-dir) \
    fidlc_tests_target=$(fx ninja -C $fx_build_dir -t targets all | grep -e 'unstripped.*fidlc-test:' | awk -F : '{ print $1; }') \
    fx ninja -C $fx_build_dir $fidlc_tests_target && ./$fx_build_dir/$fidlc_tests_target
```

To run a specific suite of tests, use the `--gtest_filter` with an appropriate
pattern. For instance:

```sh
fx_build_dir=$(cat .fx-build-dir) \
    fidlc_tests_target=$(fx ninja -C $fx_build_dir -t targets all | grep -e 'unstripped.*fidlc-test:' | awk -F : '{ print $1; }') \
    fx ninja -C $fx_build_dir $fidlc_tests_target && ./$fx_build_dir/$fidlc_tests_target --gtest_filter 'EnumsTests.*'
```

#### `fidlc` debugging

To easily run tests in a debug build, set your environment slightly differently:

```
fx set core.x64 --variant=host_asan --with //bundles/fidl:tests
export ASAN_SYMBOLIZER_PATH="$(find `pwd` -name llvm-symbolizer | grep clang | head -1)"
```

Once properly set up, you can run tests using the commands listed previously,
with or without filtering.

To step through a test, you can use [GDB](#gdb):

```sh
fx_build_dir=$(cat .fx-build-dir) \
    fidlc_tests_target=$(fx ninja -C $fx_build_dir -t targets all | grep -e 'unstripped.*fidlc-test:' | awk -F : '{ print $1; }') \
    fx ninja -C $fx_build_dir $fidlc_tests_target && fx gdb --args ./$fx_build_dir/$fidlc_tests_target --gtest_filter 'AliasTests.invalid_recursive_alias'
```

#### `fidlc` test style guide

All `fidlc` compiler tests written in C++ must conform to the following rules:

*   Tests written using the `TEST` macro must have an UpperCamelCased group name
    of the format `<CATEGORY>Tests`. and an UpperCamelCased test case name.
    For example: `TEST(BitsTests, GoodValidBits) {...`.
*   Test case names should not begin or end with "Test" since it's redundant.
*   Test case names that test parsing and/or compilation must be prefixed with
    one of the following:
    *    `Good`: when the test case is expected to pass. Ex: `GoodValidMethod`.
    *    `Bad`: when the test case is expected to pass. Ex: `BadMustBeDense`.
    *    `Warn`: when the test case is expected to pass, but with reporter
         warnings. Warnings are intended for temporary use when introducing a
         new check, so tests prefixed with `Warn` should be changed to `Good`
         or `Bad` when the check is removed. Ex: `WarnTooManyProvidedLibraries`.

Additionally, test cases which expect compilation failures should use the
`ASSERT_ERRORED_DURING_COMPILE` and `ASSERT_ERRORED_TWICE_DURING_COMPILE` macros
in cases when one and two errors are expected, respectively.

#### `fidlc` goldens

To regenerate the `fidlc` JSON goldens:

```sh
fidldev regen fidlc
```

These "golden" files are examples of what kind of JSON IR `fidlc` produces and
are used to track changes. It is required to regenerate the golden files each
time the JSON IR is changed in any way, otherwise the `json_generator_tests` fails.

### fidlgen (LLCPP, HLCPP, Rust, Go, Dart)

Build:

```sh
fx build tools/fidl
```

Run:

```sh
$FUCHSIA_DIR/out/default/host_x64/fidlgen_{llcpp,hlcpp,rust,go,dart}
```

Some example tests you can run:

```sh
fx test fidlgen_hlcpp_golden_tests
fx test fidlgen_golang_lib_tests
fx test dart-bindings-test
fidldev test --no-regen fidlgen
```

To regenerate the goldens:

```sh
fidldev regen fidlgen
```

### fidlgen_banjo

Build:

```sh
fx build host_x64/fidlgen_banjo
```

Run tests:

```sh
fx build host_x64/fidlgen_banjo_unittests
./out/default/host_x64/fidlgen_banjo_unittests
```

### Bindings

`fidldev` supports tests for each of the bindings. Some of the bindings tests
run on device and require having Fuchsia running in an emulator. Here are the
steps:

```sh
Tab 1> fx build && fx serve

Tab 2> fx qemu -kN
```

The `-k` flag enables KVM. It is not required, but the emulator is *much* slower
without it. The `-N` flag enables networking.

The bindings tests can then be run with fidldev:

```sh
fidldev test --no-regen hlcpp
fidldev test --no-regen llcpp
fidldev test --no-regen c
fidldev test --no-regen go
fidldev test --no-regen rust
fidldev test --no-regen dart
```

Alternatively, run fidldev with no arguments to test files that have changed:

```sh
fidldev test
```

To run a specific test or to pass flags to a specific test, run `fidldev` with
the `--dry-run`, `--no-build`, `--no-regen` flags to obtain the desired test
commands.

### Compatibility test

Details about how the compatibility tests work and where the code is located can be
found in the README at [//src/tests/fidl/compatibility][compat_readme].

To run the compatibility tests, you first need to have Fuchsia running in an
emulator:

```sh
Tab 1> fx build && fx serve

Tab 2> fx qemu -kN
```

To run the compatibility tests:

```sh
Tab 3> fx set core.x64 --with //src/tests/fidl/compatibility
Tab 3> fx test fidl-compatibility-test
```

### GIDL

To rebuild GIDL:

```sh
fx build host-tools/gidl
```

### Measure tape

```
fx set core.x64 --with //tools/fidl/measure-tape/src:host
fx build
```

### All tests {#all-tests}

This section gives the full `fx test` commands to run all FIDL-related tests.
Use these instead of `fidldev test` if you want to run a specific test.

#### Bindings tests {#bindings-tests}

On device tests generally have greater coverage than host tests, due to support
for only running a subset of features on host. However, host tests can be
useful for debugging issues that prevent boot of the device.

##### On device

| Name                     | Test Command                        | Coverage
|--------------------------|-------------------------------------|---------------------------
| c runtime test, coding tables      | `fx test fidl_c_tests`    | //zircon/system/ulib/fidl                                                |
| walker, misc             | `fx test fidl-walker-tests`         |  //zircon/system/ulib/fidl
| walker tests w/ handle closing checks | `fx test fidl-handle-closing-tests` | //zircon/system/ulib/fidl
| hlcpp bindings tests including conformance tests     | `fx test fidl_hlcpp_unit_test_package fidl_hlcpp_conformance_test_package`         | //sdk/lib/fidl                                                             |
| llcpp bindings tests     | `fx test //src/lib/fidl/llcpp`      | //sdk/lib/fidl/llcpp
| unified C++ bindings tests | `fx test //src/lib/fidl/cpp`      | //src/lib/fidl/cpp
| go bindings tests        | `fx test go-fidl-tests`             | //third_party/go/syscall/zx/fidl //third_party/go/syscall/zx/fidl/fidl_test //src/tests/fidl/go_bindings_test |
| dart bindings tests      | `fx test dart-bindings-test`<br>(_see note below_) | //sdk/dart/fidl                                                  |
| rust bindings tests      | `fx test //src/lib/fidl/rust`           | //src/lib/fidl/rust |

Note: `fx test dart-bindings-test` needs `--with-base //src/dart:dart_jit_runner` or it will fail.
While `fx test dart-bindings-test` prints test names as they run, it does not show stack traces
for test failures. To see those, look at the `fx qemu` or `ffx log` output.

##### Host

| Name                     | Test Command                                    | Coverage
|--------------------------|-------------------------------------------------|---------------------------
| walker, misc             | `fx test --host fidl-walker-host-tests`         | //zircon/system/ulib/fidl
| hlcpp unittests          | `fx test --host fidl_hlcpp_unit_tests`          | //sdk/lib/fidl
| hlcpp conformance tests  | `fx test --host fidl_hlcpp_conformance_tests`   | //sdk/lib/fidl
| llcpp conformance tests  | `fx test --host fidl_llcpp_conformance_tests`   | //sdk/lib/fidl/llcpp
| rust conformance tests   | `fx test --host fidl_rust_conformance_tests`    | //src/lib/fidl/rust
| rust fidl lib tests      | `fx test --host fidl_rust_lib_tests`            | //src/lib/fidl/rust
| go conformance tests     | `fx test --host fidl_go_conformance_tests`      | //third_party/go/syscall/zx/fidl
| go fidl tests (extended) | `fx test --host go_extended_fidl_test`          | //third_party/go/syscall/zx/fidl
| go unsafevalue test      | `fx test --host go_unsafevalue_test`            | //third_party/go/syscall/zx/fidl/internal/unsafevalue

#### fidlgen tests

| Name                       | Test Command                                       | Coverage
|----------------------------|----------------------------------------------------|---------
| fidlgen type definitions   | `fx test fidlgen_lib_test`                         | //tools/fidl/lib/fidlgen
| fidlgen C++ specific IR    | `fx test fidlgen_cpp_ir_test`                      | //tools/fidl/lib/fidlgen_cpp
| fidlgen hlcpp              | `fx test fidlgen_hlcpp_golden_tests`               | //tools/fidl/fidlgen_hlcpp
| fidlgen llcpp              | `fx test fidlgen_cpp_golden_tests`               | //tools/fidl/fidlgen_cpp
| fidlgen golang             | `fx test fidlgen_go_{lib,golden}_tests`            | //tools/fidl/fidlgen_golang
| fidlgen rust               | `fx test fidlgen_rust_{lib,golden}_tests`          | //tools/fidl/fidlgen_rust
| fidlgen syzkaller          | `fx test fidlgen_syzkaller_golden_tests`           | //tools/fidl/fidlgen_syzkaller
| fidlgen dart               | `fx test fidlgen_dart_golden_tests`                | //tools/fidl/fidlgen_dart

#### Other

| Name                     | Test Command                        | Coverage
|--------------------------|-------------------------------------|---------------------------
| fidlc compiler           | `fx test fidlc-test`<br>`fx test fidlc_golden_tests` | //tools/fidl/fidlc
| gidl parser              | `fx test gidl_parser_test`          | //tools/fidl/gidl/parser
| measure tape test        | `fx test measure-tape_test`         | //tools/fidl/measure-tape
| Rust IR parser           | `fx build`                          | //src/devices/tools/fidlgen_banjo/tests/parser

### All benchmarks

Benchmarks can either be run directly or through one of two test runners:
fuchsia_benchmarks (old), SL4F (new).

Benchmarks on chromeperf are currently generated through the fuchsia_benchmarks
runner but are transitioning to SL4F.
During this transition, benchmarks should be integrated in both systems.

#### Directly running benchmarks

Ensure that the benchmarks are included in your build:

```
fx set core.x64 --with //src/tests/benchmarks
```

You will need to `fx build` and restart `qemu` for the packages to be
available.

Available benchmarks:

| Name | Benchmark Command | Notes |
|------|-------------------|-------|
| Go Benchmarks |  `fx shell /bin/go_fidl_microbenchmarks` | |
| Rust Benchmarks | `fx shell /bin/rust_fidl_microbenchmarks /tmp/myresultsfile` | Results can be viewed with `fx shell cat /tmp/myresultsfile/` |
| LLCPP benchmarks |  `fx shell /bin/llcpp_fidl_microbenchmarks` | |
| lib/fidl Benchmarks | `fx shell /bin/lib_fidl_microbenchmarks` | |
| Roundtrip Benchmarks | `fx shell /bin/roundtrip_fidl_benchmarks` | |

#### Running all benchmarks with SL4F benchmark runner

This runs benchmarks the same way they are run on CQ.
SL4F requires the `terminal.x64` product. Use `fx set` to switch products:

```
fx set terminal.x64
```

To run all FIDL tests, use:

```
fx test --e2e fidl_microbenchmarks_test
```

### All regen commands

This section gives the `fx regen-goldens` commands to regnerate all FIDL-related
golden files. This is what `fidldev regen` uses under the hood.

| Name            | Regen commands                                 | Input                     |  Output
|-----------------|------------------------------------------------|---------------------------|------------
| (all goldens)   | fx regen-goldens |  |
| fidlc goldens   | fx regen-goldens fidlc                         | tools/fidl/fidlc/testdata | tools/fidl/fidlc/goldens
| fidlgen goldens | fx regen-goldens $TOOL                         | tools/fidl/fidlc/testdata | tools/fidl/$TOOL/goldens
| fidldoc goldens | fx regen-goldens fidldoc                       | tools/fidl/fidlc/testdata | tools/fidl/fidldoc/goldens
| gidl goldens    | fx regen-goldens gidl | src/tests/fidl/conformance_suite/golden{.gidl,.test.fidl} | tools/fidl/gidl/goldens
| third party go  | fx exec $FUCHSIA_DIR/third_party/go/regen-fidl |                           |

### Compiling with `ninja`

In some cases, GN can build many unneeded targets. You can build a specific target with `ninja` instead of GN. In most cases, you can `grep` for the binary name to determine the `ninja` invocation.

For example, you can `grep` for `fidlgen_dart`:

```sh
fx ninja -C out/default -t targets all | grep -e 'fidlgen_dart:'
```

This example outputs a list of ninja targets, including `host_x64/fidlgen_dart`. Therefore, to
build `fidlgen_dart` run the following ninja command:

```sh
fx ninja -C out/default host_x64/fidlgen_dart
```

## Debugging (host)

There are several ways of debugging issues in host binaries. This section gives
instructions for the case where `fidlc --files test.fidl` is crashing:

- [GDB](#gdb)
- [ASan](#asan)
- [Valgrind](#valgrind)

Note: Even with all optimizations turned off, the binaries in
`out/default/host_x64` are stripped. For debugging, you should use the binaries
in the `exe.unstripped` subdirectory, such as `out/default/host_x64/exe.unstripped/fidlc`.

### GDB {#gdb}

First, `cd` to the build directory. You can also stay in `$FUCHSIA_DIR`, but
then you need to run `dir out/default` in GDB for it to find source files.

```sh
cd $FUCHSIA_DIR/out/default
```

Next, start GDB. The copy on your system might work, but the prebuilt `fx gdb`
is more likely to work with build artifacts in the Fuchsia project. See
`fx gnu --help` for the full list of prebuilt GNU tools.

```sh
fx gdb --args host_x64/exe.unstripped/fidlc --files test.fidl
```

Then, enter "r" to start the program. For additional uses, and a convenient
quick reference we've found this [GDB Cheat
Sheet](https://darkdust.net/files/GDB%20Cheat%20Sheet.pdf) very useful.

### ASan {#asan}

Ensure you are compiling with ASan enabled:

```sh
fx set core.x64 --variant=host_asan
fx build host_x64/fidlc
```

Then run `out/default/host_x64/fidlc --files test.fidl`. That binary should be
the same as `out/default/host_x64-asan/fidlc`.

### Valgrind {#valgrind}

On Google Linux machines, you may need to install a standard version of Valgrind
instead of using the pre-installed binary:

```sh
sudo apt-get install valgrind
```

Then:

```sh
valgrind -v -- out/default/host_x64/exe.unstripped/fidlc --files test.fidl
```

## Workflows

### Go fuchsia.io and fuchsia.net

To update all the saved `fidlgen` files, run the following command,
which automatically searches for and generates the necessary go files:

```sh
fx exec $FUCHSIA_DIR/third_party/go/regen-fidl
```

## FAQs

### Why is the C back-end different than all other back-ends?

The current C bindings are deprecated. See <https://fxbug.dev/79003> for more
information on the future of using FIDL in C.

### Why aren't all back-ends in one tool?

We'd actually like all back-ends to be in _separate_ tools!

Down the road, we plan to have a script over all the various tools (`fidlc`,
`fidlfmt`, the various back-ends) to make all things accessible easily,
and manage the chaining of these things.
For instance, it should be possible to generate Go bindings in one command such as:

```sh
fidl gen --library my_library.fidl --binding go --out-dir go/src/my/library
```

Or format a library in place with:

```sh
fidl fmt --library my_library.fidl -i
```

<!-- xrefs -->
[abi-api-compat]: development/languages/fidl/guides/compatibility/README.md
[fidl-readme]: development/languages/fidl
[cpp-style]: development/languages/c-cpp/cpp-style.md
[fidlc-source]: /tools/fidl/fidlc/
[fidlc-coding-tables-tests]: /src/lib/fidl/c/coding_tables_tests/
[fidl-simple]: /src/lib/fidl/c/simple_tests/
[fidlc-compiler]: /tools/fidl/fidlc/
[fidlc-compiler-tests]: /tools/fidl/fidlc/tests/
[walker-tests]: /src/lib/fidl/c/walker_tests/
[jsonir]: reference/fidl/language/json-ir.md
[getting_started]: get-started/README.md
[compat_readme]: /src/tests/fidl/compatibility/README.md
[go-test-flags]: https://golang.org/cmd/go/#hdr-Testing_flags
[fidl-misc]: https://fuchsia.googlesource.com/fidl-misc
[fidldev]: https://fuchsia.googlesource.com/fidl-misc/+/HEAD/fidldev
[RFC-0042]: contribute/governance/rfcs/0042_non_nullable_types.md
[pagination]: development/languages/fidl/guides/max-out-pagination.md
[commit-message]: contribute/commit-message-style-guide.md

[/tools/fidl/fidlc/formatter]: /tools/fidl/fidlc/formatter
[/tools/fidl/fidlc/linter]: /tools/fidl/fidlc/linter
[/tools/fidl/fidlc/lib/c_generator.cc]: /tools/fidl/fidlc/lib/c_generator.cc
[/tools/fidl/fidlc/lib/tables_generator.cc]: /tools/fidl/fidlc/lib/tables_generator.cc
[/tools/fidl/fidlgen_hlcpp]: /tools/fidl/fidlgen_hlcpp
[/tools/fidl/fidlgen_cpp]: /tools/fidl/fidlgen_cpp
[/tools/fidl/fidlgen_go]: /tools/fidl/fidlgen_go
[/tools/fidl/fidlgen_rust]: /tools/fidl/fidlgen_rust
[/tools/fidl/fidlgen_dart]: /tools/fidl/fidlgen_dart
[/sdk/lib/fidl/cpp]: /sdk/lib/fidl/cpp
[/sdk/lib/fidl/llcpp]: /sdk/lib/fidl/llcpp
[/src/lib/fidl/rust]: /src/lib/fidl/rust
[/zircon/system/ulib/fidl]: /zircon/system/ulib/fidl
[/third_party/go/src/syscall/zx/fidl]: /third_party/go/src/syscall/zx/fidl
[/sdk/dart/fidl]: /sdk/dart/fidl
[/src/lib/fidl/c]: /src/lib/fidl/c
[/src/lib/fidl/llcpp]: /src/lib/fidl/llcpp
[/src/tests/fidl/dart_bindings_test]: /src/tests/fidl/dart_bindings_test
[/tools/fidl/lib/fidlgen]: /tools/fidl/lib/fidlgen

[/tools/fidl/gidl]: /tools/fidl/gidl
[/src/tests/fidl/conformance_suite]: /src/tests/fidl/conformance_suite
[/src/lib/fidl/llcpp/tests/conformance/conformance_utils.h]: /src/lib/fidl/llcpp/tests/conformance/conformance_utils.h
[/src/lib/fidl/rust/gidl_util]: /src/lib/fidl/rust/gidl_util
[/third_party/go/src/syscall/zx/fidl/fidl_test]: /third_party/go/src/syscall/zx/fidl/fidl_test
[/src/lib/fidl/dart/gidl]: /src/lib/fidl/dart/gidl
[/src/tests/benchmarks/fidl/benchmark_suite]: /src/tests/benchmarks/fidl/benchmark_suite
[/src/tests/benchmarks/fidl]: /src/tests/benchmarks/fidl

[/src/tests/fidl/source_compatibility]: /src/tests/fidl/source_compatibility

[/src/tests/fidl/compatibility/]: /src/tests/fidl/compatibility/
[/src/tests/fidl/dangerous_identifiers]: /src/tests/fidl/dangerous_identifiers

[/tools/fidl/fidldoc]: /tools/fidl/fidldoc
[/tools/fidl/fidlmerge]: /tools/fidl/fidlmerge
[/tools/fidl/measure-tape]: /tools/fidl/measure-tape
[/tools/fidl/scripts]: /tools/fidl/scripts
[/src/lib/fostr]: /src/lib/fostr
[/src/lib/fostr/build]: /src/lib/fostr/build
[/src/lib/fidl_codec]: /src/lib/fidl_codec
