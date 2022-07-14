# Viewing microbenchmarks with Catapult

[TOC]

## Overview

The Catapult dashboard is the UI for monitoring and
visualizing benchmark results. The dashboard is maintained by the Chrome team.
This a short guide on how to find and use the results of your benchmarks
in the dashboard.


## Accessing the dashboard

**Be sure to sign into your google.com account or else Fuchsia data will be hidden.**

The login button is in the top right corner of the screen.

The dashboard can be found at [https://chromeperf.appspot.com/report](https://chromeperf.appspot.com/report).


## Searching and adding graphs

The dashboard displays a list of search boxes.  The placeholder names are relics from the
days when Chrome infrastructure was still using BuildBot.  Since they are not relevant to
Fuchsia infrastructure, Fuchsia data is mapped to these fields with the following scheme:

* `Test suite` == the name of the benchmark suite.
* `Bot` == A Fuchsia LUCI builder that has run the benchmark at least once.
* `Subtest` == The name of the test case in your benchmark suite.

Type the name of your benchmark suite in the first box to begin searching.  For
example, if there is a benchmark suite named "zircon_benchmarks", it will get
displayed with autocomplete if you type "zircon".

![test_suite_example](/images/benchmarking/test_suite_example.png "test_suite_example")

Select a builder and a subtest.  Note that if your subtest is named "foo", there will be
multiple "foo_<metric_name>" subtests to choose from.  Each of these represents a metric
computed from the sample(s) of that subtest.   For example: if "foo" generates N sample
points each time the benchmark is run, then the subtest "foo_avg" is a plot of the
averages of these N samples.

When you're finished filling out each field, click "Add" to add your graph to the UI.
You should see something like this:

![graph_example](/images/benchmarking/graph_example.png "graph_example")


## Viewing sample metadata

If you hover over a point in a graph, you can see some extra information such as the
point's value, the date it was recorded, and a link to the log page of the build that
generated it.

![tooltip_example](/images/benchmarking/tooltip_example.png "tooltip_example")


## Saving the view

The Catapult dashboard UI does not have a built in mechanism for saving a collection
of Graphs.  If you want to save a list of graphs so that you can share with others or
re-open the list later, you can copy the URL from the Chrome Address Bar.

Beware, you will have to re-copy the URL each time you add, modify or remove a graph. This
includes moving the green slider beneath a graph or making any selections in the box to
the right of the graph.
