### Fuchsia 构建系统：变体（Variants）

Fuchsia GN 构建系统允许不同的组件在不同的 变体 中构建。一个变体常常表示使用额外的编译选项，但如果你编写更多的 GN 代码，它们的功能远不止于此。
至今定义的变体描述了：
[排查工具](https://github.com/google/sanitizers/wiki) 和
[LTO](https://llvm.org/docs/LinkTimeOptimization.html) 之类的内容。

GN 构建指令
[`select_variant`](https://fuchsia.googlesource.com/garnet/+/master/docs/gen/build_arguments.md#select_variant)
控制了哪些组件在哪些变体中被构建。这些规则自动应用于 GN 文件中定义的的每一个 `executable`，`loadable_module`，或者 `driver_module` 目标。这是一个灵活的系统，可以让你指定一张对应关系列表，指明那个目标使用哪个变体（如果有的话）。为了支持这种灵活性，`select_variant` 的值使用了一种灵活的 GN 语法。在简单的情况下，它可以只是一个字符串的列表。

这是一个直接运行 `gn gen` 的例子：

```sh
./buildtools/gn gen out/x64 --args='select_variant=["host_asan", "asan/cat", "asan/ledger"]'
```

以下命令使用 `fx set` 工具做了同样的工作：

```sh
fx set x64 --variant={host_asan,asan/cat,asan/ledger}
```

1. 参数列表中的第一个开关定义了 `host_asan` 对应规则。这一规则使能了在主机上为所有 executables 构建运行 [AddressSanitizer](https://clang.llvm.org/docs/AddressSanitizer.html) 的能力。

2. 参数列表中的第二个开关定义了 `asan` 对应规则。这一规则是能了在目标机器上为所有 executables 构建运行 AddressSanitizer 的能力。`/cat` 后缀表示这条对应规则只应用于名为 `cat` 的二进制文件。

3. 参数列表中的第三个开关和第二个一样，只不过匹配的是名为 `ledger` 的二进制文件。

GN 代码还支持二进制文件名之外更多灵活的匹配规则，但需要更为复杂的语法。如果需要做更复杂的工作，直接设置
[`select_variant`](https://fuchsia.googlesource.com/garnet/+/master/docs/gen/build_arguments.md#select_variant)
GN 构建参数。

 * 如果语法有问题，你可以通过 `--args` 开关来切换到 `gn gen`

 * 最简单的实验方式是从设置一些近似于你的需求的 `--variant` 开关开始，然后编辑 `select_variant` 的值
   `fx set` 处理了：
   * 你可以只编辑 GN 输出目录中的 `args.gn` 文件
    （e.g. `out/x64/args.gn`） 下一次执行 `ninja` 构建（通过 `fx build`）的时候， `gn gen` 将会重新执行以适配更改。
   * 你可以使用 `./buildtools/gn args out/x64` 命令，这将用你的 `$EDITOR` 打开 `args.gn` 文件，然后立即执行 `gn gen`，这样你就可以立即看到 GN 语法中的错误。

要查看可用的变体列表并学习如何定义新的变体，查看
[`known_variants`](https://fuchsia.googlesource.com/garnet/+/master/docs/gen/build_arguments.md#known_variants)
构建参数。