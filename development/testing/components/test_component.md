# Test Components

## Introduction

Test components are [components][glossary.component] that implement a test.
Tests run in a given environment, and then report whether they passed or failed.
Typically tests are written using various testing frameworks, and may report
more detailed results such as whether individual test cases within a test suite
passed or failed.

The Component Framework allows launching tests as components. Most tests are
comprised of a single component - these are typically referred to as unit tests.
Some tests exercise multiple components working together - these are typically
referred to as integration tests.

## Creating a test component and package

A test package may contain one or more test components.
Test components are components that implement a test suite.
Test packages can also contain other components that are not the test itself,
but participate in the test. For instance:

- A package may contain a single test component that implements a unit test
  that exercises some business logic.
- A package may contain a test component and a second component that implements
  a service. The test component may then act as a client of the second
  component, which makes for an integration test between client and server code.
  Both the clients and server are located in the same package in order to ensure
  that the second component is present and can be launched by the test
  component.

In order to define your test package and components, you should use the
appropriate build rules. Refer to the [test packages][test-packages] guide.

## Test component manifest

Every component has a [manifest][component-manifest]. Test components follow the
same manifest syntax as any other components.

A component manifest for a simple unit test might be named `meta/my_test.cml`
and look as follows:

```json5
{
    include: [
        "syslog/client.shard.cml",
        "//src/sys/test_runners/gtest/default.shard.cml"
    ]
    program: {
        binary: "bin/my_test"
    }
}
```

Component manifests for simple unit tests can be [generated][unit-tests]
by the build rules.

## Running your test

To run a Fuchsia test out of your build, execute:

<pre class="prettyprint">
<code class="devsite-terminal">fx test <var>TEST_NAME</var></code>
</pre>

For more information, see [Run Fuchsia tests][executing-tests].

## Further reading

- [Test Runner Framework][trf]: writing idiomatic tests in different languages
  and using common testing frameworks.
- [Complex topologies and integration testing][integration-testing]: testing
  interactions between multiple components in isolation from the rest of the
  system.

[component-manifest]: concepts/components/v2/component_manifests.md
[glossary.component]: glossary/README.md#component
[executing-tests]: development/testing/run_fuchsia_tests.md
[integration-testing]: integration_testing.md
[test-packages]: development/components/build.md#test-packages
[trf]: test_runner_framework.md
[unit-tests]: development/components/build.md#unit-tests
