# Explore Fuchsia {#explore-fuchsia}

Once you have Fuchsia up and running on a device or emulator,
check out the following resources:

*  [Run ffx commands](#run-ffx-commands).
*  [Run examples](#run-examples).
*  [Create Fuchsia components](#create-fuchsia-components).
*  [Contribute changes](#contribute-changes). -->
*  [运行 ffx 命令](#run-ffx-commands)。
*  [运行示例](#run-examples)。
*  [创建 Fuchsia 组件](#create-fuchsia-components)。
*  [贡献更改](#contribute-changes)。

## Run ffx commands {#run-ffx-commands}

[`ffx`][ffx-overview] is a host tool for Fuchsia target workflows that
provides the consistent development experience across all Fuchsia environments
and host platforms.

The following are some of `ffx` command examples:

*   Display the list of devices:

    ```posix-terminal
    ffx target list
    ```

*   Display the device information:

    ```posix-terminal
    ffx target show
    ```

*   Print the device logs:

    ```posix-terminal
    ffx log
    ```

*   Reboot the device:

    ```posix-terminal
    ffx target reboot
    ```

## Run examples {#run-examples}

To try out Fuchsia's sample software, check out the guides below:

*   [Run an example component](/development/run/run-examples.md)
*   [Run a test component](/development/run/run-test-component.md)
*   [Run an end-to-end test](/development/testing/run_an_end_to_end_test.md) -->
*   [运行示例组件](/development/run/run-examples.md)
*   [运行测试组件](/development/run/run-test-component.md)
*   [运行端到端测试](/development/testing/run_an_end_to_end_test.md)

## Create Fuchsia components {#create-fuchsia-components}

The basic executable units of software in Fuchsia are
[components](/concepts/components/v2), and these components interact
with each other using [FIDL](/concepts/fidl/overview.md)
(Fuchsia Interface Definition Language) protocols. -->
Fuchsia 中软件的最小可执行单元是[组件](/concepts/components/v2)，这些组件通过 [FIDL](/concepts/fidl/overview.md)（Fuchsia 接口定义语言）协议彼此交互。

<!-- To learn more about Fuchsia components and FIDL, check out the guides below: -->
要想了解更多有关 Fuchsia 组件和 FIDL 的信息，请参考如下指南：

*   [Build components](/development/components/build.md)
*   [FIDL overview](/development/languages/fidl/README.md)
*   [FIDL tutorials](/development/languages/fidl/tutorials/overview.md)

## Contribute changes {#contribute-changes}

When you're ready to contribute to the Fuchsia project,
see [Contribute changes][contribute-changes].

## See also

For more information on Fuchsia's development workflows,
check out the following resources:

*   [fx workflows](/development/build/fx.md)
*   [Workflow tips and questions](/development/source_code/workflow_tips_and_faq.md)
*   [Configure editors](/development/editors/)
*   [Source code layout](/development/source_code/layout.md)
*   [Build system](/development/build/build_system/index.md) -->
*   [fx 工作流程](/development/build/fx.md)
*   [工作流程技巧和问题](/development/source_code/workflow_tips_and_faq.md)
*   [配置编辑器](/development/editors/)
*   [源代码布局](/development/source_code/layout.md)
*   [构建系统](/development/build/build_system/index.md)


<!-- Reference links -->

[components]: /concepts/components/v2
[run-examples]: /development/run/run-examples.md
[ffx-overview]: /development/tools/ffx/overview.md
[fidl]: /development/languages/fidl
[fidl-tutorials]: /development/languages/fidl/tutorials/overview.md
[fidl-concepts]: /concepts/fidl/overview.md
[run-fuchsia-tests]: /development/testing/run_fuchsia_tests.md
[scenic]: /concepts/ui/scenic/index.md
[contribute-changes]: /development/source_code/contribute_changes.md
