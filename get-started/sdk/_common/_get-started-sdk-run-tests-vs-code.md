Run tests on the device by launching test components, which are included in the
[SDK samples repository][sdk-samples-repo]{:.external}.

The tasks include:

- Build and run the sample test components.
- Update one of the tests to fail.
- Verify the failure in the test results.

In VS Code, do the following:

1. In the terminal, build and run the sample test components:

   ```posix-terminal
   tools/bazel test --test_output=all //src/hello_world:test_pkg
   ```

   This command runs all the tests in the Hello World component’s test package
   ([`hello_world:test_pkg`][hello-world-test-package]{:.external}).

   The command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel test --test_output=all //src/hello_world:test_pkg
   ...
   INFO: From Testing //src/hello_world:test_pkg
   ==================== Test output for //src/hello_world:test_pkg:
   Error: Invalid build directory BUILD_WORKSPACE_DIRECTORY
   added repository bazel.test.pkg.hello.gtest.runnable
   Running test 'fuchsia-pkg://bazel.test.pkg.hello.gtest.runnable/hello_test#meta/hello_gtest_autogen_cml.cm'
   [RUNNING]       HelloTest.BasicAssertions
   [stdout - HelloTest.BasicAssertions]
   Running main() from gmock_main.cc
   Example stdout.
   [PASSED]        HelloTest.BasicAssertions

   1 out of 1 tests passed...
   fuchsia-pkg://bazel.test.pkg.hello.gtest.runnable/hello_test#meta/hello_gtest_autogen_cml.cm completed with result: PASSED
   added repository bazel.test.pkg.hello.test.runnable
   Running test 'fuchsia-pkg://bazel.test.pkg.hello.test.runnable/hello_test#meta/hello_test_autogen_cml.cm'
   [RUNNING]    main
   [PASSED] main
   [stdout - main]
   Example stdout.

   1 out of 1 tests passed...
   fuchsia-pkg://bazel.test.pkg.hello.test.runnable/hello_test#meta/hello_test_autogen_cml.cm completed with result: PASSED
   Running workflow: test_pkg_workflow_base
   Running task: test_pkg.debug_symbols_base (step 1/3)
   Running task: test_pkg.hello_gtest.run_base (step 2/3)
   Running task: test_pkg.hello_test.run_base (step 3/3)
   ================================================================================
   //src/hello_world:test_pkg                                      (cached) PASSED in 4.7s

   Executed 0 out of 1 test: 1 test passes.
   INFO: Build completed successfully, 1 total action
   ```

1. Click the **Explorer** icon on the left side of VS Code.

1. Open the `src/hello_world/hello_gtest.cc` file.

1. Edit the file to replace `EXPECT_STRNE()` with `EXPECT_STREQ()`.

   The test now should look like below:

   ```none {:.devsite-disable-click-to-copy}
   TEST(HelloTest, BasicAssertions) {
     std::cout << "Example stdout." << std::endl;

     // Expect two strings not to be equal.
     {{ '<strong>' }}EXPECT_STREQ("hello", "world");{{ '</strong>' }}
     // Expect equality.
     EXPECT_EQ(7 * 6, 42);
   }
   ```

   This change will cause the [GoogleTest][google-test]{:.external}
   (`hello_gtest`) to fail.

1. To save the file, press `CTRL+S` (or `Command+S` on macOS).

1. In the terminal, to verify that the updated test now fails, build and
   run the `hello_gtest` component:

   ```posix-terminal
   tools/bazel test --test_output=all //src/hello_world:test_pkg.hello_gtest
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel test --test_output=all //src/hello_world:test_pkg.hello_gtest
   ...
   INFO: From Testing //src/hello_world:test_pkg.hello_gtest:
   ==================== Test output for //src/hello_world:test_pkg.hello_gtest:
   Error: Invalid build directory BUILD_WORKSPACE_DIRECTORY
   added repository bazel.test.pkg.hello.gtest.runnable
   Running test 'fuchsia-pkg://bazel.test.pkg.hello.gtest.runnable/hello_test#meta/hello_gtest_autogen_cml.cm'
   [RUNNING]       HelloTest.BasicAssertions
   [stdout - HelloTest.BasicAssertions]
   Running main() from gmock_main.cc
   Example stdout.
   src/hello_world/hello_gtest.cc:14: Failure
   Expected equality of these values:
     "hello"
     "world"
   [FAILED]       HelloTest.BasicAssertions

   Failed tests: HelloTest.BasicAssertions
   0 out of 1 tests passed...
   fuchsia-pkg://bazel.test.pkg.hello.gtest.runnable/hello_test#meta/hello_gtest_autogen_cml.cm completed with result: FAILED
   Tests failed.
   More information may be available in ffx host logs in directory:
       /usr/local/google/home/alice/.local/share/Fuchsia/ffx/cache/logs
   Fatal: Shell task ['/usr/local/google/home/alice/.cache/bazel/_bazel_alice/ea119f1048230a864836be3d62fead2c/execroot/__main__/bazel-out/x86_64-fastbuild-ST-1ad63a09c27b/bin/src/hello_world/test_pkg.hello_gtest_runnable_run_component.sh'] failed.
   Running workflow: test_pkg.hello_gtest_base
   Running task: test_pkg.debug_symbols_base (step 1/2)
   Running task: test_pkg.hello_gtest.run_base (step 2/2)
   Error: Task test_pkg.hello_gtest.run_base (step 2/2) failed to run.
   ================================================================================
   Target //src/hello_world:test_pkg.hello_gtest up-to-date:
     bazel-out/x86_64-fastbuild-ST-1ad63a09c27b/bin/src/hello_world/test_pkg.hello_gtest_base.sh
     bazel-out/x86_64-fastbuild-ST-1ad63a09c27b/bin/src/hello_world/test_pkg.hello_gtest_base_workflow.json
   INFO: Elapsed time: 4.922s, Critical Path: 4.50s
   INFO: 16 processes: 8 internal, 6 linux-sandbox, 2 local.
   INFO: Build completed, 1 test FAILED, 16 total actions
   //src/hello_world:test_pkg.hello_gtest                                   FAILED in 2.5s
     /usr/local/google/home/alice/.cache/bazel/_bazel_alice/ea119f1048230a864836be3d62fead2c/execroot/__main__/bazel-out/k8-fastbuild/testlogs/src/hello_world/test_pkg.hello_gtest/test.log

   INFO: Build completed, 1 test FAILED, 16 total actions
   ```

<!-- Reference links -->

[google-test]: https://google.github.io/googletest/
[hello-world-test-package]: https://fuchsia.googlesource.com/sdk-samples/getting-started/+/refs/heads/main/src/hello_world/BUILD.bazel#68
[sdk-samples-repo]: https://fuchsia.googlesource.com/sdk-samples/getting-started
