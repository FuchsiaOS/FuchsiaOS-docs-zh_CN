# Run Fuchsia tests

This guide provides instructions on how to run Fuchsia tests using the `fx test`
command.

In Fuchsia, a test can be a component that runs on a Fuchsia device (see
[Tests as components][tests-as-components]) or a standalone executable that runs
on the host machine.

To run a Fuchsia test, use the `fx test` command with the name of the test:

```posix-terminal
fx test <TEST_NAME>
```

If `TEST_NAME` is a test component, `fx test` connects to your Fuchsia device to
load and run the test component. That is, the command finds the component's
corresponding [component URI][component-uri] and calls
[`ffx test run`][ffx-test]. However, if `TEST_NAME` is a host test, `fx test`
directly invokes that test binary to run on the host machine.

Similar to a host test, an end-to-end test also runs on a host machine. The test
then may interact with various services on a Fuchsia device for testing purposes
(see [Scripting Layer for Fuchsia][scripting-layer-for-fuchsia]). To run an
end-to-end test, provide an additional flag (`--e2e`) to `fx test`:

```posix-terminal
fx test --e2e <END_TO_END_TEST_NAME>
```

## Customize invocations {#customize-invocations}

`fx test` can run multiple tests or test suites at once. The command can also
filter those tests to be only device, host, or end-to-end tests.

To customize `fx test`, you can [add flags][fx-test-flags] and provide a
number of tests:

```posix-terminal
fx test <FLAGS> <TEST_NAME_01> <TEST_NAME_02> ...
```

Common ways to customize `fx test` are listed in the sections below.

### Run multiple tests {#run-multiple-tests}

If you want to run multiple sets of Fuchsia tests, configure your Fuchsia build
to include several of the primary testing bundles, build Fuchsia, and then run
all tests in the build. For example:

```none
fx set core.x64 --with //bundles/tools,//bundles/tests
fx build
fx test
```

You can also provide multiple targets in a single invocation:

```posix-terminal
fx test <PACKAGE_01> <PACKAGE_02> <COMPONENT_01> <COMPONENT_02>
```

