# Dart


<!--
## Overview

Dart artifacts are not built the same way in Fuchsia as they are on other
platforms.

Instead of relying on [`pub`][pub] to manage dependencies, sources of
third-party packages we depend on are checked into the tree under
[`//third_party/dart-pkg`][dart-3p].
This is to ensure we use consistent versions of our dependencies across multiple
builds.

Likewise, no build output is placed in the source tree as everything goes under
`out/`. That includes `.packages` files which are generated as part of the build
based on a target's dependency.
-->
## 概述

Dart 组件在 Fuchsia 中的构建方式和其他平台不同。

第三方依赖包被检出到 [`//third_party/dart-pkg`][dart-3p]，而不是由 [`pub`][pub] 解决依赖关系。
这样做的目的是确保在多次编译中使用固定版本的依赖。

同样，编译产物位于 `out/`,而非源代码树中，其中包括作为编译的一部分基于依赖关系生成的 `.packages` 文件。

<!--
## Targets

There are five gn targets for building Dart:
- [`dart_library`][target-library] defines a library that can be used by other
Dart targets;
- [`dart_app`][target-app] defines a Dart executable for Fuchsia;
- [`dart_tool`][target-tool] defines a Dart tool for the host;
- [`flutter_app`][target-flutter] defines a [Flutter][flutter] application;
- [`dart_test`][target-test] defines a group of test.

See the definitions of each of these targets for how to use them.
-->
## 对象

Dart 有五种 gn 对象：
- [`dart_library`][target-library] 声明可以被其他 Dart 对象引用的库；
- [`dart_app`][target-app] 声明在 Fuchsia 中可执行的 Dart 程序；
- [`dart_tool`][target-tool] 声明编译主机的 Dart 工具。
- [`flutter_app`][target-flutter] 声明 [Flutter][flutter] 应用；
- [`dart_test`][target-test] 声明测试组。

有关这些对象的用法请参见定义。

<!--
## Package layout

We use a layout very similar to the [standard layout][package-layout].

```
my_package/
  |
  |-- pubspec.yaml           # Empty, used as a marker [mandatory]
  |-- BUILD.gn               # Contains all targets
  |-- analysis_options.yaml  # Analysis configuration [mandatory]
  |-- lib/                   # dart_library contents
  |-- bin/                   # dart_binary's (target) or dart_tool's (host)
  |-- test/                  # dart_test contents
```
-->
## 包的样式

我们使用一个和 [标准样式][package-layout] 很相似的样式。

```
my_package/
  |
  |-- pubspec.yaml           # Empty, used as a marker [mandatory]
  |-- BUILD.gn               # Contains all targets
  |-- analysis_options.yaml  # Analysis configuration [mandatory]
  |-- lib/                   # dart_library contents
  |-- bin/                   # dart_binary's (target) or dart_tool's (host)
  |-- test/                  # dart_test contents
```

<!--
## Going further

- [Running analysis](analysis.md)
- [Style](style.md)
- [Testing](testing.md)
- [Logging](logging.md)
- [Using FIDL](fidl.md)
- [Managing third_party dependencies](third_party.md)
- [IDEs](ides.md)
-->
## 了解更多

- [进行分析](analysis.md)
- [格式](style.md)
- [测试](testing.md)
- [日志](logging.md)
- [使用 FIDL](fidl.md)
- [管理 third_party 依赖](third_party.md)
- [IDE](ides.md)

[pub]: https://www.dartlang.org/tools/pub/get-started "pub"
[package-layout]: https://www.dartlang.org/tools/pub/package-layout "Package layout"
[target-library]: https://fuchsia.googlesource.com/build/+/master/dart/dart_library.gni "dart_library target"
[target-app]: https://fuchsia.googlesource.com/topaz/+/master/runtime/dart_runner/dart_app.gni "dart_app target"
[target-tool]: https://fuchsia.googlesource.com/build/+/master/dart/dart_tool.gni "dart_tool target"
[target-flutter]: https://fuchsia.googlesource.com/topaz/+/master/runtime/flutter_runner/flutter_app.gni "flutter_app target"
[target-test]: https://fuchsia.googlesource.com/build/+/master/dart/dart_test.gni "dart_test target"
[flutter]: https://flutter.io/ "Flutter"