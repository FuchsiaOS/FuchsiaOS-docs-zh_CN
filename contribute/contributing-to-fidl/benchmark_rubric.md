
# FIDL Benchmark Rubric

## Builder, Encode Decode Microbenchmarks

Each binding must implement a Builder, Encode and Decode benchmark. It is fine to benchmark substeps or variants of these, but there should be one benchmark matching the following requirements for binding-to-binding comparison.

### Builder

This benchmark populates the binding’s representation of the object that would be passed to the encoder. There can sometimes be multiple ways to build these objects and a general guideline is to use the fastest method of building the object that is still natural to users. It is fine to record multiple values for different builder techniques, but one should be chosen as the main value for comparison.

#### Setup

Setup time should be excluded from the build time. It should only include the minimal set of steps needed to build.

Handle creation is considered setup time and is not included in the builder benchmark.

#### Destructors

Any destructors for objects required to be created for the build process should be included in the time. Any additional destructors such as those from setup steps should be excluded from the builder time.

While handles are created during setup, handles are consumed by the built object and the destructors will be called when the built object is cleaned up.

### Encode

Encode takes the object built by the builder phase and encodes it into the wire format bytes and handles. As specified in the FIDL wire format, encoding must validate the message. Any buffers that are allocated during this benchmark should be the smallest size required by the binding. Encode should reuse and follow exactly the same steps as actual binding code wherever possible.

#### Setup

Setup time should be excluded from the encode time. It should only include the minimal set of steps needed to encode.

#### Destructors

Any destructors for objects required to be created for the encode process should be included in the time. Destructors for build or setup, including those of built objects that are measured during the build-benchmark, should be excluded. This means that neither handle creation time nor destruction time is included in the benchmark.

### Decode

Decode takes the encoded bytes and handles and decodes them into the appropriate binding object. As specified in the FIDL wire format, decoding must validate the message. Any buffers that are allocated during this benchmark should be the smallest size required by the binding. Decode should reuse and follow exactly the same steps as actual binding code wherever possible.

#### Setup

Setup time should be excluded from the build time. It should only include the minimal set of steps needed to build.

#### Destructors

Any destructors for objects required to be created for the decode process should be included in the time. Any additional destructors from setup, build or encode should not be included.

Note: this means that closing decoded handles is included in the time

### Measurements

For each benchmark, we want to record:

* WallTime: wall time in nanoseconds (ns)
* Allocations: # Heap Allocs as a count
* AllocatedBytes: Total # bytes heap allocated in bytes
* StackDepth: max stack depth in bytes, i.e. difference between the lowest point in the starting stack size to the maximal stack size reached

WallTime will be the main focus of most of the optimizations so it has the highest priority. However, WallTime is influenced by the number of heap allocations, especially unintended ones so heap allocations are a secondary focus. Separate from optimizing WallTime, deep stacks are an issue for Rust and potentially for C++ as well, so in those languages FIDL’s contribution to stack depth should also be tracked.

### Benchmark Suite

Benchmarks in the benchmarks suite should be easy to describe, understand and talk about. They generally should come from two categories:

* Methodically created synthetic benchmarks. These should isolate specific features or combinations of features, chosen to target hypothesized weaknesses in bindings. Parameters of the benchmarks, size as number of elements should be chosen to match other benchmarks to ease comparison.

Synthetic benchmarks should NOT be arbitrary combinations of features - it can be difficult to determine what they measure, if they are worth optimizing or the effect of measurements on actual performance.

Examples of synthetic benchmarks include tables with 16, 32, 256, 1024 fields of primitives, byte vector with size 16, 32, 256, 1024, structs with alternating pairs or uint8, uint64.

* Benchmarks based on actual fidl types. These measure performance in the wild. These are useful for both forward-looking optimizations and tracking regressions.

Examples of regression benchmarks are fuchsia.io/File.ReadAt response, or fuchsia.posix.socket/DatagramSocket.SendMsg request.

#### GIDL Generation

GIDL generation should be used to produce each binding’s benchmark code from a standardized set of benchmarks.

## Requirements for All Benchmarks

### Chromeperf Integration

All benchmarks should export to chromeperf and use the test_suite: fuchsia.fidl_microbenchmarks.

#### Chromeperf paths for Builder / Encode / Decode benchmarks

* Builder benchmark path `[Binding Flavour]/Builder/[Benchmark Path]/[Measurement]`
* Encode benchmark path `[Binding Flavour]/Encode/[Benchmark Path]/[Measurement]`
* Decode benchmark path `[Binding Flavour]/Decode/[Benchmark Path]/[Measurement]`

Example:

```
Go/Builder/ByteArray/16/WallTime
```

Binding names are one of: `LLCPP`,` HLCPP`,` Rust`,` Go`,` Dart` (case sensitive)

Benchmark paths are strings identifying a particular benchmark. They can contain slashes e.g. “`ByteArray/1`”. Each word is upper camel case. Benchmark names and parts of the names should be singular e.g. ManyStructField (NOT ManyStructFields).

Measurement names are one of: `WallTime`,` Allocations`,` AllocatedBytes`,` StackDepth` (case sensitive)

#### Chromeperf paths for other benchmarks

The structure is `[Overall Category]/[Specific Benchmark]/[Subbenchmarks]/[Measurements]`.

Generally moving from least specific towards more specific going left to right.

Some details:

* If the benchmark is specific to a binding, it should be prefixed with `[Binding Flavour]/` in the path (e.g. `Go/`). If the benchmark is unrelated to a specific binding, no binding prefix should be used (e.g. `Count/` or `Memcpy/`). If there is one instance of a benchmark perf binding flavour, it should be prefixed with the binding flavour.
* Subbenchmarks (such as sizes) appear after the main benchmark, from most significant to least significant left to right.
* Substeps appear just before the measurement (this is required by the C++ perftest structure).
* A measurement type should be specified if there is a possibility of having multiple measurement types for the benchmark. Otherwise it can be omitted.
* When there are parameters to a benchmark, the parameter name should be specified if it isn’t clear or there are multiple parameters. It can be omitted if it is obvious. The parameter name should prefix the value and be upper camel case (e.g. Len256, Concurrency100)

Examples:

* `LLCPP/Encode/ByteArray/16/Step.Encode/WallTime`
    * Starts with binding, then type of benchmark (encode)
    * Benchmark name is followed by size and subbenchmark name (16)
    * It is then followed by substep name
    * Last, it has the measurement
* `Go/Roundtrip/AsciiString/Len256/Concurrency100`
    * Starts with binding, then type of benchmark (roundtrip)
    * Benchmark name is followed by two named subbenchmark parameters
    * No measurement because only wall time is expect to ever be measured
* `Memcpy/ByteArray/16`
    * Starts with benchmark type. No binding because not applicable
    * Followed by benchmark and subbenchmark name
    * No measurement because only wall time is expected to ever be measured.

### File Paths

Benchmarks should be created in `src/tests/benchmarks/fidl`. If part of the benchmark code _must_ exist elsewhere, effort should be made in creating an executable and build target in `src/tests/benchmarks/fidl` (see wrapper in dart).

Builder/encode/decode benchmarks can exist in a binding specific directory `src/tests/benchmarks/fidl/{go,rust,dart,hlcpp,llcpp}`.

Other benchmarks should be put in a subfolder corresponding to benchmark type. e.g. `src/tests/benchmarks/fidl/roundtrip`. In rare cases, where it is access is needed to specific generated types, they can be placed elsewhere (for instance memcpy benchmark inside of llcpp to access llcpp types to test memcpy of correct size). This should be considered tech debt and eventually moved to a dedicated benchmark and folder.