See [Specify a test in multiple ways](#specify-a-test-in-multiple-ways) for
various ways to specify tests.

### Pass arguments to tests {#pass-argument-to-test}

Use the `--` flag to pass additional arguments to test components.

Note: `fx test` passes these arguments to all selected tests. When you target
many test components in a single command, this option may not be ideal.

The following example passes an `arg` flag to a test:

```none
$ fx test <TEST_NAME> -- --arg=5
```

For example, the command above internally calls the following command:

```none
$ fx ffx test run <TEST_COMPONENT_URI> -- --arg=5
```

## Specify a test in multiple ways {#specify-a-test-in-multiple-ways}

`fx test` supports multiple ways to reference a specific test:

* [Host test path](#host-test-path)
* [Package URL](#package-url)
* [Package name](#package-name)
* [Component name](#component-name)

### Host test path {#host-test-path}

For a host test, provide a relative path to the test binary from the root of the
Fuchsia build output directory. If the path points to a subdirectory, not a
file, `fx test` runs all matching test binaries in that directory.

For example, you can provide a relative path to specify a test binary:

```none
fx test host_x64/pm_cmd_pm_genkey_test
```

Or you can provide a relative path to a test directory:

```none
fx test host_x64/gen/sdk
```

### Package URL {#package-url}

Provide a full [Fuchsia component URL][component-uri] to specify a test
component. For example:

```none
fx test fuchsia-pkg://fuchsia.com/my_example_test_pkg#meta/my_example_test.cm
```

Provide a partial package URL to match and run all test components in the
package with the provided Fuchsia package URL. For example:

```none
fx test fuchsia-pkg://fuchsia.com/my_example_test_pkg
```

### Package name {#package-name}

Provide a [package name][fuchsia-package-name] to run all test components in
that package. For example:

```none
fx test my_example_test_pkg
```

To explicitly specify the input to be a package name, use the flag `-p`. For
example:

```none
fx test -p my_example_test_pkg
```

### Component name {#component-name}

Provide a component name (or a [resource path][resource-path]) to test a single
component in a package. For example:

```none
fx test my_example_test
```

To explicitly specify the input to be a component name, use the flag `-c`. For
example:

```none
fx test -c my_example_test
```

To run a component on a specific package, use both `-p <PACKAGE_NAME>` and `-c
<COMPONENT_NAME>`. For example:

```none
fx test -p my_example_test_pkg -c my_example_test
```

## Set the minimum log severity

`fx test` (and the underlying `ffx test`) accept a flag ``--min-severity-logs` which allows you to
set the minimum severity of the logs that are emitted by the test and components under the test.

If the test or components under it are using logging libraries which support setting dynamic
log severity (Fuchsia Rust and C++ log libraries support this). For test components that don't
support this, [`test_manager`][test-manager] manually filters their logs if the tests emit logs of
a severity that is lower than the minimum you set.

This flag accepts two ways of defining the minimum severity:

- `<severity>`: one of `FATAL`, `ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`. This applies to logs
  emitted by the test itself and all components under the test.

- `<component selector>#<severity>` in which [`<component selector>`](#component-selectors)
  specifies a set of components under the test (for example `foo/bar`) and severity is one of the
  accepted severities mentioned earlier.

A few examples:

- `--min-severity-logs DEBUG`: the test and all components under the test are instructed to emit
  logs of severity `DEBUG` or higher. This is equivalent to using a component selector:
  `--min-severity-logs **#DEBUG`

- `--min-severity-logs a#DEBUG --min-severity-logs b/c#ERROR`: the component under the test `a`
  emits logs of severity `DEBUG` or higher and the component under the test `b/c` emits logs of
  severity ERROR or higher. Logs emitted by the test itself uses their default minimum severity.

- `--min-severity-logs '<root>#DEBUG'`: the test is instructed to emit logs of severity `DEBUG`
  or higher, but components under it emits logs using their default minimum severity.

- `--min-severity-logs foo/*/bar#ERROR`: all components named `bar` under a child of `foo` with any
  name emits logs of severity `ERROR` or higher. For example, `foo/a/bar` and `foo/baz/bar`
  are affected, but the test component, `foo/bar` and `a/b` aren't.

## Convert from run-host-tests {#convert-from-run-host-tests}

The `fx run-host-tests` and `fx run-e2e-tests` commands are being
deprecated in favor of `fx test`. See the following instructions on how to use
`fx test` in place of these commands:

* [run-host-tests](#run-host-tests)
* [run-e2e-tests](#run-e2e-tests)

fx test <NESTED_COMPONENT_NAME>

```

### run-host-tests {#run-host-tests}

Substitute `fx run-host-tests` with `fx test`.

From:

```posix-terminal
fx run-host-tests <PATH_TO_HOST_TEST>
```

To:

```posix-terminal
fx test <PATH_TO_HOST_TEST>
```

### run-e2e-tests

Substitute `fx run-e2e-tests` with `fx test` and an additional `--e2e` flag.

From:

```posix-terminal
fx run-e2e-tests <END_TO_END_TEST>
```

To:

```posix-terminal
fx test --e2e <END_TO_END_TEST>
```

## Test-driven development

The `fx smoke-test` command automatically detects all tests that are known to
the build system as affected by changes in your checkout. Try the following:

```posix-terminal
fx -i smoke-test --verbose
```

In the command above, `--verbose` also prints which tests `fx smoke-test`
thinks are affected by your change. `-i` automatically repeats this command
every time you save your changes. For test-driven development, try launching
this command in a separate shell and watching your code rebuild and retest as
you're working on it.

`fx smoke-test` works best with hermetic test packages. A test package is
hermetic if the package contains all the dependencies of any tests in it.
That is to say, any code changes that affect the outcome of this test should
require rebuilding that test's package as well.

## Inspect artifacts from test components

Component tests may produce additional artifacts that cannot be displayed to
stdout, such as [custom artifacts][custom-artifacts] and coverage profile. By
default, `fx test` silently discards these artifacts. To see these
artifacts, specify an output directory to `fx test` using
`--ffx-output-directory`. The artifacts are pulled out of the test and
saved to the specified directory.

<!-- Reference links -->

[compoennt-selectors]: /docs/reference/diagnostics/selectors.md#component-selector
[custom-artifacts]: /docs/development/testing/components/test_runner_framework.md#custom-artifacts
[tests-as-components]: /docs/development/testing/components/README.md
[scripting-layer-for-fuchsia]: /docs/development/drivers/concepts/driver_development/sl4f.md
[component-uri]: /docs/reference/components/url.md
[fuchsia-package-name]: /docs/concepts/packages/package_url.md#package-name
[resource-path]: /docs/concepts/packages/package_url.md#resource-paths
[test-manager]: /docs/get-started/learn/components/component-tests.md
[fx-test-flags]: https://fuchsia.dev/reference/tools/fx/cmd/test
[ffx-test]: /docs/development/sdk/ffx/run-device-tests.md
