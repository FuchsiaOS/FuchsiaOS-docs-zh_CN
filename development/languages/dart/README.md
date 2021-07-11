# Dart

<!-- ## Overview -->

## 概述

<!-- Dart artifacts are not built the same way in Fuchsia as they are on other
platforms. -->

Dart 工件在 Fuchsia 中的构建方式与在其他平台上不同。

<!-- Instead of relying on [`pub`][pub] to manage dependencies, sources of
third-party packages we depend on are checked into the tree under
`//third_party/dart-pkg`.
This is to ensure we use consistent versions of our dependencies across multiple
builds. -->

无需依靠 [`pub`][pub] 来管理依赖关系，而是将我们依赖的第三方 packages 的源检入到 `//third_party/dart-pkg` 下的树中。
这是为了确保我们在多个构建中使用一致的依赖版本。

<!-- Likewise, no build output is placed in the source tree as everything goes under
`out/`. That includes `.packages` files, which are generated as part of the build
based on a target's dependency. -->

同样，由于所有内容都在 `out/` 下，因此不会在源代码树中放置任何构建输出。
其中包括 `.packages` 文件，这些文件是根据目标的依赖关系在构建过程中生成的。

<!-- ## Exiting Dart programs -->

## 退出 Dart 程序

<!-- The Dart runner for Fuchsia does not
monitor the FIDL channels opened by Dart programs and as a result does not end
the program normally, but rather waits for the explicit call to `fuchsia.exit()`
to indicate the program should be ended. -->

Fuchsia 的 Dart 运行程序不会监视 Dart 程序打开的 FIDL 通道，因此不会正常结束该程序，而是等待对 `fuchsia.exit()` 的显式调用以指示该程序应该结束。

<!-- Note: Calling exit() from dart:io will result in an exception since components
are not allowed to call this method since it would shutdown the dart_runner process. -->

Note: 从 dart:io 调用 exit() 将导致异常，因为不允许组件调用此方法，因为它将关闭 dart_runner 进程。

```dart
import 'package:fuchsia/fuchsia.dart' as fuchsia;

void main(List<String> args) {
  print('Hello Dart!');
  fuchsia.exit(23);
}
```

<!-- ## Targets -->

## 目标

<!-- There are five gn targets for building Dart: -->

有五个针对构建 Dart 的 gn 目标：

<!-- - [`dart_library`][target-library] defines a library that can be used by other
Dart -;
- [`dart_app`][target-app] defines a Dart executable for Fuchsia;
- [`dart_tool`][target-tool] defines a Dart tool for the host;
- [`flutter_app`][target-flutter] defines a [Flutter][flutter] application;
- [`dart_test`][target-test] defines a group of test. -->

- [`dart_library`][target-library] 定义了一个能够被其他 Dart 目标使用的库；
- [`dart_app`][target-app] 为 Fuchsia 定义了一个 Dart 可执行程序；
- [`dart_tool`][target-tool] 为主机定义了一个 Dart 工具；
- [`flutter_app`][target-flutter] 定义了一个 [Flutter][flutter] 应用程序；
- [`dart_test`][target-test] 定义了一个测试组。

<!-- See the definitions of each of these targets for how to use them. -->

请查看每个目标的定义以了解如何使用它们。

<!-- ## Package layout {#layout} -->

## 包结构 {#layout}

<!-- We use a layout very similar to the [standard layout][package-layout]. -->

我们使用和 [标准包结构][package-layout] 十分相似的结构。

<!-- ```
my_package/
  |
  |-- pubspec.yaml           # Empty, used as a marker [mandatory]
  |-- BUILD.gn               # Contains all targets
  |-- analysis_options.yaml  # Analysis configuration [mandatory]
  |-- lib/                   # dart_library contents
  |-- bin/                   # dart_binary's (target) or dart_tool's (host)
  |-- test/                  # dart_test contents
``` -->

```
my_package/
  |
  |-- pubspec.yaml           # 空的，用来做标记【必需】
  |-- BUILD.gn               # 包含所有的目标
  |-- analysis_options.yaml  # 分析配置项 [必要的]
  |-- lib/                   # dart_library 内容
  |-- bin/                   # dart_binary 的（目标）或 dart_tool 的（宿主）
  |-- test/                  # dart_test 内容
```

<!-- ## Going further -->

## 拓展延伸

<!-- - [Running analysis](analysis.md)
- [Style](style.md)
- [Testing](testing.md)
- [Logging](logging.md)
- [Using FIDL](fidl.md)
- [Managing third_party dependencies](third_party.md)
- [IDEs](ides.md) -->

- [运行分析](analysis.md)
- [样式](style.md)
- [测试](testing.md)
- [日志](logging.md)
- [使用 FIDL](fidl.md)
- [管理第三方依赖](third_party.md)
- [集成开发环境列表](ides.md)

[pub]: https://www.dartlang.org/tools/pub/get-started "Pub"
[package-layout]: https://www.dartlang.org/tools/pub/package-layout "Package layout"
[target-library]: /build/dart/dart_library.gni "dart_library target"
[target-app]: https://fuchsia.googlesource.com/topaz/+/HEAD/runtime/dart_runner/dart_app.gni "dart_app target"
[target-tool]: /build/dart/dart_tool.gni "dart_tool target"
[target-flutter]: https://fuchsia.googlesource.com/topaz/+/HEAD/runtime/flutter_runner/flutter_app.gni "flutter_app target"
[target-test]: /build/dart/dart.gni "dart_test target"
[flutter]: https://flutter.io/ "Flutter"
