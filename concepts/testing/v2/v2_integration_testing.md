# Complex topologies and integration testing

<<../../components/_v2_banner.md>>

Integration testing scenarios involve two or more components operating in the
same realm and exchanging capabilities. While the majority of tests are unit
tests that span only a single component, integration testing scenarios call for
defining realm topologies and capability routes.

## The "driver" pattern for v2 component tests

We demonstrate the driver pattern for writing an integration test with a custom
component topology.

### Build definition

We define the `BUILD.gn` file as shown below:

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/basic/integration_tests/BUILD.gn" region_tag="example_snippet" adjust_indentation="auto" %}
```

### Component topology

The topology for the test realm in this example looks as follows:

<br>![Test driver topology](images/hello_world_topology.png)<br>

In this example the test package `hello-world-integration-test` contains four
components:

- **hello-world-integration-test-component** - Main entry point
- **hello-world** - Component under test
- **hello-world-integration-test-driver** - Test driver
- **archivist-for-embedding** - Helper component that provides services to
  other components.

`hello-world-integration-test-component` has two children:

- **hello-world-integration-test-driver**
- **archivist-for-embedding**

This is a simple component realm that launches
`hello-world-integration-test-driver` and offers it the helper services.

Finally, note that all components under test are included in the test's own
package. This promotes hermeticity and has many benefits. For instance it's
possible to push an updated version of the same package to the device and run
the test again without worrying whether the different components are all of the
same version or have fallen out of sync.

### Test Runner Framework integration

Note that the driver component exposes `fuchsia.test.Suite` from its child to
the test root. The root of a test realm must always expose this protocol in
order to integrate with the [Test Runner Framework][trf].

The root realm is defined as follows:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/basic/integration_tests/meta/hello-world-integration-test.cml" region_tag="example_snippet" adjust_indentation="auto" %}
```

`hello-world-integration-test-driver` contains the test logic and expectations.
The driver launches the `hello-world` component and asserts that it is writing
the expected strings to the log.

Note that this is a Rust test, and therefore includes `rust/default.shard.cml`
as required to integrate with the [Test Runner Framework][trf].

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/basic/integration_tests/meta/hello-world-integration-test-driver.cml" region_tag="example_snippet" adjust_indentation="auto" %}
```

### Further study

The code for this example can be found under
[`//examples/components/basic/integration_tests`][driver-pattern-example].

[driver-pattern-example]: /examples/components/basic/integration_tests/
[trf]: test_runner_framework.md
