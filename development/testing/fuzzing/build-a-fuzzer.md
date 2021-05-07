# Build a fuzzer

This guide assumes you have already [created](write-a-fuzzer.md) a fuzzer that you now want to
build. It uses the same [sample code](write-a-fuzzer.md#samples) as in that guide.

Fuchsia uses [GN][fuchsia-gn], a meta-build system, to generate `.ninja` files that explicitly
describe how to build the system. [_GN targets_][gn-targets] are nodes in the build graph that
represent a specific output such as a library or executable. [_GN templates_][gn-templates] are
rules that generate additional targets.

In order to make adding new fuzzers as easy as possible, Fuchsia provides fuzzing-related GN
templates.

 * To build a fuzzer binary, see [Fuzzer GN template](#fuzzer) for the appropriate language.
 * To assemble a package of fuzzer binaries, see [Fuzzers package GN template](#fuzzers-package).

## Fuzzer GN template {#fuzzer}

Each language has a specific fuzzer GN template. All of these templates support certain common
parameters, as detailed in [fuzzer.gni]:

 * An optional [component manifest][cmx] file. A manifest for fuzzing is always generated. If a
   `cmx` file is provided, it is combined with and overrides the generated file.
 * An optional [`dictionary`][dictionary]. If not provided, an empty dictionary file is created.
 * An optional list of libFuzzer [`options`]. These key-value pairs are written to a options file.

For example:

```
cpp_fuzzer("my-fuzzer") {
  output_name = "the-fuzzer"
  sources = [ "my_fuzzer.cc" ]
  deps = [ ":my-lib" ]
  dictionary = "my.dict"
  cmx = "meta/the-fuzzer.cmx"
  options = [
    "key1=val1",
    "key2=val2",
  ]
}
```

Each language has a specific fuzzer GN template:

* {C/C++}
  The [`cpp_fuzzer`][cpp_fuzzer.gni] GN template generates a GN target that compiles the fuzz target
  function and links it with the code under test and with libFuzzer.

  To build a C or C++ fuzzer, add a `cpp_fuzzer` GN target to an appropriate BUILD.gn.

   For example:

   ```
   import("//build/cpp/cpp_fuzzer.gni")

   cpp_fuzzer("parser-fuzzer") {
     sources = [ "parser_fuzzer.cc" ]
     deps = [ ":parser-lib" ]
   }
   ```

* {Rust}

  The [`rustc_fuzzer`][rustc_fuzzer.gni] GN template generates a GN target that compiles the Rust
  fuzz target function into a C object file that it then links with libFuzzer.

  To build a Rust fuzzer, add a `rustc_fuzzer` GN target to the crate's BUILD.gn.

  When choosing where and how to add this target, consider the following:

  * It is recommended to have the fuzzer name match the fuzz target function name, and to include
    the fuzz target function in a Rust library, i.e. in `src/lib.rs`. You may leave the body of the
    template empty when following these recommendations. For example, using the
    `toy_example_arbitrary` [example](write-a-fuzzer.md#advanced), you would add the following to
    your `BUILD.gn`:

    ```
    import("//build/rust/rustc_fuzzer.gni")

    rustc_fuzzer("toy_example_arbitrary") {
    }
    ```

  * If the fuzz target function name differs from the fuzzer name, you must provide it with the
    `rustfunction` parameter. For example, using the `toy_example_u8`
    [example](write-a-fuzzer.md#basic), you would add the following to your `BUILD.gn`:

    ```
    import("//build/rust/rustc_fuzzer.gni")

    rustc_fuzzer("toy_example_raw_bytes") {
        rustfunction = "toy_example_u8"
    }
    ```

  * If the code to be tested cannot be easily factored into a library, a Rust binary can be used
    with two additional steps:

    * You must exclude the `main` function from compilation, along with any items not used when
      fuzzing, e.g. imports only used in `main`. For example:

      ```rust
      #[cfg(not(fuzz))]
      use only::used::in::main;

      #[cfg(not(fuzz))]
      fn main() { ... }
      ```

    * You must explicitly provide the fuzz target function to the `rustc_fuzzer` with the
      `source_root` parameter. For example, in your `BUILD.gn`:

      ```
      import("//build/rust/rustc_fuzzer.gni")

      rustc_fuzzer("toy_example_with_main") {
          source_root = "src/main.rs"
      }
      ```

* {Go}

  The [`go_fuzzer`][go_fuzzer.gni] GN template generates a GN target that compiles the Go fuzz
  target function into a C object file that it then links with libFuzzer.

  To build a Go fuzzer:

  1. Ensure the Go package in the [previous step](#write) is available as a `go_library` GN target.

     For example:
     ```
     import("//build/go/go_library.gni")

     go_library("my_library") {
       sources = "pkg/file_with_fuzz.go"
     }
     ```

  1. Write a `go_fuzzer` GN target to build the package containing the fuzz target function. Make
     sure to include the `go_library` in [`deps`][gn-deps].

     For example:
     ```
     import("//build/go/go_fuzzer.gni")

     go_fuzzer("my_fuzzer") {
       gopackage = "my_library/pkg"
       deps = [ ":my_library" ]
     }
     ```

When a [fuzzing variant][variants] is [selected](#fx-set), these templates will build a fuzzer
binary by linking the [libFuzzer] compiler runtime against code that provides a
[fuzz target][fuzz-target] function.

Otherwise, a fuzzer unit test is built by linking a [test harness][test-harness] that calls the
fuzz target function with a zero length input against the provided `sources`, `deps`, or both. This
test ensures the fuzzer can compile and link, even when not building for fuzzing.

Note: Since the generated unit test uses a zero-length input, your fuzzer _must not_ crash when
provided with a zero-length input. If a fuzzer input is shorter than your fuzzer's minimum input
length, you can simply return early.

## Fuzzers package GN template {#fuzzers-package}

The `fuzzers_package` [template][fuzzer.gni] bundles fuzzers into a Fuchsia [package] similar to how
a normal `package` bundles binaries or a `test_package` bundles tests. The `fuzzers_package`
template is distinguished from these other package templates in how it interacts with the currently
selected toolchain [variants].

Note: The Fuchsia build system will build the fuzzers _only_ if their package is
selected by a fuzzing variant. See [Build fuzzers with fx](#fx-set).

The most important parameters to the template are the lists of fuzzers, organized by language.

For example:

```
fuzzers_package("my-fuzzers") {
  cpp_fuzzers = [ ":my-cpp-fuzzer" ]
  go_fuzzers = [ ":my-go-fuzzer" ]
  rust_fuzzers = [ ":my-rust-fuzzer" ]
}
```

It is not necessary to include a list if the package has no fuzzers written in the corresponding
languages.

A `fuzzers_package` can use all the same parameters as a [`package`][gn-package].

For example:

```
fuzzers_package("my-fuzzers") {
  package_name = "the-fuzzers"
  cpp_fuzzers = [ ":my-fuzzer" ]
}
```

Additional parameters include:

 * `fuzz_host`: Also builds a fuzzer as a host tool (when [selected](#variant-selection)). Defaults
    to false.
 * `host_only`: Implies `fuzz_host` and does not create a Fuchsia package. Defaults to false.
 * `sanitizers`: Sets the [sanitizers] to match during [selection](#variant-selection). Defaults to
    language-specific lists in [fuzzer.gni]. This typically does not need to be set.

For example:

```
fuzzers_package("my-fuzzers") {
  cpp_fuzzers = [ ":my-fuzzer" ]
  fuzz_host = true
}
```

The list of fuzzers can contain a mix of GN labels and scopes. Each scope element must include a
label and can override the parameters above. Additionally, scopes can indicate output names for
fuzzers that specify them.

For example:

```
fuzzers_package("my-fuzzers") {
  cpp_fuzzers = [
    {
      label = ":my-fuzzer"
      output_name = "the-fuzzer"
    },
    {
      label = ":no-host-fuzzer"
      fuzz_host = false
    },
  ]
  fuzz_host = true
}
```

Once defined, a package needs to be included in the build dependency graph like any other test
package. This typically means adding it to a group of tests.

For example:

```
group("tests") {
  deps = [
    ":my-test-package",
    ":my-fuzzers",
  ]
}
```

## Build fuzzers with `fx` {#fx-set}

As noted above, the Fuchsia build system will build the fuzzers _only_ if it is explicitly told to
instrument them for fuzzing with an appropriate fuzzing variant. These are the
[known variants][known_variants] that end in `-fuzzer`. Each one is an extension of a
[sanitizer][sanitizers] variant, including:

 * _asan_: Use [AddressSanitizer][asan] to detect memory errors such as memory usage after
   [free][asan-uaf] or [return][asan-uar], [heap][asan-hbo] and [stack][asan-sbo] buffer overflows,
   and more.
 * _ubsan_: Use [UndefinedBehaviorSanitizer][ubsan] to detect behavior that violates the language
   specification such as [signed integer overflow][ubsan-sio], misaligned pointers, and
   [more][ubsan-all].
 * _lsan_: Use [LeakSanitizer][lsan] to detect memory leaks.

The easiest way to build a `fuzzers_package` with a fuzzing variant is to use the
`--fuzz-with <sanitizer>` flag with [`fx set`][fx-set].

For example:

<pre>
<code class="devsite-terminal">fx set core.x64 --fuzz-with asan --with //bundles:tests</code>
<code class="devsite-terminal">fx build</code>
</pre>

Note: In some situations, Ninja cannot determine when an output needs to be rebuilt as a result of
compiler configuration changes. If building fails, try [`fx clean-build`][fx-build].

After running `fx set`, you can view the currently configured fuzzers with `fx fuzz list`.
Additional `fx fuzz` commands can be used to [run a fuzzer](run-a-fuzzer.md).

[asan]: https://clang.llvm.org/docs/AddressSanitizer.html
[asan-hbo]: https://github.com/google/sanitizers/wiki/AddressSanitizerExampleHeapOutOfBounds
[asan-sbo]: https://github.com/google/sanitizers/wiki/AddressSanitizerExampleStackOutOfBounds
[asan-uaf]: https://github.com/google/sanitizers/wiki/AddressSanitizerExampleUseAfterFree
[asan-uar]: https://github.com/google/sanitizers/wiki/AddressSanitizerExampleUseAfterReturn
[cmx]: /docs/glossary.md#component-manifest
[cpp_fuzzer.gni]: /build/cpp/cpp_fuzzer.gni
[dictionary]: https://llvm.org/docs/LibFuzzer.html#dictionaries
[fuchsia-gn]: /docs/concepts/build_system/intro.md
[fuzz-target]: https://llvm.org/docs/LibFuzzer.html#fuzz-target
[fuzzer.gni]: /build/fuzzing/fuzzer.gni
[fx-build]: /docs/development/build/fx.md#execute-a-build
[fx-set]: /docs/development/build/fx.md#configure-a-build
[gn-deps]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#var_deps
[gn-package]: /build/package.gni
[gn-targets]: https://gn.googlesource.com/gn/+/HEAD/docs/language.md#Targets
[gn-templates]: https://gn.googlesource.com/gn/+/HEAD/docs/language.md#Templates
[go_fuzzer.gni]: /build/go/go_fuzzer.gni
[known_variants]: /docs/gen/build_arguments.md#known_variants
[lsan]: https://clang.llvm.org/docs/LeakSanitizer.html
[options]: https://llvm.org/docs/LibFuzzer.html#options
[package]: /docs/glossary.md#fuchsia-package
[rustc_fuzzer.gni]: /build/rust/rustc_fuzzer.gni
[sanitizers]: https://github.com/google/sanitizers/wiki
[test-harness]: /src/lib/fuzzing/cpp/fuzzer_test.cc
[ubsan]: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html
[ubsan-sio]: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html#usage
[ubsan-all]: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html#available-checks
[variants]: /docs/concepts/build_system/variants.md
