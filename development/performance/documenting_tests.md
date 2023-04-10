# Documenting performance tests

[TOC]

Each performance test should come with an English-language description
of what the test measures, typically in a single sentence.

For example, *"Measure the time taken for an IPC round trip between
processes, using Zircon channels"*.

An exception is for trivial tests (primarily one-liners) where the
code does not need summarizing. For a family of very similar tests,
only a single description is necessary.

It may also be useful to say what the intended use cases for the test
are. See the list of [potential use cases for performance
tests](use_cases_for_tests.md).

## Rationale

Having a description of a test is more important for performance tests
than for correctness tests (pass-or-fail tests), because the
interpretation of performance tests is more subtle than for
pass-or-fail results.

For example, if your CL breaks a correctness test so that the test
always fails, it is unambiguous that you must either find a way to
make the test pass or remove the test. In contrast, if your CL makes a
test 10% slower, it is much less clear whether that matters. If a
change has made a test 50% slower 1% of the time, it is even less
clear whether that should be considered important.

Furthermore, more things in the codebase can affect the results from a
performance test than the pass-or-fail results from a correctness
test. To be more precise, there are many ways the codebase can be
changed that would not affect correctness but would affect
performance.

This tends to mean that more people need to interpret performance
results than pass-or-fail results of a given test. For example, if a
change to component A causes a regression in component B's performance
tests, the meaning of those performance tests may need to be
interpreted by the maintainers of component A and the maintainers of
component B, and by other people triaging post-commit performance
regression issues.

As a result, there should be a higher bar for documenting performance
tests than for correctness tests.

The description of what a test measures will often be much shorter
than the test code, so providing a description will potentially save
developers from spending a lot of time reading the code to figure out
what it is intended to measure.

## Location

The description of a performance test can be put in comments in the
test code, or in nearby Markdown files.

We currently do not have a browsable list of performance tests and
their descriptions, or a way to extract test descriptions from the
code, but we might add one of these in the future.

## Examples

*   Examples from Fuchsia microbenchmarks:

    > "Test IPC round trips using Zircon channels where the client and
    > server both use Zircon ports to wait."

    (Source: microbenchmarks for IPC round trip times,
    [src/tests/microbenchmarks/round_trips.cc](/src/tests/microbenchmarks/round_trips.cc))

    > "Measure the times taken to enqueue and then dequeue a message
    > from a Zircon channel, on a single thread. This does not involve
    > any cross-thread wakeups."

    (Source: microbenchmark for channels,
    [src/tests/microbenchmarks/channels.cc](/src/tests/microbenchmarks/channels.cc))

*   Example from storage performance tests:

    > "Measures the time to write blocks of size 8KiB in turn to a
    > newly-created file, using the system filesystem, without any
    > explicit syncing or flushing. The resulting metric is the time for
    > each per-block write to return."

    (Source:
    [old version of garnet/bin/odu/README.md](https://fuchsia.googlesource.com/fuchsia/+/81d891c03f2419a7f9ee9bc3af70647519f8c311/garnet/bin/odu/README.md);
    the [current description](/src/storage/benchmarks/README.md) is longer)
