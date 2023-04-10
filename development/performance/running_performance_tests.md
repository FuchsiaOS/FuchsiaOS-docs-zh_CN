# How to run performance tests

The primary way to run a performance test locally is to run its
host-side entry point.  These entry points are defined in
[src/tests/end_to_end/perf/](/src/tests/end_to_end/perf/).  These are
what are used in Fuchsia Infra builds for collecting performance
results for uploading to Chromeperf.  Running one of these entry
points involves the following:

*   Use the `terminal` product, in order to match what is tested on
    Fuchsia Infra in CI and CQ.  Example:

    ```
    fx set terminal.x64-reduced-perf-variation
    fx build
    ```

*   Launch Fuchsia on hardware or on an emulator as usual.

*   Run the test's host-side entry point with `fx test --e2e`.  Example:

    ```
    fx test --e2e host_x64/fidlc_microbenchmarks_test
    ```

The test will write its results to a newly-created, timestamped
subdirectory of `out/test_out`.  This will include
`*.fuchsiaperf.json` files containing the performance results.  It may
include other files such as Fuchsia traces.
