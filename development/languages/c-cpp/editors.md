# C++ 编辑器/IDE 设置

本页面列出了各种社区贡献的插件，供不同的编辑器使用。注意，这些插件均不受官方支持，但是一些用户反映使用体验较好。

## CLion

按照下面的**编译数据库**说明在 fuchsia 根目录中创建适当的项目描述文件。

然后，在 Clion 中选择*从源导入项目*，并选择 fuchsia 根目录。

### CLion 性能调整

您可以尝试以下一些或全部方法以提高性能。这些方法仅是建议，最好直接与 JetBrains（<https://intellij-support.jetbrains.com/hc>）进行联系，以确保最适合您的环境。

#### 排除目录

为了加快索引时间，您可以排除不使用的目录。您可以在项目视图中通过右键单击每个目录并选择*将目录标记为->排除*来执行此操作。注意，受影响的配置存储在 `<project>/.idea/misc.xml` 中。

有关详细信息，请参阅
[控制源目录、库目录和排除目录 \- 帮助 \| CLion](https://www.jetbrains.com/help/clion/controlling-source-library-and-exclude-directories.html)
。

#### 注销 Git 存储库

fuchsia 的源代码树有大量的 git 存储库。CLion 扫描它们占用 CPU 周期。您可以在*文件 -> 设置 -> 版本控制*下注销未使用的 git 存储库。它们仍将在此处列出，因此您以后可以根据需要重新添加它们。

#### 调整 JVM 选项和平台属性

有关调整 CLion JVM 选项和平台属性的一般提示，请参阅[调整 CLion \- 帮助 \| CLion](https://www.jetbrains.com/help/clion/tuning-the-ide.html)。如链接所示，请联系 CLion 支持人员，以获取有关可能会帮助您解决所遇到问题的选项和值的说明。

### 编译数据库

[编译数据库](https://clang.llvm.org/docs/JSONCompilationDatabase.html) 文件 `compile_commands.json` 将由 `fx` 在当前构建目录中自动创建，并自动符号链接到源根目录。

请注意，此文件仅用于帮助 IDE 查找和解析源文件。构建过程仍应使用 `fx build` 来完成。

注意：CLion 在 Fuchsia 源代码中显示数百个文件的编译器错误，这是一个持续存在的问题。其他文件在 CLion 中应该可以正常工作。

## Vim

请参阅[Fuchsia 开发的有用 Vim 工具](/scripts/vim/README.md)。

## Visual Studio Code (VS Code) {#visual-studio-code}

请参阅更多关于
[Fuchsia 贡献者的 VS Code 配置建议](/docs/development/editors/vscode.md)。

### clangd

安装 [vscode-clangd](https://marketplace.visualstudio.com/items?itemName=llvm-vs-code-extensions.vscode-clangd)。如果安装了默认的 C/C++ 扩展，请禁用它。

在设置中，添加：

```
"clangd.path": "<absolute path to fuchsia root directory>/prebuilt/third_party/clang/<platform>/bin/clangd",
```

注意：指向 clangd 的路径必须是绝对路径。

最后，按照下面的**编译数据库**说明在 fuchsia 根目录中生成 `compile_commands.json`。然后重新加载 VS Code 以生效。

您还可以使用以下设置启用后台索引和 clang-tidy 功能：

```
"clangd.arguments": [
    "--clang-tidy",
    "--background-index"
]
```

有关 clangd 设置的更多详细信息，请参见[此处](https://clang.llvm.org/extra/clangd/Installation.html)。
