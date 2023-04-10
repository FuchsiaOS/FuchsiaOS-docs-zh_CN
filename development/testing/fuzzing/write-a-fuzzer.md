# Write a fuzzer

Fuchsia's toolchain supports fuzzing using LLVM's [libFuzzer]. To create a fuzzer for a particular
interface, you need to implement a [fuzz target function][fuzz-target]{:.external} that uses a
provided sequence of bytes to exercise the interface. The sequence of bytes is referred to as a
fuzzer "input". The fuzz target function is used by libFuzzer to search for inputs that cause panics
or other errors.

## Sample Code to Fuzz {#samples}

For each of the examples below, assume you want to test code like the following:

* {C/C++}

  ```cpp
  class Parser {
    Parser(const std::string &name, uint32_t flags);
    virtual ~Parser();
    int Parse(const uint8 *buf, size_t len);
  };
  ```

* {Rust}

  ```rust
  struct ToyStruct {
      n: u8,
      s: String,
  }

  fn toy_example(input: ToyStruct) -> Result<u8, &'static str>;
  ```

## Simple Fuzz Target Function {#basic}

For each language, your fuzz target function will use the bytes provided to call the code you want
to fuzz. If the interface being fuzzed has documented constraints on its parameters, you can reject
inputs that don't meet those constraints. You can also ignore returned errors since failing
gracefully on invalid parameters is correct behavior.

* {C/C++}

  For C and C++, the fuzz target function must have the signature
  `extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)` and return 0:

  ```cpp
  extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    Parser parser("", 0);
    if (size < 5) {
      return 0;
    }
    parser.Parse(data, size);
    return 0;
  }
  ```

  Place this code in a source file adjacent to code being fuzzed as you would with a unit test. For
  example, the code above might be in `parser-fuzztest.cc`.

* {Rust}

  For Rust, the recommended approach is to use the `Arbitrary` trait discussed in the next section.

  It is also possible to create a "manual" fuzz target function that is analgous to the simple fuzz
  target functions in other languages. This function must take a reference to a byte slice as its
  single parameter, return nothing, and have the [`#[fuzz]`][fuzz-crate] attribute:

  ```rust
  use fuzz::fuzz;

  #[fuzz]
  fn toy_example_u8(input: &[u8]) {
      if input.len() == 0 {
          return
      }
      let n = input[0];
      if let Ok(s) = std::str::from_utf8(input[1:]) {
          let _ = toy_example(ToyStruct{n, s: s.to_string(),});
      }
  }
  ```

  As with unit tests, this code can be place in the same file as the code it is testing. For
  example, the code above might be in `toy_example/src/lib.rs`.

## Support for Fuzzing More Complex Types {#advanced}

Each language has utilities to facilitate making more complicated fuzz target functions:

* {C/C++}

  The [`FuzzedDataProvider`][fuzzed-data-provider]{:.external} class provided by LLVM can help you
  map portions of the provided `data` to more complex types.

  For example:

  ```cpp
    #include <fuzzer/FuzzedDataProvider.h>

    extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
      FuzzedDataProvider provider(data, size);
      auto flags = provider.ConsumeIntegral<uint32_t>();
      auto name = provider.ConsumeRandomLengthString();
      Parser parser(name, flags);
      auto buf = provider.ConsumeRemainingBytes<uint8_t>();
      parser.Parse(buf.data(), buf.size());
      return 0;
    }
  ```

  There are two notable advantages to using this library:

   * First, it makes it easier to rapidly write a fuzzer.
   * Second, it is designed to dynamically split inputs in such a way that the fuzzer can
     efficiently create new inputs from coverage data.

  There is one notable disadvantage:

   * Since inputs are dynamically split, it is more difficult to provide a pre-existing
     [corpus][corpus]{:.external}. It is still feasible to provide a
     [dictionary][dictionary]{:.external}.

* {Rust}

  You can create a fuzz target function that takes one or more inputs with the `Arbitrary` trait
  from the [`arbitrary`][arbitrary]{:.external} crate. This is the recommended approach.

  To write a fuzz target function that automatically transforms arbitrary inputs:

  1. If needed, implement the `Arbitrary` trait for the types used by your test code. If possible,
     the recommended way to do this is by automatically deriving the trait. Otherwise, this can be
     done "by hand" by following the crate's [instructions][arbitrary]{:.external}.

     For example, in your `src/lib.rs`:

     ```rust
     use arbitrary:Arbitrary;

     #[derive(Arbitrary)]
     struct ToyStruct { ... }
     ```

  1. Create a function with the [`#[fuzz]`][fuzz-crate] attribute that passes the necessary
     parameters to the code you wish to test.

     For example, in your `src/lib.rs`:

     ```rust
     use fuzz::fuzz;

     #[fuzz]
     fn toy_example_arbitrary(input: ToyStruct) {
         let _ = toy_example(input);
     }
     ```

Next, you can [build](build-a-fuzzer.md) your newly created fuzzer using GN and Ninja.

[arbitrary]: https://docs.rs/arbitrary/0.4.0/arbitrary
[corpus]: https://llvm.org/docs/LibFuzzer.html#corpus
[dictionary]: https://llvm.org/docs/LibFuzzer.html#dictionaries
[fuzz-crate]: /src/lib/fuzzing/rust/src/lib.rs
[fuzz-target]: https://llvm.org/docs/LibFuzzer.html#fuzz-target
[fuzzed-data-provider]: https://github.com/google/fuzzing/blob/HEAD/docs/split-inputs.md#fuzzed-data-provider
