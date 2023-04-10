# Improving your fuzzer

When you first begin fuzzing a new target, the fuzzer may crash very quickly. Fuzzing typically
produces an initial spike of defects found followed by a long tail. The frequency of defects being
found can drop for several reasons, including:
 * There are fewer defects in the code being tested.
 * There are sections of the code that aren't being tested by the fuzzer.

To distinguish between these, you need to be able to assess and improve your fuzzer.

## Improve code coverage

Note: This information is for the deprecated `fx fuzz` tool. Similar workflows are in development
for the `ffx fuzz` tool.

The first step in improving a fuzzer is to understand how well it is performing currently. An
obvious key metric for coverage-guided fuzzers is code coverage. You can collect this information
using `fx fuzz`.

For example:

<pre class="devsite-terminal">
fx fuzz analyze <var>package</var>/<var>fuzzer</var>
</pre>

This will run the fuzzer for 60 seconds and report the code coverage of the corpus on the device.
If you specify a `--staging` option, files in that directory will first be added to the corpus.

## Add or improve the seed corpus

If notice gaps in the code coverage, you can add individual inputs to a seed corpus:

1. Add a directory to the source tree near your fuzzer.
1. Add one or more files to this directory, each containing the raw bytes of a test input that
   causes the fuzzer to reach previously uncovered code.
1. Add a [`resource`][resource] GN target for this directory and add it to the fuzzer's `deps`.
1. Add an argument with the package-relative path to the fuzzer's
   [component manifest source][component-manifest-source].

For example:

<pre>
<code class="devsite-terminal">cd $FUCHSIA_DIR</code>
<code class="devsite-terminal">mkdir <var>path-to-library</var>/my-fuzzer-corpus</code>
<code class="devsite-terminal">cp <var>handcrafted-input</var> <var>path-to-library</var>/corpus</code>
</pre>

And in `//path/to/library/BUILD.gn`:

```
{% verbatim %}
import("//build/fuzz.gni")
import("//build/dist/resource.gni")

resource("my-library-corpus") {
  sources = [
    ...
    "corpus/handcrafted-input",
    ...
  ]
  outputs = [ "data/my-library-corpus/{{source_file_part}}" ]
}

cpp_fuzzer("my_fuzzer"){
  sources = [ "my-fuzzer.cc" ]
  deps = [
    ":my-library",
    ":my-library-corpus"
  ]
}
```

And in the fuzzer's [component manifest source][component-manifest-source]:

```
{
  ...
  program: {
    args: [
      ...
      "data/my-library-corpus",
      ...
    ]
  }
}
{% endverbatim %}
```

## Make code friendlier to fuzzing

Generally, libFuzzer is fairly effective at finding inputs that explore new conditional branches
when the decision is based on bytes of the input. For example, it can use instrumentation on
comparison instructions, such as CMP, to determine what value is needed to match a check on some
portion of the input.

But this approach can fail when the fuzzer encounters "fuzzer-hostile" conditions. These include:

* {C/C++}
  * Conditions that use data from external sources. For example:

    ```cpp
    zx_cprng_draw(&val, sizeof(val));
    if (val == 0) { ... }
    ```

  * Conditions checking values that are possible to construct, but hard to guess. For example:

    ```cpp
    uint32_t actual = header.checksum;
    header.checksum = 0;
    uint32_t expected =  crc32(0, reinterpret_cast<const uint8_t*>(&header), sizeof(header));
    if (actual == expected) { ... }
    ```

  * Conditions that check the results of [one-way functions][one-way-function]{:.external}.

    ```cpp
    int result = ECDSA_verify(0, data, data_len, signature, signature_len, ec_key);
    if (result == 0) { ... }
    ```

* {Rust}
  * Conditions that use data from external sources. For example:

    ```rust
    let mut randbuf = [0; 8];
    zx::cprng_draw(&mut randbuf)?;
    let val = u64::from_le_bytes(randbuf);
    if val == 0 { ... }
    ```

  * Conditions checking values that are possible to construct, but hard to guess. For example:

    ```rust
    let mut c = Checksum::new();
    c.add_bytes(&buf);
    c.checksum()
    if c == expected { ... }
    ```

  * Conditions that check the results of [one-way functions][one-way-function]{:.external}.

    ```rust
    let digest = H::hash(message);
    if boringssl::ecdsa_verify(digest.as_ref(), self.bytes(), &key.inner.key) { ... }
    ```

