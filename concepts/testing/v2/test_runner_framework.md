# The Fuchsia Test Runner Framework

<<../../components/_v2_banner.md>>

## Integrating testing frameworks with the Component Framework

The Fuchsia [Component Framework][cf] allows developers to create components in
a variety of languages and runtimes. Fuchsia's own code uses a diverse mix of
programming languages for components, including C/C++, Rust, Dart, and Go.

The Test Runner Framework uses Component Framework [runners][runners] as an
integration layer between various testing runtimes and a common Fuchsia protocol
for launching tests and receiving their results. This makes for an [inclusive]
design that on one hand allows developers to bring their language and testing
framework of choice, and on the other hand allows building and testing Fuchsia
on a variety of systems and targeting different hardware.

## The Test Manager

The `test_manager` component is responsible for running tests on a Fuchsia
device. Test manager exposes the
[`fuchsia.test.manager.Harness`][fidl-test-manager] protocol, which allows
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
    include: [ "src/sys/test_runners/gtest/default.shard.cml" ]
}
```

By default GoogleTest test cases run serially (one test case at a time).

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
    include: [ "src/sys/test_runners/rust/default.shard.cml" ]
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
    include: [ "src/sys/test_runners/gotests/default.shard.cml" ]
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
    include: [ "src/sys/test_runners/elf/default.shard.cml" ]
}
```

There is no notion of parallelism since tests that use this runner don't have a
notion of multiple test cases.

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

If using `run-test-suite` to launch tests, pass this as an argument:

```posix-terminal
fx shell run-test-suite --parallel=5 <test_url>
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
 fx test --test-filter=<my_filter> <test_url>
```

- **--gtest_also_run_disabled_tests** - Instead use:

```posix-terminal
 fx test --also-run-disabled-tests <test_url>
```

- **--gtest_repeat** - See [Running test multiple times](#running_test_multiple_times).
- **--gtest_output** - Emitting gtest json output is not supported.
- **--gtest_list_tests** - Listing test cases is not supported.

#### Rust runner {#rust-runner-custom-arg}

The following flags are restricted and the test fails if any are passed as
fuchsia.test.Suite provides equivalent functionality that replaces them.

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
 fx test --test-filter=<my_filter> <test_url>
```

- **-test.count** - See [Running test multiple times](#running_test_multiple_times).
- **-test.v** - Output is printed by default.
- **-test.parallel** - See [Controlling parallel execution of test cases](#controlling_parallel_execution_of_test_cases).

## Temporary storage

To use temporary storage in your test, add the following to your component manifest:

```json5
{
    include: [ "src/sys/test_runners/tmp_storage.shard.cml" ]
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

A test is *hermetic* if it [uses][manifests-use] or [offers][manifests-offer] no
capabilities from the [test root](#tests-as-components)'s parent. As a rule of
thumb, tests should be hermetic, but sometimes a test requires a capability that
cannot be injected in the test realm.

In the context of hermetic tests, a capability that originates from outside of
the test's realm is called a *system capability*.

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

## Further reading

- [Complex topologies and integration testing][integration-testing]: testing
  interactions between multiple components in isolation from the rest of the
  system.

[cf]: /docs/concepts/components/v2/
[component-manifest]: /docs/concepts/components/v2/component_manifests.md
[component-unit-tests]: /docs/development/components/build.md#unit-tests
[fidl-test-manager]: /sdk/fidl/fuchsia.test.manager/test_manager.fidl
[fidl-test-suite]: /sdk/fidl/fuchsia.test/suite.fidl
[ffx]: /docs/development/tools/ffx/overview.md
[fx-test]: https://fuchsia.dev/reference/tools/fx/cmd/test
[inclusive]: /docs/concepts/principles/inclusive.md
[integration-testing]: /docs/concepts/testing/v2/v2_integration_testing.md
[manifests-offer]: /docs/concepts/components/v2/component_manifests.md#offer
[manifests-use]: /docs/concepts/components/v2/component_manifests.md#use
[runners]: /docs/concepts/components/v2/capabilities/runners.md
[test-suite-protocol]: /docs/concepts/components/v2/realms.md
[unit-tests]: /docs/development/components/build.md#unit_tests_with_generated_manifests
