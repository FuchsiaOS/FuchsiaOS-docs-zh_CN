<!-- 
# Explore Fuchsia {#explore-fuchsia}

In Fuchsia, components are the basic unit of executable software.
When a Fuchsia device or emulator is booted and displays the `$` prompt in the shell,
you can run [components](/docs/concepts/components/v2). 

To try running an example component on your Fuchsia device, see
[Run an example component](/docs/development/run/run-examples.md).

-->

# 探索 Fuchsia {#explore-fuchsia}

在 Fuchsia 中，组件是可执行软件的基础单元。当 Fuchsia 设备或模拟器启动完成，并成功在 shell 中显示 `$` 提示符时，你就可以运行 [组件](/docs/concepts/components/v2)了。 

参见[运行示例组件](/docs/development/run/run-examples.md)一文，尝试在你的 Fuchsia 设备上运行一个示例组件。

<!-- 
## Run shell commands

Device commands in Fuchsia use the command `dm`. For example, to get a list
of device commands, use the following command:

```posix-terminal
dm help
```

To reboot Fuchsia, use the following command:

```posix-terminal
dm reboot
```

See
[Connect to a target shell](/docs/development/build/fx.md#connect-to-a-target-shell)
for more information on connecting to your Fuchsia device or emulator.
-->

## 运行 shell 命令

在 Fuchsia 中，`dm` 命令用来执行设备命令，如重启等。 使用下述命令，获取所有可用设备命令列表：

```posix-terminal
dm help
```

使用下述命令，来重启 Fuchsia 设备：

```posix-terminal
dm reboot
```

参见[连接到目标设备 shell](/docs/development/build/fx.md#connect-to-a-target-shell)一文，了解更多关于连接 Fuchsia 设备或模拟器的说明。

<!-- 
## Write software for Fuchsia

FIDL (Fuchsia Interface Definition Language) is the Interprocess Communication (IPC) system for
Fuchsia. For an example of writing [FIDL](/docs/development/languages/fidl) APIs and client
and server components, review the
[FIDL tutorials](/docs/development/languages/fidl/tutorials/overview.md).

You can also read the [FIDL concepts doc](/docs/concepts/fidl/overview.md) to get a brief
overview of what FIDL is, including its design goals, requirements, and workflows.

-->

## 为 Fuchsia 编写软件

FIDL (Fuchsia 接口定义语言) 是 Fuchsia 的进程间通信(IPC)系统。参见[FIDL 教程](/docs/development/languages/fidl/tutorials/overview.md) 一文，了解编写 [FIDL](/docs/development/languages/fidl) API、客户端、服务端组件的示例。

阅读 [FIDL 概念](/docs/concepts/fidl/overview.md) 了解 FIDL 的概要描述，其中包括其设计目标、使用前提以及工作流程。

<!-- 
## Run tests

To test Fuchsia on your device, see
[Run Fuchsia tests](/docs/development/testing/run_fuchsia_tests.md).
-->

## 运行测试

参见[运行 Fuchsia 测试](/docs/development/testing/run_fuchsia_tests.md)一文，了解如何在你的设备上测试 Fuchsia。

<!-- 
## Launch a graphical component

Most graphical components in Fuchsia use the
[Scenic](/docs/concepts/graphics/scenic/scenic.md) system compositor. You can
launch such components (commonly found in `/system/apps`) using the
`present_view` command, for example:

```sh
present_view fuchsia-pkg://fuchsia.com/spinning_square_view#meta/spinning_square_view.cmx
```

See [Scenic example apps](/src/ui/examples).

-->

## 载入图形组件

Fuchsia 上大部分图形组件都使用[Scenic](/docs/concepts/graphics/scenic/scenic.md) 系统合成器。你可以使用 `present_view` 命令来载入这部分组件 (通常存放在 `/system/apps` 目录中) ：

```sh
present_view fuchsia-pkg://fuchsia.com/spinning_square_view#meta/spinning_square_view.cmx
```

参见 [Scenic 示例程序](/src/ui/examples) 了解更多信息。

<!-- 
If you launch a component that uses Scenic or hardware-accelerated graphics,
Fuchsia enters the graphics mode, which doesn't display the shell. To use the
shell, press `Alt+Escape` to enter the console mode. In the console mode,
`Alt+Tab` has the same behavior described in [Select a tab](#select-a-tab).
Press `Alt+Escape` again to return to the graphics mode.

-->

如果你载入了一个使用 Scenic 或硬件加速图形的组件，Fuchsia 则会进入图形模式，而非显示shell界面。
当然你也可以使用快捷键 `Alt+Escape` 来进入控制台模式。 在控制台模式下，使用快捷键 `Alt+Tab` 时，跟 [Select a tab](#select-a-tab)（译者注：原文链接无效）产生的效果一致。再次使用快捷键 `Alt+Escape` 即可返回图形模式。

<!-- 
## Contribute changes

To submit your contribution to Fuchsia, see
[Contribute changes](/docs/development/source_code/contribute_changes.md).

-->

## 为 Fuchsia 贡献源代码

参见[贡献源代码](/docs/development/source_code/contribute_changes.md)一文，向 Fuchsia 提交源代码，做出自己的一份贡献。

<!-- 
*   [fx workflows](/docs/development/build/fx.md)
*   [Workflow tips and questions](/docs/development/source_code/workflow_tips_and_faq.md)
*   [Configure editors](/docs/development/editors/)
*   [Source code layout](/docs/concepts/source_code/layout.md)
*   [Build system](/docs/concepts/build_system/index.md)
-->

## 扩展阅读

*   [fx 工作流](/docs/development/build/fx.md)
*   [工作流的提示及问答](/docs/development/source_code/workflow_tips_and_faq.md)
*   [配置你的编辑器](/docs/development/editors/)
*   [源代码结构](/docs/concepts/source_code/layout.md)
*   [构建系统](/docs/concepts/build_system/index.md)