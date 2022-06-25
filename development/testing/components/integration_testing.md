# Integration testing topologies

Integration testing scenarios involve two or more components operating in the
same realm and exchanging capabilities. While the majority of tests are unit
tests that span only a single component, integration testing scenarios call for
defining realm topologies and capability routes.

In cases where all components in the test are static, you can define the
topology for the test realm in the test component manifest.
In cases where realm topology needs to be defined at runtime, or components
need to be mocked out, [Realm Builder][realm-builder] should be used.

Note: The example code referenced in this guide can be found under
[`//examples/components/routing/integration_tests`][driver-pattern-example].

## Component topology

The integration test component declares the topology of the test realm with
itself as the parent. This allows the test controller to be responsible for
capability routing between components under test and their dependencies.

The following is an example topology for integration testing the `echo_server`
component:

<br>![Integration test topology](images/echo-test-topology.png)<br>

This is a simple test realm that binds to the `fidl.examples.routing.echo.Echo`
protocol exposed by the `echo_server` component.
The `echo_integration_test` package contains the following components:

- **echo_integration_test** - Test controller
- **echo_server** - Component under test

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/rust/meta/echo_integration_test.cml" region_tag="example_snippet" adjust_indentation="auto" %}
```

The test controller component contains the test case logic, and interacts with
the `echo_server` through its exposed capabilities.

### Test Runner Framework integration

The integration test component includes `test_runners/rust/default.shard.cml`
in order to integrate with the [Test Runner Framework][trf]. This shard provides
two key elements:

1.  Expose the `fuchsia.test.Suite` protocol, required for the framework to
    discover and execute the test cases.
1.  Set the program `runner` to the Rust test runner, required to execute test
    cases using the Rust testing framework.

## Build definition

See the following `BUILD.gn` file that defines the `fuchsia_test_package()`
target for this example:

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/integration_tests/rust/BUILD.gn" region_tag="example_snippet" adjust_indentation="auto" %}
```

All components under test are included in the same **hermetic test package**.
This promotes the ability to run and update tests in different environments
without concern for dependencies falling out of sync.

## Determining a component's moniker {#test-component-moniker}

The `ArchiveReader` library allows your test to validate data in Inspect. The
moniker for a component is always relative to the root realm. For a component
running in a test, the moniker is therefore the `name` field defined in the
test root. In the example above, the moniker for the `echo_server` is simply
`echo_server`.

See the [Inspect Codelab][inspect-codelab] for detailed information on using
`ArchiveReader`.

[driver-pattern-example]: /examples/components/routing/integration_tests/
[inspect-codelab]: development/diagnostics/inspect/codelab/codelab.md
[trf]: test_runner_framework.md
[realm-builder]: development/testing/components/realm_builder.md
