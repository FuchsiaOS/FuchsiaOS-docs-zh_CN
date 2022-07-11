# Run a test component

This guide shows you how to build Fuchsia to include a test package from
Fuchsia's source [`//examples`](/examples/) directory and run tests on
your Fuchsia target.

Note: You can find the source code for the "Hello, World" example at
[`//examples/hello_world`](/examples/hello_world).

## Prerequisites

Before you can run this test component, you must:

*   [Set up the Fuchsia development environment](/get-started/get_fuchsia_source.md)

## Exploring the example {#exploring-the-example}

The `Hello, world!` examples includes unit test components written in the
various supported languages. A test component has two key elements:

*   An [executable test suite](#executable-test) written in a supported language.
*   A [`BUILD.gn`](#build-gn) file to define the test component build target and
    include it in a Fuchsia test package.

### Executable test suite {#executable-test}

Fuchsia test suites are built as components and execute within a
[test runner](/development/testing/components/test_runner_framework.md) for supported
testing frameworks, such as GoogleTest.
The test suite binary includes test cases written against these
language-specific frameworks.

* {C++}

   ```cpp
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/cpp/hello_world_unittest.cc" region_tag="hello_test" adjust_indentation="auto" %}
   ```

* {Rust}

   ```rust
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/rust/src/main.rs" region_tag="test_mod" adjust_indentation="auto" %}
   ```

For more details on testing Fuchsia components, see
[Testing with Components](/development/testing/components)

### BUILD.gn {#build-gn}

The `BUILD.gn` file declares build targets for `fuchsia_unittest_package()`.
This template is specifically designed to package components containing tests.

* {C++}

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/cpp/BUILD.gn" region_tag="cpp_test" adjust_indentation="auto" %}
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/cpp/BUILD.gn" region_tag="fuchsia_test" adjust_indentation="auto" %}
   ```

* {Rust}

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/rust/BUILD.gn" region_tag="rustc_tests" adjust_indentation="auto" %}
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/rust/BUILD.gn" region_tag="fuchsia_test" adjust_indentation="auto" %}
   ```

To learn more about how Fuchsia uses GN to define test packages,
see: [Building components](/development/components/build.md).

## Include the example tests in your Fuchsia image {#include-the-example}

Note: For new build configurations, these commands can take up to 90 minutes.

To include the example package in your build configuration, use the `--with` flag
when setting your product and board environment:

<pre class="prettyprint">
<code class="devsite-terminal">fx set <var>product</var>.<var>board</var> --with //examples/hello_world:tests</code>
</pre>

For a Fuchsia emulator with the minimum build configuration, the command is:

```posix-terminal
fx set core.qemu-x64 --with //examples/hello_world:tests
```

In this example, `core` is a product with a minimal feature set, which includes
common network capabilities, and `x64` refers to the x64 architecture.

For a Fuchsia device with the minimum build configuration, the command is:

```posix-terminal
fx set core.x64 --with //examples/hello_world:tests
```

See [Configure a build](/development/build/fx.md#configure-a-build) for
more options.

Once you have set your build configuration, build Fuchsia with the following
command:

```posix-terminal
fx build
```

You now have a build that includes the example tests that can be
[fetched and launched on demand](/development/build/build_system/boards_and_products.md#universe).

## Run the test suite {#run-the-test-suite}

1.  Open a terminal and run `fx serve-updates`:

    ```posix-terminal
    fx serve-updates
    ```

1.  Open another terminal and run the `hello-world` unit tests:

    * {C++}

      ```posix-terminal
      fx test hello-world-cpp-unittests
      ```

    * {Rust}

      ```posix-terminal
      fx test hello-world-rust-tests
      ```

This command prints the following output:

* {C++}

  ```none
  $ fx test hello-world-cpp-unittests
  ...

  PASS: 0 FAIL: 0 00:00 🤔  fx shell run-test-suite '--max-severity-logs WARN' fuchsia-pkg://fuchsia.com/hello-world-cpp-unittests?hash=c34ed8b2ea21fd5158f1de77a5581a4b5123161ef851eea430768a00efc1cbbf#meta/hello-world-cpp-unittests.cm
  >> Runtime has exceeded 2 seconds (adjust this value with the -s|--slow flag)


  Running test 'fuchsia-pkg://fuchsia.com/hello-world-cpp-unittests?hash=c34ed8b2ea21fd5158f1de77a5581a4b5123161ef851eea430768a00efc1cbbf#meta/hello-world-cpp-unittests.cm'
  [RUNNING]	HelloWorldTest.True
  [PASSED]	HelloWorldTest.True


  1 out of 1 tests passed...
  fuchsia-pkg://fuchsia.com/hello-world-cpp-unittests?hash=c34ed8b2ea21fd5158f1de77a5581a4b5123161ef851eea430768a00efc1cbbf#meta/hello-world-cpp-unittests.cm completed with result: PASSED

  PASS: 1 FAIL: 0 00:15 ✅  fx shell run-test-suite '--max-severity-logs WARN' fuchsia-pkg://fuchsia.com/hello-world-cpp-unittests?hash=c34ed8b2ea21fd5158f1de77a5581a4b5123161ef851eea430768a00efc1cbbf#meta/hello-world-cpp-unittests.cm

  🎉  Ran 1 tests with 0 failures (use the -v flag to see each test) 🎉
  ```

* {Rust}

  ```none
  $ fx test hello-world-rust-tests
  ...

  PASS: 0 FAIL: 0 00:00 🤔  fx shell run-test-suite '--max-severity-logs WARN' fuchsia-pkg://fuchsia.com/hello-world-rust-tests?hash=45cc85adaa09af18c575c45be942d72e173719c53e69d879eeb9602fa38e4884#meta/hello-world-rust-tests.cm
  >> Runtime has exceeded 2 seconds (adjust this value with the -s|--slow flag)


  Running test 'fuchsia-pkg://fuchsia.com/hello-world-rust-tests?hash=45cc85adaa09af18c575c45be942d72e173719c53e69d879eeb9602fa38e4884#meta/hello-world-rust-tests.cm'
  [RUNNING]	hello_tests::greeting_test
  [RUNNING]	hello_tests::my_test
  [RUNNING]	tests::it_works
  [PASSED]	tests::it_works
  [PASSED]	hello_tests::my_test
  [PASSED]	hello_tests::greeting_test


  3 out of 3 tests passed...
  fuchsia-pkg://fuchsia.com/hello-world-rust-tests?hash=45cc85adaa09af18c575c45be942d72e173719c53e69d879eeb9602fa38e4884#meta/hello-world-rust-tests.cm completed with result: PASSED

  PASS: 1 FAIL: 0 00:15 ✅  fx shell run-test-suite '--max-severity-logs WARN' fuchsia-pkg://fuchsia.com/hello-world-rust-tests?hash=45cc85adaa09af18c575c45be942d72e173719c53e69d879eeb9602fa38e4884#meta/hello-world-rust-tests.cm
  ```
