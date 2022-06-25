# Integration testing

<<../../../_common/fidl/_testing_intro.md>>

## Test components

Below is an example component manifest for a simple integration test component:

`meta/integration_tests.cml`:

```json5
{
    include: [
        "//pkg/syslog/client.shard.cml",
        "//pkg/sys/testing/elf_test_runner.shard.cml",
    ],
    program: {
        binary: "bin/client_test",
    },
    children: [
        {
            name: "service",
            url: "fuchsia-pkg://fuchsia.com/foo-package-tests#meta/mock_service.cm",
        },
        {
            name: "client",
            url: "fuchsia-pkg://fuchsia.com/foo-package-tests#meta/foo_client.cm",
        },
    ],
    offer: [
        {
            protocol: "fuchsia.example.Foo",
            from: "#service",
            to: [ "#client" ],
        },
    ],
}
```

This test component declaration contains the following key elements:

1.  An `include` of the necessary language-specific test runner shard. This
    enables the test manager to properly execute the test suite.
1.  Listing the component under test and dependent components as `children`.
1.  Routing required capabilities between components in the test realm.

The Fuchsia SDK provides additional templates to facilitate the creation of
integration test components:

* `fuchsia_cc_test()`: Compiles the C++ source code into a test binary.
* `fuchsia_test_component()`: Generates a Fuchsia component containing tests
  using the provided component manifest. You can combine multiple test components
  into the same `fuchsia_test_package()`.

Here is an example of how the above integration test could be included in the
`BUILD.bazel` file:

```bazel
load(
    "fuchsia_cc_test",
    "fuchsia_component",
    "fuchsia_component_manifest",
    "fuchsia_test_component",
    "fuchsia_test_package",
)

// Component under test
fuchsia_component(
    name = "foo_client",
    manifest = ":foo_client_manifest",
    visibility = ["//visibility:public"],
)

// Test dependencies
fuchsia_component(
    name = "mock_service",
    manifest = ":mock_service_manifest",
    visibility = ["//visibility:public"],
)

// Component containing integration tests
fuchsia_cc_test(
    name = "client_integration_test",
    srcs = [ ... ],
    depc = [ ... ],
)
fuchsia_component_manifest(
    name = "integration_test_manifest",
    src = "meta/integration_tests.cml",
)
fuchsia_test_component(
    name = "integration_test_component",
    manifest = ":integration_test_manifest",
    test_name = "client_integration_test",
    visibility = ["//visibility:public"],
    deps = [":client_integration_test"],
)

// Hermetic test package
fuchsia_test_package(
    name = "integration_test_pkg",
    visibility = ["//visibility:public"],
    deps = [
        ":foo_client",
        ":mock_service",
        ":integration_test_component",
    ],
)
```

This integration test build configuration contains the following key elements:

1.  A `fuchsia_test_component()` target describing the integration test component
    and its component manifest.
1.  Additional `fuchsia_component()` targets representing component dependencies
    requited by the integration tests.
1.  A single hermetic `fuchsia_test_package()` that bundles the test component
    and all dependencies together.

## Exercise: Echo server integration test

In this exercise, you'll add an integration test component to exercise the FIDL
protocol interface of the `echo_server` component with the Test Runner
Framework and run those tests in a FEMU environment.

### Add an integration test component

To begin, create a new project directory in your Bazel workspace:

```posix-terminal
mkdir -p fuchsia-codelab/echo-integration
```

This component project should have the following directory structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/echo-integration
                  |- BUILD.bazel
                  |- meta
                  |   |- echo_integration_test.cml
                  |
                  |- echo_integration_test.cc
```

### Update the test component manifest

Update the test component manifest to declare the `echo-server` component as a
child and route the `Echo` protocol capability to the test component.

`echo-integration/meta/echo_integration_test.cml`:

```json5
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/integration_tests/meta/echo_integration_test.cml" region_tag="example_snippet" adjust_indentation="auto" %}
```

Notice that the `echo-server` instance comes from the same package as the
integration test. This practice promotes test packages that are **hermetic** by
avoiding dependencies on components from other packages.

Add the following rules to your `BUILD.bazel` file to include the integration
test component in the build configuration:

`echo-integration/BUILD.bazel`:

```bazel
load(
    "@rules_fuchsia//fuchsia:defs.bzl",
    "fuchsia_cc_test",
    "fuchsia_component_manifest",
    "fuchsia_test_component",
    "fuchsia_test_package",
)

fuchsia_cc_test(
    name = "echo_integration_test",
    size = "small",
    srcs = ["echo_integration_test.cc"],
    deps = [
        "//fuchsia-codelab/echo-fidl:fidl.examples.routing.echo.fidl_cc",
        "@com_google_googletest//:gtest_main",
        "@fuchsia_sdk//pkg/async-default",
        "@fuchsia_sdk//pkg/async-loop",
        "@fuchsia_sdk//pkg/async-loop-cpp",
        "@fuchsia_sdk//pkg/async-loop-default",
        "@fuchsia_sdk//pkg/fdio",
        "@fuchsia_sdk//pkg/sys_cpp",
        "@fuchsia_sdk//pkg/syslog",
    ],
)

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/integration_tests/BUILD.bazel" region_tag="component" adjust_indentation="auto" %}

fuchsia_test_package(
    name = "test_pkg",
    package_name = "echo_integration_test",
    visibility = ["//visibility:public"],
    deps = [
        ":echo_integration_test_component",
        "//fuchsia-codelab/echo-server:echo_server_component",
    ],
)
```

### Implement the integration test

The integration test connects to the `Echo` protocol exposed by the
`echo-server` in the same way as the client component, sends a string request,
and validates the expected response.

Add the following code to implement an integration test:

`echo-integration/echo_integration_test.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/integration_tests/echo_integration_test.cc" region_tag="example_snippet" adjust_indentation="auto" %}
```

### Update the build configuration

Run `bazel build` and verify that the build completes successfully:

```posix-terminal
bazel build --config=fuchsia_x64 //fuchsia-codelab/echo-integration:test_pkg \
     --publish_to=$HOME/.package_repos/sdk-samples
```

### Run the integration test

The `fuchsia_test_package()` rule generates a package with the test component
and its dependencies. The integration test component has the following URL:

```none {:.devsite-disable-click-to-copy}
fuchsia-pkg://fuchsiasamples.com/echo_integration_test#meta/echo_integration_test.cm
```

Use the `ffx test` command to execute the integration tests. Verify that the
tests pass:

```posix-terminal
ffx test run \
    fuchsia-pkg://fuchsiasamples.com/echo_integration_test#meta/echo_integration_test.cm
```
