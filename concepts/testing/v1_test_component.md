# Test components (Components v1)

<<../components/_v1_banner.md>>

Note: see [Testing with Components][testing-v2] for modern (`.cml`) component
testing.

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

- A package may contain a single test component, which implements a unit test
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

Every component has a [manifest][component-manifest]. Test components may have
manifests that are similar to regular components, or they may use additional
special syntax for testing that's covered below.

A component manifest for a simple unit test might be named `meta/my_test.cmx`
and look as follows:

```json
{
    "include": [ "syslog/client.shard.cmx" ],
    "program": {
        "binary": "bin/my_test"
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

## Isolated Storage

By default, the test component is launched in a new hermetic environment,
isolated from system services and system storage. This keeps the rest of the
system from interfering with the results of your test, and vice versa.

Test environments have unique generated names in the form `test_env_XXXXXXXX`,
where the placeholder is 8 random hexadecimal digits.

Test components may use persistent storage as usual, by specifying
`"isolated-persistent-storage"` under `sandbox.features` in their manifest.
However unlike regular components, the test's storage directory will be cleared
before the test component is launched and after it terminates.

This is usually the desired behavior since it keeps test runs from interfering
with each other via side effects. However, if you need to retain a test's
storage for troubleshooting, use [run-test-component][run-test-component] in the
Fuchsia shell and pass the `--realm-label` flag followed by a name for your test
environment.

The `--realm-label` flag defines the label for an environment that your test
runs in. When the test ends, the storage won't be deleted automatically.
Instead it'll be accessible at a path under `/data`. The path will take the
following form:

```
/data/r/sys/r/<realm-label>/fuchsia.com:<package-name>:0#meta:<cmx>/</file>
```

For instance:
```
/data/r/sys/r/foo/fuchsia.com:mypackage:0#meta:myurl.cmx/bar
```

You can copy the files from the target device using `fx scp`. Afterwards,
consider deleting the storage directory.

Example commands

Using fx:

```posix-terminal
fx test my_test_url.cmx --realm my_realm
```

On-device:

```posix-terminal
run-test-component --realm-label=my_realm my_test_url.cmx

```

**Note**: `run-test-component` will be deprecated in a future release, prefer
`fx test` when remove device access is available.

## Services

### Basic system services

By default, test components may only access a small subset of system services,
in order to promote hermeticity. These system services can be used in a test
component by specifying them in the test manifest under `sandbox.services` as
usual.

```
fuchsia.logger.LogSink
fuchsia.process.Launcher
fuchsia.process.Resolver
fuchsia.sys.Environment
fuchsia.sys.Launcher
fuchsia.sys.Loader
```

### Integration testing

A test component may need to interact with other components, such as in an
integration test. One way to do this is to include all components under test in
the test's package, and then specify in the test's manifest a mapping between
the services that these components offer and their launch URLs.

This is done as follows:

```json
"facets": {
  "fuchsia.test": {
    "injected-services": {
        "service_name1": "component_url1",
        "service_name2": "component_url2"
    }
  }
}
```

However, note that *all the test executions* will run in the *same environment*.
If a service had dirtied state, a subsequent `TEST_F` execution will
inadvertently run against that dirtied state.

See [this doc](/src/ui/tests/README.md) for authoring more sophisticated
scenarios (such as graphics and UI tests) in v1.

### Additional system services

Tests may request access to additional system services, at the expense of their
own hermeticity (as they become subject to elements of the system outside of the
test's scope).

This is done as follows:

```json
"facets": {
  "fuchsia.test": {
    "system-services": [
        "service_name1",
        "service_name2"
    ]
  }
}
```

Real system services cannot be accessed by test components unless explicitly
allowlisted as shown below:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="src/sys/run_test_component/test_metadata.cc" region_tag="allowed_system_services" adjust_indentation="auto" %}
```

[glossary.component]: /glossary/README.md#component
[component-manifest]: /concepts/components/v1/component_manifests.md
[executing-tests]: /development/testing/run_fuchsia_tests.md
[run-test-component]: /development/testing/run_fuchsia_tests.md
[test-packages]: /development/components/build.md#test-packages
[testing-v2]: /development/testing/components/README.md
[unit-tests]: /development/components/build.md#unit-tests
