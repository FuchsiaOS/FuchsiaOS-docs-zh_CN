# Component tests

<<../../../_common/components/_tests_intro.md>>

## Test runners

Test runners are reusable adapters between the Test Runner Framework and common
frameworks used by developers to write tests in their preferred language.
Each test runner component exposes the `fuchsia.test.Suite` capability that
enables the `test_manager` to enumerate and execute individual tests, and
declares the appropriate execution `runner` with test framework support.

```json5
{
    // Execute tests using language-specific runner
    program: { runner: "elf_test_runner", },
    // Expose test suite protocol to test manager
    capabilities: [
        { protocol: "fuchsia.test.Suite" },
    ],
    expose: [
        {
            protocol: "fuchsia.test.Suite",
            from: "self",
        },
    ],
}
```

To simplify integration, the Test Runner Framework provides **manifest shards**
for each language-specific runner. The following is an equivalent test runner
CML for declaring the capabilities from the previous example component tests.

```json5
{
    include: [ "sys/testing/elf_test_runner.shard.cml" ]
}
```

Note: For complete details on the Test Runner Framework and component testing,
see the
[testing documentation](/docs/development/testing/components/test_runner_framework.md).

## Unit tests

[Unit testing](https://en.wikipedia.org/wiki/Unit_testing){:.external} focuses
on validating the individual units of code within your component and isolated
from other components on the system. Unit tests should be **hermetic**, meaning
that they do not require or provide additional capabilities outside of the test.

The Fuchsia SDK provides additional templates to facilitate the creation of
unit test components:

* `fuchsia_cc_test()`: Compiles the C++ source code into a test binary. When
  added to a package, this rule also generates a minimal component manifest that
  references the test binary and requires no additional capabilities.
* `fuchsia_test_package()`: Generates a Fuchsia package containing one or more
  test components and their dependencies.

Below is an example `BUILD.bazel` snippet for including unit tests:

```bazel
load(
    "fuchsia_cc_test",
    "fuchsia_test_package",
    "if_fuchsia",
)

fuchsia_cc_test(
    name = "hello_world_test",
    srcs = ["hello_world_test.cc"],
    deps = if_fuchsia([
        "@fuchsia_sdk//pkg/fdio",
        "@fuchsia_sdk//pkg/syslog",
    ]),
)

fuchsia_test_package(
    name = "unit_test_pkg",
    visibility = ["//visibility:public"],
    deps = [
      ":hello_world_test",
    ],
)
```

## Exercise: Echo unit tests

In this exercise, you'll add unit tests to the `echo` component with the
Test Runner Framework and run those tests in a FEMU environment.

After you complete this section, the project should have the following directory
structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/echo
                  |- BUILD.bazel
                  |- meta
                  |   |- echo.cml
                  |
                  |- echo_component.cc
                  |- echo_component.h
{{ '<strong>' }}                  |- echo_unittest.cc {{ '</strong>' }}
                  |- main.cc
```

* `echo_unittest.cc`: Source code for the C++ unit tests.

### Implement unit tests

Unit tests verify that the internal functions of the component behave as
expected. For the `echo` component, you'll validate that the `greeting()`
function used in the previous exercise returns the expected values.

Create `echo/echo_unittest.cc` and add the following unit test functions to
validate the behavior of the `greeting()` function when supplied with one, two,
or three arguments:

`echo/echo_unittest.cc`:

```
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/echo/echo_unittest.cc" region_tag="imports" adjust_indentation="auto" %}

#include "echo_component.h"

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/echo/echo_unittest.cc" region_tag="test_mod" adjust_indentation="auto" %}
```

### Run the unit tests

Update the imports section of your `echo/BUILD.bazel` file to include the
additional test rules:

`echo/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/echo/BUILD.bazel" region_tag="imports" adjust_indentation="auto" %}
```

Add the following build rules to include your tests in the build configuration:

`echo/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/echo/BUILD.bazel" region_tag="unittest" adjust_indentation="auto" %}
```

This rule packages your unit tests into a component with the following URL:

```none {:.devsite-disable-click-to-copy}
fuchsia-pkg://fuchsiasamples.com/echo_unittests#meta/echo_unittests.cm
```

Build and publish the test package to the `fuchsiasamples.com` repository:

```posix-terminal
bazel run //fuchsia-codelab/echo:test_pkg.publish -- \
    --repo_name fuchsiasamples.com
```

Use the `ffx test` command to execute the unit tests inside this package.
Verify that the tests pass:

```posix-terminal
ffx test run \
    fuchsia-pkg://fuchsiasamples.com/echo_unittests#meta/echo_unittests.cm
```