* {Go}

  Note: Go fuzzing is experimental and may not be supported on your development host.

  * Conditions that use data from external sources. For example:
    ```golang
    if rand.Intn(100) == 0 { ... }
    ```

  * Conditions checking values that are possible to construct, but hard to guess. For example:

    ```golang
    iCksum := ipv4.CalculateChecksum()
    if iCksum != want { ... }
    ```

  * Conditions that check the results of [one-way functions][one-way-function]{:.external}.

    ```golang
    ecdsaKey, ok := key.(*ecdsa.PublicKey)
    h := e.hash.New()
    h.Write(msg)
    if ecdsa.Verify(ecdsaKey, h.Sum(nil), ecdsaSignature.R, ecdsaSignature.S) { ... }
    ```

As a code maintainer, you can use conditional compilation to add workarounds to these conditions in
the code being tested. libFuzzer refers to this as using a [fuzzer-friendly build mode][friendly].

* {C/C++}

  Use the common build macro, `FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION`.

  For example:

  ```cpp
  #ifdef FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION
    // Use hard-coded value when fuzzing.
    memset(&val, 0, sizeof(val));
  #else
    zx_cprng_draw(&val, sizeof(val));
  #endif
  ```

  In this example, we have set all the bytes of `val` to always be zero. Depending on the code, it
  may be more useful to the fuzzer if `val` is some other deterministic value, or possibly even
  directly depends on the fuzzer input.

* {Rust}

  Use the `fuzz` [cfg attribute][cfg-attribute].

  For example:

  ```rust
  #[cfg(not(fuzz))]
  fn is_valid(&self, key: &EcPubKey<C>, message: &[u8]) -> bool {
      let digest = H::hash(message);
      boringssl::ecdsa_verify(digest.as_ref(), self.bytes(), &key.inner.key)
  }

  #[cfg(fuzz)]
  fn is_valid(&self, key: &EcPubKey<C>, message: &[u8]) -> bool {
      // Skip validation when fuzzing.
      return true;
  }
  ```

* {Go}

  Note: Go fuzzing is experimental and may not be supported on your development host.

  Use the `fuzz` package. Since Go only performs [conditional compilation][go-build] at the file
  level, this package include two files that define an `const Enabled <bool>`. Which file is
  included, and therefore the value of `Enabled` is determined by whether the code is being built in
  a fuzzer variant or not.

  For example:

  ```golang
  import "fuzz"

  func (b IPv4) CalculateChecksum() uint16 {
    if fuzz.Enabled {
      // Return hard-coded value when fuzzing.
      return uint16(0xffff)
    }
    return Checksum(b[:b.HeaderLength()], 0)
  }
  ```

### Add custom mutators

Note: Custom mutators are currently only supported in C/C++. You may be able to use them in other
languages using [Rust's FFI][rust-ffi]{:.external} or [Go's cgo][golang-cgo]{:.external}.

In some case, the inputs being provided by the fuzzer may be transformed before being acted on.
This can greatly reduce the fuzzer's ability to associate inputs with the behaviors they produce.
For example, a library being fuzzed my first decompress its inputs before processing them. The most
effective way to fuzz this library is to preform mutations on uncompressed inputs, then compress
them before invoking the library.

One way to achieve this is by adding custom mutators. These are user-provided implementations of
an optional function defined by LLVM's [fuzzing interface][fuzzer-interface]:

```cpp
extern "C" size_t LLVMFuzzerCustomMutator(uint8_t *Data, size_t Size,
                                          size_t MaxSize, unsigned int Seed);
```

If provided, libFuzzer will call this function with a buffer `Data` of size `MaxSize` initially
filled with `Size` bytes of a valid input from the corpus. This function can transform this data
before calling another LLVM [fuzzing interface][fuzzer-interface] function:

```cpp
size_t LLVMFuzzerMutate(uint8_t *Data, size_t Size, size_t MaxSize);
```

This function performs the actual mutation to create a new input.

In the example of the library that requires compressed inputs, a custom mutator could decompress the
input from the corpus, call `LLVMFuzzerMutate` on it to create a new input, and compress the result.

