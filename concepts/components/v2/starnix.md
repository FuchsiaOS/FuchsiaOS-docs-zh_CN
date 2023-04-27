# Starnix

<!--
`starnix` is a [runner][glossary.runner] that allows for running
unmodified Linux binaries on Fuchsia.
 -->
Starnix 是一个允许在 Fuchsia 上运行未修改的 Linux 二进制文件的[运行器][glossary.runner]。

<!--
Linux binaries are not run inside a virtual machine. Instead,
`starnix` creates a compatible runtime environment by implementing
the Linux UAPI.
 -->
Linux 二进制文件不在虚拟机内运行。相反，Starnix 通过实现 Linux UAPI 来创建兼容的运行时环境。

<!--
Check out [`RFC-0082`][starnix-rfc] to learn more about the
motivation and design of `starnix`.
 -->
请查看 [`RFC-0082`][starnix-rfc]，以了解有关 Starnix 的动机和设计的更多信息。

<!--
## Using starnix
 -->
## 使用 Starnix

<!--
The main entry point to `starnix` is the `ffx starnix` tool.
 -->
Starnix 的主要入口点是 `ffx starnix` 工具。

<!--
Check out the [`README`][starnix-readme] for instructions
to run Linux binaries, tests, or an interactive Android shell.
 -->
请查看 [`README`][starnix-readme] 以获取运行 Linux 二进制文件、测试或交互式 Android Shell 的说明。

<!--
## Contributing
 -->
## 贡献

<!--
Patches that improve Linux compatibility are welcome. If you want
to report a bug or file a feature request, create an issue in the
[Starnix][starnix-monorail-component] component in Monorail.
 -->
欢迎改善 Linux 兼容性的补丁。如果要报告错误或提交功能请求，请在 Monorail 的 [Starnix][starnix-monorail-component] 组件中创建议题。

[glossary.runner]: /glossary/README.md#runner
[starnix-rfc]: /contribute/governance/rfcs/0082_starnix.md
[starnix-readme]: /src/starnix/kernel/README.md
[starnix-monorail-component]: https://bugs.fuchsia.dev/p/fuchsia/issues/list?q=component:Starnix
