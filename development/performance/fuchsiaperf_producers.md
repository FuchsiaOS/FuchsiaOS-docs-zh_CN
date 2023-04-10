# Producing Fuchsia performance test results

This page lists libraries for producing Fuchsia performance test
results files.

The format used for Fuchsia performance test results is the
[`fuchsiaperf.json`](fuchsiaperf_format.md) format. All performance
tests that run on Fuchsia Infra and whose results are tracked by the
Fuchsia tools produce results in the `fuchsiaperf.json` format. All of
these tests are run through the Dart-based SL4F testing framework,
though in many cases the Dart SL4F code is just a small wrapper that
runs some other test program that generates a `fuchsiaperf.json` file.

## Options

There are multiple options for generating `fuchsiaperf.json` files,
depending on which programming language you want to use.

The low level options listed below are thin wrappers for outputting
`fuchsiaperf` JSON files, whereas the higher level options make more
assumptions about the type of performance test being written.

*   **Dart:**

    *   **High level:** You can use the Dart [`trace_processing`
        library][trace_processing] to extract performance metrics from
        Fuchsia traces. This approach is useful if you have an
        existing correctness test and you want to extend it to also
        produce performance results. In that case, it is common to
        modify the software-under-test to generate extra trace events.

        An example is
        [`flatland_benchmarks_test.dart`](/src/tests/end_to_end/perf/test/flatland_benchmarks_test.dart),
        which uses the `trace_processing` library by defining a
        `MetricsSpecSet`.

    *   **Low level:** You can use the [`TestCaseResults`
        class][metrics_results] to generate entries for
        `fuchsiaperf.json` files. This is commonly used with the
        `trace_processing` library, but it can also be used
        separately.

    *   **High or low level:** From Dart, you can run a subprocess
        that generates a `fuchsiaperf.json` file. The subprocess can
        run code written in a language other than Dart. There are
        various [Dart SL4F wrappers][dart-wrappers] that do this.

*   **C++:** The [perftest C++ library] provides two interfaces:

    *   **High level:** You can use [perftest.h] to create
        microbenchmarks. In this context, a microbenchmark is a test
        where we run an operation repeatedly, in isolation, and
        measure its running time. New microbenchmarks can be added to
        [src/tests/microbenchmarks/](/src/tests/microbenchmarks/), or
        they can be added elsewhere in the source tree if they are
        significantly different from the tests in that directory.

    *   **Low level:** You can use [perftest/results.h] to generate
        `fuchsiaperf.json` files more directly.

*   **Rust:**

    *   **High level:** You can use the [fuchsia-criterion Rust
        library] to create microbenchmarks.

    *   **Low level:** You can use the [Fuchsiaperf Rust library] to
        generate `fuchsiaperf.json` files.

*   **Go:**

    *   **Low level:** You can use the [go-benchmarking] library to
        generate `fuchsiaperf.json` files.


[dart-wrappers]: /src/tests/end_to_end/perf/test/
[trace_processing]: /sdk/testing/sl4f/client/lib/src/trace_processing/
[metrics_results]: /sdk/testing/sl4f/client/lib/src/trace_processing/metrics_results.dart
[perftest C++ library]: /zircon/system/ulib/perftest/
[perftest.h]: /zircon/system/ulib/perftest/include/perftest/perftest.h
[perftest/results.h]: /zircon/system/ulib/perftest/include/perftest/results.h
[fuchsia-criterion Rust library]: /src/developer/fuchsia-criterion/
[go-benchmarking]: /src/lib/go-benchmarking/
[Fuchsiaperf Rust library]: /src/performance/lib/fuchsiaperf/src/lib.rs