Google's public fuzzing documentation has detailed examples for this case and others. See
[Structure-Aware Fuzzing with libFuzzer][structure-aware-fuzzing]{:.external}.

### Provide a dictionary {#dictionary}

A dictionary is a set of tokens commonly found in an interface's valid inputs. When provided, a
fuzzer will use a dictionary to construct inputs that are more likely to be valid and therefore
provide deeper coverage.

If you know what sort of tokens your code expects, you can add them to a dictionary file, one per
line. Then provide the file to the fuzzer as a [`resource`][resource] in the same manner as a seed
corpus, and add them to the fuzzer's [component manifest source][component-manifest-source].

For example:

```
{
  ...
  program: {
    args: [
      ...
      "-dict=data/my-dictionary.txt",
      ...
    ]
  }
}
```

## Improve fuzzer performance

Once a fuzzer is achieving good coverage, then another key metric is simply how many iterations it
can perform for a given finite set of compute resources. Fuzzers and the code they test can have a
considerable range of complexity, so there isn't a single number of iterations per second a fuzzer
should perform or a single memory limit a fuzzer should stay below. But if everything else is the
same, a faster, leaner fuzzer will have a greater likelihood of finding defects over a slower, more
memory-intensive one.

### Startup initialization

Often, code under test requires some amount of expensive set up before it can be tested. Performing
this initialization on every iteration can drag down the fuzzer's speed. In these situations it can
be better to lazily initialize a variable with a lifetime equal to that of the process.

For example:

```cpp
bool SetUp() {
   DoSomeExpensiveWork();
   return true;
}

extern "C" LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
  static bool ready = SetUp();
  ...
}
```

However, be _very_ careful about program state that is carried over from one iteration to the
next. If a defect depends not only on the current test input, but also some subset of all previous
inputs, it can become very difficult to reproduce without replaying the entire fuzzer run.

### Pre-allocate storage

Some code operates on large amounts of memory, such as compression algorithms. A fuzzer that tries
to allocate and free many megabytes of memory on each iteration will see degraded performance. An
alternate approach is to use pre-allocate a maximally-sized buffer.

For example:

```cpp
static const size_t kMaxOutSize = 0x10000000; // 256 MB

extern "C" LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
  static uint8_t out_buf[kMaxOutSize];
  size_t max = Decompressor::GetMaxUncompressedSize(size);
  if (sizeof(out_buf) < max) {
    return 0;
  }
  Decompressor::Decompress(data, size, out_buf, max);
  return 0;
}
```

As written, this introduces a trade-off between performance and sanitizer accuracy. If the code
above instead allocated a memory region of length `max`, [AddressSanitizer][asan]{:.external} would
be able to detect if `Decompress` overflowed by any amount. With a pre-allocated region, it may
silently succeed. Fortunately, [AddressSanitizer][asan]{:.external} provides a way to
[manually poison][manual-poison]{:.external} memory.

For example:

```cpp
#include <sanitizer/asan_interface.h>

static const size_t kMaxOutSize = 0x10000000; // 256 MB

extern "C" LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
  static uint8_t out_buf[kMaxOutSize];
  size_t max = Decompressor::GetMaxUncompressedSize(size);
  if (sizeof(out_buf) < max) {
    return 0;
  }
  ASAN_POISON_MEMORY_REGION(&data[max], sizeof(out_buf) - max);
  Decompressor::Decompress(data, size, out_buf, max);
  ASAN_UNPOISON_MEMORY_REGION(&data[max], sizeof(out_buf) - max);
  return 0;
}
```

[asan]: https://clang.llvm.org/docs/AddressSanitizer.html
[fuzzer-interface]: https://github.com/llvm/llvm-project/blob/HEAD/compiler-rt/lib/fuzzer/FuzzerInterface.h
[golang-cgo]: https://golang.org/cmd/cgo/
[manual-poison]: https://github.com/google/sanitizers/wiki/AddressSanitizerManualPoisoning
[one-way-function]: https://en.wikipedia.org/wiki/One-way_function
[rust-ffi]: https://doc.rust-lang.org/nomicon/ffi.html
[structure-aware-fuzzing]: https://github.com/google/fuzzing/blob/HEAD/docs/structure-aware-fuzzing.md
