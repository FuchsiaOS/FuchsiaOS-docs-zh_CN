# Build a fuzzer

This guide assumes you have already [created](write-a-fuzzer.md) a fuzzer that you now want to
build. It uses the same [sample code](write-a-fuzzer.md#samples) as in that guide.

Fuchsia uses [GN][fuchsia-gn], a meta-build system, to generate `.ninja` files that explicitly
describe how to build the system. [_GN targets_][gn-targets]{: .external} are nodes in the build graph
that represent a specific output such as a library or executable.
[_GN templates_][gn-templates]{: .external} are rules that generate additional targets.

In order to make adding new fuzzers as easy as possible, Fuchsia provides fuzzing-related GN
templates.

 * To create build rules for a fuzzer binary for Fuchsia, see the
   [Fuchsia library fuzzer GN template](#fuchsia-library-fuzzer) for the appropriate language.
 * To create build rules for a fuzzer binary for your development host, see the
   [Host library fuzzer GN template](#host-library-fuzzer)
 * To create build rules for a fuzzer component, see
   [Fuchsia fuzzer component GN template](#fuchsia-fuzzer-component).
 * To create build rules for a package of fuzzer binaries, see
   [Fuchsia fuzzer package GN template](#fuzzers-package).

Once you have defined your build rules, you can [build fuzzers with fx](#fx-set).

## Fuchsia library fuzzer GN template {#fuchsia-library-fuzzer}

Each language has a specific fuzzer GN template:

* {C/C++}
  The [`fuchsia_library_fuzzer`][fuchsia_library_fuzzer.gni] GN template generates an `executable`
  target that compiles and links the fuzz target function with the code under test and the fuzzing
  engine.

  To create build rules for a C or C++ fuzzer, add a `fuchsia_library_fuzzer` GN target to an
  appropriate BUILD.gn, such as the one with the corresponding unit test rules.

   For example:

   ```
   import("//build/fuzz.gni")

   fuchsia_library_fuzzer("parser-fuzzer") {
     sources = [ "parser_fuzzer.cc" ]
     deps = [ ":parser-lib" ]
   }
   ```

* {Rust}

  The [`rustc_fuzzer`][rustc_fuzzer.gni] GN template generates a GN target that compiles the Rust
  fuzz target function into a C object file that it then links with the fuzzing engine.

  To create build rules for a Rust fuzzer, add a `rustc_fuzzer` GN target to the crate's BUILD.gn.

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

When a [fuzzing variant][variants] is [selected](#fx-set), these templates will build a fuzzer
binary by linking the [libFuzzer] compiler runtime with the provided `sources`, `deps` or both. This
code must provide a [fuzz target][fuzz-target]{:.external} function.

Otherwise, a fuzzer unit test is built by linking a [test harness][test-harness] with the provided
code. This test harness calls the fuzz target function with fixed inputs, such
as a zero length input. This test ensures the fuzzer can compile and link, even when not building
for fuzzing.

Note: Since the generated unit test uses a zero-length input, your fuzzer _must not_ crash when
provided with a zero-length input. If a fuzzer input is shorter than your fuzzer's minimum input
length, you can simply return early.

## Host library fuzzer GN template {#fuchsia-library-fuzzer}

You can also build fuzzers that run on your development host using the Fuchsia build system.
To build host fuzzers, use the [`host_library_fuzzer`][host_library_fuzzer] GN template.

For example:

```
host_library_fuzzer("my_host_fuzzer") {
  sources = [ ... ]
  deps = [ ... ]
}
```

Host fuzzers can be built using [`fx`](#fx-set) without adding them to a Fuchsia component or
package.

## Fuchsia fuzzer component GN template

The `fuchsia_fuzzer_component` [template][fuchsia.fuzzer_component.gni] creates a component used to
run the fuzzer. It can include the usual component parameters, such as `component_name` and `deps`.

For example:

```
fuchsia_fuzzer_component("my-fuzzer-component") {
  component_name = "my-fuzzer"
  manifest = "meta/my-fuzzer.cml"
  deps = [ ":my-corpus"]
}
```

The [component manifest source][glossary.component_manifest_source] for library fuzzers must include
the [default shard for libfuzzer][libfuzzer_default_shard]. The output name of the fuzzer must be
provided as the first program argument as a package-relative path. Additional arguments may include
libFuzzer [options][options]{:.external}, or package-relative paths to directories of seed inputs
known as [seed corpora][corpus]{:.external}.

For example:

```
{
    include: [
        "//sdk/lib/inspect/client.shard.cml",
        "//src/sys/fuzzing/libfuzzer/default.shard.cml",
        "//src/sys/test_runners/fuzz/default.shard.cml",
        "//src/sys/test_runners/tmp_storage.shard.cml",
        "syslog/client.shard.cml",
    ]
    program: {
        args: [
            "test/my-fuzzer",
            "-max_input_size=256",
            "data/my-corpus",
        ]
    }
}
```

A seed corpus should match a [`resource`][resource] target that is included in the component's
`deps`.

For example:

```
{% verbatim %}
import("//build/dist/resource.gni")

resource("my-corpus") {
  sources = [
    "input0",
    "input1",
    "input2",
  ]
  outputs = [ "data/my-corpus/{{source_file_part}}" ]
}
{% endverbatim %}
```

## Fuchsia fuzzer package GN template {#fuchsia-fuzzer-package}

The `fuchsia_fuzzer_package` [template][fuchsia_fuzzer_package.gni] bundles fuzzer components into a
Fuchsia [package][glossary.package], similar to how `fuchsia_test_package` bundles test components.
The `fuchsia_fuzzer_package` template is distinguished by adding a specific [build_rule][build_rule]
to annotate fuzzers when built by a fuzzing toolchain [variant][variants].

Note: Executables built by these templates are only be capable of fuzzing if they are selected by a
fuzzing toolchain variant. If they are built by another toolchain, they only test a fixed set of
inputs. See [Build fuzzers with fx](#fx-set) for more details.

The template includes parameters that are lists of fuzzer components, organized by language. Each
language has a set of supported sanitizers provided by their toolchain as compiler runtimes.
When the selected toolchain variant includes a sanitizer that is supported for a given language, the
corresponding list of fuzzer components are capable of fuzzing.

For example, if the C++ toolchain has support for a hypotheical _examplesan_, the Rust toolchain
does not, and the _examplesan-fuzzer_ variant is selected, then the package definition below builds
`my-cpp-fuzzer` for fuzzing and `my-rust-fuzzer` for testing only.

```
fuchsia_fuzzer_package("my-fuzzers") {
  cpp_fuzz_components = [ ":my-cpp-fuzzer" ]
  rust_fuzz_components = [ ":my-rust-fuzzer" ]
}
```

It is not necessary to include a list if the package has no fuzzers written in the corresponding
languages.

A `fuchsia_fuzzer_package` can use all the same parameters as a [`fuchsia_package`][gn-package].

For example:

```
fuchsia_fuzzer_package("my-fuzzers") {
  package_name = "the-fuzzers"
  cpp_fuzz_components = [ ":my-fuzzer" ]
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
[sanitizer][sanitizers]{:.external} variant, including:

 * _asan_: Use [AddressSanitizer][asan]{:.external} to detect memory errors such as using memory
   after [freeing][asan-uaf]{:.external} or [returning][asan-uar]{:.external} it, overflowing
   [heap][asan-hbo]{:.external} and [stack][asan-sbo]{:.external} buffer overflows, and more.
 * _ubsan_: Use [UndefinedBehaviorSanitizer][ubsan]{:.external} to detect behavior that violates the
   language specification such as [signed integer overflow][ubsan-sio]{:.external}, misaligned
   pointers, and [more][ubsan-all]{:.external}.

The easiest way to build a `fuzzer_package` with a fuzzing variant is to use the
`--fuzz-with <sanitizer>` flag with [`fx set`][fx-set].

For example:

<pre>
<code class="devsite-terminal">fx set core.x64 --fuzz-with asan --with //bundles/tests</code>
<code class="devsite-terminal">fx build</code>
</pre>

Note: In some situations, Ninja cannot determine when an output needs to be rebuilt as a result of
compiler configuration changes. If building fails, try [`fx clean-build`][fx-build].

After running `fx set`, you can view the currently configured fuzzers with `ffx fuzz list`.
Additional `ffx fuzz` commands can be used to [run a fuzzer](run-a-fuzzer.md).

[glossary.package]: /glossary/README.md#package
[glossary.manifest]: /glossary/README.md#component-manifest-source
[asan]: https://clang.llvm.org/docs/AddressSanitizer.html
[asan-hbo]: https://github.com/google/sanitizers/wiki/AddressSanitizerExampleHeapOutOfBounds
[asan-sbo]: https://github.com/google/sanitizers/wiki/AddressSanitizerExampleStackOutOfBounds
[asan-uaf]: https://github.com/google/sanitizers/wiki/AddressSanitizerExampleUseAfterFree
[asan-uar]: https://github.com/google/sanitizers/wiki/AddressSanitizerExampleUseAfterReturn
[cpp_fuzzer.gni]: /build/cpp/cpp_fuzzer.gni
[fuchsia-gn]: /development/build/build_system/intro.md
[fuzz-target]: https://llvm.org/docs/LibFuzzer.html#fuzz-target
[fuzzer.gni]: /build/fuzzing/fuzzer.gni
[fuzzer_package.gni]: /build/fuzzing/fuzzer_package.gni
[fx-build]: /development/build/fx.md#execute-a-build
[fx-set]: /development/build/fx.md#configure-a-build
[gn-deps]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#var_deps
[gn-package]: /development/components/build.md
[gn-targets]: https://gn.googlesource.com/gn/+/HEAD/docs/language.md#Targets
[gn-templates]: https://gn.googlesource.com/gn/+/HEAD/docs/language.md#Templates
[known_variants]: /gen/build_arguments.md#known_variants
[options]: https://llvm.org/docs/LibFuzzer.html#options
[rustc_fuzzer.gni]: /build/rust/rustc_fuzzer.gni
[sanitizers]: https://github.com/google/sanitizers/wiki
[test-harness]: /src/lib/fuzzing/cpp/fuzzer_test.cc
[ubsan]: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html
[ubsan-sio]: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html#usage
[ubsan-all]: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html#available-checks
[variants]: /development/build/build_system/variants.md
