# Integration testing

<<../../_common/fidl/_testing_intro.md>>

## Test components

Below is an example component manifest for a simple integration test component:

* {Rust}

  `meta/integration_tests.cml`:

  ```json5
  {
      include: [
          "syslog/client.shard.cml",
          "//src/sys/test_runners/rust/default.shard.cml",
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

* {C++}

  `meta/integration_tests.cml`:

  ```json5
  {
      include: [
          "syslog/client.shard.cml",
          "//src/sys/test_runners/gtest/default.shard.cml",
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

1.  An `include`  of the necessary language-specific test runner shard. This
    enables the test manager to properly execute the test suite.
1.  Listing the component under test and dependent components as `children`.
1.  Routing required capabilities between components in the test realm.

The Fuchsia build system provides the `fuchsia_test_package()` GN target for
distinct test components such as integration tests. This rule enables you to
declare the components containing tests separately from those required as
dependencies, and describes the target device environment where the tests
should run.

Here is an example of how the above integration test could be included in the
`BUILD.gn` file:

```gn
import("//build/components.gni")
...

// Component under test
fuchsia_component("foo_client") {
  deps = [ ... ]
  manifest = "meta/foo_client.cml"
}

// Test dependencies
fuchsia_component("mock_service") {
  deps = [ ... ]
  manifest = "meta/mock_service.cml"
  testonly = true
}

// Component containing integration tests
fuchsia_component("integration_tests") {
  deps = [ ":bin_test" ]
  manifest = "meta/integration_tests.cml"
  testonly = true
}

fuchsia_test_package("hello-world-tests") {
  test_components = [ ":integration_tests" ]
  deps = [
    ":foo_client",
    ":mock_service",
  ]
}
```

<aside class="key-point">
Test components and other targets used exclusively in tests should set the
<code>testonly</code> attribute to <code>true</code>.
</aside>

## Exercise: Echo server integration test

In this exercise, you'll add an integration test component to exercise the FIDL
protocol interface of the `echo_server` component with the Test Runner
Framework and run those tests in a FEMU environment.

### Add an integration test component

To begin, create a project scaffold for a new integration test component called
`echo-integration` in the `//vendor/fuchsia-codelab` directory:

```posix-terminal
mkdir -p vendor/fuchsia-codelab/echo-integration
```

Create the following file and directory structure in the new project directory:

* {Rust}

  ```none {:.devsite-disable-click-to-copy}
  echo-integration
    |- BUILD.gn
    |- meta
    |   |- echo_integration.cml
    |
    |- src
        |- lib.rs
  ```

  * `BUILD.gn`: GN build targets for the test binaries, component, and package.
  * `meta/echo_integration.cml`: Manifest declaring the components under test
    and their capabilities.
  * `src/lib.rs`: Source code for the Rust integration tests.

* {C++}

  ```none {:.devsite-disable-click-to-copy}
  echo-integration
    |- BUILD.gn
    |- meta
    |   |- echo_integration.cml
    |
    |- echo_integration_test.cc
  ```

  * `BUILD.gn`: GN build targets for the test binaries, component, and package.
  * `meta/echo_integration.cml`: Manifest declaring the components under test
    and their capabilities.
  * `echo_integration_test.cc`: Source code for the C++ integration tests.

### Update the test component manifest

The generated manifest for the test component applies the baseline
dependencies, such as the Rust test runner. Update the `echo_integration.cml`
file to declare the `echo-server` component as a child and route the `Echo`
protocol capability to the test component.

{% set cml_rust %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/rust/meta/echo_integration_test.cml" region_tag="example_snippet" adjust_indentation="auto" %}
{% endset %}

{% set cml_cpp %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/cpp/meta/echo_integration_test.cml" region_tag="example_snippet" adjust_indentation="auto" %}
{% endset %}

* {Rust}

  `echo-integration/meta/echo_integration.cml`:

  ```json5
  {{ cml_rust|replace("echo_integration_test_rust","echo_integration_test")|trim() }}
  ```

* {C++}

  `echo-integration/meta/echo_integration.cml`:

  ```json5
  {{ cml_cpp|replace("echo_integration_test_cpp","echo_integration_test")|trim() }}
  ```

Notice that the `echo-server` instance comes from the same package as the
integration test. This practice promotes test packages that are **hermetic** by
avoiding dependencies on components from other packages.

### Implement the integration test

The integration test connects to the `Echo` protocol exposed by the
`echo-server` in the same way as the client component, sends a string request,
and validates the expected response.

Add the following code to implement an integration test:

* {Rust}

  `echo-integration/src/lib.rs`:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/rust/src/lib.rs" region_tag="example_snippet" adjust_indentation="auto" %}
  ```

  <aside class="key-point">
  The <code>fuchsia::test</code> attribute removes some common boilerplate for
  component tests in Rust, such as initializing logging for each test case.
  </aside>

* {C++}

  `echo-integration/echo_integration_test.cc`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/cpp/echo_integration_test.cc" region_tag="example_snippet" adjust_indentation="auto" %}
  ```

Add the following build rules to your `BUILD.gn` file to build and package the integration test
component along with the echo server dependencies:

{% set gn_rust_binary %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/rust/BUILD.gn" region_tag="executable" adjust_indentation="auto" %}
{% endset %}

{% set gn_rust_component %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/rust/BUILD.gn" region_tag="component" adjust_indentation="auto" %}
{% endset %}

{% set gn_cpp_binary %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/cpp/BUILD.gn" region_tag="executable" adjust_indentation="auto" %}
{% endset %}

{% set gn_cpp_component %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/cpp/BUILD.gn" region_tag="component" adjust_indentation="auto" %}
{% endset %}

* {Rust}

  `echo-integration/BUILD.gn`:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/rust/BUILD.gn" region_tag="imports" adjust_indentation="auto" %}

  {{ gn_rust_binary|replace("echo_integration_test_rust","echo-integration-test")|replace("//examples/components/routing/fidl","//vendor/fuchsia-codelab/echo-fidl")|trim() }}

  {{ gn_rust_component|replace("echo_integration_test_component","component")|replace("echo_integration_test","echo_integration")|trim() }}

  fuchsia_test_package("tests") {
    package_name = "echo-integration-tests"
    test_components = [ ":component" ]
    deps =
        [ "//vendor/fuchsia-codelab/echo-server:component" ]
  }
  ```

* {C++}

  `echo-integration/BUILD.gn`:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/cpp/BUILD.gn" region_tag="imports" adjust_indentation="auto" %}

  {{ gn_cpp_binary|replace("echo_integration_test_cpp","echo-integration-test")|replace("//examples/components/routing/fidl","//vendor/fuchsia-codelab/echo-fidl")|trim() }}

  {{ gn_cpp_component|replace("echo_integration_test_component","component")|replace("echo_integration_test","echo_integration")|trim() }}

  fuchsia_test_package("tests") {
    package_name = "echo-integration-tests"
    test_components = [ ":component" ]
    deps =
        [ "//vendor/fuchsia-codelab/echo-server:component" ]
  }
  ```

### Update the build configuration

Add the integration test package to the build configuration:

```posix-terminal
fx set workstation_eng.qemu-x64 \
    --with //vendor/fuchsia-codelab/echo-server \
    --with //vendor/fuchsia-codelab/echo-client \
    --with //vendor/fuchsia-codelab/echo-realm \
    --with //vendor/fuchsia-codelab/echo-integration:tests
```

Run `fx build` and verify that the build completes successfully:

```posix-terminal
fx build
```

### Run the integration test

The `fuchsia_test_package()` rule generates a package with the test component
and its dependencies. The integration test component has the following URL:

```none {:.devsite-disable-click-to-copy}
fuchsia-pkg://fuchsia.com/echo-integration-tests#meta/echo_integration.cm
```

Use the `ffx test` command to execute the integration tests. Verify that the
tests pass:

```posix-terminal
ffx test run \
    fuchsia-pkg://fuchsia.com/echo-integration-tests#meta/echo_integration.cm
```
