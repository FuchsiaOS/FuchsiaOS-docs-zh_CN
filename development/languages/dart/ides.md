<!-- # IDEs -->
# IDE

<!--
### Dart SDK

A prebuilt Dart SDK is available for IDE consumption at:
`//third_party/dart/tools/sdks/<linux|mac>/dart-sdk`.
Note that this SDK is sometimes a few days behind the version of
`//third_party/dart`. If you require an up-to-date SDK, one gets built with
Fuchsia at:
`//out/<build-type>/dart_host/dart-sdk`.
-->
### Dart SDK

供 IDE 使用的预编译 Dart SDK 位于：`//third_party/dart/tools/sdks/<linux|mac>/dart-sdk`。
注意，此 SDK 的版本比 `//third_party/dart` 有时旧几天。若需要最新的 SDK，可使用和 Fuchsia 一起编译的版本，位于 `//out/<build-type>/dart_host/dart-sdk`。

<!--
## Visual Studio Code

1. Download and install [Visual Studio Code](https://code.visualstudio.com/)
1. [optional] Setup VS Code to launch from the command line
    * For Macs: To allow running VS Code from the terminal using the `code` command, follow the instructions [here](https://code.visualstudio.com/docs/setup/mac#_launching-from-the-command-line)

    * For Linux and Windows: This should already be done as part of the installation
1. Install the following extensions:
    * [Dart Code](https://marketplace.visualstudio.com/items?itemName=Dart-Code.dart-code): Support for programming in Dart
    * [FIDL language support](https://marketplace.visualstudio.com/items?itemName=fuchsia-authors.language-fidl): Syntax highlighting support for Fuchsia's FIDL files
    * [GN](https://marketplace.visualstudio.com/items?itemName=npclaudiu.vscode-gn): Syntax highlighting for GN build files
    * Optional but helpful git extensions:
      * [Git Blame](https://marketplace.visualstudio.com/items?itemName=waderyan.gitblame): See git blam information in the status bar
      * [Git History](https://marketplace.visualstudio.com/items?itemName=donjayamanne.githistory): View git log, file history, etc.
1. Here are some helpful user settings for Dart, you can open your user settings by typing `Ctrl+,`:
```json
{
  // Auto-formats your files when you save
  "editor.formatOnSave": true,

  // Settings only when working in Dart
  "[dart]": {
        // Adds a ruler at 80 characters
        "editor.rulers": [
            80
        ],

        // Makes the tab size 2 spaces
        "editor.tabSize": 2,
    },
}

```
-->
## Visual Studio Code

1. 下载并安装 [Visual Studio Code](https://code.visualstudio.com/)
1. [可选] 配置 VS Code 以从命令行启动
    * Mac：按照 [这里](https://code.visualstudio.com/docs/setup/mac#_launching-from-the-command-line) 的说明来允许用 `code` 命令在终端运行 VS Code

    * Linux 和 Windows：应已作为安装的一部分配置好了
1. 安装下列扩展：
    * [Dart Code](https://marketplace.visualstudio.com/items?itemName=Dart-Code.dart-code)：Dart 编程支持
    * [FIDL language support](https://marketplace.visualstudio.com/items?itemName=fuchsia-authors.language-fidl)：Fuchsia 的 FIDL 文件的语法高亮
    * [GN](https://marketplace.visualstudio.com/items?itemName=npclaudiu.vscode-gn)：GN 编译文件的语法高亮
    * 可选但有用的 git 扩展：
      * [Git Blame](https://marketplace.visualstudio.com/items?itemName=waderyan.gitblame)：在状态栏显示 git blame 信息
      * [Git History](https://marketplace.visualstudio.com/items?itemName=donjayamanne.githistory)：查看 git 日志、文件历史等等
1. 这里是一些有用的 Dart 用户设置，你可以输入 `Ctrl+,` 打开设置界面：
```json
{
  // Auto-formats your files when you save
  "editor.formatOnSave": true,

  // Settings only when working in Dart
  "[dart]": {
        // Adds a ruler at 80 characters
        "editor.rulers": [
            80
        ],

        // Makes the tab size 2 spaces
        "editor.tabSize": 2,
    },
}

```

<!--
## Troubleshooting

When you find the Dart analysis is not working properly in your IDE, try the
following:
- Delete `//out` and rebuild. Specifically, a release build overrides a debug
  build. This means that if you have a broken release build, any release build
  overrides a debug build. With a broken release build, no amount of correct
  rebuilding on debug will solve the issue until you delete
  `//out/release-x64`.
- Delete the .packages file in your project and rebuild.
- Reboot your machine.  (This has sometimes fixed the issue.)
-->
## 疑难解答

当发现 Dart 分析在你的 IDE 中工作不正常时，尝试以下选项：
- 删除 `//out` 并重新编译。特别地，发布版本会覆盖调试版本。这意味着如果发布版本中存在问题，它将会覆盖调试版本。坏的发布版本会导致无论多少次正确的调试模式重编译都无法解决问题，除非删除 `//out/release-x64`。
- 删除项目中的 .packages 文件并重新编译。
- 重启你的机器。（这有时解决了问题。）