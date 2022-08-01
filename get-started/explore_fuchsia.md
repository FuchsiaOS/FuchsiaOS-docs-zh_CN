<!-- # Explore Fuchsia {#explore-fuchsia} -->
# 探索 Fuchsia {#explore-fuchsia}

<!-- Once you have Fuchsia up and running on a device or emulator,
check out the following resources: -->
一旦 Fuchsia 在真机或者模拟器上启动并运行起来之后，可以查阅如下资源：

<!-- *  [Run ffx commands](#run-ffx-commands).
*  [Run examples](#run-examples).
*  [Create Fuchsia components](#create-fuchsia-components).
*  [Contribute changes](#contribute-changes). -->
*  [运行 ffx 命令](#run-ffx-commands).
*  [运行示例](#run-examples).
*  [创建 Fuchsia 组件](#create-fuchsia-components).
*  [贡献修订](#contribute-changes).

<!-- ## Run ffx commands {#run-ffx-commands} -->
## 运行 ffx 命令 {#run-ffx-commands}

<!-- [`ffx`][ffx-overview] is a host tool for Fuchsia target workflows that
provides the consistent development experience across all Fuchsia environments
and host platforms. -->
[`ffx`][ffx-overview] 是一个 Fuchsia 目标工作流的主机工具，为所有 Fuchsia 环境和主机平台上提供一致的开发体验。

<!-- The following are some of `ffx` command examples: -->
以下是一些 `ffx` 命令示例：

<!-- *   Display the list of devices: -->
* 显示设备列表

    ```posix-terminal
    ffx target list
    ```

<!-- *   Display the device information: -->
* 显示设备信息

    ```posix-terminal
    ffx target show
    ```

<!-- *   Print the device logs: -->
* 输出设备日志

    ```posix-terminal
    ffx log
    ```

<!-- *   Reboot the device: -->
* 重启设备

    ```posix-terminal
    ffx target reboot
    ```

<!-- ## Run examples {#run-examples} -->
## 运行示例

<!-- To try out Fuchsia's sample software, check out the guides below: -->
想试试 Fuchsia 的样例程序，参考如下指南：

<!-- *   [Run an example component](/development/run/run-examples.md)
*   [Run a test component](/development/run/run-test-component.md)
*   [Run an end-to-end test](/development/testing/run_an_end_to_end_test.md) -->
*   [运行一个样例组件](/development/run/run-examples.md)
*   [运行一个测试组件](/development/run/run-test-component.md)
*   [运行端到端测试](/development/testing/run_an_end_to_end_test.md)

<!-- ## Create Fuchsia components {#create-fuchsia-components} -->
## 创建 Fuchsia 组件 {#create-fuchsia-components}

<!-- The basic executable units of software in Fuchsia are
[components](/concepts/components/v2), and these components interact
with each other using [FIDL](/concepts/fidl/overview.md)
(Fuchsia Interface Definition Language) protocols. -->
Fuchsia 中最小可运行软件单元是[组件](/concepts/components/v2)，这些组件通过
[FIDL](/concepts/fidl/overview.md)（Fuchsia 接口定义语言）协议彼此交互。

<!-- To learn more about Fuchsia components and FIDL, check out the guides below: -->
要想了解更多有关 Fuchsia 组件和 FIDL 的信息，请参考下列指南：

<!-- *   [Build components](/development/components/build.md)
*   [FIDL overview](/development/languages/fidl/README.md)
*   [FIDL tutorials](/development/languages/fidl/tutorials/overview.md) -->
*   [构建组件](/development/components/build.md)
*   [FIDL 概览](/development/languages/fidl/README.md)
*   [FIDL 教程](/development/languages/fidl/tutorials/overview.md)

<!-- ## Contribute changes {#contribute-changes} -->
## 贡献修订 {#contribute-changes}

<!-- When you're ready to contribute to the Fuchsia project,
see [Contribute changes][contribute-changes]. -->
当您准备好为 Fuchsia 项目做出贡献时，请参阅[贡献修订][contribute-changes]。

<!-- ## See also -->
## 参阅

<!-- For more information on Fuchsia's development workflows,
check out the following resources: -->
要获取关于 Fuchsia 开发流程的更多信息，请参阅下列资源：

<!-- *   [fx workflows](/development/build/fx.md)
*   [Workflow tips and questions](/development/source_code/workflow_tips_and_faq.md)
*   [Configure editors](/development/editors/)
*   [Source code layout](/development/source_code/layout.md)
*   [Build system](/development/build/build_system/index.md) -->
*   [fx 工作流程](/development/build/fx.md)
*   [工作流程技巧和问题](/development/source_code/workflow_tips_and_faq.md)
*   [配置编辑器](/development/editors/)
*   [源码规划](/development/source_code/layout.md)
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
