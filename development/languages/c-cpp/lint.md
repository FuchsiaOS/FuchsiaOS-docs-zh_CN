# Lint

使用 clang-tidy 分析 C++ 代码，旨在保持存储库警告干净。linter 在根目录层的 `.clang-tidy` 文件中配置。开发人员不应在较低目录层上创建其他配置文件，因为这将导致树中出现分歧。

## 如何分析

`fx lint` 是一个 Fuchsia 脚本，它将特定于语言的 linter 包装在一个通用的命令行界面中。它根据您指定的选项收集文件列表，通过匹配 Linter 将它们分开，并执行每个所需的 Linter。`clang-tidy` 用于 C 和 C++ 文件。

在没有任何其他参数的情况下，`fx lint` 会分析您最近 git 提交的文件，并通过 linter 传递它们：

```
fx lint
```

要将 lint 限制为 C++，请添加双破折号（--），后跟要匹配的文件模式，例如：

```
fx lint -- '*.cc' '*.cpp'
```

要通过 lint 运行特定的 GN 目标，请使用：

```
fx lint --target=<target>
```

通常不建议从顶层 `fuchsia` 目录运行 `fx lint --all`，可能需要几个小时才能完成。请确保 `cd` 到目录是满足您的分析要求的最佳顶层目录。例如：

```
(cd <your/subdir>; fx lint --all -- '*.cc')
```

您还可以添加 `--fix`，以便自动为某些（但不是全部）警告生成修复。

工具本身中记录了其他选项和示例。有关 `fx lint` 的最新文档（包括示例），请运行：

```
fx lint --help
```

## 取消显示警告

通过向有问题的行添加 `// NOLINT(<check_name>)` 或 `// NOLINTNEXTLINE(<check_name>)` 注释，可以取消任何警告。还可以通过编辑根目录层的 `.clang-tidy` 文件在存储库中完全禁用检查。

## 检查

已启用多个检查类别，其中的特定检查已被禁用，原因如下。启用的检查类别列表如下：

 - `bugprone-*`
 - `clang-diagnostic-*`
 - `google-*`
 - `misc-*`
 - `modernize-`
 - `performance-*`
 - `readability-*`

以下列表跟踪禁用特定[检查]的原因：

 - `clang-diagnostic-unused-command-line-argument` - ninja 生成的编译数据库包含链接器参数，该参数最终未使用，并为每个文件触发此警告。
 - `misc-noexcept*` - Fuchsia 不使用 C++ 异常。
 - `misc-non-private-member-variables-in-classes` - We don't allow classes/structs
   with a mix of private and public members, but all public is fine.不允许使用将私有成员和公有成员混在一起的类/结构体，只允许所有成员公有。
 - `modernize-deprecated-headers` - Fuchsia 使用旧式 C 头文件。
 - `modernize-use-nodiscard` - 在 Fuchsia 代码库中不常用。
 - `modernize-raw-string-literal` - 检查建议转换 `\xFF` 字面量，最好将其保留为转义形式。
 - `modernize-return-braced-init-list` - 考虑到为构造函数参数返回带括号的初始化列表的可读性，建议显式使用构造函数。
 - `modernize-use-emplace` - 启用 IgnoreImplicitConstructors 选项以符合[Abseil 本周提示 #112](https://abseil.io/tips/112)。
 - `modernize-use-equals-delete` - 标记所有 gtest TEST_F。
 - `modernize-use-trailing-return-type` - Fuchsia C++ 代码通常使用 `int foo()` 样式定义函数，而不是检查推荐的 `auto foo() -> int` 样式。
 - `readability-implicit-bool-conversion` - Furhsia C++代码通常使用指针和数字的隐式布尔强制转换
 - `readability-isolate-declaration` - Zircon 代码通常使用成对的声明。
 - `readability-uppercase-literal-suffix` - Fuchsia C++ 代码选择不对此施加样式。

# 静态分析

严格讲，它不分析，但 Clang 静态分析器可以进行深入分析，以发现错误。有关详细信息，请参见[静态分析][static_analysis]。

[static_analysis]: /docs/development/build/static_analysis.md
[检查]: https://clang.llvm.org/extra/clang-tidy/checks/list.html
