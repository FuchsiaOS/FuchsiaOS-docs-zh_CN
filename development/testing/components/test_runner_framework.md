# The Fuchsia Test Runner Framework

The Fuchsia [Component Framework][cf] allows developers to create components in
a variety of languages and runtimes. Fuchsia's own code uses a diverse mix of
programming languages for components, including C/C++, Rust, Dart, and Go.

The Test Runner Framework uses Component Framework [runners][runners] as an
integration layer between various testing runtimes and a common Fuchsia protocol
for launching tests and receiving their results. This makes for an inclusive
design that on one hand allows developers to bring their language and testing
framework of choice, and on the other hand allows building and testing Fuchsia
on a variety of systems and targeting different hardware.

## The Test Manager

The `test_manager` component is responsible for running tests on a Fuchsia
device. Test manager exposes the
[`fuchsia.test.manager.RunBuilder`][fidl-test-manager] protocol, which allows
launching test suites.

Each test suite is launched as a child of test manager. Test suites are offered
capabilities by test manager that enable them to do their work while
maintaining isolation between the test and the rest of the system. For instance
hermetic tests are given the capability to log messages, but are not given the
capability to interact with real system resources outside of their sandbox.
Test manager uses only one capability from the test realm, a controller protocol
that test suites expose. This is done to ensure hermeticity (test results aren't
affected by anything outside of their intended sandbox) and isolation (tests
don't affect each other or the rest of the system).

The test manager controller itself is offered to other components in the system
in order to integrate test execution with various developer tools. Tests can
then be launched with such tools as [`fx test`][fx-test] and [`ffx`][ffx].

## The test suite protocol {#test-suite-protocol}

The test suite protocol, [`fuchsia.test.Suite`][fidl-test-suite], is used by the
test manager to control tests, such as to invoke test cases and to collect their
results.

Test authors typically don't need to implement this protocol. Instead, they rely
on a [test runner](#test-runners) to do this for them. For instance, you might
write a test in C++ using the GoogleTest framework, and then use
[`gtest_runner`](#gtest-runner) in your [component manifest][component-manifest]
to integrate with the Test Runner Framework.

## Test runners {#test-runners}

### A language and runtime-inclusive framework

Test runners are reusable adapters between the Test Runner Framework and common
languages & frameworks used by developers to write tests. They implement the
[`fuchsia.test.Suite`][fidl-test-suite] protocol on behalf of the test author,
allowing developers to write idiomatic tests for their language and framework of
choice.

Component manifests for simple unit tests can be [generated][unit-tests]
by the build rules. Generated component manifests for v2 tests will include the
appropriate test runner based on their build definition. For instance a test
executable that depends on the GoogleTest library will include the
[GoogleTest runner](#gtest-runner) in its generated manifest.

### Inventory of test runners

The following test runners are currently available for general use:

#### GoogleTest runner {#gtest-runner}

A runner for tests written in C/C++ using the GoogleTest framework.
Use this for all tests written using GoogleTest.

Common GoogleTest features are supported, such as disabling tests, running
only specified tests, running the same test multiple times, etc'.
Standard output, standard error, and logs are captured from the test.

In order to use this runner, add the following to your component manifest:

```json5
{
    include: [ "//src/sys/test_runners/gtest/default.shard.cml" ]
}
```

By default GoogleTest test cases run serially (one test case at a time).

#### GoogleTest (Gunit) runner {#gunit-runner}

A runner for tests written in C/C++ using the GUnit framework.
Use this for all tests written using the gUnit flavor of GoogleTest.

Note: Gtest and Gunit testing framework differ in flag names, so we have a
separate runner for gunit.

Common GoogleTest features are supported, such as disabling tests, running
only specified tests, running the same test multiple times, etc'.
Standard output, standard error, and logs are captured from the test.

In order to use this runner, add the following to your component manifest:

```json5
{
    include: [ "//src/sys/test_runners/gunit/default.shard.cml" ]
}
```

By default test cases run serially (one test case at a time).

#### Rust runner {#rust-runner}

A runner for tests written in the Rust programming language and following Rust
testing idioms.
Use this for all idiomatic Rust tests (i.e. tests with modules that set the
attribute `[cfg(test)]`).

Common Rust testing features are supported, such as disabling tests, running
only specified tests, running the same test multiple times, etc'.
Standard output, standard error, and logs are captured from the test.

In order to use this runner, add the following to your component manifest:

```json5
{
    include: [ "//src/sys/test_runners/rust/default.shard.cml" ]
}
```

By default Rust test cases run in parallel, at most 10 cases at a time.

#### Go test runner {#gotest-runner}

A runner for tests written in the Go programming language and following Go
testing idioms.
Use this for all tests written in Go using `import "testing"`.

Common Go testing features are supported, such as disabling tests, running
only specified tests, running the same test multiple times, etc'.
Standard output, standard error, and logs are captured from the test.

In order to use this runner, add the following to your component manifest:

```json5
{
    include: [ "//src/sys/test_runners/gotests/default.shard.cml" ]
}
```

By default Go test cases run in parallel, at most 10 cases at a time.

#### ELF test runner {#elf-test-runner}

The simplest test runner - it waits for your program to terminate, then reports
that the test passed if the program returned zero or that it failed for any
non-zero return value.

Use this test runner if your test is implemented as an ELF program (for instance
an executable written in C/C++) but it does not use a common testing framework
that's supported by existing runners and you'd rather not implement a bespoke
test runner.

In order to use this runner, add the following to your component manifest:

```json5
{
    include: [ "sys/testing/elf_test_runner.shard.cml" ]
}
```

If you are [using in-tree unit test GN templates][component-unit-tests],
and you are not already using a test framework with a dedicated test runner,
add the following to your build deps:

```
fuchsia_unittest_package("my-test-packkage") {
    // ...
    deps = [
        // ...
        "//src/sys/testing/elftest",
    ]
}
```

Note: If you see the error message "Component has a \`program\` block defined,
but doesn't specify a \`runner\`" for your test, this indicates you are not using a
test framework with a dedicated test runner, and you should add the above dependency.

#### Legacy test runner {#legacy-test-runner}

Legacy tests are tests that were written before the Test Runner Framework was
introduced. The legacy test runner offers a simple adapter between the modern
test framework and legacy tests that were not converted to modern ones. For help
with migrations see [this guide][sys-migration-guide].
**It is not recommended to use the legacy test runner in new tests.**

The legacy test runner detects if a test passed or failed by observing its
return code, with zero indicating success and non-zero indicating failure.

All legacy tests are automatically wrapped in a modern test and executed using
the legacy test runner. The launch URL of the wrapper will be derived from the wrapped
test's launch URL. For instance:

&nbsp;&nbsp;&nbsp;&nbsp;`fuchsia-pkg://fuchsia.com/package#meta/test_component.cmx`

will become:

&nbsp;&nbsp;&nbsp;&nbsp;`fuchsia-pkg://fuchsia.com/package#meta/test_component.cm`

The legacy test runner does not understand concepts such as test cases (or
filtering on them), running multiple test cases in parallel, etc. It does
however forward arguments to the test, so you can pass arguments that are
specific to the underlying test framework. For instance, to run just a specific
test case from a gtest:

```posix-terminal
fx test <test> -- --gtest_filter=MyTestCase
```

To run Rust tests, at most 5 test cases at a time:

```posix-terminal
fx test <test> -- --test-threads=5
```

To suppress this behavior set `wrap_cmx_test_with_cml_test` to false on `fuchsia_test_package`
or `fuchsia_unittest_package`. **Don't forget to file a bug and track the reason
for the exclusion.**

Change your `BUILD.gn` to exclude your legacy test:

```gn
import("//build/components.gni")

# This is your legacy test
fuchsia_test_component("simple_test_legacy") {
  component_name = "simple_test"
  manifest = "meta/simple_test.cmx"
  deps = [ ":simple_test_bin" ]
}

# Exclude your test from auto-wrapping.
fuchsia_test_package("simple_test") {
  test_components = [ ":simple_test_legacy" ]

  # TODO(fxbug.dev/XXXXX) : Excluding the test due to ...
  # Remove below line once the issue is fixed.
  wrap_cmx_test_with_cml_test = false
}

```

### Controlling parallel execution of test cases

When using `fx test` to launch tests, they may run each test case in sequence or
run multiple test cases in parallel up to a given limit. The default
parallelism behavior is determined by the test runner. To manually control the
number of test cases to run in parallel use test spec:

```gn
fuchsia_test_package("my-test-pkg") {
  test_components = [ ":my_test_component" ]
  test_specs = {
    # control the parallelism
    parallel = 10
  }
}
```

To override the value specified in the test spec, pass the parallel option when
invoking fx test:

```posix-terminal
fx test --parallel=5 <test_url>
```

### Running test multiple times

To run a test multiple times use:

```posix-terminal
 fx test --count=<n> <test_url>
```

If an iteration times out, no further iteration will be executed.

### Passing arguments

Custom arguments to the tests can be passed using `fx test`:

```posix-terminal
fx test <test_url> -- <custom_args>
```

Individual test runners have restrictions on these custom flags:

#### GoogleTest runner {#gtest-runner-custom-arg}

Note the following known behavior change:

**--gtest_break_on_failure**: As each test case is executed in a different process,
this flag will not work.

The following flags are restricted and the test fails if any are passed as
fuchsia.test.Suite provides equivalent functionality that replaces them.

- **--gtest_filter** - Instead use:

```posix-terminal
 fx test --test-filter=<glob_pattern> <test_url>
```

`--test-filter` may be specified multiple times. Tests that match any of the
given glob patterns will be executed.

- **--gtest_also_run_disabled_tests** - Instead use:

```posix-terminal
 fx test --also-run-disabled-tests <test_url>
```

- **--gtest_repeat** - See [Running test multiple times](#running_test_multiple_times).
- **--gtest_output** - Emitting gtest json output is not supported.
- **--gtest_list_tests** - Listing test cases is not supported.

#### GoogleTest (Gunit) runner {#gunit-runner-custom-arg}

Note the following known behavior change:

**--gunit_break_on_failure**: As each test case is executed in a different process,
this flag will not work.

The following flags are restricted and the test fails if any are passed as
fuchsia.test.Suite provides equivalent functionality that replaces them.

- **--gunit_filter** - Instead use:

```posix-terminal
 fx test --test-filter=<glob_pattern> <test_url>
```

`--test-filter` may be specified multiple times. Tests that match any of the
given glob patterns will be executed.

- **--gunit_also_run_disabled_tests** - Instead use:

```posix-terminal
 fx test --also-run-disabled-tests <test_url>
```

- **--gunit_repeat** - See [Running test multiple times](#running_test_multiple_times).
- **--gunit_output** - Emitting gtest json/xml output is not supported.
- **--gunit_list_tests** - Listing test cases is not supported.

#### Rust runner {#rust-runner-custom-arg}

The following flags are restricted and the test fails if any are passed as
fuchsia.test.Suite provides equivalent functionality that replaces them.

- **\<test_name_matcher\>** - Instead use:

```posix-terminal
 fx test --test-filter=<glob_pattern> <test_url>
```

`--test-filter` may be specified multiple times. Tests that match any of the
given glob patterns will be executed.

- **--nocapture** - Output is printed by default.
- **--list** - Listing test cases is not supported.

#### Go test runner {#gotest-runner-custom-arg}

Note the following known behavior change:

**-test.failfast**: As each test case is executed in a different process, this
flag will only influence sub-tests.

The following flags are restricted and the test fails if any are passed as
fuchsia.test.Suite provides equivalent functionality that replaces them

- **-test.run** - Instead use:

```posix-terminal
 fx test --test-filter=<glob_pattern> <test_url>
```

`--test-filter` may be specified multiple times. Tests that match any of the
given glob patterns will be executed.

- **-test.count** - See [Running test multiple times](#running_test_multiple_times).
- **-test.v** - Output is printed by default.
- **-test.parallel** - See [Controlling parallel execution of test cases](#controlling_parallel_execution_of_test_cases).

### A runtime-agnostic, runtime-inclusive testing framework {#inclusive}

Fuchsia aims to be inclusive, for instance in the sense that
developers can create components (and their tests) in their language and runtime
of choice. The Test Runner Framework itself is language-agnostic by design, with
individual test runners specializing in particular programming languages or test
runtimes and therefore being language-inclusive. Anyone can create and use new
test runners.

Creating new test runners is relatively easy, with the possibility of sharing
code between different runners. For instance, the GoogleTest runner and the Rust
runner share code related to launching an ELF binary, but differ in code for
passing command line arguments to the test and parsing the test's results.

## Temporary storage

To use temporary storage in your test, add the following to your component manifest:

```json5
{
    include: [ "//src/sys/test_runners/tmp_storage.shard.cml" ]
}
```

At runtime, your test will have read/write access to `/tmp`.
The contents of this directory will be empty when the test starts, and will be
deleted after the test finishes.

[Tests that don't specify a custom manifest][component-unit-tests] and instead
rely on the build system to generate their component manifest can add the
following dependency:

```gn
fuchsia_unittest_package("foo-tests") {
  deps = [
    ":foo_test",
    "//src/sys/test_runners:tmp_storage",
  ]
}
```

## Hermeticity

A test is *hermetic* if it:

1. Does not [use][manifests-use] or [offer][manifests-offer] any
capabilities from the [test root's](#tests-as-components) parent.
1. Does not [resolve][resolvers] any components outside of the test package.

The tests are by default hermetic unless explicitly stated otherwise.


### Hermetic capabilities for tests

There are some capabilities which all tests can use which do not violate test
hermeticity:

| Protocol | Description |
| -----------| ------------|
| `fuchsia.boot.WriteOnlyLog` | Write to kernel log |
| `fuchsia.logger.LogSink` | Write to syslog |
| `fuchsia.process.Launcher` | Launch a child process from the test package |
| `fuchsia.sys2.EventSource` | Access to event protocol |

The hermeticity is retained because these capabilities are carefully curated
to not allow tests to affect the behavior of system components outside the test
realm or of other tests.

To use these capabilities, there should be a use declaration added to test's
manifest file:

```json5
// my_test.cml
{
    use: [
        ...
        {
            protocol: [
              "{{ '<var label="protocol">fuchsia.logger.LogSink</var>' }}"
            ],
        },
    ],
}
```

Tests are also provided with some default storage capabilities which are
destroyed after the test finishes execution.

| Storage Capability | Description | Path |
| ------------------ | ----------- | ---- |
|  `data` | Isolated data storage directory | `/data` |
|  `cache` | Isolated cache storage directory | `/cache` |
|  `tmp` | Isolated in-memory [temporary storage directory](#temporary_storage) | `/tmp` |

Add a use declaration in test's manifest file to use these capabilities.

```json5
// my_test.cml
{
    use: [
        ...
        {
            storage: "{{ '<var label="storage">data</var>' }}",
            path: "{{ '<var label="storage path">/data</var>' }}",
        },
    ],
}
```

The framework also provides some [capabilities][framework-capabilities] to all
the components and can be used by test components if required.


### Hermetic component resolution {#hermetic-resolver}

Hermetic test components are launched in a realm that utilizes the hermetic
component resolver. This resolver disallows resolving URLs outside of the
test's package. This is necessary for enforcing hermeticity, as we don't
want the availability of an arbitrary component on the system or in an
associated package server to affect the outcome of a test.

Attempts to resolve a component not in the test's package will be met with a
`PackageNotFound` error and the following message in the syslog:

```
failed to resolve component fuchsia-pkg://fuchsia.com/[package_name]#meta/[component_name]: package [package_name] is not in the set of allowed packages...
```

You can avoid this error by including any components your test relies on
to the test package - see [this CL](https://fxrev.dev/608222) for an example of
how to do this.

### Tier 2 Hermetic tests

These kind of tests are hermetic with respect to capabilities (i.e they don't
have access to capabilities which can affect system state outside of the test),
but they are *allowed* to resolve URLs from outside the test package.

These kind of tests are useful when it is not trivial to package all dependent
components inside test's own package, for example when the component under test
has a deep hierarchy and it is not possible to package all dependent
components hermetically without re-writing corresponding manifest files.

*Whenever possible it is preferred to hermetically packages the test and its
dependencies. See [Hermetic component resolution](#hermetic-resolver).*

A test must explicitly mark itself to run as a **tier-2 hermetic** test.

```json5
// my_component_test.cml

{
    include: [
        // Select the appropriate test runner shard here:
        // rust, gtest, go, etc.
        "//src/sys/test_runners/rust/default.shard.cml",

        // This includes the facet which marks the test type as "hermetic-tier-2".
        {{ '<strong>' }}"sys/testing/hermetic-tier-2-test.shard.cml",{{ '</strong>' }}
    ],
    program: {
        binary: "bin/my_component_test",
    },
}
```

The shard includes following facet in the manifest file:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/lib/sys/testing/hermetic-tier-2-test.shard.cml" %}
```

### Legacy non-hermetic tests

These tests that were introduced before hermetic testing was enforced. They
could access some pre-defined capabilities outside of the test realm. A
capability accessed by non-hermetic test from outside its test realm is called
a *system capability*.

To use a system capability, a test must explicitly mark itself to run in
non-hermetic "system" realm as shown below.

```json5
// my_component_test.cml

{
    include: [
        // Select the appropriate test runner shard here:
        // rust, gtest, go, etc.
        "//src/sys/test_runners/rust/default.shard.cml",

        // This includes the facet which marks the test type as "system".
        {{ '<strong>' }}"sys/testing/system-test.shard.cml",{{ '</strong>' }}
    ],
    program: {
        binary: "bin/my_component_test",
    },
    {{ '<strong>' }}
    use: [
        {
            protocol: [ "fuchsia.sysmem.Allocator" ],
        },
    ],{{ '</strong>' }}
}
```

The shard includes following facet in the manifest file:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="src/sys/test_manager/system-test.shard.cml" %}
```

Possible values of `fuchsia.test.type`:

| Value | Description |
| ----- | ----------- |
| `hermetic` | Hermetic realm |
| `hermetic-tier-2` | Hermetic realm with non-hermetic resolver |
| `system` | Legacy non hermetic realm with access to some system capabilities. |
| `cts` | [CTS test realm] |

Below is the list of system capabilities provided to legacy non-hermetic tests:

{# Update the list when it is updated at
//src/sys/test_manager/meta/common.shard.cml#}
Protocols:

```text
fuchsia.boot.ReadOnlyLog
fuchsia.boot.RootResource
fuchsia.component.resolution.Resolver
fuchsia.exception.Handler
fuchsia.hwinfo.Board
fuchsia.hwinfo.Device
fuchsia.hwinfo.Product
fuchsia.kernel.Counter
fuchsia.kernel.CpuResource
fuchsia.kernel.DebugResource
fuchsia.kernel.HypervisorResource
fuchsia.kernel.InfoResource
fuchsia.kernel.IoportResource
fuchsia.kernel.IrqResource
fuchsia.kernel.MmioResource
fuchsia.kernel.PowerResource
fuchsia.kernel.RootJob
fuchsia.kernel.RootJobForInspect
fuchsia.kernel.SmcResource
fuchsia.kernel.Stats
fuchsia.kernel.VmexResource
fuchsia.net.http.Loader
fuchsia.scheduler.ProfileProvider
fuchsia.sysinfo.SysInfo
fuchsia.sysmem.Allocator
fuchsia.tracing.provider.Registry
fuchsia.vulkan.loader.Loader
```

Directories:

```text
root-ssl-certificates
config-data
dev-input-report
dev-display-controller
dev-goldfish-address-space
dev-goldfish-control
dev-goldfish-pipe
dev-goldfish-sync
dev-gpu
dev-gpu-performance-counters
dev-mediacodec
```

## Restricted logs

By default, a test will fail if it logs a message with a severity of
`ERROR` or higher. See this [guide][restricted-logs] for more information.

## Performance

When writing a test runner that launches processes, the runner needs to
provide a [library loader][loader-service] implementation.

Test runners typically launch individual test cases in separate processes to
achieve a greater degree of isolation between test cases. However this can come
at a significant performance cost. To mitigate this, the test runners listed
above use a [caching loader service][caching-loader-service] which reduces the
extra overhead per process launched.

## Test roles {#test-roles}

Components in the test realm may play various roles in the test, as follows:

-   Test driver: The component that actually runs the test, and implements
    (either directly or through a [test runner](#test-runners)) the
    [`fuchsia.test.Suite`][test-suite-protocol] protocol. This role may be, but
    is not necessarily, owned by the [test root](#tests-as-components).
-   Capability provider: A component that provides a capability that the test
    will exercise somehow. The component may either provide a "fake"
    implementation of the capability for test, or a "real" implementation that
    is equivalent to what production uses.
-   Component under test: A component that exercises some behavior to be tested.
    This may be identical to a component from production, or a component written
    specifically for the test intended to model production behavior.

## Troubleshooting {#troubleshooting}

This section contains common issues you may encounter while developing test components
with the Test Runner Framework. If one of your test components fails to run, you may see
an error like the following from `fx test`:

```none {:.devsite-disable-click-to-copy}
Test suite encountered error trying to run tests: getting test cases
Caused by:
    The test protocol was closed. This may mean `fuchsia.test.Suite` was not configured correctly.
```

To address the issue, explore the following options:

- [The test failed to expose `fuchsia.test.Suite` to test manager](#troubleshoot-test-root)
- [The test driver failed to expose `fuchsia.test.Suite` to the root](#troubleshoot-test-routing)
- [The test driver does not use a test runner](#troubleshoot-test-runner)

### The test failed to expose `fuchsia.test.Suite` to test manager {#troubleshoot-test-root}

This happens when the test root fails to expose `fuchsia.test.Suite` from the
[test root](#test-roles). The simple fix is to add an `expose` declaration:

```json5
// test_root.cml
expose: [
    ...
    {
        protocol: "fuchsia.test.Suite",
        from: "self",  // If a child component is the test driver, put `from: "#driver"`
    },
],
```

### The test driver failed to expose `fuchsia.test.Suite` to the root {#troubleshoot-test-routing}

Your test may fail with an error similar to the following if the `fuchsia.test.Suite`
protocol is not properly exposed:

```none {:.devsite-disable-click-to-copy}
ERROR: Failed to route protocol `/svc/fuchsia.test.Suite` from component
`/test_manager/...`: An `expose from #driver` declaration was found at `/test_manager/...`
for `/svc/fuchsia.test.Suite`, but no matching `expose` declaration was found in the child
```

If the [test driver and test root](#test-roles) are different components, the test driver
must also expose `fuchsia.test.Suite` to its parent, the test root.

To address this issue, ensure the [test driver](#test-roles) component manifest includes
the following `expose` declaration:

```json5
// test_driver.cml
expose: [
    ...
    {
        protocol: "fuchsia.test.Suite",
        from: "self",
    },
],
```

### The test driver does not use a test runner {#troubleshoot-test-runner}

The [test driver](#test-roles) must use the appropriate [test runner](#test-runners)
corresponding to the language and test framework the test is written with.
For example, the driver of a Rust test needs the following declaration:

```json5
// test_driver.cml
include: [ "//src/sys/test_runners/rust/default.shard.cml" ]
```

Also, if the test driver is a child of the [test root](#test-roles), you need
to offer it to the driver:

```json5
// test_root.cml
offer: [
    {
        runner: "rust_test_runner",
        to: [ "#driver" ],
    },
],
```

## Further reading

- [Complex topologies and integration testing][integration-testing]: testing
  interactions between multiple components in isolation from the rest of the
  system.

[cf]: /concepts/components/v2/
[component-manifest]: /concepts/components/v2/component_manifests.md
[component-unit-tests]: /development/components/build.md#unit-tests
[CTS test realm]: /development/testing/cts/test_realm.md
[fidl-test-manager]: /sdk/fidl/fuchsia.test.manager/test_manager.fidl
[fidl-test-suite]: /sdk/fidl/fuchsia.test/suite.fidl
[ffx]: /development/tools/ffx/overview.md
[fx-test]: https://fuchsia.dev/reference/tools/fx/cmd/test
[integration-testing]: /development/testing/components/integration_testing.md
[manifests-offer]: https://fuchsia.dev/reference/cml#offer
[manifests-use]: https://fuchsia.dev/reference/cml#use
[resolvers]:  /concepts/components/v2/capabilities/resolvers.md
[restricted-logs]: /development/diagnostics/test_and_logs.md#restricting_log_severity
[runners]: /concepts/components/v2/capabilities/runners.md
[test-suite-protocol]: /concepts/components/v2/realms.md
[unit-tests]: /development/components/build.md#unit_tests_with_generated_manifests
[loader-service]: /concepts/process/program_loading.md#the_loader_service
[caching-loader-service]: /src/sys/test_runners/src/elf/elf_component.rs
[framework-capabilities]: /concepts/components/v2/capabilities/protocol.md#framework
[sys-migration-guide]: /development/components/v2/migration/tests.md
