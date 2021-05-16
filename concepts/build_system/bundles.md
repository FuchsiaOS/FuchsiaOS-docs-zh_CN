<!-- 
# Bundles
 -->
# 套装

<!-- 
Bundles are GN group labels that provide common major groups of features.
They can be included into one of the [dependency
sets](boards_and_products.md#dependency-sets).
 -->
套装（bundle）是 GN 组标签，它提供了常用主要特性组合。它们可以被包括在[依赖集合](boards_and_products.md#dependency-sets)之一内。

<!-- 
When using the `fx set` command, bundles are most commonly added to the
`universe` dependency set by use of the `--with` flag. See [fx build
configuration][fx-build-config] for more information.
 -->
使用 `fx set` 命令时，套装最常被通过使用 `--with` 标记添加至 `universe` 依赖集合中。参阅 [fx 构建配置][fx-build-config]以获取更多信息。

<!-- 
More information on the currently available bundles can be found in
[`//bundles`](/bundles/README.md).
 -->
您可以在 [`//bundles`](/bundles/README.md) 查找更多关于当前可用的套装信息。
<!-- 
## Key bundles
 -->
## 关键套装

<!-- 
* `tools` contains a broad array of the most common developer tools. This
  includes tools for spawning components from command-line shells, tools for
  reconfiguring and testing networks, making http requests, debugging programs,
  changing audio volume, and so on.
* `tests` causes all test programs to be built. Most test programs can be
  invoked using `run-test-component` on the device, or via `fx test`.
* `kitchen_sink` is a target that causes all other build targets to be
  included. It is useful when testing the impact of core changes, or when
  making large scale changes in the code base. It also may be a fun
  configuration for enthusiasts to play with, as it includes all software
  available in the source tree. Note that kitchen sink will produce more than
  20GB of build artifacts and requires at least 2GB of storage on the target
  device (size estimates from Q1/2019).
 -->
* `tools` 包含各种最常用开发工具。这包括用于从命令行外壳（command-line shell）生成组件的工具、用于重新配置和测试网络的工具、发送 http 请求的工具、调试程序的工具、更改音量的工具等等。
* `tests` 使得所有测试程序被构建。 大多数测试程序都可以通过使用设备上的 `run-test-component` 或通过 `fx test` 来调用。
* `kitchen_sink`是一个使所有其他构建目标都包括在内的目标。在测试核心更改的影响，或在代码库中进行大规模改动时， 此功能很有用。这也可能是一个有趣的配置，可供爱好者探索，因为它包括了源代码树中所有可用的软件。 注意，kitchen sink 将产生超过 20GB 的构建产物，并且要求目标设备上具有至少 2GB 的存储空间（根据2019年第一季度的大小估算）。

[fx-build-config]: /docs/development/build/fx.md#configure-a-build
