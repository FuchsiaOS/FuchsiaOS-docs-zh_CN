The Fuchsia **Test Runner Framework** enables developers to build tests for
components using a variety of languages and runtimes and execute them on a
target device. The framework provides **test runner** components that implement
the `fuchsia.test.Suite` protocol and integrate with common language-specific
testing frameworks such as GoogleTest (C++).

The `test_manager` component is responsible for running tests on a Fuchsia
device. It examines components implementing the test suite protocol and launches
them as child components. This means that `test_manager` is also responsible for
providing capabilities to each test suite, creating what is commonly called the
**test realm**.

![Diagram showing how the Test Runner Framework provides interfaces for
developers to expose test suites and for developer tools to execute tests on
the Fuchsia device.](/get-started/images/components/test-realm.png){: width="714"}

Developer tools such as `ffx test` communicate with the `test_manager` on the
device to execute test suites and retrieve the results.
