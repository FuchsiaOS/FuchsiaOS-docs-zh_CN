<!-- 
The Fuchsia **Test Runner Framework** enables developers to build tests for
components using a variety of languages and runtimes and execute them on a
target device. The framework provides **test runner** components that implement
the `fuchsia.test.Suite` protocol and integrate with common language-specific
testing frameworks such as GoogleTest (C++).
-->
Fuchsia **测试运行器框架**（Test Runner Framework）使开发者能够使用各种语言和运行时为组件构建测试，并在目标设备上执行它们。该框架提供了实现 `fuchsia.test.Suite` 协议的**测试运行器**组件，并与通用的特定语言的测试框架（如 GoogleTest (C++)）集成。

<!-- 
The `test_manager` component is responsible for running tests on a Fuchsia
device. It examines components implementing the test suite protocol and launches
them as child components. This means that `test_manager` is also responsible for
providing capabilities to each test suite, creating what is commonly called the
**test realm**.
-->
`test_manager` 组件负责在 Fuchsia 设备上运行测试。它检查实现测试套件协议的组件，并将它们作为子组件启动。这意味着 `test_manager` 也负责为每个测试套件提供能力，即创建通常所谓的**测试领域**。

<!-- 
![Diagram showing how the Test Runner Framework provides interfaces for
developers to expose test suites and for developer tools to execute tests on
the Fuchsia device.]
(/get-started/images/components/test-realm.png){: width="714"}
-->
![图中显示了测试运行器框架如何为开发者提供接口来公开测试套件，以及开发者工具如何在 Fuchsia 设备上执行测试。](/get-started/images/components/test-realm.png){: width="714"}

<!-- 
Developer tools such as `ffx test` communicate with the `test_manager` on the
device to execute test suites and retrieve the results.
-->
`ffx test` 等开发者工具与设备上的 `test_manager` 进行通信，以执行测试套件并取回结果。
