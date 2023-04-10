# Potential use cases for performance tests

## Background

This page describes various use cases that people may have for
automated performance tests.

One reason for describing these use cases is to document how well they
are served by Fuchsia's tools. Some of these use cases are better
served by Fuchsia's current tools than others.

A broader reason for describing these use cases is to help us
communicate about testing. The use cases one person has in mind for
testing may be different from the use cases another person has in
mind.

If a person has some use cases in mind when writing a test, it may be
useful to state the use cases explicitly when sending the test to a
reviewer for code review, or when asking for help.

The use cases for a test are related to how we treat regressions in
its results, which in turn is affected by limitations in how we
measure performance and limitations of the statistical tests we use
for detecting regressions and improvements.

For example, a large set of tests that is useful for *comparing cases*
(see below) may produce a high rate of false regression warnings due
to the [**multiple comparisons
problem**](https://en.wikipedia.org/wiki/Multiple_comparisons_problem)
in statistics. This set of tests might therefore not be very useful
for *detecting regressions*. We might find that only large regressions
are actionable, while small regression warnings are usually spurious
and can be ignored.

Some of the use cases described here overlap -- they are not meant to
be mutually exclusive.

## Use cases

*   **Detecting regressions**, either post-commit or pre-commit: We
    mostly do this post-commit using [Chromeperf and the culprit
    finder tools](fuchsiaperf_consumers.md). Pre-commit detection is
    opt-in only, via [perfcompare](perfcompare.md) -- it is not
    applied by default.

    *   **Detecting gradual regressions (creep)**, resulting from the
        cumulative effect of many changes. We currently do not have
        any automated tools for doing this. Chromeperf's detection
        algorithm only looks for regressions introduced by a single
        revision or at a single point in time.

*   **Testing potential improvements**: That is, testing the effect of
    changes that attempt to improve performance. This can be done
    using [perfcompare](perfcompare.md).

*   **Comparing cases**: That is, comparing the relative performance
    of related test cases.

    This can be used to look for cases that perform unexpectedly badly
    relative to others, because we may want to fix those cases. As an
    example, the performance tests for FIDL encoding and decoding have
    this as a use case.

    This can also be used to measure the costs of operations or
    subsystems without using profiling. For example, the [IPC round
    trip microbenchmarks] measure the round trip time between threads
    or processes using various different kernel and userland IPC
    operations. By testing this with and without using FIDL, we can
    estimate the overhead that FIDL and other userland libraries add
    on top of the kernel IPC primitives. Similarly, by testing IPC
    between processes and between threads within a process, we can
    estimate the cost of a context switch that switches between
    address spaces.

*   **Providing clues about other regressions**: A regression in
    metric A might not be something we care about as such, but it
    might be useful in providing a clue about the cause of a
    regression in metric B. This use case is similar to profiling, but
    more general.

    For example, if the *frames per second* metric has regressed, we
    can look at the *frame build time* metric to see whether that also
    changed.

*   **Profiling**: That is, analyzing the breakdown of time or memory
    usage within a test.

    While Linux has tools such as [perf] and [OProfile] for doing
    statistical profiling of CPU time usage, Fuchsia currently has no
    equivalent tools.

    It is common to use Fuchsia's tracing system for examining the
    breakdown of time usage, for either automated tests or manual
    tests. (For this, automated tests have the benefit over manual
    tests of being more reproducible and less work to run.) However,
    Fuchsia's tracing system has two differences from statistical
    profiling tools like perf and OProfile:

    *   Tracing only records time usage for code that has annotated to
        produce trace events.
    *   The typical uses for traces are to inspect them manually or to
        extract from them a fixed set of metrics (such as *frames per
        second* and *frame build times*). We don't yet have tools for
        generating more open-ended sets of statistics of the kind
        usually produced by profiling tools.

    Note that the infrastructure around `fuchsiaperf` files is not
    well suited to recording profiling data. It is not well suited to
    recording large numbers of metrics describing the breakdown of
    time or memory usage.

*   **Informing design decisions**: The performance characteristics of
    a subsystem inform how we use it. If the subsystem is slow, we
    might avoid it, build a layer on top of it (such as caching), or
    work around it in some other way.

    An example of this use case is the "Latency numbers every
    programmer should know" table. See [this recent
    version][latency-numbers-pdf] of the table.

    An early version of this table appears in a [talk by Jeff
    Dean][talk] ([Stanford CS295 class lecture, spring,
    2007][talk-link-context]) in which he advocates writing
    microbenchmarks for building intuition about performance and for
    using as a basis for performance estimates. Various updated
    versions of this table exist; see [this Stack Exchange
    question][stack-exchange-page] for further discussion.

[IPC round trip microbenchmarks]: /src/tests/microbenchmarks/round_trips.cc
[perf]: https://en.wikipedia.org/wiki/Perf_(Linux)
[OProfile]: https://en.wikipedia.org/wiki/OProfile
[latency-numbers-pdf]: https://static.googleusercontent.com/media/sre.google/en//static/pdf/rule-of-thumb-latency-numbers-letter.pdf
[talk]: https://static.googleusercontent.com/media/research.google.com/en/us/people/jeff/stanford-295-talk.pdf#page=13
[talk-link-context]: https://research.google/people/jeff/
[stack-exchange-page]: https://softwareengineering.stackexchange.com/questions/312485/how-can-jeff-deans-latency-numbers-every-programmer-should-know-be-accurate-i
