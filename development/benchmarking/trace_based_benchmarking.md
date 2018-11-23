# Trace-based benchmarking

* Updated: 2018 Sep 18

This document describes how to use trace-based benchmarking to measure and track
performance of Fuchsia apps.

[TOC]

## Overview

Trace-based benchmarks measure the performance of an application by running it
under [tracing] and analyzing the collected traces to
compute performance metrics.

For a typical **service** application (application to which clients connect over
FIDL), the following components participate in a benchmarking run:

 - **service binary** - the service being benchmarked.
 - **benchmark app** - a client app that connects to the service and
     exercises the usage patterns we are interested in benchmarking.
 - **benchmark spec** - a JSON file specifying which trace events captured
     during a run of the benchmark app should be measured, and how.

The same framework can be also used to benchmark single binaries (without the
client-server split).

## Mechanics

Trace-based benchmarks are run using the `trace` binary. The spec file needs to be
passed to the tool as follows:

```sh
trace record --spec-file=<path to the spec file>
```

### Specification file

The specification file configures tracing parameters and specifies measurements.
(see [examples/benchmark] if you'd like to see a full example straight away)

The file supports the following top level-parameters:

 - `app`: string, url of the application to be run
 - `args`: array of strings, startup arguments to be passed to the application
 - `categories`: array of strings, tracing categories to be enabled
 - `duration`: integer, maximum duration of tracing in seconds
 - `measure`: array of measurement specifications, see below

Given the specification file, the `trace` tool runs the `app` with the given
`args` for at most `duration` seconds and gathers trace events from the selected
`categories`. Then, the tool computes the measurements specified in the
`measure` section on the recorded trace events.

Example:

```{json}
{
  "app": "benchmark_example",
  "args": [],
  "categories": ["benchmark"],
  "measure": [
    ...
  ]
}
```

For any tracing parameters that can be passed both as arguments to `trace record`
and set in the specification file, the command line value overrides the one from
the file.


### Measurement types

The `trace` tool supports the following types of measurements:

 - `duration`
 - `time_between`
 - `argument_value`

A `duration` measurement targets a single trace event and computes the
duration of its occurrences. The target trace event can be recorded as a
duration, an async, or a flow event.

**Example**:

```{json}
    {
      "type": "duration",
      "event_name": "example",
      "event_category": "benchmark"
    },
```


A `time_between` measurement targets two trace events with the specified
anchors (either the beginning or the end of the events) and computes the time
between the consecutive occurrences of the two. The target events can be
"duration", "async", "flow" or "instant" (in which case the anchor doesn't matter).
Takes arguments: `first_event_name`, `first_event_category`,
`first_event_anchor`, `second_event_name`, `second_event_category`,
`second_event_anchor`.

**Example**:

```{json}
    {
      "type": "time_between",
      "first_event_name": "task_end",
      "first_event_category": "benchmark",
      "second_event_name": "task_start",
      "second_event_category": "benchmark"
    }
```

In the example above the `time_between` measurement captures the time between
the two instant events and measures the time between the end of one task and
the beginning of another.


An `argument_value` measurement is used to record a value of an argument passed
to the trace event. Takes as arguments a name and category of the event, name of
the argument to be recorded and unit in which it is measured. The type of trace
event doesn't matter, but the recorded argument must have `uint64` type.

**Example**:

```{json}
    {
      "type": "argument_value",
      "event_name": "example",
      "event_category": "benchmark",
      "argument_name": "disk_space_used",
      "argument_unit": "Mb"
    }
```

### Samples

It is possible to specify an exact number of expected samples. In order to do
so, an optional parameter `"expected_sample_count"` with a positive value must be
specified for a given measurement. In that case, if the number of recorded
samples does not match the one provided, an error will be logged and the
measurement will produce no results (failing the benchmark).

You can also specify the `"split_first"` flag to separate the first sample from
the rest. This is useful for recording the "cold run" samples (see the
[best practices] section). This flag is passed to the exported file as well, in
compliance with the [results schema].

### Full example

See [examples/benchmark] for a full example of a traced-based benchmark.


This example can be run with the following command:
```{shell}
trace record --spec-file=/pkgfs/packages/benchmark/0/data/benchmark_example.tspec
```

## Best practices

### Consider reusing benchmark binaries

The separation between specification files and benchmark binaries allows to
define multiple benchmarks based on a single benchmark binary. Note that you can
parametrize the benchmark binary by taking command line arguments which can be
set to different values in each spec file.

### Record "cold run" samples separately

For any duration measurement that happens more than once, chances are that the
first time has different performance characteristics that the subsequent ones.
You can set `"split_first": true` to report and track the first sample
separately.

## Results

By default, the results are printed on the command line in a human-friendly
format.

### Export

If you prefer a machine-friendly format, pass the path to the output file to
`trace record` as `--benchmark-results-file=<file>`.  See the [results schema]
for the format of the resulting file.

### Dashboard upload

Dashboard upload integration and infra support is WIP as of March, 2018.  See
the [dashboard user guide] and the instructions for [automating benchmarks].

[automating benchmarks]: running_on_ci.md
[dashboard user guide]: catapult_user_guide.md
[examples/benchmark]: https://fuchsia.googlesource.com/garnet/+/master/examples/benchmark/
[results schema]: results_schema.md
[best practices]: #best-practices
[tracing]: https://fuchsia.googlesource.com/garnet/+/master/docs/tracing_usage_guide.md

