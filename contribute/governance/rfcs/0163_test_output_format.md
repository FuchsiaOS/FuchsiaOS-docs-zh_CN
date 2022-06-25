<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0163" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Former API Design Document

This RFC was previously submitted as an API design document and converted to an
RFC afterwards when the API design doc template was deprecated.

## Summary

This document proposes a stable, directory based format output by testing tools
provided through the fuchsia SDK.

## Goals and use cases

The [Test Architecture][test-runner-framework] allows clients to schedule
and execute multiple tests at once, and will also collect a large set of
diagnostic artifacts for each test. `ffx test` serves as the host tool through
which clients may interact with Test Architecture.

Automated tools that invoke `ffx test`, such as tools that run in CI
infrastructure, need a way to reliably obtain the complete set of results and
artifacts produced during test execution. Today, these tools obtain test
results by parsing stdout, which is brittle and cannot express the full set of
artifacts collected by Test Architecture. This design is intended to provide a
stable output format optimized for parsing by machines.

A stable format on disk also enables a developer to inspect results after a
test has run, or share results with other developers.

`ffx test` is intended for use by SDK consumers out-of-tree. To ensure these
users' tools are not broken by updates to the tool, the format must have
stability guarantees.

This output format is not intended to replace _all_ signals of test success
or failure. For example, `ffx test` will still support human readable output as
well as return codes that indicate success or failure.

### Background

A single execution of `ffx test` produces a *test run*. A *test run* is
comprised of *suites*, and each suite is comprised of *test cases*.

A *suite run* is an execution of a test component. The test component is most
commonly identified by its URL, for example,
`fuchsia-pkg://fuchsia.com/run_test_suite_integration_tests#meta/passing-test-example.cm`.
The same suite may be run multiple times in a *test run*. In this case there
are multiple separate *suite runs*.

A *test case* is an execution of a single test case contained in a suite. The
same *test case* may be run multiple times within a *suite run*. In this case
there are multiple *test cases*.

An *artifact* is diagnostic output collected by the Test Architecture. An
*artifact* is scoped to either the test run, a test suite run, or a test case.
For example, Test Architecture today collects stdout and stderr per test case,
but collects syslog per test suite run.

## Design

### Overview

Test output is stored as a directory. The root of the directory contains a
single JSON file and any number of subdirectories. A subdirectory will contain
artifacts scoped to a single test case, test suite, or the test run. The JSON
file contains test results, details of test execution, and lists the
artifacts contained in the directory.

### Directory Layout

The JSON file is always called `run_summary.json` and is always located in the
top level of the directory. `run_summary.json` contains the complete set of
results for the overall test run, suite runs within the test run, and test
cases within each suite run. It also contains the name of each artifact
subdirectory and a list of artifacts within it.

The names of subdirectories and artifacts within the subdirectories is
unspecified. The actual names of subdirectories and artifacts are defined in
`run_summary.json`. Artifacts are always located in the top level of an artifact
subdirectory.

### JSON Schemata

JSON schemata will be placed under [`//sdk/schema/ffx_test`][schema-dir] and
exported through the SDK. Schema evolution will rely on the schema versioning
mechanisms introduced in [RFC-0100][rfc-0100].

The initial version of the schema is defined in this [commit][schema-change].

### Consumption By Tools

Tools that need to interpret test results should begin by parsing
`run_summary.json`, which is the only file in the output format with a defined
location. `run_summary.json` contains the complete set of results and
references to all artifacts. Tools should not assume that any other locations
in the output are stable.

## Unknowns

The output format might be updated based on user needs. For example, we may
update the format to simplify parsing in common use cases we discover.

## Usability

### Extensibility and Evolution

The primary extensions we anticipate are addition of new test statuses and
artifact types. In general, additional enum variants and JSON fields to the
schemata are not breaking changes, and tools consuming the output can safely
ignore fields and enum variants they do not understand.
Breaking changes include modifications to required fields, and changes to the
structure of the directory. In these cases, a new version of the JSON schema
will be produced and published in the SDK following the strategy in
[RFC-0100][rfc-0100].

### Similar Outputs

Outside of Fuchsia, there are a number of individual test frameworks which
support a number of machine readable test result formats. For example,
[googletest][googletest] supports both a JSON and XML output format.

## Testing

Testing will primarily rely on unit and integration tests that verify that the
output produced by `ffx test` conforms to the output format.

## Performance considerations

Generation of this file format is not expected to take a significant amount of
time. A prototype implementation to save a suite with 100,000 test cases, each
with a stdout and stderr artifacts, takes under a second to generate and
persist `run_summary.json`, which is around 26 MB on disk. Given 1,000,000
cases, the same prototype takes around 15 seconds to generate
`run_summary.json`, which is around 256 MB on disk. In all cases the
prototype's maximum memory usage is roughly double the size of the summary on
disk.

For comparison, a large known test suite in chromium contains around 100,000
cases. Since saving `run_summary.json` for this number of cases takes under a
second, this should not pose an issue.

In fuchsia.git, the largest test suites today contain around 300 - 400 test
cases, and a single infra shard may contain around 300 suites. For
fuchsia.git, we intend to use ffx test's multitest feature, which will save
results for all test cases in a single output directory. So for this case, we
should expect up to around 120,000 cases in `run_summary.json`, which we
estimate to take less than 50 MB on disk and around a second to save.

There are a number of common local development flows where a developer runs a
test repeatedly. For example, a developer may run a test repeatedly to
reproduce a flaky failure. In such cases, we can easily run batches of several
thousand test cases without exceedimg 10 MB of memory usage.

## Security considerations

This output is used only to store results and artifacts produced by tests and
are already made available by the test framework, and does not introduce
additional security considerations.

## Privacy considerations

This output is used only to store results and artifacts produced by tests and
are already made available by the test framework, and does not introduce
additional privacy considerations.

## Drawbacks and alternatives

The output format supports multiple test suite runs by default. This complicates
parsing for clients that will only ever run one test at a time. One alternative
is to support an output format that contains results for a single test suite
run, and a second output format which contains results for multiple test suite
runs. While this simplifies immediate parsing, having multiple formats
complicates sharing tools that analyze the output.

Another alternative is to use multiple JSON files to store test results. One
disadvantage of using a single file to store all test results is that tools
that interact with it must hold the entire contents in memory at once. From the
prototype, we would need around 4 million test cases before `run_summary.json`
exceeds 1GB, at which point serializing would take around a minute. As our
current use cases are an order of magnitude less than this, multiple files are
not yet necessary. Since storing results in multiple files complicates parsing
we will use a single file.

A third alternative is to store all artifacts in `run_summary.json` to further
simplify parsing. This has the disadvantage that the artifacts also now need to
be held in memory. The largest artifacts collected today are profile data used
for coverage. In a single infra shard, this profile data is around 500 MB split
between multiple files.  While this profile does not exceed 1GB, a combination
of changes to how tests are sharded and how profile data is represented could
quickly bring the size closer to 1GB.

[googletest]: https://github.com/google/googletest
[rfc-0100]: contribute/governance/rfcs/0100_product_metadata.md#schema-evolution
[schema-change]: https://fuchsia-review.googlesource.com/c/fuchsia/+/654222
[schema-dir]: /sdk/schema/ffx_test/
[test-runner-framework]: development/testing/components/test_runner_framework.md
