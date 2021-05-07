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
corresponding [component URI][component-uri] and calls `run-test-component` on
the target device. However, if `TEST_NAME` is a host test, `fx test` directly
invokes that test binary to run on the host machine.

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
$ fx set core.x64 --with //bundles:tools,//bundles:tests
$ fx build
$ fx test
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

The following example passes a `timeout` flag to a test:

```none
$ fx test <TEST_NAME> -- --timeout=5
```

For example, the command above internally calls the following command on the
device:

```none
$ fx shell run-test-component <TEST_COMPONENT_URI> -- --timeout=5
$ fx shell run-test-suite <TEST_COMPONENT_URI> -- --timeout=5
```

## Specify a test in multiple ways {#specify-a-test-in-multiple-ways}

`fx test` supports multiple ways to reference a specific test:

*   [Host test path](#host-test-path)
*   [Package URL](#package-url)
*   [Package name](#package-name)
*   [Component name](#component-name)

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
fx test fuchsia-pkg://fuchsia.com/my_example_test_pkg#meta/my_example_test.cmx
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

## Convert from run-test or run-host-tests {#convert-from-run-test-or-run-host-tests}

The `fx run-test`, `fx run-host-tests` and `fx run-e2e-tests` commands are being
deprecated in favor of `fx test`. See the following instructions on how to use
`fx test` in place of these commands:

*   [run-test](#run-test)
*   [run-host-tests](#run-host-tests)
*   [run-e2e-tests](#run-e2e-tests)

### run-test {#run-test}

Substitute `fx run-test` with `fx test`.

From:

```posix-terminal
fx run-test <TEST_PACKAGE_NAME>
```

To:

```posix-terminal
fx test <TEST_PACKAGE_NAME>
```

#### The -t flag

With `run-test`, you were able to use the `-t` flag to specify a single test
component to run. For example:

```posix-terminal
fx run-test <PACKAGE_NAME> -t <NESTED_COMPONENT_NAME>
```

With `fx test`, this command becomes:

```posix-terminal
fx test -p <PACKAGE_NAME> -c <NESTED_COMPONENT_NAME>
```

If there are no name collisions for the test component, you can simply run:

```posix-terminal
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

In the command above, `--verbose` will also print which tests `fx smoke-test`
thinks are affected by your change. `-i` will automatically repeat this command
every time you save your changes. For test-driven development, try launching
this command in a separate shell and watching your code rebuild and retest as
you're working on it.

`fx smoke-test` works best with hermetic test packages. A test package is
hermetic if the package contains all the dependencies of any tests in it.
That is to say, any code changes that affect the outcome of this test should
require rebuilding that test's package as well.

<!-- Reference links -->

[tests-as-components]: /docs/concepts/testing/v1_test_component.md
[scripting-layer-for-fuchsia]: /docs/concepts/testing/sl4f.md
[component-uri]: /docs/concepts/components/component_urls.md
[glossary-components-v2]: /docs/glossary.md#components-v2
[rust-glob-syntax]: https://docs.rs/glob/0.3.0/glob/struct.Pattern.html
[fuchsia-package-name]: /docs/concepts/packages/package_url.md#package-name
[resource-path]: /docs/concepts/packages/package_url.md#resource-paths
[fx-test-flags]: https://fuchsia.dev/reference/tools/fx/cmd/test
