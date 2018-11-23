# How to write benchmarks

* Updated: 2018 August 9

[TOC]


## Overview

This guide will walk you through the process of writing a benchmark, running it at every
commit, and automatically tracking the results in the [Performance Dashboard].

Today we support automating benchmarks for these projects:
* Garnet (Also runs Zircon benchmarks)
* Peridot
* Topaz

## Writing a benchmark

Fuchsia benchmarks are command-line executables that produce a JSON results file.  The
executable must meet the following criteria:

1. It accepts the location to the results file as a command line flag.
2. It produces JSON results that match the [benchmark results schema]:

## Building your benchmark

Your benchmark executable should be built into a Fuchsia package.  For more information
please read the [Fuchsia package documentation].

## Automating your benchmark

We have shell scripts that run all of a layer's benchmarks at every commit to that layer.

* Garnet: [//garnet/tests/benchmarks](https://fuchsia.googlesource.com/garnet/+/master/tests/benchmarks)
* Peridot: [//peridot/tests/benchmarks](https://fuchsia.googlesource.com/peridot/+/master/tests/benchmarks)
* Topaz: [//topaz/tests/benchmarks](https://fuchsia.googlesource.com/topaz/+/master/tests/benchmarks)

These shell scripts are written using a helper library called [Runbenchmarks].  Add a
command to the appropriate script to execute your test.  See the existing commands for
examples.

## Testing

At this point, you're ready to build Fuchsia and test that your benchmark runs
successfully. Run the following in a shell:

```sh
fx set-petal $layer
jiri update -gc
# Benchmarks are not included in production packages, so use $layer/packages/kitchen_sink
# or they will not be built.
fx set <arch> --packages $layer/packages/kitchen_sink
fx full-build && fx run
```

Once the Fuchsia shell is loaded:

```sh
# Run just your benchmark
run my_benchmark [options]

# Run all benchmarks for $layer
/pkgfs/packages/${layer}_benchmarks/0/bin/benchmarks.sh /tmp
```

If no errors occurred, you should see your benchmark's output file in `/tmp`, along with
the results files of other benchmarks.

## Tracking in the performance dashboard

Please see the [Performance Dashboard User Guide]

NOTE: We do not yet have a User guide for the [Performance Dashboard Version 2].

[benchmark results schema]: results_schema.md
[Fuchsia package documentation]: /development/build/packages.md
[Performance Dashboard]: https://chromeperf.appspot.com/report
[Performance Dashboard User Guide]: catapult_user_guide.md
[Performance Dashboard Version 2]: https://v2spa-dot-chromeperf.appspot.com/
[Runbenchmarks]: https://fuchsia.googlesource.com/garnet/+/master/testing/runbenchmarks
[//zircon/system/ulib/perftest]: https://fuchsia.googlesource.com/zircon/+/master/system/ulib/perftest/
[//garnet/go/src/benchmarks]: https://fuchsia.googlesource.com/garnet/+/master/go/src/benchmarks
